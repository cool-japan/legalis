//! UAE Arbitration Law - Federal Decree-Law No. 6/2018
//!
//! Governing domestic and international arbitration in the UAE.
//!
//! ## Key Features
//!
//! - Based on UNCITRAL Model Law
//! - Supports both domestic and international arbitration
//! - Arbitration agreement must be in writing
//! - Limited grounds for challenging awards
//! - Enforcement under New York Convention
//!
//! ## Arbitration Centres
//!
//! - **DIAC** - Dubai International Arbitration Centre
//! - **ADCCAC** - Abu Dhabi Commercial Conciliation and Arbitration Centre
//! - **DIFC-LCIA** - DIFC-London Court of International Arbitration
//! - **ADGM Arbitration Centre**

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for arbitration operations
pub type ArbitrationResult<T> = Result<T, ArbitrationError>;

/// Types of arbitration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArbitrationType {
    /// Domestic arbitration (تحكيم داخلي)
    Domestic,
    /// International arbitration (تحكيم دولي)
    International,
    /// Ad hoc arbitration (تحكيم خاص)
    AdHoc,
    /// Institutional arbitration (تحكيم مؤسسي)
    Institutional,
}

impl ArbitrationType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Domestic => "تحكيم داخلي",
            Self::International => "تحكيم دولي",
            Self::AdHoc => "تحكيم خاص",
            Self::Institutional => "تحكيم مؤسسي",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Domestic => "Domestic Arbitration",
            Self::International => "International Arbitration",
            Self::AdHoc => "Ad Hoc Arbitration",
            Self::Institutional => "Institutional Arbitration",
        }
    }
}

/// Arbitration agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationAgreement {
    /// Agreement in writing (required)
    pub in_writing: bool,
    /// Dispute subject matter
    pub subject_matter: String,
    /// Number of arbitrators (1 or 3 typically)
    pub number_of_arbitrators: u32,
    /// Seat of arbitration
    pub seat: String,
    /// Governing law
    pub governing_law: String,
    /// Language of arbitration
    pub language: String,
    /// Arbitration institution (if institutional)
    pub institution: Option<ArbitrationInstitution>,
}

impl ArbitrationAgreement {
    /// Validate arbitration agreement
    pub fn is_valid(&self) -> ArbitrationResult<()> {
        if !self.in_writing {
            return Err(ArbitrationError::WritingRequired);
        }

        if self.number_of_arbitrators == 0 || self.number_of_arbitrators > 3 {
            return Err(ArbitrationError::InvalidArbitratorCount {
                count: self.number_of_arbitrators,
            });
        }

        // Odd number of arbitrators required (1 or 3)
        if self.number_of_arbitrators.is_multiple_of(2) {
            return Err(ArbitrationError::InvalidArbitratorCount {
                count: self.number_of_arbitrators,
            });
        }

        Ok(())
    }
}

/// UAE arbitration institutions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArbitrationInstitution {
    /// Dubai International Arbitration Centre
    Diac,
    /// Abu Dhabi Commercial Conciliation and Arbitration Centre
    Adccac,
    /// DIFC-LCIA Arbitration Centre
    DifcLcia,
    /// ADGM Arbitration Centre
    AdgmArbitration,
    /// ICC International Court of Arbitration
    Icc,
    /// LCIA (London)
    Lcia,
    /// Other institution
    Other(String),
}

impl ArbitrationInstitution {
    /// Get institution name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Diac => "Dubai International Arbitration Centre (DIAC)",
            Self::Adccac => "Abu Dhabi Commercial Conciliation and Arbitration Centre (ADCCAC)",
            Self::DifcLcia => "DIFC-LCIA Arbitration Centre",
            Self::AdgmArbitration => "ADGM Arbitration Centre",
            Self::Icc => "ICC International Court of Arbitration",
            Self::Lcia => "LCIA (London)",
            Self::Other(_) => "Other Institution",
        }
    }

    /// Check if UAE-based institution
    pub fn is_uae_based(&self) -> bool {
        matches!(
            self,
            Self::Diac | Self::Adccac | Self::DifcLcia | Self::AdgmArbitration
        )
    }

    /// Get typical administrative fee (percentage of claim amount)
    pub fn admin_fee_percentage(&self) -> f64 {
        match self {
            Self::Diac | Self::Adccac => 3.0,
            Self::DifcLcia | Self::AdgmArbitration => 2.5,
            Self::Icc => 4.0,
            Self::Lcia => 3.5,
            Self::Other(_) => 3.0,
        }
    }
}

