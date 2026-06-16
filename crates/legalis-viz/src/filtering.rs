//! Interactive filtering: a composable, serializable filter model with a
//! multi-criteria filter-panel descriptor.
//!
//! Where the rest of the crate *renders* legal structures, this module *selects*
//! which parts of them to show. It provides a small, dependency-free filtering
//! engine that operates on a neutral [`FilterableRecord`] view of any data set,
//! plus adapters that derive those records from the crate's existing
//! [`Timeline`] and [`DependencyGraph`] models.
//!
//! The pieces are:
//!
//! - [`FilterableRecord`] — a flat, queryable projection of one item (text
//!   fields, tags, an optional ISO-8601 date and arbitrary attributes).
//! - [`FilterCriterion`] — a single predicate (text-contains, tag membership,
//!   date range, attribute match).
//! - [`FilterExpr`] — criteria combined with `AND` / `OR` / `NOT`, evaluated by
//!   [`FilterExpr::matches`] and applied with [`FilterExpr::apply`].
//! - [`DateRange`] — an inclusive, half-open-capable date window using
//!   lexical comparison of ISO-8601 (`YYYY-MM-DD`) strings.
//! - [`FilterPreset`] / [`FilterPresetLibrary`] — named, JSON-round-trippable
//!   saved filters.
//! - [`FilterPanel`] — an exportable HTML + JavaScript multi-criteria filtering
//!   UI whose client-side logic mirrors this engine's semantics.
//!
//! [`Timeline`]: crate::Timeline
//! [`DependencyGraph`]: crate::DependencyGraph

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::data_exchange::escape_xml;
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use crate::{VizError, VizResult};

// ===========================================================================
// Filterable record
// ===========================================================================

/// A flat, queryable projection of a single data item.
///
/// Records are intentionally schema-light: text fields are searched by
/// [`FilterCriterion::TextContains`], `tags` by the tag criteria, `date` by
/// [`FilterCriterion::DateInRange`] and `attributes` by the attribute criteria.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterableRecord {
    /// Stable identifier (used to map back to a rendered element).
    pub id: String,
    /// Named, free-text searchable fields.
    pub text_fields: BTreeMap<String, String>,
    /// Categorical tags.
    pub tags: BTreeSet<String>,
    /// Optional ISO-8601 (`YYYY-MM-DD`) date for range filtering.
    pub date: Option<String>,
    /// Arbitrary key/value attributes for exact-match filtering.
    pub attributes: BTreeMap<String, String>,
}

impl FilterableRecord {
    /// Creates a record with the given id and no fields.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Self::default()
        }
    }

    /// Adds a named text field.
    pub fn with_text(mut self, field: &str, value: &str) -> Self {
        self.text_fields
            .insert(field.to_string(), value.to_string());
        self
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_string());
        self
    }

    /// Sets the date.
    pub fn with_date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    /// Adds an attribute.
    pub fn with_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    /// Returns true if any text field (or the named one) contains `needle`.
    fn text_contains(&self, field: Option<&str>, needle: &str, case_insensitive: bool) -> bool {
        let hay_matches = |hay: &str| {
            if case_insensitive {
                hay.to_lowercase().contains(&needle.to_lowercase())
            } else {
                hay.contains(needle)
            }
        };
        match field {
            Some(name) => self.text_fields.get(name).is_some_and(|v| hay_matches(v)),
            None => self.text_fields.values().any(|v| hay_matches(v)),
        }
    }
}

// ===========================================================================
// Date range
// ===========================================================================

/// An inclusive date window over ISO-8601 (`YYYY-MM-DD`) strings.
///
/// Comparison is lexical, which is correct for zero-padded ISO-8601 dates (the
/// same convention [`Timeline`] uses for sorting). Either bound may be omitted
/// for an open-ended range.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    /// Inclusive lower bound, or `None` for unbounded below.
    pub start: Option<String>,
    /// Inclusive upper bound, or `None` for unbounded above.
    pub end: Option<String>,
}

impl DateRange {
    /// An unbounded range (matches every date).
    pub fn new() -> Self {
        Self::default()
    }

    /// A range from `start` onward (inclusive).
    pub fn since(start: &str) -> Self {
        Self {
            start: Some(start.to_string()),
            end: None,
        }
    }

