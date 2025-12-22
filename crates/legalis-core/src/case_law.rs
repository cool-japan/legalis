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
}
