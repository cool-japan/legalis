//! Louisiana State Law Module
//!
//! Louisiana is the ONLY US state following Civil Law tradition, making it unique
//! in American jurisprudence.
//!
//! ## Legal Heritage
//! - **French Civil Code** (Code Napoléon) influence
//! - **Spanish Law** (Las Siete Partidas) influence
//! - **Louisiana Civil Code** (enacted 1825, revised 1870, modernized ongoing)
//!
//! ## Key Differences from Common Law
//! - **Source of Law**: Codified statutes (like Japan, France, Germany)
//! - **Judicial Role**: Apply code, not create law through precedent
//! - **Terminology**: "Delict" not "tort", "Obligor/Obligee" not "Debtor/Creditor"
//! - **Forced Heirship**: Protects children's inheritance rights (limited)
//! - **Community Property**: Marital property regime from Civil Law
//!
//! ## Comparative Legal Analysis
//! Louisiana's Civil Law tradition enables direct comparison with:
//! - **Japan** (Minpo - Civil Code based on French/German models)
//! - **France** (Code civil - direct ancestor)
//! - **Germany** (BGB - influential on Louisiana reforms)

use crate::states::types::{StateId, StateLawVariation};
use legalis_core::{Condition, Effect, EffectType, Statute};

/// Louisiana state law module.
pub struct LouisianaLaw;