    /// A range up to `end` (inclusive).
    pub fn until(end: &str) -> Self {
        Self {
            start: None,
            end: Some(end.to_string()),
        }
    }

    /// A closed range `[start, end]` (inclusive on both ends).
    pub fn between(start: &str, end: &str) -> Self {
        Self {
            start: Some(start.to_string()),
            end: Some(end.to_string()),
        }
    }

    /// Returns true if `date` lies within the (inclusive) range.
    pub fn contains(&self, date: &str) -> bool {
        if self.start.as_deref().is_some_and(|start| date < start) {
            return false;
        }
        if self.end.as_deref().is_some_and(|end| date > end) {
            return false;
        }
        true
    }

    /// A compact human-readable description such as `[2000-01-01, 2010-12-31]`.
    pub fn describe(&self) -> String {
        let lower = self.start.as_deref().unwrap_or("-∞");
        let upper = self.end.as_deref().unwrap_or("+∞");
        format!("[{}, {}]", lower, upper)
    }
}

// ===========================================================================
// Criteria
// ===========================================================================

/// A single filter predicate over a [`FilterableRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterCriterion {
    /// The record id equals the given value.
    IdEquals(String),
    /// A text field (or any, when `field` is `None`) contains a substring.
    TextContains {
        /// Field name, or `None` to search all text fields.
        field: Option<String>,
        /// Substring to look for.
        needle: String,
        /// Whether the match ignores ASCII/Unicode case.
        case_insensitive: bool,
    },
    /// The record carries at least one of these tags.
    TagAny(BTreeSet<String>),
    /// The record carries all of these tags.
    TagAll(BTreeSet<String>),
    /// The record's date falls within the range.
    DateInRange(DateRange),
    /// An attribute equals a value.
    AttributeEquals {
        /// Attribute key.
        key: String,
        /// Required value.
        value: String,
    },
    /// An attribute's value is one of a set.
    AttributeOneOf {
        /// Attribute key.
        key: String,
        /// Allowed values.
        values: BTreeSet<String>,
    },
}

impl FilterCriterion {
    /// Evaluates the criterion against a record.
    pub fn matches(&self, record: &FilterableRecord) -> bool {
        match self {
            FilterCriterion::IdEquals(id) => record.id == *id,
            FilterCriterion::TextContains {
                field,
                needle,
                case_insensitive,
            } => record.text_contains(field.as_deref(), needle, *case_insensitive),
            FilterCriterion::TagAny(tags) => tags.iter().any(|t| record.tags.contains(t)),
            FilterCriterion::TagAll(tags) => tags.iter().all(|t| record.tags.contains(t)),
            FilterCriterion::DateInRange(range) => {
                record.date.as_deref().is_some_and(|d| range.contains(d))
            }
            FilterCriterion::AttributeEquals { key, value } => {
                record.attributes.get(key) == Some(value)
            }
            FilterCriterion::AttributeOneOf { key, values } => record
                .attributes
                .get(key)
                .is_some_and(|v| values.contains(v)),
        }
    }

    /// A human-readable rendering of the criterion.
    pub fn describe(&self) -> String {
        match self {
            FilterCriterion::IdEquals(id) => format!("id = \"{}\"", id),
            FilterCriterion::TextContains {
                field,
                needle,
                case_insensitive,
            } => {
                let scope = field.as_deref().unwrap_or("any text");
                let ci = if *case_insensitive { " (ci)" } else { "" };
                format!("{} contains \"{}\"{}", scope, needle, ci)
            }
            FilterCriterion::TagAny(tags) => {
                format!("tag in {{{}}}", join_sorted(tags))
            }
            FilterCriterion::TagAll(tags) => {
                format!("has all tags {{{}}}", join_sorted(tags))
            }
            FilterCriterion::DateInRange(range) => format!("date in {}", range.describe()),
            FilterCriterion::AttributeEquals { key, value } => {
                format!("{} = \"{}\"", key, value)
            }
            FilterCriterion::AttributeOneOf { key, values } => {
                format!("{} in {{{}}}", key, join_sorted(values))
            }
        }
    }
}

fn join_sorted(set: &BTreeSet<String>) -> String {
    set.iter().cloned().collect::<Vec<_>>().join(", ")
}

// ===========================================================================
// Filter expression (AND / OR / NOT)
// ===========================================================================

