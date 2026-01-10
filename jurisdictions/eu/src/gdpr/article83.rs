//! GDPR Article 83 - Administrative Fines
//!
//! Supervisory authorities can impose administrative fines for GDPR violations.
//! Fines must be effective, proportionate, and dissuasive in each case.

use crate::gdpr::error::GdprError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Fine tier under Article 83
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FineTier {
    /// Article 83(4) - Lower tier
    ///
    /// Up to €10,000,000 or 2% of global annual turnover, whichever is higher.
    /// Applies to less severe violations (e.g., Article 8 child consent, processor obligations).
    LowerTier,

    /// Article 83(5)/(6) - Upper tier
    ///
    /// Up to €20,000,000 or 4% of global annual turnover, whichever is higher.
    /// Applies to more severe violations (e.g., Article 6 lawful basis, Article 9 special categories,
    /// data subject rights, cross-border transfers).
    UpperTier,
}

/// Article violated (determines fine tier)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolatedArticle {
    // Upper tier violations (Article 83(5))
    Article5BasicPrinciples,
    Article6LawfulBasis,
    Article7ConsentConditions,
    Article9SpecialCategories,
    DataSubjectRights,    // Articles 12-22
    CrossBorderTransfers, // Articles 44-49

    // Lower tier violations (Article 83(4))
    Article8ChildConsent,
    ProcessorObligations, // Article 28
    CertificationBody,    // Article 42-43
    MonitoringBody,

    // Other violations
    Other { description: String, tier: FineTier },
}

impl ViolatedArticle {
    /// Get the fine tier for this violation
    pub fn tier(&self) -> FineTier {
        match self {
            Self::Article5BasicPrinciples
            | Self::Article6LawfulBasis
            | Self::Article7ConsentConditions
            | Self::Article9SpecialCategories
            | Self::DataSubjectRights
            | Self::CrossBorderTransfers => FineTier::UpperTier,

            Self::Article8ChildConsent
            | Self::ProcessorObligations
            | Self::CertificationBody
            | Self::MonitoringBody => FineTier::LowerTier,

            Self::Other { tier, .. } => *tier,
        }
    }
}

/// Factors under Article 83(2) for determining fine amount
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article83Factors {
    // (a) Nature, gravity, duration
    pub duration_months: Option<u32>,
    pub data_subjects_affected: Option<u64>,
    pub damage_suffered: Option<f64>, // Monetary damage estimate

    // (b) Intentional or negligent
    pub intentional: bool,

    // (c) Actions to mitigate damage
    pub mitigation_actions_taken: Vec<String>,

    // (d) Degree of responsibility (data protection by design/default)
    pub technical_organizational_measures: Vec<String>,

    // (e) Previous relevant violations
    pub previous_violations: Vec<String>,

    // (f) Cooperation with supervisory authority
    pub cooperated_with_authority: bool,

    // (g) Categories of personal data affected
    pub special_categories_involved: bool,

    // (h) Notification compliance (Articles 33/34)
    pub breach_notification_timely: Option<bool>,

    // (i) Compliance with approved codes/certifications
    pub certifications: Vec<String>,

    // (j) Other aggravating/mitigating circumstances
    pub other_aggravating: Vec<String>,
    pub other_mitigating: Vec<String>,

    // (k) Financial benefits gained or losses avoided
    pub financial_benefit_gained: Option<f64>,
}

impl Default for Article83Factors {
    fn default() -> Self {
        Self {
            duration_months: None,
            data_subjects_affected: None,
            damage_suffered: None,
            intentional: false,
            mitigation_actions_taken: Vec::new(),
            technical_organizational_measures: Vec::new(),
            previous_violations: Vec::new(),
            cooperated_with_authority: true,
            special_categories_involved: false,
            breach_notification_timely: None,
            certifications: Vec::new(),
            other_aggravating: Vec::new(),
            other_mitigating: Vec::new(),
            financial_benefit_gained: None,
        }
    }
}

/// Builder for calculating Article 83 administrative fines
#[derive(Debug, Clone)]
pub struct AdministrativeFine {
    pub controller_name: Option<String>,
    pub violation: Option<ViolatedArticle>,
    pub global_annual_turnover_eur: Option<f64>,
    pub factors: Article83Factors,
}

impl AdministrativeFine {
    pub fn new() -> Self {
        Self {
            controller_name: None,
            violation: None,
            global_annual_turnover_eur: None,
            factors: Article83Factors::default(),
        }
    }

    pub fn with_controller(mut self, name: impl Into<String>) -> Self {
        self.controller_name = Some(name.into());
        self
    }

