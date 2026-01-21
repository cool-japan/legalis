//! Case Law support for Common Law jurisdictions.
//!
//! This module provides types for representing judicial precedents,
//! the doctrine of stare decisis, and case-based legal reasoning.
//!
//! ## Common Law vs Civil Law
//!
//! | Civil Law (大陸法) | Common Law (英米法) |
//! |-------------------|---------------------|
//! | Statutes primary | Cases primary |
//! | Codified rules | Judge-made law |
//! | Deductive reasoning | Analogical reasoning |
//! | General principles | Specific holdings |
//!
//! ## Stare Decisis (先例拘束性)
//!
//! The doctrine that courts are bound by prior decisions:
//! - **Binding precedent** (拘束的先例): Must follow higher courts in same jurisdiction
//! - **Persuasive precedent** (説得的先例): May consider other jurisdictions
//!
//! ## Example: Tort Law Development
//!
//! ```text
//! Donoghue v Stevenson (1932) → Established duty of care
//!         ↓
//! Palsgraf v Long Island RR (1928) → Proximate cause doctrine
//!         ↓
//! Modern negligence law
//! ```

use chrono::NaiveDate;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::{Condition, Effect};

/// A judicial decision (判例).
///
/// In common law systems, cases create binding precedents that
/// future courts must follow under the doctrine of stare decisis.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Case {
    /// Unique case identifier
    pub id: Uuid,
    /// Case citation (e.g., "Palsgraf v. Long Island R.R., 248 N.Y. 339 (1928)")
    pub citation: String,
    /// Short case name (e.g., "Palsgraf v. Long Island")
    pub short_name: String,
    /// Year decided
    pub year: u32,
    /// Court that decided the case
    pub court: Court,
    /// Jurisdiction (e.g., "US-NY" for New York, "UK" for United Kingdom)
    pub jurisdiction: String,
    /// Facts of the case
    pub facts: String,
    /// Legal issue(s) presented
    pub issues: Vec<String>,
    /// Holding (the court's decision)
    pub holding: String,
    /// Ratio decidendi (reasoning) - the binding part
    pub ratio: String,
    /// Obiter dicta (remarks) - non-binding commentary
    pub obiter: Option<String>,
    /// Legal rule established by this case
    pub rule: Option<CaseRule>,
    /// Precedents cited by this case
    pub cited_cases: Vec<Uuid>,
    /// Whether this case was overruled
    pub overruled: bool,
    /// Case that overruled this one (if any)
    pub overruled_by: Option<Uuid>,
    /// Date decided
    pub date: NaiveDate,
}

impl Case {
    /// Creates a new case.
    #[must_use]
    pub fn new(
        citation: impl Into<String>,
        short_name: impl Into<String>,
        year: u32,
        court: Court,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            citation: citation.into(),
            short_name: short_name.into(),
            year,
            court,
            jurisdiction: jurisdiction.into(),
            facts: String::new(),
            issues: Vec::new(),
            holding: String::new(),
            ratio: String::new(),
            obiter: None,
            rule: None,
            cited_cases: Vec::new(),
            overruled: false,
            overruled_by: None,
            date: NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap(),
        }
    }

    /// Sets the facts of the case.
    #[must_use]
    pub fn with_facts(mut self, facts: impl Into<String>) -> Self {
        self.facts = facts.into();
        self
    }

    /// Adds a legal issue.
    #[must_use]
    pub fn with_issue(mut self, issue: impl Into<String>) -> Self {
        self.issues.push(issue.into());
        self
    }

    /// Sets the holding.
    #[must_use]
    pub fn with_holding(mut self, holding: impl Into<String>) -> Self {
        self.holding = holding.into();
        self
    }

    /// Sets the ratio decidendi (binding reasoning).
    #[must_use]
    pub fn with_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.ratio = ratio.into();
        self
    }

    /// Sets the legal rule established.
    #[must_use]
    pub fn with_rule(mut self, rule: CaseRule) -> Self {
        self.rule = Some(rule);
        self
    }

    /// Adds a cited precedent.
    #[must_use]
    pub fn citing(mut self, case_id: Uuid) -> Self {
        self.cited_cases.push(case_id);
        self
    }

    /// Sets the decision date.
    #[must_use]
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = date;
        self
    }

    /// Sets the obiter dicta (non-binding remarks).
    #[must_use]
    pub fn with_obiter(mut self, obiter: impl Into<String>) -> Self {
        self.obiter = Some(obiter.into());
        self
    }

    /// Marks this case as overruled by another case.
    #[must_use]
    pub fn overruled_by(mut self, overruling_case_id: Uuid) -> Self {
        self.overruled = true;
        self.overruled_by = Some(overruling_case_id);
        self
    }

    /// Returns whether this case is still good law (not overruled).
    #[must_use]
    pub fn is_good_law(&self) -> bool {
        !self.overruled
    }

    /// Determines precedent weight for another case in a given jurisdiction.
    #[must_use]
    pub fn precedent_weight(
        &self,
        target_jurisdiction: &str,
        target_court: &Court,
    ) -> PrecedentWeight {
        // Same jurisdiction
        if self.jurisdiction == target_jurisdiction {
            // Higher court in same system
            if self.court.level() > target_court.level() {
                PrecedentWeight::Binding
            }
            // Same level court
            else if self.court.level() == target_court.level() {
                PrecedentWeight::StronglyPersuasive
            }
            // Lower court
            else {
                PrecedentWeight::Persuasive
            }
        }
        // Different jurisdiction (persuasive regardless of legal system)
        else {
            PrecedentWeight::Persuasive
        }
    }
}

/// Legal rule extracted from a case.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CaseRule {
    /// Name of the rule/doctrine
    pub name: String,
    /// Conditions under which the rule applies
    pub conditions: Vec<Condition>,
    /// Effect when conditions are met
    pub effect: Effect,
    /// Exceptions to the rule
    pub exceptions: Vec<String>,
}

