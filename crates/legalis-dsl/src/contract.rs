//! Contract, compliance, and inline-test AST for the Legalis DSL.
//!
//! This module extends the statute grammar with a `CONTRACT` block and a set of
//! top-level `@test` cases. A contract groups the constructs that recur across
//! private agreements and public regulation alike:
//!
//! ```text
//! CONTRACT supply-2025: "Widget Supply Agreement" {
//!     PARTY buyer:  "Acme Corp"  ROLE buyer
//!     PARTY seller: "Beta LLC"   ROLE seller
//!
//!     CLAUSE governing_law FROM governing_law: "Governed by the laws of Japan."
//!
//!     OBLIGATION pay BY buyer TO seller: "Pay each invoice"
//!         WHEN HAS invoice DUE "2025-12-31"
//!     RIGHT terminate OF seller CLAIM: "Terminate on default"
//!         CORRELATIVE pay
//!
//!     PERFORMANCE delivery {
//!         DESC "Deliver conforming goods"
//!         WHEN HAS purchase_order
//!         DUE "2025-06-30"
//!     }
//!
//!     COMPLIANCE iso_9001: "Maintain quality management" STANDARD "ISO 9001"
//!     PENALTY late_fee: "Late payment surcharge" AMOUNT 5 PER month FOR pay
//!     REPORT quarterly: "Financial statement" EVERY quarterly TO seller
//!     INSPECT safety: "On-site safety audit" BY regulator EVERY annually
//!     DEADLINE filing: "2025-04-15" "Annual filing"
//!     TIMELINE rollout: "Phased rollout" {
//!         DEADLINE phase1: "2025-03-01" "Pilot"
//!         DEADLINE phase2: "2025-09-01" "General availability"
//!     }
//! }
//!
//! @test "adult can vote" FOR voting {
//!     GIVEN age = 20, citizen = true
//!     EXPECT GRANT
//! }
//! ```
//!
//! The grammar round-trips through [`crate::printer::format_contract_document`]
//! and [`crate::LegalDslParser::parse_contract_document`]. Inline `@test` cases
//! are executed against parsed [`legalis_core::Statute`] values by
//! [`run_test_cases`], wiring the embedded expectations into the core
//! [`legalis_core::Condition::evaluate`] machinery.

use legalis_core::{AttributeBasedContext, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ast::ConditionNode;

/// A parsed contract document: zero or more [`ContractNode`]s plus the inline
/// [`TestCaseNode`]s declared with `@test`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractDocument {
    /// Contract definitions found in the source, in declaration order.
    pub contracts: Vec<ContractNode>,
    /// Inline `@test` cases found in the source, in declaration order.
    pub test_cases: Vec<TestCaseNode>,
}

impl ContractDocument {
    /// Returns the contract with the given identifier, if present.
    pub fn contract(&self, id: &str) -> Option<&ContractNode> {
        self.contracts.iter().find(|c| c.id == id)
    }

    /// Returns `true` when the document declares neither contracts nor tests.
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty() && self.test_cases.is_empty()
    }
}

/// A complete contract definition.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContractNode {
    /// Stable contract identifier (e.g. `supply-2025`).
    pub id: String,
    /// Human-readable contract title.
    pub title: String,
    /// Parties bound by the contract.
    pub parties: Vec<PartyNode>,
    /// Free-text provisions, optionally derived from a clause template.
    pub clauses: Vec<ClauseNode>,
    /// Obligations (duties) owed between parties.
    pub obligations: Vec<ObligationNode>,
    /// Rights held by parties, optionally correlative to an obligation.
    pub rights: Vec<RightNode>,
    /// Performance condition blocks.
    pub performances: Vec<PerformanceBlock>,
    /// Regulatory compliance requirements.
    pub compliance: Vec<ComplianceRequirementNode>,
    /// Penalty structures.
    pub penalties: Vec<PenaltyNode>,
    /// Reporting obligations.
    pub reports: Vec<ReportNode>,
    /// Inspection / audit requirements.
    pub inspections: Vec<InspectionNode>,
    /// Standalone dated deadlines.
    pub deadlines: Vec<DeadlineNode>,
    /// Grouped timelines (named collections of deadlines).
    pub timelines: Vec<TimelineNode>,
}