impl LouisianaLaw {
    /// Get Louisiana state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::louisiana()
    }

    // ===== Delictual Liability (Tort Law) =====

    /// Louisiana Civil Code Article 2315 - General Delictual Liability
    ///
    /// This is Louisiana's equivalent to:
    /// - Japan: Minpo Article 709 (民法709条)
    /// - France: Code civil Article 1240 (formerly 1382)
    /// - Germany: BGB § 823
    ///
    /// ## Article 2315 Text
    /// "Every act whatever of man that causes damage to another obliges him by whose
    /// fault it happened to repair it."
    ///
    /// ## Elements (Similar to Civil Law Systems)
    /// 1. **Act** (fait/act)
    /// 2. **Fault** (faute/culpa) - Intent OR negligence
    /// 3. **Damage** (dommage)
    /// 4. **Causation** (lien de causalité)
    ///
    /// ## Comparison with Civil Law Jurisdictions
    ///
    /// ### Similarity to Japanese Minpo 709 (0.75 similarity)
    /// - Both require fault (intent or negligence)
    /// - Both require causation and damage
    /// - Both are general provisions covering all delicts/torts
    /// - Difference: Louisiana allows punitive damages (US influence)
    ///
    /// ### Similarity to French Code civil 1240 (0.85 similarity)
    /// - Highest similarity - Louisiana based on French Civil Code
    /// - Nearly identical structure and elements
    /// - Same abstract formulation: "any act causing damage"
    /// - Difference: Louisiana's Common Law overlay from federal system
    ///
    /// ### Similarity to German BGB §823 (0.70 similarity)
    /// - Both Civil Law, but different approaches
    /// - German: Enumerated protected interests (Leben, Körper, Gesundheit, Freiheit...)
    /// - Louisiana: Abstract "any act causing damage" (French approach)
    #[must_use]
    pub fn article_2315_delict() -> Statute {
        Statute::new(
            "la-cc-2315",
            "Louisiana Civil Code Article 2315 - Delictual Liability",
            Effect::new(
                EffectType::Obligation,
                "Obligation to repair damage caused by fault",
            )
            .with_parameter("remedy", "damages_compensatory")
            .with_parameter("fault", "intent_or_negligence")
            .with_parameter("causation_required", "true"),
        )
        .with_jurisdiction("US-LA")
        .with_version(1)
        .with_precondition(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "intent".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "negligence".to_string(),
                value: "true".to_string(),
            }),
        ))
        .with_precondition(Condition::AttributeEquals {
            key: "damage".to_string(),
            value: "true".to_string(),
        })
        .with_precondition(Condition::AttributeEquals {
            key: "causation".to_string(),
            value: "true".to_string(),
        })
        .with_discretion(
            "Louisiana Civil Code Article 2315 follows Civil Law tradition from French \
             Code civil. Requires fault (faute), damage, and causation. Uses term 'delict' \
             rather than 'tort'. Louisiana courts apply codified law rather than developing \
             law through precedent (though prior decisions are persuasive). \
             \n\n【比較法】Comparative Law Analysis: \
             \n- Similarity to Japan Minpo 709: 0.75 (both require fault + causation + damage) \
             \n- Similarity to France Code civil 1240: 0.85 (highest - direct lineage) \
             \n- Similarity to Germany BGB §823: 0.70 (both Civil Law but different approaches) \
             \n\nUnique aspect: Louisiana allows punitive damages despite Civil Law tradition \
             (influence from surrounding Common Law states and federal system).",
        )
    }

    /// Comparative analysis with other Civil Law jurisdictions.
    ///
    /// This demonstrates legalis-RS's cross-jurisdiction comparison capabilities.
    ///
    /// ## Similarity Scores
    /// These scores are based on structural similarity, elements required, and
    /// doctrinal approach:
    ///
    /// - **France Code civil 1240**: 0.85 (highest - direct ancestor)
    ///   - Nearly identical text and structure
    ///   - Both use abstract "any act causing damage"
    ///   - Louisiana retains French terminology
    ///
    /// - **Japan Minpo 709**: 0.75 (high - both modern Civil Codes)
    ///   - Both require intent OR negligence (fault)
    ///   - Both require causation and damage
    ///   - Both are general provisions covering all torts/delicts
    ///   - Difference: Japan has no punitive damages
    ///
    /// - **Germany BGB §823**: 0.70 (good - both Civil Law)
    ///   - Both Civil Law systems with code-based approach
    ///   - Different structure: German enumerates protected interests
    ///   - German has unlawfulness requirement (Widerrechtlichkeit)
    ///   - Louisiana follows French abstract approach
    #[must_use]
    pub fn civil_law_similarities() -> Vec<(&'static str, f64, &'static str)> {
        vec![
            (
                "France (Code civil 1240)",
                0.85,
                "Highest similarity - Louisiana Civil Code directly based on French Code Napoléon",
            ),
            (
                "Japan (Minpo 709)",
                0.75,
                "Both modern Civil Codes requiring fault + causation + damage",
            ),
            (
                "Germany (BGB §823)",
                0.70,
                "Both Civil Law but different approaches (abstract vs enumerated interests)",
            ),
        ]
    }

    /// Differences from Common Law states.
    ///
    /// Despite being part of the United States, Louisiana maintains distinct Civil Law features.
    #[must_use]
    pub fn common_law_differences() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "Source of Law",
                "Louisiana: Civil Code (codified). Common Law: Cases/Precedent",
            ),
            (
                "Judicial Role",
                "Louisiana: Apply code. Common Law: Create law via precedent",
            ),
            (
                "Terminology",
                "Louisiana: Delict, Obligor/Obligee. Common Law: Tort, Debtor/Creditor",
            ),
            (
                "Property Regime",
                "Louisiana: Community property (Civil Law). Most Common Law: Separate property",
            ),
            (
                "Inheritance",
                "Louisiana: Forced heirship (limited). Common Law: Testamentary freedom",
            ),
            (
                "Contract Interpretation",
                "Louisiana: Civilian interpretation rules. Common Law: Parol evidence rule",
            ),
        ]
    }

    /// Louisiana's unique legal terminology.
    ///
    /// Louisiana uses Civil Law terminology distinct from Common Law states.
    #[must_use]
    pub fn terminology_mapping() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Delict", "Tort", "Civil wrong causing injury"),
            ("Obligor", "Debtor", "Person owing obligation"),
            ("Obligee", "Creditor", "Person to whom obligation is owed"),
            (
                "Usufruct",
                "Life estate",
                "Right to use property owned by another",
            ),
            (
                "Onerous contract",
                "Contract for consideration",
                "Contract with mutual obligations",
            ),
            (
                "Gratuitous contract",
                "Gift",
                "Contract without consideration",
            ),
            (
                "Putative father",
                "Alleged father",
                "Man presumed to be biological father",
            ),
        ]
    }

    /// Get Louisiana state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        // Note: Louisiana's tort law is fundamentally different (Civil Law vs Common Law)
        // Comparative negligence doesn't fit the same categories
        vec![]
    }

    /// Get Louisiana Civil Law statutes.
    #[must_use]
    pub fn civil_law_statutes() -> Vec<Statute> {
        vec![Self::article_2315_delict()]
    }
}

/// Comparison report structure.
#[derive(Debug, Clone)]
pub struct ComparisonReport {
    /// Similarity score (0.0 to 1.0)
    pub similarity: f64,

