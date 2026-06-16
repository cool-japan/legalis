//! Document assembly and contract generation.
//!
//! This sub-module provides a real, dependency-free template engine tuned for
//! legal drafting:
//!
//! * **Variable substitution** - `{{variable}}` is replaced by a typed
//!   [`FieldValue`] rendered to text (with template-level defaults).
//! * **Conditional sections** - `{{#if flag}} ... {{else}} ... {{/if}}` and the
//!   inverse `{{#unless flag}} ... {{/unless}}` include text based on the
//!   truthiness of a value.
//! * **Loops** - `{{#each parties}} ... {{.}} ... {{@index}} ... {{/each}}`
//!   repeats a body over a [`FieldValue::List`], exposing the current item as
//!   `{{.}}` and a 1-based counter as `{{@index}}`.
//! * **Clause libraries** - `{{> governing_law}}` inlines a named
//!   [`ClauseDefinition`] from a [`ClauseLibrary`], itself rendered with the
//!   same context (clauses may reference variables and nest other clauses).
//! * **Validation of required fields** - before rendering, the assembler reports
//!   missing required variables, type mismatches and unknown clause references.
//!
//! [`DocumentAssembler`] drives offline assembly; [`ContractGenerator`] wires a
//! template to a clause library for contract generation. Where a
//! [`crate::LLMProvider`] is available, [`DocumentAssembler::augment`] can
//! *optionally* polish a rendered draft - but it is never required.

use super::{FieldKind, FieldValue, word_count};
use crate::{Jurisdiction, LLMProvider, LegalDocumentType};
use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum clause-inclusion nesting depth (guards against recursive clauses).
const CLAUSE_MAX_DEPTH: usize = 64;

// ============================================================================
// Template AST + parser
// ============================================================================

/// A parsed template segment.
#[derive(Debug, Clone, PartialEq)]
enum Segment {
    /// Literal text emitted verbatim.
    Literal(String),
    /// `{{name}}` variable substitution.
    Variable(String),
    /// `{{> id}}` clause inclusion.
    Clause(String),
    /// `{{#if var}} body {{else}} alt {{/if}}` (or `#unless` when `negated`).
    Conditional {
        var: String,
        negated: bool,
        body: Vec<Segment>,
        alt: Vec<Segment>,
    },
    /// `{{#each var}} body {{/each}}`.
    Each { var: String, body: Vec<Segment> },
}

/// A raw lexical token.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Text(String),
    Tag(String),
}

/// Splits template source into literal/tag tokens.
fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut rest = source;
    loop {
        match rest.find("{{") {
            Some(start) => {
                if start > 0 {
                    tokens.push(Token::Text(rest[..start].to_string()));
                }
                let after = &rest[start + 2..];
                let end = match after.find("}}") {
                    Some(end) => end,
                    None => bail!("unterminated template tag (missing closing braces)"),
                };
                let tag = after[..end].trim().to_string();
                if tag.is_empty() {
                    bail!("empty template tag");
                }
                tokens.push(Token::Tag(tag));
                rest = &after[end + 2..];
            }
            None => {
                if !rest.is_empty() {
                    tokens.push(Token::Text(rest.to_string()));
                }
                break;
            }
        }
    }
    Ok(tokens)
}

