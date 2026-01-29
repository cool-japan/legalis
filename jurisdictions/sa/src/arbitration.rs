//! Saudi Arbitration Law (نظام التحكيم)
//!
//! Royal Decree No. M/34 dated 24/5/1433H (2012)
//!
//! Based on UNCITRAL Model Law. Governs domestic and international
//! commercial arbitration in Saudi Arabia.

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for arbitration operations
pub type ArbitrationResult<T> = Result<T, ArbitrationError>;

/// Arbitration errors
#[derive(Debug, Error)]
pub enum ArbitrationError {
    /// Invalid arbitration agreement
    #[error("اتفاقية تحكيم غير صالحة: {reason}")]
    InvalidAgreement { reason: String },

    /// Procedural error
    #[error("خطأ إجرائي: {description}")]
    ProceduralError { description: String },
}

/// Types of arbitration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArbitrationType {
    /// Ad hoc arbitration (تحكيم حر)
    AdHoc,
    /// Institutional arbitration (تحكيم مؤسسي)
    Institutional,
    /// Fast-track arbitration (تحكيم مستعجل)
    FastTrack,
}

impl ArbitrationType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::AdHoc => "تحكيم حر",
            Self::Institutional => "تحكيم مؤسسي",
            Self::FastTrack => "تحكيم مستعجل",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::AdHoc => "Ad Hoc Arbitration",
            Self::Institutional => "Institutional Arbitration",
            Self::FastTrack => "Fast-Track Arbitration",
        }
    }
}

/// Dispute resolution methods
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisputeResolution {
    /// Arbitration (تحكيم)
    Arbitration,
    /// Mediation (وساطة)
    Mediation,
    /// Conciliation (توفيق)
    Conciliation,
    /// Litigation (تقاضي)
    Litigation,
}

impl DisputeResolution {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Arbitration => "تحكيم",
            Self::Mediation => "وساطة",
            Self::Conciliation => "توفيق",
            Self::Litigation => "تقاضي",
        }
    }

    /// Check if binding
    pub fn is_binding(&self) -> bool {
        matches!(self, Self::Arbitration | Self::Litigation)
    }

    /// Check if confidential
    pub fn is_confidential(&self) -> bool {
        matches!(
            self,
            Self::Arbitration | Self::Mediation | Self::Conciliation
        )
    }
}

/// Arbitration agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationAgreement {
    /// Arbitration type
    pub arbitration_type: ArbitrationType,
    /// Number of arbitrators (1 or 3 typically)
    pub number_of_arbitrators: u32,
    /// Seat of arbitration
    pub seat: String,
    /// Applicable law
    pub applicable_law: String,
    /// Language
    pub language: String,
    /// Institutional rules (if institutional)
    pub institutional_rules: Option<String>,
}

impl ArbitrationAgreement {
    /// Create new arbitration agreement
    pub fn new(
        arbitration_type: ArbitrationType,
        number_of_arbitrators: u32,
        seat: impl Into<String>,
    ) -> ArbitrationResult<Self> {
        // Number of arbitrators must be odd
        if number_of_arbitrators.is_multiple_of(2) {
            return Err(ArbitrationError::InvalidAgreement {
                reason: "Number of arbitrators must be odd (1, 3, 5, etc.)".to_string(),
            });
        }

        Ok(Self {
            arbitration_type,
            number_of_arbitrators,
            seat: seat.into(),
            applicable_law: "Saudi Arabia".to_string(),
            language: "Arabic".to_string(),
            institutional_rules: None,
        })
    }

    /// Set applicable law
    pub fn with_applicable_law(mut self, law: impl Into<String>) -> Self {
        self.applicable_law = law.into();
        self
    }

    /// Set institutional rules
    pub fn with_institutional_rules(mut self, rules: impl Into<String>) -> Self {
        self.institutional_rules = Some(rules.into());
        self
    }

    /// Validate agreement
    pub fn validate(&self) -> ArbitrationResult<()> {
        if self.seat.is_empty() {
            return Err(ArbitrationError::InvalidAgreement {
                reason: "Seat of arbitration must be specified".to_string(),
            });
        }

        if self.arbitration_type == ArbitrationType::Institutional
            && self.institutional_rules.is_none()
        {
            return Err(ArbitrationError::InvalidAgreement {
                reason: "Institutional rules must be specified for institutional arbitration"
                    .to_string(),
            });
        }

        Ok(())
    }
}

/// Calculate arbitration costs estimate
pub fn estimate_arbitration_costs(dispute_amount: Sar, arbitrators_count: u32) -> Sar {
    // Simplified cost calculation
    // Typical costs: 1-5% of dispute amount plus arbitrator fees
    let admin_fee_pct = 0.02; // 2%
    let admin_fee = Sar::from_halalas((dispute_amount.halalas() as f64 * admin_fee_pct) as i64);

    let arbitrator_fee_per_day = Sar::from_riyals(5_000);
    let estimated_days = 10u32;
    let arbitrator_fees =
        arbitrator_fee_per_day * (arbitrators_count as i64) * (estimated_days as i64);

    admin_fee + arbitrator_fees
}

/// Get arbitration checklist
pub fn get_arbitration_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("اتفاقية التحكيم", "Valid arbitration agreement"),
        ("تحديد المحكمين", "Appoint arbitrators"),
        ("تحديد مقر التحكيم", "Determine seat of arbitration"),
        ("تحديد القانون الواجب التطبيق", "Specify applicable law"),
        ("لائحة الدعوى", "Statement of claim"),
        ("لائحة الدفاع", "Statement of defense"),
        ("جلسات الاستماع", "Hearings"),
        ("تقديم الأدلة", "Evidence submission"),
        ("حكم التحكيم", "Arbitral award"),
        ("تنفيذ الحكم", "Enforcement of award"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitration_types() {
        assert_eq!(ArbitrationType::AdHoc.name_ar(), "تحكيم حر");
        assert_eq!(
            ArbitrationType::Institutional.name_en(),
            "Institutional Arbitration"
        );
    }

    #[test]
    fn test_dispute_resolution() {
        assert!(DisputeResolution::Arbitration.is_binding());
        assert!(DisputeResolution::Arbitration.is_confidential());
        assert!(!DisputeResolution::Mediation.is_binding());
    }

    #[test]
    fn test_valid_agreement() {
        let agreement = ArbitrationAgreement::new(
            ArbitrationType::AdHoc,
            3, // Odd number
            "Riyadh",
        );
        assert!(agreement.is_ok());
    }

    #[test]
    fn test_invalid_even_arbitrators() {
        let agreement = ArbitrationAgreement::new(
            ArbitrationType::AdHoc,
            2, // Even number - invalid
            "Riyadh",
        );
        assert!(agreement.is_err());
    }

    #[test]
    fn test_institutional_requires_rules() {
        let mut agreement =
            ArbitrationAgreement::new(ArbitrationType::Institutional, 3, "Riyadh").unwrap();

        assert!(agreement.validate().is_err());

        agreement = agreement.with_institutional_rules("SCCA Rules");
        assert!(agreement.validate().is_ok());
    }

    #[test]
    fn test_cost_estimation() {
        let costs = estimate_arbitration_costs(Sar::from_riyals(1_000_000), 3);
        assert!(costs.riyals() > 0);
    }

    #[test]
    fn test_checklist() {
        let checklist = get_arbitration_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
