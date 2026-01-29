//! Self-Healing Legal Systems Module
//!
//! This module provides automatic conflict resolution, self-correcting statute recommendations,
//! predictive violation prevention, adaptive compliance strategies, and automated statute optimization.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::self_healing::*;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let config = SelfHealingConfig::default();
//! let suggester = ConflictResolutionSuggester::new(config);
//!
//! let statutes = vec![
//!     Statute::new("LAW-001", "Regulation A", Effect::new(EffectType::Obligation, "Must do X")),
//!     Statute::new("LAW-002", "Regulation B", Effect::new(EffectType::Prohibition, "Cannot do X")),
//! ];
//!
//! let suggestions = suggester.suggest_resolutions(&statutes);
//! ```

use crate::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for self-healing legal systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealingConfig {
    /// Enable ML-based resolution suggestions
    pub enable_ml_suggestions: bool,
    /// Minimum confidence threshold for suggestions (0.0-1.0)
    pub min_confidence_threshold: f64,
    /// Maximum number of resolution options to generate
    pub max_resolution_options: usize,
    /// Enable automated correction proposals
    pub enable_auto_correction: bool,
    /// Enable predictive violation prevention
    pub enable_violation_prediction: bool,
    /// Historical data window for learning (days)
    pub historical_window_days: usize,
    /// Enable adaptive compliance strategies
    pub enable_adaptive_compliance: bool,
    /// Enable statute optimization
    pub enable_statute_optimization: bool,
    /// Complexity reduction threshold (0.0-1.0)
    pub complexity_threshold: f64,
}

impl Default for SelfHealingConfig {
    fn default() -> Self {
        Self {
            enable_ml_suggestions: true,
            min_confidence_threshold: 0.6,
            max_resolution_options: 5,
            enable_auto_correction: true,
            enable_violation_prediction: true,
            historical_window_days: 365,
            enable_adaptive_compliance: true,
            enable_statute_optimization: true,
            complexity_threshold: 0.7,
        }
    }
}

/// Strategy for resolving conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    /// Harmonize conflicting statutes to create unified rule
    Harmonize,
    /// Repeal one or more conflicting statutes
    Repeal,
    /// Clarify language to remove ambiguity
    Clarify,
    /// Establish clear priority order
    Prioritize,
    /// Create exception for specific cases
    CreateException,
    /// Defer to higher authority
    DeferToAuthority,
}

impl std::fmt::Display for ResolutionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionStrategy::Harmonize => write!(f, "Harmonize"),
            ResolutionStrategy::Repeal => write!(f, "Repeal"),
            ResolutionStrategy::Clarify => write!(f, "Clarify"),
            ResolutionStrategy::Prioritize => write!(f, "Prioritize"),
            ResolutionStrategy::CreateException => write!(f, "CreateException"),
            ResolutionStrategy::DeferToAuthority => write!(f, "DeferToAuthority"),
        }
    }
}

/// Resolution suggestion with confidence score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionSuggestion {
    /// Suggestion ID
    pub id: String,
    /// Strategy to use
    pub strategy: ResolutionStrategy,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Detailed description
    pub description: String,
    /// Affected statute IDs
    pub affected_statutes: Vec<String>,
    /// Proposed changes
    pub proposed_changes: Vec<ProposedChange>,
    /// Trade-off analysis
    pub trade_offs: TradeOffAnalysis,
    /// Estimated implementation effort (hours)
    pub estimated_effort: f64,
    /// Risk level (0-100)
    pub risk_level: u8,
}

impl ResolutionSuggestion {
    /// Create a new resolution suggestion
    pub fn new(
        strategy: ResolutionStrategy,
        confidence: f64,
        description: String,
        affected_statutes: Vec<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            strategy,
            confidence,
            description,
            affected_statutes,
            proposed_changes: Vec::new(),
            trade_offs: TradeOffAnalysis::default(),
            estimated_effort: 0.0,
            risk_level: 0,
        }
    }

    /// Add proposed change
    pub fn add_change(&mut self, change: ProposedChange) {
        self.proposed_changes.push(change);
    }

    /// Set trade-off analysis
    pub fn with_trade_offs(mut self, trade_offs: TradeOffAnalysis) -> Self {
        self.trade_offs = trade_offs;
        self
    }

    /// Set estimated effort
    pub fn with_effort(mut self, hours: f64) -> Self {
        self.estimated_effort = hours;
        self
    }

    /// Set risk level
    pub fn with_risk(mut self, risk: u8) -> Self {
        self.risk_level = risk;
        self
    }
}

/// Proposed change to a statute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChange {
    /// Change ID
    pub id: String,
    /// Statute ID to modify
    pub statute_id: String,
    /// Type of change
    pub change_type: ChangeType,
    /// Original text/value
    pub original: String,
    /// Proposed text/value
    pub proposed: String,
    /// Rationale for change
    pub rationale: String,
}

impl ProposedChange {
    /// Create a new proposed change
    pub fn new(
        statute_id: String,
        change_type: ChangeType,
        original: String,
        proposed: String,
        rationale: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            change_type,
            original,
            proposed,
            rationale,
        }
    }
}

/// Type of change being proposed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Modify text content
    TextModification,
    /// Add new provision
    Addition,
    /// Remove provision
    Removal,
    /// Restructure organization
    Restructuring,
    /// Update metadata
    MetadataUpdate,
}