/// Recursive-descent template parser.
struct TemplateParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl TemplateParser {
    fn parse_block(&mut self, terminators: &[&str]) -> Result<(Vec<Segment>, Option<String>)> {
        let mut segments = Vec::new();
        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Text(text) => {
                    let literal = text.clone();
                    self.pos += 1;
                    segments.push(Segment::Literal(literal));
                }
                Token::Tag(raw) => {
                    let tag = raw.clone();
                    if terminators.contains(&tag.as_str()) {
                        self.pos += 1;
                        return Ok((segments, Some(tag)));
                    }
                    if let Some(var) = tag.strip_prefix("#if ") {
                        let var = var.trim().to_string();
                        if var.is_empty() {
                            bail!("if-condition requires a variable name");
                        }
                        self.pos += 1;
                        let (body, alt) = self.parse_conditional_arms("/if")?;
                        segments.push(Segment::Conditional {
                            var,
                            negated: false,
                            body,
                            alt,
                        });
                    } else if let Some(var) = tag.strip_prefix("#unless ") {
                        let var = var.trim().to_string();
                        if var.is_empty() {
                            bail!("unless-condition requires a variable name");
                        }
                        self.pos += 1;
                        let (body, alt) = self.parse_conditional_arms("/unless")?;
                        segments.push(Segment::Conditional {
                            var,
                            negated: true,
                            body,
                            alt,
                        });
                    } else if let Some(var) = tag.strip_prefix("#each ") {
                        let var = var.trim().to_string();
                        if var.is_empty() {
                            bail!("each-loop requires a list variable name");
                        }
                        self.pos += 1;
                        let (body, term) = self.parse_block(&["/each"])?;
                        if term.is_none() {
                            bail!("unterminated each-loop");
                        }
                        segments.push(Segment::Each { var, body });
                    } else if let Some(id) = tag.strip_prefix('>') {
                        let id = id.trim().to_string();
                        if id.is_empty() {
                            bail!("clause reference requires an id");
                        }
                        self.pos += 1;
                        segments.push(Segment::Clause(id));
                    } else if tag == "else" || tag.starts_with('/') || tag.starts_with('#') {
                        bail!("unexpected template tag: {}", tag);
                    } else {
                        self.pos += 1;
                        segments.push(Segment::Variable(tag));
                    }
                }
            }
        }
        Ok((segments, None))
    }

    /// Parses the `body` (and optional `else` `alt`) of a conditional, expecting
    /// the supplied closing tag.
    fn parse_conditional_arms(&mut self, close: &str) -> Result<(Vec<Segment>, Vec<Segment>)> {
        let (body, term) = self.parse_block(&["else", close])?;
        let alt = match term.as_deref() {
            Some("else") => {
                let (alt, term2) = self.parse_block(&[close])?;
                if term2.is_none() {
                    bail!("unterminated conditional block");
                }
                alt
            }
            Some(_) => Vec::new(),
            None => bail!("unterminated conditional block"),
        };
        Ok((body, alt))
    }
}

/// Parses a full template source into a segment tree.
fn parse_template(source: &str) -> Result<Vec<Segment>> {
    let tokens = tokenize(source)?;
    let mut parser = TemplateParser { tokens, pos: 0 };
    let (segments, term) = parser.parse_block(&[])?;
    if term.is_some() {
        bail!("dangling closing tag in template");
    }
    Ok(segments)
}

/// Collects referenced variable / clause names from a segment tree.
fn collect_references(
    segments: &[Segment],
    variables: &mut Vec<String>,
    clauses: &mut Vec<String>,
) {
    for segment in segments {
        match segment {
            Segment::Literal(_) => {}
            Segment::Variable(name) => {
                if name != "." && name != "@index" && !variables.contains(name) {
                    variables.push(name.clone());
                }
            }
            Segment::Clause(id) => {
                if !clauses.contains(id) {
                    clauses.push(id.clone());
                }
            }
            Segment::Conditional { var, body, alt, .. } => {
                if var != "." && var != "@index" && !variables.contains(var) {
                    variables.push(var.clone());
                }
                collect_references(body, variables, clauses);
                collect_references(alt, variables, clauses);
            }
            Segment::Each { var, body } => {
                if var != "." && var != "@index" && !variables.contains(var) {
                    variables.push(var.clone());
                }
                collect_references(body, variables, clauses);
            }
        }
    }
}

// ============================================================================
// Renderer
// ============================================================================

/// Resolves a name against the context plus the current loop scope.
fn resolve(
    name: &str,
    context: &HashMap<String, FieldValue>,
    item: Option<&FieldValue>,
    index: usize,
) -> Option<FieldValue> {
    match name {
        "." => item.cloned(),
        "@index" => Some(FieldValue::Integer(index as i64)),
        _ => context.get(name).cloned(),
    }
}

/// Renders a segment tree into `out`.
#[allow(clippy::too_many_arguments)]
fn render_segments(
    segments: &[Segment],
    context: &HashMap<String, FieldValue>,
    item: Option<&FieldValue>,
    index: usize,
    clauses: &ClauseLibrary,
    depth: usize,
    out: &mut String,
) -> Result<()> {
    if depth > CLAUSE_MAX_DEPTH {
        bail!("clause inclusion nested too deeply (possible cycle)");
    }
    for segment in segments {
        match segment {
            Segment::Literal(text) => out.push_str(text),
            Segment::Variable(name) => match resolve(name, context, item, index) {
                Some(value) => out.push_str(&value.render()),
                None => bail!("unresolved variable: {}", name),
            },
            Segment::Clause(id) => {
                let clause = match clauses.get(id) {
                    Some(clause) => clause,
                    None => bail!("unknown clause: {}", id),
                };
                let clause_segments = parse_template(&clause.body)?;
                render_segments(
                    &clause_segments,
                    context,
                    item,
                    index,
                    clauses,
                    depth + 1,
                    out,
                )?;
            }
            Segment::Conditional {
                var,
                negated,
                body,
                alt,
            } => {
                let truthy = resolve(var, context, item, index)
                    .map(|value| value.is_truthy())
                    .unwrap_or(false);
                let branch = if truthy ^ *negated { body } else { alt };
                render_segments(branch, context, item, index, clauses, depth, out)?;
            }
            Segment::Each { var, body } => match resolve(var, context, item, index) {
                Some(FieldValue::List(items)) => {
                    for (offset, list_item) in items.iter().enumerate() {
                        render_segments(
                            body,
                            context,
                            Some(list_item),
                            offset + 1,
                            clauses,
                            depth,
                            out,
                        )?;
                    }
                }
                Some(scalar) => {
                    render_segments(body, context, Some(&scalar), 1, clauses, depth, out)?;
                }
                None => {}
            },
        }
    }
    Ok(())
}