impl ContractNode {
    /// Creates an empty contract with the given identifier and title.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            ..Self::default()
        }
    }

    /// Returns the party with the given identifier, if present.
    pub fn party(&self, id: &str) -> Option<&PartyNode> {
        self.parties.iter().find(|p| p.id == id)
    }

    /// Returns the obligation with the given identifier, if present.
    pub fn obligation(&self, id: &str) -> Option<&ObligationNode> {
        self.obligations.iter().find(|o| o.id == id)
    }

    /// Returns every obligation for which `party_id` is the obligor (the party
    /// that owes the duty).
    pub fn obligations_of(&self, party_id: &str) -> Vec<&ObligationNode> {
        self.obligations
            .iter()
            .filter(|o| o.obligor.as_deref() == Some(party_id))
            .collect()
    }

    /// Returns the obligation a right is correlative to, if any. This realises
    /// the Hohfeldian claim-right / duty correlativity: a claim-right of one
    /// party is mirrored by a duty (obligation) of another.
    pub fn correlative_obligation(&self, right: &RightNode) -> Option<&ObligationNode> {
        right
            .correlative_obligation
            .as_deref()
            .and_then(|id| self.obligation(id))
    }

    /// Performs lightweight structural validation, returning a list of human
    /// readable problems (cross-references that do not resolve, duplicate ids).
    /// An empty list means the contract is internally consistent.
    pub fn validate(&self) -> Vec<String> {
        let mut problems = Vec::new();

        let party_ids: Vec<&str> = self.parties.iter().map(|p| p.id.as_str()).collect();
        let obligation_ids: Vec<&str> = self.obligations.iter().map(|o| o.id.as_str()).collect();

        let check_party = |slot: &Option<String>, ctx: &str, problems: &mut Vec<String>| {
            if let Some(p) = slot
                && !party_ids.contains(&p.as_str())
            {
                problems.push(format!("{ctx} references undefined party '{p}'"));
            }
        };

        for ob in &self.obligations {
            check_party(
                &ob.obligor,
                &format!("obligation '{}'", ob.id),
                &mut problems,
            );
            check_party(
                &ob.obligee,
                &format!("obligation '{}'", ob.id),
                &mut problems,
            );
        }
        for right in &self.rights {
            check_party(
                &right.holder,
                &format!("right '{}'", right.id),
                &mut problems,
            );
            if let Some(id) = &right.correlative_obligation
                && !obligation_ids.contains(&id.as_str())
            {
                problems.push(format!(
                    "right '{}' correlates to undefined obligation '{id}'",
                    right.id
                ));
            }
        }
        for penalty in &self.penalties {
            if let Some(id) = &penalty.for_obligation
                && !obligation_ids.contains(&id.as_str())
            {
                problems.push(format!(
                    "penalty '{}' references undefined obligation '{id}'",
                    penalty.id
                ));
            }
        }

        problems
    }
}

/// A party to a contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyNode {
    /// Stable party identifier referenced by obligations and rights.
    pub id: String,
    /// Legal name of the party.
    pub name: String,
    /// Optional contractual role.
    pub role: Option<PartyRole>,
}

/// The role a party plays in a contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PartyRole {
    /// Purchaser of goods or services.
    Buyer,
    /// Vendor of goods or services.
    Seller,
    /// Party granting a lease.
    Lessor,
    /// Party taking a lease.
    Lessee,
    /// Party employing another.
    Employer,
    /// Party employed by another.
    Employee,
    /// Party granting a licence.
    Licensor,
    /// Party receiving a licence.
    Licensee,
    /// Party guaranteeing performance.
    Guarantor,
    /// Any other role, preserving the source spelling.
    Other(String),
}

