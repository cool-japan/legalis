//! Risk scoring models for compliance and decision analysis.
//!
//! This module provides risk assessment and scoring capabilities to evaluate
//! the compliance risk level of decisions, actors, and patterns.

use crate::{Actor, AuditError, AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for risk scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScoringConfig {
    /// Weight for override risk factor (0.0-1.0)
    pub override_weight: f64,
    /// Weight for volume risk factor (0.0-1.0)
    pub volume_weight: f64,
    /// Weight for temporal risk factor (0.0-1.0)
    pub temporal_weight: f64,
    /// Weight for pattern risk factor (0.0-1.0)
    pub pattern_weight: f64,
    /// Weight for compliance risk factor (0.0-1.0)
    pub compliance_weight: f64,
    /// High risk threshold (0.0-1.0)
    pub high_risk_threshold: f64,
    /// Medium risk threshold (0.0-1.0)
    pub medium_risk_threshold: f64,
}

impl Default for RiskScoringConfig {
    fn default() -> Self {
        Self {
            override_weight: 0.25,
            volume_weight: 0.20,
            temporal_weight: 0.15,
            pattern_weight: 0.20,
            compliance_weight: 0.20,
            high_risk_threshold: 0.7,
            medium_risk_threshold: 0.4,
        }
    }
}

/// Risk assessment for a decision or set of decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk score (0.0-1.0)
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Individual risk factors
    pub risk_factors: Vec<RiskFactorScore>,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
    /// Mitigation recommendations
    pub recommendations: Vec<String>,
    /// Associated records
    pub record_ids: Vec<Uuid>,
}

/// Risk level categorization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Individual risk factor with score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorScore {
    /// Factor name
    pub factor_name: String,
    /// Factor category
    pub category: RiskCategory,
    /// Score for this factor (0.0-1.0)
    pub score: f64,
    /// Weight applied to this factor
    pub weight: f64,
    /// Weighted contribution to overall score
    pub contribution: f64,
    /// Factor details
    pub details: HashMap<String, String>,
}

/// Risk category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskCategory {
    Override,
    Volume,
    Temporal,
    Pattern,
    Compliance,
    Integrity,
    Custom(String),
}

/// Actor risk profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRiskProfile {
    /// Actor identifier
    pub actor_id: String,
    /// Overall risk score
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Risk factors
    pub risk_factors: Vec<RiskFactorScore>,
    /// Total decisions analyzed
    pub total_decisions: usize,
    /// High-risk decisions count
    pub high_risk_decisions: usize,
    /// Profile generated at
    pub generated_at: DateTime<Utc>,
}

/// Statute risk profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteRiskProfile {
    /// Statute identifier
    pub statute_id: String,
    /// Overall risk score
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Risk factors
    pub risk_factors: Vec<RiskFactorScore>,
    /// Total applications
    pub total_applications: usize,
    /// Failure rate
    pub failure_rate: f64,
    /// Override rate
    pub override_rate: f64,
    /// Profile generated at
    pub generated_at: DateTime<Utc>,
}

/// Risk scoring engine.
pub struct RiskScorer {
    config: RiskScoringConfig,
    baseline_stats: Option<BaselineStats>,
}

/// Baseline statistics for risk calculation.
#[derive(Debug, Clone)]
struct BaselineStats {
    avg_override_rate: f64,
    avg_daily_volume: f64,
    #[allow(dead_code)]
    peak_hourly_volume: f64,
    #[allow(dead_code)]
    total_records: usize,
}

impl RiskScorer {
    /// Creates a new risk scorer with default configuration.
    pub fn new() -> Self {
        Self::with_config(RiskScoringConfig::default())
    }

    /// Creates a new risk scorer with custom configuration.
    pub fn with_config(config: RiskScoringConfig) -> Self {
        Self {
            config,
            baseline_stats: None,
        }
    }