// ============================================================================
// Variable specification
// ============================================================================

/// Declares a variable expected by a [`DocumentTemplate`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableSpec {
    /// Variable name (matches `{{name}}` in the template body).
    pub name: String,
    /// Expected value kind.
    pub kind: FieldKind,
    /// Whether a value (or default) must be present to assemble.
    pub required: bool,
    /// Default value used when the caller omits the variable.
    pub default: Option<FieldValue>,
    /// Optional human-readable description.
    pub description: Option<String>,
}

impl VariableSpec {
    /// Creates a required variable specification.
    pub fn required(name: impl Into<String>, kind: FieldKind) -> Self {
        Self {
            name: name.into(),
            kind,
            required: true,
            default: None,
            description: None,
        }
    }

    /// Creates an optional variable specification.
    pub fn optional(name: impl Into<String>, kind: FieldKind) -> Self {
        Self {
            name: name.into(),
            kind,
            required: false,
            default: None,
            description: None,
        }
    }

    /// Sets a default value (and marks the variable as satisfiable without
    /// caller input).
    pub fn with_default(mut self, value: FieldValue) -> Self {
        self.default = Some(value);
        self
    }

    /// Sets a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns whether this specification is satisfied without caller input
    /// (i.e. optional or carrying a default).
    pub fn is_self_satisfied(&self) -> bool {
        !self.required || self.default.is_some()
    }
}

// ============================================================================
// Document template
// ============================================================================

/// A reusable legal document template.
#[derive(Debug, Clone)]
pub struct DocumentTemplate {
    /// Stable identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Document classification (reuses the crate's [`LegalDocumentType`]).
    pub document_type: LegalDocumentType,
    /// Raw template body.
    pub body: String,
    /// Declared variables.
    pub variables: Vec<VariableSpec>,
    /// Optional jurisdiction the template targets.
    pub jurisdiction: Option<Jurisdiction>,
    /// Optional description.
    pub description: Option<String>,
}

impl DocumentTemplate {
    /// Creates a new template.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        document_type: LegalDocumentType,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            document_type,
            body: body.into(),
            variables: Vec::new(),
            jurisdiction: None,
            description: None,
        }
    }

    /// Declares a variable.
    pub fn with_variable(mut self, spec: VariableSpec) -> Self {
        self.variables.push(spec);
        self
    }

    /// Declares a required variable (convenience).
    pub fn requiring(self, name: impl Into<String>, kind: FieldKind) -> Self {
        self.with_variable(VariableSpec::required(name, kind))
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the specification for a named variable.
    pub fn spec(&self, name: &str) -> Option<&VariableSpec> {
        self.variables.iter().find(|spec| spec.name == name)
    }

    /// Validates that the template body parses (without rendering).
    pub fn validate_syntax(&self) -> Result<()> {
        parse_template(&self.body)?;
        Ok(())
    }

    /// Returns the variable names referenced by the body.
    pub fn referenced_variables(&self) -> Result<Vec<String>> {
        let segments = parse_template(&self.body)?;
        let mut variables = Vec::new();
        let mut clauses = Vec::new();
        collect_references(&segments, &mut variables, &mut clauses);
        variables.sort();
        Ok(variables)
    }

    /// Returns the clause ids referenced by the body.
    pub fn referenced_clauses(&self) -> Result<Vec<String>> {
        let segments = parse_template(&self.body)?;
        let mut variables = Vec::new();
        let mut clauses = Vec::new();
        collect_references(&segments, &mut variables, &mut clauses);
        clauses.sort();
        Ok(clauses)
    }

    /// Builds the default context contributed by variable specifications.
    pub fn default_context(&self) -> HashMap<String, FieldValue> {
        let mut context = HashMap::new();
        for spec in &self.variables {
            if let Some(default) = &spec.default {
                context.insert(spec.name.clone(), default.clone());
            }
        }
        context
    }
}