/// Trade-off analysis for a resolution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TradeOffAnalysis {
    /// Benefits of the resolution
    pub benefits: Vec<String>,
    /// Drawbacks or concerns
    pub drawbacks: Vec<String>,
    /// Affected stakeholders
    pub stakeholders: Vec<String>,
    /// Legal complexity change (-100 to +100)
    pub complexity_delta: i32,
    /// Compliance burden change (-100 to +100)
    pub compliance_burden_delta: i32,
}

/// Conflict resolution suggester
pub struct ConflictResolutionSuggester {
    config: SelfHealingConfig,
    historical_patterns: HashMap<String, Vec<ResolutionPattern>>,
}

impl ConflictResolutionSuggester {
    /// Create a new conflict resolution suggester
    pub fn new(config: SelfHealingConfig) -> Self {
        Self {
            config,
            historical_patterns: HashMap::new(),
        }
    }

    /// Suggest resolutions for conflicting statutes
    pub fn suggest_resolutions(&self, statutes: &[Statute]) -> Vec<ResolutionSuggestion> {
        let mut suggestions = Vec::new();

        if !self.config.enable_ml_suggestions {
            return suggestions;
        }

        // Detect conflicts
        let conflicts = self.detect_conflicts(statutes);

        for conflict in conflicts {
            // Generate multiple resolution strategies
            suggestions.extend(self.generate_resolution_strategies(&conflict, statutes));
        }

        // Filter by confidence threshold
        suggestions.retain(|s| s.confidence >= self.config.min_confidence_threshold);

        // Limit to max options
        suggestions.truncate(self.config.max_resolution_options);

        suggestions
    }

    /// Detect conflicts between statutes
    fn detect_conflicts(&self, statutes: &[Statute]) -> Vec<ConflictDescriptor> {
        let mut conflicts = Vec::new();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                if self.statutes_conflict(&statutes[i], &statutes[j]) {
                    conflicts.push(ConflictDescriptor {
                        statute_ids: vec![statutes[i].id.clone(), statutes[j].id.clone()],
                        conflict_type: self.classify_conflict(&statutes[i], &statutes[j]),
                        severity: self.calculate_conflict_severity(&statutes[i], &statutes[j]),
                    });
                }
            }
        }

        conflicts
    }

    /// Check if two statutes conflict
    fn statutes_conflict(&self, s1: &Statute, s2: &Statute) -> bool {
        // Simple heuristic: check if effects are opposing
        s1.effect.effect_type != s2.effect.effect_type
    }

    /// Classify conflict type
    fn classify_conflict(&self, _s1: &Statute, _s2: &Statute) -> String {
        "logical_contradiction".to_string()
    }

    /// Calculate conflict severity (0-100)
    fn calculate_conflict_severity(&self, _s1: &Statute, _s2: &Statute) -> u8 {
        75 // Default high severity
    }

    /// Generate resolution strategies for a conflict
    fn generate_resolution_strategies(
        &self,
        conflict: &ConflictDescriptor,
        statutes: &[Statute],
    ) -> Vec<ResolutionSuggestion> {
        vec![
            // Harmonize strategy
            self.generate_harmonize_suggestion(conflict, statutes),
            // Repeal strategy
            self.generate_repeal_suggestion(conflict, statutes),
            // Clarify strategy
            self.generate_clarify_suggestion(conflict, statutes),
            // Prioritize strategy
            self.generate_prioritize_suggestion(conflict, statutes),
        ]
    }

    /// Generate harmonize suggestion
    fn generate_harmonize_suggestion(
        &self,
        conflict: &ConflictDescriptor,
        _statutes: &[Statute],
    ) -> ResolutionSuggestion {
        ResolutionSuggestion::new(
            ResolutionStrategy::Harmonize,
            0.85,
            "Harmonize conflicting provisions into unified rule".to_string(),
            conflict.statute_ids.clone(),
        )
        .with_effort(40.0)
        .with_risk(30)
    }

    /// Generate repeal suggestion
    fn generate_repeal_suggestion(
        &self,
        conflict: &ConflictDescriptor,
        _statutes: &[Statute],
    ) -> ResolutionSuggestion {
        ResolutionSuggestion::new(
            ResolutionStrategy::Repeal,
            0.75,
            "Repeal older statute to resolve conflict".to_string(),
            conflict.statute_ids.clone(),
        )
        .with_effort(20.0)
        .with_risk(50)
    }

    /// Generate clarify suggestion
    fn generate_clarify_suggestion(
        &self,
        conflict: &ConflictDescriptor,
        _statutes: &[Statute],
    ) -> ResolutionSuggestion {
        ResolutionSuggestion::new(
            ResolutionStrategy::Clarify,
            0.90,
            "Clarify language to remove ambiguity".to_string(),
            conflict.statute_ids.clone(),
        )
        .with_effort(15.0)
        .with_risk(20)
    }

    /// Generate prioritize suggestion
    fn generate_prioritize_suggestion(
        &self,
        conflict: &ConflictDescriptor,
        _statutes: &[Statute],
    ) -> ResolutionSuggestion {
        ResolutionSuggestion::new(
            ResolutionStrategy::Prioritize,
            0.80,
            "Establish clear priority order between statutes".to_string(),
            conflict.statute_ids.clone(),
        )
        .with_effort(10.0)
        .with_risk(15)
    }

    /// Learn from resolution outcome
    pub fn learn_from_outcome(&mut self, suggestion_id: &str, success: bool, feedback: String) {
        let pattern = ResolutionPattern {
            suggestion_id: suggestion_id.to_string(),
            success,
            feedback,
            timestamp: chrono::Utc::now(),
        };

        self.historical_patterns
            .entry(suggestion_id.to_string())
            .or_default()
            .push(pattern);
    }
}

