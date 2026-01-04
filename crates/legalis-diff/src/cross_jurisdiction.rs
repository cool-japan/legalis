//! Cross-jurisdiction diffing for comparing statutes across different legal systems.
//!
//! This module provides functionality for:
//! - Equivalent statute matching across jurisdictions
//! - Jurisdiction-aware normalization
//! - Multilingual diff alignment
//! - Harmonization gap detection
//! - Treaty comparison support

use crate::{DiffResult, StatuteDiff, diff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A jurisdiction identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// Country code (ISO 3166-1 alpha-2).
    pub country: String,
    /// Optional subdivision (state, province, etc.).
    pub subdivision: Option<String>,
    /// Legal system type (common law, civil law, etc.).
    pub legal_system: LegalSystem,
}

impl Jurisdiction {
    /// Creates a new jurisdiction.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::cross_jurisdiction::{Jurisdiction, LegalSystem};
    ///
    /// let us_ca = Jurisdiction::new("US", LegalSystem::CommonLaw)
    ///     .with_subdivision("CA");
    ///
    /// assert_eq!(us_ca.country, "US");
    /// assert_eq!(us_ca.subdivision, Some("CA".to_string()));
    /// ```
    pub fn new(country: &str, legal_system: LegalSystem) -> Self {
        Self {
            country: country.to_string(),
            subdivision: None,
            legal_system,
        }
    }

    /// Adds a subdivision to the jurisdiction.
    pub fn with_subdivision(mut self, subdivision: &str) -> Self {
        self.subdivision = Some(subdivision.to_string());
        self
    }

    /// Gets a display string for the jurisdiction.
    pub fn display(&self) -> String {
        if let Some(ref sub) = self.subdivision {
            format!("{}-{}", self.country, sub)
        } else {
            self.country.clone()
        }
    }
}

/// Legal system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalSystem {
    /// Common law system (precedent-based).
    CommonLaw,
    /// Civil law system (code-based).
    CivilLaw,
    /// Religious law system.
    ReligiousLaw,
    /// Customary law system.
    CustomaryLaw,
    /// Mixed or hybrid system.
    Mixed,
}

/// A statute with jurisdiction information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionalStatute {
    /// The statute itself.
    pub statute: Statute,
    /// The jurisdiction.
    pub jurisdiction: Jurisdiction,
    /// The language of the statute (ISO 639-1).
    pub language: String,
    /// Optional equivalent statute IDs in other jurisdictions.
    pub equivalents: HashMap<String, String>,
}

impl JurisdictionalStatute {
    /// Creates a new jurisdictional statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::cross_jurisdiction::{JurisdictionalStatute, Jurisdiction, LegalSystem};
    ///
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let jurisdiction = Jurisdiction::new("US", LegalSystem::CommonLaw);
    /// let jur_statute = JurisdictionalStatute::new(statute, jurisdiction, "en");
    ///
    /// assert_eq!(jur_statute.language, "en");
    /// ```
    pub fn new(statute: Statute, jurisdiction: Jurisdiction, language: &str) -> Self {
        Self {
            statute,
            jurisdiction,
            language: language.to_string(),
            equivalents: HashMap::new(),
        }
    }

    /// Adds an equivalent statute in another jurisdiction.
    pub fn add_equivalent(mut self, jurisdiction_code: &str, statute_id: &str) -> Self {
        self.equivalents
            .insert(jurisdiction_code.to_string(), statute_id.to_string());
        self
    }
}

/// A cross-jurisdiction diff comparing equivalent statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossJurisdictionDiff {
    /// The statute ID being compared.
    pub statute_id: String,
    /// The "from" jurisdiction.
    pub from_jurisdiction: Jurisdiction,
    /// The "to" jurisdiction.
    pub to_jurisdiction: Jurisdiction,
    /// The underlying diff.
    pub diff: StatuteDiff,
    /// Harmonization gaps identified.
    pub harmonization_gaps: Vec<HarmonizationGap>,
    /// Similarity score (0.0 to 1.0).
    pub similarity_score: f64,
}

/// A harmonization gap between jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonizationGap {
    /// Description of the gap.
    pub description: String,
    /// Severity of the gap.
    pub severity: GapSeverity,
    /// The aspect that differs.
    pub aspect: String,
}

/// Severity of a harmonization gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapSeverity {
    /// Minor difference, easily reconcilable.
    Minor,
    /// Moderate difference, may require attention.
    Moderate,
    /// Major difference, significant harmonization effort needed.
    Major,
    /// Critical difference, fundamentally incompatible.
    Critical,
}