// ============================================================================
// Clause library
// ============================================================================

/// A reusable, named clause that can be inlined into templates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClauseDefinition {
    /// Stable identifier referenced by `{{> id}}`.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Category / grouping (e.g. `boilerplate`, `payment`).
    pub category: String,
    /// Clause body (itself a template).
    pub body: String,
    /// Optional jurisdiction the clause is appropriate for.
    pub jurisdiction: Option<Jurisdiction>,
    /// Topical tags.
    pub tags: Vec<String>,
}

impl ClauseDefinition {
    /// Creates a clause.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        category: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category: category.into(),
            body: body.into(),
            jurisdiction: None,
            tags: Vec::new(),
        }
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// A library of reusable clauses keyed by id.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClauseLibrary {
    clauses: HashMap<String, ClauseDefinition>,
}

impl ClauseLibrary {
    /// Creates an empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a clause (builder style).
    pub fn with_clause(mut self, clause: ClauseDefinition) -> Self {
        self.add(clause);
        self
    }

    /// Adds or replaces a clause.
    pub fn add(&mut self, clause: ClauseDefinition) {
        self.clauses.insert(clause.id.clone(), clause);
    }

    /// Returns a clause by id.
    pub fn get(&self, id: &str) -> Option<&ClauseDefinition> {
        self.clauses.get(id)
    }

    /// Returns whether the library contains a clause.
    pub fn contains(&self, id: &str) -> bool {
        self.clauses.contains_key(id)
    }

    /// Returns the number of clauses.
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    /// Returns whether the library is empty.
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    /// Returns all clause ids (sorted).
    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.clauses.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Returns clauses in a category (sorted by id).
    pub fn by_category(&self, category: &str) -> Vec<&ClauseDefinition> {
        let mut matches: Vec<&ClauseDefinition> = self
            .clauses
            .values()
            .filter(|clause| clause.category == category)
            .collect();
        matches.sort_by(|a, b| a.id.cmp(&b.id));
        matches
    }

    /// Returns clauses appropriate for a jurisdiction (sorted by id).
    pub fn by_jurisdiction(&self, jurisdiction: &Jurisdiction) -> Vec<&ClauseDefinition> {
        let mut matches: Vec<&ClauseDefinition> = self
            .clauses
            .values()
            .filter(|clause| clause.jurisdiction.as_ref() == Some(jurisdiction))
            .collect();
        matches.sort_by(|a, b| a.id.cmp(&b.id));
        matches
    }

    /// Case-insensitive keyword search over title, body, category and tags.
    pub fn search(&self, keyword: &str) -> Vec<&ClauseDefinition> {
        let needle = keyword.to_lowercase();
        let mut matches: Vec<&ClauseDefinition> = self
            .clauses
            .values()
            .filter(|clause| {
                clause.title.to_lowercase().contains(&needle)
                    || clause.body.to_lowercase().contains(&needle)
                    || clause.category.to_lowercase().contains(&needle)
                    || clause
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&needle))
            })
            .collect();
        matches.sort_by(|a, b| a.id.cmp(&b.id));
        matches
    }

    /// Builds a library of common contract boilerplate clauses.
    pub fn standard() -> Self {
        let mut library = Self::new();
        library.add(ClauseDefinition::new(
            "governing_law",
            "Governing Law",
            "boilerplate",
            "This Agreement shall be governed by and construed in accordance with the laws of \
             {{governing_law}}, without regard to its conflict-of-laws principles.",
        ));
        library.add(ClauseDefinition::new(
            "entire_agreement",
            "Entire Agreement",
            "boilerplate",
            "This Agreement constitutes the entire agreement between the parties and supersedes \
             all prior negotiations, representations and agreements, whether written or oral.",
        ));
        library.add(ClauseDefinition::new(
            "severability",
            "Severability",
            "boilerplate",
            "If any provision of this Agreement is held to be invalid or unenforceable, the \
             remaining provisions shall continue in full force and effect.",
        ));
        library.add(ClauseDefinition::new(
            "confidentiality",
            "Confidentiality",
            "protection",
            "Each party shall keep confidential all non-public information disclosed by the other \
             party and shall not use such information except as necessary to perform this \
             Agreement.",
        ));
        library.add(ClauseDefinition::new(
            "notices",
            "Notices",
            "boilerplate",
            "All notices under this Agreement shall be in writing and delivered to the addresses \
             set forth above or to such other address as a party may designate in writing.",
        ));
        library.add(ClauseDefinition::new(
            "force_majeure",
            "Force Majeure",
            "risk",
            "Neither party shall be liable for any failure or delay in performance caused by \
             events beyond its reasonable control, including acts of God, war and governmental \
             action.",
        ));
        library
    }
}

