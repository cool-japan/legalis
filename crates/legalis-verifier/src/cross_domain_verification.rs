//! Cross-Domain Verification Module
//!
//! This module provides multi-jurisdictional coherence checking, treaty compliance verification,
//! international law conflict detection, and cross-border regulation analysis.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::cross_domain_verification::*;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let config = CrossDomainConfig::default();
//! let verifier = CrossDomainVerifier::new(config);
//!
//! let statutes = vec![
//!     Statute::new("EU-GDPR-ART5", "GDPR Article 5", Effect::new(EffectType::Obligation, "Data protection")),
//!     Statute::new("US-CCPA-1798", "CCPA Section 1798", Effect::new(EffectType::Obligation, "Consumer privacy")),
//! ];
//!
//! let result = verifier.verify_cross_jurisdictional_coherence(&statutes);
//! ```

use crate::Statute;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration for cross-domain verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainConfig {
    /// Enable treaty compliance checking
    pub enable_treaty_compliance: bool,
    /// Enable international law conflict detection
    pub enable_international_conflicts: bool,
    /// Enable cross-border analysis
    pub enable_cross_border: bool,
    /// Minimum similarity threshold for cross-jurisdiction conflicts (0.0-1.0)
    pub similarity_threshold: f64,
    /// Enable automatic harmonization suggestions
    pub enable_harmonization_suggestions: bool,
    /// Maximum depth for jurisdictional hierarchy analysis
    pub max_hierarchy_depth: usize,
}

impl Default for CrossDomainConfig {
    fn default() -> Self {
        Self {
            enable_treaty_compliance: true,
            enable_international_conflicts: true,
            enable_cross_border: true,
            similarity_threshold: 0.7,
            enable_harmonization_suggestions: true,
            max_hierarchy_depth: 5,
        }
    }
}

/// Jurisdiction level in the legal hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JurisdictionLevel {
    /// International law (UN, treaties, etc.)
    International,
    /// Regional law (EU, ASEAN, etc.)
    Regional,
    /// National/federal law
    National,
    /// State/provincial law
    State,
    /// Local/municipal law
    Local,
}

impl std::fmt::Display for JurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JurisdictionLevel::International => write!(f, "International"),
            JurisdictionLevel::Regional => write!(f, "Regional"),
            JurisdictionLevel::National => write!(f, "National"),
            JurisdictionLevel::State => write!(f, "State"),
            JurisdictionLevel::Local => write!(f, "Local"),
        }
    }
}

/// Cross-jurisdictional conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossJurisdictionalConflict {
    /// Conflict ID
    pub id: String,
    /// Statute IDs involved
    pub statute_ids: Vec<String>,
    /// Jurisdictions involved
    pub jurisdictions: Vec<String>,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Severity (0-100)
    pub severity: u8,
    /// Description
    pub description: String,
    /// Harmonization suggestion
    pub harmonization_suggestion: Option<String>,
    /// Applicable treaties/conventions
    pub applicable_treaties: Vec<String>,
}

impl CrossJurisdictionalConflict {
    /// Create a new cross-jurisdictional conflict
    pub fn new(
        statute_ids: Vec<String>,
        jurisdictions: Vec<String>,
        conflict_type: ConflictType,
        severity: u8,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_ids,
            jurisdictions,
            conflict_type,
            severity,
            description: String::new(),
            harmonization_suggestion: None,
            applicable_treaties: Vec::new(),
        }
    }

    /// Add description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add harmonization suggestion
    pub fn with_harmonization(mut self, suggestion: String) -> Self {
        self.harmonization_suggestion = Some(suggestion);
        self
    }

    /// Add applicable treaty
    pub fn add_treaty(&mut self, treaty: String) {
        self.applicable_treaties.push(treaty);
    }
}