/// Court hierarchy (裁判所の階層).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Court {
    /// Supreme Court / House of Lords (最高裁判所)
    Supreme,
    /// Court of Appeals / Court of Appeal (控訴裁判所)
    Appellate,
    /// District Court / High Court (地方裁判所)
    Trial,
    /// Specialized court (専門裁判所)
    Specialized,
}

impl Court {
    /// Returns the hierarchical level (higher = more authority).
    #[must_use]
    pub const fn level(&self) -> u8 {
        match self {
            Court::Supreme => 3,
            Court::Appellate => 2,
            Court::Trial => 1,
            Court::Specialized => 1,
        }
    }
}

impl std::fmt::Display for Court {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Court::Supreme => write!(f, "Supreme Court"),
            Court::Appellate => write!(f, "Court of Appeals"),
            Court::Trial => write!(f, "Trial Court"),
            Court::Specialized => write!(f, "Specialized Court"),
        }
    }
}

/// Weight of precedent (先例の拘束力).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PrecedentWeight {
    /// Must follow (same jurisdiction, higher court) - 拘束的先例
    Binding,
    /// Should strongly consider (same jurisdiction, same level) - 強く説得的
    StronglyPersuasive,
    /// May consider (different jurisdiction or lower court) - 説得的
    Persuasive,
    /// Distinguished or not applicable - 区別可能
    Distinguished,
}

impl std::fmt::Display for PrecedentWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrecedentWeight::Binding => write!(f, "Binding"),
            PrecedentWeight::StronglyPersuasive => write!(f, "Strongly Persuasive"),
            PrecedentWeight::Persuasive => write!(f, "Persuasive"),
            PrecedentWeight::Distinguished => write!(f, "Distinguished"),
        }
    }
}

/// Precedent relationship between cases.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Precedent {
    /// The precedent case
    pub case_id: Uuid,
    /// The case being decided
    pub target_case_id: Uuid,
    /// Weight of this precedent
    pub weight: PrecedentWeight,
    /// How the precedent was applied
    pub application: PrecedentApplication,
}

impl Precedent {
    /// Creates a new precedent relationship where the current case is distinguished from the precedent.
    ///
    /// Distinguishing a case means identifying material differences in facts such that
    /// the precedent's ratio decidendi doesn't apply to the current case.
    ///
    /// # Arguments
    /// * `precedent_case` - The case to distinguish from
    /// * `current_case` - The current case being decided
    /// * `distinguishing_factors` - Reasons why the cases differ materially
    ///
    /// # Returns
    /// A `Precedent` with application set to `Distinguished`
    ///
    /// # Example
    /// ```
    /// # use legalis_core::case_law::{Case, Precedent, Court, PrecedentWeight};
    /// # use chrono::NaiveDate;
    /// # use uuid::Uuid;
    /// let precedent_case = Case::new(
    ///     "Smith v. Jones, 100 F.2d 200 (1950)",
    ///     "Smith v. Jones",
    ///     1950,
    ///     Court::Supreme,
    ///     "US-NY"
    /// ).with_facts("Contract dispute over sale of goods");
    ///
    /// let current_case = Case::new(
    ///     "Doe v. Roe, 200 F.2d 400 (2020)",
    ///     "Doe v. Roe",
    ///     2020,
    ///     Court::Trial,
    ///     "US-NY"
    /// ).with_facts("Contract dispute over sale of services");
    ///
    /// let distinguished = Precedent::distinguish(
    ///     &precedent_case,
    ///     &current_case,
    ///     vec!["Precedent involved goods, current case involves services".to_string()],
    /// );
    /// ```
    #[must_use]
    pub fn distinguish(
        precedent_case: &Case,
        current_case: &Case,
        _distinguishing_factors: Vec<String>,
    ) -> Self {
        Self {
            case_id: precedent_case.id,
            target_case_id: current_case.id,
            weight: PrecedentWeight::Persuasive,
            application: PrecedentApplication::Distinguished,
        }
    }

    /// Creates a new precedent relationship where the current case follows the precedent's ratio decidendi.
    ///
    /// Following a precedent means applying its legal reasoning to the current case,
    /// optionally with modifications to adapt to the specific circumstances.
    ///
    /// # Arguments
    /// * `precedent_case` - The case to follow
    /// * `current_case` - The current case being decided
    /// * `modifications` - Optional modifications or clarifications to the precedent's rule
    ///
    /// # Returns
    /// A `Precedent` with application set to `Followed` or `Limited` (if modifications provided)
    ///
    /// # Example
    /// ```
    /// # use legalis_core::case_law::{Case, Precedent, Court, PrecedentWeight};
    /// # use chrono::NaiveDate;
    /// # use uuid::Uuid;
    /// let precedent_case = Case::new(
    ///     "Brown v. Board, 347 U.S. 483 (1954)",
    ///     "Brown v. Board",
    ///     1954,
    ///     Court::Supreme,
    ///     "US"
    /// ).with_holding("Separate educational facilities are inherently unequal");
    ///
    /// let current_case = Case::new(
    ///     "Similar Case, 400 U.S. 100 (1970)",
    ///     "Similar Case",
    ///     1970,
    ///     Court::Appellate,
    ///     "US"
    /// );
    ///
    /// let followed = Precedent::follow(&precedent_case, &current_case, None);
    /// ```
    #[must_use]
    pub fn follow(
        precedent_case: &Case,
        current_case: &Case,
        modifications: Option<Vec<String>>,
    ) -> Self {
        // Determine weight based on court hierarchy
        let weight = if precedent_case.court.level() > current_case.court.level() {
            PrecedentWeight::Binding
        } else if precedent_case.court == current_case.court {
            PrecedentWeight::StronglyPersuasive
        } else {
            PrecedentWeight::Persuasive
        };

        let application = if modifications.is_some() {
            PrecedentApplication::Limited
        } else {
            PrecedentApplication::Followed
        };

        Self {
            case_id: precedent_case.id,
            target_case_id: current_case.id,
            weight,
            application,
        }
    }

