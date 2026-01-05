//! Pattern recognition and best practices for statute amendments.
//!
//! This module provides:
//! - Common amendment pattern library
//! - Pattern-based change suggestions
//! - Anti-pattern detection
//! - Best practice recommendations
//! - Historical pattern learning

use crate::{ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Amendment pattern with detailed metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentPatternInfo {
    /// Pattern identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Common triggers for this pattern
    pub triggers: Vec<String>,
    /// Expected outcomes
    pub expected_outcomes: Vec<String>,
    /// Potential risks
    pub risks: Vec<String>,
    /// Recommended actions
    pub recommendations: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

/// Anti-pattern that should be avoided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiPattern {
    /// Anti-pattern identifier
    pub id: String,
    /// Name of the anti-pattern
    pub name: String,
    /// Description of why this is problematic
    pub description: String,
    /// Severity of the anti-pattern
    pub severity: AntiPatternSeverity,
    /// How to fix it
    pub remediation: String,
    /// Examples of this anti-pattern
    pub examples: Vec<String>,
}

/// Severity levels for anti-patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AntiPatternSeverity {
    /// Informational - not critical but worth noting
    Info,
    /// Warning - should be reviewed
    Warning,
    /// Error - should be fixed before proceeding
    Error,
    /// Critical - must be fixed immediately
    Critical,
}

/// Best practice recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPractice {
    /// Practice identifier
    pub id: String,
    /// Category of the practice
    pub category: String,
    /// Title of the best practice
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Why this is important
    pub rationale: String,
    /// How to implement
    pub implementation: String,
    /// Related patterns
    pub related_patterns: Vec<String>,
}

/// Pattern detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDetectionResult {
    /// Detected patterns
    pub patterns: Vec<AmendmentPatternInfo>,
    /// Detected anti-patterns
    pub anti_patterns: Vec<AntiPattern>,
    /// Applicable best practices
    pub best_practices: Vec<BestPractice>,
    /// Overall pattern score (0.0 to 1.0)
    pub pattern_score: f64,
}

/// Pattern library containing known patterns and anti-patterns.
pub struct PatternLibrary {
    patterns: Vec<AmendmentPatternInfo>,
    anti_patterns: Vec<AntiPattern>,
    best_practices: Vec<BestPractice>,
}

impl PatternLibrary {
    /// Creates a new pattern library with common patterns.
    pub fn new() -> Self {
        Self {
            patterns: Self::init_patterns(),
            anti_patterns: Self::init_anti_patterns(),
            best_practices: Self::init_best_practices(),
        }
    }