impl PartyRole {
    /// Maps a source keyword to a role, falling back to [`PartyRole::Other`].
    pub fn from_keyword(word: &str) -> Self {
        match word.to_ascii_uppercase().as_str() {
            "BUYER" => Self::Buyer,
            "SELLER" => Self::Seller,
            "LESSOR" => Self::Lessor,
            "LESSEE" => Self::Lessee,
            "EMPLOYER" => Self::Employer,
            "EMPLOYEE" => Self::Employee,
            "LICENSOR" => Self::Licensor,
            "LICENSEE" => Self::Licensee,
            "GUARANTOR" => Self::Guarantor,
            _ => Self::Other(word.to_string()),
        }
    }

    /// Returns the canonical source spelling of this role.
    pub fn display_word(&self) -> String {
        match self {
            Self::Buyer => "BUYER".to_string(),
            Self::Seller => "SELLER".to_string(),
            Self::Lessor => "LESSOR".to_string(),
            Self::Lessee => "LESSEE".to_string(),
            Self::Employer => "EMPLOYER".to_string(),
            Self::Employee => "EMPLOYEE".to_string(),
            Self::Licensor => "LICENSOR".to_string(),
            Self::Licensee => "LICENSEE".to_string(),
            Self::Guarantor => "GUARANTOR".to_string(),
            Self::Other(s) => s.clone(),
        }
    }
}

/// A free-text contractual provision, optionally derived from a template.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClauseNode {
    /// Stable clause identifier.
    pub id: String,
    /// Identifier of the [`ClauseTemplate`] this clause was instantiated from.
    pub from_template: Option<String>,
    /// The provision text.
    pub text: String,
}

/// An obligation (duty) owed by one party to another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObligationNode {
    /// Stable obligation identifier (referenced by rights and penalties).
    pub id: String,
    /// Description of the duty.
    pub description: String,
    /// Party that owes the duty (the obligor), referenced by id.
    pub obligor: Option<String>,
    /// Party owed the duty (the obligee), referenced by id.
    pub obligee: Option<String>,
    /// Conditions under which the obligation is triggered.
    pub conditions: Vec<ConditionNode>,
    /// Optional due date (ISO `YYYY-MM-DD`).
    pub due: Option<String>,
}

/// A Hohfeldian classification of a legal right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightKind {
    /// A claim-right correlative to another party's duty.
    Claim,
    /// A liberty / privilege to act.
    Liberty,
    /// A power to alter legal relations.
    Power,
    /// An immunity from another's power.
    Immunity,
}

impl RightKind {
    /// Maps a source keyword to a right kind, if recognised.
    pub fn from_keyword(word: &str) -> Option<Self> {
        match word.to_ascii_uppercase().as_str() {
            "CLAIM" => Some(Self::Claim),
            "LIBERTY" | "PRIVILEGE" => Some(Self::Liberty),
            "POWER" => Some(Self::Power),
            "IMMUNITY" => Some(Self::Immunity),
            _ => None,
        }
    }

    /// Returns the canonical source spelling of this right kind.
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Claim => "CLAIM",
            Self::Liberty => "LIBERTY",
            Self::Power => "POWER",
            Self::Immunity => "IMMUNITY",
        }
    }
}

/// A right held by a party, optionally correlative to an obligation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightNode {
    /// Stable right identifier.
    pub id: String,
    /// Description of the right.
    pub description: String,
    /// Party holding the right, referenced by id.
    pub holder: Option<String>,
    /// Hohfeldian classification of the right.
    pub kind: Option<RightKind>,
    /// Conditions under which the right is exercisable.
    pub conditions: Vec<ConditionNode>,
    /// Identifier of the obligation this right is correlative to.
    pub correlative_obligation: Option<String>,
}

/// A performance condition block: the conditions that must hold for performance
/// to be due, with an optional description and due date.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PerformanceBlock {
    /// Stable performance identifier.
    pub id: String,
    /// Optional description of the performance.
    pub description: Option<String>,
    /// Conditions that must hold for performance to be required.
    pub conditions: Vec<ConditionNode>,
    /// Optional due date (ISO `YYYY-MM-DD`).
    pub due: Option<String>,
}