    /// Checks if this precedent is binding (must be followed).
    #[must_use]
    pub const fn is_binding(&self) -> bool {
        matches!(self.weight, PrecedentWeight::Binding)
    }

    /// Checks if this precedent was distinguished.
    #[must_use]
    pub const fn was_distinguished(&self) -> bool {
        matches!(self.application, PrecedentApplication::Distinguished)
    }

    /// Checks if this precedent was overruled.
    #[must_use]
    pub const fn was_overruled(&self) -> bool {
        matches!(self.application, PrecedentApplication::Overruled)
    }
}

/// How a precedent was applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PrecedentApplication {
    /// Followed the precedent directly
    Followed,
    /// Distinguished from the precedent (facts differ)
    Distinguished,
    /// Overruled the precedent
    Overruled,
    /// Affirmed the precedent
    Affirmed,
    /// Modified or limited the precedent
    Limited,
}

impl std::fmt::Display for PrecedentApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrecedentApplication::Followed => write!(f, "Followed"),
            PrecedentApplication::Distinguished => write!(f, "Distinguished"),
            PrecedentApplication::Overruled => write!(f, "Overruled"),
            PrecedentApplication::Affirmed => write!(f, "Affirmed"),
            PrecedentApplication::Limited => write!(f, "Limited"),
        }
    }
}

/// Damages in Common Law (英米法の損害賠償).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum DamageType {
    /// Compensatory damages (補償的損害賠償) - restore plaintiff to original position
    Compensatory {
        /// Economic losses (medical bills, lost wages, property damage)
        economic: u64,
        /// Non-economic losses (pain and suffering, emotional distress)
        noneconomic: u64,
    },
    /// Nominal damages (名目的損害賠償) - token amount when right violated but no harm
    Nominal(u64),
    /// Punitive damages (懲罰的損害賠償) - punish defendant for egregious conduct
    /// **This is unique to Common Law and not found in Civil Law systems**
    Punitive {
        /// Amount awarded to punish and deter
        amount: u64,
        /// Rationale for punitive award
        rationale: String,
    },
}

impl DamageType {
    /// Total monetary amount.
    #[must_use]
    pub fn total(&self) -> u64 {
        match self {
            DamageType::Compensatory {
                economic,
                noneconomic,
            } => economic + noneconomic,
            DamageType::Nominal(amount) | DamageType::Punitive { amount, .. } => *amount,
        }
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageType::Compensatory {
                economic,
                noneconomic,
            } => write!(
                f,
                "Compensatory Damages: ${} (economic: ${}, non-economic: ${})",
                economic + noneconomic,
                economic,
                noneconomic
            ),
            DamageType::Nominal(amount) => write!(f, "Nominal Damages: ${}", amount),
            DamageType::Punitive { amount, rationale } => {
                write!(f, "Punitive Damages: ${} ({})", amount, rationale)
            }
        }
    }
}

/// Case database for precedent searches.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CaseDatabase {
    cases: HashMap<Uuid, Case>,
    precedents: Vec<Precedent>,
}

impl CaseDatabase {
    /// Creates a new case database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a case to the database.
    pub fn add_case(&mut self, case: Case) {
        self.cases.insert(case.id, case);
    }

    /// Adds a precedent relationship.
    pub fn add_precedent(&mut self, precedent: Precedent) {
        self.precedents.push(precedent);
    }

    /// Finds cases by jurisdiction.
    pub fn cases_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&Case> {
        self.cases
            .values()
            .filter(|c| c.jurisdiction == jurisdiction)
            .collect()
    }

    /// Finds binding precedents for a given jurisdiction and court level.
    pub fn binding_precedents(&self, jurisdiction: &str, court: &Court) -> Vec<&Case> {
        self.cases
            .values()
            .filter(|c| {
                c.jurisdiction == jurisdiction && c.court.level() > court.level() && !c.overruled
            })
            .collect()
    }

    /// Gets a case by ID.
    #[must_use]
    pub fn get_case(&self, id: &Uuid) -> Option<&Case> {
        self.cases.get(id)
    }

    /// Returns an iterator over all cases.
    pub fn iter(&self) -> impl Iterator<Item = &Case> {
        self.cases.values()
    }