    /// Initializes common amendment patterns.
    fn init_patterns() -> Vec<AmendmentPatternInfo> {
        vec![
            AmendmentPatternInfo {
                id: "age-threshold-adjustment".to_string(),
                name: "Age Threshold Adjustment".to_string(),
                description: "Modification of age-based eligibility criteria".to_string(),
                triggers: vec![
                    "Demographic changes".to_string(),
                    "Life expectancy updates".to_string(),
                    "Policy alignment".to_string(),
                ],
                expected_outcomes: vec![
                    "Changed eligibility pool".to_string(),
                    "Budget impact".to_string(),
                ],
                risks: vec![
                    "Sudden eligibility changes for borderline cases".to_string(),
                    "Public backlash if restrictive".to_string(),
                ],
                recommendations: vec![
                    "Consider gradual phase-in".to_string(),
                    "Analyze demographic impact".to_string(),
                    "Provide transition period".to_string(),
                ],
                confidence: 0.0,
            },
            AmendmentPatternInfo {
                id: "income-limit-adjustment".to_string(),
                name: "Income Limit Adjustment".to_string(),
                description: "Modification of income-based thresholds".to_string(),
                triggers: vec![
                    "Inflation adjustment".to_string(),
                    "Cost of living changes".to_string(),
                    "Policy priorities shift".to_string(),
                ],
                expected_outcomes: vec![
                    "Eligibility scope change".to_string(),
                    "Fiscal impact".to_string(),
                ],
                risks: vec![
                    "Benefit cliff effects".to_string(),
                    "Unintended exclusions".to_string(),
                ],
                recommendations: vec![
                    "Index to inflation".to_string(),
                    "Model impact scenarios".to_string(),
                    "Consider phase-out ranges".to_string(),
                ],
                confidence: 0.0,
            },
            AmendmentPatternInfo {
                id: "eligibility-expansion".to_string(),
                name: "Eligibility Expansion".to_string(),
                description: "Broadening of eligibility criteria".to_string(),
                triggers: vec![
                    "Policy liberalization".to_string(),
                    "Increased funding".to_string(),
                    "Public demand".to_string(),
                ],
                expected_outcomes: vec![
                    "Increased beneficiary count".to_string(),
                    "Higher program costs".to_string(),
                    "Improved access".to_string(),
                ],
                risks: vec![
                    "Budget overruns".to_string(),
                    "Administrative burden".to_string(),
                ],
                recommendations: vec![
                    "Estimate fiscal impact".to_string(),
                    "Ensure adequate funding".to_string(),
                    "Plan for increased volume".to_string(),
                ],
                confidence: 0.0,
            },
            AmendmentPatternInfo {
                id: "eligibility-restriction".to_string(),
                name: "Eligibility Restriction".to_string(),
                description: "Narrowing of eligibility criteria".to_string(),
                triggers: vec![
                    "Budget constraints".to_string(),
                    "Program targeting".to_string(),
                    "Fraud prevention".to_string(),
                ],
                expected_outcomes: vec![
                    "Reduced beneficiary count".to_string(),
                    "Cost savings".to_string(),
                    "Potential hardship".to_string(),
                ],
                risks: vec![
                    "Unintended exclusions".to_string(),
                    "Political backlash".to_string(),
                    "Equity concerns".to_string(),
                ],
                recommendations: vec![
                    "Impact analysis on vulnerable groups".to_string(),
                    "Provide grandfather clauses".to_string(),
                    "Consider transition support".to_string(),
                ],
                confidence: 0.0,
            },
            AmendmentPatternInfo {
                id: "discretion-introduction".to_string(),
                name: "Discretion Introduction".to_string(),
                description: "Adding human judgment requirements".to_string(),
                triggers: vec![
                    "Complex case handling".to_string(),
                    "Flexibility needs".to_string(),
                    "Exception handling".to_string(),
                ],
                expected_outcomes: vec![
                    "Case-by-case evaluation".to_string(),
                    "Slower processing".to_string(),
                    "Varied outcomes".to_string(),
                ],
                risks: vec![
                    "Inconsistent application".to_string(),
                    "Potential bias".to_string(),
                    "Reduced transparency".to_string(),
                ],
                recommendations: vec![
                    "Provide clear guidelines".to_string(),
                    "Implement oversight mechanisms".to_string(),
                    "Document decision criteria".to_string(),
                ],
                confidence: 0.0,
            },
            AmendmentPatternInfo {
                id: "discretion-removal".to_string(),
                name: "Discretion Removal".to_string(),
                description: "Making deterministic rules".to_string(),
                triggers: vec![
                    "Consistency improvement".to_string(),
                    "Automation enablement".to_string(),
                    "Bias reduction".to_string(),
                ],
                expected_outcomes: vec![
                    "Faster processing".to_string(),
                    "Consistent outcomes".to_string(),
                    "Easier compliance".to_string(),
                ],
                risks: vec![
                    "Loss of flexibility".to_string(),
                    "Edge case issues".to_string(),
                ],
                recommendations: vec![
                    "Ensure rules cover common cases".to_string(),
                    "Provide exception mechanism".to_string(),
                    "Monitor for unforeseen issues".to_string(),
                ],
                confidence: 0.0,
            },
        ]
    }