/// Type of cross-jurisdictional conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Direct contradiction between jurisdictions
    DirectContradiction,
    /// Extraterritorial overreach
    ExtraterritorialOverreach,
    /// Treaty violation
    TreatyViolation,
    /// Harmonization gap
    HarmonizationGap,
    /// Jurisdictional ambiguity
    JurisdictionalAmbiguity,
    /// Mutual recognition issue
    MutualRecognitionIssue,
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConflictType::DirectContradiction => write!(f, "Direct Contradiction"),
            ConflictType::ExtraterritorialOverreach => write!(f, "Extraterritorial Overreach"),
            ConflictType::TreatyViolation => write!(f, "Treaty Violation"),
            ConflictType::HarmonizationGap => write!(f, "Harmonization Gap"),
            ConflictType::JurisdictionalAmbiguity => write!(f, "Jurisdictional Ambiguity"),
            ConflictType::MutualRecognitionIssue => write!(f, "Mutual Recognition Issue"),
        }
    }
}

/// Treaty compliance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyComplianceResult {
    /// Treaty ID/name
    pub treaty_id: String,
    /// Statute ID being checked
    pub statute_id: String,
    /// Compliance status
    pub is_compliant: bool,
    /// Violations found
    pub violations: Vec<String>,
    /// Relevant treaty articles
    pub relevant_articles: Vec<String>,
    /// Compliance score (0-100)
    pub compliance_score: u8,
}

impl TreatyComplianceResult {
    /// Create a new treaty compliance result
    pub fn new(treaty_id: String, statute_id: String, is_compliant: bool) -> Self {
        Self {
            treaty_id,
            statute_id,
            is_compliant,
            violations: Vec::new(),
            relevant_articles: Vec::new(),
            compliance_score: if is_compliant { 100 } else { 0 },
        }
    }

    /// Add violation
    pub fn add_violation(&mut self, violation: String) {
        self.violations.push(violation);
        self.is_compliant = false;
        self.compliance_score = 100 - (self.violations.len() as u8 * 20);
    }

    /// Add relevant article
    pub fn add_article(&mut self, article: String) {
        self.relevant_articles.push(article);
    }
}

/// Cross-border regulation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBorderAnalysis {
    /// Analysis ID
    pub id: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Applicable statutes in source
    pub source_statutes: Vec<String>,
    /// Applicable statutes in target
    pub target_statutes: Vec<String>,
    /// Compatibility score (0-100)
    pub compatibility_score: u8,
    /// Key differences
    pub differences: Vec<String>,
    /// Harmonization opportunities
    pub harmonization_opportunities: Vec<String>,
}

impl CrossBorderAnalysis {
    /// Create a new cross-border analysis
    pub fn new(source: String, target: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_jurisdiction: source,
            target_jurisdiction: target,
            source_statutes: Vec::new(),
            target_statutes: Vec::new(),
            compatibility_score: 50,
            differences: Vec::new(),
            harmonization_opportunities: Vec::new(),
        }
    }

    /// Add source statute
    pub fn add_source_statute(&mut self, statute_id: String) {
        self.source_statutes.push(statute_id);
    }

    /// Add target statute
    pub fn add_target_statute(&mut self, statute_id: String) {
        self.target_statutes.push(statute_id);
    }

    /// Add difference
    pub fn add_difference(&mut self, difference: String) {
        self.differences.push(difference);
        self.compatibility_score = self.compatibility_score.saturating_sub(10);
    }

    /// Add harmonization opportunity
    pub fn add_harmonization_opportunity(&mut self, opportunity: String) {
        self.harmonization_opportunities.push(opportunity);
    }
}

/// Global legal consistency check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConsistencyCheck {
    /// Total statutes analyzed
    pub total_statutes: usize,
    /// Jurisdictions covered
    pub jurisdictions: HashSet<String>,
    /// Consistency score (0-100)
    pub consistency_score: u8,
    /// Detected conflicts
    pub conflicts: Vec<CrossJurisdictionalConflict>,
    /// Harmonization gaps
    pub harmonization_gaps: Vec<String>,
    /// Treaty compliance issues
    pub treaty_issues: Vec<String>,
}

impl Default for GlobalConsistencyCheck {
    fn default() -> Self {
        Self {
            total_statutes: 0,
            jurisdictions: HashSet::new(),
            consistency_score: 100,
            conflicts: Vec::new(),
            harmonization_gaps: Vec::new(),
            treaty_issues: Vec::new(),
        }
    }
}