    pub fn with_violation(mut self, violation: ViolatedArticle) -> Self {
        self.violation = Some(violation);
        self
    }

    pub fn with_turnover_eur(mut self, turnover: f64) -> Self {
        self.global_annual_turnover_eur = Some(turnover);
        self
    }

    pub fn with_factors(mut self, factors: Article83Factors) -> Self {
        self.factors = factors;
        self
    }

    /// Calculate maximum fine under Article 83
    pub fn calculate_maximum(&self) -> Result<FineCalculation, GdprError> {
        let violation = self
            .violation
            .as_ref()
            .ok_or_else(|| GdprError::missing_field("violation"))?;

        let tier = violation.tier();

        let (statutory_max_eur, turnover_percentage): (f64, f64) = match tier {
            FineTier::LowerTier => (10_000_000.0, 0.02), // €10M or 2%
            FineTier::UpperTier => (20_000_000.0, 0.04), // €20M or 4%
        };

        // Calculate turnover-based maximum if turnover known
        let turnover_based_max = self
            .global_annual_turnover_eur
            .map(|turnover| turnover * turnover_percentage);

        // Maximum is the higher of statutory or turnover-based
        let maximum_fine_eur = match turnover_based_max {
            Some(turnover_max) => statutory_max_eur.max(turnover_max),
            None => statutory_max_eur,
        };

        // Calculate severity multiplier based on Article 83(2) factors
        let severity_score = self.calculate_severity_score();

        // Suggested fine (illustrative - actual determination by supervisory authority)
        // Range: 0% to 100% of maximum based on severity
        let suggested_fine_eur = maximum_fine_eur * severity_score;

        Ok(FineCalculation {
            tier,
            statutory_maximum_eur: statutory_max_eur,
            turnover_based_maximum_eur: turnover_based_max,
            maximum_fine_eur,
            severity_score,
            suggested_fine_eur,
            factors_summary: self.summarize_factors(),
        })
    }

    /// Calculate severity score (0.0 to 1.0) based on Article 83(2) factors
    fn calculate_severity_score(&self) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Intentional vs negligent (high weight)
        max_score += 20.0;
        if self.factors.intentional {
            score += 20.0;
        } else {
            score += 10.0; // Negligent
        }

        // Data subjects affected (high weight)
        max_score += 15.0;
        if let Some(affected) = self.factors.data_subjects_affected {
            score += match affected {
                0..=100 => 3.0,
                101..=1_000 => 7.0,
                1_001..=10_000 => 11.0,
                _ => 15.0,
            };
        }

        // Special categories involved (high weight)
        max_score += 15.0;
        if self.factors.special_categories_involved {
            score += 15.0;
        }

        // Duration (moderate weight)
        max_score += 10.0;
        if let Some(months) = self.factors.duration_months {
            score += match months {
                0..=3 => 2.0,
                4..=12 => 5.0,
                13..=36 => 8.0,
                _ => 10.0,
            };
        }

        // Previous violations (moderate weight)
        max_score += 10.0;
        score += (self.factors.previous_violations.len() as f64 * 3.0).min(10.0);

        // Financial benefit gained (moderate weight)
        max_score += 10.0;
        if self.factors.financial_benefit_gained.is_some() {
            score += 10.0;
        }

        // Cooperation (mitigating factor)
        max_score += 10.0;
        if self.factors.cooperated_with_authority {
            score += 0.0; // No penalty for cooperation
        } else {
            score += 10.0; // Penalty for non-cooperation
        }

        // Mitigation actions (mitigating factor)
        max_score += 5.0;
        if self.factors.mitigation_actions_taken.is_empty() {
            score += 5.0;
        }

        // Breach notification compliance (if applicable)
        max_score += 5.0;
        match self.factors.breach_notification_timely {
            Some(true) => score += 0.0,
            Some(false) => score += 5.0,
            None => score += 0.0,
        }