// ============================================================================
// Validation + assembled output
// ============================================================================

/// A type mismatch between a provided value and its variable specification.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeMismatch {
    /// Variable name.
    pub variable: String,
    /// Expected kind.
    pub expected: FieldKind,
    /// Actual kind supplied.
    pub actual: FieldKind,
}

/// The outcome of pre-assembly validation.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AssemblyValidation {
    /// Required variables with neither a value nor a default.
    pub missing_required: Vec<String>,
    /// Variables supplied with the wrong value kind.
    pub type_mismatches: Vec<TypeMismatch>,
    /// Clauses referenced by the template but absent from the library.
    pub unknown_clauses: Vec<String>,
}

impl AssemblyValidation {
    /// Returns whether assembly can proceed.
    pub fn is_valid(&self) -> bool {
        self.missing_required.is_empty()
            && self.type_mismatches.is_empty()
            && self.unknown_clauses.is_empty()
    }

    /// Returns a flat list of human-readable issue messages.
    pub fn issues(&self) -> Vec<String> {
        let mut issues = Vec::new();
        for name in &self.missing_required {
            issues.push(format!("missing required variable '{}'", name));
        }
        for mismatch in &self.type_mismatches {
            issues.push(format!(
                "variable '{}' expected {} but got {}",
                mismatch.variable,
                mismatch.expected.label(),
                mismatch.actual.label()
            ));
        }
        for id in &self.unknown_clauses {
            issues.push(format!("unknown clause '{}'", id));
        }
        issues
    }
}

/// A fully assembled document.
#[derive(Debug, Clone)]
pub struct AssembledDocument {
    /// Source template id.
    pub template_id: String,
    /// Document title.
    pub title: String,
    /// Document classification.
    pub document_type: LegalDocumentType,
    /// Rendered body.
    pub body: String,
    /// Variables that were resolved during rendering (sorted).
    pub variables_resolved: Vec<String>,
    /// Clauses that were included (sorted).
    pub clauses_included: Vec<String>,
    /// Word count of the rendered body.
    pub word_count: usize,
    /// Timestamp the document was generated.
    pub generated_at: DateTime<Utc>,
}

// ============================================================================
// Assembler
// ============================================================================

/// Assembles documents from templates using an attached clause library.
#[derive(Debug, Clone, Default)]
pub struct DocumentAssembler {
    clauses: ClauseLibrary,
}

impl DocumentAssembler {
    /// Creates an assembler with an empty clause library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an assembler backed by the supplied clause library.
    pub fn with_clause_library(clauses: ClauseLibrary) -> Self {
        Self { clauses }
    }

    /// Returns the clause library.
    pub fn clause_library(&self) -> &ClauseLibrary {
        &self.clauses
    }

    /// Returns the clause library mutably.
    pub fn clause_library_mut(&mut self) -> &mut ClauseLibrary {
        &mut self.clauses
    }

    /// Validates a context against a template prior to assembly.
    pub fn validate(
        &self,
        template: &DocumentTemplate,
        context: &HashMap<String, FieldValue>,
    ) -> AssemblyValidation {
        let mut validation = AssemblyValidation::default();

        for spec in &template.variables {
            if spec.required && spec.default.is_none() && !context.contains_key(&spec.name) {
                validation.missing_required.push(spec.name.clone());
            }
        }

        for (name, value) in context {
            let Some(spec) = template.spec(name) else {
                continue;
            };
            if !kind_compatible(spec.kind, value.kind()) {
                validation.type_mismatches.push(TypeMismatch {
                    variable: name.clone(),
                    expected: spec.kind,
                    actual: value.kind(),
                });
            }
        }

        if let Ok(referenced) = template.referenced_clauses() {
            for id in referenced {
                if !self.clauses.contains(&id) {
                    validation.unknown_clauses.push(id);
                }
            }
        }

        validation.missing_required.sort();
        validation
            .type_mismatches
            .sort_by(|a, b| a.variable.cmp(&b.variable));
        validation.unknown_clauses.sort();
        validation
    }