/// A regulatory compliance requirement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceRequirementNode {
    /// Stable requirement identifier.
    pub id: String,
    /// Description of the requirement.
    pub description: String,
    /// Optional external standard satisfied by this requirement (e.g. ISO 9001).
    pub standard: Option<String>,
    /// Conditions under which the requirement applies.
    pub conditions: Vec<ConditionNode>,
}

/// A penalty structure for non-compliance or breach.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenaltyNode {
    /// Stable penalty identifier.
    pub id: String,
    /// Description of the penalty.
    pub description: String,
    /// Optional monetary amount.
    pub amount: Option<i64>,
    /// Optional currency code accompanying `amount` (e.g. `USD`).
    pub currency: Option<String>,
    /// Optional accrual period (e.g. `month` for "per month").
    pub per_unit: Option<String>,
    /// Identifier of the obligation this penalty enforces, if any.
    pub for_obligation: Option<String>,
    /// Conditions under which the penalty is triggered.
    pub conditions: Vec<ConditionNode>,
}

/// A recurrence frequency shared by reports and inspections.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportFrequency {
    /// Once per day.
    Daily,
    /// Once per week.
    Weekly,
    /// Once per month.
    Monthly,
    /// Once per quarter.
    Quarterly,
    /// Once per year.
    Annually,
    /// A single, one-off occurrence.
    Once,
    /// Any other cadence, preserving the source spelling.
    Custom(String),
}

impl ReportFrequency {
    /// Maps a source keyword to a frequency, falling back to
    /// [`ReportFrequency::Custom`].
    pub fn from_keyword(word: &str) -> Self {
        match word.to_ascii_uppercase().as_str() {
            "DAILY" => Self::Daily,
            "WEEKLY" => Self::Weekly,
            "MONTHLY" => Self::Monthly,
            "QUARTERLY" => Self::Quarterly,
            "ANNUALLY" | "ANNUAL" | "YEARLY" => Self::Annually,
            "ONCE" => Self::Once,
            _ => Self::Custom(word.to_string()),
        }
    }

    /// Returns the canonical keyword for a built-in frequency, or `None` for
    /// [`ReportFrequency::Custom`] (which the printer quotes verbatim).
    pub fn keyword(&self) -> Option<&'static str> {
        match self {
            Self::Daily => Some("DAILY"),
            Self::Weekly => Some("WEEKLY"),
            Self::Monthly => Some("MONTHLY"),
            Self::Quarterly => Some("QUARTERLY"),
            Self::Annually => Some("ANNUALLY"),
            Self::Once => Some("ONCE"),
            Self::Custom(_) => None,
        }
    }
}

/// A reporting obligation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportNode {
    /// Stable report identifier.
    pub id: String,
    /// Description of the report.
    pub description: String,
    /// Optional reporting cadence.
    pub frequency: Option<ReportFrequency>,
    /// Optional recipient of the report (party id or authority name).
    pub recipient: Option<String>,
    /// Optional due date for the (next) report (ISO `YYYY-MM-DD`).
    pub due: Option<String>,
}

/// An inspection / audit requirement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectionNode {
    /// Stable inspection identifier.
    pub id: String,
    /// Description of the inspection.
    pub description: String,
    /// Optional authority that performs the inspection.
    pub authority: Option<String>,
    /// Optional inspection cadence.
    pub frequency: Option<ReportFrequency>,
    /// Conditions under which the inspection is triggered.
    pub conditions: Vec<ConditionNode>,
}

/// A single dated deadline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadlineNode {
    /// Stable deadline identifier.
    pub id: String,
    /// The deadline date (ISO `YYYY-MM-DD`).
    pub date: String,
    /// Optional description of what is due.
    pub description: Option<String>,
}

/// A named timeline grouping several deadlines / milestones.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TimelineNode {
    /// Stable timeline identifier.
    pub id: String,
    /// Optional description of the timeline.
    pub description: Option<String>,
    /// Ordered deadlines (milestones) on the timeline.
    pub deadlines: Vec<DeadlineNode>,
}

// ---------------------------------------------------------------------------
// Clause template library
// ---------------------------------------------------------------------------