        // Normalize to 0.0-1.0
        (score / max_score).clamp(0.0, 1.0)
    }

    /// Summarize factors for reporting
    fn summarize_factors(&self) -> Vec<String> {
        let mut summary = Vec::new();

        if self.factors.intentional {
            summary.push("⚠️ AGGRAVATING: Intentional violation".to_string());
        }

        if let Some(affected) = self.factors.data_subjects_affected {
            summary.push(format!("Data subjects affected: {}", affected));
        }

        if self.factors.special_categories_involved {
            summary.push("⚠️ AGGRAVATING: Special categories involved".to_string());
        }

        if !self.factors.previous_violations.is_empty() {
            summary.push(format!(
                "⚠️ AGGRAVATING: Previous violations ({})",
                self.factors.previous_violations.len()
            ));
        }

        if self.factors.cooperated_with_authority {
            summary.push("✅ MITIGATING: Cooperated with supervisory authority".to_string());
        }

        if !self.factors.mitigation_actions_taken.is_empty() {
            summary.push(format!(
                "✅ MITIGATING: Mitigation actions taken ({})",
                self.factors.mitigation_actions_taken.len()
            ));
        }

        if let Some(false) = self.factors.breach_notification_timely {
            summary.push("⚠️ AGGRAVATING: Late breach notification".to_string());
        }

        if let Some(benefit) = self.factors.financial_benefit_gained {
            summary.push(format!(
                "⚠️ AGGRAVATING: Financial benefit gained: €{:.2}",
                benefit
            ));
        }

        summary
    }
}

impl Default for AdministrativeFine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of fine calculation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FineCalculation {
    pub tier: FineTier,
    pub statutory_maximum_eur: f64,
    pub turnover_based_maximum_eur: Option<f64>,
    pub maximum_fine_eur: f64,
    pub severity_score: f64,
    pub suggested_fine_eur: f64,
    pub factors_summary: Vec<String>,
}

impl FineCalculation {
    /// Format fine amount for display
    pub fn format_amount(&self) -> String {
        if self.suggested_fine_eur >= 1_000_000.0 {
            format!("€{:.2}M", self.suggested_fine_eur / 1_000_000.0)
        } else if self.suggested_fine_eur >= 1_000.0 {
            format!("€{:.2}K", self.suggested_fine_eur / 1_000.0)
        } else {
            format!("€{:.2}", self.suggested_fine_eur)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_tier_statutory_max() {
        let fine = AdministrativeFine::new()
            .with_controller("Small Company")
            .with_violation(ViolatedArticle::Article8ChildConsent);

        let calc = fine.calculate_maximum().unwrap();
        assert_eq!(calc.tier, FineTier::LowerTier);
        assert_eq!(calc.statutory_maximum_eur, 10_000_000.0);
        assert_eq!(calc.maximum_fine_eur, 10_000_000.0);
    }

    #[test]
    fn test_upper_tier_turnover_based() {
        let fine = AdministrativeFine::new()
            .with_controller("Large Corp")
            .with_violation(ViolatedArticle::Article6LawfulBasis)
            .with_turnover_eur(1_000_000_000.0); // €1B turnover

        let calc = fine.calculate_maximum().unwrap();
        assert_eq!(calc.tier, FineTier::UpperTier);
        assert_eq!(calc.turnover_based_maximum_eur, Some(40_000_000.0)); // 4% of €1B
        assert_eq!(calc.maximum_fine_eur, 40_000_000.0); // Higher than €20M statutory
    }

    #[test]
    fn test_severity_score_aggravating_factors() {
        let factors = Article83Factors {
            intentional: true,
            data_subjects_affected: Some(100_000),
            special_categories_involved: true,
            previous_violations: vec!["Previous Article 6 violation".to_string()],
            cooperated_with_authority: false,
            ..Default::default()
        };

        let fine = AdministrativeFine::new()
            .with_controller("Bad Actor Inc")
            .with_violation(ViolatedArticle::Article9SpecialCategories)
            .with_turnover_eur(500_000_000.0)
            .with_factors(factors);

        let calc = fine.calculate_maximum().unwrap();
        assert!(calc.severity_score > 0.6); // High severity
        assert!(calc.suggested_fine_eur > 10_000_000.0);
    }

    #[test]
    fn test_severity_score_mitigating_factors() {
        let factors = Article83Factors {
            intentional: false,
            data_subjects_affected: Some(50),
            cooperated_with_authority: true,
            mitigation_actions_taken: vec![
                "Immediate notification".to_string(),
                "Remediation implemented".to_string(),
            ],
            ..Default::default()
        };

        let fine = AdministrativeFine::new()
            .with_controller("Good Corp")
            .with_violation(ViolatedArticle::Article6LawfulBasis)
            .with_factors(factors);

        let calc = fine.calculate_maximum().unwrap();
        assert!(calc.severity_score < 0.5); // Lower severity due to mitigation
    }

    #[test]
    fn test_fine_tier_determination() {
        assert_eq!(
            ViolatedArticle::Article6LawfulBasis.tier(),
            FineTier::UpperTier
        );
        assert_eq!(
            ViolatedArticle::Article9SpecialCategories.tier(),
            FineTier::UpperTier
        );
        assert_eq!(
            ViolatedArticle::Article8ChildConsent.tier(),
            FineTier::LowerTier
        );
    }
}