impl GlobalConsistencyCheck {
    /// Add conflict
    pub fn add_conflict(&mut self, conflict: CrossJurisdictionalConflict) {
        self.conflicts.push(conflict);
        self.consistency_score = self.consistency_score.saturating_sub(5);
    }

    /// Add harmonization gap
    pub fn add_harmonization_gap(&mut self, gap: String) {
        self.harmonization_gaps.push(gap);
        self.consistency_score = self.consistency_score.saturating_sub(3);
    }

    /// Add treaty issue
    pub fn add_treaty_issue(&mut self, issue: String) {
        self.treaty_issues.push(issue);
        self.consistency_score = self.consistency_score.saturating_sub(7);
    }
}

/// Cross-domain verifier
pub struct CrossDomainVerifier {
    /// Configuration
    config: CrossDomainConfig,
    /// Known treaties and conventions
    treaties: HashMap<String, Vec<String>>,
}

impl CrossDomainVerifier {
    /// Create a new cross-domain verifier
    pub fn new(config: CrossDomainConfig) -> Self {
        let mut treaties = HashMap::new();

        // Initialize known international treaties
        treaties.insert(
            "GDPR".to_string(),
            vec![
                "Data Protection".to_string(),
                "Privacy Rights".to_string(),
                "Consent Management".to_string(),
            ],
        );
        treaties.insert(
            "TRIPS".to_string(),
            vec![
                "Intellectual Property".to_string(),
                "Patent Protection".to_string(),
                "Trademark Rights".to_string(),
            ],
        );
        treaties.insert(
            "Vienna Convention".to_string(),
            vec![
                "Treaty Interpretation".to_string(),
                "Good Faith".to_string(),
            ],
        );

        Self { config, treaties }
    }