    /// Returns a mutable iterator over all cases.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Case> {
        self.cases.values_mut()
    }

    /// Returns all cases as a vector.
    pub fn all_cases(&self) -> Vec<&Case> {
        self.cases.values().collect()
    }

    /// Finds cases decided in a specific year range (inclusive).
    pub fn cases_by_year_range(&self, start_year: u32, end_year: u32) -> Vec<&Case> {
        self.cases
            .values()
            .filter(|c| c.year >= start_year && c.year <= end_year)
            .collect()
    }

    /// Finds cases by court type.
    pub fn cases_by_court(&self, court: &Court) -> Vec<&Case> {
        self.cases.values().filter(|c| &c.court == court).collect()
    }

    /// Finds cases that cite a specific case.
    pub fn cases_citing(&self, cited_case_id: &Uuid) -> Vec<&Case> {
        self.cases
            .values()
            .filter(|c| c.cited_cases.contains(cited_case_id))
            .collect()
    }

    /// Returns the number of cases in the database.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cases.len()
    }

    /// Returns whether the database is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }

    /// Returns the number of precedent relationships.
    #[must_use]
    pub fn precedent_count(&self) -> usize {
        self.precedents.len()
    }

    /// Returns an iterator over precedents.
    pub fn precedents(&self) -> impl Iterator<Item = &Precedent> {
        self.precedents.iter()
    }

    /// Creates a fluent query builder for complex case queries.
    pub fn query(&self) -> CaseQuery<'_> {
        CaseQuery::new(self)
    }

    /// Finds conflicting precedents - cases that are binding but reach different conclusions.
    ///
    /// This is useful for detecting circuit splits or conflicting case law within a jurisdiction.
    /// Returns pairs of case IDs where both are non-overruled, binding precedents that conflict.
    ///
    /// # Arguments
    /// * `jurisdiction` - Jurisdiction to check for conflicts
    /// * `court_level` - Minimum court level to consider (e.g., `Court::Appellate`)
    ///
    /// # Returns
    /// Vector of (case_id1, case_id2, reason) tuples for conflicting precedents
    ///
    /// # Example
    /// ```
    /// # use legalis_core::case_law::{CaseDatabase, Case, Court, CaseRule};
    /// # use legalis_core::{Condition, Effect};
    /// # use chrono::NaiveDate;
    /// let mut db = CaseDatabase::new();
    ///
    /// let case1 = Case::new("Case 1", "First", 2010, Court::Appellate, "US-CA")
    ///     .with_holding("Software is patentable");
    /// db.add_case(case1);
    ///
    /// let case2 = Case::new("Case 2", "Second", 2015, Court::Appellate, "US-CA")
    ///     .with_holding("Software is not patentable");
    /// db.add_case(case2);
    ///
    /// let conflicts = db.find_conflicting_precedents("US-CA", Court::Appellate);
    /// ```
    pub fn find_conflicting_precedents(
        &self,
        jurisdiction: &str,
        court_level: Court,
    ) -> Vec<(Uuid, Uuid, String)> {
        let mut conflicts = Vec::new();

        // Get all binding precedents in the jurisdiction
        let binding_cases: Vec<&Case> = self
            .cases
            .values()
            .filter(|c| {
                c.jurisdiction == jurisdiction
                    && c.court.level() >= court_level.level()
                    && !c.overruled
            })
            .collect();

        // Compare pairs of cases
        for (i, case1) in binding_cases.iter().enumerate() {
            for case2 in binding_cases.iter().skip(i + 1) {
                // Check if they address similar issues but reach different conclusions
                let has_common_issue = case1.issues.iter().any(|issue1| {
                    case2
                        .issues
                        .iter()
                        .any(|issue2| Self::issues_similar(issue1, issue2))
                });

                if has_common_issue {
                    // Simple heuristic: check if holdings contain opposing keywords
                    let holding1_lower = case1.holding.to_lowercase();
                    let holding2_lower = case2.holding.to_lowercase();

                    let conflict_detected = (holding1_lower.contains("not")
                        && !holding2_lower.contains("not"))
                        || (!holding1_lower.contains("not") && holding2_lower.contains("not"))
                        || (holding1_lower.contains("liable")
                            && holding2_lower.contains("not liable"))
                        || (holding1_lower.contains("valid") && holding2_lower.contains("invalid"));

                    if conflict_detected {
                        conflicts.push((
                            case1.id,
                            case2.id,
                            format!(
                                "Potential conflict: {} ({}) vs {} ({})",
                                case1.short_name, case1.year, case2.short_name, case2.year
                            ),
                        ));
                    }
                }
            }
        }

        conflicts
    }

    /// Checks if two legal issues are similar enough to be considered related.
    fn issues_similar(issue1: &str, issue2: &str) -> bool {
        // Simple keyword-based similarity
        let issue1_lower = issue1.to_lowercase();
        let issue2_lower = issue2.to_lowercase();

        let words1: std::collections::HashSet<_> = issue1_lower
            .split_whitespace()
            .filter(|w| w.len() > 3) // Only significant words
            .collect();
        let words2: std::collections::HashSet<_> = issue2_lower
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        let common_words = words1.intersection(&words2).count();
        let total_words = words1.len().min(words2.len());

        // Consider similar if >50% of words overlap
        total_words > 0 && common_words as f64 / total_words as f64 > 0.5
    }

    /// Finds binding precedents by legal issue.
    ///
    /// Returns cases that:
    /// - Are from higher courts (binding)
    /// - Address the specified legal issue
    /// - Have not been overruled
    /// - Are in the same jurisdiction
    ///
    /// # Example
    /// ```
    /// # use legalis_core::case_law::{CaseDatabase, Case, Court};
    /// # use chrono::NaiveDate;
    /// let mut db = CaseDatabase::new();
    ///
    /// let case = Case::new("Case", "Test Case", 2020, Court::Supreme, "US")
    ///     .with_issue("contract formation");
    /// db.add_case(case);
    ///
    /// let binding = db.binding_precedents_by_issue("US", Court::Trial, "contract formation");
    /// assert_eq!(binding.len(), 1);
    /// ```
    pub fn binding_precedents_by_issue(
        &self,
        jurisdiction: &str,
        current_court: Court,
        issue: &str,
    ) -> Vec<&Case> {
        self.cases
            .values()
            .filter(|c| {
                c.jurisdiction == jurisdiction
                    && c.court.level() > current_court.level()
                    && !c.overruled
                    && c.issues
                        .iter()
                        .any(|case_issue| Self::issues_similar(case_issue, issue))
            })
            .collect()
    }

    /// Finds cases similar to the given case using text similarity.
    ///
    /// Returns cases sorted by similarity score (highest first), excluding the query case itself.
    /// Uses a simple term frequency-based similarity measure for analogical reasoning.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::{CaseDatabase, Case, Court};
    /// use chrono::NaiveDate;
    ///
    /// let mut db = CaseDatabase::new();
    /// let case1 = Case::new("Case 1", "Case One", 2020, Court::Trial, "US")
    ///     .with_facts("negligence train platform explosion")
    ///     .with_holding("no duty of care");
    /// let id1 = case1.id;
    /// db.add_case(case1);
    ///
    /// let case2 = Case::new("Case 2", "Case Two", 2021, Court::Trial, "US")
    ///     .with_facts("negligence automobile collision")
    ///     .with_holding("duty of care established");
    /// db.add_case(case2);
    ///
    /// // Find cases similar to case1
    /// let similar = db.find_similar_cases(&id1, 5);
    /// assert!(similar.len() <= 5);
    /// ```
    pub fn find_similar_cases(&self, case_id: &Uuid, limit: usize) -> Vec<SimilarityResult> {
        let Some(query_case) = self.get_case(case_id) else {
            return Vec::new();
        };

        let query_terms = Self::tokenize_case(query_case);
        let mut scores: Vec<_> = self
            .cases
            .values()
            .filter(|c| c.id != *case_id && !c.overruled)
            .map(|c| {
                let candidate_terms = Self::tokenize_case(c);
                let score = Self::compute_similarity(&query_terms, &candidate_terms);
                SimilarityResult {
                    case_id: c.id,
                    score,
                    reason: Self::compute_similarity_reason(query_case, c),
                }
            })
            .collect();

        scores.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scores.truncate(limit);
        scores
    }

    /// Tokenizes case text into normalized terms for similarity computation.
    fn tokenize_case(case: &Case) -> HashMap<String, usize> {
        let text = format!(
            "{} {} {} {}",
            case.facts,
            case.holding,
            case.ratio,
            case.issues.join(" ")
        );

        let mut term_freq = HashMap::new();
        for word in text
            .to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3)
        {
            // Simple stopword filtering and term counting
            if !Self::is_stopword(word) {
                *term_freq.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        term_freq
    }

    /// Computes cosine similarity between two term frequency vectors.
    fn compute_similarity(terms1: &HashMap<String, usize>, terms2: &HashMap<String, usize>) -> f64 {
        if terms1.is_empty() || terms2.is_empty() {
            return 0.0;
        }

        let mut dot_product = 0usize;
        let mut magnitude1 = 0usize;
        let mut magnitude2 = 0usize;

        for (term, &freq1) in terms1 {
            magnitude1 += freq1 * freq1;
            if let Some(&freq2) = terms2.get(term) {
                dot_product += freq1 * freq2;
            }
        }

        for &freq2 in terms2.values() {
            magnitude2 += freq2 * freq2;
        }

        if magnitude1 == 0 || magnitude2 == 0 {
            return 0.0;
        }

        #[allow(clippy::cast_precision_loss)]
        let similarity =
            dot_product as f64 / ((magnitude1 as f64).sqrt() * (magnitude2 as f64).sqrt());
        similarity
    }

    /// Computes a human-readable reason for similarity.
    fn compute_similarity_reason(case1: &Case, case2: &Case) -> String {
        let mut reasons = Vec::new();

        if case1.jurisdiction == case2.jurisdiction {
            reasons.push("same jurisdiction".to_string());
        }
        if case1.court == case2.court {
            reasons.push("same court level".to_string());
        }

        let terms1 = Self::tokenize_case(case1);
        let terms2 = Self::tokenize_case(case2);
        let common_terms: Vec<_> = terms1
            .keys()
            .filter(|k| terms2.contains_key(*k))
            .take(3)
            .map(|s| s.as_str())
            .collect();

        if !common_terms.is_empty() {
            reasons.push(format!("common terms: {}", common_terms.join(", ")));
        }

        if reasons.is_empty() {
            "general similarity".to_string()
        } else {
            reasons.join("; ")
        }
    }

    /// Simple stopword check (common legal and English words to ignore).
    #[allow(dead_code)]
    fn is_stopword(word: &str) -> bool {
        matches!(
            word,
            "the"
                | "and"
                | "that"
                | "this"
                | "with"
                | "from"
                | "have"
                | "has"
                | "had"
                | "not"
                | "but"
                | "for"
                | "are"
                | "was"
                | "were"
                | "been"
                | "being"
                | "case"
                | "court"
                | "held"
        )
    }
}