/// Boolean combinator for joining filter expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Combinator {
    /// Logical conjunction.
    And,
    /// Logical disjunction.
    Or,
}

impl Combinator {
    /// The uppercase keyword (`"AND"` / `"OR"`).
    pub fn keyword(&self) -> &'static str {
        match self {
            Combinator::And => "AND",
            Combinator::Or => "OR",
        }
    }
}

/// A composable filter expression evaluated over [`FilterableRecord`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterExpr {
    /// Matches every record (identity for `AND`).
    Always,
    /// Matches no record (identity for `OR`).
    Never,
    /// A single criterion.
    Criterion(FilterCriterion),
    /// Conjunction of sub-expressions.
    And(Vec<FilterExpr>),
    /// Disjunction of sub-expressions.
    Or(Vec<FilterExpr>),
    /// Negation of a sub-expression.
    Not(Box<FilterExpr>),
}

impl FilterExpr {
    /// Wraps a single criterion.
    pub fn criterion(criterion: FilterCriterion) -> Self {
        FilterExpr::Criterion(criterion)
    }

    /// Builds a conjunction from sub-expressions.
    pub fn all(exprs: Vec<FilterExpr>) -> Self {
        FilterExpr::And(exprs)
    }

    /// Builds a disjunction from sub-expressions.
    pub fn any(exprs: Vec<FilterExpr>) -> Self {
        FilterExpr::Or(exprs)
    }

    /// Combines two expressions with `self AND other`.
    pub fn and(self, other: FilterExpr) -> Self {
        FilterExpr::And(vec![self, other])
    }

    /// Combines two expressions with `self OR other`.
    pub fn or(self, other: FilterExpr) -> Self {
        FilterExpr::Or(vec![self, other])
    }

    /// Negates this expression.
    pub fn negate(self) -> Self {
        FilterExpr::Not(Box::new(self))
    }

    /// Joins expressions with the given [`Combinator`].
    pub fn combine(combinator: Combinator, exprs: Vec<FilterExpr>) -> Self {
        match combinator {
            Combinator::And => FilterExpr::And(exprs),
            Combinator::Or => FilterExpr::Or(exprs),
        }
    }

    /// Evaluates the expression against a record.
    ///
    /// An empty `And` matches everything and an empty `Or` matches nothing,
    /// consistent with their algebraic identities.
    pub fn matches(&self, record: &FilterableRecord) -> bool {
        match self {
            FilterExpr::Always => true,
            FilterExpr::Never => false,
            FilterExpr::Criterion(c) => c.matches(record),
            FilterExpr::And(exprs) => exprs.iter().all(|e| e.matches(record)),
            FilterExpr::Or(exprs) => exprs.iter().any(|e| e.matches(record)),
            FilterExpr::Not(inner) => !inner.matches(record),
        }
    }

    /// Returns references to the records that satisfy the expression, in order.
    pub fn apply<'a>(&self, records: &'a [FilterableRecord]) -> Vec<&'a FilterableRecord> {
        records.iter().filter(|r| self.matches(r)).collect()
    }

    /// Returns the indices of the records that satisfy the expression.
    pub fn apply_indices(&self, records: &[FilterableRecord]) -> Vec<usize> {
        records
            .iter()
            .enumerate()
            .filter_map(|(i, r)| if self.matches(r) { Some(i) } else { None })
            .collect()
    }

    /// Renders the expression as a parenthesized boolean string.
    pub fn to_predicate_string(&self) -> String {
        match self {
            FilterExpr::Always => "true".to_string(),
            FilterExpr::Never => "false".to_string(),
            FilterExpr::Criterion(c) => c.describe(),
            FilterExpr::And(exprs) => join_exprs(exprs, "AND"),
            FilterExpr::Or(exprs) => join_exprs(exprs, "OR"),
            FilterExpr::Not(inner) => format!("NOT ({})", inner.to_predicate_string()),
        }
    }
}

fn join_exprs(exprs: &[FilterExpr], op: &str) -> String {
    if exprs.is_empty() {
        return if op == "AND" { "true" } else { "false" }.to_string();
    }
    let parts: Vec<String> = exprs.iter().map(|e| e.to_predicate_string()).collect();
    format!("({})", parts.join(&format!(" {} ", op)))
}

// ===========================================================================
// Saved presets
// ===========================================================================

/// A named, savable filter expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterPreset {
    /// Unique preset name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// The filter expression to apply.
    pub expr: FilterExpr,
}