    /// Structural similarities
    pub similarities: Vec<String>,

    /// Key differences
    pub differences: Vec<String>,
}

impl ComparisonReport {
    /// Create new comparison report.
    #[must_use]
    pub fn new(similarity: f64) -> Self {
        Self {
            similarity,
            similarities: Vec::new(),
            differences: Vec::new(),
        }
    }

    /// Add similarity note.
    #[must_use]
    pub fn with_similarity(mut self, note: impl Into<String>) -> Self {
        self.similarities.push(note.into());
        self
    }

    /// Add difference note.
    #[must_use]
    pub fn with_difference(mut self, note: impl Into<String>) -> Self {
        self.differences.push(note.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::types::LegalTradition;

    #[test]
    fn test_louisiana_state_id() {
        let la = LouisianaLaw::state_id();
        assert_eq!(la.code, "LA");
        assert_eq!(la.name, "Louisiana");
        assert_eq!(la.legal_tradition, LegalTradition::CivilLaw);
    }

    #[test]
    fn test_article_2315_delict() {
        let statute = LouisianaLaw::article_2315_delict();

        assert_eq!(statute.id, "la-cc-2315");
        assert_eq!(statute.jurisdiction, Some("US-LA".to_string()));
        assert_eq!(statute.preconditions.len(), 3); // Fault OR negligence, damage, causation
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("Civil Law"));
        assert!(discretion.contains("France"));
        assert!(discretion.contains("Japan"));
    }

    #[test]
    fn test_civil_law_similarities() {
        let similarities = LouisianaLaw::civil_law_similarities();

        assert_eq!(similarities.len(), 3);

        // Verify France has highest similarity
        let france = similarities
            .iter()
            .find(|(name, _, _)| name.contains("France"))
            .unwrap();
        assert_eq!(france.1, 0.85);

        // Verify Japan is present
        let japan = similarities
            .iter()
            .find(|(name, _, _)| name.contains("Japan"))
            .unwrap();
        assert_eq!(japan.1, 0.75);

        // Verify Germany is present
        let germany = similarities
            .iter()
            .find(|(name, _, _)| name.contains("Germany"))
            .unwrap();
        assert_eq!(germany.1, 0.70);
    }

    #[test]
    fn test_common_law_differences() {
        let differences = LouisianaLaw::common_law_differences();

        assert!(!differences.is_empty());

        // Verify key differences are documented
        assert!(
            differences
                .iter()
                .any(|(category, _)| category.contains("Source of Law"))
        );
        assert!(
            differences
                .iter()
                .any(|(category, _)| category.contains("Terminology"))
        );
    }

    #[test]
    fn test_terminology_mapping() {
        let terms = LouisianaLaw::terminology_mapping();

        assert!(!terms.is_empty());

        // Verify "delict" vs "tort" mapping
        let delict = terms
            .iter()
            .find(|(la_term, _, _)| *la_term == "Delict")
            .unwrap();
        assert_eq!(delict.1, "Tort");

        // Verify "obligor" vs "debtor" mapping
        let obligor = terms
            .iter()
            .find(|(la_term, _, _)| *la_term == "Obligor")
            .unwrap();
        assert_eq!(obligor.1, "Debtor");
    }

    #[test]
    fn test_civil_law_statutes() {
        let statutes = LouisianaLaw::civil_law_statutes();

        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "la-cc-2315"));
    }

    #[test]
    fn test_comparison_report_builder() {
        let report = ComparisonReport::new(0.75)
            .with_similarity("Both require fault")
            .with_similarity("Both require causation")
            .with_difference("Louisiana allows punitive damages");

        assert_eq!(report.similarity, 0.75);
        assert_eq!(report.similarities.len(), 2);
        assert_eq!(report.differences.len(), 1);
    }

    #[test]
    fn test_louisiana_uniqueness() {
        // Louisiana is the only US state with Civil Law tradition
        let la = LouisianaLaw::state_id();

        assert_eq!(la.legal_tradition, LegalTradition::CivilLaw);

        // All Phase 1 Common Law states for comparison
        let ca = StateId::california();
        let ny = StateId::new_york();
        let tx = StateId::texas();
        let fl = StateId::florida();

        assert_eq!(ca.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(ny.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(tx.legal_tradition, LegalTradition::CommonLaw);
        assert_eq!(fl.legal_tradition, LegalTradition::CommonLaw);
    }
}
