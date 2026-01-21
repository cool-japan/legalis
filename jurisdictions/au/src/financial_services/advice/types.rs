//! Financial Advice Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Advice types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdviceType {
    /// Personal advice (considers personal circumstances)
    Personal,
    /// General advice (does not consider personal circumstances)
    General,
    /// Execution-only (no advice)
    ExecutionOnly,
}

impl AdviceType {
    /// Check if best interests duty applies
    pub fn best_interests_duty_applies(&self) -> bool {
        matches!(self, AdviceType::Personal)
    }

    /// Check if SOA required
    pub fn requires_soa(&self) -> bool {
        matches!(self, AdviceType::Personal)
    }

    /// Check if general advice warning required
    pub fn requires_general_advice_warning(&self) -> bool {
        matches!(self, AdviceType::General)
    }
}

/// Best interests assessment (s.961B)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BestInterestsAssessment {
    /// Client name
    pub client_name: String,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Advice type
    pub advice_type: AdviceType,
    /// Safe harbour steps completed
    pub safe_harbour_steps: Vec<SafeHarbourStep>,
    /// Client objectives identified
    pub objectives_identified: bool,
    /// Financial situation assessed
    pub financial_situation_assessed: bool,
    /// Needs identified
    pub needs_identified: bool,
    /// Product investigation conducted
    pub product_investigation: bool,
    /// Recommendation appropriate
    pub recommendation_appropriate: bool,
    /// Priority given to client interests
    pub client_priority: bool,
    /// Overall compliance
    pub compliant: bool,
    /// Non-compliance details
    pub non_compliance_details: Option<String>,
}

/// Safe harbour steps (s.961B(2))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafeHarbourStep {
    /// (a) Identify objectives, financial situation, needs
    IdentifyClientCircumstances { completed: bool },
    /// (b) Identify subject matter of advice
    IdentifySubjectMatter { completed: bool },
    /// (c) Reasonable investigation
    ReasonableInvestigation {
        completed: bool,
        products_considered: u32,
    },
    /// (d) Ensure advice appropriate
    EnsureAppropriate { completed: bool },
    /// (e) Base on reasonable assessment
    ReasonableAssessment { completed: bool },
    /// (f) Consider whether to recommend product
    ConsiderRecommendation { completed: bool },
    /// (g) Other relevant inquiries
    OtherInquiries { completed: bool },
}

impl SafeHarbourStep {
    /// Check if step is completed
    pub fn is_completed(&self) -> bool {
        match self {
            SafeHarbourStep::IdentifyClientCircumstances { completed } => *completed,
            SafeHarbourStep::IdentifySubjectMatter { completed } => *completed,
            SafeHarbourStep::ReasonableInvestigation { completed, .. } => *completed,
            SafeHarbourStep::EnsureAppropriate { completed } => *completed,
            SafeHarbourStep::ReasonableAssessment { completed } => *completed,
            SafeHarbourStep::ConsiderRecommendation { completed } => *completed,
            SafeHarbourStep::OtherInquiries { completed } => *completed,
        }
    }

    /// Get step description
    pub fn description(&self) -> &'static str {
        match self {
            SafeHarbourStep::IdentifyClientCircumstances { .. } => {
                "Identify objectives, financial situation, and needs"
            }
            SafeHarbourStep::IdentifySubjectMatter { .. } => "Identify subject matter of advice",
            SafeHarbourStep::ReasonableInvestigation { .. } => {
                "Reasonable investigation of financial products"
            }
            SafeHarbourStep::EnsureAppropriate { .. } => "Ensure advice is appropriate",
            SafeHarbourStep::ReasonableAssessment { .. } => "Base on reasonable assessment",
            SafeHarbourStep::ConsiderRecommendation { .. } => {
                "Consider whether to recommend product"
            }
            SafeHarbourStep::OtherInquiries { .. } => "Conduct other relevant inquiries",
        }
    }
}

/// Advice document types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdviceDocument {
    /// Financial Services Guide
    Fsg,
    /// Product Disclosure Statement
    Pds,
    /// Statement of Advice
    Soa,
    /// Record of Advice
    Roa,
}