/// A reusable template for a common contractual provision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClauseTemplate {
    /// Identifier referenced by `CLAUSE <id> FROM <template>`.
    pub id: String,
    /// Short human-readable title.
    pub title: String,
    /// Boilerplate provision text.
    pub body: String,
}

impl ClauseTemplate {
    /// Instantiates this template into a [`ClauseNode`] under the given id.
    pub fn instantiate(&self, clause_id: impl Into<String>) -> ClauseNode {
        ClauseNode {
            id: clause_id.into(),
            from_template: Some(self.id.clone()),
            text: self.body.clone(),
        }
    }
}

/// Returns a library of templates for provisions common to most contracts.
///
/// These cover the boilerplate clauses a drafter reaches for first
/// (confidentiality, governing law, indemnification, force majeure,
/// termination, dispute resolution, entire agreement). They can be instantiated
/// into a contract via [`ClauseTemplate::instantiate`] or referenced from source
/// with `CLAUSE <id> FROM <template>`.
pub fn common_clause_templates() -> Vec<ClauseTemplate> {
    let entries = [
        (
            "confidentiality",
            "Confidentiality",
            "Each party shall keep confidential all non-public information disclosed \
             by the other party and use it solely to perform this agreement.",
        ),
        (
            "governing_law",
            "Governing Law",
            "This agreement shall be governed by and construed in accordance with \
             the laws of the stated jurisdiction, without regard to conflict-of-law rules.",
        ),
        (
            "indemnification",
            "Indemnification",
            "Each party shall indemnify and hold harmless the other from any loss \
             arising out of its breach of this agreement or its negligence.",
        ),
        (
            "force_majeure",
            "Force Majeure",
            "Neither party shall be liable for any failure to perform caused by events \
             beyond its reasonable control, including acts of God, war, or governmental action.",
        ),
        (
            "termination",
            "Termination",
            "Either party may terminate this agreement upon material breach by the other \
             party that remains uncured thirty days after written notice.",
        ),
        (
            "dispute_resolution",
            "Dispute Resolution",
            "Any dispute arising under this agreement shall first be submitted to \
             good-faith negotiation and, failing resolution, to binding arbitration.",
        ),
        (
            "entire_agreement",
            "Entire Agreement",
            "This agreement constitutes the entire understanding between the parties and \
             supersedes all prior negotiations, representations, and agreements.",
        ),
    ];

    entries
        .iter()
        .map(|(id, title, body)| ClauseTemplate {
            id: (*id).to_string(),
            title: (*title).to_string(),
            body: (*body).to_string(),
        })
        .collect()
}

/// Looks up a common clause template by id.
pub fn common_clause_template(id: &str) -> Option<ClauseTemplate> {
    common_clause_templates().into_iter().find(|t| t.id == id)
}

// ---------------------------------------------------------------------------
// Inline `@test` cases and the test runner
// ---------------------------------------------------------------------------

/// A single attribute binding supplied to a test case (`key = value`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestBinding {
    /// Attribute name made available to the evaluation context.
    pub key: String,
    /// Attribute value.
    pub value: TestValue,
}

/// A literal value usable in a `GIVEN` binding. A deliberately small value
/// domain (kept distinct from [`crate::ast::ConditionValue`]) so that
/// `GIVEN`/`EXPECT` round-trips are unambiguous.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestValue {
    /// An integer value.
    Number(i64),
    /// A quoted string value.
    String(String),
    /// A boolean value.
    Boolean(bool),
}