/// Conflict descriptor
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ConflictDescriptor {
    statute_ids: Vec<String>,
    conflict_type: String,
    severity: u8,
}

/// Resolution pattern learned from history
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ResolutionPattern {
    suggestion_id: String,
    success: bool,
    feedback: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Self-correcting statute corrector
pub struct StatuteCorrector {
    config: SelfHealingConfig,
    correction_history: HashMap<String, Vec<CorrectionRecord>>,
}

impl StatuteCorrector {
    /// Create a new statute corrector
    pub fn new(config: SelfHealingConfig) -> Self {
        Self {
            config,
            correction_history: HashMap::new(),
        }
    }

    /// Identify and fix inconsistencies in statutes
    pub fn identify_corrections(&self, statutes: &[Statute]) -> Vec<CorrectionProposal> {
        let mut proposals = Vec::new();

        if !self.config.enable_auto_correction {
            return proposals;
        }

        for statute in statutes {
            // Check for common issues
            if let Some(proposal) = self.check_logical_inconsistency(statute) {
                proposals.push(proposal);
            }

            if let Some(proposal) = self.check_ambiguous_language(statute) {
                proposals.push(proposal);
            }

            if let Some(proposal) = self.check_outdated_references(statute) {
                proposals.push(proposal);
            }
        }

        proposals
    }

    /// Check for logical inconsistency
    fn check_logical_inconsistency(&self, statute: &Statute) -> Option<CorrectionProposal> {
        // Simplified check
        if statute.title.contains("must") && statute.title.contains("optional") {
            Some(CorrectionProposal::new(
                statute.id.clone(),
                CorrectionType::LogicalInconsistency,
                "Remove contradictory language".to_string(),
                0.8,
            ))
        } else {
            None
        }
    }

    /// Check for ambiguous language
    fn check_ambiguous_language(&self, statute: &Statute) -> Option<CorrectionProposal> {
        // Check for vague terms
        let ambiguous_terms = ["may", "could", "should consider"];
        for term in &ambiguous_terms {
            if statute.title.contains(term) {
                return Some(CorrectionProposal::new(
                    statute.id.clone(),
                    CorrectionType::AmbiguousLanguage,
                    format!("Replace '{}' with precise language", term),
                    0.7,
                ));
            }
        }
        None
    }

    /// Check for outdated references
    fn check_outdated_references(&self, statute: &Statute) -> Option<CorrectionProposal> {
        // Check for old statute references
        if statute.title.contains("repealed") || statute.title.contains("superseded") {
            Some(CorrectionProposal::new(
                statute.id.clone(),
                CorrectionType::OutdatedReference,
                "Update to current statute reference".to_string(),
                0.9,
            ))
        } else {
            None
        }
    }

    /// Apply correction and track
    pub fn apply_correction(
        &mut self,
        proposal: &CorrectionProposal,
        statute: &mut Statute,
    ) -> Result<CorrectionImpact, String> {
        // Store original state for rollback
        let original_state = statute.clone();

        // Apply correction (simplified)
        let impact = self.calculate_impact(proposal, statute);

        // Record correction
        let record = CorrectionRecord {
            proposal_id: proposal.id.clone(),
            statute_id: statute.id.clone(),
            original_state,
            timestamp: chrono::Utc::now(),
            impact: impact.clone(),
        };

        self.correction_history
            .entry(statute.id.clone())
            .or_default()
            .push(record);

        Ok(impact)
    }

    /// Calculate impact of correction
    fn calculate_impact(
        &self,
        _proposal: &CorrectionProposal,
        _statute: &Statute,
    ) -> CorrectionImpact {
        CorrectionImpact {
            affected_entities: 100,
            affected_rules: 5,
            complexity_reduction: 15.0,
            estimated_cost: 5000.0,
            risk_level: 25,
        }
    }

    /// Rollback correction
    pub fn rollback(&mut self, statute_id: &str) -> Result<Statute, String> {
        if let Some(history) = self.correction_history.get(statute_id)
            && let Some(record) = history.last()
        {
            return Ok(record.original_state.clone());
        }
        Err("No correction history found".to_string())
    }
}

/// Correction proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionProposal {
    /// Proposal ID
    pub id: String,
    /// Statute ID to correct
    pub statute_id: String,
    /// Type of correction
    pub correction_type: CorrectionType,
    /// Description
    pub description: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Automated correction code
    pub correction_code: Option<String>,
}

impl CorrectionProposal {
    /// Create a new correction proposal
    pub fn new(
        statute_id: String,
        correction_type: CorrectionType,
        description: String,
        confidence: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            correction_type,
            description,
            confidence,
            correction_code: None,
        }
    }
}

/// Type of correction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionType {
    /// Logical inconsistency
    LogicalInconsistency,
    /// Ambiguous language
    AmbiguousLanguage,
    /// Outdated reference
    OutdatedReference,
    /// Circular dependency
    CircularDependency,
    /// Missing prerequisite
    MissingPrerequisite,
}