    /// Assembles a template with the provided context.
    pub fn assemble(
        &self,
        template: &DocumentTemplate,
        context: &HashMap<String, FieldValue>,
    ) -> Result<AssembledDocument> {
        template.validate_syntax()?;

        let validation = self.validate(template, context);
        if !validation.is_valid() {
            bail!(
                "template '{}' cannot be assembled: {}",
                template.id,
                validation.issues().join("; ")
            );
        }

        let mut merged = template.default_context();
        for (name, value) in context {
            merged.insert(name.clone(), value.clone());
        }

        let segments = parse_template(&template.body)?;
        let mut body = String::new();
        render_segments(&segments, &merged, None, 0, &self.clauses, 0, &mut body)?;

        let referenced_variables = template.referenced_variables()?;
        let mut variables_resolved: Vec<String> = referenced_variables
            .into_iter()
            .filter(|name| merged.contains_key(name))
            .collect();
        variables_resolved.sort();

        let clauses_included = template.referenced_clauses()?;

        Ok(AssembledDocument {
            template_id: template.id.clone(),
            title: template.title.clone(),
            document_type: template.document_type,
            word_count: word_count(&body),
            body,
            variables_resolved,
            clauses_included,
            generated_at: Utc::now(),
        })
    }

    /// Optionally polishes an assembled draft with an LLM provider.
    ///
    /// This is the only method in the module that requires a live provider; all
    /// assembly works offline. The rendered body is sent to the provider with an
    /// instruction to refine the prose without changing the legal substance.
    pub async fn augment<P: LLMProvider>(
        &self,
        document: &AssembledDocument,
        instruction: &str,
        provider: &P,
    ) -> Result<String> {
        let prompt = format!(
            "You are a senior legal drafter. Improve the following assembled document while \
             preserving every defined term, party name, figure and clause. Do not introduce new \
             obligations. Instruction: {}\n\n---\n{}\n---\n\nReturn the revised document only.",
            instruction, document.body
        );
        provider.generate_text(&prompt).await
    }
}

/// Returns whether a supplied value kind satisfies the declared kind.
///
/// An integer is accepted where a decimal is expected (widening), otherwise the
/// kinds must match exactly.
fn kind_compatible(expected: FieldKind, actual: FieldKind) -> bool {
    expected == actual || (expected == FieldKind::Decimal && actual == FieldKind::Integer)
}

// ============================================================================
// Contract generation
// ============================================================================

/// Generates contracts by pairing a template with a clause library.
#[derive(Debug, Clone)]
pub struct ContractGenerator {
    template: DocumentTemplate,
    assembler: DocumentAssembler,
}

impl ContractGenerator {
    /// Creates a generator for the supplied template (empty clause library).
    pub fn new(template: DocumentTemplate) -> Self {
        Self {
            template,
            assembler: DocumentAssembler::new(),
        }
    }

    /// Attaches a clause library.
    pub fn with_clause_library(mut self, clauses: ClauseLibrary) -> Self {
        self.assembler = DocumentAssembler::with_clause_library(clauses);
        self
    }

    /// Returns the underlying template.
    pub fn template(&self) -> &DocumentTemplate {
        &self.template
    }

    /// Returns the underlying assembler.
    pub fn assembler(&self) -> &DocumentAssembler {
        &self.assembler
    }

    /// Validates a context against the contract template.
    pub fn validate(&self, context: &HashMap<String, FieldValue>) -> AssemblyValidation {
        self.assembler.validate(&self.template, context)
    }

    /// Generates the contract for the supplied context.
    pub fn generate(&self, context: &HashMap<String, FieldValue>) -> Result<AssembledDocument> {
        self.assembler.assemble(&self.template, context)
    }