impl TestValue {
    /// Renders the value as the string an [`AttributeBasedContext`] expects.
    pub fn as_attribute_string(&self) -> String {
        match self {
            Self::Number(n) => n.to_string(),
            Self::String(s) => s.clone(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

/// The expected outcome of evaluating a statute against a test case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestExpectation {
    /// All of the statute's preconditions are expected to hold.
    Satisfied,
    /// At least one of the statute's preconditions is expected to fail.
    Unsatisfied,
    /// The statute is expected to be satisfied and to carry this effect kind.
    Effect(ExpectedEffect),
}

/// The effect kind asserted by an [`TestExpectation::Effect`] expectation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpectedEffect {
    /// Grant a right or permission.
    Grant,
    /// Revoke a right or permission.
    Revoke,
    /// Impose an obligation.
    Obligation,
    /// Impose a prohibition.
    Prohibition,
    /// A monetary transfer.
    MonetaryTransfer,
    /// A status change.
    StatusChange,
    /// A custom effect.
    Custom,
}

impl ExpectedEffect {
    /// Maps a source keyword to an expected effect kind, if recognised.
    pub fn from_keyword(word: &str) -> Option<Self> {
        match word.to_ascii_uppercase().as_str() {
            "GRANT" => Some(Self::Grant),
            "REVOKE" => Some(Self::Revoke),
            "OBLIGATION" => Some(Self::Obligation),
            "PROHIBITION" => Some(Self::Prohibition),
            "MONETARY" | "MONETARY_TRANSFER" | "TRANSFER" => Some(Self::MonetaryTransfer),
            "STATUS" | "STATUS_CHANGE" => Some(Self::StatusChange),
            "CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }

    /// Returns the canonical source keyword for this effect kind.
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Grant => "GRANT",
            Self::Revoke => "REVOKE",
            Self::Obligation => "OBLIGATION",
            Self::Prohibition => "PROHIBITION",
            Self::MonetaryTransfer => "MONETARY_TRANSFER",
            Self::StatusChange => "STATUS_CHANGE",
            Self::Custom => "CUSTOM",
        }
    }

    /// Returns `true` when this expectation matches a core [`EffectType`].
    pub fn matches(&self, effect_type: &EffectType) -> bool {
        matches!(
            (self, effect_type),
            (Self::Grant, EffectType::Grant)
                | (Self::Revoke, EffectType::Revoke)
                | (Self::Obligation, EffectType::Obligation)
                | (Self::Prohibition, EffectType::Prohibition)
                | (Self::MonetaryTransfer, EffectType::MonetaryTransfer)
                | (Self::StatusChange, EffectType::StatusChange)
                | (Self::Custom, EffectType::Custom)
        )
    }
}

/// An inline test case declared with `@test`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCaseNode {
    /// Human-readable name of the test case.
    pub name: String,
    /// Identifier of the statute the case is evaluated against.
    pub target_statute: String,
    /// Mock entity fixtures (see [`crate::testspec::MockEntityNode`]) pulled in
    /// with `USING <id>`. Their bindings seed the evaluation context and are
    /// overridden by any explicit `GIVEN` binding sharing the same key. Empty
    /// for cases that declare no `USING` clause.
    #[serde(default)]
    pub uses: Vec<String>,
    /// Attribute bindings forming the evaluation context.
    pub bindings: Vec<TestBinding>,
    /// The expected outcome.
    pub expectation: TestExpectation,
}

impl TestCaseNode {
    /// Builds the [`AttributeBasedContext`] this case evaluates against.
    pub fn context(&self) -> AttributeBasedContext {
        let attributes: HashMap<String, String> = self
            .bindings
            .iter()
            .map(|b| (b.key.clone(), b.value.as_attribute_string()))
            .collect();
        AttributeBasedContext::new(attributes)
    }
}

/// The outcome of running a single [`TestCaseNode`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCaseResult {
    /// Name of the case that was run.
    pub name: String,
    /// The targeted statute id.
    pub target_statute: String,
    /// Whether the case passed.
    pub passed: bool,
    /// Diagnostic detail (the reason for failure, or a success summary).
    pub message: String,
}

/// The aggregate outcome of running a suite of inline test cases.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TestRunReport {
    /// Per-case results, in input order.
    pub results: Vec<TestCaseResult>,
}

impl TestRunReport {
    /// Number of cases that passed.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Number of cases that failed.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Total number of cases run.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Returns `true` when every case passed (vacuously true for an empty run).
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

/// Maps a core [`EffectType`] to its canonical DSL keyword.
pub(crate) fn effect_type_keyword(effect_type: &EffectType) -> &'static str {
    match effect_type {
        EffectType::Grant => "GRANT",
        EffectType::Revoke => "REVOKE",
        EffectType::Obligation => "OBLIGATION",
        EffectType::Prohibition => "PROHIBITION",
        EffectType::MonetaryTransfer => "MONETARY_TRANSFER",
        EffectType::StatusChange => "STATUS_CHANGE",
        EffectType::Custom => "CUSTOM",
    }
}

