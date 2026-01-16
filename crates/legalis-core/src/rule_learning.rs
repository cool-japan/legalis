//! Rule Learning & Discovery
//!
//! This module provides machine learning capabilities for discovering legal rules,
//! including inductive logic programming, case-based reasoning, anomaly detection,
//! and statute clustering.
//!
//! # Features
//!
//! - **Inductive Logic Programming**: Learn new rules from examples
//! - **Case-Based Reasoning**: Derive conclusions from similar precedents
//! - **Anomaly Detection**: Identify unusual or inconsistent statute patterns
//! - **Statute Clustering**: Group similar statutes for analysis
//! - **Rule Synthesis**: Generate new rules from positive/negative examples
//!
//! # Examples
//!
//! ```
//! use legalis_core::*;
//! use legalis_core::rule_learning::*;
//!
//! // Learn rules from examples
//! let learner = InductiveLearner::new();
//! let positive_examples = vec![
//!     Statute::new("S1", "Example 1", Effect::grant("benefit"))
//!         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
//! ];
//! let learned_rule = learner.learn_from_examples(&positive_examples, &[]);
//! assert!(learned_rule.is_some());
//! ```

use crate::{ComparisonOp, Condition, Statute, case_law::Case};
use std::collections::{HashMap, HashSet};

/// Inductive logic programming learner for discovering legal rules
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::rule_learning::InductiveLearner;
///
/// let learner = InductiveLearner::new();
/// let examples = vec![
///     Statute::new("S1", "Senior Benefit", Effect::grant("pension"))
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
/// ];
/// let rule = learner.learn_from_examples(&examples, &[]);
/// assert!(rule.is_some());
/// ```
#[derive(Debug, Clone)]
pub struct InductiveLearner {
    /// Minimum coverage threshold (0.0-1.0)
    min_coverage: f64,

    /// Maximum rule complexity (number of conditions)
    max_complexity: usize,
}

impl InductiveLearner {
    /// Create a new inductive learner with default parameters
    pub fn new() -> Self {
        Self {
            min_coverage: 0.7,
            max_complexity: 5,
        }
    }

    /// Set minimum coverage threshold
    pub fn with_min_coverage(mut self, coverage: f64) -> Self {
        self.min_coverage = coverage.clamp(0.0, 1.0);
        self
    }

    /// Set maximum rule complexity
    pub fn with_max_complexity(mut self, complexity: usize) -> Self {
        self.max_complexity = complexity;
        self
    }

    /// Learn a new rule from positive and negative examples
    pub fn learn_from_examples(
        &self,
        positive: &[Statute],
        negative: &[Statute],
    ) -> Option<LearnedRule> {
        if positive.is_empty() {
            return None;
        }

        // Extract common patterns from positive examples
        let patterns = self.extract_patterns(positive);

        // Find the pattern that best separates positive from negative
        let best_pattern = self.find_best_pattern(&patterns, positive, negative)?;

        // Calculate coverage and confidence
        let coverage = self.calculate_coverage(&best_pattern, positive);
        let confidence = self.calculate_confidence(&best_pattern, positive, negative);

        Some(LearnedRule {
            conditions: best_pattern,
            coverage,
            confidence,
            positive_count: positive.len(),
            negative_count: negative.len(),
        })
    }

    /// Extract common patterns from statutes
    fn extract_patterns(&self, statutes: &[Statute]) -> Vec<Vec<Condition>> {
        let mut patterns = Vec::new();

        // Extract all conditions from all statutes
        for statute in statutes {
            if !statute.preconditions.is_empty() {
                patterns.push(statute.preconditions.clone());
            }
        }

        patterns
    }

    /// Find the pattern that best separates positive from negative examples
    fn find_best_pattern(
        &self,
        patterns: &[Vec<Condition>],
        positive: &[Statute],
        _negative: &[Statute],
    ) -> Option<Vec<Condition>> {
        if patterns.is_empty() {
            return None;
        }

        // For simplicity, return the first pattern that covers enough positives
        // In a real implementation, this would use more sophisticated algorithms
        for pattern in patterns {
            if pattern.len() <= self.max_complexity {
                let coverage = self.calculate_coverage(pattern, positive);
                if coverage >= self.min_coverage {
                    return Some(pattern.clone());
                }
            }
        }

        patterns.first().cloned()
    }

