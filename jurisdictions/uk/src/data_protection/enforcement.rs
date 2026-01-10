//! ICO Enforcement Powers
//!
//! Information Commissioner's Office enforcement under DPA 2018 Part 6

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ICO enforcement action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IcoEnforcement {
    /// Information Notice (DPA 2018 s.142)
    /// ICO requires controller/processor to provide information
    InformationNotice {
        /// Deadline in days to respond
        deadline_days: u32,
        /// Information required
        information_required: Vec<String>,
        /// Date issued
        issued_date: DateTime<Utc>,
    },

    /// Enforcement Notice (DPA 2018 s.149)
    /// ICO requires controller/processor to take steps to comply
    EnforcementNotice {
        /// Required actions
        required_actions: Vec<String>,
        /// Deadline to comply
        deadline: DateTime<Utc>,
        /// Date issued
        issued_date: DateTime<Utc>,
    },

    /// Assessment Notice (DPA 2018 s.146)
    /// ICO inspects processing operations
    AssessmentNotice {
        /// Scope of assessment
        scope: String,
        /// Date of inspection
        inspection_date: DateTime<Utc>,
    },

    /// Monetary Penalty (DPA 2018 s.155, UK GDPR Article 83)
    MonetaryPenalty {
        /// Penalty amount in GBP
        amount_gbp: f64,
        /// Reason for penalty
        reason: String,
        /// Article 83 tier (4 or 5)
        article_83_tier: u8,
        /// Date issued
        issued_date: DateTime<Utc>,
    },

    /// Prosecution (DPA 2018 s.196-197)
    /// Criminal offences under DPA 2018
    Prosecution {
        /// Offence under DPA 2018
        offence: Dpa2018Offence,
        /// Description
        description: String,
    },
}

/// DPA 2018 criminal offences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dpa2018Offence {
    /// s.170: Unlawful obtaining of personal data
    UnlawfulObtaining,

    /// s.171: Re-identification of de-identified data
    ReIdentification,

    /// s.173: Alteration etc. of personal data to prevent disclosure
    AlterationToPreventDisclosure,

    /// s.196: Obstruction of ICO
    ObstructionOfIco,
}

/// ICO enforcement type (for error reporting)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IcoEnforcementType {
    /// Information notice required
    InformationNotice,
    /// Enforcement notice required
    EnforcementNotice,
    /// Assessment notice
    AssessmentNotice,
    /// Monetary penalty
    MonetaryPenalty,
    /// Criminal prosecution
    Prosecution,
}

impl IcoEnforcement {
    /// Create information notice
    pub fn information_notice(deadline_days: u32, information_required: Vec<String>) -> Self {
        Self::InformationNotice {
            deadline_days,
            information_required,
            issued_date: Utc::now(),
        }
    }

    /// Create enforcement notice
    pub fn enforcement_notice(required_actions: Vec<String>, deadline: DateTime<Utc>) -> Self {
        Self::EnforcementNotice {
            required_actions,
            deadline,
            issued_date: Utc::now(),
        }
    }

    /// Create monetary penalty
    pub fn monetary_penalty(amount_gbp: f64, reason: String, article_83_tier: u8) -> Self {
        Self::MonetaryPenalty {
            amount_gbp,
            reason,
            article_83_tier,
            issued_date: Utc::now(),
        }
    }

    /// Get enforcement type
    pub fn enforcement_type(&self) -> IcoEnforcementType {
        match self {
            Self::InformationNotice { .. } => IcoEnforcementType::InformationNotice,
            Self::EnforcementNotice { .. } => IcoEnforcementType::EnforcementNotice,
            Self::AssessmentNotice { .. } => IcoEnforcementType::AssessmentNotice,
            Self::MonetaryPenalty { .. } => IcoEnforcementType::MonetaryPenalty,
            Self::Prosecution { .. } => IcoEnforcementType::Prosecution,
        }
    }
}

/// Calculate ICO fine under UK GDPR Article 83
///
/// Two tiers:
/// - Tier 1 (Article 83(4)): Up to £8.7m or 2% of global turnover
/// - Tier 2 (Article 83(5)): Up to £17.5m or 4% of global turnover
pub fn calculate_ico_fine(
    infringement_type: Article83Tier,
    global_turnover_gbp: f64,
    aggravating_factors: &[String],
    mitigating_factors: &[String],
) -> f64 {
    // Base maximum
    let max_fixed: f64 = match infringement_type {
        Article83Tier::Tier4 => 8_700_000.0,  // £8.7m
        Article83Tier::Tier5 => 17_500_000.0, // £17.5m
    };

    let max_percentage: f64 = match infringement_type {
        Article83Tier::Tier4 => global_turnover_gbp * 0.02, // 2%
        Article83Tier::Tier5 => global_turnover_gbp * 0.04, // 4%
    };

    // Statutory maximum is higher of the two
    let statutory_max = max_fixed.max(max_percentage);

    // Apply factors (simplified - ICO considers many factors)
    let aggravating_multiplier = 1.0 + (aggravating_factors.len() as f64 * 0.1);
    let mitigating_multiplier = 1.0 - (mitigating_factors.len() as f64 * 0.1).min(0.5);

    let base_fine = statutory_max * 0.1; // Start at 10% of max
    let adjusted = base_fine * aggravating_multiplier * mitigating_multiplier;

    adjusted.min(statutory_max)
}

/// Article 83 fine tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Article83Tier {
    /// Tier 4: Lower tier (Article 83(4))
    /// Up to £8.7m or 2% of global turnover
    Tier4,

    /// Tier 5: Higher tier (Article 83(5))
    /// Up to £17.5m or 4% of global turnover
    Tier5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ico_fine_calculation_tier4() {
        let fine = calculate_ico_fine(
            Article83Tier::Tier4,
            100_000_000.0, // £100m turnover
            &["Intentional".to_string()],
            &[],
        );

        // 2% of £100m = £2m (less than £8.7m fixed, so use £2m)
        // Base 10% = £200k, aggravating +10% = £220k
        assert!(fine > 0.0);
        assert!(fine <= 2_000_000.0);
    }

    #[test]
    fn test_ico_fine_calculation_tier5() {
        let fine = calculate_ico_fine(
            Article83Tier::Tier5,
            500_000_000.0, // £500m turnover
            &[],
            &["Cooperation".to_string(), "First offence".to_string()],
        );

        // 4% of £500m = £20m (higher than £17.5m, so cap at £17.5m)
        assert!(fine > 0.0);
        assert!(fine <= 17_500_000.0);
    }
}