    /// Initializes anti-patterns to avoid.
    fn init_anti_patterns() -> Vec<AntiPattern> {
        vec![
            AntiPattern {
                id: "excessive-changes".to_string(),
                name: "Excessive Simultaneous Changes".to_string(),
                description: "Too many changes in a single amendment, making review difficult"
                    .to_string(),
                severity: AntiPatternSeverity::Warning,
                remediation: "Split into multiple focused amendments".to_string(),
                examples: vec![
                    "Changing 10+ conditions at once".to_string(),
                    "Modifying multiple unrelated aspects".to_string(),
                ],
            },
            AntiPattern {
                id: "breaking-without-transition".to_string(),
                name: "Breaking Change Without Transition".to_string(),
                description: "Major changes without grandfather clauses or transition periods"
                    .to_string(),
                severity: AntiPatternSeverity::Error,
                remediation: "Add transition period or grandfather existing beneficiaries"
                    .to_string(),
                examples: vec![
                    "Sudden age limit increase".to_string(),
                    "Immediate income threshold reduction".to_string(),
                ],
            },
            AntiPattern {
                id: "vague-discretion".to_string(),
                name: "Vague Discretionary Criteria".to_string(),
                description: "Adding discretion without clear guidelines".to_string(),
                severity: AntiPatternSeverity::Warning,
                remediation: "Provide detailed guidelines and decision criteria".to_string(),
                examples: vec![
                    "May grant if deemed appropriate".to_string(),
                    "At administrator's discretion".to_string(),
                ],
            },
            AntiPattern {
                id: "retroactive-restriction".to_string(),
                name: "Retroactive Restriction".to_string(),
                description: "Applying restrictive changes retroactively".to_string(),
                severity: AntiPatternSeverity::Critical,
                remediation: "Apply restrictions only to new cases, not existing ones".to_string(),
                examples: vec![
                    "Removing benefits from current recipients".to_string(),
                    "Retroactive eligibility tightening".to_string(),
                ],
            },
            AntiPattern {
                id: "benefit-cliff".to_string(),
                name: "Benefit Cliff Effect".to_string(),
                description: "Sharp benefit cutoffs creating perverse incentives".to_string(),
                severity: AntiPatternSeverity::Warning,
                remediation: "Use gradual phase-outs instead of hard cutoffs".to_string(),
                examples: vec![
                    "100% benefit loss at $1 over threshold".to_string(),
                    "All-or-nothing eligibility".to_string(),
                ],
            },
            AntiPattern {
                id: "ambiguous-wording".to_string(),
                name: "Ambiguous Legal Wording".to_string(),
                description: "Changes that introduce unclear or contradictory language".to_string(),
                severity: AntiPatternSeverity::Error,
                remediation: "Use precise, unambiguous language; resolve contradictions"
                    .to_string(),
                examples: vec![
                    "May or may not apply".to_string(),
                    "Reasonable interpretation varies".to_string(),
                ],
            },
        ]
    }

    /// Initializes best practices.
    fn init_best_practices() -> Vec<BestPractice> {
        vec![
            BestPractice {
                id: "gradual-transition".to_string(),
                category: "Change Management".to_string(),
                title: "Implement Gradual Transitions".to_string(),
                description: "Phase in changes over time rather than abruptly".to_string(),
                rationale: "Reduces shock, allows adaptation, improves public acceptance"
                    .to_string(),
                implementation: "Use effective dates, phase-in schedules, or grandfather clauses"
                    .to_string(),
                related_patterns: vec![
                    "age-threshold-adjustment".to_string(),
                    "eligibility-restriction".to_string(),
                ],
            },
            BestPractice {
                id: "inflation-indexing".to_string(),
                category: "Economic Stability".to_string(),
                title: "Index Thresholds to Inflation".to_string(),
                description: "Automatically adjust monetary thresholds for inflation".to_string(),
                rationale: "Maintains real value, reduces need for frequent amendments".to_string(),
                implementation: "Link thresholds to CPI or other inflation indices".to_string(),
                related_patterns: vec!["income-limit-adjustment".to_string()],
            },
            BestPractice {
                id: "impact-analysis".to_string(),
                category: "Due Diligence".to_string(),
                title: "Conduct Comprehensive Impact Analysis".to_string(),
                description: "Analyze effects on all stakeholder groups before implementation"
                    .to_string(),
                rationale: "Identifies unintended consequences, improves decision quality"
                    .to_string(),
                implementation:
                    "Model scenarios, consult stakeholders, analyze distributional effects"
                        .to_string(),
                related_patterns: vec![
                    "eligibility-expansion".to_string(),
                    "eligibility-restriction".to_string(),
                ],
            },
            BestPractice {
                id: "clear-documentation".to_string(),
                category: "Transparency".to_string(),
                title: "Maintain Clear Documentation".to_string(),
                description: "Document rationale, expected effects, and implementation details"
                    .to_string(),
                rationale: "Improves understanding, facilitates review, aids future amendments"
                    .to_string(),
                implementation: "Include explanatory notes, impact statements, and change history"
                    .to_string(),
                related_patterns: vec![],
            },
            BestPractice {
                id: "stakeholder-consultation".to_string(),
                category: "Governance".to_string(),
                title: "Engage Stakeholder Consultation".to_string(),
                description: "Involve affected parties in the amendment process".to_string(),
                rationale: "Improves legitimacy, identifies issues, builds support".to_string(),
                implementation: "Hold public hearings, solicit feedback, consider input"
                    .to_string(),
                related_patterns: vec![
                    "eligibility-expansion".to_string(),
                    "eligibility-restriction".to_string(),
                ],
            },
            BestPractice {
                id: "monitoring-mechanism".to_string(),
                category: "Quality Assurance".to_string(),
                title: "Establish Monitoring Mechanisms".to_string(),
                description: "Track implementation and outcomes after amendment".to_string(),
                rationale:
                    "Enables course correction, validates assumptions, informs future changes"
                        .to_string(),
                implementation: "Set metrics, collect data, review periodically".to_string(),
                related_patterns: vec![],
            },
        ]
    }