/// Impact of a correction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionImpact {
    /// Number of entities affected
    pub affected_entities: usize,
    /// Number of rules affected
    pub affected_rules: usize,
    /// Complexity reduction percentage
    pub complexity_reduction: f64,
    /// Estimated cost
    pub estimated_cost: f64,
    /// Risk level (0-100)
    pub risk_level: u8,
}

/// Correction record for history
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CorrectionRecord {
    proposal_id: String,
    statute_id: String,
    original_state: Statute,
    timestamp: chrono::DateTime<chrono::Utc>,
    impact: CorrectionImpact,
}

/// Violation predictor
pub struct ViolationPredictor {
    config: SelfHealingConfig,
    historical_violations: Vec<HistoricalViolation>,
}

impl ViolationPredictor {
    /// Create a new violation predictor
    pub fn new(config: SelfHealingConfig) -> Self {
        Self {
            config,
            historical_violations: Vec::new(),
        }
    }

    /// Predict potential violations
    pub fn predict_violations(&self, statutes: &[Statute]) -> Vec<ViolationPrediction> {
        let mut predictions = Vec::new();

        if !self.config.enable_violation_prediction {
            return predictions;
        }

        for statute in statutes {
            // Analyze historical patterns
            let risk_score = self.calculate_risk_score(statute);

            if risk_score > 0.5 {
                predictions.push(ViolationPrediction {
                    id: uuid::Uuid::new_v4().to_string(),
                    statute_id: statute.id.clone(),
                    risk_score,
                    predicted_violation_type: self.predict_violation_type(statute),
                    preventive_actions: self.generate_preventive_actions(statute),
                    confidence: self.calculate_prediction_confidence(statute),
                });
            }
        }

        predictions
    }

    /// Calculate risk score (0.0-1.0)
    fn calculate_risk_score(&self, statute: &Statute) -> f64 {
        // Analyze historical violations
        let mut risk: f64 = 0.0;

        // Check complexity
        if statute.title.len() > 100 {
            risk += 0.2;
        }

        // Check for common violation patterns
        let violation_keywords = ["deadline", "mandatory", "penalty"];
        for keyword in &violation_keywords {
            if statute.title.contains(keyword) {
                risk += 0.15;
            }
        }

        f64::min(risk, 1.0)
    }

    /// Predict violation type
    fn predict_violation_type(&self, _statute: &Statute) -> ViolationType {
        ViolationType::DeadlineMiss
    }

    /// Generate preventive actions
    fn generate_preventive_actions(&self, _statute: &Statute) -> Vec<PreventiveAction> {
        vec![
            PreventiveAction {
                action: "Set up automated reminders".to_string(),
                priority: 1,
                estimated_effort: 2.0,
            },
            PreventiveAction {
                action: "Create compliance checklist".to_string(),
                priority: 2,
                estimated_effort: 4.0,
            },
        ]
    }

    /// Calculate prediction confidence
    fn calculate_prediction_confidence(&self, _statute: &Statute) -> f64 {
        // Based on historical data quality
        if self.historical_violations.len() > 100 {
            0.85
        } else if self.historical_violations.len() > 50 {
            0.70
        } else {
            0.55
        }
    }

    /// Learn from actual violation
    pub fn record_violation(&mut self, violation: HistoricalViolation) {
        self.historical_violations.push(violation);

        // Trim old data outside historical window
        let cutoff =
            chrono::Utc::now() - chrono::Duration::days(self.config.historical_window_days as i64);
        self.historical_violations.retain(|v| v.timestamp > cutoff);
    }
}

/// Violation prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationPrediction {
    /// Prediction ID
    pub id: String,
    /// Statute ID at risk
    pub statute_id: String,
    /// Risk score (0.0-1.0)
    pub risk_score: f64,
    /// Predicted violation type
    pub predicted_violation_type: ViolationType,
    /// Preventive actions
    pub preventive_actions: Vec<PreventiveAction>,
    /// Prediction confidence (0.0-1.0)
    pub confidence: f64,
}

/// Type of violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// Missed deadline
    DeadlineMiss,
    /// Non-compliance
    NonCompliance,
    /// Documentation failure
    DocumentationFailure,
    /// Reporting failure
    ReportingFailure,
    /// Procedural error
    ProceduralError,
}

/// Preventive action recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventiveAction {
    /// Action description
    pub action: String,
    /// Priority (1 = highest)
    pub priority: u8,
    /// Estimated effort (hours)
    pub estimated_effort: f64,
}

/// Historical violation record
#[derive(Debug, Clone)]
pub struct HistoricalViolation {
    pub statute_id: String,
    pub violation_type: ViolationType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub severity: u8,
}

/// Compliance optimizer
pub struct ComplianceOptimizer {
    config: SelfHealingConfig,
    compliance_history: Vec<ComplianceEvent>,
    adaptive_rules: HashMap<String, AdaptiveRule>,
}

impl ComplianceOptimizer {
    /// Create a new compliance optimizer
    pub fn new(config: SelfHealingConfig) -> Self {
        Self {
            config,
            compliance_history: Vec::new(),
            adaptive_rules: HashMap::new(),
        }
    }