/// Result of a similarity search for analogical reasoning.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SimilarityResult {
    /// ID of the similar case
    pub case_id: Uuid,
    /// Similarity score (0.0 to 1.0, higher is more similar)
    pub score: f64,
    /// Human-readable explanation of why cases are similar
    pub reason: String,
}

/// Fluent query builder for case database queries.
///
/// Allows chaining multiple filters for complex case searches.
///
/// # Examples
///
/// ```
/// use legalis_core::case_law::{CaseDatabase, Court};
/// use chrono::NaiveDate;
///
/// let db = CaseDatabase::new();
/// // Add some cases...
///
/// // Find all Supreme Court cases from US in 2020-2022 that are still good law
/// let results = db.query()
///     .jurisdiction("US")
///     .court(&Court::Supreme)
///     .year_range(2020, 2022)
///     .not_overruled()
///     .execute();
/// ```
#[derive(Clone)]
pub struct CaseQuery<'a> {
    db: &'a CaseDatabase,
    jurisdiction: Option<String>,
    court: Option<Court>,
    year_min: Option<u32>,
    year_max: Option<u32>,
    date_min: Option<NaiveDate>,
    date_max: Option<NaiveDate>,
    only_not_overruled: bool,
    only_with_rule: bool,
    // Full-text search fields
    search_facts: Option<String>,
    search_holding: Option<String>,
    search_ratio: Option<String>,
    search_all: Option<String>,
    search_keywords: Vec<String>,
}

impl<'a> CaseQuery<'a> {
    /// Creates a new query builder.
    fn new(db: &'a CaseDatabase) -> Self {
        Self {
            db,
            jurisdiction: None,
            court: None,
            year_min: None,
            year_max: None,
            date_min: None,
            date_max: None,
            only_not_overruled: false,
            only_with_rule: false,
            search_facts: None,
            search_holding: None,
            search_ratio: None,
            search_all: None,
            search_keywords: Vec::new(),
        }
    }