    /// Builds a ready-to-use service-agreement generator with standard clauses.
    ///
    /// The template requires `provider`, `client`, `effective_date`, `services`,
    /// `fee` and `term_months`, optionally toggles a confidentiality clause via
    /// the `confidential` flag, and inlines boilerplate clauses from the
    /// [`ClauseLibrary::standard`] library.
    pub fn standard_service_agreement() -> Self {
        let body = "SERVICES AGREEMENT\n\n\
            This Services Agreement (the \"Agreement\") is entered into as of {{effective_date}} \
            by and between {{provider}} (\"Provider\") and {{client}} (\"Client\").\n\n\
            1. SERVICES. Provider shall provide the following services:\n{{#each services}}  ({{@index}}) {{.}}\n{{/each}}\n\
            2. TERM. This Agreement shall remain in effect for {{term_months}} months from the \
            Effective Date.\n\n\
            3. FEES. Client shall pay Provider a fee of {{fee}}.\n\n\
            4. GOVERNING LAW. {{> governing_law}}\n\n\
            {{#if confidential}}5. CONFIDENTIALITY. {{> confidentiality}}\n\n{{/if}}\
            6. ENTIRE AGREEMENT. {{> entire_agreement}}\n\n\
            7. SEVERABILITY. {{> severability}}\n"
            .to_string();

        let template = DocumentTemplate::new(
            "service_agreement",
            "Services Agreement",
            LegalDocumentType::Contract,
            body,
        )
        .requiring("provider", FieldKind::Text)
        .requiring("client", FieldKind::Text)
        .requiring("effective_date", FieldKind::Date)
        .requiring("services", FieldKind::List)
        .requiring("fee", FieldKind::Text)
        .requiring("term_months", FieldKind::Integer)
        .with_variable(
            VariableSpec::optional("confidential", FieldKind::Boolean)
                .with_default(FieldValue::Boolean(true)),
        )
        .with_variable(
            VariableSpec::optional("governing_law", FieldKind::Text)
                .with_default(FieldValue::text("the State of Delaware")),
        );

        Self::new(template).with_clause_library(ClauseLibrary::standard())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextStream;
    use async_trait::async_trait;
    use chrono::NaiveDate;
    use serde::de::DeserializeOwned;

    fn context(pairs: &[(&str, FieldValue)]) -> HashMap<String, FieldValue> {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }

    #[test]
    fn test_variable_substitution_with_defaults() {
        let template = DocumentTemplate::new(
            "letter",
            "Demand Letter",
            LegalDocumentType::Brief,
            "Dear {{recipient}}, you owe {{amount}}. Regards, {{sender}}.",
        )
        .requiring("recipient", FieldKind::Text)
        .requiring("amount", FieldKind::Text)
        .with_variable(
            VariableSpec::optional("sender", FieldKind::Text)
                .with_default(FieldValue::text("Acme Legal")),
        );

        let assembler = DocumentAssembler::new();
        let doc = assembler
            .assemble(
                &template,
                &context(&[
                    ("recipient", FieldValue::text("Jane Roe")),
                    ("amount", FieldValue::text("$500")),
                ]),
            )
            .expect("assembles");
        assert_eq!(
            doc.body,
            "Dear Jane Roe, you owe $500. Regards, Acme Legal."
        );
        assert!(doc.variables_resolved.contains(&"sender".to_string()));
        assert!(doc.word_count > 0);
    }

    #[test]
    fn test_conditional_sections() {
        let template = DocumentTemplate::new(
            "notice",
            "Notice",
            LegalDocumentType::Brief,
            "{{#if urgent}}URGENT: {{/if}}{{#unless paid}}Payment outstanding.{{else}}Paid.{{/unless}}",
        )
        .with_variable(VariableSpec::optional("urgent", FieldKind::Boolean))
        .with_variable(VariableSpec::optional("paid", FieldKind::Boolean));

        let assembler = DocumentAssembler::new();
        let body = assembler
            .assemble(
                &template,
                &context(&[
                    ("urgent", FieldValue::boolean(true)),
                    ("paid", FieldValue::boolean(false)),
                ]),
            )
            .expect("assembles")
            .body;
        assert_eq!(body, "URGENT: Payment outstanding.");

        let body2 = assembler
            .assemble(&template, &context(&[("paid", FieldValue::boolean(true))]))
            .expect("assembles")
            .body;
        assert_eq!(body2, "Paid.");
    }

    #[test]
    fn test_each_loop_with_item_and_index() {
        let template = DocumentTemplate::new(
            "list",
            "Schedule",
            LegalDocumentType::Brief,
            "Items:\n{{#each items}}{{@index}}. {{.}}\n{{/each}}End",
        )
        .with_variable(VariableSpec::optional("items", FieldKind::List));

        let assembler = DocumentAssembler::new();
        let body = assembler
            .assemble(
                &template,
                &context(&[(
                    "items",
                    FieldValue::list([FieldValue::text("Alpha"), FieldValue::text("Beta")]),
                )]),
            )
            .expect("assembles")
            .body;
        assert_eq!(body, "Items:\n1. Alpha\n2. Beta\nEnd");
    }

    #[test]
    fn test_clause_partial_inclusion() {
        let library = ClauseLibrary::standard();
        assert!(library.len() >= 6);
        assert!(!library.by_category("boilerplate").is_empty());
        assert!(!library.search("confidential").is_empty());

        let template = DocumentTemplate::new(
            "agreement",
            "Mini Agreement",
            LegalDocumentType::Contract,
            "Law: {{> governing_law}}",
        )
        .with_variable(VariableSpec::optional("governing_law", FieldKind::Text));

        let assembler = DocumentAssembler::with_clause_library(library);
        let body = assembler
            .assemble(
                &template,
                &context(&[("governing_law", FieldValue::text("New York"))]),
            )
            .expect("assembles")
            .body;
        assert!(body.contains("governed by"));
        assert!(body.contains("New York"));
    }

    #[test]
    fn test_validation_detects_problems() {
        let template = DocumentTemplate::new(
            "bad",
            "Bad",
            LegalDocumentType::Contract,
            "{{name}} owes {{amount}} {{> missing_clause}}",
        )
        .requiring("name", FieldKind::Text)
        .requiring("amount", FieldKind::Integer);

        let assembler = DocumentAssembler::new();
        let validation =
            assembler.validate(&template, &context(&[("amount", FieldValue::text("oops"))]));
        assert!(!validation.is_valid());
        assert!(validation.missing_required.contains(&"name".to_string()));
        assert_eq!(validation.type_mismatches.len(), 1);
        assert!(
            validation
                .unknown_clauses
                .contains(&"missing_clause".to_string())
        );

        let err = assembler
            .assemble(&template, &context(&[("amount", FieldValue::text("oops"))]))
            .unwrap_err();
        assert!(err.to_string().contains("cannot be assembled"));
    }

    #[test]
    fn test_syntax_error_detection() {
        let template = DocumentTemplate::new(
            "broken",
            "Broken",
            LegalDocumentType::Brief,
            "Hello {{#if x}}world",
        );
        assert!(template.validate_syntax().is_err());
    }

    #[test]
    fn test_standard_service_agreement_generation() {
        let generator = ContractGenerator::standard_service_agreement();
        let doc = generator
            .generate(&context(&[
                ("provider", FieldValue::text("Acme LLP")),
                ("client", FieldValue::text("Globex Inc.")),
                (
                    "effective_date",
                    FieldValue::date(NaiveDate::from_ymd_opt(2026, 6, 14).expect("valid")),
                ),
                (
                    "services",
                    FieldValue::list([
                        FieldValue::text("Legal advisory"),
                        FieldValue::text("Contract review"),
                    ]),
                ),
                ("fee", FieldValue::text("$10,000")),
                ("term_months", FieldValue::integer(12)),
            ]))
            .expect("generates");

        assert_eq!(doc.document_type, LegalDocumentType::Contract);
        assert!(doc.body.contains("Acme LLP"));
        assert!(doc.body.contains("Globex Inc."));
        assert!(doc.body.contains("(1) Legal advisory"));
        assert!(doc.body.contains("(2) Contract review"));
        assert!(doc.body.contains("12 months"));
        // confidential defaults to true -> clause present
        assert!(doc.body.contains("CONFIDENTIALITY"));
        // governing_law default applied
        assert!(doc.body.contains("Delaware"));
        assert!(doc.clauses_included.contains(&"governing_law".to_string()));
    }

    struct MockProvider;

    #[async_trait]
    impl LLMProvider for MockProvider {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            Ok(format!("POLISHED ({} chars)", prompt.len()))
        }
        async fn generate_structured<T: DeserializeOwned + Send>(&self, _p: &str) -> Result<T> {
            anyhow::bail!("unsupported")
        }
        async fn generate_text_stream(&self, _p: &str) -> Result<TextStream> {
            anyhow::bail!("unsupported")
        }
        fn provider_name(&self) -> &str {
            "mock"
        }
        fn model_name(&self) -> &str {
            "mock"
        }
    }

    #[tokio::test]
    async fn test_optional_llm_augmentation() {
        let generator = ContractGenerator::standard_service_agreement();
        let doc = generator
            .generate(&context(&[
                ("provider", FieldValue::text("A")),
                ("client", FieldValue::text("B")),
                (
                    "effective_date",
                    FieldValue::date(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid")),
                ),
                ("services", FieldValue::list([FieldValue::text("x")])),
                ("fee", FieldValue::text("$1")),
                ("term_months", FieldValue::integer(6)),
            ]))
            .expect("generates");
        let augmented = generator
            .assembler()
            .augment(&doc, "tighten the prose", &MockProvider)
            .await
            .expect("augments");
        assert!(augmented.starts_with("POLISHED"));
    }
}