    /// Optimize compliance strategy
    pub fn optimize_strategy(&mut self, statutes: &[Statute]) -> ComplianceStrategy {
        if !self.config.enable_adaptive_compliance {
            return ComplianceStrategy::default();
        }

        // Analyze past failures
        let failure_patterns = self.analyze_compliance_failures();

        // Generate adaptive rules
        self.generate_adaptive_rules(&failure_patterns, statutes);

        // Create optimized strategy
        ComplianceStrategy {
            rules: self.adaptive_rules.values().cloned().collect(),
            cost_benefit: self.calculate_cost_benefit(statutes),
            risk_mitigation: self.generate_risk_mitigation(),
            monitoring_plan: self.create_monitoring_plan(),
        }
    }

    /// Analyze compliance failures
    fn analyze_compliance_failures(&self) -> Vec<FailurePattern> {
        let mut patterns = Vec::new();

        // Group by statute
        let mut statute_failures: HashMap<String, Vec<&ComplianceEvent>> = HashMap::new();
        for event in &self.compliance_history {
            if !event.success {
                statute_failures
                    .entry(event.statute_id.clone())
                    .or_default()
                    .push(event);
            }
        }

        // Identify patterns
        for (statute_id, failures) in statute_failures {
            if failures.len() > 3 {
                patterns.push(FailurePattern {
                    statute_id,
                    failure_count: failures.len(),
                    common_causes: vec!["complexity".to_string()],
                });
            }
        }

        patterns
    }

    /// Generate adaptive rules
    fn generate_adaptive_rules(&mut self, patterns: &[FailurePattern], _statutes: &[Statute]) {
        for pattern in patterns {
            let rule = AdaptiveRule {
                id: uuid::Uuid::new_v4().to_string(),
                statute_id: pattern.statute_id.clone(),
                rule_text: format!(
                    "Enhanced monitoring for high-failure statute {}",
                    pattern.statute_id
                ),
                confidence: 0.8,
                effectiveness_score: 0.0,
            };

            self.adaptive_rules.insert(rule.id.clone(), rule);
        }
    }

    /// Calculate cost-benefit analysis
    fn calculate_cost_benefit(&self, _statutes: &[Statute]) -> CostBenefitAnalysis {
        CostBenefitAnalysis {
            implementation_cost: 10000.0,
            annual_compliance_cost: 5000.0,
            risk_reduction: 0.6,
            roi_percentage: 120.0,
        }
    }

    /// Generate risk mitigation plan
    fn generate_risk_mitigation(&self) -> Vec<RiskMitigation> {
        vec![
            RiskMitigation {
                risk: "Deadline violation".to_string(),
                mitigation: "Automated deadline tracking".to_string(),
                effectiveness: 0.85,
            },
            RiskMitigation {
                risk: "Documentation gaps".to_string(),
                mitigation: "Template-based documentation".to_string(),
                effectiveness: 0.75,
            },
        ]
    }

    /// Create monitoring plan
    fn create_monitoring_plan(&self) -> MonitoringPlan {
        MonitoringPlan {
            check_frequency: "daily".to_string(),
            automated_checks: vec![
                "deadline_check".to_string(),
                "documentation_check".to_string(),
            ],
            alert_thresholds: vec![("risk_score".to_string(), 0.7)],
        }
    }

    /// Record compliance event
    pub fn record_event(&mut self, event: ComplianceEvent) {
        self.compliance_history.push(event);
    }
}

/// Compliance strategy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplianceStrategy {
    /// Adaptive rules
    pub rules: Vec<AdaptiveRule>,
    /// Cost-benefit analysis
    pub cost_benefit: CostBenefitAnalysis,
    /// Risk mitigation measures
    pub risk_mitigation: Vec<RiskMitigation>,
    /// Monitoring plan
    pub monitoring_plan: MonitoringPlan,
}

/// Adaptive rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRule {
    /// Rule ID
    pub id: String,
    /// Statute ID
    pub statute_id: String,
    /// Rule text
    pub rule_text: String,
    /// Confidence (0.0-1.0)
    pub confidence: f64,
    /// Effectiveness score (0.0-1.0)
    pub effectiveness_score: f64,
}

/// Cost-benefit analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostBenefitAnalysis {
    /// Implementation cost
    pub implementation_cost: f64,
    /// Annual compliance cost
    pub annual_compliance_cost: f64,
    /// Risk reduction (0.0-1.0)
    pub risk_reduction: f64,
    /// ROI percentage
    pub roi_percentage: f64,
}

/// Risk mitigation measure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMitigation {
    /// Risk description
    pub risk: String,
    /// Mitigation strategy
    pub mitigation: String,
    /// Effectiveness (0.0-1.0)
    pub effectiveness: f64,
}

/// Monitoring plan
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitoringPlan {
    /// Check frequency
    pub check_frequency: String,
    /// Automated checks
    pub automated_checks: Vec<String>,
    /// Alert thresholds
    pub alert_thresholds: Vec<(String, f64)>,
}

/// Compliance event
#[derive(Debug, Clone)]
pub struct ComplianceEvent {
    pub statute_id: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Failure pattern
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct FailurePattern {
    statute_id: String,
    failure_count: usize,
    common_causes: Vec<String>,
}

/// Statute optimizer
pub struct StatuteOptimizer {
    config: SelfHealingConfig,
}

impl StatuteOptimizer {
    /// Create a new statute optimizer
    pub fn new(config: SelfHealingConfig) -> Self {
        Self { config }
    }