    /// Filters by jurisdiction.
    #[must_use]
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Filters by court type.
    #[must_use]
    pub fn court(mut self, court: &Court) -> Self {
        self.court = Some(*court);
        self
    }

    /// Filters by year range (inclusive).
    #[must_use]
    pub fn year_range(mut self, start: u32, end: u32) -> Self {
        self.year_min = Some(start);
        self.year_max = Some(end);
        self
    }

    /// Filters by minimum year.
    #[must_use]
    pub fn year_min(mut self, year: u32) -> Self {
        self.year_min = Some(year);
        self
    }

    /// Filters by maximum year.
    #[must_use]
    pub fn year_max(mut self, year: u32) -> Self {
        self.year_max = Some(year);
        self
    }

    /// Filters by date range (inclusive).
    #[must_use]
    pub fn date_range(mut self, start: NaiveDate, end: NaiveDate) -> Self {
        self.date_min = Some(start);
        self.date_max = Some(end);
        self
    }

    /// Filters by minimum date.
    #[must_use]
    pub fn date_min(mut self, date: NaiveDate) -> Self {
        self.date_min = Some(date);
        self
    }

    /// Filters by maximum date.
    #[must_use]
    pub fn date_max(mut self, date: NaiveDate) -> Self {
        self.date_max = Some(date);
        self
    }

    /// Only includes cases that have not been overruled.
    #[must_use]
    pub fn not_overruled(mut self) -> Self {
        self.only_not_overruled = true;
        self
    }

    /// Only includes cases that established a legal rule.
    #[must_use]
    pub fn with_rule(mut self) -> Self {
        self.only_with_rule = true;
        self
    }

    /// Full-text search in case facts (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::{CaseDatabase, Court};
    ///
    /// let db = CaseDatabase::new();
    /// let results = db.query()
    ///     .search_facts("negligence train platform")
    ///     .execute();
    /// ```
    #[must_use]
    pub fn search_facts(mut self, query: impl Into<String>) -> Self {
        self.search_facts = Some(query.into().to_lowercase());
        self
    }

    /// Full-text search in case holding (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::CaseDatabase;
    ///
    /// let db = CaseDatabase::new();
    /// let results = db.query()
    ///     .search_holding("duty of care")
    ///     .execute();
    /// ```
    #[must_use]
    pub fn search_holding(mut self, query: impl Into<String>) -> Self {
        self.search_holding = Some(query.into().to_lowercase());
        self
    }

    /// Full-text search in case ratio decidendi (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::CaseDatabase;
    ///
    /// let db = CaseDatabase::new();
    /// let results = db.query()
    ///     .search_ratio("foreseeability")
    ///     .execute();
    /// ```
    #[must_use]
    pub fn search_ratio(mut self, query: impl Into<String>) -> Self {
        self.search_ratio = Some(query.into().to_lowercase());
        self
    }

    /// Full-text search across all text fields (facts, holding, ratio, issues).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::CaseDatabase;
    ///
    /// let db = CaseDatabase::new();
    /// let results = db.query()
    ///     .search_all("proximate cause")
    ///     .execute();
    /// ```
    #[must_use]
    pub fn search_all(mut self, query: impl Into<String>) -> Self {
        self.search_all = Some(query.into().to_lowercase());
        self
    }

    /// Search for multiple keywords (AND logic - all must be present).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::case_law::CaseDatabase;
    ///
    /// let db = CaseDatabase::new();
    /// let results = db.query()
    ///     .search_keywords(vec!["negligence", "duty", "breach"])
    ///     .execute();
    /// ```
    #[must_use]
    pub fn search_keywords(mut self, keywords: Vec<impl Into<String>>) -> Self {
        self.search_keywords = keywords
            .into_iter()
            .map(|k| k.into().to_lowercase())
            .collect();
        self
    }

    /// Executes the query and returns matching cases.
    pub fn execute(&self) -> Vec<&Case> {
        self.db.cases.values().filter(|c| self.matches(c)).collect()
    }

    /// Counts matching cases without collecting them.
    pub fn count(&self) -> usize {
        self.db.cases.values().filter(|c| self.matches(c)).count()
    }

    /// Returns the first matching case, if any.
    pub fn first(&self) -> Option<&Case> {
        self.db.cases.values().find(|c| self.matches(c))
    }

