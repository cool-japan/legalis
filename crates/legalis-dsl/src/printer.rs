//! Pretty-printer for Legal DSL AST.
//!
//! Converts `Statute` structures back to human-readable DSL format.

use crate::ast::{ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode};
use legalis_core::{ComparisonOp, Condition, EffectType, Statute};

/// Configuration for the pretty-printer.
#[derive(Debug, Clone)]
pub struct PrinterConfig {
    /// Indentation string (default: 4 spaces)
    pub indent: String,
    /// Include comments with metadata
    pub include_comments: bool,
    /// Line width for wrapping (0 = no wrapping)
    pub line_width: usize,
    /// Uppercase keywords
    pub uppercase_keywords: bool,
}

impl Default for PrinterConfig {
    fn default() -> Self {
        Self {
            indent: "    ".to_string(),
            include_comments: false,
            line_width: 80,
            uppercase_keywords: true,
        }
    }
}

impl PrinterConfig {
    /// Creates a compact configuration with minimal formatting.
    pub fn compact() -> Self {
        Self {
            indent: "  ".to_string(),
            include_comments: false,
            line_width: 0,
            uppercase_keywords: true,
        }
    }

    /// Creates a verbose configuration with comments and wide lines.
    pub fn verbose() -> Self {
        Self {
            indent: "    ".to_string(),
            include_comments: true,
            line_width: 120,
            uppercase_keywords: true,
        }
    }
}

/// Pretty-printer for Legal DSL.
#[derive(Debug, Default)]
pub struct DslPrinter {
    config: PrinterConfig,
}

impl DslPrinter {
    /// Creates a new printer with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a printer with custom configuration.
    pub fn with_config(config: PrinterConfig) -> Self {
        Self { config }
    }

    /// Formats a statute as DSL text.
    pub fn format(&self, statute: &Statute) -> String {
        let mut output = String::new();

        // Optional header comment
        if self.config.include_comments {
            output.push_str(&format!("// Statute: {}\n", statute.title));
            if let Some(ref jur) = statute.jurisdiction {
                output.push_str(&format!("// Jurisdiction: {}\n", jur));
            }
            output.push('\n');
        }

        // STATUTE declaration
        output.push_str(&self.kw("STATUTE"));
        output.push(' ');
        output.push_str(&statute.id);
        output.push_str(": ");
        output.push_str(&self.quote(&statute.title));
        output.push_str(" {\n");

        // Metadata
        if let Some(ref jur) = statute.jurisdiction {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("JURISDICTION"));
            output.push(' ');
            output.push_str(&self.quote(jur));
            output.push('\n');
        }

