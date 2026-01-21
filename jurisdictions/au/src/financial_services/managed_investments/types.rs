//! Managed Investments Types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Managed investment scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagedInvestmentScheme {
    /// Scheme name
    pub name: String,
    /// ARSN (Australian Registered Scheme Number)
    pub arsn: String,
    /// Scheme type
    pub scheme_type: SchemeType,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Responsible entity
    pub responsible_entity: ResponsibleEntity,
    /// Compliance plan
    pub compliance_plan: CompliancePlan,
    /// Net asset value (AUD)
    pub nav_aud: f64,
    /// Number of members
    pub member_count: u32,
    /// Listed on ASX
    pub asx_listed: bool,
    /// ASX code (if listed)
    pub asx_code: Option<String>,
}

/// Scheme types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemeType {
    /// Unit trust
    UnitTrust,
    /// Property trust (A-REIT)
    PropertyTrust,
    /// Infrastructure fund
    InfrastructureFund,
    /// Managed fund
    ManagedFund,
    /// Exchange traded fund (ETF)
    Etf,
    /// Hedge fund
    HedgeFund,
    /// Private equity fund
    PrivateEquityFund,
    /// Mortgage trust
    MortgageTrust,
}

/// Responsible entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsibleEntity {
    /// RE name
    pub name: String,
    /// ABN
    pub abn: String,
    /// AFSL number
    pub afsl_number: String,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Has adequate resources
    pub adequate_resources: bool,
    /// Risk management framework
    pub risk_management: bool,
    /// Compliance framework
    pub compliance_framework: bool,
    /// Meets RG 259 requirements
    pub rg259_compliant: bool,
}

/// Compliance plan (s.601HA)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompliancePlan {
    /// Plan lodged with ASIC
    pub lodged_with_asic: bool,
    /// Lodgement date
    pub lodgement_date: Option<NaiveDate>,
    /// Measures to ensure compliance with Act
    pub act_compliance_measures: bool,
    /// Measures to ensure compliance with constitution
    pub constitution_compliance_measures: bool,
    /// Regular reviews
    pub regular_reviews: bool,
    /// Review frequency
    pub review_frequency_months: u32,
    /// Last review date
    pub last_review_date: Option<NaiveDate>,
    /// Independent compliance committee
    pub compliance_committee: Option<ComplianceCommittee>,
    /// Auditor appointed
    pub auditor_appointed: bool,
    /// Auditor name
    pub auditor_name: Option<String>,
}

/// Compliance committee
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceCommittee {
    /// Committee members
    pub members: u32,
    /// External members (majority required)
    pub external_members: u32,
    /// Meets quarterly
    pub meets_quarterly: bool,
    /// Reports to RE board
    pub reports_to_board: bool,
}

impl ComplianceCommittee {
    /// Check if majority external
    pub fn has_external_majority(&self) -> bool {
        self.external_members > self.members / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_committee_majority() {
        let committee = ComplianceCommittee {
            members: 5,
            external_members: 3,
            meets_quarterly: true,
            reports_to_board: true,
        };
        assert!(committee.has_external_majority());

        let committee = ComplianceCommittee {
            members: 4,
            external_members: 2,
            meets_quarterly: true,
            reports_to_board: true,
        };
        assert!(!committee.has_external_majority());
    }
}