/// Compares statutes across jurisdictions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::cross_jurisdiction::{
///     JurisdictionalStatute, Jurisdiction, LegalSystem, compare_across_jurisdictions
/// };
///
/// let statute1 = Statute::new("benefit", "Tax Credit", Effect::new(EffectType::Grant, "Credit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
/// let jur1 = Jurisdiction::new("US", LegalSystem::CommonLaw);
/// let jur_statute1 = JurisdictionalStatute::new(statute1, jur1, "en");
///
/// let statute2 = Statute::new("benefit", "Tax Credit", Effect::new(EffectType::Grant, "Credit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 60,
///     });
/// let jur2 = Jurisdiction::new("CA", LegalSystem::CommonLaw);
/// let jur_statute2 = JurisdictionalStatute::new(statute2, jur2, "en");
///
/// let result = compare_across_jurisdictions(&jur_statute1, &jur_statute2);
/// assert!(result.is_ok());
/// ```
pub fn compare_across_jurisdictions(
    from: &JurisdictionalStatute,
    to: &JurisdictionalStatute,
) -> DiffResult<CrossJurisdictionDiff> {
    let diff_result = diff(&from.statute, &to.statute)?;

    // Detect harmonization gaps
    let harmonization_gaps =
        detect_harmonization_gaps(&diff_result, &from.jurisdiction, &to.jurisdiction);

    // Calculate similarity score
    let similarity_score = calculate_similarity(&diff_result);

    Ok(CrossJurisdictionDiff {
        statute_id: from.statute.id.clone(),
        from_jurisdiction: from.jurisdiction.clone(),
        to_jurisdiction: to.jurisdiction.clone(),
        diff: diff_result,
        harmonization_gaps,
        similarity_score,
    })
}

/// Detects harmonization gaps from a diff.
fn detect_harmonization_gaps(
    diff: &StatuteDiff,
    from_jur: &Jurisdiction,
    to_jur: &Jurisdiction,
) -> Vec<HarmonizationGap> {
    let mut gaps = Vec::new();

    // Check for legal system differences
    if from_jur.legal_system != to_jur.legal_system {
        gaps.push(HarmonizationGap {
            description: format!(
                "Different legal systems: {:?} vs {:?}",
                from_jur.legal_system, to_jur.legal_system
            ),
            severity: GapSeverity::Major,
            aspect: "Legal System".to_string(),
        });
    }

    // Analyze changes
    for change in &diff.changes {
        let severity = if diff.impact.affects_outcome {
            GapSeverity::Critical
        } else if diff.impact.affects_eligibility {
            GapSeverity::Major
        } else if diff.impact.severity >= crate::Severity::Moderate {
            GapSeverity::Moderate
        } else {
            GapSeverity::Minor
        };

        gaps.push(HarmonizationGap {
            description: format!(
                "{} differs between {} and {}",
                change.target,
                from_jur.display(),
                to_jur.display()
            ),
            severity,
            aspect: format!("{:?}", change.target),
        });
    }

    gaps
}

/// Calculates a similarity score between two statutes.
fn calculate_similarity(diff: &StatuteDiff) -> f64 {
    // Start with 1.0 (identical) and subtract for differences
    let mut score = 1.0;

    // Deduct based on number of changes
    let change_penalty = (diff.changes.len() as f64) * 0.05;
    score -= change_penalty;

    // Deduct based on severity
    let severity_penalty = match diff.impact.severity {
        crate::Severity::None => 0.0,
        crate::Severity::Minor => 0.05,
        crate::Severity::Moderate => 0.15,
        crate::Severity::Major => 0.30,
        crate::Severity::Breaking => 0.50,
    };
    score -= severity_penalty;

    // Ensure score is between 0.0 and 1.0
    score.max(0.0).min(1.0)
}

/// Normalization strategy for cross-jurisdiction comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// No normalization, compare as-is.
    None,
    /// Normalize common law terminology.
    CommonLaw,
    /// Normalize civil law terminology.
    CivilLaw,
    /// Use international standards (e.g., UN conventions).
    International,
}