        if statute.version > 1 {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("VERSION"));
            output.push(' ');
            output.push_str(&statute.version.to_string());
            output.push('\n');
        }

        if let Some(eff) = statute.temporal_validity.effective_date {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("EFFECTIVE_DATE"));
            output.push(' ');
            output.push_str(&eff.format("%Y-%m-%d").to_string());
            output.push('\n');
        }

        if let Some(exp) = statute.temporal_validity.expiry_date {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("EXPIRY_DATE"));
            output.push(' ');
            output.push_str(&exp.format("%Y-%m-%d").to_string());
            output.push('\n');
        }

        // Conditions
        if !statute.preconditions.is_empty() {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("WHEN"));
            output.push(' ');

            let conditions: Vec<String> = statute
                .preconditions
                .iter()
                .map(|c| self.format_condition(c))
                .collect();

            if conditions.len() == 1 {
                output.push_str(&conditions[0]);
            } else {
                output.push_str(&conditions.join(&format!(" {} ", self.kw("AND"))));
            }
            output.push('\n');
        }

        // Effect
        output.push_str(&self.config.indent);
        output.push_str(&self.kw("THEN"));
        output.push(' ');
        output.push_str(&self.format_effect_type(&statute.effect.effect_type));
        output.push(' ');
        output.push_str(&self.quote(&statute.effect.description));
        output.push('\n');

        // Discretion
        if let Some(ref discretion) = statute.discretion_logic {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("DISCRETION"));
            output.push(' ');
            output.push_str(&self.quote(discretion));
            output.push('\n');
        }

        output.push('}');
        output.push('\n');

        output
    }

    /// Formats multiple statutes as DSL text.
    pub fn format_batch(&self, statutes: &[Statute]) -> String {
        statutes
            .iter()
            .map(|s| self.format(s))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Formats a condition expression.
    fn format_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("{} {} {}", self.kw("AGE"), self.format_op(*operator), value)
            }
            Condition::Income { operator, value } => {
                format!(
                    "{} {} {}",
                    self.kw("INCOME"),
                    self.format_op(*operator),
                    value
                )
            }
            Condition::HasAttribute { key } => {
                if key.contains('-') || key.contains(' ') {
                    format!("{} {}", self.kw("HAS"), self.quote(key))
                } else {
                    format!("{} {}", self.kw("HAS"), key)
                }
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} = {}", self.quote(key), self.quote(value))
            }
            Condition::And(left, right) => {
                let left_str = self.format_condition(left);
                let right_str = self.format_condition(right);
                format!("{} {} {}", left_str, self.kw("AND"), right_str)
            }
            Condition::Or(left, right) => {
                let left_str = self.format_condition_with_parens(left, true);
                let right_str = self.format_condition_with_parens(right, true);
                format!("{} {} {}", left_str, self.kw("OR"), right_str)
            }
            Condition::Not(inner) => {
                let inner_str = self.format_condition_with_parens(inner, false);
                format!("{} {}", self.kw("NOT"), inner_str)
            }
            Condition::ResidencyDuration { operator, months } => {
                format!(
                    "{} {} {} months",
                    self.kw("RESIDENCY"),
                    self.format_op(*operator),
                    months
                )
            }
            Condition::Geographic {
                region_type,
                region_id,
            } => {
                format!(
                    "{} {:?} {}",
                    self.kw("REGION"),
                    region_type,
                    self.quote(region_id)
                )
            }
            Condition::DateRange { start, end } => {
                let start_str = start
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "*".to_string());
                let end_str = end
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "*".to_string());
                format!("{} {} TO {}", self.kw("DATE"), start_str, end_str)
            }
            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => {
                let target = target_entity_id
                    .as_ref()
                    .map(|t| self.quote(t))
                    .unwrap_or_else(|| "*".to_string());
                format!(
                    "{} {:?} {}",
                    self.kw("RELATIONSHIP"),
                    relationship_type,
                    target
                )
            }
            Condition::Custom { description } => {
                format!("{} {}", self.kw("CUSTOM"), self.quote(description))
            }
            Condition::Duration {
                operator,
                value,
                unit,
            } => {
                format!(
                    "{} {} {} {}",
                    self.kw("DURATION"),
                    self.format_op(*operator),
                    value,
                    unit
                )
            }
            Condition::Percentage {
                operator,
                value,
                context,
            } => {
                format!(
                    "{} {} {}% ({})",
                    self.kw("PERCENTAGE"),
                    self.format_op(*operator),
                    value,
                    context
                )
            }
            Condition::SetMembership {
                attribute,
                values,
                negated,
            } => {
                let values_str = values
                    .iter()
                    .map(|v| self.quote(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                if *negated {
                    format!(
                        "{} {} {} {{{}}}",
                        attribute,
                        self.kw("NOT"),
                        self.kw("IN"),
                        values_str
                    )
                } else {
                    format!("{} {} {{{}}}", attribute, self.kw("IN"), values_str)
                }
            }
            Condition::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                if *negated {
                    format!(
                        "{} {} {} {}",
                        attribute,
                        self.kw("NOT"),
                        self.kw("MATCHES"),
                        self.quote(pattern)
                    )
                } else {
                    format!(
                        "{} {} {}",
                        attribute,
                        self.kw("MATCHES"),
                        self.quote(pattern)
                    )
                }
            }
            Condition::Calculation {
                formula,
                operator,
                value,
            } => {
                format!(
                    "{} {} {} {}",
                    self.kw("CALC"),
                    self.quote(formula),
                    self.format_op(*operator),
                    value
                )
            }
            Condition::Composite {
                conditions,
                threshold,
            } => {
                let conditions_str = conditions
                    .iter()
                    .map(|(weight, cond)| {
                        format!("{:.2} * ({})", weight, self.format_condition(cond))
                    })
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!(
                    "{} [{}] >= {}",
                    self.kw("COMPOSITE"),
                    conditions_str,
                    threshold
                )
            }
            Condition::Threshold {
                attributes,
                operator,
                value,
            } => {
                let attrs_str = attributes
                    .iter()
                    .map(|(attr, multiplier)| {
                        if (*multiplier - 1.0).abs() < 0.0001 {
                            attr.clone()
                        } else {
                            format!("{:.2} * {}", multiplier, attr)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!(
                    "{} [{}] {} {}",
                    self.kw("THRESHOLD"),
                    attrs_str,
                    self.format_op(*operator),
                    value
                )
            }
            Condition::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                let points_str = membership_points
                    .iter()
                    .map(|(val, membership)| format!("({}, {:.2})", val, membership))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{} {} [{}] >= {:.2}",
                    self.kw("FUZZY"),
                    attribute,
                    points_str,
                    min_membership
                )
            }
            Condition::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                format!(
                    "{} ({}) p={:.2} >= {:.2}",
                    self.kw("PROBABILISTIC"),
                    self.format_condition(condition),
                    probability,
                    threshold
                )
            }
            Condition::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                format!(
                    "{} base={} ref={} rate={:.4} {} {}",
                    self.kw("TEMPORAL"),
                    base_value,
                    reference_time,
                    rate,
                    self.format_op(*operator),
                    target_value
                )
            }
        }
    }

    /// Formats a condition, adding parentheses if needed for clarity.
    fn format_condition_with_parens(&self, condition: &Condition, is_or_context: bool) -> String {
        let needs_parens = match condition {
            Condition::And(_, _) if is_or_context => true,
            Condition::Or(_, _) => true,
            _ => false,
        };

        let inner = self.format_condition(condition);
        if needs_parens {
            format!("({})", inner)
        } else {
            inner
        }
    }

    /// Formats a comparison operator.
    fn format_op(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    /// Formats an effect type.
    fn format_effect_type(&self, effect_type: &EffectType) -> String {
        let name = match effect_type {
            EffectType::Grant => "GRANT",
            EffectType::Revoke => "REVOKE",
            EffectType::Obligation => "OBLIGATION",
            EffectType::Prohibition => "PROHIBITION",
            EffectType::MonetaryTransfer => "MONETARY_TRANSFER",
            EffectType::StatusChange => "STATUS_CHANGE",
            EffectType::Custom => "CUSTOM",
        };
        self.kw(name)
    }

    /// Applies keyword casing based on configuration.
    fn kw(&self, keyword: &str) -> String {
        if self.config.uppercase_keywords {
            keyword.to_uppercase()
        } else {
            keyword.to_lowercase()
        }
    }

    /// Quotes a string value.
    fn quote(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

/// Formats a statute to DSL string using default configuration.
pub fn format_statute(statute: &Statute) -> String {
    DslPrinter::new().format(statute)
}

/// Formats multiple statutes to DSL string using default configuration.
pub fn format_statutes(statutes: &[Statute]) -> String {
    DslPrinter::new().format_batch(statutes)
}

/// Formats a LegalDocument AST back to DSL string.
pub fn format_document(doc: &LegalDocument) -> String {
    let mut output = String::new();

    // Format imports
    for import in &doc.imports {
        output.push_str("IMPORT \"");
        output.push_str(&import.path);
        output.push('"');
        if let Some(alias) = &import.alias {
            output.push_str(" AS ");
            output.push_str(alias);
        }
        output.push('\n');
    }

    if !doc.imports.is_empty() && !doc.statutes.is_empty() {
        output.push('\n');
    }

    // Format statutes
    for (idx, statute) in doc.statutes.iter().enumerate() {
        if idx > 0 {
            output.push('\n');
        }
        output.push_str(&format_statute_node(statute));
    }

    output
}

/// Formats a single StatuteNode back to DSL string.
fn format_statute_node(statute: &StatuteNode) -> String {
    let mut output = String::new();

    output.push_str("STATUTE ");
    output.push_str(&statute.id);
    output.push_str(": \"");
    output.push_str(&statute.title);
    output.push_str("\" {\n");

    // Requirements
    for req in &statute.requires {
        output.push_str("    REQUIRES ");
        output.push_str(req);
        output.push('\n');
    }

    // Supersedes
    if !statute.supersedes.is_empty() {
        output.push_str("    SUPERSEDES ");
        output.push_str(&statute.supersedes.join(", "));
        output.push('\n');
    }

    // Conditions
    for cond in &statute.conditions {
        output.push_str("    WHEN ");
        output.push_str(&format_condition_node(cond));
        output.push('\n');
    }

    // Effects
    for effect in &statute.effects {
        output.push_str("    THEN ");
        output.push_str(&format_effect_node(effect));
        output.push('\n');
    }

    // Defaults
    for default in &statute.defaults {
        output.push_str("    DEFAULT ");
        output.push_str(&default.field);
        output.push_str(" = ");
        output.push_str(&format_condition_value(&default.value));
        output.push('\n');
    }

    // Exceptions
    for exception in &statute.exceptions {
        output.push_str("    EXCEPTION");
        if !exception.conditions.is_empty() {
            output.push_str(" WHEN ");
            output.push_str(&format_condition_node(&exception.conditions[0]));
        }
        output.push_str(" \"");
        output.push_str(&exception.description);
        output.push_str("\"\n");
    }

    // Amendments
    for amendment in &statute.amendments {
        output.push_str("    AMENDMENT ");
        output.push_str(&amendment.target_id);
        if let Some(ver) = amendment.version {
            output.push_str(" VERSION ");
            output.push_str(&ver.to_string());
        }
        output.push_str(" \"");
        output.push_str(&amendment.description);
        output.push_str("\"\n");
    }

    // Discretion
    if let Some(disc) = &statute.discretion {
        output.push_str("    DISCRETION \"");
        output.push_str(disc);
        output.push_str("\"\n");
    }

    output.push_str("}\n");
    output
}

/// Formats a ConditionNode back to DSL string.
fn format_condition_node(cond: &ConditionNode) -> String {
    match cond {
        ConditionNode::Comparison {
            field,
            operator,
            value,
        } => {
            format!("{} {} {}", field, operator, format_condition_value(value))
        }
        ConditionNode::HasAttribute { key } => {
            format!("HAS {}", key)
        }
        ConditionNode::Between { field, min, max } => {
            format!(
                "{} BETWEEN {} AND {}",
                field,
                format_condition_value(min),
                format_condition_value(max)
            )
        }
        ConditionNode::In { field, values } => {
            let vals: Vec<String> = values.iter().map(format_condition_value).collect();
            format!("{} IN ({})", field, vals.join(", "))
        }
        ConditionNode::Like { field, pattern } => {
            format!("{} LIKE \"{}\"", field, pattern)
        }
        ConditionNode::Matches {
            field,
            regex_pattern,
        } => {
            format!("{} MATCHES \"{}\"", field, regex_pattern)
        }
        ConditionNode::InRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            let open = if *inclusive_min { "[" } else { "(" };
            let close = if *inclusive_max { "]" } else { ")" };
            format!(
                "{} IN_RANGE {}{}..{}{}",
                field,
                open,
                format_condition_value(min),
                format_condition_value(max),
                close
            )
        }
        ConditionNode::NotInRange {
            field,
            min,
            max,
            inclusive_min,
            inclusive_max,
        } => {
            let open = if *inclusive_min { "[" } else { "(" };
            let close = if *inclusive_max { "]" } else { ")" };
            format!(
                "{} NOT_IN_RANGE {}{}..{}{}",
                field,
                open,
                format_condition_value(min),
                format_condition_value(max),
                close
            )
        }
        ConditionNode::TemporalComparison {
            field,
            operator,
            value,
        } => {
            format!("{:?} {} {}", field, operator, format_condition_value(value))
        }
        ConditionNode::And(left, right) => {
            format!(
                "({}) AND ({})",
                format_condition_node(left),
                format_condition_node(right)
            )
        }
        ConditionNode::Or(left, right) => {
            format!(
                "({}) OR ({})",
                format_condition_node(left),
                format_condition_node(right)
            )
        }
        ConditionNode::Not(inner) => {
            format!("NOT ({})", format_condition_node(inner))
        }
    }
}

/// Formats a ConditionValue back to DSL string.
fn format_condition_value(value: &ConditionValue) -> String {
    match value {
        ConditionValue::Number(n) => n.to_string(),
        ConditionValue::String(s) => format!("\"{}\"", s),
        ConditionValue::Boolean(b) => b.to_string(),
        ConditionValue::Date(d) => d.clone(),
        ConditionValue::SetExpr(_) => "SET_EXPR".to_string(), // Simplified
    }
}

/// Formats an EffectNode back to DSL string.
fn format_effect_node(effect: &EffectNode) -> String {
    let mut output = effect.effect_type.to_uppercase();
    output.push_str(" \"");
    output.push_str(&effect.description);
    output.push('"');
    output
}

// ---------------------------------------------------------------------------
// Contract / compliance / inline-test printing (round-trips with
// `LegalDslParser::parse_contract_document`).
// ---------------------------------------------------------------------------

use crate::contract::{
    ClauseNode, ComplianceRequirementNode, ContractDocument, ContractNode, DeadlineNode,
    InspectionNode, ObligationNode, PartyNode, PenaltyNode, PerformanceBlock, ReportFrequency,
    ReportNode, RightNode, TestCaseNode, TestExpectation, TestValue, TimelineNode,
};
use crate::testspec::{
    CoverageRequirementNode, MockEntityNode, PropertyDomain, PropertySpecNode,
    SnapshotAssertionNode, SnapshotMode, TestSpecDocument,
};

/// Renders a scalar as a bare identifier when it is a "simple" word (letters,
/// digits and underscores only) and as a quoted string otherwise. This mirrors
/// the convention used by [`DslPrinter::format_condition`] for `HAS` keys and is
/// what the contract parser accepts on the way back in.
fn ident_or_quoted(value: &str) -> String {
    let simple = !value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if simple {
        value.to_string()
    } else {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

/// Quotes a string value (escaping backslashes and double quotes).
fn quote_str(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Formats a [`ContractDocument`] (its contracts then its `@test` cases) back to
/// DSL text.
pub fn format_contract_document(doc: &ContractDocument) -> String {
    let mut output = String::new();
    for (idx, contract) in doc.contracts.iter().enumerate() {
        if idx > 0 {
            output.push('\n');
        }
        output.push_str(&format_contract(contract));
    }
    if !doc.contracts.is_empty() && !doc.test_cases.is_empty() {
        output.push('\n');
    }
    for (idx, case) in doc.test_cases.iter().enumerate() {
        if idx > 0 {
            output.push('\n');
        }
        output.push_str(&format_test_case(case));
    }
    output
}

/// Formats a single [`ContractNode`] back to DSL text.
pub fn format_contract(contract: &ContractNode) -> String {
    let mut out = String::new();
    out.push_str("CONTRACT ");
    out.push_str(&contract.id);
    out.push_str(": ");
    out.push_str(&quote_str(&contract.title));
    out.push_str(" {\n");

    for party in &contract.parties {
        format_party(&mut out, party);
    }
    for clause in &contract.clauses {
        format_clause(&mut out, clause);
    }
    for obligation in &contract.obligations {
        format_obligation(&mut out, obligation);
    }
    for right in &contract.rights {
        format_right(&mut out, right);
    }
    for performance in &contract.performances {
        format_performance(&mut out, performance);
    }
    for requirement in &contract.compliance {
        format_compliance(&mut out, requirement);
    }
    for penalty in &contract.penalties {
        format_penalty(&mut out, penalty);
    }
    for report in &contract.reports {
        format_report(&mut out, report);
    }
    for inspection in &contract.inspections {
        format_inspection(&mut out, inspection);
    }
    for deadline in &contract.deadlines {
        out.push_str("    ");
        format_deadline_line(&mut out, deadline);
    }
    for timeline in &contract.timelines {
        format_timeline(&mut out, timeline);
    }

    out.push_str("}\n");
    out
}

fn format_party(out: &mut String, party: &PartyNode) {
    out.push_str("    PARTY ");
    out.push_str(&party.id);
    out.push_str(": ");
    out.push_str(&quote_str(&party.name));
    if let Some(role) = &party.role {
        out.push_str(" ROLE ");
        out.push_str(&ident_or_quoted(&role.display_word()));
    }
    out.push('\n');
}

fn format_clause(out: &mut String, clause: &ClauseNode) {
    out.push_str("    CLAUSE ");
    out.push_str(&clause.id);
    if let Some(template) = &clause.from_template {
        out.push_str(" FROM ");
        out.push_str(&ident_or_quoted(template));
    }
    out.push_str(": ");
    out.push_str(&quote_str(&clause.text));
    out.push('\n');
}

fn format_obligation(out: &mut String, obligation: &ObligationNode) {
    out.push_str("    OBLIGATION ");
    out.push_str(&obligation.id);
    if let Some(obligor) = &obligation.obligor {
        out.push_str(" BY ");
        out.push_str(&ident_or_quoted(obligor));
    }
    if let Some(obligee) = &obligation.obligee {
        out.push_str(" TO ");
        out.push_str(&ident_or_quoted(obligee));
    }
    out.push_str(": ");
    out.push_str(&quote_str(&obligation.description));
    for condition in &obligation.conditions {
        out.push_str(" WHEN ");
        out.push_str(&format_condition_node(condition));
    }
    if let Some(due) = &obligation.due {
        out.push_str(" DUE ");
        out.push_str(&quote_str(due));
    }
    out.push('\n');
}

fn format_right(out: &mut String, right: &RightNode) {
    out.push_str("    RIGHT ");
    out.push_str(&right.id);
    if let Some(holder) = &right.holder {
        out.push_str(" OF ");
        out.push_str(&ident_or_quoted(holder));
    }
    if let Some(kind) = &right.kind {
        out.push(' ');
        out.push_str(kind.keyword());
    }
    out.push_str(": ");
    out.push_str(&quote_str(&right.description));
    for condition in &right.conditions {
        out.push_str(" WHEN ");
        out.push_str(&format_condition_node(condition));
    }
    if let Some(correlative) = &right.correlative_obligation {
        out.push_str(" CORRELATIVE ");
        out.push_str(&ident_or_quoted(correlative));
    }
    out.push('\n');
}

fn format_performance(out: &mut String, performance: &PerformanceBlock) {
    out.push_str("    PERFORMANCE ");
    out.push_str(&performance.id);
    out.push_str(" {\n");
    if let Some(desc) = &performance.description {
        out.push_str("        DESC ");
        out.push_str(&quote_str(desc));
        out.push('\n');
    }
    for condition in &performance.conditions {
        out.push_str("        WHEN ");
        out.push_str(&format_condition_node(condition));
        out.push('\n');
    }
    if let Some(due) = &performance.due {
        out.push_str("        DUE ");
        out.push_str(&quote_str(due));
        out.push('\n');
    }
    out.push_str("    }\n");
}

fn format_compliance(out: &mut String, requirement: &ComplianceRequirementNode) {
    out.push_str("    COMPLIANCE ");
    out.push_str(&requirement.id);
    out.push_str(": ");
    out.push_str(&quote_str(&requirement.description));
    if let Some(standard) = &requirement.standard {
        out.push_str(" STANDARD ");
        out.push_str(&quote_str(standard));
    }
    for condition in &requirement.conditions {
        out.push_str(" WHEN ");
        out.push_str(&format_condition_node(condition));
    }
    out.push('\n');
}

fn format_penalty(out: &mut String, penalty: &PenaltyNode) {
    out.push_str("    PENALTY ");
    out.push_str(&penalty.id);
    out.push_str(": ");
    out.push_str(&quote_str(&penalty.description));
    if let Some(amount) = penalty.amount {
        out.push_str(" AMOUNT ");
        out.push_str(&amount.to_string());
        if let Some(currency) = &penalty.currency {
            out.push(' ');
            out.push_str(&ident_or_quoted(currency));
        }
    }
    if let Some(per) = &penalty.per_unit {
        out.push_str(" PER ");
        out.push_str(&ident_or_quoted(per));
    }
    if let Some(for_obligation) = &penalty.for_obligation {
        out.push_str(" FOR ");
        out.push_str(&ident_or_quoted(for_obligation));
    }
    for condition in &penalty.conditions {
        out.push_str(" WHEN ");
        out.push_str(&format_condition_node(condition));
    }
    out.push('\n');
}

fn format_frequency(out: &mut String, frequency: &ReportFrequency) {
    out.push_str(" EVERY ");
    match frequency.keyword() {
        Some(keyword) => out.push_str(keyword),
        None => {
            if let ReportFrequency::Custom(text) = frequency {
                out.push_str(&quote_str(text));
            }
        }
    }
}

fn format_report(out: &mut String, report: &ReportNode) {
    out.push_str("    REPORT ");
    out.push_str(&report.id);
    out.push_str(": ");
    out.push_str(&quote_str(&report.description));
    if let Some(frequency) = &report.frequency {
        format_frequency(out, frequency);
    }
    if let Some(recipient) = &report.recipient {
        out.push_str(" TO ");
        out.push_str(&ident_or_quoted(recipient));
    }
    if let Some(due) = &report.due {
        out.push_str(" DUE ");
        out.push_str(&quote_str(due));
    }
    out.push('\n');
}

fn format_inspection(out: &mut String, inspection: &InspectionNode) {
    out.push_str("    INSPECT ");
    out.push_str(&inspection.id);
    out.push_str(": ");
    out.push_str(&quote_str(&inspection.description));
    if let Some(authority) = &inspection.authority {
        out.push_str(" BY ");
        out.push_str(&ident_or_quoted(authority));
    }
    if let Some(frequency) = &inspection.frequency {
        format_frequency(out, frequency);
    }
    for condition in &inspection.conditions {
        out.push_str(" WHEN ");
        out.push_str(&format_condition_node(condition));
    }
    out.push('\n');
}

/// Appends a `DEADLINE <id>: "<date>" ["<desc>"]` line (the caller supplies the
/// leading indentation).
fn format_deadline_line(out: &mut String, deadline: &DeadlineNode) {
    out.push_str("DEADLINE ");
    out.push_str(&deadline.id);
    out.push_str(": ");
    out.push_str(&quote_str(&deadline.date));
    if let Some(desc) = &deadline.description {
        out.push(' ');
        out.push_str(&quote_str(desc));
    }
    out.push('\n');
}

fn format_timeline(out: &mut String, timeline: &TimelineNode) {
    out.push_str("    TIMELINE ");
    out.push_str(&timeline.id);
    if let Some(desc) = &timeline.description {
        out.push_str(": ");
        out.push_str(&quote_str(desc));
    }
    out.push_str(" {\n");
    for deadline in &timeline.deadlines {
        out.push_str("        ");
        format_deadline_line(out, deadline);
    }
    out.push_str("    }\n");
}

/// Formats a single inline `@test` case back to DSL text.
pub fn format_test_case(case: &TestCaseNode) -> String {
    let mut out = String::new();
    out.push_str("@test ");
    out.push_str(&quote_str(&case.name));
    out.push_str(" FOR ");
    out.push_str(&ident_or_quoted(&case.target_statute));
    out.push_str(" {\n");

    if !case.uses.is_empty() {
        out.push_str("    USING ");
        let rendered: Vec<String> = case.uses.iter().map(|id| ident_or_quoted(id)).collect();
        out.push_str(&rendered.join(", "));
        out.push('\n');
    }

    if !case.bindings.is_empty() {
        out.push_str("    GIVEN ");
        out.push_str(&format_bindings(&case.bindings));
        out.push('\n');
    }

    out.push_str("    EXPECT ");
    out.push_str(&format_expectation(&case.expectation));
    out.push('\n');

    out.push_str("}\n");
    out
}

/// Renders a `key = value, ...` binding list (shared by `@test`/`@property`).
fn format_bindings(bindings: &[crate::contract::TestBinding]) -> String {
    bindings
        .iter()
        .map(|binding| {
            format!(
                "{} = {}",
                ident_or_quoted(&binding.key),
                format_test_value(&binding.value)
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Renders an `EXPECT` outcome keyword.
fn format_expectation(expectation: &TestExpectation) -> String {
    match expectation {
        TestExpectation::Satisfied => "SATISFIED".to_string(),
        TestExpectation::Unsatisfied => "NOT SATISFIED".to_string(),
        TestExpectation::Effect(effect) => effect.keyword().to_string(),
    }
}

fn format_test_value(value: &TestValue) -> String {
    match value {
        TestValue::Number(n) => n.to_string(),
        TestValue::String(s) => quote_str(s),
        TestValue::Boolean(b) => b.to_string(),
    }
}

/// Formats a [`TestSpecDocument`] (mocks, `@test`s, properties, coverage,
/// snapshots) back to DSL text. Constructs are grouped by kind; within each kind
/// declaration order is preserved, so the result re-parses to an equal document.
pub fn format_test_spec_document(doc: &TestSpecDocument) -> String {
    let mut blocks: Vec<String> = Vec::new();
    for mock in &doc.mocks {
        blocks.push(format_mock(mock));
    }
    for case in &doc.tests {
        blocks.push(format_test_case(case));
    }
    for prop in &doc.properties {
        blocks.push(format_property(prop));
    }
    for req in &doc.coverage {
        blocks.push(format_coverage(req));
    }
    for snap in &doc.snapshots {
        blocks.push(format_snapshot(snap));
    }
    blocks.join("\n")
}

/// Formats a single `@mock` entity definition.
pub fn format_mock(mock: &MockEntityNode) -> String {
    let mut out = String::new();
    out.push_str("@mock ");
    out.push_str(&ident_or_quoted(&mock.id));
    out.push_str(" {\n");
    for binding in &mock.bindings {
        out.push_str("    ");
        out.push_str(&ident_or_quoted(&binding.key));
        out.push_str(" = ");
        out.push_str(&format_test_value(&binding.value));
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

/// Formats a single `@property` specification.
pub fn format_property(prop: &PropertySpecNode) -> String {
    let mut out = String::new();
    out.push_str("@property ");
    out.push_str(&quote_str(&prop.name));
    out.push_str(" FOR ");
    out.push_str(&ident_or_quoted(&prop.target_statute));
    out.push_str(" {\n");

    for var in &prop.vars {
        out.push_str("    FORALL ");
        out.push_str(&ident_or_quoted(&var.name));
        out.push_str(" IN ");
        format_domain(&mut out, &var.domain);
        out.push('\n');
    }
    if !prop.fixed_bindings.is_empty() {
        out.push_str("    GIVEN ");
        out.push_str(&format_bindings(&prop.fixed_bindings));
        out.push('\n');
    }
    if !prop.uses.is_empty() {
        out.push_str("    USING ");
        let rendered: Vec<String> = prop.uses.iter().map(|id| ident_or_quoted(id)).collect();
        out.push_str(&rendered.join(", "));
        out.push('\n');
    }
    out.push_str("    EXPECT ");
    out.push_str(&format_expectation(&prop.expectation));
    out.push('\n');
    if let Some(cases) = prop.max_cases {
        out.push_str("    CASES ");
        out.push_str(&cases.to_string());
        out.push('\n');
    }

    out.push_str("}\n");
    out
}

/// Appends a property generation domain.
fn format_domain(out: &mut String, domain: &PropertyDomain) {
    match domain {
        PropertyDomain::IntRange { lo, hi } => {
            out.push_str(&lo.to_string());
            out.push_str(" TO ");
            out.push_str(&hi.to_string());
        }
        PropertyDomain::Values(values) => {
            out.push_str("( ");
            let rendered: Vec<String> = values.iter().map(format_test_value).collect();
            out.push_str(&rendered.join(", "));
            out.push_str(" )");
        }
    }
}

/// Formats a single `@coverage` requirement.
pub fn format_coverage(req: &CoverageRequirementNode) -> String {
    let mut out = String::new();
    out.push_str("@coverage REQUIRE ");
    out.push_str(req.metric.keyword());
    out.push(' ');
    out.push_str(req.comparator.symbol());
    out.push(' ');
    out.push_str(&format_percent(req.threshold));
    out.push('%');
    if let Some(target) = &req.target {
        out.push_str(" FOR ");
        out.push_str(&ident_or_quoted(target));
    }
    out.push('\n');
    out
}

/// Renders a percentage, dropping the decimal point for whole numbers.
fn format_percent(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// Formats a single `@snapshot` assertion.
pub fn format_snapshot(snap: &SnapshotAssertionNode) -> String {
    let mut out = String::new();
    out.push_str("@snapshot ");
    out.push_str(&quote_str(&snap.name));
    out.push_str(" FOR ");
    out.push_str(&ident_or_quoted(&snap.target_statute));
    match &snap.mode {
        SnapshotMode::Match(signature) => {
            out.push_str(" EXPECT ");
            out.push_str(&quote_str(signature));
        }
        SnapshotMode::Record => out.push_str(" RECORD"),
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::Effect;

    fn sample_statute() -> Statute {
        Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_format_simple_statute() {
        let statute = sample_statute();
        let output = format_statute(&statute);

        assert!(output.contains("STATUTE adult-rights:"));
        assert!(output.contains("\"Adult Rights Act\""));
        assert!(output.contains("WHEN AGE >= 18"));
        assert!(output.contains("THEN GRANT \"Full legal capacity\""));
    }

    #[test]
    fn test_format_with_discretion() {
        let statute = sample_statute().with_discretion("Consider individual circumstances");
        let output = format_statute(&statute);

        assert!(output.contains("DISCRETION \"Consider individual circumstances\""));
    }

    #[test]
    fn test_format_and_condition() {
        let statute = Statute::new(
            "complex",
            "Complex Statute",
            Effect::new(EffectType::Grant, "Rights"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let output = format_statute(&statute);
        assert!(output.contains("AGE >= 18 AND INCOME < 50000"));
    }

    #[test]
    fn test_format_or_condition() {
        let statute = Statute::new(
            "either",
            "Either Or",
            Effect::new(EffectType::Grant, "Benefits"),
        )
        .with_precondition(Condition::Or(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
            Box::new(Condition::HasAttribute {
                key: "disabled".to_string(),
            }),
        ));

        let output = format_statute(&statute);
        assert!(output.contains("AGE >= 65 OR HAS disabled"));
    }

    #[test]
    fn test_format_not_condition() {
        let statute = Statute::new(
            "exclude",
            "Exclusion",
            Effect::new(EffectType::Grant, "Rights"),
        )
        .with_precondition(Condition::Not(Box::new(Condition::HasAttribute {
            key: "convicted".to_string(),
        })));

        let output = format_statute(&statute);
        assert!(output.contains("NOT HAS convicted"));
    }

    #[test]
    fn test_format_with_jurisdiction() {
        let mut statute = sample_statute();
        statute.jurisdiction = Some("JP".to_string());

        let output = format_statute(&statute);
        assert!(output.contains("JURISDICTION \"JP\""));
    }

    #[test]
    fn test_format_with_version() {
        let mut statute = sample_statute();
        statute.version = 3;

        let output = format_statute(&statute);
        assert!(output.contains("VERSION 3"));
    }

    #[test]
    fn test_format_compact() {
        let printer = DslPrinter::with_config(PrinterConfig::compact());
        let statute = sample_statute();
        let output = printer.format(&statute);

        // Compact uses 2-space indent
        assert!(output.contains("  WHEN"));
    }

    #[test]
    fn test_format_verbose() {
        let printer = DslPrinter::with_config(PrinterConfig::verbose());
        let statute = sample_statute();
        let output = printer.format(&statute);

        // Verbose includes comments
        assert!(output.contains("// Statute:"));
    }

    #[test]
    fn test_format_batch() {
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "another",
                "Another Statute",
                Effect::new(EffectType::Revoke, "Something"),
            ),
        ];

        let output = format_statutes(&statutes);
        assert!(output.contains("adult-rights"));
        assert!(output.contains("another"));
    }

    #[test]
    fn test_roundtrip_simple() {
        let statute = sample_statute();
        let dsl = format_statute(&statute);

        // Parse it back
        let parser = crate::LegalDslParser::new();
        let parsed = parser.parse_statute(&dsl).unwrap();

        assert_eq!(parsed.id, statute.id);
        assert_eq!(parsed.title, statute.title);
    }

    #[test]
    fn test_format_has_with_hyphen() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"))
            .with_precondition(Condition::HasAttribute {
                key: "active-member".to_string(),
            });

        let output = format_statute(&statute);
        assert!(output.contains("HAS \"active-member\""));
    }
}