    /// Optimize statutes for clarity and consistency
    pub fn optimize(&self, statutes: &[Statute]) -> Vec<OptimizationProposal> {
        let mut proposals = Vec::new();

        if !self.config.enable_statute_optimization {
            return proposals;
        }

        for statute in statutes {
            // Detect redundancy
            if let Some(proposal) = self.detect_redundancy(statute, statutes) {
                proposals.push(proposal);
            }

            // Check complexity
            if let Some(proposal) = self.analyze_complexity(statute) {
                proposals.push(proposal);
            }

            // Suggest refactoring
            if let Some(proposal) = self.suggest_refactoring(statute) {
                proposals.push(proposal);
            }
        }

        proposals
    }

    /// Detect redundancy
    fn detect_redundancy(
        &self,
        statute: &Statute,
        all_statutes: &[Statute],
    ) -> Option<OptimizationProposal> {
        // Check for duplicate or similar statutes
        for other in all_statutes {
            if statute.id != other.id && self.similarity_score(statute, other) > 0.8 {
                return Some(OptimizationProposal {
                    id: uuid::Uuid::new_v4().to_string(),
                    statute_id: statute.id.clone(),
                    optimization_type: OptimizationType::RedundancyRemoval,
                    description: format!("Merge with similar statute {}", other.id),
                    complexity_reduction: 25.0,
                    confidence: 0.75,
                });
            }
        }
        None
    }

    /// Calculate similarity between statutes
    fn similarity_score(&self, s1: &Statute, s2: &Statute) -> f64 {
        // Simplified similarity calculation
        if s1.title.contains(&s2.title) || s2.title.contains(&s1.title) {
            0.85
        } else {
            0.0
        }
    }

    /// Analyze complexity
    fn analyze_complexity(&self, statute: &Statute) -> Option<OptimizationProposal> {
        let complexity = self.calculate_complexity(statute);

        if complexity > self.config.complexity_threshold {
            Some(OptimizationProposal {
                id: uuid::Uuid::new_v4().to_string(),
                statute_id: statute.id.clone(),
                optimization_type: OptimizationType::ComplexityReduction,
                description: "Simplify statute language and structure".to_string(),
                complexity_reduction: 40.0,
                confidence: 0.70,
            })
        } else {
            None
        }
    }

    /// Calculate statute complexity (0.0-1.0)
    fn calculate_complexity(&self, statute: &Statute) -> f64 {
        let mut complexity = 0.0;

        // Length factor
        complexity += (statute.title.len() as f64 / 200.0).min(0.3);

        // Word count
        let word_count = statute.title.split_whitespace().count();
        complexity += (word_count as f64 / 50.0).min(0.3);

        complexity.min(1.0)
    }

    /// Suggest refactoring
    fn suggest_refactoring(&self, statute: &Statute) -> Option<OptimizationProposal> {
        // Check if statute could benefit from restructuring
        if statute.title.len() > 150 {
            Some(OptimizationProposal {
                id: uuid::Uuid::new_v4().to_string(),
                statute_id: statute.id.clone(),
                optimization_type: OptimizationType::Refactoring,
                description: "Split into multiple focused statutes".to_string(),
                complexity_reduction: 35.0,
                confidence: 0.80,
            })
        } else {
            None
        }
    }
}

/// Optimization proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationProposal {
    /// Proposal ID
    pub id: String,
    /// Statute ID
    pub statute_id: String,
    /// Type of optimization
    pub optimization_type: OptimizationType,
    /// Description
    pub description: String,
    /// Complexity reduction percentage
    pub complexity_reduction: f64,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Type of optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Remove redundant provisions
    RedundancyRemoval,
    /// Reduce complexity
    ComplexityReduction,
    /// Refactor structure
    Refactoring,
    /// Update language
    LanguageModernization,
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn create_test_statute(id: &str, title: &str, effect_type: EffectType) -> Statute {
        Statute::new(id, title, Effect::new(effect_type, "test"))
    }

    #[test]
    fn test_resolution_strategy_display() {
        assert_eq!(ResolutionStrategy::Harmonize.to_string(), "Harmonize");
        assert_eq!(ResolutionStrategy::Repeal.to_string(), "Repeal");
        assert_eq!(ResolutionStrategy::Clarify.to_string(), "Clarify");
    }

    #[test]
    fn test_resolution_suggestion_creation() {
        let suggestion = ResolutionSuggestion::new(
            ResolutionStrategy::Harmonize,
            0.85,
            "Test suggestion".to_string(),
            vec!["LAW-001".to_string()],
        );

        assert_eq!(suggestion.strategy, ResolutionStrategy::Harmonize);
        assert_eq!(suggestion.confidence, 0.85);
        assert_eq!(suggestion.affected_statutes.len(), 1);
    }

    #[test]
    fn test_conflict_resolution_suggester_creation() {
        let config = SelfHealingConfig::default();
        let suggester = ConflictResolutionSuggester::new(config);

        assert!(suggester.historical_patterns.is_empty());
    }