    /// Calculate coverage of a pattern over examples
    fn calculate_coverage(&self, pattern: &[Condition], examples: &[Statute]) -> f64 {
        if examples.is_empty() {
            return 0.0;
        }

        let covered = examples
            .iter()
            .filter(|statute| self.pattern_matches(&statute.preconditions, pattern))
            .count();

        covered as f64 / examples.len() as f64
    }

    /// Calculate confidence (precision) of a pattern
    fn calculate_confidence(
        &self,
        pattern: &[Condition],
        positive: &[Statute],
        negative: &[Statute],
    ) -> f64 {
        let true_positives = positive
            .iter()
            .filter(|statute| self.pattern_matches(&statute.preconditions, pattern))
            .count();

        let false_positives = negative
            .iter()
            .filter(|statute| self.pattern_matches(&statute.preconditions, pattern))
            .count();

        let total = true_positives + false_positives;
        if total == 0 {
            return 0.0;
        }

        true_positives as f64 / total as f64
    }

    /// Check if statute conditions match a pattern
    fn pattern_matches(&self, conditions: &[Condition], pattern: &[Condition]) -> bool {
        // Simple pattern matching - check if all pattern conditions are present
        pattern.iter().all(|p| conditions.contains(p))
    }
}

impl Default for InductiveLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// A rule learned from examples
#[derive(Debug, Clone)]
pub struct LearnedRule {
    /// The learned conditions
    pub conditions: Vec<Condition>,

    /// Coverage of positive examples (0.0-1.0)
    pub coverage: f64,

    /// Confidence/precision (0.0-1.0)
    pub confidence: f64,

    /// Number of positive examples
    pub positive_count: usize,

    /// Number of negative examples
    pub negative_count: usize,
}

impl LearnedRule {
    /// Check if this rule is high quality
    pub fn is_high_quality(&self) -> bool {
        self.coverage >= 0.7 && self.confidence >= 0.8
    }

    /// Get quality score (combines coverage and confidence)
    pub fn quality_score(&self) -> f64 {
        (self.coverage + self.confidence) / 2.0
    }
}

/// Case-based reasoner for deriving conclusions from similar precedents
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::case_law::*;
/// use legalis_core::rule_learning::CaseBasedReasoner;
///
/// let reasoner = CaseBasedReasoner::new().with_threshold(0.3); // Lower threshold
/// let case1 = Case::new("Case1", "Landmark v. State", 2020, Court::Supreme, "US")
///     .with_facts("Senior citizen denied benefits due to age")
///     .with_holding("Benefits must be granted");
///
/// let case2 = Case::new("Case2", "Similar v. State", 2021, Court::Supreme, "US")
///     .with_facts("Senior person denied benefits due to age")
///     .with_holding("Benefits must be granted");
///
/// let similar_cases = reasoner.find_similar_cases(&case2, &[case1.clone()], 3);
/// assert!(similar_cases.len() > 0);
/// ```
#[derive(Debug, Clone)]
pub struct CaseBasedReasoner {
    /// Similarity threshold (0.0-1.0)
    similarity_threshold: f64,
}