    /// Detects patterns in a statute diff.
    pub fn detect_patterns(&self, diff: &StatuteDiff) -> PatternDetectionResult {
        let mut detected_patterns = Vec::new();
        let mut detected_anti_patterns = Vec::new();
        let mut applicable_practices = Vec::new();

        // Detect amendment patterns
        for pattern in &self.patterns {
            if let Some(detected) = self.match_pattern(pattern, diff) {
                detected_patterns.push(detected);
            }
        }

        // Detect anti-patterns
        for anti_pattern in &self.anti_patterns {
            if self.matches_anti_pattern(anti_pattern, diff) {
                detected_anti_patterns.push(anti_pattern.clone());
            }
        }

        // Find applicable best practices
        for practice in &self.best_practices {
            if self.is_practice_applicable(practice, &detected_patterns) {
                applicable_practices.push(practice.clone());
            }
        }

        // Calculate overall pattern score
        let pattern_score =
            self.calculate_pattern_score(&detected_patterns, &detected_anti_patterns);

        PatternDetectionResult {
            patterns: detected_patterns,
            anti_patterns: detected_anti_patterns,
            best_practices: applicable_practices,
            pattern_score,
        }
    }

    /// Matches a pattern against a diff.
    fn match_pattern(
        &self,
        pattern: &AmendmentPatternInfo,
        diff: &StatuteDiff,
    ) -> Option<AmendmentPatternInfo> {
        let confidence = match pattern.id.as_str() {
            "age-threshold-adjustment" => self.detect_age_adjustment(diff),
            "income-limit-adjustment" => self.detect_income_adjustment(diff),
            "eligibility-expansion" => self.detect_eligibility_expansion(diff),
            "eligibility-restriction" => self.detect_eligibility_restriction(diff),
            "discretion-introduction" => self.detect_discretion_introduction(diff),
            "discretion-removal" => self.detect_discretion_removal(diff),
            _ => 0.0,
        };

        if confidence > 0.5 {
            let mut matched = pattern.clone();
            matched.confidence = confidence;
            Some(matched)
        } else {
            None
        }
    }

    /// Detects age threshold adjustments.
    fn detect_age_adjustment(&self, diff: &StatuteDiff) -> f64 {
        let age_changes = diff
            .changes
            .iter()
            .filter(|c| {
                matches!(c.change_type, ChangeType::Modified)
                    && c.old_value.as_ref().is_some_and(|v| v.contains("Age"))
                    && c.new_value.as_ref().is_some_and(|v| v.contains("Age"))
            })
            .count();

        if age_changes > 0 { 0.9 } else { 0.0 }
    }

    /// Detects income limit adjustments.
    fn detect_income_adjustment(&self, diff: &StatuteDiff) -> f64 {
        let income_changes = diff
            .changes
            .iter()
            .filter(|c| {
                matches!(c.change_type, ChangeType::Modified)
                    && c.old_value.as_ref().is_some_and(|v| v.contains("Income"))
                    && c.new_value.as_ref().is_some_and(|v| v.contains("Income"))
            })
            .count();

        if income_changes > 0 { 0.9 } else { 0.0 }
    }

    /// Detects eligibility expansion.
    fn detect_eligibility_expansion(&self, diff: &StatuteDiff) -> f64 {
        if !diff.impact.affects_eligibility {
            return 0.0;
        }

        let removed_preconditions = diff
            .changes
            .iter()
            .filter(|c| {
                matches!(c.change_type, ChangeType::Removed)
                    && matches!(c.target, crate::ChangeTarget::Precondition { .. })
            })
            .count();

        if removed_preconditions > 0 { 0.85 } else { 0.0 }
    }