    /// Establishes baseline statistics from historical data.
    pub fn establish_baseline(&mut self, records: &[AuditRecord]) -> AuditResult<()> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Cannot establish baseline from empty dataset".to_string(),
            ));
        }

        let total_records = records.len();

        // Calculate override rate
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();
        let avg_override_rate = override_count as f64 / total_records as f64;

        // Calculate daily volume
        let mut daily_counts: HashMap<String, usize> = HashMap::new();
        for record in records {
            let date_key = record.timestamp.format("%Y-%m-%d").to_string();
            *daily_counts.entry(date_key).or_insert(0) += 1;
        }
        let avg_daily_volume = if !daily_counts.is_empty() {
            daily_counts.values().sum::<usize>() as f64 / daily_counts.len() as f64
        } else {
            0.0
        };

        // Calculate peak hourly volume
        let mut hourly_counts: HashMap<u32, usize> = HashMap::new();
        for record in records {
            let hour = record.timestamp.time().hour();
            *hourly_counts.entry(hour).or_insert(0) += 1;
        }
        let peak_hourly_volume = *hourly_counts.values().max().unwrap_or(&0) as f64;

        self.baseline_stats = Some(BaselineStats {
            avg_override_rate,
            avg_daily_volume,
            peak_hourly_volume,
            total_records,
        });

        Ok(())
    }

    /// Assesses risk for a set of audit records.
    pub fn assess_risk(&self, records: &[AuditRecord]) -> AuditResult<RiskAssessment> {
        if records.is_empty() {
            return Err(AuditError::InvalidRecord(
                "Cannot assess empty record set".to_string(),
            ));
        }

        let baseline = self.baseline_stats.as_ref().ok_or_else(|| {
            AuditError::InvalidRecord("Baseline must be established before assessment".to_string())
        })?;

        let mut risk_factors = Vec::new();

        // Assess override risk
        let override_risk = self.assess_override_risk(records, baseline);
        risk_factors.push(override_risk);

        // Assess volume risk
        let volume_risk = self.assess_volume_risk(records, baseline);
        risk_factors.push(volume_risk);

        // Assess temporal risk
        let temporal_risk = self.assess_temporal_risk(records, baseline);
        risk_factors.push(temporal_risk);

        // Assess pattern risk
        let pattern_risk = self.assess_pattern_risk(records);
        risk_factors.push(pattern_risk);

        // Assess compliance risk
        let compliance_risk = self.assess_compliance_risk(records);
        risk_factors.push(compliance_risk);

        // Calculate overall risk score
        let risk_score: f64 = risk_factors.iter().map(|rf| rf.contribution).sum();

        // Determine risk level
        let risk_level = if risk_score >= self.config.high_risk_threshold {
            if risk_score >= 0.9 {
                RiskLevel::Critical
            } else {
                RiskLevel::High
            }
        } else if risk_score >= self.config.medium_risk_threshold {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        // Generate recommendations
        let recommendations = self.generate_recommendations(&risk_factors, &risk_level);

        let record_ids = records.iter().map(|r| r.id).collect();

        Ok(RiskAssessment {
            risk_score,
            risk_level,
            risk_factors,
            assessed_at: Utc::now(),
            recommendations,
            record_ids,
        })
    }

    /// Generates actor risk profiles.
    pub fn profile_actor_risks(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<ActorRiskProfile>> {
        let mut actor_records: HashMap<String, Vec<&AuditRecord>> = HashMap::new();

        for record in records {
            let actor_id = Self::get_actor_id(&record.actor);
            actor_records.entry(actor_id).or_default().push(record);
        }

        let mut profiles = Vec::new();

        for (actor_id, actor_recs) in actor_records {
            let owned_recs: Vec<AuditRecord> = actor_recs.iter().map(|&r| r.clone()).collect();
            let assessment = self.assess_risk(&owned_recs)?;

            let high_risk_decisions = owned_recs
                .iter()
                .filter(|r| {
                    // Simplified: consider overrides as high-risk
                    matches!(r.result, DecisionResult::Overridden { .. })
                })
                .count();

            profiles.push(ActorRiskProfile {
                actor_id,
                risk_score: assessment.risk_score,
                risk_level: assessment.risk_level,
                risk_factors: assessment.risk_factors,
                total_decisions: owned_recs.len(),
                high_risk_decisions,
                generated_at: Utc::now(),
            });
        }

        // Sort by risk score descending
        profiles.sort_by(|a, b| b.risk_score.partial_cmp(&a.risk_score).unwrap());

        Ok(profiles)
    }

    /// Generates statute risk profiles.
    pub fn profile_statute_risks(
        &self,
        records: &[AuditRecord],
    ) -> AuditResult<Vec<StatuteRiskProfile>> {
        let mut statute_records: HashMap<String, Vec<&AuditRecord>> = HashMap::new();

        for record in records {
            statute_records
                .entry(record.statute_id.clone())
                .or_default()
                .push(record);
        }

        let mut profiles = Vec::new();

        for (statute_id, statute_recs) in statute_records {
            let owned_recs: Vec<AuditRecord> = statute_recs.iter().map(|&r| r.clone()).collect();
            let assessment = self.assess_risk(&owned_recs)?;

            let total = owned_recs.len();
            let override_count = owned_recs
                .iter()
                .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
                .count();
            let override_rate = override_count as f64 / total as f64;

            let void_count = owned_recs
                .iter()
                .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
                .count();
            let failure_rate = void_count as f64 / total as f64;

            profiles.push(StatuteRiskProfile {
                statute_id,
                risk_score: assessment.risk_score,
                risk_level: assessment.risk_level,
                risk_factors: assessment.risk_factors,
                total_applications: total,
                failure_rate,
                override_rate,
                generated_at: Utc::now(),
            });
        }

        // Sort by risk score descending
        profiles.sort_by(|a, b| b.risk_score.partial_cmp(&a.risk_score).unwrap());

        Ok(profiles)
    }

    /// Assesses override risk.
    fn assess_override_risk(
        &self,
        records: &[AuditRecord],
        baseline: &BaselineStats,
    ) -> RiskFactorScore {
        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();

        let current_override_rate = if !records.is_empty() {
            override_count as f64 / records.len() as f64
        } else {
            0.0
        };

        // Score based on deviation from baseline
        let score = if baseline.avg_override_rate > 0.0 {
            (current_override_rate / baseline.avg_override_rate).min(1.0)
        } else if current_override_rate > 0.0 {
            1.0
        } else {
            0.0
        };

        let contribution = score * self.config.override_weight;

        let mut details = HashMap::new();
        details.insert(
            "current_rate".to_string(),
            format!("{:.2}%", current_override_rate * 100.0),
        );
        details.insert(
            "baseline_rate".to_string(),
            format!("{:.2}%", baseline.avg_override_rate * 100.0),
        );
        details.insert("override_count".to_string(), override_count.to_string());

        RiskFactorScore {
            factor_name: "Override Risk".to_string(),
            category: RiskCategory::Override,
            score,
            weight: self.config.override_weight,
            contribution,
            details,
        }
    }

    /// Assesses volume risk.
    fn assess_volume_risk(
        &self,
        records: &[AuditRecord],
        baseline: &BaselineStats,
    ) -> RiskFactorScore {
        let current_volume = records.len() as f64;

        // Score based on deviation from baseline
        let score = if baseline.avg_daily_volume > 0.0 {
            (current_volume / baseline.avg_daily_volume / 3.0).min(1.0) // 3x baseline = max score
        } else if current_volume > 0.0 {
            0.5
        } else {
            0.0
        };

        let contribution = score * self.config.volume_weight;

        let mut details = HashMap::new();
        details.insert("current_volume".to_string(), current_volume.to_string());
        details.insert(
            "baseline_volume".to_string(),
            baseline.avg_daily_volume.to_string(),
        );

        RiskFactorScore {
            factor_name: "Volume Risk".to_string(),
            category: RiskCategory::Volume,
            score,
            weight: self.config.volume_weight,
            contribution,
            details,
        }
    }

    /// Assesses temporal risk (unusual times).
    fn assess_temporal_risk(
        &self,
        records: &[AuditRecord],
        _baseline: &BaselineStats,
    ) -> RiskFactorScore {
        let unusual_time_count = records
            .iter()
            .filter(|r| {
                let hour = r.timestamp.time().hour();
                !(6..22).contains(&hour) // Night time
            })
            .count();

        let score = if !records.is_empty() {
            (unusual_time_count as f64 / records.len() as f64).min(1.0)
        } else {
            0.0
        };

        let contribution = score * self.config.temporal_weight;

        let mut details = HashMap::new();
        details.insert(
            "unusual_time_count".to_string(),
            unusual_time_count.to_string(),
        );
        details.insert("percentage".to_string(), format!("{:.2}%", score * 100.0));

        RiskFactorScore {
            factor_name: "Temporal Risk".to_string(),
            category: RiskCategory::Temporal,
            score,
            weight: self.config.temporal_weight,
            contribution,
            details,
        }
    }

    /// Assesses pattern risk.
    fn assess_pattern_risk(&self, records: &[AuditRecord]) -> RiskFactorScore {
        // Check for rapid sequences
        let mut sorted_records: Vec<_> = records.iter().collect();
        sorted_records.sort_by_key(|r| r.timestamp);

        let mut rapid_count = 0;
        for window in sorted_records.windows(2) {
            let time_diff = (window[1].timestamp - window[0].timestamp).num_seconds();
            if time_diff < 5 {
                rapid_count += 1;
            }
        }

        let score = if records.len() > 1 {
            (rapid_count as f64 / (records.len() - 1) as f64).min(1.0)
        } else {
            0.0
        };

        let contribution = score * self.config.pattern_weight;

        let mut details = HashMap::new();
        details.insert("rapid_sequences".to_string(), rapid_count.to_string());

        RiskFactorScore {
            factor_name: "Pattern Risk".to_string(),
            category: RiskCategory::Pattern,
            score,
            weight: self.config.pattern_weight,
            contribution,
            details,
        }
    }

    /// Assesses compliance risk.
    fn assess_compliance_risk(&self, records: &[AuditRecord]) -> RiskFactorScore {
        let discretionary_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();

        let void_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
            .count();

        let score = if !records.is_empty() {
            ((discretionary_count + void_count * 2) as f64 / records.len() as f64).min(1.0)
        } else {
            0.0
        };

        let contribution = score * self.config.compliance_weight;

        let mut details = HashMap::new();
        details.insert(
            "discretionary_count".to_string(),
            discretionary_count.to_string(),
        );
        details.insert("void_count".to_string(), void_count.to_string());

        RiskFactorScore {
            factor_name: "Compliance Risk".to_string(),
            category: RiskCategory::Compliance,
            score,
            weight: self.config.compliance_weight,
            contribution,
            details,
        }
    }

    /// Generates recommendations based on risk factors.
    fn generate_recommendations(
        &self,
        risk_factors: &[RiskFactorScore],
        risk_level: &RiskLevel,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        match risk_level {
            RiskLevel::Critical | RiskLevel::High => {
                recommendations.push("Immediate review required".to_string());
                recommendations.push("Escalate to compliance team".to_string());
            }
            RiskLevel::Medium => {
                recommendations.push("Schedule review within 24 hours".to_string());
            }
            RiskLevel::Low => {
                recommendations.push("Continue monitoring".to_string());
            }
        }

        for factor in risk_factors {
            if factor.score > 0.7 {
                match factor.category {
                    RiskCategory::Override => {
                        recommendations
                            .push("Review override justifications and patterns".to_string());
                    }
                    RiskCategory::Volume => {
                        recommendations.push("Investigate cause of volume anomaly".to_string());
                    }
                    RiskCategory::Temporal => {
                        recommendations.push("Review off-hours activity authorization".to_string());
                    }
                    RiskCategory::Pattern => {
                        recommendations.push("Analyze decision sequence patterns".to_string());
                    }
                    RiskCategory::Compliance => {
                        recommendations.push("Review compliance procedures".to_string());
                    }
                    _ => {}
                }
            }
        }

        recommendations.dedup();
        recommendations
    }

    /// Extracts actor ID from Actor enum.
    fn get_actor_id(actor: &Actor) -> String {
        match actor {
            Actor::System { component } => format!("system:{}", component),
            Actor::User { user_id, .. } => format!("user:{}", user_id),
            Actor::External { system_id } => format!("external:{}", system_id),
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &RiskScoringConfig {
        &self.config
    }

    /// Returns true if baseline has been established.
    pub fn has_baseline(&self) -> bool {
        self.baseline_stats.is_some()
    }
}

impl Default for RiskScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, EventType};
    use chrono::Duration;
    use std::collections::HashMap;

    fn create_test_record(hours_ago: i64, is_override: bool) -> AuditRecord {
        let timestamp = Utc::now() - Duration::hours(hours_ago);

        let result = if is_override {
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "denied".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "test override".to_string(),
            }
        } else {
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            }
        };

        AuditRecord {
            id: Uuid::new_v4(),
            timestamp,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test-statute".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_risk_scorer_creation() {
        let scorer = RiskScorer::new();
        assert!(!scorer.has_baseline());
        assert_eq!(scorer.config().override_weight, 0.25);
    }

    #[test]
    fn test_establish_baseline() {
        let mut scorer = RiskScorer::new();

        let records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();

        let result = scorer.establish_baseline(&records);
        assert!(result.is_ok());
        assert!(scorer.has_baseline());
    }

    #[test]
    fn test_assess_risk() {
        let mut scorer = RiskScorer::new();

        // Establish baseline with normal data
        let baseline_records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();
        scorer.establish_baseline(&baseline_records).unwrap();

        // Assess risk with some overrides
        let test_records: Vec<_> = (0..10).map(|i| create_test_record(i, i % 2 == 0)).collect();

        let assessment = scorer.assess_risk(&test_records).unwrap();
        assert!(assessment.risk_score >= 0.0 && assessment.risk_score <= 1.0);
        assert!(!assessment.risk_factors.is_empty());
        assert!(!assessment.recommendations.is_empty());
    }

    #[test]
    fn test_profile_actor_risks() {
        let mut scorer = RiskScorer::new();

        let baseline_records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();
        scorer.establish_baseline(&baseline_records).unwrap();

        let test_records: Vec<_> = (0..20).map(|i| create_test_record(i, i % 3 == 0)).collect();

        let profiles = scorer.profile_actor_risks(&test_records).unwrap();
        assert!(!profiles.is_empty());
        assert_eq!(profiles[0].total_decisions, 20);
    }

    #[test]
    fn test_profile_statute_risks() {
        let mut scorer = RiskScorer::new();

        let baseline_records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();
        scorer.establish_baseline(&baseline_records).unwrap();

        let test_records: Vec<_> = (0..20).map(|i| create_test_record(i, i % 4 == 0)).collect();

        let profiles = scorer.profile_statute_risks(&test_records).unwrap();
        assert!(!profiles.is_empty());
        assert_eq!(profiles[0].total_applications, 20);
    }

    #[test]
    fn test_risk_level_categorization() {
        // Use custom config with lower thresholds so override risk alone can trigger Medium
        let config = RiskScoringConfig {
            medium_risk_threshold: 0.2,
            high_risk_threshold: 0.5,
            ..Default::default()
        };
        let mut scorer = RiskScorer::with_config(config);

        let baseline_records: Vec<_> = (0..100).map(|i| create_test_record(i, false)).collect();
        scorer.establish_baseline(&baseline_records).unwrap();

        // Low risk (normal data)
        let low_risk_records: Vec<_> = (0..10).map(|i| create_test_record(i, false)).collect();
        let assessment = scorer.assess_risk(&low_risk_records).unwrap();
        assert_eq!(assessment.risk_level, RiskLevel::Low);

        // High risk (many overrides - 100% override rate with 0% baseline triggers max override score)
        let high_risk_records: Vec<_> = (0..10).map(|i| create_test_record(i, true)).collect();
        let assessment = scorer.assess_risk(&high_risk_records).unwrap();
        assert!(matches!(
            assessment.risk_level,
            RiskLevel::High | RiskLevel::Critical | RiskLevel::Medium
        ));
    }

    #[test]
    fn test_assessment_without_baseline() {
        let scorer = RiskScorer::new();
        let records: Vec<_> = (0..10).map(|i| create_test_record(i, false)).collect();

        let result = scorer.assess_risk(&records);
        assert!(result.is_err());
    }
}