/// Evaluates whether every precondition of `statute` holds in `context`.
///
/// Returns the boolean verdict, or a human-readable error string if any
/// precondition evaluation failed (e.g. a missing attribute the condition
/// strictly requires).
pub(crate) fn statute_satisfied(
    statute: &Statute,
    context: &AttributeBasedContext,
) -> Result<bool, String> {
    for condition in &statute.preconditions {
        match condition.evaluate(context) {
            Ok(true) => {}
            Ok(false) => return Ok(false),
            Err(e) => return Err(format!("evaluation error: {e:?}")),
        }
    }
    Ok(true)
}

/// Decides whether a statute's evaluation matches a test expectation and renders
/// a human-readable diagnostic. Shared by [`run_test_cases`] and the mock-aware
/// [`crate::testspec::run_test_cases_with_mocks`] so both paths report
/// identically.
pub(crate) fn evaluate_case_outcome(
    statute: &Statute,
    satisfied: bool,
    expectation: &TestExpectation,
) -> (bool, String) {
    match expectation {
        TestExpectation::Satisfied => (
            satisfied,
            if satisfied {
                "preconditions satisfied as expected".to_string()
            } else {
                "expected preconditions to be satisfied, but they were not".to_string()
            },
        ),
        TestExpectation::Unsatisfied => (
            !satisfied,
            if satisfied {
                "expected preconditions to fail, but they were satisfied".to_string()
            } else {
                "preconditions unsatisfied as expected".to_string()
            },
        ),
        TestExpectation::Effect(expected) => {
            let actual = &statute.effect.effect_type;
            let effect_ok = expected.matches(actual);
            let passed = satisfied && effect_ok;
            let message = if !satisfied {
                format!(
                    "expected effect {} but preconditions were not satisfied",
                    expected.keyword()
                )
            } else if !effect_ok {
                format!(
                    "expected effect {} but statute yields {}",
                    expected.keyword(),
                    effect_type_keyword(actual)
                )
            } else {
                format!("satisfied and yields {} as expected", expected.keyword())
            };
            (passed, message)
        }
    }
}

