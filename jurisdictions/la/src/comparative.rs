//! Comparative Law Analysis Module
//!
//! This module provides tools for analyzing the Lao Civil Code in comparative context,
//! showing influences from Japanese and French legal systems through legal transplantation.
//!
//! ## Purpose
//! - Document legal transplants from Japanese and French civil law
//! - Analyze adaptations to Lao cultural and economic context
//! - Support comparative law research and ODA program evaluation
//!
//! ## Research Value
//! This module is particularly valuable for:
//! - 比較法学 (Comparative Law) research
//! - Evaluating effectiveness of Japanese ODA legal assistance
//! - Understanding legal modernization in developing countries
//! - Academic study of legal transplantation processes

use serde::{Deserialize, Serialize};

/// Comparative analysis of a Lao Civil Code provision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeAnalysis {
    pub lao_article: u32,
    pub lao_provision: String,
    pub japanese_influences: Vec<JapaneseInfluence>,
    pub french_influences: Vec<FrenchInfluence>,
    pub adaptations: Vec<String>,
    pub cultural_considerations: Vec<String>,
}

/// Japanese legal influence on Lao law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JapaneseInfluence {
    pub japanese_article: String,
    pub japanese_code: String, // e.g., "Civil Code", "Commercial Code"
    pub similarity_level: SimilarityLevel,
    pub description: String,
    pub oda_contribution: Option<String>,
}

/// French legal influence on Lao law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchInfluence {
    pub french_article: String,
    pub french_code: String, // e.g., "Code civil", "Code de commerce"
    pub similarity_level: SimilarityLevel,
    pub description: String,
    pub historical_context: String, // French colonial influence in Indochina
}

/// Level of similarity between legal provisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimilarityLevel {
    /// Nearly identical provision
    VeryHigh,
    /// Substantially similar with minor adaptations
    High,
    /// Similar concept with moderate adaptations
    Medium,
    /// Inspired by but significantly adapted
    Low,
    /// Conceptually related but different
    Minimal,
}

/// Type of legal transplant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalTransplant {
    /// Direct adoption of foreign legal text
    Direct,
    /// Adaptation with modifications
    Adapted,
    /// Synthesis of multiple sources
    Synthesized,
    /// Local innovation
    Indigenous,
}

impl ComparativeAnalysis {
    /// Create new comparative analysis for Lao Civil Code article
    pub fn new(lao_article: u32, lao_provision: String) -> Self {
        Self {
            lao_article,
            lao_provision,
            japanese_influences: vec![],
            french_influences: vec![],
            adaptations: vec![],
            cultural_considerations: vec![],
        }
    }

    /// Add Japanese influence
    pub fn add_japanese_influence(
        mut self,
        article: &str,
        code: &str,
        similarity: SimilarityLevel,
        description: &str,
    ) -> Self {
        self.japanese_influences.push(JapaneseInfluence {
            japanese_article: article.to_string(),
            japanese_code: code.to_string(),
            similarity_level: similarity,
            description: description.to_string(),
            oda_contribution: None,
        });
        self
    }

    /// Add French influence
    pub fn add_french_influence(
        mut self,
        article: &str,
        code: &str,
        similarity: SimilarityLevel,
        description: &str,
        historical_context: &str,
    ) -> Self {
        self.french_influences.push(FrenchInfluence {
            french_article: article.to_string(),
            french_code: code.to_string(),
            similarity_level: similarity,
            description: description.to_string(),
            historical_context: historical_context.to_string(),
        });
        self
    }

    /// Add adaptation note
    pub fn add_adaptation(mut self, adaptation: &str) -> Self {
        self.adaptations.push(adaptation.to_string());
        self
    }

    /// Add cultural consideration
    pub fn add_cultural_consideration(mut self, consideration: &str) -> Self {
        self.cultural_considerations.push(consideration.to_string());
        self
    }
}

/// Compare Lao provision with Japanese law
///
/// # Example
///
/// ```
/// use legalis_la::comparative::{compare_with_japanese_law, SimilarityLevel};
///
/// let analysis = compare_with_japanese_law(
///     20,
///     "A person who has attained the age of eighteen years has full legal capacity.",
/// );
///
/// assert_eq!(analysis.lao_article, 20);
/// assert!(!analysis.japanese_influences.is_empty());
/// ```
pub fn compare_with_japanese_law(lao_article: u32, lao_text: &str) -> ComparativeAnalysis {
    let mut analysis = ComparativeAnalysis::new(lao_article, lao_text.to_string());

    // Example comparisons for key articles
    match lao_article {
        1 => {
            analysis = analysis.add_japanese_influence(
                "Article 1",
                "Civil Code",
                SimilarityLevel::High,
                "Basic principle of rights protection parallels Japanese Article 1(1) \
                 on conformity with public welfare (公共の福祉)",
            );
        }
        3 => {
            analysis = analysis.add_japanese_influence(
                "Article 1(2)",
                "Civil Code",
                SimilarityLevel::VeryHigh,
                "Good faith principle directly based on Japanese 信義誠実の原則 \
                 (principle of good faith and sincerity)",
            );
        }
        20 => {
            analysis = analysis
                .add_japanese_influence(
                    "Article 4",
                    "Civil Code",
                    SimilarityLevel::VeryHigh,
                    "Age of majority at 18 follows Japanese 2022 reform \
                     (成年年齢の引下げ) lowering from 20 to 18",
                )
                .add_adaptation(
                    "Lao adopted age 18 directly, avoiding Japan's historical 20-year threshold",
                );
        }
        432 => {
            analysis = analysis.add_japanese_influence(
                "Articles 399-400",
                "Civil Code",
                SimilarityLevel::High,
                "General obligations structure follows Japanese 債権総則 (general provisions on obligations)",
            );
        }
        500 => {
            analysis = analysis.add_japanese_influence(
                "Article 522",
                "Civil Code",
                SimilarityLevel::VeryHigh,
                "Contract formation by offer and acceptance based on Japanese 2017 reform \
                 (契約法改正)",
            );
        }
        600 => {
            analysis = analysis.add_japanese_influence(
                "Article 709",
                "Civil Code",
                SimilarityLevel::High,
                "General tort liability principle parallels Japanese 不法行為 \
                 (tort) provisions requiring intent/negligence, causation, and damages",
            );
        }
        673 => {
            analysis = analysis
                .add_japanese_influence(
                    "Articles 731-739",
                    "Civil Code",
                    SimilarityLevel::Medium,
                    "Marriage requirements structure follows Japanese 婚姻 (marriage) provisions",
                )
                .add_adaptation(
                    "Marriage age (18 for both) differs from pre-2022 Japanese law (18 for men, 16 for women)",
                );
        }
        910 => {
            analysis = analysis.add_japanese_influence(
                "Articles 882-1044",
                "Civil Code",
                SimilarityLevel::High,
                "Succession structure based on Japanese 相続法 (inheritance law)",
            );
        }
        _ => {}
    }

    analysis
}