    /// Detects eligibility restriction.
    fn detect_eligibility_restriction(&self, diff: &StatuteDiff) -> f64 {
        if !diff.impact.affects_eligibility {
            return 0.0;
        }

        let added_preconditions = diff
            .changes
            .iter()
            .filter(|c| {
                matches!(c.change_type, ChangeType::Added)
                    && matches!(c.target, crate::ChangeTarget::Precondition { .. })
            })
            .count();

        if added_preconditions > 0 { 0.85 } else { 0.0 }
    }

    /// Detects discretion introduction.
    fn detect_discretion_introduction(&self, diff: &StatuteDiff) -> f64 {
        if diff.impact.discretion_changed {
            let has_discretion_added = diff.changes.iter().any(|c| {
                matches!(c.change_type, ChangeType::Added)
                    && matches!(c.target, crate::ChangeTarget::DiscretionLogic)
            });

            if has_discretion_added { 0.95 } else { 0.0 }
        } else {
            0.0
        }
    }

    /// Detects discretion removal.
    fn detect_discretion_removal(&self, diff: &StatuteDiff) -> f64 {
        if diff.impact.discretion_changed {
            let has_discretion_removed = diff.changes.iter().any(|c| {
                matches!(c.change_type, ChangeType::Removed)
                    && matches!(c.target, crate::ChangeTarget::DiscretionLogic)
            });

            if has_discretion_removed { 0.95 } else { 0.0 }
        } else {
            0.0
        }
    }

    /// Checks if an anti-pattern matches.
    fn matches_anti_pattern(&self, anti_pattern: &AntiPattern, diff: &StatuteDiff) -> bool {
        match anti_pattern.id.as_str() {
            "excessive-changes" => diff.changes.len() > 10,
            "breaking-without-transition" => {
                diff.impact.severity >= Severity::Breaking && !diff.changes.is_empty()
            }
            "vague-discretion" => {
                diff.impact.discretion_changed
                    && diff.changes.iter().any(|c| {
                        matches!(c.target, crate::ChangeTarget::DiscretionLogic)
                            && c.new_value.as_ref().is_some_and(|v| v.len() < 50)
                    })
            }
            "retroactive-restriction" => {
                // This would require more context; for now, assume false
                false
            }
            "benefit-cliff" => {
                // This would require semantic analysis; for now, check for threshold changes
                diff.changes.iter().any(|c| {
                    c.old_value
                        .as_ref()
                        .is_some_and(|v| v.contains("threshold") || v.contains("limit"))
                })
            }
            "ambiguous-wording" => {
                // This would require NLP; for now, assume false
                false
            }
            _ => false,
        }
    }

    /// Checks if a best practice is applicable.
    fn is_practice_applicable(
        &self,
        practice: &BestPractice,
        detected_patterns: &[AmendmentPatternInfo],
    ) -> bool {
        if practice.related_patterns.is_empty() {
            // General practices always apply
            return true;
        }

        // Check if any detected pattern matches related patterns
        detected_patterns
            .iter()
            .any(|p| practice.related_patterns.iter().any(|rp| rp == &p.id))
    }

    /// Calculates overall pattern score.
    fn calculate_pattern_score(
        &self,
        patterns: &[AmendmentPatternInfo],
        anti_patterns: &[AntiPattern],
    ) -> f64 {
        if patterns.is_empty() && anti_patterns.is_empty() {
            return 1.0; // No issues detected
        }

        let pattern_score: f64 =
            patterns.iter().map(|p| p.confidence).sum::<f64>() / patterns.len().max(1) as f64;

        let anti_pattern_penalty = anti_patterns
            .iter()
            .map(|ap| match ap.severity {
                AntiPatternSeverity::Info => 0.05,
                AntiPatternSeverity::Warning => 0.15,
                AntiPatternSeverity::Error => 0.30,
                AntiPatternSeverity::Critical => 0.50,
            })
            .sum::<f64>();

        (pattern_score - anti_pattern_penalty).clamp(0.0, 1.0)
    }
}

impl Default for PatternLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Analyzes a diff for patterns and provides suggestions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_diff::{diff, patterns::analyze_patterns};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
///     .with_precondition(Condition::Age {
///         operator: ComparisonOp::GreaterOrEqual,
///         value: 65,
///     });
///
/// let mut new = old.clone();
/// new.preconditions[0] = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 67,
/// };
///
/// let diff_result = diff(&old, &new).unwrap();
/// let pattern_result = analyze_patterns(&diff_result);
///
/// assert!(!pattern_result.patterns.is_empty());
/// ```
pub fn analyze_patterns(diff: &StatuteDiff) -> PatternDetectionResult {
    let library = PatternLibrary::new();
    library.detect_patterns(diff)
}