    /// Verify cross-jurisdictional coherence
    pub fn verify_cross_jurisdictional_coherence(
        &self,
        statutes: &[Statute],
    ) -> GlobalConsistencyCheck {
        let mut result = GlobalConsistencyCheck {
            total_statutes: statutes.len(),
            ..Default::default()
        };

        // Collect all jurisdictions
        for statute in statutes {
            if let Some(jurisdiction) = &statute.jurisdiction
                && !jurisdiction.is_empty()
            {
                result.jurisdictions.insert(jurisdiction.clone());
            }
        }

        // Check for cross-jurisdictional conflicts
        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                let s1 = &statutes[i];
                let s2 = &statutes[j];

                if s1.jurisdiction != s2.jurisdiction
                    && let (Some(j1), Some(j2)) = (&s1.jurisdiction, &s2.jurisdiction)
                    && !j1.is_empty()
                    && !j2.is_empty()
                    && let Some(conflict) = self.detect_cross_jurisdictional_conflict(s1, s2)
                {
                    result.add_conflict(conflict);
                }
            }
        }

        // Check for harmonization gaps
        if self.config.enable_harmonization_suggestions {
            let gaps = self.detect_harmonization_gaps(statutes);
            for gap in gaps {
                result.add_harmonization_gap(gap);
            }
        }

        result
    }

    /// Detect cross-jurisdictional conflict
    fn detect_cross_jurisdictional_conflict(
        &self,
        s1: &Statute,
        s2: &Statute,
    ) -> Option<CrossJurisdictionalConflict> {
        // Check if statutes address similar topics but with different requirements
        let similarity = self.calculate_topic_similarity(s1, s2);

        if similarity >= self.config.similarity_threshold {
            // Check if effects are contradictory
            if self.effects_are_contradictory(&s1.effect, &s2.effect) {
                let severity = ((similarity * 100.0) as u8).min(95);
                let j1 = s1.jurisdiction.as_deref().unwrap_or("Unknown");
                let j2 = s2.jurisdiction.as_deref().unwrap_or("Unknown");

                let mut conflict = CrossJurisdictionalConflict::new(
                    vec![s1.id.clone(), s2.id.clone()],
                    vec![j1.to_string(), j2.to_string()],
                    ConflictType::DirectContradiction,
                    severity,
                )
                .with_description(format!(
                    "Contradictory requirements between {} ({}) and {} ({})",
                    s1.id, j1, s2.id, j2
                ));

                if self.config.enable_harmonization_suggestions {
                    conflict = conflict.with_harmonization(format!(
                        "Consider adopting international standard or mutual recognition agreement between {} and {}",
                        j1, j2
                    ));
                }

                return Some(conflict);
            }
        }

        None
    }

    /// Calculate topic similarity between statutes
    fn calculate_topic_similarity(&self, s1: &Statute, s2: &Statute) -> f64 {
        // Simple Jaccard similarity based on title words
        let words1: HashSet<String> = s1
            .title
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let words2: HashSet<String> = s2
            .title
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Check if effects are contradictory
    fn effects_are_contradictory(
        &self,
        effect1: &legalis_core::Effect,
        effect2: &legalis_core::Effect,
    ) -> bool {
        use legalis_core::EffectType;

        matches!(
            (&effect1.effect_type, &effect2.effect_type),
            (EffectType::Grant, EffectType::Revoke)
                | (EffectType::Revoke, EffectType::Grant)
                | (EffectType::Prohibition, EffectType::Obligation)
                | (EffectType::Obligation, EffectType::Prohibition)
        )
    }

    /// Detect harmonization gaps
    fn detect_harmonization_gaps(&self, statutes: &[Statute]) -> Vec<String> {
        let mut gaps = Vec::new();

        // Group statutes by jurisdiction
        let mut by_jurisdiction: HashMap<String, Vec<&Statute>> = HashMap::new();
        for statute in statutes {
            if let Some(jurisdiction) = &statute.jurisdiction {
                by_jurisdiction
                    .entry(jurisdiction.clone())
                    .or_default()
                    .push(statute);
            }
        }

        // Check for topics covered in some jurisdictions but not others
        let mut all_topics: HashMap<String, HashSet<String>> = HashMap::new();

        for (jurisdiction, stats) in &by_jurisdiction {
            for statute in stats {
                // Extract topic from title (simplified)
                let topic = statute.title.split_whitespace().next().unwrap_or("Unknown");
                all_topics
                    .entry(topic.to_string())
                    .or_default()
                    .insert(jurisdiction.clone());
            }
        }

        // Find topics not covered in all jurisdictions
        let total_jurisdictions = by_jurisdiction.len();
        for (topic, jurisdictions) in &all_topics {
            if jurisdictions.len() < total_jurisdictions && total_jurisdictions > 1 {
                let missing: Vec<_> = by_jurisdiction
                    .keys()
                    .filter(|j| !jurisdictions.contains(*j))
                    .cloned()
                    .collect();

                gaps.push(format!(
                    "Topic '{}' is regulated in {} but not in: {}",
                    topic,
                    jurisdictions.iter().next().unwrap_or(&String::new()),
                    missing.join(", ")
                ));
            }
        }

        gaps
    }

    /// Verify treaty compliance
    pub fn verify_treaty_compliance(
        &self,
        statute: &Statute,
        treaty_id: &str,
    ) -> TreatyComplianceResult {
        let mut result =
            TreatyComplianceResult::new(treaty_id.to_string(), statute.id.clone(), true);

        // Check if treaty is known
        if let Some(treaty_requirements) = self.treaties.get(treaty_id) {
            // Simple compliance check based on keywords
            let statute_text = format!("{} {}", statute.title, statute.effect.description);
            let statute_lower = statute_text.to_lowercase();

            for requirement in treaty_requirements {
                result.add_article(requirement.clone());

                // Check if statute addresses this requirement
                let req_lower = requirement.to_lowercase();
                let req_keywords: Vec<&str> = req_lower.split_whitespace().collect();
                let matches = req_keywords.iter().any(|kw| statute_lower.contains(kw));

                if !matches {
                    result.add_violation(format!("Does not explicitly address: {}", requirement));
                }
            }
        } else {
            result.add_violation(format!("Unknown treaty: {}", treaty_id));
        }

        result
    }

    /// Analyze cross-border regulation
    pub fn analyze_cross_border(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        statutes: &[Statute],
    ) -> CrossBorderAnalysis {
        let mut analysis = CrossBorderAnalysis::new(
            source_jurisdiction.to_string(),
            target_jurisdiction.to_string(),
        );

        // Categorize statutes
        for statute in statutes {
            if let Some(jurisdiction) = &statute.jurisdiction {
                if jurisdiction == source_jurisdiction {
                    analysis.add_source_statute(statute.id.clone());
                } else if jurisdiction == target_jurisdiction {
                    analysis.add_target_statute(statute.id.clone());
                }
            }
        }

        // Analyze differences
        if analysis.source_statutes.len() != analysis.target_statutes.len() {
            analysis.add_difference(format!(
                "Different number of applicable statutes: {} in source vs {} in target",
                analysis.source_statutes.len(),
                analysis.target_statutes.len()
            ));
        }

        // Check for harmonization opportunities
        if !analysis.differences.is_empty() {
            analysis.add_harmonization_opportunity(format!(
                "Establish mutual recognition agreement between {} and {}",
                source_jurisdiction, target_jurisdiction
            ));
        }

        analysis
    }

    /// Check international law conflicts
    pub fn check_international_law_conflicts(
        &self,
        statutes: &[Statute],
    ) -> Vec<CrossJurisdictionalConflict> {
        let mut conflicts = Vec::new();

        // Categorize statutes by jurisdiction level
        let mut international_statutes = Vec::new();
        let mut national_statutes = Vec::new();

        for statute in statutes {
            if let Some(jurisdiction) = &statute.jurisdiction {
                if jurisdiction.contains("UN") || jurisdiction.contains("International") {
                    international_statutes.push(statute);
                } else {
                    national_statutes.push(statute);
                }
            } else {
                national_statutes.push(statute);
            }
        }

        // Check for conflicts between international and national law
        for int_statute in &international_statutes {
            for nat_statute in &national_statutes {
                if self.effects_are_contradictory(&int_statute.effect, &nat_statute.effect) {
                    let int_jurisdiction = int_statute.jurisdiction.as_deref().unwrap_or("Unknown");
                    let nat_jurisdiction = nat_statute.jurisdiction.as_deref().unwrap_or("Unknown");

                    let conflict = CrossJurisdictionalConflict::new(
                        vec![int_statute.id.clone(), nat_statute.id.clone()],
                        vec![int_jurisdiction.to_string(), nat_jurisdiction.to_string()],
                        ConflictType::TreatyViolation,
                        85,
                    )
                    .with_description(format!(
                        "National law {} conflicts with international law {}",
                        nat_statute.id, int_statute.id
                    ))
                    .with_harmonization(format!(
                        "National law should be amended to comply with international obligations under {}",
                        int_jurisdiction
                    ));

                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// Generate cross-domain verification report
    pub fn generate_report(&self, check: &GlobalConsistencyCheck) -> String {
        let mut report = String::new();

        report.push_str("# Cross-Domain Verification Report\n\n");
        report.push_str("## Overview\n\n");
        report.push_str(&format!(
            "- **Total Statutes Analyzed**: {}\n",
            check.total_statutes
        ));
        report.push_str(&format!(
            "- **Jurisdictions Covered**: {}\n",
            check.jurisdictions.len()
        ));
        report.push_str(&format!(
            "- **Global Consistency Score**: {}%\n\n",
            check.consistency_score
        ));

        if !check.conflicts.is_empty() {
            report.push_str("## Cross-Jurisdictional Conflicts\n\n");
            for conflict in &check.conflicts {
                report.push_str(&format!(
                    "### {} (Severity: {})\n",
                    conflict.conflict_type, conflict.severity
                ));
                report.push_str(&format!(
                    "- **Jurisdictions**: {}\n",
                    conflict.jurisdictions.join(", ")
                ));
                report.push_str(&format!(
                    "- **Statutes**: {}\n",
                    conflict.statute_ids.join(", ")
                ));
                if !conflict.description.is_empty() {
                    report.push_str(&format!("- **Description**: {}\n", conflict.description));
                }
                if let Some(ref suggestion) = conflict.harmonization_suggestion {
                    report.push_str(&format!("- **Harmonization**: {}\n", suggestion));
                }
                report.push('\n');
            }
        }

        if !check.harmonization_gaps.is_empty() {
            report.push_str("## Harmonization Gaps\n\n");
            for gap in &check.harmonization_gaps {
                report.push_str(&format!("- {}\n", gap));
            }
            report.push('\n');
        }

        if !check.treaty_issues.is_empty() {
            report.push_str("## Treaty Compliance Issues\n\n");
            for issue in &check.treaty_issues {
                report.push_str(&format!("- {}\n", issue));
            }
            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_cross_domain_config_default() {
        let config = CrossDomainConfig::default();
        assert!(config.enable_treaty_compliance);
        assert!(config.enable_international_conflicts);
        assert!(config.enable_cross_border);
        assert_eq!(config.similarity_threshold, 0.7);
    }

    #[test]
    fn test_jurisdiction_level_ordering() {
        assert!(JurisdictionLevel::International < JurisdictionLevel::Regional);
        assert!(JurisdictionLevel::Regional < JurisdictionLevel::National);
        assert!(JurisdictionLevel::National < JurisdictionLevel::State);
        assert!(JurisdictionLevel::State < JurisdictionLevel::Local);
    }

    #[test]
    fn test_jurisdiction_level_display() {
        assert_eq!(
            JurisdictionLevel::International.to_string(),
            "International"
        );
        assert_eq!(JurisdictionLevel::Regional.to_string(), "Regional");
        assert_eq!(JurisdictionLevel::National.to_string(), "National");
    }

    #[test]
    fn test_cross_jurisdictional_conflict_creation() {
        let conflict = CrossJurisdictionalConflict::new(
            vec!["S1".to_string(), "S2".to_string()],
            vec!["US".to_string(), "EU".to_string()],
            ConflictType::DirectContradiction,
            80,
        );

        assert_eq!(conflict.statute_ids.len(), 2);
        assert_eq!(conflict.jurisdictions.len(), 2);
        assert_eq!(conflict.severity, 80);
    }

    #[test]
    fn test_conflict_type_display() {
        assert_eq!(
            ConflictType::DirectContradiction.to_string(),
            "Direct Contradiction"
        );
        assert_eq!(
            ConflictType::TreatyViolation.to_string(),
            "Treaty Violation"
        );
        assert_eq!(
            ConflictType::HarmonizationGap.to_string(),
            "Harmonization Gap"
        );
    }

    #[test]
    fn test_treaty_compliance_result_creation() {
        let result = TreatyComplianceResult::new("GDPR".to_string(), "DATA-2026".to_string(), true);

        assert_eq!(result.treaty_id, "GDPR");
        assert_eq!(result.statute_id, "DATA-2026");
        assert!(result.is_compliant);
        assert_eq!(result.compliance_score, 100);
    }

    #[test]
    fn test_treaty_compliance_add_violation() {
        let mut result =
            TreatyComplianceResult::new("GDPR".to_string(), "DATA-2026".to_string(), true);

        result.add_violation("Missing consent requirement".to_string());
        assert!(!result.is_compliant);
        assert_eq!(result.violations.len(), 1);
        assert_eq!(result.compliance_score, 80);
    }

    #[test]
    fn test_cross_border_analysis_creation() {
        let analysis = CrossBorderAnalysis::new("US".to_string(), "EU".to_string());
        assert_eq!(analysis.source_jurisdiction, "US");
        assert_eq!(analysis.target_jurisdiction, "EU");
        assert_eq!(analysis.compatibility_score, 50);
    }

    #[test]
    fn test_cross_border_analysis_add_difference() {
        let mut analysis = CrossBorderAnalysis::new("US".to_string(), "EU".to_string());
        analysis.add_difference("Different privacy standards".to_string());

        assert_eq!(analysis.differences.len(), 1);
        assert_eq!(analysis.compatibility_score, 40);
    }

    #[test]
    fn test_global_consistency_check_default() {
        let check = GlobalConsistencyCheck::default();
        assert_eq!(check.total_statutes, 0);
        assert_eq!(check.consistency_score, 100);
        assert!(check.conflicts.is_empty());
    }

    #[test]
    fn test_global_consistency_add_conflict() {
        let mut check = GlobalConsistencyCheck::default();
        let conflict = CrossJurisdictionalConflict::new(
            vec!["S1".to_string()],
            vec!["US".to_string()],
            ConflictType::DirectContradiction,
            70,
        );

        check.add_conflict(conflict);
        assert_eq!(check.conflicts.len(), 1);
        assert_eq!(check.consistency_score, 95);
    }

    #[test]
    fn test_cross_domain_verifier_creation() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);
        assert!(verifier.treaties.contains_key("GDPR"));
        assert!(verifier.treaties.contains_key("TRIPS"));
    }

    #[test]
    fn test_verify_cross_jurisdictional_coherence_empty() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statutes = vec![];
        let result = verifier.verify_cross_jurisdictional_coherence(&statutes);

        assert_eq!(result.total_statutes, 0);
        assert_eq!(result.jurisdictions.len(), 0);
        assert_eq!(result.consistency_score, 100);
    }

    #[test]
    fn test_verify_cross_jurisdictional_coherence_no_conflicts() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statutes = vec![
            Statute::new(
                "US-LAW-1",
                "Data Protection",
                Effect::new(EffectType::Obligation, "Protect data"),
            )
            .with_jurisdiction("US"),
            Statute::new(
                "EU-LAW-1",
                "Consumer Rights",
                Effect::new(EffectType::Grant, "Consumer protections"),
            )
            .with_jurisdiction("EU"),
        ];

        let result = verifier.verify_cross_jurisdictional_coherence(&statutes);
        assert_eq!(result.total_statutes, 2);
        assert_eq!(result.jurisdictions.len(), 2);
    }

    #[test]
    fn test_verify_treaty_compliance_known_treaty() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statute = Statute::new(
            "DATA-2026",
            "Data Protection Act",
            Effect::new(
                EffectType::Obligation,
                "Protect personal data and ensure privacy rights",
            ),
        );

        let result = verifier.verify_treaty_compliance(&statute, "GDPR");
        assert_eq!(result.treaty_id, "GDPR");
        assert!(!result.relevant_articles.is_empty());
    }

    #[test]
    fn test_verify_treaty_compliance_unknown_treaty() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statute = Statute::new(
            "TEST-1",
            "Test Law",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_treaty_compliance(&statute, "UNKNOWN_TREATY");
        assert!(!result.is_compliant);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_analyze_cross_border() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statutes = vec![
            Statute::new(
                "US-1",
                "Law A",
                Effect::new(EffectType::Obligation, "US requirement"),
            )
            .with_jurisdiction("US"),
            Statute::new(
                "EU-1",
                "Law B",
                Effect::new(EffectType::Obligation, "EU requirement"),
            )
            .with_jurisdiction("EU"),
        ];

        let analysis = verifier.analyze_cross_border("US", "EU", &statutes);
        assert_eq!(analysis.source_statutes.len(), 1);
        assert_eq!(analysis.target_statutes.len(), 1);
    }

    #[test]
    fn test_check_international_law_conflicts_no_conflicts() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let statutes = vec![
            Statute::new(
                "UN-1",
                "International Law",
                Effect::new(EffectType::Obligation, "International requirement"),
            )
            .with_jurisdiction("UN International"),
        ];

        let conflicts = verifier.check_international_law_conflicts(&statutes);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_generate_report() {
        let config = CrossDomainConfig::default();
        let verifier = CrossDomainVerifier::new(config);

        let mut check = GlobalConsistencyCheck {
            total_statutes: 10,
            ..Default::default()
        };
        check.jurisdictions.insert("US".to_string());
        check.jurisdictions.insert("EU".to_string());

        let report = verifier.generate_report(&check);
        assert!(report.contains("Cross-Domain Verification Report"));
        assert!(report.contains("**Total Statutes Analyzed**: 10"));
        assert!(report.contains("**Jurisdictions Covered**: 2"));
    }
}