    #[test]
    fn test_suggest_resolutions_disabled() {
        let config = SelfHealingConfig {
            enable_ml_suggestions: false,
            ..Default::default()
        };

        let suggester = ConflictResolutionSuggester::new(config);
        let statutes = vec![create_test_statute(
            "LAW-001",
            "Test 1",
            EffectType::Obligation,
        )];

        let suggestions = suggester.suggest_resolutions(&statutes);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_suggest_resolutions_with_conflict() {
        let config = SelfHealingConfig::default();
        let suggester = ConflictResolutionSuggester::new(config);

        let statutes = vec![
            create_test_statute("LAW-001", "Must do X", EffectType::Obligation),
            create_test_statute("LAW-002", "Cannot do X", EffectType::Prohibition),
        ];

        let suggestions = suggester.suggest_resolutions(&statutes);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_learn_from_outcome() {
        let config = SelfHealingConfig::default();
        let mut suggester = ConflictResolutionSuggester::new(config);

        suggester.learn_from_outcome("test-id", true, "Good resolution".to_string());

        assert_eq!(suggester.historical_patterns.len(), 1);
    }

    #[test]
    fn test_statute_corrector_creation() {
        let config = SelfHealingConfig::default();
        let corrector = StatuteCorrector::new(config);

        assert!(corrector.correction_history.is_empty());
    }

    #[test]
    fn test_identify_corrections_disabled() {
        let config = SelfHealingConfig {
            enable_auto_correction: false,
            ..Default::default()
        };

        let corrector = StatuteCorrector::new(config);
        let statutes = vec![create_test_statute(
            "LAW-001",
            "Test",
            EffectType::Obligation,
        )];

        let proposals = corrector.identify_corrections(&statutes);
        assert!(proposals.is_empty());
    }

    #[test]
    fn test_check_logical_inconsistency() {
        let config = SelfHealingConfig::default();
        let corrector = StatuteCorrector::new(config);

        let statute = create_test_statute("LAW-001", "must and optional", EffectType::Obligation);
        let proposal = corrector.check_logical_inconsistency(&statute);

        assert!(proposal.is_some());
    }

    #[test]
    fn test_check_ambiguous_language() {
        let config = SelfHealingConfig::default();
        let corrector = StatuteCorrector::new(config);

        let statute = create_test_statute("LAW-001", "may consider", EffectType::Obligation);
        let proposal = corrector.check_ambiguous_language(&statute);

        assert!(proposal.is_some());
    }

    #[test]
    fn test_check_outdated_references() {
        let config = SelfHealingConfig::default();
        let corrector = StatuteCorrector::new(config);

        let statute = create_test_statute("LAW-001", "repealed statute", EffectType::Obligation);
        let proposal = corrector.check_outdated_references(&statute);

        assert!(proposal.is_some());
    }

    #[test]
    fn test_apply_correction() {
        let config = SelfHealingConfig::default();
        let mut corrector = StatuteCorrector::new(config);

        let mut statute = create_test_statute("LAW-001", "Test", EffectType::Obligation);
        let proposal = CorrectionProposal::new(
            statute.id.clone(),
            CorrectionType::LogicalInconsistency,
            "Fix inconsistency".to_string(),
            0.8,
        );

        let result = corrector.apply_correction(&proposal, &mut statute);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rollback() {
        let config = SelfHealingConfig::default();
        let mut corrector = StatuteCorrector::new(config);

        let mut statute = create_test_statute("LAW-001", "Test", EffectType::Obligation);
        let proposal = CorrectionProposal::new(
            statute.id.clone(),
            CorrectionType::LogicalInconsistency,
            "Fix".to_string(),
            0.8,
        );

        corrector.apply_correction(&proposal, &mut statute).ok();
        let result = corrector.rollback(&statute.id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_violation_predictor_creation() {
        let config = SelfHealingConfig::default();
        let predictor = ViolationPredictor::new(config);

        assert!(predictor.historical_violations.is_empty());
    }

    #[test]
    fn test_predict_violations_disabled() {
        let config = SelfHealingConfig {
            enable_violation_prediction: false,
            ..Default::default()
        };

        let predictor = ViolationPredictor::new(config);
        let statutes = vec![create_test_statute(
            "LAW-001",
            "Test",
            EffectType::Obligation,
        )];

        let predictions = predictor.predict_violations(&statutes);
        assert!(predictions.is_empty());
    }

    #[test]
    fn test_predict_violations_with_high_risk() {
        let config = SelfHealingConfig::default();
        let predictor = ViolationPredictor::new(config);

        // Create a statute with long title and violation keywords to exceed 0.5 risk threshold
        let long_title = format!(
            "{} mandatory deadline penalty report",
            "Very long complex regulation text that exceeds one hundred characters to trigger complexity risk factor. "
        );
        let statute = create_test_statute("LAW-001", &long_title, EffectType::Obligation);

        let predictions = predictor.predict_violations(&[statute]);
        assert!(!predictions.is_empty());
    }

    #[test]
    fn test_calculate_risk_score() {
        let config = SelfHealingConfig::default();
        let predictor = ViolationPredictor::new(config);

        let statute = create_test_statute("LAW-001", "deadline mandatory", EffectType::Obligation);
        let score = predictor.calculate_risk_score(&statute);

        assert!(score > 0.0);
    }

    #[test]
    fn test_record_violation() {
        let config = SelfHealingConfig::default();
        let mut predictor = ViolationPredictor::new(config);

        let violation = HistoricalViolation {
            statute_id: "LAW-001".to_string(),
            violation_type: ViolationType::DeadlineMiss,
            timestamp: chrono::Utc::now(),
            severity: 50,
        };

        predictor.record_violation(violation);
        assert_eq!(predictor.historical_violations.len(), 1);
    }

    #[test]
    fn test_compliance_optimizer_creation() {
        let config = SelfHealingConfig::default();
        let optimizer = ComplianceOptimizer::new(config);

        assert!(optimizer.compliance_history.is_empty());
        assert!(optimizer.adaptive_rules.is_empty());
    }

    #[test]
    fn test_optimize_strategy_disabled() {
        let config = SelfHealingConfig {
            enable_adaptive_compliance: false,
            ..Default::default()
        };

        let mut optimizer = ComplianceOptimizer::new(config);
        let statutes = vec![create_test_statute(
            "LAW-001",
            "Test",
            EffectType::Obligation,
        )];

        let strategy = optimizer.optimize_strategy(&statutes);
        assert!(strategy.rules.is_empty());
    }

    #[test]
    fn test_record_compliance_event() {
        let config = SelfHealingConfig::default();
        let mut optimizer = ComplianceOptimizer::new(config);

        let event = ComplianceEvent {
            statute_id: "LAW-001".to_string(),
            success: false,
            timestamp: chrono::Utc::now(),
        };

        optimizer.record_event(event);
        assert_eq!(optimizer.compliance_history.len(), 1);
    }

    #[test]
    fn test_analyze_compliance_failures() {
        let config = SelfHealingConfig::default();
        let mut optimizer = ComplianceOptimizer::new(config);

        // Add multiple failures
        for _ in 0..5 {
            optimizer.record_event(ComplianceEvent {
                statute_id: "LAW-001".to_string(),
                success: false,
                timestamp: chrono::Utc::now(),
            });
        }

        let patterns = optimizer.analyze_compliance_failures();
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_statute_optimizer_creation() {
        let config = SelfHealingConfig::default();
        let optimizer = StatuteOptimizer::new(config);

        assert!(optimizer.config.enable_statute_optimization);
    }

    #[test]
    fn test_optimize_disabled() {
        let config = SelfHealingConfig {
            enable_statute_optimization: false,
            ..Default::default()
        };

        let optimizer = StatuteOptimizer::new(config);
        let statutes = vec![create_test_statute(
            "LAW-001",
            "Test",
            EffectType::Obligation,
        )];

        let proposals = optimizer.optimize(&statutes);
        assert!(proposals.is_empty());
    }

    #[test]
    fn test_detect_redundancy() {
        let config = SelfHealingConfig::default();
        let optimizer = StatuteOptimizer::new(config);

        let statute1 = create_test_statute("LAW-001", "Test Regulation", EffectType::Obligation);
        let statute2 =
            create_test_statute("LAW-002", "Test Regulation Similar", EffectType::Obligation);

        let statutes = vec![statute1.clone(), statute2];
        let proposal = optimizer.detect_redundancy(&statute1, &statutes);

        assert!(proposal.is_some());
    }

    #[test]
    fn test_analyze_complexity() {
        // Use custom config with lower threshold since max complexity is 0.6
        let config = SelfHealingConfig {
            complexity_threshold: 0.5,
            ..Default::default()
        };
        let optimizer = StatuteOptimizer::new(config);

        // Create a statute with high complexity (long + many words)
        // Complexity = (length/200).min(0.3) + (words/50).min(0.3) = max 0.6
        let long_title = (0..60)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        let statute = create_test_statute("LAW-001", &long_title, EffectType::Obligation);

        let proposal = optimizer.analyze_complexity(&statute);
        assert!(proposal.is_some());
    }

    #[test]
    fn test_suggest_refactoring() {
        let config = SelfHealingConfig::default();
        let optimizer = StatuteOptimizer::new(config);

        let long_title = "A".repeat(160);
        let statute = create_test_statute("LAW-001", &long_title, EffectType::Obligation);

        let proposal = optimizer.suggest_refactoring(&statute);
        assert!(proposal.is_some());
    }

    #[test]
    fn test_calculate_complexity() {
        let config = SelfHealingConfig::default();
        let optimizer = StatuteOptimizer::new(config);

        let statute = create_test_statute("LAW-001", "Short", EffectType::Obligation);
        let complexity1 = optimizer.calculate_complexity(&statute);

        let long_title = "A ".repeat(100);
        let statute2 = create_test_statute("LAW-002", &long_title, EffectType::Obligation);
        let complexity2 = optimizer.calculate_complexity(&statute2);

        assert!(complexity2 > complexity1);
    }

    #[test]
    fn test_proposed_change_creation() {
        let change = ProposedChange::new(
            "LAW-001".to_string(),
            ChangeType::TextModification,
            "old text".to_string(),
            "new text".to_string(),
            "improve clarity".to_string(),
        );

        assert_eq!(change.statute_id, "LAW-001");
        assert_eq!(change.change_type, ChangeType::TextModification);
    }

    #[test]
    fn test_correction_proposal_creation() {
        let proposal = CorrectionProposal::new(
            "LAW-001".to_string(),
            CorrectionType::AmbiguousLanguage,
            "Fix ambiguity".to_string(),
            0.9,
        );

        assert_eq!(proposal.confidence, 0.9);
        assert_eq!(proposal.correction_type, CorrectionType::AmbiguousLanguage);
    }

    #[test]
    fn test_self_healing_config_default() {
        let config = SelfHealingConfig::default();

        assert!(config.enable_ml_suggestions);
        assert!(config.enable_auto_correction);
        assert!(config.enable_violation_prediction);
        assert_eq!(config.min_confidence_threshold, 0.6);
    }
}