impl FilterPreset {
    /// Creates a new preset.
    pub fn new(name: &str, description: &str, expr: FilterExpr) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            expr,
        }
    }
}

/// A library of named filter presets with JSON import/export.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterPresetLibrary {
    /// The presets, kept in insertion order.
    pub presets: Vec<FilterPreset>,
}

impl FilterPresetLibrary {
    /// Creates an empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds (or replaces, by name) a preset.
    pub fn add(&mut self, preset: FilterPreset) {
        if let Some(existing) = self.presets.iter_mut().find(|p| p.name == preset.name) {
            *existing = preset;
        } else {
            self.presets.push(preset);
        }
    }

    /// Looks up a preset by name.
    pub fn get(&self, name: &str) -> Option<&FilterPreset> {
        self.presets.iter().find(|p| p.name == name)
    }

    /// Removes a preset by name, returning whether one was removed.
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.presets.len();
        self.presets.retain(|p| p.name != name);
        self.presets.len() != before
    }

    /// The preset names, in order.
    pub fn names(&self) -> Vec<&str> {
        self.presets.iter().map(|p| p.name.as_str()).collect()
    }

    /// A small library of generally useful legal-timeline presets.
    pub fn builtin() -> Self {
        let mut library = Self::new();
        library.add(FilterPreset::new(
            "enactments",
            "Only statute enactment events",
            FilterExpr::criterion(FilterCriterion::TagAny(
                ["enacted".to_string()].into_iter().collect(),
            )),
        ));
        library.add(FilterPreset::new(
            "amendments-and-repeals",
            "Amendment or repeal events",
            FilterExpr::criterion(FilterCriterion::TagAny(
                ["amended".to_string(), "repealed".to_string()]
                    .into_iter()
                    .collect(),
            )),
        ));
        library.add(FilterPreset::new(
            "modern-non-repeals",
            "Events from 2000 onward that are not repeals",
            FilterExpr::all(vec![
                FilterExpr::criterion(FilterCriterion::DateInRange(DateRange::since("2000-01-01"))),
                FilterExpr::criterion(FilterCriterion::TagAny(
                    ["repealed".to_string()].into_iter().collect(),
                ))
                .negate(),
            ]),
        ));
        library
    }

    /// Serializes the library to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("preset library to JSON: {}", e)))
    }

    /// Parses a library from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("preset library from JSON: {}", e)))
    }
}

// ===========================================================================
// Adapters from existing models
// ===========================================================================

/// Builds [`FilterableRecord`]s from a [`Timeline`].
///
/// Each event becomes a record whose `date` is the event date, with a tag for
/// the event type (e.g. `"enacted"`), a `statute` text field, a `detail` text
/// field (title/description, when present) and a `type`/`statute_id` attribute.
pub fn records_from_timeline(timeline: &Timeline) -> Vec<FilterableRecord> {
    timeline
        .events
        .iter()
        .enumerate()
        .map(|(i, (date, event))| {
            let (type_label, statute_id, detail) =
                crate::data_exchange::timeline_event_parts(event);
            let mut record = FilterableRecord::new(&format!("{}#{}", date, i))
                .with_date(date)
                .with_tag(&type_label.to_lowercase())
                .with_text("statute", statute_id)
                .with_attribute("type", type_label)
                .with_attribute("statute_id", statute_id);
            if let Some(detail) = detail {
                record = record.with_text("detail", detail);
            }
            record
        })
        .collect()
}

/// Builds [`FilterableRecord`]s from a [`DependencyGraph`].
///
/// Each node becomes a record whose id is the statute id, tagged with the
/// relations on its incident edges, with `in_degree`/`out_degree` attributes.
pub fn records_from_dependency_graph(graph: &DependencyGraph) -> Vec<FilterableRecord> {
    let inner = &graph.graph;
    inner
        .node_indices()
        .map(|idx| {
            let id = &inner[idx];
            let mut tags: BTreeSet<String> = BTreeSet::new();
            let mut out_degree = 0_usize;
            let mut in_degree = 0_usize;
            for edge in inner.edge_indices() {
                if let Some((source, target)) = inner.edge_endpoints(edge) {
                    if source == idx {
                        out_degree += 1;
                        tags.insert(inner[edge].clone());
                    }
                    if target == idx {
                        in_degree += 1;
                        tags.insert(inner[edge].clone());
                    }
                }
            }
            FilterableRecord {
                id: id.clone(),
                text_fields: [("id".to_string(), id.clone())].into_iter().collect(),
                tags,
                date: None,
                attributes: [
                    ("out_degree".to_string(), out_degree.to_string()),
                    ("in_degree".to_string(), in_degree.to_string()),
                ]
                .into_iter()
                .collect(),
            }
        })
        .collect()
}