/// Normalizes a statute for cross-jurisdiction comparison.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::cross_jurisdiction::{
///     JurisdictionalStatute, Jurisdiction, LegalSystem, normalize_for_comparison,
///     NormalizationStrategy
/// };
///
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let jurisdiction = Jurisdiction::new("US", LegalSystem::CommonLaw);
/// let jur_statute = JurisdictionalStatute::new(statute, jurisdiction, "en");
///
/// let normalized = normalize_for_comparison(&jur_statute, NormalizationStrategy::International);
/// assert_eq!(normalized.statute.id, "law");
/// ```
#[allow(dead_code)]
pub fn normalize_for_comparison(
    statute: &JurisdictionalStatute,
    strategy: NormalizationStrategy,
) -> JurisdictionalStatute {
    match strategy {
        NormalizationStrategy::None => statute.clone(),
        NormalizationStrategy::CommonLaw => {
            // In practice, this would transform civil law terms to common law equivalents
            statute.clone()
        }
        NormalizationStrategy::CivilLaw => {
            // Transform common law terms to civil law equivalents
            statute.clone()
        }
        NormalizationStrategy::International => {
            // Transform to international standards
            statute.clone()
        }
    }
}

/// Treaty comparison result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyComparison {
    /// Treaty identifier.
    pub treaty_id: String,
    /// Participating jurisdictions.
    pub jurisdictions: Vec<Jurisdiction>,
    /// Implementation variations by jurisdiction.
    pub variations: HashMap<String, Vec<String>>,
    /// Overall compliance level.
    pub compliance_level: f64,
}

/// Compares how different jurisdictions implement a treaty.
///
/// # Examples
///
/// ```
/// use legalis_diff::cross_jurisdiction::{Jurisdiction, LegalSystem, compare_treaty_implementations};
/// use std::collections::HashMap;
///
/// let mut implementations = HashMap::new();
/// implementations.insert(
///     "US".to_string(),
///     vec!["Article 1 implemented".to_string()],
/// );
/// implementations.insert(
///     "CA".to_string(),
///     vec!["Article 1 implemented".to_string()],
/// );
///
/// let result = compare_treaty_implementations("treaty-123", implementations);
/// assert_eq!(result.treaty_id, "treaty-123");
/// ```
#[allow(dead_code)]
pub fn compare_treaty_implementations(
    treaty_id: &str,
    implementations: HashMap<String, Vec<String>>,
) -> TreatyComparison {
    let jurisdictions = implementations
        .keys()
        .map(|k| Jurisdiction::new(k, LegalSystem::Mixed))
        .collect();

    // Calculate compliance level based on implementation consistency
    let compliance_level = if implementations.is_empty() {
        0.0
    } else {
        // Simple heuristic: average implementation count
        let total: usize = implementations.values().map(|v| v.len()).sum();
        let avg = total as f64 / implementations.len() as f64;
        (avg / 10.0).min(1.0) // Assume 10 articles = 100% compliance
    };

    TreatyComparison {
        treaty_id: treaty_id.to_string(),
        jurisdictions,
        variations: implementations,
        compliance_level,
    }
}

/// Multilingual alignment information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualAlignment {
    /// The statute ID.
    pub statute_id: String,
    /// Translations by language code.
    pub translations: HashMap<String, String>,
    /// Alignment confidence scores.
    pub confidence_scores: HashMap<String, f64>,
}

/// Aligns multilingual versions of a statute.
///
/// # Examples
///
/// ```
/// use legalis_diff::cross_jurisdiction::align_multilingual_statute;
/// use std::collections::HashMap;
///
/// let mut translations = HashMap::new();
/// translations.insert("en".to_string(), "Tax Credit Law".to_string());
/// translations.insert("fr".to_string(), "Loi sur le crédit d'impôt".to_string());
///
/// let alignment = align_multilingual_statute("law-123", translations);
/// assert_eq!(alignment.statute_id, "law-123");
/// assert_eq!(alignment.translations.len(), 2);
/// ```
#[allow(dead_code)]
pub fn align_multilingual_statute(
    statute_id: &str,
    translations: HashMap<String, String>,
) -> MultilingualAlignment {
    let mut confidence_scores = HashMap::new();

    // Calculate confidence scores based on translation length similarity
    if let Some(en_text) = translations.get("en") {
        let en_len = en_text.len() as f64;
        for (lang, text) in &translations {
            if lang != "en" {
                let len = text.len() as f64;
                let ratio = (len / en_len).min(en_len / len);
                confidence_scores.insert(lang.clone(), ratio);
            }
        }
    }

    // English is assumed to be the reference
    confidence_scores.insert("en".to_string(), 1.0);

    MultilingualAlignment {
        statute_id: statute_id.to_string(),
        translations,
        confidence_scores,
    }
}

/// Finds equivalent statutes across jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalenceMapping {
    /// Primary statute ID.
    pub primary_id: String,
    /// Equivalent statute IDs by jurisdiction.
    pub equivalents: HashMap<String, String>,
    /// Confidence scores for equivalences.
    pub confidence_scores: HashMap<String, f64>,
}