/// Compare Lao provision with French law
///
/// # Example
///
/// ```
/// use legalis_la::comparative::{compare_with_french_law, SimilarityLevel};
///
/// let analysis = compare_with_french_law(
///     600,
///     "A person who intentionally or negligently causes harm must compensate.",
/// );
///
/// assert_eq!(analysis.lao_article, 600);
/// assert!(!analysis.french_influences.is_empty());
/// ```
pub fn compare_with_french_law(lao_article: u32, lao_text: &str) -> ComparativeAnalysis {
    let mut analysis = ComparativeAnalysis::new(lao_article, lao_text.to_string());

    // Example comparisons for key articles
    match lao_article {
        1 => {
            analysis = analysis.add_french_influence(
                "Article 4",
                "Code civil",
                SimilarityLevel::Medium,
                "Rights protection principle has roots in French revolutionary legal tradition",
                "French colonial influence in Indochina (1893-1953) left lasting impact on legal concepts",
            );
        }
        3 => {
            analysis = analysis.add_french_influence(
                "Article 1104",
                "Code civil",
                SimilarityLevel::High,
                "Good faith principle (bonne foi) is fundamental in French contract law",
                "French legal tradition emphasizes good faith in all legal relations",
            );
        }
        162 => {
            analysis = analysis.add_french_influence(
                "Article 516",
                "Code civil",
                SimilarityLevel::High,
                "Classification of property as movable/immovable from French distinction \
                 between meubles and immeubles",
                "French property law classification system adopted throughout Indochina",
            );
        }
        500 => {
            analysis = analysis.add_french_influence(
                "Article 1103",
                "Code civil",
                SimilarityLevel::Medium,
                "Contract formation principles have French origins (2016 reform)",
                "French contract law modernization influenced Asian civil law reforms",
            );
        }
        600 => {
            analysis = analysis.add_french_influence(
                "Article 1240 (former 1382)",
                "Code civil",
                SimilarityLevel::VeryHigh,
                "General tort liability 'Tout fait quelconque' directly inspired Lao provision",
                "French tort law was basis for civil law throughout French Indochina",
            );
        }
        1000 => {
            analysis = analysis.add_french_influence(
                "Articles 912-917",
                "Code civil",
                SimilarityLevel::High,
                "Forced heirship (réserve héréditaire) system adopted from French inheritance law",
                "French forced heirship protected family interests in colonial era",
            );
        }
        _ => {}
    }

    analysis
}

/// Generate comprehensive comparative law report for Lao Civil Code article
pub fn generate_comparative_report(lao_article: u32, lao_text: &str) -> ComparativeAnalysis {
    let mut analysis = compare_with_japanese_law(lao_article, lao_text);
    let french = compare_with_french_law(lao_article, lao_text);

    // Merge French influences
    analysis.french_influences = french.french_influences;

    // Add general cultural considerations
    analysis = analysis
        .add_cultural_consideration(
            "Lao legal system balances socialist legal tradition with market economy needs",
        )
        .add_cultural_consideration(
            "Buddhist cultural values influence interpretation and application of civil law",
        )
        .add_cultural_consideration(
            "Rural-urban divide creates challenges for uniform law application",
        );

    analysis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparative_analysis_builder() {
        let analysis = ComparativeAnalysis::new(20, "Legal capacity at age 18".to_string())
            .add_japanese_influence(
                "Article 4",
                "Civil Code",
                SimilarityLevel::VeryHigh,
                "Same age threshold",
            )
            .add_adaptation("Direct adoption without historical 20-year threshold");

        assert_eq!(analysis.lao_article, 20);
        assert_eq!(analysis.japanese_influences.len(), 1);
        assert_eq!(analysis.adaptations.len(), 1);
    }

    #[test]
    fn test_compare_with_japanese_law() {
        let analysis = compare_with_japanese_law(3, "Good faith principle");
        assert_eq!(analysis.lao_article, 3);
        assert!(!analysis.japanese_influences.is_empty());
    }

    #[test]
    fn test_compare_with_french_law() {
        let analysis = compare_with_french_law(600, "Tort liability");
        assert_eq!(analysis.lao_article, 600);
        assert!(!analysis.french_influences.is_empty());
    }

    #[test]
    fn test_generate_comprehensive_report() {
        let report = generate_comparative_report(600, "Tort liability provision");
        assert_eq!(report.lao_article, 600);
        assert!(!report.japanese_influences.is_empty());
        assert!(!report.french_influences.is_empty());
        assert!(!report.cultural_considerations.is_empty());
    }
}
