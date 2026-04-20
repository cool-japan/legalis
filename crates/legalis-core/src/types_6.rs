//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::NaiveDate;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::functions::EvaluationContext;
use super::types::{Effect, StatuteException, ValidationError};
use super::types_3::EffectType;
use super::types_4::{StatuteChange, TemporalValidity};
use super::types_5::{Condition, StatuteDiff};

/// Simple builder for constructing `Statute` objects with template support
/// and progressive validation.
///
/// Unlike `TypedStatuteBuilder`, this builder is runtime-validated and provides
/// convenience methods like `from_template()` and progressive validation.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteBuilder, Effect, EffectType, Condition, ComparisonOp};
///
/// let statute = StatuteBuilder::new()
///     .id("tax-law-1")
///     .title("Tax Credit Law")
///     .effect(Effect::new(EffectType::Grant, "Tax credit"))
///     .precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18))
///     .validate_progressive(true)
///     .build()
///     .expect("Failed to build statute");
///
/// assert_eq!(statute.id, "tax-law-1");
/// ```
#[derive(Debug, Clone)]
pub struct StatuteBuilder {
    id: Option<String>,
    title: Option<String>,
    effect: Option<Effect>,
    preconditions: Vec<Condition>,
    discretion_logic: Option<String>,
    temporal_validity: TemporalValidity,
    version: u32,
    jurisdiction: Option<String>,
    derives_from: Vec<String>,
    applies_to: Vec<String>,
    exceptions: Vec<StatuteException>,
    progressive_validation: bool,
    validation_errors: Vec<ValidationError>,
}
impl StatuteBuilder {
    /// Creates a new statute builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            effect: None,
            preconditions: Vec::new(),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
            progressive_validation: false,
            validation_errors: Vec::new(),
        }
    }
    /// Creates a builder from an existing statute template.
    ///
    /// This copies all fields from the template statute, allowing you to modify
    /// specific fields while keeping others the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteBuilder, Statute, Effect, EffectType};
    ///
    /// let template = Statute::new("template-1", "Template Law", Effect::grant("Benefit"))
    ///     .with_version(1)
    ///     .with_jurisdiction("US");
    ///
    /// let derived = StatuteBuilder::from_template(&template)
    ///     .id("derived-1")
    ///     .title("Derived Law")
    ///     .build()
    ///     .expect("Failed to build");
    ///
    /// assert_eq!(derived.jurisdiction, Some("US".to_string()));
    /// assert_eq!(derived.version, 1);
    /// ```
    #[must_use]
    pub fn from_template(template: &Statute) -> Self {
        Self {
            id: Some(template.id.clone()),
            title: Some(template.title.clone()),
            effect: Some(template.effect.clone()),
            preconditions: template.preconditions.clone(),
            discretion_logic: template.discretion_logic.clone(),
            temporal_validity: template.temporal_validity.clone(),
            version: template.version,
            jurisdiction: template.jurisdiction.clone(),
            derives_from: template.derives_from.clone(),
            applies_to: template.applies_to.clone(),
            exceptions: template.exceptions.clone(),
            progressive_validation: false,
            validation_errors: Vec::new(),
        }
    }
    /// Enables or disables progressive validation.
    ///
    /// When enabled, the builder validates each field as it's set and accumulates
    /// validation errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteBuilder, Effect, EffectType};
    ///
    /// let result = StatuteBuilder::new()
    ///     .validate_progressive(true)
    ///     .id("") // Invalid ID - empty
    ///     .title("Test")
    ///     .effect(Effect::grant("Benefit"))
    ///     .build();
    ///
    /// assert!(result.is_err());
    /// ```
    #[must_use]
    pub fn validate_progressive(mut self, enabled: bool) -> Self {
        self.progressive_validation = enabled;
        self
    }
    /// Sets the statute ID.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        if self.progressive_validation {
            if id.is_empty() {
                self.validation_errors.push(ValidationError::EmptyId);
            } else if !self.is_valid_id(&id) {
                self.validation_errors
                    .push(ValidationError::InvalidId(id.clone()));
            }
        }
        self.id = Some(id);
        self
    }
    /// Sets the statute title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let title = title.into();
        if self.progressive_validation && title.is_empty() {
            self.validation_errors.push(ValidationError::EmptyTitle);
        }
        self.title = Some(title);
        self
    }
    /// Sets the effect.
    #[must_use]
    pub fn effect(mut self, effect: Effect) -> Self {
        if self.progressive_validation && effect.description.is_empty() {
            self.validation_errors
                .push(ValidationError::EmptyEffectDescription);
        }
        self.effect = Some(effect);
        self
    }
    /// Adds a precondition.
    #[must_use]
    pub fn precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }
    /// Sets the discretion logic.
    #[must_use]
    pub fn discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }
    /// Sets temporal validity.
    #[must_use]
    pub fn temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }
    /// Sets the version.
    #[must_use]
    pub fn version(mut self, version: u32) -> Self {
        if self.progressive_validation && version == 0 {
            self.validation_errors.push(ValidationError::InvalidVersion);
        }
        self.version = version;
        self
    }
    /// Sets the jurisdiction.
    #[must_use]
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Adds a derivation source.
    #[must_use]
    pub fn derives_from(mut self, source: impl Into<String>) -> Self {
        self.derives_from.push(source.into());
        self
    }
    /// Adds an applicable entity type.
    #[must_use]
    pub fn applies_to(mut self, entity_type: impl Into<String>) -> Self {
        self.applies_to.push(entity_type.into());
        self
    }
    /// Adds an exception.
    #[must_use]
    pub fn exception(mut self, exception: StatuteException) -> Self {
        self.exceptions.push(exception);
        self
    }
    /// Returns accumulated validation errors (when progressive validation is enabled).
    #[must_use]
    pub fn validation_errors(&self) -> &[ValidationError] {
        &self.validation_errors
    }
    /// Checks if an ID is valid.
    fn is_valid_id(&self, id: &str) -> bool {
        !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && id.chars().next().is_some_and(|c| c.is_alphabetic())
    }
    /// Builds the statute, returning an error if required fields are missing or validation fails.
    pub fn build(self) -> Result<Statute, Vec<ValidationError>> {
        let mut errors = self.validation_errors;
        if self.id.is_none() {
            errors.push(ValidationError::EmptyId);
        }
        if self.title.is_none() {
            errors.push(ValidationError::EmptyTitle);
        }
        if self.effect.is_none() {
            errors.push(ValidationError::EmptyEffectDescription);
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let statute = Statute {
            id: self
                .id
                .expect("invariant: id validated as Some before this point"),
            title: self
                .title
                .expect("invariant: title validated as Some before this point"),
            effect: self
                .effect
                .expect("invariant: effect validated as Some before this point"),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            derives_from: self.derives_from,
            applies_to: self.applies_to,
            exceptions: self.exceptions,
        };
        let validation_errors = statute.validate();
        if !validation_errors.is_empty() {
            Err(validation_errors)
        } else {
            Ok(statute)
        }
    }
}
/// Strategies for resolving conflicts between composed effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CompositionStrategy {
    /// First effect of each type wins (default).
    FirstWins,
    /// Last effect of each type wins (later overrides earlier).
    LastWins,
    /// Most specific effect wins (most parameters).
    MostSpecific,
    /// All effects apply (no deduplication).
    AllApply,
}
/// Context wrapper that provides fallback evaluation strategies.
///
/// When the primary context cannot provide a value, it falls back to a secondary context.
///
/// # Example
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, FallbackContext, EvaluationContext};
/// # use std::collections::HashMap;
/// let mut primary_attrs = HashMap::new();
/// primary_attrs.insert("name".to_string(), "Alice".to_string());
/// let primary = AttributeBasedContext::new(primary_attrs);
///
/// let mut fallback_attrs = HashMap::new();
/// fallback_attrs.insert("age".to_string(), "25".to_string());
/// fallback_attrs.insert("name".to_string(), "Bob".to_string()); // Will not be used
/// let fallback = AttributeBasedContext::new(fallback_attrs);
///
/// let ctx = FallbackContext::new(&primary, &fallback);
///
/// // name comes from primary
/// assert_eq!(ctx.get_attribute("name"), Some("Alice".to_string()));
/// // age comes from fallback
/// assert_eq!(ctx.get_attribute("age"), Some("25".to_string()));
/// ```
#[derive(Debug)]
pub struct FallbackContext<'a, C1: EvaluationContext, C2: EvaluationContext> {
    pub(super) primary: &'a C1,
    pub(super) fallback: &'a C2,
}
impl<'a, C1: EvaluationContext, C2: EvaluationContext> FallbackContext<'a, C1, C2> {
    /// Creates a new context with fallback.
    pub fn new(primary: &'a C1, fallback: &'a C2) -> Self {
        Self { primary, fallback }
    }
}
/// Fluent query builder for searching and filtering statutes.
///
/// Provides a chainable API for constructing complex queries over statute collections.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, StatuteQuery, Condition, ComparisonOp};
/// use chrono::NaiveDate;
///
/// let statutes = vec![
///     Statute::new("law1", "Voting Rights", Effect::grant("vote"))
///         .with_jurisdiction("US")
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
///     Statute::new("law2", "Tax Credit", Effect::grant("credit"))
///         .with_jurisdiction("US-CA")
///         .with_precondition(Condition::income(ComparisonOp::LessThan, 50000)),
/// ];
///
/// // Find all US statutes with preconditions
/// let results = StatuteQuery::new(&statutes)
///     .jurisdiction("US")
///     .with_preconditions()
///     .execute();
///
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0].id, "law1");
/// ```
pub struct StatuteQuery<'a> {
    statutes: &'a [Statute],
    #[allow(clippy::type_complexity)]
    filters: Vec<Box<dyn Fn(&Statute) -> bool + 'a>>,
}
impl<'a> StatuteQuery<'a> {
    /// Creates a new query over the given statute collection.
    #[must_use]
    pub fn new(statutes: &'a [Statute]) -> Self {
        Self {
            statutes,
            filters: Vec::new(),
        }
    }
    /// Filters statutes by jurisdiction.
    #[must_use]
    pub fn jurisdiction(mut self, jurisdiction: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.jurisdiction.as_ref().is_some_and(|j| j == jurisdiction)
        }));
        self
    }
    /// Filters statutes by jurisdiction prefix (e.g., "US" matches "US", "US-CA", "US-NY").
    #[must_use]
    pub fn jurisdiction_prefix(mut self, prefix: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.jurisdiction
                .as_ref()
                .is_some_and(|j| j.starts_with(prefix))
        }));
        self
    }
    /// Filters statutes by effect type.
    #[must_use]
    pub fn effect_type(mut self, effect_type: EffectType) -> Self {
        self.filters
            .push(Box::new(move |s| s.effect.effect_type == effect_type));
        self
    }
    /// Filters statutes that grant a specific right or privilege.
    #[must_use]
    pub fn grants(mut self, description: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.effect.effect_type == EffectType::Grant && s.effect.description.contains(description)
        }));
        self
    }
    /// Filters statutes that revoke a specific right or privilege.
    #[must_use]
    pub fn revokes(mut self, description: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.effect.effect_type == EffectType::Revoke && s.effect.description.contains(description)
        }));
        self
    }
    /// Filters statutes that have preconditions.
    #[must_use]
    pub fn with_preconditions(mut self) -> Self {
        self.filters.push(Box::new(|s| !s.preconditions.is_empty()));
        self
    }
    /// Filters statutes that have no preconditions (unconditional).
    #[must_use]
    pub fn unconditional(mut self) -> Self {
        self.filters.push(Box::new(|s| s.preconditions.is_empty()));
        self
    }
    /// Filters statutes by minimum number of preconditions.
    #[must_use]
    pub fn min_preconditions(mut self, min: usize) -> Self {
        self.filters
            .push(Box::new(move |s| s.preconditions.len() >= min));
        self
    }
    /// Filters statutes effective at a given date.
    #[must_use]
    pub fn effective_at(mut self, date: NaiveDate) -> Self {
        self.filters
            .push(Box::new(move |s| s.temporal_validity.is_active(date)));
        self
    }
    /// Filters statutes that are currently effective.
    #[must_use]
    pub fn currently_effective(mut self) -> Self {
        let today = chrono::Utc::now().date_naive();
        self.filters
            .push(Box::new(move |s| s.temporal_validity.is_active(today)));
        self
    }
    /// Filters statutes with a specific version.
    #[must_use]
    pub fn version(mut self, version: u32) -> Self {
        self.filters.push(Box::new(move |s| s.version == version));
        self
    }
    /// Filters statutes by ID prefix.
    #[must_use]
    pub fn id_prefix(mut self, prefix: &'a str) -> Self {
        self.filters
            .push(Box::new(move |s| s.id.starts_with(prefix)));
        self
    }
    /// Filters statutes by ID suffix.
    #[must_use]
    pub fn id_suffix(mut self, suffix: &'a str) -> Self {
        self.filters.push(Box::new(move |s| s.id.ends_with(suffix)));
        self
    }
    /// Filters statutes containing a keyword in title or ID.
    #[must_use]
    pub fn keyword(mut self, keyword: &'a str) -> Self {
        self.filters.push(Box::new(move |s| {
            s.id.contains(keyword) || s.title.contains(keyword)
        }));
        self
    }
    /// Filters statutes with a custom predicate.
    #[must_use]
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Statute) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }
    /// Executes the query and returns matching statutes.
    #[must_use]
    pub fn execute(self) -> Vec<&'a Statute> {
        self.statutes
            .iter()
            .filter(|statute| self.filters.iter().all(|f| f(statute)))
            .collect()
    }
    /// Executes the query and returns the first matching statute.
    #[must_use]
    pub fn first(self) -> Option<&'a Statute> {
        self.statutes
            .iter()
            .find(|statute| self.filters.iter().all(|f| f(statute)))
    }
    /// Executes the query and returns the count of matching statutes.
    #[must_use]
    pub fn count(self) -> usize {
        self.statutes
            .iter()
            .filter(|statute| self.filters.iter().all(|f| f(statute)))
            .count()
    }
    /// Executes the query and checks if any statutes match.
    #[must_use]
    pub fn exists(self) -> bool {
        self.statutes
            .iter()
            .any(|statute| self.filters.iter().all(|f| f(statute)))
    }
}
/// Legal judgment result as an Algebraic Data Type (ADT).
///
/// This type embodies the core philosophy of Legalis-RS:
/// "Not everything should be computable" - preserving human agency
/// in legal interpretation.
///
/// # Examples
///
/// ## Deterministic Result
///
/// ```
/// use legalis_core::LegalResult;
///
/// let age = 25;
/// let result: LegalResult<bool> = if age >= 18 {
///     LegalResult::Deterministic(true)
/// } else {
///     LegalResult::Deterministic(false)
/// };
///
/// assert!(result.is_deterministic());
/// ```
///
/// ## Judicial Discretion
///
/// ```
/// use legalis_core::LegalResult;
/// use uuid::Uuid;
///
/// let result: LegalResult<bool> = LegalResult::JudicialDiscretion {
///     issue: "Determine if there is just cause for termination".to_string(),
///     context_id: Uuid::new_v4(),
///     narrative_hint: Some("Consider employment history and circumstances".to_string()),
/// };
///
/// assert!(result.requires_discretion());
/// ```
///
/// ## Mapping Values
///
/// ```
/// use legalis_core::LegalResult;
///
/// let amount: LegalResult<u32> = LegalResult::Deterministic(100);
/// let doubled = amount.map(|x| x * 2);
///
/// assert_eq!(doubled, LegalResult::Deterministic(200));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LegalResult<T> {
    /// Deterministic domain: Results derived automatically through computation.
    /// Examples: age requirements, income limits, deadline calculations.
    Deterministic(T),
    /// Discretionary domain: Cannot be determined by logic alone,
    /// requires human "narrative" (interpretation).
    /// This is the safeguard against "AI theocracy".
    /// The system halts here and passes the ball to humans.
    JudicialDiscretion {
        /// The issue at hand (e.g., "existence of just cause", "violation of public welfare")
        issue: String,
        /// Reference to context data
        context_id: Uuid,
        /// Recommended judgment materials (generated by LLM, but does not decide)
        narrative_hint: Option<String>,
    },
    /// Logical contradiction: A bug in the law itself.
    Void { reason: String },
}
impl<T> LegalResult<T> {
    /// Returns true if this is a deterministic result.
    pub fn is_deterministic(&self) -> bool {
        matches!(self, Self::Deterministic(_))
    }
    /// Returns true if judicial discretion is required.
    pub fn requires_discretion(&self) -> bool {
        matches!(self, Self::JudicialDiscretion { .. })
    }
    /// Returns true if this represents a void/invalid state.
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Void { .. })
    }
    /// Maps a deterministic value using the provided function.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> LegalResult<U> {
        match self {
            Self::Deterministic(t) => LegalResult::Deterministic(f(t)),
            Self::JudicialDiscretion {
                issue,
                context_id,
                narrative_hint,
            } => LegalResult::JudicialDiscretion {
                issue,
                context_id,
                narrative_hint,
            },
            Self::Void { reason } => LegalResult::Void { reason },
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Statute {
    /// Unique identifier (e.g., "civil-code-article-1")
    pub id: String,
    /// Title of the statute
    pub title: String,
    /// Preconditions (If)
    pub preconditions: Vec<Condition>,
    /// Legal effect (Then)
    pub effect: Effect,
    /// Discretion logic description (Else If Maybe)
    pub discretion_logic: Option<String>,
    /// Temporal validity (effective dates, sunset clauses)
    pub temporal_validity: TemporalValidity,
    /// Version number
    pub version: u32,
    /// Jurisdiction identifier
    pub jurisdiction: Option<String>,
    /// Derivation source - the statute(s) this one is derived from
    pub derives_from: Vec<String>,
    /// Applicable entity types - what types of entities this statute applies to
    pub applies_to: Vec<String>,
    /// Structured exceptions - conditions under which this statute does not apply
    pub exceptions: Vec<StatuteException>,
}
impl Statute {
    /// Creates a new Statute.
    pub fn new(id: impl Into<String>, title: impl Into<String>, effect: Effect) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            preconditions: Vec::new(),
            effect,
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
        }
    }
    /// Adds a precondition.
    pub fn with_precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }
    /// Sets the discretion logic.
    pub fn with_discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }
    /// Sets temporal validity.
    pub fn with_temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }
    /// Sets the version.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }
    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Adds a statute ID that this statute is derived from.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let derived = Statute::new("state-law-1", "State Law", Effect::grant("Benefit"))
    ///     .with_derives_from("federal-law-1");
    ///
    /// assert_eq!(derived.derives_from, vec!["federal-law-1"]);
    /// ```
    pub fn with_derives_from(mut self, source_id: impl Into<String>) -> Self {
        self.derives_from.push(source_id.into());
        self
    }
    /// Adds an entity type that this statute applies to.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let law = Statute::new("business-law-1", "Business Regulation", Effect::grant("License"))
    ///     .with_applies_to("Corporation")
    ///     .with_applies_to("LLC");
    ///
    /// assert!(law.applies_to.contains(&"Corporation".to_string()));
    /// ```
    pub fn with_applies_to(mut self, entity_type: impl Into<String>) -> Self {
        self.applies_to.push(entity_type.into());
        self
    }
    /// Adds an exception to this statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteException, Condition, ComparisonOp};
    ///
    /// let law = Statute::new("tax-law-1", "Income Tax", Effect::grant("Tax liability"))
    ///     .with_exception(StatuteException::new(
    ///         "minor-exception",
    ///         "Minors are exempt",
    ///         Condition::age(ComparisonOp::LessThan, 18)
    ///     ));
    ///
    /// assert_eq!(law.exceptions.len(), 1);
    /// ```
    pub fn with_exception(mut self, exception: StatuteException) -> Self {
        self.exceptions.push(exception);
        self
    }
    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        self.temporal_validity.is_active(as_of)
    }
    /// Returns the number of preconditions.
    #[must_use]
    pub fn precondition_count(&self) -> usize {
        self.preconditions.len()
    }
    /// Returns whether this statute has any preconditions.
    #[must_use]
    pub fn has_preconditions(&self) -> bool {
        !self.preconditions.is_empty()
    }
    /// Returns whether this statute has discretion logic.
    #[must_use]
    pub fn has_discretion(&self) -> bool {
        self.discretion_logic.is_some()
    }
    /// Returns whether this statute has a jurisdiction set.
    #[must_use]
    pub fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.is_some()
    }
    /// Returns a reference to the preconditions.
    pub fn preconditions(&self) -> &[Condition] {
        &self.preconditions
    }
    /// Returns whether this statute is derived from other statutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let derived = Statute::new("derived", "Derived Law", Effect::grant("Benefit"))
    ///     .with_derives_from("source-law");
    ///
    /// assert!(derived.is_derived());
    /// ```
    #[must_use]
    pub fn is_derived(&self) -> bool {
        !self.derives_from.is_empty()
    }
    /// Returns the IDs of statutes this statute is derived from.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let derived = Statute::new("derived", "Derived Law", Effect::grant("Benefit"))
    ///     .with_derives_from("source-1")
    ///     .with_derives_from("source-2");
    ///
    /// assert_eq!(derived.derivation_sources(), &["source-1", "source-2"]);
    /// ```
    pub fn derivation_sources(&self) -> &[String] {
        &self.derives_from
    }
    /// Returns whether this statute applies to a specific entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let law = Statute::new("law-1", "Law", Effect::grant("License"))
    ///     .with_applies_to("Corporation");
    ///
    /// assert!(law.applies_to_entity_type("Corporation"));
    /// assert!(!law.applies_to_entity_type("Individual"));
    /// ```
    #[must_use]
    pub fn applies_to_entity_type(&self, entity_type: &str) -> bool {
        self.applies_to.iter().any(|t| t == entity_type)
    }
    /// Returns whether this statute has any entity type restrictions.
    ///
    /// If this returns `false`, the statute applies to all entity types.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect};
    ///
    /// let general_law = Statute::new("law-1", "General Law", Effect::grant("Benefit"));
    /// assert!(!general_law.has_entity_restrictions());
    ///
    /// let specific_law = Statute::new("law-2", "Specific Law", Effect::grant("Benefit"))
    ///     .with_applies_to("Corporation");
    /// assert!(specific_law.has_entity_restrictions());
    /// ```
    #[must_use]
    pub fn has_entity_restrictions(&self) -> bool {
        !self.applies_to.is_empty()
    }
    /// Returns the entity types this statute applies to.
    pub fn applicable_entity_types(&self) -> &[String] {
        &self.applies_to
    }
    /// Returns whether this statute has exceptions.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, StatuteException, Condition, ComparisonOp};
    ///
    /// let law = Statute::new("law-1", "Law", Effect::grant("Benefit"))
    ///     .with_exception(StatuteException::new(
    ///         "exc-1",
    ///         "Exception",
    ///         Condition::age(ComparisonOp::LessThan, 18)
    ///     ));
    ///
    /// assert!(law.has_exceptions());
    /// ```
    #[must_use]
    pub fn has_exceptions(&self) -> bool {
        !self.exceptions.is_empty()
    }
    /// Returns a reference to the exceptions.
    pub fn exception_list(&self) -> &[StatuteException] {
        &self.exceptions
    }
    /// Returns the number of exceptions.
    #[must_use]
    pub fn exception_count(&self) -> usize {
        self.exceptions.len()
    }
    /// Validates the statute and returns a list of validation errors.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.id.is_empty() {
            errors.push(ValidationError::EmptyId);
        } else if !self.is_valid_id(&self.id) {
            errors.push(ValidationError::InvalidId(self.id.clone()));
        }
        if self.title.is_empty() {
            errors.push(ValidationError::EmptyTitle);
        }
        if let (Some(eff), Some(exp)) = (
            self.temporal_validity.effective_date,
            self.temporal_validity.expiry_date,
        ) && exp < eff
        {
            errors.push(ValidationError::ExpiryBeforeEffective {
                effective: eff,
                expiry: exp,
            });
        }
        for (i, cond) in self.preconditions.iter().enumerate() {
            if let Some(err) = Self::validate_condition(cond) {
                errors.push(ValidationError::InvalidCondition {
                    index: i,
                    message: err,
                });
            }
        }
        if self.effect.description.is_empty() {
            errors.push(ValidationError::EmptyEffectDescription);
        }
        if self.version == 0 {
            errors.push(ValidationError::InvalidVersion);
        }
        errors
    }
    /// Returns true if the statute is valid (has no validation errors).
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
    /// Validates the statute and returns an error if invalid.
    pub fn validated(self) -> Result<Self, Vec<ValidationError>> {
        let errors = self.validate();
        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }
    /// Checks if an ID is valid (alphanumeric with dashes/underscores).
    fn is_valid_id(&self, id: &str) -> bool {
        !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && id.chars().next().is_some_and(|c| c.is_alphabetic())
    }
    /// Validates a condition recursively.
    fn validate_condition(condition: &Condition) -> Option<String> {
        match condition {
            Condition::Age { value, .. } => {
                if *value > 150 {
                    Some(format!("Unrealistic age value: {}", value))
                } else {
                    None
                }
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::validate_condition(left).or_else(|| Self::validate_condition(right))
            }
            Condition::Not(inner) => Self::validate_condition(inner),
            Condition::ResidencyDuration { months, .. } => {
                if *months > 1200 {
                    Some(format!("Unrealistic residency duration: {} months", months))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Checks if this statute subsumes another statute.
    ///
    /// Statute A subsumes statute B if:
    /// - A's preconditions are more general than (or equal to) B's preconditions
    /// - A's effect is the same or broader than B's effect
    /// - Whenever B applies, A also applies
    ///
    /// This is useful for detecting redundancy and logical relationships between statutes.
    ///
    /// **Note**: This is a simplified heuristic-based implementation.
    /// Full subsumption checking would require logical analysis of condition relationships
    /// (e.g., recognizing that age >= 18 subsumes age >= 21).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// // Statute with no preconditions subsumes one with preconditions (same effect)
    /// let general = Statute::new("general", "Voting Rights", Effect::grant("Vote"));
    ///
    /// let specific = Statute::new("specific", "Voting Rights (21+)", Effect::grant("Vote"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// // General (no conditions) subsumes specific (has conditions)
    /// assert_eq!(general.subsumes(&specific), true);
    /// assert_eq!(specific.subsumes(&general), false);
    /// ```
    #[must_use]
    pub fn subsumes(&self, other: &Self) -> bool {
        if self.effect.effect_type != other.effect.effect_type
            || self.effect.description != other.effect.description
        {
            return false;
        }
        if self.jurisdiction != other.jurisdiction && self.jurisdiction.is_some() {
            return false;
        }
        if self.preconditions.is_empty() {
            return true;
        }
        if other.preconditions.is_empty() {
            return false;
        }
        self.preconditions.len() <= other.preconditions.len()
    }
    /// Checks if this statute is subsumed by another statute.
    ///
    /// This is the inverse of [`Self::subsumes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let general = Statute::new("general", "General Rule", Effect::grant("Benefit"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let specific = Statute::new("specific", "Specific Rule", Effect::grant("Benefit"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// assert_eq!(specific.is_subsumed_by(&general), true);
    /// ```
    #[must_use]
    pub fn is_subsumed_by(&self, other: &Self) -> bool {
        other.subsumes(self)
    }
    /// Computes the differences between this statute and another version.
    ///
    /// This is useful for tracking amendments, understanding changes over time,
    /// and generating change logs for legal documents.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    ///
    /// let version1 = Statute::new("tax-1", "Tax Law", Effect::grant("Tax Credit"))
    ///     .with_precondition(Condition::income(ComparisonOp::LessThan, 50000))
    ///     .with_version(1);
    ///
    /// let version2 = Statute::new("tax-1", "Tax Law (Amended)", Effect::grant("Tax Credit"))
    ///     .with_precondition(Condition::income(ComparisonOp::LessThan, 60000))
    ///     .with_version(2);
    ///
    /// let diff = version1.diff(&version2);
    /// assert!(!diff.changes.is_empty());
    /// ```
    #[must_use]
    pub fn diff(&self, other: &Self) -> StatuteDiff {
        let mut changes = Vec::new();
        if self.id != other.id {
            changes.push(StatuteChange::IdChanged {
                old: self.id.clone(),
                new: other.id.clone(),
            });
        }
        if self.title != other.title {
            changes.push(StatuteChange::TitleChanged {
                old: self.title.clone(),
                new: other.title.clone(),
            });
        }
        if self.effect != other.effect {
            changes.push(StatuteChange::EffectChanged {
                old: format!("{}", self.effect),
                new: format!("{}", other.effect),
            });
        }
        if self.preconditions != other.preconditions {
            changes.push(StatuteChange::PreconditionsChanged {
                added: other
                    .preconditions
                    .len()
                    .saturating_sub(self.preconditions.len()),
                removed: self
                    .preconditions
                    .len()
                    .saturating_sub(other.preconditions.len()),
            });
        }
        if self.temporal_validity != other.temporal_validity {
            changes.push(StatuteChange::TemporalValidityChanged);
        }
        if self.version != other.version {
            changes.push(StatuteChange::VersionChanged {
                old: self.version,
                new: other.version,
            });
        }
        if self.jurisdiction != other.jurisdiction {
            changes.push(StatuteChange::JurisdictionChanged {
                old: self.jurisdiction.clone(),
                new: other.jurisdiction.clone(),
            });
        }
        StatuteDiff {
            statute_id: self.id.clone(),
            changes,
        }
    }
}