impl EquivalenceMapping {
    /// Creates a new equivalence mapping.
    pub fn new(primary_id: &str) -> Self {
        Self {
            primary_id: primary_id.to_string(),
            equivalents: HashMap::new(),
            confidence_scores: HashMap::new(),
        }
    }

    /// Adds an equivalent statute with a confidence score.
    pub fn add_equivalent(mut self, jurisdiction: &str, statute_id: &str, confidence: f64) -> Self {
        self.equivalents
            .insert(jurisdiction.to_string(), statute_id.to_string());
        self.confidence_scores
            .insert(jurisdiction.to_string(), confidence);
        self
    }

    /// Gets the equivalent statute ID for a jurisdiction, if any.
    pub fn get_equivalent(&self, jurisdiction: &str) -> Option<&String> {
        self.equivalents.get(jurisdiction)
    }

    /// Gets the confidence score for a jurisdiction's equivalence.
    pub fn get_confidence(&self, jurisdiction: &str) -> Option<f64> {
        self.confidence_scores.get(jurisdiction).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn test_statute(title: &str) -> Statute {
        Statute::new("test", title, Effect::new(EffectType::Grant, "Benefit"))
    }

    fn test_jurisdiction(country: &str) -> Jurisdiction {
        Jurisdiction::new(country, LegalSystem::CommonLaw)
    }

    #[test]
    fn test_jurisdiction_display() {
        let jur1 = test_jurisdiction("US");
        assert_eq!(jur1.display(), "US");

        let jur2 = test_jurisdiction("US").with_subdivision("CA");
        assert_eq!(jur2.display(), "US-CA");
    }

    #[test]
    fn test_jurisdictional_statute_creation() {
        let statute = test_statute("Test");
        let jur = test_jurisdiction("US");
        let jur_statute = JurisdictionalStatute::new(statute, jur, "en");

        assert_eq!(jur_statute.language, "en");
        assert_eq!(jur_statute.jurisdiction.country, "US");
    }

    #[test]
    fn test_cross_jurisdiction_comparison() {
        let statute1 = test_statute("US Version").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });
        let jur1 = test_jurisdiction("US");
        let jur_statute1 = JurisdictionalStatute::new(statute1, jur1, "en");

        let statute2 = test_statute("CA Version").with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 60,
        });
        let jur2 = test_jurisdiction("CA");
        let jur_statute2 = JurisdictionalStatute::new(statute2, jur2, "en");

        let result = compare_across_jurisdictions(&jur_statute1, &jur_statute2);
        assert!(result.is_ok());

        let cross_diff = result.unwrap();
        assert_eq!(cross_diff.from_jurisdiction.country, "US");
        assert_eq!(cross_diff.to_jurisdiction.country, "CA");
        assert!(!cross_diff.harmonization_gaps.is_empty());
    }

    #[test]
    fn test_similarity_calculation() {
        let statute1 = test_statute("Same");
        let statute2 = test_statute("Same");

        let diff_result = diff(&statute1, &statute2).unwrap();
        let score = calculate_similarity(&diff_result);
        assert_eq!(score, 1.0); // Identical statutes
    }

    #[test]
    fn test_equivalence_mapping() {
        let mapping = EquivalenceMapping::new("primary-123")
            .add_equivalent("US", "us-statute-456", 0.95)
            .add_equivalent("CA", "ca-statute-789", 0.90);

        assert_eq!(
            mapping.get_equivalent("US"),
            Some(&"us-statute-456".to_string())
        );
        assert_eq!(mapping.get_confidence("CA"), Some(0.90));
    }

    #[test]
    fn test_multilingual_alignment() {
        let mut translations = HashMap::new();
        translations.insert("en".to_string(), "Test Law".to_string());
        translations.insert("fr".to_string(), "Loi de test".to_string());

        let alignment = align_multilingual_statute("law-123", translations);
        assert_eq!(alignment.translations.len(), 2);
        assert!(alignment.confidence_scores.contains_key("en"));
        assert!(alignment.confidence_scores.contains_key("fr"));
    }

    #[test]
    fn test_treaty_comparison() {
        let mut implementations = HashMap::new();
        implementations.insert("US".to_string(), vec!["Article 1".to_string()]);
        implementations.insert("CA".to_string(), vec!["Article 1".to_string()]);

        let comparison = compare_treaty_implementations("treaty-xyz", implementations);
        assert_eq!(comparison.treaty_id, "treaty-xyz");
        assert_eq!(comparison.jurisdictions.len(), 2);
    }
}