// ===========================================================================
// Filter panel (UI descriptor)
// ===========================================================================

/// An exportable multi-criteria filtering UI (HTML + JavaScript).
///
/// The panel exposes per-field text inputs, tag checkboxes, an optional date
/// range and an `AND`/`OR` combinator with a global `NOT` toggle. The emitted
/// JavaScript reproduces this engine's matching semantics client-side over a
/// supplied `records` array, toggling the visibility of elements carrying a
/// `data-filter-id` matching each record id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterPanel {
    /// Panel heading.
    pub title: String,
    /// Text fields to expose as search inputs.
    pub text_fields: Vec<String>,
    /// Tags to expose as checkboxes.
    pub tags: BTreeSet<String>,
    /// Whether to render a date-range control.
    pub enable_date_range: bool,
    /// Default combinator joining the active criteria.
    pub default_combinator: Combinator,
}

impl FilterPanel {
    /// Creates a panel with the given title and `AND` default combinator.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            text_fields: Vec::new(),
            tags: BTreeSet::new(),
            enable_date_range: false,
            default_combinator: Combinator::And,
        }
    }

    /// Adds a searchable text field control.
    pub fn with_text_field(mut self, field: &str) -> Self {
        if !self.text_fields.iter().any(|f| f == field) {
            self.text_fields.push(field.to_string());
        }
        self
    }

    /// Adds a tag checkbox.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_string());
        self
    }

    /// Enables the date-range control.
    pub fn with_date_range(mut self) -> Self {
        self.enable_date_range = true;
        self
    }

    /// Sets the default combinator.
    pub fn with_combinator(mut self, combinator: Combinator) -> Self {
        self.default_combinator = combinator;
        self
    }

    /// Derives a panel from a set of records, collecting their text fields,
    /// tags and whether any carry a date.
    pub fn from_records(title: &str, records: &[FilterableRecord]) -> Self {
        let mut fields: Vec<String> = Vec::new();
        let mut tags: BTreeSet<String> = BTreeSet::new();
        let mut has_date = false;
        for record in records {
            for field in record.text_fields.keys() {
                if !fields.iter().any(|f| f == field) {
                    fields.push(field.clone());
                }
            }
            tags.extend(record.tags.iter().cloned());
            has_date |= record.date.is_some();
        }
        fields.sort();
        Self {
            title: title.to_string(),
            text_fields: fields,
            tags,
            enable_date_range: has_date,
            default_combinator: Combinator::And,
        }
    }

    /// Renders the filter panel as an HTML fragment.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<form class=\"filter-panel\" onsubmit=\"return false;\">\n");
        html.push_str(&format!("  <h3>{}</h3>\n", escape_xml(&self.title)));
        for field in &self.text_fields {
            let safe = escape_xml(field);
            html.push_str(&format!(
                "  <label class=\"filter-text\">{}<input type=\"text\" data-filter-field=\"{}\"></label>\n",
                safe, safe
            ));
        }
        if !self.tags.is_empty() {
            html.push_str("  <fieldset class=\"filter-tags\"><legend>Tags</legend>\n");
            for tag in &self.tags {
                let safe = escape_xml(tag);
                html.push_str(&format!(
                    "    <label><input type=\"checkbox\" data-filter-tag=\"{}\"> {}</label>\n",
                    safe, safe
                ));
            }
            html.push_str("  </fieldset>\n");
        }
        if self.enable_date_range {
            html.push_str("  <fieldset class=\"filter-dates\"><legend>Date range</legend>\n");
            html.push_str(
                "    <label>From <input type=\"date\" data-filter-date=\"start\"></label>\n",
            );
            html.push_str("    <label>To <input type=\"date\" data-filter-date=\"end\"></label>\n");
            html.push_str("  </fieldset>\n");
        }
        let (and_sel, or_sel) = match self.default_combinator {
            Combinator::And => (" selected", ""),
            Combinator::Or => ("", " selected"),
        };
        html.push_str("  <label class=\"filter-combinator\">Match\n");
        html.push_str("    <select data-filter-combinator>\n");
        html.push_str(&format!(
            "      <option value=\"AND\"{}>all criteria (AND)</option>\n",
            and_sel
        ));
        html.push_str(&format!(
            "      <option value=\"OR\"{}>any criterion (OR)</option>\n",
            or_sel
        ));
        html.push_str("    </select>\n  </label>\n");
        html.push_str(
            "  <label class=\"filter-negate\"><input type=\"checkbox\" data-filter-negate> Invert (NOT)</label>\n",
        );
        html.push_str("</form>\n");
        html
    }

    /// Renders the client-side filtering logic mirroring this engine.
    ///
    /// The returned script defines `applyLegalisFilter(records)`, reading the
    /// panel controls, evaluating each record and toggling
    /// `[data-filter-id="<id>"]` visibility.
    pub fn to_javascript(&self) -> String {
        let mut js = String::new();
        js.push_str("// Multi-criteria filter panel controller\n");
        js.push_str("function applyLegalisFilter(records) {\n");
        js.push_str("  const panel = document.querySelector('.filter-panel');\n");
        js.push_str("  if (!panel) { return []; }\n");
        js.push_str("  const texts = {};\n");
        js.push_str("  panel.querySelectorAll('[data-filter-field]').forEach(el => {\n");
        js.push_str("    if (el.value) { texts[el.getAttribute('data-filter-field')] = el.value.toLowerCase(); }\n");
        js.push_str("  });\n");
        js.push_str(
            "  const tags = Array.from(panel.querySelectorAll('[data-filter-tag]:checked'))\n",
        );
        js.push_str("    .map(el => el.getAttribute('data-filter-tag'));\n");
        js.push_str("  const dateStart = (panel.querySelector('[data-filter-date=\"start\"]') || {}).value || '';\n");
        js.push_str("  const dateEnd = (panel.querySelector('[data-filter-date=\"end\"]') || {}).value || '';\n");
        js.push_str("  const combinator = (panel.querySelector('[data-filter-combinator]') || {}).value || 'AND';\n");
        js.push_str(
            "  const negate = !!(panel.querySelector('[data-filter-negate]') || {}).checked;\n",
        );
        js.push_str("  function recordMatches(rec) {\n");
        js.push_str("    const checks = [];\n");
        js.push_str("    for (const f in texts) {\n");
        js.push_str(
            "      const v = (rec.text_fields && rec.text_fields[f] || '').toLowerCase();\n",
        );
        js.push_str("      checks.push(v.indexOf(texts[f]) !== -1);\n");
        js.push_str("    }\n");
        js.push_str("    if (tags.length) {\n");
        js.push_str("      const recTags = rec.tags || [];\n");
        js.push_str("      checks.push(tags.some(t => recTags.indexOf(t) !== -1));\n");
        js.push_str("    }\n");
        js.push_str("    if (dateStart) { checks.push(rec.date && rec.date >= dateStart); }\n");
        js.push_str("    if (dateEnd) { checks.push(rec.date && rec.date <= dateEnd); }\n");
        js.push_str("    if (checks.length === 0) { return true; }\n");
        js.push_str("    const base = combinator === 'OR' ? checks.some(Boolean) : checks.every(Boolean);\n");
        js.push_str("    return negate ? !base : base;\n");
        js.push_str("  }\n");
        js.push_str("  const visible = [];\n");
        js.push_str("  records.forEach(rec => {\n");
        js.push_str("    const ok = recordMatches(rec);\n");
        js.push_str("    if (ok) { visible.push(rec.id); }\n");
        js.push_str("    document.querySelectorAll('[data-filter-id=\"' + rec.id + '\"]')\n");
        js.push_str("      .forEach(el => { el.style.display = ok ? '' : 'none'; });\n");
        js.push_str("  });\n");
        js.push_str("  return visible;\n");
        js.push_str("}\n");
        js
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;

    fn sample_records() -> Vec<FilterableRecord> {
        vec![
            FilterableRecord::new("a")
                .with_text("title", "Housing Benefit Act")
                .with_tag("enacted")
                .with_date("1998-04-01")
                .with_attribute("type", "Enacted"),
            FilterableRecord::new("b")
                .with_text("title", "Housing Benefit Amendment")
                .with_tag("amended")
                .with_date("2005-06-15")
                .with_attribute("type", "Amended"),
            FilterableRecord::new("c")
                .with_text("title", "Old Poor Law")
                .with_tag("repealed")
                .with_date("2012-01-01")
                .with_attribute("type", "Repealed"),
        ]
    }

    #[test]
    fn date_range_inclusive_bounds_and_open_ends() {
        let closed = DateRange::between("2000-01-01", "2010-12-31");
        assert!(closed.contains("2000-01-01"));
        assert!(closed.contains("2010-12-31"));
        assert!(!closed.contains("1999-12-31"));
        assert!(!closed.contains("2011-01-01"));
        assert!(DateRange::since("2000-01-01").contains("2999-01-01"));
        assert!(DateRange::until("2000-01-01").contains("1900-01-01"));
        assert!(DateRange::new().contains("any"));
    }

    #[test]
    fn text_contains_respects_field_and_case() {
        let rec = FilterableRecord::new("x").with_text("title", "Housing Benefit");
        assert!(
            FilterCriterion::TextContains {
                field: Some("title".to_string()),
                needle: "housing".to_string(),
                case_insensitive: true,
            }
            .matches(&rec)
        );
        assert!(
            !FilterCriterion::TextContains {
                field: Some("title".to_string()),
                needle: "housing".to_string(),
                case_insensitive: false,
            }
            .matches(&rec)
        );
        // Wrong field does not match.
        assert!(
            !FilterCriterion::TextContains {
                field: Some("body".to_string()),
                needle: "Housing".to_string(),
                case_insensitive: false,
            }
            .matches(&rec)
        );
        // Any-field search finds it.
        assert!(
            FilterCriterion::TextContains {
                field: None,
                needle: "Benefit".to_string(),
                case_insensitive: false,
            }
            .matches(&rec)
        );
    }

    #[test]
    fn tag_any_and_all_semantics() {
        let rec = FilterableRecord::new("x")
            .with_tag("enacted")
            .with_tag("federal");
        let any = FilterCriterion::TagAny(
            ["repealed".to_string(), "federal".to_string()]
                .into_iter()
                .collect(),
        );
        assert!(any.matches(&rec));
        let all = FilterCriterion::TagAll(
            ["enacted".to_string(), "federal".to_string()]
                .into_iter()
                .collect(),
        );
        assert!(all.matches(&rec));
        let all_missing = FilterCriterion::TagAll(
            ["enacted".to_string(), "state".to_string()]
                .into_iter()
                .collect(),
        );
        assert!(!all_missing.matches(&rec));
    }

    #[test]
    fn attribute_equals_and_one_of() {
        let rec = FilterableRecord::new("x").with_attribute("type", "Enacted");
        assert!(
            FilterCriterion::AttributeEquals {
                key: "type".to_string(),
                value: "Enacted".to_string(),
            }
            .matches(&rec)
        );
        assert!(
            FilterCriterion::AttributeOneOf {
                key: "type".to_string(),
                values: ["Enacted".to_string(), "Amended".to_string()]
                    .into_iter()
                    .collect(),
            }
            .matches(&rec)
        );
        assert!(
            !FilterCriterion::AttributeOneOf {
                key: "type".to_string(),
                values: ["Repealed".to_string()].into_iter().collect(),
            }
            .matches(&rec)
        );
    }

    #[test]
    fn filter_expr_and_or_not_combination() {
        let records = sample_records();
        // (date >= 2000) AND NOT(tag repealed)
        let expr = FilterExpr::all(vec![
            FilterExpr::criterion(FilterCriterion::DateInRange(DateRange::since("2000-01-01"))),
            FilterExpr::criterion(FilterCriterion::TagAny(
                ["repealed".to_string()].into_iter().collect(),
            ))
            .negate(),
        ]);
        let matched: Vec<&str> = expr.apply(&records).iter().map(|r| r.id.as_str()).collect();
        assert_eq!(matched, vec!["b"]);

        // OR keeps repealed too.
        let or_expr = FilterExpr::any(vec![
            FilterExpr::criterion(FilterCriterion::TagAny(
                ["amended".to_string()].into_iter().collect(),
            )),
            FilterExpr::criterion(FilterCriterion::TagAny(
                ["repealed".to_string()].into_iter().collect(),
            )),
        ]);
        assert_eq!(or_expr.apply_indices(&records), vec![1, 2]);
    }

    #[test]
    fn empty_and_or_identities() {
        let rec = FilterableRecord::new("x");
        assert!(FilterExpr::And(vec![]).matches(&rec));
        assert!(!FilterExpr::Or(vec![]).matches(&rec));
        assert!(FilterExpr::Always.matches(&rec));
        assert!(!FilterExpr::Never.matches(&rec));
    }

    #[test]
    fn predicate_string_is_readable() {
        let expr = FilterExpr::criterion(FilterCriterion::TagAny(
            ["enacted".to_string()].into_iter().collect(),
        ))
        .and(
            FilterExpr::criterion(FilterCriterion::DateInRange(DateRange::since("2000-01-01")))
                .negate(),
        );
        let s = expr.to_predicate_string();
        assert!(s.contains("AND"));
        assert!(s.contains("NOT"));
        assert!(s.contains("tag in {enacted}"));
    }

    #[test]
    fn preset_library_add_replace_and_round_trip() {
        let mut library = FilterPresetLibrary::builtin();
        assert!(library.get("enactments").is_some());
        let count = library.presets.len();
        // Replace by name keeps the count stable.
        library.add(FilterPreset::new(
            "enactments",
            "updated",
            FilterExpr::Always,
        ));
        assert_eq!(library.presets.len(), count);
        assert_eq!(
            library.get("enactments").map(|p| p.description.as_str()),
            Some("updated")
        );

        let json = library.to_json().expect("to_json");
        let restored = FilterPresetLibrary::from_json(&json).expect("from_json");
        assert_eq!(library, restored);

        assert!(library.remove("enactments"));
        assert!(!library.remove("nonexistent"));
    }

    #[test]
    fn records_from_timeline_carry_dates_and_tags() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2000-01-01",
            TimelineEvent::Enacted {
                statute_id: "s-1".to_string(),
                title: "Act".to_string(),
            },
        );
        timeline.add_event(
            "2010-01-01",
            TimelineEvent::Repealed {
                statute_id: "s-1".to_string(),
            },
        );
        let records = records_from_timeline(&timeline);
        assert_eq!(records.len(), 2);
        assert!(records[0].tags.contains("enacted"));
        assert_eq!(records[0].date.as_deref(), Some("2000-01-01"));
        // Repeal-only event has no detail field.
        assert!(!records[1].text_fields.contains_key("detail"));

        let recent =
            FilterExpr::criterion(FilterCriterion::DateInRange(DateRange::since("2005-01-01")));
        assert_eq!(recent.apply(&records).len(), 1);
    }

    #[test]
    fn records_from_dependency_graph_tag_relations_and_degrees() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        graph.add_dependency("a", "c", "amends");
        let records = records_from_dependency_graph(&graph);
        let a = records.iter().find(|r| r.id == "a").expect("node a");
        assert_eq!(
            a.attributes.get("out_degree").map(String::as_str),
            Some("2")
        );
        assert!(a.tags.contains("requires"));
        assert!(a.tags.contains("amends"));
        let b = records.iter().find(|r| r.id == "b").expect("node b");
        assert_eq!(b.attributes.get("in_degree").map(String::as_str), Some("1"));
    }

    #[test]
    fn filter_panel_html_and_js_contain_controls() {
        let panel = FilterPanel::from_records("Filter events", &sample_records())
            .with_combinator(Combinator::Or);
        let html = panel.to_html();
        assert!(html.contains("<form class=\"filter-panel\""));
        assert!(html.contains("data-filter-tag=\"enacted\""));
        assert!(html.contains("data-filter-date=\"start\""));
        assert!(html.contains("any criterion (OR)</option>"));
        let js = panel.to_javascript();
        assert!(js.contains("function applyLegalisFilter"));
        assert!(js.contains("data-filter-id"));
    }

    #[test]
    fn filter_panel_escapes_titles_and_tags() {
        let panel = FilterPanel::new("A <b> & \"c\"").with_tag("x<y");
        let html = panel.to_html();
        assert!(html.contains("A &lt;b&gt; &amp; &quot;c&quot;"));
        assert!(html.contains("data-filter-tag=\"x&lt;y\""));
    }
}
