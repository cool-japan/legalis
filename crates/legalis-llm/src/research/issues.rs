//! Legal issue identification (issue spotting).
//!
//! [`IssueSpotter`] scans a fact pattern for the presence of recognised legal
//! issues. Each issue is defined by a set of *elements*, and each element by a
//! set of trigger synonyms. An element counts as present if any of its
//! synonyms appears in the facts (single-word synonyms are matched after
//! stemming, multi-word synonyms by phrase). The fraction of an issue's
//! elements that are present becomes the spotting confidence, which lets the
//! spotter also report which elements are *missing* - useful for flagging weak
//! claims.
//!
//! The default catalogue covers common first-year subjects; the catalogue is
//! fully extensible via [`IssueSpotter::with_issue`].

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Broad area of law an issue belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalArea {
    /// Tort law.
    Torts,
    /// Contract law.
    Contracts,
    /// Property law.
    Property,
    /// Criminal law.
    CriminalLaw,
    /// Constitutional law.
    ConstitutionalLaw,
    /// Civil procedure.
    CivilProcedure,
    /// Evidence.
    Evidence,
    /// Anything else.
    Other,
}

impl LegalArea {
    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            LegalArea::Torts => "Torts",
            LegalArea::Contracts => "Contracts",
            LegalArea::Property => "Property",
            LegalArea::CriminalLaw => "Criminal Law",
            LegalArea::ConstitutionalLaw => "Constitutional Law",
            LegalArea::CivilProcedure => "Civil Procedure",
            LegalArea::Evidence => "Evidence",
            LegalArea::Other => "Other",
        }
    }
}

/// One element of a legal issue, defined by its trigger synonyms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueElement {
    /// Short label for the element (e.g. `duty`).
    pub label: String,
    /// Trigger synonyms that indicate the element is present.
    pub synonyms: Vec<String>,
}

impl IssueElement {
    /// Creates an element from a label and a list of synonyms.
    pub fn new(label: impl Into<String>, synonyms: &[&str]) -> Self {
        Self {
            label: label.into(),
            synonyms: synonyms.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// A legal issue defined by its constituent elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueDefinition {
    /// Name of the issue (e.g. `Negligence`).
    pub name: String,
    /// Area of law.
    pub area: LegalArea,
    /// Elements that make up the issue.
    pub elements: Vec<IssueElement>,
}

impl IssueDefinition {
    /// Creates an issue definition.
    pub fn new(name: impl Into<String>, area: LegalArea, elements: Vec<IssueElement>) -> Self {
        Self {
            name: name.into(),
            area,
            elements,
        }
    }
}

/// A legal issue spotted in a fact pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpottedIssue {
    /// Name of the issue.
    pub issue: String,
    /// Area of law.
    pub area: LegalArea,
    /// Fraction of elements present, in `[0, 1]`.
    pub confidence: f64,
    /// Element labels found in the facts.
    pub matched_elements: Vec<String>,
    /// Element labels absent from the facts.
    pub missing_elements: Vec<String>,
}

/// Identifies legal issues present in a fact pattern.
#[derive(Debug, Clone)]
pub struct IssueSpotter {
    catalog: Vec<IssueDefinition>,
    min_confidence: f64,
}

impl Default for IssueSpotter {
    fn default() -> Self {
        Self::new()
    }
}

impl IssueSpotter {
    /// Creates a spotter pre-loaded with the default issue catalogue.
    pub fn new() -> Self {
        Self {
            catalog: default_catalog(),
            min_confidence: 0.0,
        }
    }

    /// Creates a spotter with an empty catalogue.
    pub fn empty() -> Self {
        Self {
            catalog: Vec::new(),
            min_confidence: 0.0,
        }
    }

    /// Adds an issue definition to the catalogue.
    pub fn with_issue(mut self, issue: IssueDefinition) -> Self {
        self.catalog.push(issue);
        self
    }

    /// Sets the minimum confidence required to report an issue.
    pub fn with_min_confidence(mut self, min_confidence: f64) -> Self {
        self.min_confidence = min_confidence.clamp(0.0, 1.0);
        self
    }

    /// Returns the number of issues in the catalogue.
    pub fn catalog_size(&self) -> usize {
        self.catalog.len()
    }

    /// Spots all issues present in the facts, most confident first.
    pub fn spot(&self, facts: &str) -> Vec<SpottedIssue> {
        let facts_lower = facts.to_lowercase();
        let token_set: HashSet<String> = super::tokenize(facts).into_iter().collect();

        let mut results: Vec<SpottedIssue> = Vec::new();
        for def in &self.catalog {
            if def.elements.is_empty() {
                continue;
            }
            let mut matched = Vec::new();
            let mut missing = Vec::new();
            for element in &def.elements {
                if element_present(element, &facts_lower, &token_set) {
                    matched.push(element.label.clone());
                } else {
                    missing.push(element.label.clone());
                }
            }
            if matched.is_empty() {
                continue;
            }
            let confidence = matched.len() as f64 / def.elements.len() as f64;
            if confidence < self.min_confidence {
                continue;
            }
            results.push(SpottedIssue {
                issue: def.name.clone(),
                area: def.area,
                confidence,
                matched_elements: matched,
                missing_elements: missing,
            });
        }

        results.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.issue.cmp(&b.issue))
        });
        results
    }

    /// Spots issues restricted to a single area of law.
    pub fn spot_in_area(&self, facts: &str, area: LegalArea) -> Vec<SpottedIssue> {
        self.spot(facts)
            .into_iter()
            .filter(|i| i.area == area)
            .collect()
    }
}