    /// Checks if a case matches all filters.
    fn matches(&self, case: &Case) -> bool {
        if let Some(ref j) = self.jurisdiction
            && &case.jurisdiction != j
        {
            return false;
        }

        if let Some(ref c) = self.court
            && &case.court != c
        {
            return false;
        }

        if let Some(min) = self.year_min
            && case.year < min
        {
            return false;
        }

        if let Some(max) = self.year_max
            && case.year > max
        {
            return false;
        }

        if let Some(min) = self.date_min
            && case.date < min
        {
            return false;
        }

        if let Some(max) = self.date_max
            && case.date > max
        {
            return false;
        }

        if self.only_not_overruled && case.overruled {
            return false;
        }

        if self.only_with_rule && case.rule.is_none() {
            return false;
        }

        // Full-text search filters
        if let Some(ref query) = self.search_facts
            && !case.facts.to_lowercase().contains(query)
        {
            return false;
        }

        if let Some(ref query) = self.search_holding
            && !case.holding.to_lowercase().contains(query)
        {
            return false;
        }

        if let Some(ref query) = self.search_ratio
            && !case.ratio.to_lowercase().contains(query)
        {
            return false;
        }

        if let Some(ref query) = self.search_all {
            let all_text = format!(
                "{} {} {} {}",
                case.facts.to_lowercase(),
                case.holding.to_lowercase(),
                case.ratio.to_lowercase(),
                case.issues.join(" ").to_lowercase()
            );
            if !all_text.contains(query) {
                return false;
            }
        }

        // Keyword search - all keywords must be present in combined text
        if !self.search_keywords.is_empty() {
            let all_text = format!(
                "{} {} {} {}",
                case.facts.to_lowercase(),
                case.holding.to_lowercase(),
                case.ratio.to_lowercase(),
                case.issues.join(" ").to_lowercase()
            );
            for keyword in &self.search_keywords {
                if !all_text.contains(keyword) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_creation() {
        let case = Case::new(
            "Palsgraf v. Long Island R.R., 248 N.Y. 339 (1928)",
            "Palsgraf v. Long Island",
            1928,
            Court::Appellate,
            "US-NY",
        )
        .with_facts("Plaintiff injured by explosion on train platform")
        .with_holding("No liability - unforeseeable plaintiff");

        assert_eq!(case.year, 1928);
        assert_eq!(case.court, Court::Appellate);
        assert!(!case.overruled);
    }

    #[test]
    fn test_court_hierarchy() {
        assert!(Court::Supreme.level() > Court::Appellate.level());
        assert!(Court::Appellate.level() > Court::Trial.level());
    }

    #[test]
    fn test_precedent_weight_same_jurisdiction() {
        let supreme_case = Case::new("Test v. Case", "Test", 2020, Court::Supreme, "US-NY");

        // Supreme court case is binding on trial court in same jurisdiction
        assert_eq!(
            supreme_case.precedent_weight("US-NY", &Court::Trial),
            PrecedentWeight::Binding
        );

        // Same level is strongly persuasive
        assert_eq!(
            supreme_case.precedent_weight("US-NY", &Court::Supreme),
            PrecedentWeight::StronglyPersuasive
        );
    }

    #[test]
    fn test_precedent_weight_different_jurisdiction() {
        let ny_case = Case::new("Test v. Case", "Test", 2020, Court::Supreme, "US-NY");

        // Different US state is persuasive
        assert_eq!(
            ny_case.precedent_weight("US-CA", &Court::Trial),
            PrecedentWeight::Persuasive
        );

        // Different country is also persuasive
        assert_eq!(
            ny_case.precedent_weight("UK", &Court::Trial),
            PrecedentWeight::Persuasive
        );
    }

    #[test]
    fn test_damage_types() {
        let comp = DamageType::Compensatory {
            economic: 10000,
            noneconomic: 5000,
        };
        assert_eq!(comp.total(), 15000);

        let pun = DamageType::Punitive {
            amount: 100000,
            rationale: "Gross negligence".to_string(),
        };
        assert_eq!(pun.total(), 100000);
    }

    #[test]
    fn test_case_database() {
        let mut db = CaseDatabase::new();

        let case = Case::new(
            "Donoghue v Stevenson [1932] AC 562",
            "Donoghue v Stevenson",
            1932,
            Court::Supreme,
            "UK",
        );
        let case_id = case.id;

        db.add_case(case);

        assert!(db.get_case(&case_id).is_some());
        assert_eq!(db.cases_by_jurisdiction("UK").len(), 1);
    }

    #[test]
    fn test_court_display() {
        assert_eq!(Court::Supreme.to_string(), "Supreme Court");
        assert_eq!(Court::Appellate.to_string(), "Court of Appeals");
        assert_eq!(Court::Trial.to_string(), "Trial Court");
        assert_eq!(Court::Specialized.to_string(), "Specialized Court");
    }

    #[test]
    fn test_precedent_weight_display() {
        assert_eq!(PrecedentWeight::Binding.to_string(), "Binding");
        assert_eq!(
            PrecedentWeight::StronglyPersuasive.to_string(),
            "Strongly Persuasive"
        );
        assert_eq!(PrecedentWeight::Persuasive.to_string(), "Persuasive");
        assert_eq!(PrecedentWeight::Distinguished.to_string(), "Distinguished");
    }

    #[test]
    fn test_precedent_application_display() {
        assert_eq!(PrecedentApplication::Followed.to_string(), "Followed");
        assert_eq!(
            PrecedentApplication::Distinguished.to_string(),
            "Distinguished"
        );
        assert_eq!(PrecedentApplication::Overruled.to_string(), "Overruled");
        assert_eq!(PrecedentApplication::Affirmed.to_string(), "Affirmed");
        assert_eq!(PrecedentApplication::Limited.to_string(), "Limited");
    }

    #[test]
    fn test_damage_type_display() {
        let comp = DamageType::Compensatory {
            economic: 10000,
            noneconomic: 5000,
        };
        assert_eq!(
            comp.to_string(),
            "Compensatory Damages: $15000 (economic: $10000, non-economic: $5000)"
        );

        let nom = DamageType::Nominal(100);
        assert_eq!(nom.to_string(), "Nominal Damages: $100");

        let pun = DamageType::Punitive {
            amount: 100000,
            rationale: "Gross negligence".to_string(),
        };
        assert_eq!(
            pun.to_string(),
            "Punitive Damages: $100000 (Gross negligence)"
        );
    }

    #[test]
    fn test_precedent_weight_ordering() {
        assert!(PrecedentWeight::Binding < PrecedentWeight::StronglyPersuasive);
        assert!(PrecedentWeight::StronglyPersuasive < PrecedentWeight::Persuasive);
        assert!(PrecedentWeight::Persuasive < PrecedentWeight::Distinguished);
    }

    #[test]
    fn test_precedent_application_ordering() {
        assert!(PrecedentApplication::Followed < PrecedentApplication::Distinguished);
        assert!(PrecedentApplication::Distinguished < PrecedentApplication::Overruled);
    }

    #[test]
    fn test_case_database_iterators() {
        let mut db = CaseDatabase::new();
        assert!(db.is_empty());
        assert_eq!(db.len(), 0);

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US-NY");
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US-CA");
        let case3 = Case::new("Case 3", "Case3", 2020, Court::Trial, "US-NY");

        db.add_case(case1.clone());
        db.add_case(case2.clone());
        db.add_case(case3.clone());

        assert_eq!(db.len(), 3);
        assert!(!db.is_empty());

        let all = db.all_cases();
        assert_eq!(all.len(), 3);

        let count = db.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_case_database_queries() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US-NY");
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US-CA");
        let case3 = Case::new("Case 3", "Case3", 2022, Court::Supreme, "US-NY");

        db.add_case(case1.clone());
        db.add_case(case2.clone());
        db.add_case(case3.clone());

        // Test year range query
        let cases_2020_2021 = db.cases_by_year_range(2020, 2021);
        assert_eq!(cases_2020_2021.len(), 2);

        // Test court query
        let supreme_cases = db.cases_by_court(&Court::Supreme);
        assert_eq!(supreme_cases.len(), 2);

        // Test jurisdiction query
        let ny_cases = db.cases_by_jurisdiction("US-NY");
        assert_eq!(ny_cases.len(), 2);
    }

    #[test]
    fn test_case_database_citing() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US-NY");
        let case1_id = case1.id;

        let mut case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US-CA");
        case2.cited_cases.push(case1_id);

        db.add_case(case1);
        db.add_case(case2);

        let citing = db.cases_citing(&case1_id);
        assert_eq!(citing.len(), 1);
        assert_eq!(citing[0].short_name, "Case2");
    }

    #[test]
    fn test_query_builder_jurisdiction() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US-NY");
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US-CA");
        let case3 = Case::new("Case 3", "Case3", 2022, Court::Supreme, "US-NY");

        db.add_case(case1);
        db.add_case(case2);
        db.add_case(case3);

        let query_ny = db.query().jurisdiction("US-NY");
        let ny_cases = query_ny.execute();
        assert_eq!(ny_cases.len(), 2);

        let query_ca = db.query().jurisdiction("US-CA");
        let ca_cases = query_ca.execute();
        assert_eq!(ca_cases.len(), 1);
    }

    #[test]
    fn test_query_builder_year_range() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US");
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US");
        let case3 = Case::new("Case 3", "Case3", 2022, Court::Supreme, "US");

        db.add_case(case1);
        db.add_case(case2);
        db.add_case(case3);

        let query_2020_2021 = db.query().year_range(2020, 2021);
        let cases_2020_2021 = query_2020_2021.execute();
        assert_eq!(cases_2020_2021.len(), 2);

        let query_from_2021 = db.query().year_min(2021);
        let cases_from_2021 = query_from_2021.execute();
        assert_eq!(cases_from_2021.len(), 2);

        let query_until_2021 = db.query().year_max(2021);
        let cases_until_2021 = query_until_2021.execute();
        assert_eq!(cases_until_2021.len(), 2);
    }