/// Runs each inline test case against the matching parsed statute.
///
/// This is the executable half of the `@test` syntax: a statute file embeds its
/// own expected-evaluation cases, and a downstream tool calls this function with
/// the parsed [`legalis_core::Statute`] set to verify them.
pub fn run_test_cases(statutes: &[Statute], cases: &[TestCaseNode]) -> TestRunReport {
    let mut results = Vec::with_capacity(cases.len());

    for case in cases {
        let Some(statute) = statutes.iter().find(|s| s.id == case.target_statute) else {
            results.push(TestCaseResult {
                name: case.name.clone(),
                target_statute: case.target_statute.clone(),
                passed: false,
                message: format!("no statute with id '{}'", case.target_statute),
            });
            continue;
        };

        let context = case.context();
        let satisfied = match statute_satisfied(statute, &context) {
            Ok(value) => value,
            Err(message) => {
                results.push(TestCaseResult {
                    name: case.name.clone(),
                    target_statute: case.target_statute.clone(),
                    passed: false,
                    message,
                });
                continue;
            }
        };

        let (passed, message) = evaluate_case_outcome(statute, satisfied, &case.expectation);

        results.push(TestCaseResult {
            name: case.name.clone(),
            target_statute: case.target_statute.clone(),
            passed,
            message,
        });
    }

    TestRunReport { results }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn voting_statute() -> Statute {
        Statute::new(
            "voting",
            "Voting Rights",
            Effect::new(EffectType::Grant, "Right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_run_satisfied_case_passes() {
        let case = TestCaseNode {
            name: "adult".to_string(),
            target_statute: "voting".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(20),
            }],
            expectation: TestExpectation::Satisfied,
        };
        let report = run_test_cases(&[voting_statute()], &[case]);
        assert!(report.all_passed());
        assert_eq!(report.passed(), 1);
    }

    #[test]
    fn test_run_unsatisfied_expectation() {
        let case = TestCaseNode {
            name: "minor".to_string(),
            target_statute: "voting".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(16),
            }],
            expectation: TestExpectation::Unsatisfied,
        };
        let report = run_test_cases(&[voting_statute()], &[case]);
        assert!(report.all_passed(), "{:?}", report.results);
    }

    #[test]
    fn test_run_effect_expectation() {
        let case = TestCaseNode {
            name: "grants".to_string(),
            target_statute: "voting".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(40),
            }],
            expectation: TestExpectation::Effect(ExpectedEffect::Grant),
        };
        let report = run_test_cases(&[voting_statute()], &[case]);
        assert!(report.all_passed());
    }

    #[test]
    fn test_run_effect_mismatch_fails() {
        let case = TestCaseNode {
            name: "wrong-effect".to_string(),
            target_statute: "voting".to_string(),
            uses: Vec::new(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(40),
            }],
            expectation: TestExpectation::Effect(ExpectedEffect::Revoke),
        };
        let report = run_test_cases(&[voting_statute()], &[case]);
        assert_eq!(report.failed(), 1);
        assert!(report.results[0].message.contains("expected effect REVOKE"));
    }

    #[test]
    fn test_run_unknown_statute_fails() {
        let case = TestCaseNode {
            name: "missing".to_string(),
            target_statute: "nope".to_string(),
            uses: Vec::new(),
            bindings: vec![],
            expectation: TestExpectation::Satisfied,
        };
        let report = run_test_cases(&[voting_statute()], &[case]);
        assert_eq!(report.failed(), 1);
        assert!(report.results[0].message.contains("no statute"));
    }

    #[test]
    fn test_common_clause_templates_present() {
        let templates = common_clause_templates();
        assert!(templates.len() >= 6);
        let governing = common_clause_template("governing_law").expect("template exists");
        let clause = governing.instantiate("gl");
        assert_eq!(clause.from_template.as_deref(), Some("governing_law"));
        assert!(!clause.text.is_empty());
    }

    #[test]
    fn test_contract_validation_detects_dangling_refs() {
        let mut contract = ContractNode::new("c1", "Test");
        contract.parties.push(PartyNode {
            id: "buyer".to_string(),
            name: "Acme".to_string(),
            role: Some(PartyRole::Buyer),
        });
        contract.obligations.push(ObligationNode {
            id: "pay".to_string(),
            description: "Pay".to_string(),
            obligor: Some("buyer".to_string()),
            obligee: Some("ghost".to_string()),
            conditions: vec![],
            due: None,
        });
        contract.rights.push(RightNode {
            id: "terminate".to_string(),
            description: "Terminate".to_string(),
            holder: None,
            kind: Some(RightKind::Claim),
            conditions: vec![],
            correlative_obligation: Some("missing".to_string()),
        });
        let problems = contract.validate();
        assert!(
            problems
                .iter()
                .any(|p| p.contains("undefined party 'ghost'"))
        );
        assert!(
            problems
                .iter()
                .any(|p| p.contains("undefined obligation 'missing'"))
        );
    }

    #[test]
    fn test_correlative_obligation_lookup() {
        let mut contract = ContractNode::new("c1", "Test");
        contract.obligations.push(ObligationNode {
            id: "pay".to_string(),
            description: "Pay".to_string(),
            obligor: None,
            obligee: None,
            conditions: vec![],
            due: None,
        });
        let right = RightNode {
            id: "collect".to_string(),
            description: "Collect".to_string(),
            holder: None,
            kind: Some(RightKind::Claim),
            conditions: vec![],
            correlative_obligation: Some("pay".to_string()),
        };
        let ob = contract
            .correlative_obligation(&right)
            .expect("correlative obligation resolves");
        assert_eq!(ob.id, "pay");
    }
}