/// Grounds for challenging arbitral award - Article 53
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChallengeGround {
    /// No valid arbitration agreement
    NoAgreement,
    /// Party lacked capacity
    LackOfCapacity,
    /// Party not properly notified
    ImproperNotification,
    /// Award deals with matter outside scope
    ExceedsScope,
    /// Composition of tribunal not in accordance with agreement
    TribunalComposition,
    /// Award contrary to public policy
    PublicPolicy,
    /// Subject matter not arbitrable
    NotArbitrable,
}

impl ChallengeGround {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::NoAgreement => "عدم وجود اتفاق تحكيم",
            Self::LackOfCapacity => "انعدام الأهلية",
            Self::ImproperNotification => "عدم الإخطار الصحيح",
            Self::ExceedsScope => "تجاوز نطاق التحكيم",
            Self::TribunalComposition => "تشكيل هيئة التحكيم",
            Self::PublicPolicy => "مخالفة النظام العام",
            Self::NotArbitrable => "غير قابل للتحكيم",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::NoAgreement => "No Valid Arbitration Agreement",
            Self::LackOfCapacity => "Lack of Capacity",
            Self::ImproperNotification => "Improper Notification",
            Self::ExceedsScope => "Exceeds Scope of Arbitration",
            Self::TribunalComposition => "Tribunal Composition",
            Self::PublicPolicy => "Contrary to Public Policy",
            Self::NotArbitrable => "Not Arbitrable",
        }
    }

    /// Article reference in UAE Arbitration Law
    pub fn article_reference(&self) -> &'static str {
        "Article 53"
    }
}

/// Arbitration costs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationCosts {
    /// Claim amount
    pub claim_amount: Aed,
    /// Arbitrators' fees
    pub arbitrator_fees: Aed,
    /// Administrative fees (institution)
    pub admin_fees: Aed,
    /// Legal representation fees
    pub legal_fees: Option<Aed>,
    /// Venue and hearing costs
    pub venue_costs: Aed,
    /// Total costs
    pub total: Aed,
}

impl ArbitrationCosts {
    /// Estimate costs for institutional arbitration
    pub fn estimate(
        claim_amount: Aed,
        institution: &ArbitrationInstitution,
        number_of_arbitrators: u32,
    ) -> Self {
        // Administrative fees (percentage of claim)
        let admin_percentage = institution.admin_fee_percentage();
        let admin_fees = Aed::from_fils(claim_amount.fils() * (admin_percentage as i64) / 100);

        // Arbitrator fees (approximately 1-3% of claim, depends on amount)
        let arbitrator_fee_per_arbitrator = Aed::from_fils(claim_amount.fils() * 15 / 1000); // 1.5% per arbitrator
        let arbitrator_fees =
            Aed::from_fils(arbitrator_fee_per_arbitrator.fils() * number_of_arbitrators as i64);

        // Venue costs (estimated)
        let venue_costs = Aed::from_dirhams(50_000);

        let total = admin_fees + arbitrator_fees + venue_costs;

        Self {
            claim_amount,
            arbitrator_fees,
            admin_fees,
            legal_fees: None,
            venue_costs,
            total,
        }
    }
}

/// Arbitration timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationTimeline {
    /// Notice of arbitration filed
    pub notice_filed: bool,
    /// Tribunal constituted
    pub tribunal_constituted: bool,
    /// Preliminary hearing held
    pub preliminary_hearing: bool,
    /// Final hearing held
    pub final_hearing: bool,
    /// Award rendered
    pub award_rendered: bool,
    /// Days since commencement
    pub days_since_commencement: u32,
}

impl ArbitrationTimeline {
    /// Get typical duration for each stage (days)
    pub fn typical_duration_days() -> Vec<(&'static str, u32)> {
        vec![
            ("Notice to Tribunal Constitution", 60),
            ("Tribunal to Preliminary Hearing", 90),
            ("Preliminary to Final Hearing", 120),
            ("Final Hearing to Award", 60),
            ("Total (typical)", 330),
        ]
    }
}