    #[test]
    fn test_query_builder_chaining() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US-NY")
            .with_date(NaiveDate::from_ymd_opt(2020, 5, 15).unwrap());
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US-CA")
            .with_date(NaiveDate::from_ymd_opt(2021, 3, 10).unwrap());
        let case3 = Case::new("Case 3", "Case3", 2020, Court::Supreme, "US-NY")
            .with_date(NaiveDate::from_ymd_opt(2020, 8, 22).unwrap());

        db.add_case(case1);
        db.add_case(case2);
        db.add_case(case3);

        // Chain multiple filters
        let query = db
            .query()
            .jurisdiction("US-NY")
            .court(&Court::Supreme)
            .year_range(2020, 2021);
        let results = query.execute();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_builder_not_overruled() {
        let mut db = CaseDatabase::new();

        let mut case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US");
        case1.overruled = true;

        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US");

        db.add_case(case1);
        db.add_case(case2);

        let query = db.query().not_overruled();
        let good_law = query.execute();
        assert_eq!(good_law.len(), 1);
        assert_eq!(good_law[0].short_name, "Case2");
    }

    #[test]
    fn test_query_builder_with_rule() {
        let mut db = CaseDatabase::new();

        let mut case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US");
        case1.rule = Some(CaseRule {
            name: "Test Rule".to_string(),
            conditions: vec![],
            effect: Effect::grant("Test effect"),
            exceptions: vec![],
        });

        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US");

        db.add_case(case1);
        db.add_case(case2);

        let query = db.query().with_rule();
        let with_rules = query.execute();
        assert_eq!(with_rules.len(), 1);
        assert_eq!(with_rules[0].short_name, "Case1");
    }

    #[test]
    fn test_query_builder_count_and_first() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US");
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US");

        db.add_case(case1);
        db.add_case(case2);

        let query = db.query().jurisdiction("US");

        assert_eq!(query.count(), 2);

        let first = query.first();
        assert!(first.is_some());
        assert!(first.unwrap().jurisdiction == "US");
    }

    #[test]
    fn test_query_builder_date_range() {
        let mut db = CaseDatabase::new();

        let case1 = Case::new("Case 1", "Case1", 2020, Court::Supreme, "US")
            .with_date(NaiveDate::from_ymd_opt(2020, 5, 15).unwrap());
        let case2 = Case::new("Case 2", "Case2", 2021, Court::Appellate, "US")
            .with_date(NaiveDate::from_ymd_opt(2021, 3, 10).unwrap());
        let case3 = Case::new("Case 3", "Case3", 2022, Court::Supreme, "US")
            .with_date(NaiveDate::from_ymd_opt(2022, 8, 22).unwrap());

        db.add_case(case1);
        db.add_case(case2);
        db.add_case(case3);

        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2021, 12, 31).unwrap();

        let query = db.query().date_range(start, end);
        let results = query.execute();
        assert_eq!(results.len(), 2);
    }
}