/// Historical pattern tracker for learning from past amendments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalPatternTracker {
    /// Tracked patterns with frequency
    pattern_frequency: HashMap<String, usize>,
    /// Successful pattern outcomes
    successful_patterns: HashMap<String, usize>,
    /// Failed pattern outcomes
    failed_patterns: HashMap<String, usize>,
}

impl HistoricalPatternTracker {
    /// Creates a new historical pattern tracker.
    pub fn new() -> Self {
        Self {
            pattern_frequency: HashMap::new(),
            successful_patterns: HashMap::new(),
            failed_patterns: HashMap::new(),
        }
    }

    /// Records a pattern occurrence.
    pub fn record_pattern(&mut self, pattern_id: &str) {
        *self
            .pattern_frequency
            .entry(pattern_id.to_string())
            .or_insert(0) += 1;
    }

    /// Records a successful pattern outcome.
    pub fn record_success(&mut self, pattern_id: &str) {
        *self
            .successful_patterns
            .entry(pattern_id.to_string())
            .or_insert(0) += 1;
    }

    /// Records a failed pattern outcome.
    pub fn record_failure(&mut self, pattern_id: &str) {
        *self
            .failed_patterns
            .entry(pattern_id.to_string())
            .or_insert(0) += 1;
    }

    /// Gets success rate for a pattern.
    pub fn success_rate(&self, pattern_id: &str) -> f64 {
        let successes = self
            .successful_patterns
            .get(pattern_id)
            .copied()
            .unwrap_or(0);
        let failures = self.failed_patterns.get(pattern_id).copied().unwrap_or(0);
        let total = successes + failures;

        if total == 0 {
            0.5 // Unknown
        } else {
            successes as f64 / total as f64
        }
    }

    /// Gets pattern frequency.
    pub fn frequency(&self, pattern_id: &str) -> usize {
        self.pattern_frequency.get(pattern_id).copied().unwrap_or(0)
    }

    /// Gets most common patterns.
    pub fn most_common_patterns(&self, limit: usize) -> Vec<(String, usize)> {
        let mut patterns: Vec<_> = self.pattern_frequency.iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(a.1));
        patterns
            .into_iter()
            .take(limit)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
}

impl Default for HistoricalPatternTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Change, ChangeTarget, ImpactAssessment};

    fn create_test_diff() -> StatuteDiff {
        StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Age threshold changed".to_string(),
                old_value: Some("Age >= 65".to_string()),
                new_value: Some("Age >= 67".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        }
    }

    #[test]
    fn test_pattern_library_creation() {
        let library = PatternLibrary::new();
        assert!(!library.patterns.is_empty());
        assert!(!library.anti_patterns.is_empty());
        assert!(!library.best_practices.is_empty());
    }

    #[test]
    fn test_detect_age_adjustment() {
        let library = PatternLibrary::new();
        let diff = create_test_diff();
        let result = library.detect_patterns(&diff);

        assert!(!result.patterns.is_empty());
        assert!(
            result
                .patterns
                .iter()
                .any(|p| p.id == "age-threshold-adjustment")
        );
    }

    #[test]
    fn test_analyze_patterns() {
        let diff = create_test_diff();
        let result = analyze_patterns(&diff);

        assert!(!result.patterns.is_empty());
        assert!(result.pattern_score > 0.0);
    }

    #[test]
    fn test_historical_tracker() {
        let mut tracker = HistoricalPatternTracker::new();

        tracker.record_pattern("age-threshold-adjustment");
        tracker.record_pattern("age-threshold-adjustment");
        tracker.record_success("age-threshold-adjustment");

        assert_eq!(tracker.frequency("age-threshold-adjustment"), 2);
        assert!(tracker.success_rate("age-threshold-adjustment") > 0.0);
    }

    #[test]
    fn test_most_common_patterns() {
        let mut tracker = HistoricalPatternTracker::new();

        tracker.record_pattern("pattern1");
        tracker.record_pattern("pattern1");
        tracker.record_pattern("pattern2");

        let common = tracker.most_common_patterns(1);
        assert_eq!(common.len(), 1);
        assert_eq!(common[0].0, "pattern1");
        assert_eq!(common[0].1, 2);
    }
}