impl CaseBasedReasoner {
    /// Create a new case-based reasoner
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.5,
        }
    }

    /// Set similarity threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Find similar cases from a case base
    pub fn find_similar_cases(
        &self,
        query_case: &Case,
        case_base: &[Case],
        k: usize,
    ) -> Vec<SimilarCase> {
        let mut similarities: Vec<_> = case_base
            .iter()
            .map(|case| {
                let similarity = self.calculate_similarity(query_case, case);
                SimilarCase {
                    case: case.clone(),
                    similarity,
                }
            })
            .filter(|sc| sc.similarity >= self.similarity_threshold)
            .collect();

        // Sort by similarity (highest first)
        similarities.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // Return top k
        similarities.into_iter().take(k).collect()
    }

    /// Derive a conclusion from similar cases
    pub fn derive_conclusion(&self, similar_cases: &[SimilarCase]) -> Option<String> {
        if similar_cases.is_empty() {
            return None;
        }

        // Find the most common holding among similar cases
        let mut holding_counts: HashMap<String, usize> = HashMap::new();

        for similar in similar_cases {
            *holding_counts
                .entry(similar.case.holding.clone())
                .or_insert(0) += 1;
        }

        holding_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(holding, _)| holding)
    }

    /// Calculate similarity between two cases
    fn calculate_similarity(&self, case1: &Case, case2: &Case) -> f64 {
        let mut score = 0.0;
        let mut total = 0.0;

        // Court similarity (30% weight)
        if case1.court == case2.court {
            score += 0.3;
        }
        total += 0.3;

        // Facts similarity (40% weight)
        let facts_sim = self.text_similarity(&case1.facts, &case2.facts);
        score += facts_sim * 0.4;
        total += 0.4;

        // Holding similarity (30% weight)
        let holding_sim = self.text_similarity(&case1.holding, &case2.holding);
        score += holding_sim * 0.3;
        total += 0.3;

        if total > 0.0 { score / total } else { 0.0 }
    }

    /// Calculate text similarity using simple token overlap
    fn text_similarity(&self, text1: &str, text2: &str) -> f64 {
        let lower1 = text1.to_lowercase();
        let lower2 = text2.to_lowercase();

        let tokens1: HashSet<_> = lower1.split_whitespace().collect();
        let tokens2: HashSet<_> = lower2.split_whitespace().collect();

        if tokens1.is_empty() || tokens2.is_empty() {
            return 0.0;
        }

        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

impl Default for CaseBasedReasoner {
    fn default() -> Self {
        Self::new()
    }
}

/// A case with similarity score
#[derive(Debug, Clone)]
pub struct SimilarCase {
    /// The similar case
    pub case: Case,

    /// Similarity score (0.0-1.0)
    pub similarity: f64,
}

/// Anomaly detector for identifying unusual statute patterns
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::rule_learning::AnomalyDetector;
///
/// let detector = AnomalyDetector::new();
/// let statutes = vec![
///     Statute::new("S1", "Normal", Effect::grant("benefit"))
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
///     Statute::new("S2", "Unusual", Effect::revoke("benefit"))
///         .with_precondition(Condition::age(ComparisonOp::LessThan, 18)),
/// ];
///
/// let anomalies = detector.detect_anomalies(&statutes);
/// assert!(anomalies.len() <= statutes.len());
/// ```
#[derive(Debug, Clone)]
pub struct AnomalyDetector {
    /// Anomaly threshold (higher = stricter)
    threshold: f64,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new() -> Self {
        Self { threshold: 2.0 }
    }

    /// Set anomaly threshold (standard deviations from mean)
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.max(0.0);
        self
    }

    /// Detect anomalies in a set of statutes
    pub fn detect_anomalies(&self, statutes: &[Statute]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Detect unusual condition counts
        let condition_counts: Vec<_> = statutes.iter().map(|s| s.preconditions.len()).collect();
        let unusual_counts = self.find_outliers(&condition_counts);

        for (idx, statute) in statutes.iter().enumerate() {
            let mut reasons = Vec::new();

            // Check condition count
            if unusual_counts.contains(&idx) {
                reasons.push(format!(
                    "Unusual number of conditions: {} (most statutes have fewer)",
                    statute.preconditions.len()
                ));
            }

            // Check for rare effect types
            if self.is_rare_effect(statute, statutes) {
                reasons.push(format!(
                    "Rare effect type: {:?}",
                    statute.effect.effect_type
                ));
            }

            // Check for contradictory conditions
            if self.has_contradictory_conditions(statute) {
                reasons.push("Contains potentially contradictory conditions".to_string());
            }

            if !reasons.is_empty() {
                anomalies.push(Anomaly {
                    statute_id: statute.id.clone(),
                    anomaly_score: self.calculate_anomaly_score(&reasons),
                    reasons,
                });
            }
        }

        anomalies
    }

    /// Find outliers using standard deviation
    fn find_outliers(&self, values: &[usize]) -> HashSet<usize> {
        if values.len() < 3 {
            return HashSet::new();
        }

        let mean = values.iter().sum::<usize>() as f64 / values.len() as f64;
        let variance = values
            .iter()
            .map(|&v| {
                let diff = v as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / values.len() as f64;
        let std_dev = variance.sqrt();

        if std_dev == 0.0 {
            return HashSet::new();
        }

        values
            .iter()
            .enumerate()
            .filter(|&(_, &v)| {
                let z_score = ((v as f64 - mean) / std_dev).abs();
                z_score > self.threshold
            })
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Check if effect type is rare
    fn is_rare_effect(&self, statute: &Statute, all_statutes: &[Statute]) -> bool {
        let effect_count = all_statutes
            .iter()
            .filter(|s| s.effect.effect_type == statute.effect.effect_type)
            .count();

        let ratio = effect_count as f64 / all_statutes.len() as f64;
        ratio < 0.1 // Less than 10% is considered rare
    }

    /// Check for contradictory conditions
    fn has_contradictory_conditions(&self, statute: &Statute) -> bool {
        // Simple check: look for age conditions that might be contradictory
        let age_conditions: Vec<_> = statute
            .preconditions
            .iter()
            .filter_map(|c| match c {
                Condition::Age { operator, value } => Some((operator, value)),
                _ => None,
            })
            .collect();

        // Check if we have conflicting age requirements
        for i in 0..age_conditions.len() {
            for j in (i + 1)..age_conditions.len() {
                let (op1, val1) = age_conditions[i];
                let (op2, val2) = age_conditions[j];

                // Example: age >= 65 AND age < 60 is contradictory
                if matches!(op1, ComparisonOp::GreaterOrEqual)
                    && matches!(op2, ComparisonOp::LessThan)
                    && val1 > val2
                {
                    return true;
                }
            }
        }

        false
    }

    /// Calculate anomaly score
    fn calculate_anomaly_score(&self, reasons: &[String]) -> f64 {
        // Simple scoring: more reasons = higher score
        reasons.len() as f64 / 3.0 // Normalize to roughly 0-1 range
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// An detected anomaly in a statute
#[derive(Debug, Clone)]
pub struct Anomaly {
    /// ID of the anomalous statute
    pub statute_id: String,

    /// Anomaly score (higher = more anomalous)
    pub anomaly_score: f64,

    /// Reasons for flagging as anomaly
    pub reasons: Vec<String>,
}

/// Statute clusterer for grouping similar statutes
///
/// # Example
///
/// ```
/// use legalis_core::*;
/// use legalis_core::rule_learning::StatuteClusterer;
///
/// let clusterer = StatuteClusterer::new();
/// let statutes = vec![
///     Statute::new("S1", "Senior Benefit A", Effect::grant("pension"))
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
///     Statute::new("S2", "Senior Benefit B", Effect::grant("healthcare"))
///         .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
/// ];
///
/// let clusters = clusterer.cluster(&statutes, 2);
/// assert!(clusters.len() <= 2);
/// ```
#[derive(Debug, Clone)]
pub struct StatuteClusterer {
    /// Similarity threshold for clustering
    similarity_threshold: f64,
}

impl StatuteClusterer {
    /// Create a new clusterer
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.6,
        }
    }

    /// Set similarity threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Cluster statutes into k clusters
    pub fn cluster(&self, statutes: &[Statute], k: usize) -> Vec<StatuteCluster> {
        if statutes.is_empty() || k == 0 {
            return Vec::new();
        }

        // Simple clustering: group by similar characteristics
        let mut clusters = Vec::new();
        let mut assigned: HashSet<usize> = HashSet::new();

        for i in 0..statutes.len() {
            if assigned.contains(&i) {
                continue;
            }

            if clusters.len() >= k {
                break;
            }

            let mut cluster_statutes = vec![statutes[i].clone()];
            assigned.insert(i);

            // Find similar statutes
            for j in (i + 1)..statutes.len() {
                if assigned.contains(&j) {
                    continue;
                }

                if self.are_similar(&statutes[i], &statutes[j]) {
                    cluster_statutes.push(statutes[j].clone());
                    assigned.insert(j);
                }
            }

            let description = self.describe_cluster(&cluster_statutes);
            clusters.push(StatuteCluster {
                id: clusters.len(),
                statutes: cluster_statutes,
                centroid_description: description,
            });
        }

        // Add remaining statutes to nearest cluster or create new clusters
        for (idx, statute) in statutes.iter().enumerate() {
            if !assigned.contains(&idx) && clusters.len() < k {
                clusters.push(StatuteCluster {
                    id: clusters.len(),
                    statutes: vec![statute.clone()],
                    centroid_description: statute.title.clone(),
                });
            }
        }

        clusters
    }

    /// Check if two statutes are similar
    fn are_similar(&self, s1: &Statute, s2: &Statute) -> bool {
        let mut score = 0.0;
        let mut total = 0.0;

        // Same effect type
        if s1.effect.effect_type == s2.effect.effect_type {
            score += 1.0;
        }
        total += 1.0;

        // Similar number of conditions
        let cond_diff = (s1.preconditions.len() as i32 - s2.preconditions.len() as i32).abs();
        if cond_diff <= 1 {
            score += 1.0;
        }
        total += 1.0;

        // Same jurisdiction
        if s1.jurisdiction == s2.jurisdiction {
            score += 1.0;
        }
        total += 1.0;

        (score / total) >= self.similarity_threshold
    }

    /// Generate a description for a cluster
    fn describe_cluster(&self, statutes: &[Statute]) -> String {
        if statutes.is_empty() {
            return "Empty cluster".to_string();
        }

        let effect_type = &statutes[0].effect.effect_type;
        let avg_conditions = statutes
            .iter()
            .map(|s| s.preconditions.len())
            .sum::<usize>()
            / statutes.len();

        format!(
            "Cluster of {} statutes with {:?} effects, avg {} conditions",
            statutes.len(),
            effect_type,
            avg_conditions
        )
    }
}

impl Default for StatuteClusterer {
    fn default() -> Self {
        Self::new()
    }
}

/// A cluster of similar statutes
#[derive(Debug, Clone)]
pub struct StatuteCluster {
    /// Cluster ID
    pub id: usize,

    /// Statutes in this cluster
    pub statutes: Vec<Statute>,

    /// Description of cluster centroid
    pub centroid_description: String,
}

impl StatuteCluster {
    /// Get cluster size
    pub fn size(&self) -> usize {
        self.statutes.len()
    }

    /// Check if cluster is empty
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Effect;

    #[test]
    fn test_inductive_learner() {
        let learner = InductiveLearner::new();
        let positive = vec![
            Statute::new("S1", "Test", Effect::grant("benefit"))
                .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
        ];
        let rule = learner.learn_from_examples(&positive, &[]);
        assert!(rule.is_some());
        let rule = rule.unwrap();
        assert!(rule.coverage > 0.0);
    }

    #[test]
    fn test_case_based_reasoner() {
        let reasoner = CaseBasedReasoner::new();
        let case1 = Case::new(
            "C1",
            "Test v. State",
            2020,
            crate::case_law::Court::Supreme,
            "US",
        )
        .with_facts("Test facts");
        let similar = reasoner.find_similar_cases(&case1, std::slice::from_ref(&case1), 1);
        assert_eq!(similar.len(), 1);
    }

    #[test]
    fn test_anomaly_detector() {
        let detector = AnomalyDetector::new();
        let statutes = vec![
            Statute::new("S1", "Normal", Effect::grant("benefit"))
                .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 65)),
        ];
        let anomalies = detector.detect_anomalies(&statutes);
        // Single statute shouldn't be flagged as anomaly
        assert_eq!(anomalies.len(), 0);
    }

    #[test]
    fn test_statute_clusterer() {
        let clusterer = StatuteClusterer::new();
        let statutes = vec![
            Statute::new("S1", "Test 1", Effect::grant("benefit")),
            Statute::new("S2", "Test 2", Effect::grant("benefit")),
        ];
        let clusters = clusterer.cluster(&statutes, 1);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].size(), 2);
    }

    #[test]
    fn test_learned_rule_quality() {
        let rule = LearnedRule {
            conditions: vec![],
            coverage: 0.8,
            confidence: 0.9,
            positive_count: 10,
            negative_count: 2,
        };
        assert!(rule.is_high_quality());
        assert!(rule.quality_score() > 0.8);
    }
}