/// Arbitration errors
#[derive(Debug, Error)]
pub enum ArbitrationError {
    /// Writing required for arbitration agreement
    #[error("يجب أن يكون اتفاق التحكيم كتابياً")]
    WritingRequired,

    /// Invalid number of arbitrators
    #[error("عدد المحكمين غير صحيح: {count} (يجب أن يكون 1 أو 3)")]
    InvalidArbitratorCount { count: u32 },

    /// Challenge to award
    #[error("طعن في حكم التحكيم: {ground}")]
    AwardChallenge { ground: String },

    /// Not arbitrable
    #[error("النزاع غير قابل للتحكيم: {reason}")]
    NotArbitrable { reason: String },

    /// Enforcement failed
    #[error("فشل تنفيذ حكم التحكيم: {reason}")]
    EnforcementFailed { reason: String },
}

/// Get arbitration compliance checklist
pub fn get_arbitration_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("اتفاق تحكيم كتابي", "Written arbitration agreement"),
        ("تعيين المحكمين", "Appointment of arbitrators"),
        ("تحديد مقر التحكيم", "Seat of arbitration"),
        ("القانون الواجب التطبيق", "Applicable law"),
        ("لغة التحكيم", "Language of arbitration"),
        ("إجراءات التحكيم", "Arbitration procedure"),
        ("تبادل المذكرات", "Exchange of submissions"),
        ("جلسات الاستماع", "Hearings"),
        ("إصدار الحكم", "Award issuance"),
        ("تنفيذ الحكم", "Award enforcement"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitration_types() {
        let international = ArbitrationType::International;
        assert_eq!(international.name_ar(), "تحكيم دولي");
        assert_eq!(international.name_en(), "International Arbitration");
    }

    #[test]
    fn test_arbitration_agreement_valid() {
        let agreement = ArbitrationAgreement {
            in_writing: true,
            subject_matter: "Commercial dispute".to_string(),
            number_of_arbitrators: 3,
            seat: "Dubai".to_string(),
            governing_law: "UAE Law".to_string(),
            language: "English".to_string(),
            institution: Some(ArbitrationInstitution::Diac),
        };

        assert!(agreement.is_valid().is_ok());
    }

    #[test]
    fn test_arbitration_agreement_invalid_count() {
        let agreement = ArbitrationAgreement {
            in_writing: true,
            subject_matter: "Dispute".to_string(),
            number_of_arbitrators: 2, // Even number not allowed
            seat: "Dubai".to_string(),
            governing_law: "UAE Law".to_string(),
            language: "Arabic".to_string(),
            institution: None,
        };

        assert!(agreement.is_valid().is_err());
    }

    #[test]
    fn test_arbitration_institutions() {
        let diac = ArbitrationInstitution::Diac;
        assert!(diac.is_uae_based());
        assert_eq!(diac.admin_fee_percentage(), 3.0);

        let icc = ArbitrationInstitution::Icc;
        assert!(!icc.is_uae_based());
    }

    #[test]
    fn test_challenge_grounds() {
        let public_policy = ChallengeGround::PublicPolicy;
        assert_eq!(public_policy.name_ar(), "مخالفة النظام العام");
        assert_eq!(public_policy.article_reference(), "Article 53");
    }

    #[test]
    fn test_arbitration_costs() {
        let costs = ArbitrationCosts::estimate(
            Aed::from_dirhams(10_000_000),
            &ArbitrationInstitution::Diac,
            3,
        );

        assert!(costs.total.dirhams() > 0);
        assert!(costs.admin_fees.dirhams() > 0);
        assert!(costs.arbitrator_fees.dirhams() > 0);
    }

    #[test]
    fn test_arbitration_timeline() {
        let durations = ArbitrationTimeline::typical_duration_days();
        assert!(!durations.is_empty());
        assert_eq!(durations.last().unwrap().0, "Total (typical)");
    }

    #[test]
    fn test_arbitration_checklist() {
        let checklist = get_arbitration_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