/// Returns whether an element is present, matching single-word synonyms after
/// stemming and multi-word synonyms by case-insensitive phrase containment.
fn element_present(element: &IssueElement, facts_lower: &str, token_set: &HashSet<String>) -> bool {
    for synonym in &element.synonyms {
        let lowered = synonym.to_lowercase();
        let matched = if lowered.contains(' ') {
            facts_lower.contains(&lowered)
        } else {
            token_set.contains(&super::stem(&lowered))
        };
        if matched {
            return true;
        }
    }
    false
}

fn default_catalog() -> Vec<IssueDefinition> {
    use LegalArea::*;
    vec![
        IssueDefinition::new(
            "Negligence",
            Torts,
            vec![
                IssueElement::new(
                    "duty",
                    &["duty", "duty of care", "owed a duty", "obligation"],
                ),
                IssueElement::new(
                    "breach",
                    &[
                        "breach",
                        "breached",
                        "negligent",
                        "unreasonable",
                        "failed to",
                    ],
                ),
                IssueElement::new(
                    "causation",
                    &[
                        "caused",
                        "causation",
                        "proximate cause",
                        "but for",
                        "foreseeable",
                    ],
                ),
                IssueElement::new("damages", &["damage", "damages", "injury", "harm", "loss"]),
            ],
        ),
        IssueDefinition::new(
            "Battery",
            Torts,
            vec![
                IssueElement::new("intent", &["intentional", "intent", "deliberately"]),
                IssueElement::new(
                    "contact",
                    &["contact", "touched", "struck", "hit", "punched"],
                ),
                IssueElement::new("harm", &["harmful", "offensive", "injury"]),
            ],
        ),
        IssueDefinition::new(
            "Defamation",
            Torts,
            vec![
                IssueElement::new(
                    "statement",
                    &["defamatory", "defamation", "libel", "slander"],
                ),
                IssueElement::new("falsity", &["false statement", "false", "untrue"]),
                IssueElement::new("publication", &["published", "publication", "told"]),
                IssueElement::new("harm", &["reputation", "harm", "damages"]),
            ],
        ),
        IssueDefinition::new(
            "Breach of Contract",
            Contracts,
            vec![
                IssueElement::new("contract", &["contract", "agreement", "promise"]),
                IssueElement::new(
                    "breach",
                    &["breach", "breached", "failed to perform", "default"],
                ),
                IssueElement::new("damages", &["damage", "damages", "loss", "harm"]),
            ],
        ),
        IssueDefinition::new(
            "Contract Formation",
            Contracts,
            vec![
                IssueElement::new("offer", &["offer", "offered", "proposal"]),
                IssueElement::new("acceptance", &["acceptance", "accepted", "agreed"]),
                IssueElement::new(
                    "consideration",
                    &["consideration", "bargained for", "exchange", "promise"],
                ),
            ],
        ),
        IssueDefinition::new(
            "Statute of Frauds",
            Contracts,
            vec![IssueElement::new(
                "writing",
                &["statute of frauds", "in writing", "writing", "signed"],
            )],
        ),
        IssueDefinition::new(
            "Adverse Possession",
            Property,
            vec![
                IssueElement::new(
                    "possession",
                    &["adverse possession", "possession", "possessed"],
                ),
                IssueElement::new("open", &["open and notorious", "open", "notorious"]),
                IssueElement::new("continuous", &["continuous", "continuously"]),
                IssueElement::new("hostile", &["hostile", "without permission"]),
                IssueElement::new("exclusive", &["exclusive", "exclusively"]),
            ],
        ),
        IssueDefinition::new(
            "Burglary",
            CriminalLaw,
            vec![
                IssueElement::new("breaking", &["breaking", "broke", "forced entry"]),
                IssueElement::new("entering", &["entering", "entered", "entry"]),
                IssueElement::new("dwelling", &["dwelling", "building", "house", "residence"]),
                IssueElement::new("intent", &["intent", "intended", "purpose"]),
            ],
        ),
        IssueDefinition::new(
            "Larceny",
            CriminalLaw,
            vec![
                IssueElement::new("taking", &["taking", "took", "theft", "larceny", "stole"]),
                IssueElement::new("property", &["property", "goods", "belongings"]),
                IssueElement::new(
                    "intent",
                    &["intent to permanently deprive", "deprive", "permanently"],
                ),
            ],
        ),
        IssueDefinition::new(
            "Equal Protection",
            ConstitutionalLaw,
            vec![
                IssueElement::new(
                    "classification",
                    &[
                        "classification",
                        "race",
                        "gender",
                        "suspect class",
                        "national origin",
                    ],
                ),
                IssueElement::new(
                    "discrimination",
                    &[
                        "equal protection",
                        "discrimination",
                        "discriminate",
                        "unequal",
                    ],
                ),
            ],
        ),
        IssueDefinition::new(
            "Due Process",
            ConstitutionalLaw,
            vec![
                IssueElement::new("deprivation", &["due process", "deprivation", "deprived"]),
                IssueElement::new("interest", &["life", "liberty", "property"]),
                IssueElement::new(
                    "procedure",
                    &["notice", "hearing", "opportunity to be heard"],
                ),
            ],
        ),
        IssueDefinition::new(
            "Fourth Amendment Search and Seizure",
            ConstitutionalLaw,
            vec![
                IssueElement::new(
                    "search",
                    &["fourth amendment", "search", "seizure", "searched"],
                ),
                IssueElement::new("warrant", &["warrant", "probable cause"]),
                IssueElement::new("privacy", &["reasonable expectation of privacy", "privacy"]),
            ],
        ),
        IssueDefinition::new(
            "Personal Jurisdiction",
            CivilProcedure,
            vec![
                IssueElement::new(
                    "contacts",
                    &[
                        "personal jurisdiction",
                        "minimum contacts",
                        "purposeful availment",
                    ],
                ),
                IssueElement::new("fairness", &["fair play", "substantial justice", "forum"]),
            ],
        ),
        IssueDefinition::new(
            "Subject Matter Jurisdiction",
            CivilProcedure,
            vec![IssueElement::new(
                "basis",
                &[
                    "subject matter jurisdiction",
                    "diversity",
                    "federal question",
                    "amount in controversy",
                ],
            )],
        ),
        IssueDefinition::new(
            "Hearsay",
            Evidence,
            vec![
                IssueElement::new(
                    "statement",
                    &["hearsay", "out of court statement", "out-of-court"],
                ),
                IssueElement::new("purpose", &["truth of the matter", "to prove", "asserted"]),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_catalog_loaded() {
        let spotter = IssueSpotter::new();
        assert!(spotter.catalog_size() >= 12);
    }

    #[test]
    fn test_spot_negligence_full() {
        let spotter = IssueSpotter::new();
        let facts = "The defendant owed a duty of care to the plaintiff but breached that \
                     duty when he negligently caused the collision, and the breach caused \
                     damages and injury to the plaintiff.";
        let issues = spotter.spot(facts);
        let negligence = issues
            .iter()
            .find(|i| i.issue == "Negligence")
            .expect("negligence spotted");
        assert!((negligence.confidence - 1.0).abs() < 1e-9);
        assert!(negligence.missing_elements.is_empty());
        assert_eq!(negligence.area, LegalArea::Torts);
    }

    #[test]
    fn test_spot_reports_missing_elements() {
        let spotter = IssueSpotter::new();
        // Duty mentioned, but no breach/causation/damages language.
        let facts = "The parties discussed whether a duty of care existed.";
        let negligence = spotter
            .spot(facts)
            .into_iter()
            .find(|i| i.issue == "Negligence")
            .expect("negligence partially spotted");
        assert!(negligence.confidence < 1.0);
        assert!(negligence.matched_elements.contains(&"duty".to_string()));
        assert!(
            negligence
                .missing_elements
                .contains(&"causation".to_string())
        );
    }

    #[test]
    fn test_spot_contract_issue() {
        let spotter = IssueSpotter::new();
        let facts = "There was a valid contract, but the seller breached the agreement, \
                     causing the buyer substantial damages.";
        let issues = spotter.spot(facts);
        assert!(issues.iter().any(|i| i.issue == "Breach of Contract"));
    }

    #[test]
    fn test_spot_in_area_filter() {
        let spotter = IssueSpotter::new();
        let facts = "The state classification discriminates on the basis of race in \
                     violation of equal protection, and the defendant also breached a contract.";
        let con_issues = spotter.spot_in_area(facts, LegalArea::ConstitutionalLaw);
        assert!(
            con_issues
                .iter()
                .all(|i| i.area == LegalArea::ConstitutionalLaw)
        );
        assert!(con_issues.iter().any(|i| i.issue == "Equal Protection"));
    }

    #[test]
    fn test_min_confidence_filter() {
        let spotter = IssueSpotter::new().with_min_confidence(0.99);
        let facts = "The parties merely discussed a duty.";
        // Only the "duty" element matches negligence (0.25), filtered out by threshold.
        assert!(spotter.spot(facts).iter().all(|i| i.issue != "Negligence"));
    }

    #[test]
    fn test_custom_issue() {
        let custom = IssueDefinition::new(
            "Securities Fraud",
            LegalArea::Other,
            vec![
                IssueElement::new(
                    "misrepresentation",
                    &["misrepresentation", "material misstatement"],
                ),
                IssueElement::new("scienter", &["scienter", "intent to deceive"]),
            ],
        );
        let spotter = IssueSpotter::empty().with_issue(custom);
        let facts = "The defendant made a material misstatement with intent to deceive investors.";
        let issues = spotter.spot(facts);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue, "Securities Fraud");
        assert!((issues[0].confidence - 1.0).abs() < 1e-9);
    }
}