/// Financial Services Guide (s.941A-942C)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialServicesGuide {
    /// Issuer name
    pub issuer_name: String,
    /// AFSL number
    pub afsl_number: String,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Services described
    pub services_described: bool,
    /// Remuneration disclosed
    pub remuneration_disclosed: bool,
    /// Associations disclosed
    pub associations_disclosed: bool,
    /// Dispute resolution information
    pub dispute_resolution_info: bool,
    /// Compensation arrangements
    pub compensation_arrangements: bool,
    /// Provided to client
    pub provided_to_client: bool,
    /// Provision date
    pub provision_date: Option<NaiveDate>,
}

/// Product Disclosure Statement (s.1012A-1013L)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDisclosureStatement {
    /// Product name
    pub product_name: String,
    /// Issuer name
    pub issuer_name: String,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Product features described
    pub features_described: bool,
    /// Fees disclosed
    pub fees_disclosed: bool,
    /// Risks disclosed
    pub risks_disclosed: bool,
    /// Cooling off rights
    pub cooling_off_rights: bool,
    /// Complaints handling
    pub complaints_handling: bool,
    /// Taxation information
    pub taxation_info: bool,
    /// Provided to client
    pub provided_to_client: bool,
    /// Provision date
    pub provision_date: Option<NaiveDate>,
}

/// Statement of Advice (s.946A-947D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatementOfAdvice {
    /// Client name
    pub client_name: String,
    /// Adviser name
    pub adviser_name: String,
    /// Advice date
    pub advice_date: NaiveDate,
    /// Advice summary included
    pub advice_summary: bool,
    /// Basis for advice explained
    pub basis_explained: bool,
    /// Information relied on disclosed
    pub information_disclosed: bool,
    /// Incomplete information warning (if applicable)
    pub incomplete_info_warning: Option<String>,
    /// Remuneration disclosed
    pub remuneration_disclosed: bool,
    /// Associations disclosed
    pub associations_disclosed: bool,
    /// Replacement product disclosure (if applicable)
    pub replacement_disclosure: Option<bool>,
    /// Provided to client
    pub provided_to_client: bool,
    /// Provision date
    pub provision_date: Option<NaiveDate>,
}

/// Conflicted remuneration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictedRemuneration {
    /// Description
    pub description: String,
    /// Amount (AUD)
    pub amount_aud: f64,
    /// Source
    pub source: String,
    /// Remuneration type
    pub remuneration_type: RemunerationType,
    /// Is permitted under exemption
    pub is_permitted: bool,
    /// Exemption reason if permitted
    pub exemption_reason: Option<String>,
}

/// Remuneration types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemunerationType {
    /// Volume-based benefit (prohibited)
    VolumeBased,
    /// Soft dollar benefit (generally prohibited)
    SoftDollar,
    /// Asset-based fee (permitted, except on borrowed in super)
    AssetBased,
    /// Flat fee (permitted)
    FlatFee,
    /// Hourly fee (permitted)
    HourlyFee,
    /// Insurance commission (grandfathered)
    InsuranceCommission,
    /// Platform rebate
    PlatformRebate,
}

impl RemunerationType {
    /// Check if generally prohibited
    pub fn is_generally_prohibited(&self) -> bool {
        matches!(
            self,
            RemunerationType::VolumeBased | RemunerationType::SoftDollar
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advice_type() {
        assert!(AdviceType::Personal.best_interests_duty_applies());
        assert!(AdviceType::Personal.requires_soa());

        assert!(!AdviceType::General.best_interests_duty_applies());
        assert!(AdviceType::General.requires_general_advice_warning());
    }

    #[test]
    fn test_safe_harbour_step() {
        let step = SafeHarbourStep::IdentifyClientCircumstances { completed: true };
        assert!(step.is_completed());

        let step = SafeHarbourStep::ReasonableInvestigation {
            completed: false,
            products_considered: 0,
        };
        assert!(!step.is_completed());
    }

    #[test]
    fn test_remuneration_type() {
        assert!(RemunerationType::VolumeBased.is_generally_prohibited());
        assert!(RemunerationType::SoftDollar.is_generally_prohibited());
        assert!(!RemunerationType::FlatFee.is_generally_prohibited());
    }
}
