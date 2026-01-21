//! Self-Managed Superannuation Funds (SMSF)
//!
//! Implements SMSF-specific rules under SIS Act 1993.
//!
//! ## Key Requirements
//!
//! - Maximum 6 members
//! - All members must be trustees (or directors of corporate trustee)
//! - Annual audit by approved SMSF auditor
//! - Investment strategy required
//! - In-house asset rule (max 5%)
//! - Sole purpose test
//!
//! ## Trustee Duties
//!
//! Section 52B of SIS Act sets out trustee covenants:
//! - Act honestly
//! - Exercise care, skill and diligence
//! - Perform duties in best interests of members
//! - Keep money and assets separate
//! - Comply with superannuation law

use super::error::{Result, SuperannuationError};
use super::types::*;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// SMSF compliance assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsfComplianceAssessment {
    /// Fund ABN
    pub fund_abn: String,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Is compliant
    pub is_compliant: bool,
    /// Compliance issues
    pub issues: Vec<ComplianceIssue>,
    /// Sole purpose test status
    pub sole_purpose_compliant: bool,
    /// In-house asset rule status
    pub in_house_asset_compliant: bool,
    /// Trustee status compliant
    pub trustee_status_compliant: bool,
    /// Investment strategy documented
    pub investment_strategy_documented: bool,
    /// Annual return lodged
    pub annual_return_lodged: bool,
    /// Audit completed
    pub audit_completed: bool,
}

/// Compliance issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceIssue {
    /// Issue category
    pub category: ComplianceCategory,
    /// Description
    pub description: String,
    /// SIS Act reference
    pub sis_reference: String,
    /// Severity
    pub severity: IssueSeverity,
    /// Remediation required
    pub remediation: String,
}

/// Compliance category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceCategory {
    /// Sole purpose test
    SolePurpose,
    /// In-house assets
    InHouseAssets,
    /// Trustee duties
    TrusteeDuties,
    /// Investment strategy
    InvestmentStrategy,
    /// Arm's length dealing
    ArmsLength,
    /// Documentation
    Documentation,
    /// Lodgement
    Lodgement,
    /// Member eligibility
    MemberEligibility,
    /// Contribution rules
    ContributionRules,
    /// Benefit payment
    BenefitPayment,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Minor - remediation recommended
    Minor,
    /// Moderate - remediation required
    Moderate,
    /// Serious - may affect complying status
    Serious,
    /// Critical - fund likely non-complying
    Critical,
}

/// SMSF trustee disqualification reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisqualificationReason {
    /// Convicted of dishonesty offence
    DishonestyConviction,
    /// Civil penalty order
    CivilPenaltyOrder,
    /// Undischarged bankrupt
    UndischargedBankrupt,
    /// Disqualified from managing corporations
    DisqualifiedFromCorporations,
    /// Mentally incapacitated
    MentallyIncapacitated,
}

/// SMSF investment strategy requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestmentStrategy {
    /// Last review date
    pub last_review_date: NaiveDate,
    /// Considers risk profile
    pub considers_risk: bool,
    /// Considers diversification
    pub considers_diversification: bool,
    /// Considers liquidity
    pub considers_liquidity: bool,
    /// Considers insurance needs
    pub considers_insurance: bool,
    /// Considers members' circumstances
    pub considers_member_circumstances: bool,
    /// Documented investment objectives
    pub documented_objectives: bool,
    /// Documented asset allocation
    pub documented_asset_allocation: bool,
}

impl InvestmentStrategy {
    /// Check if strategy is compliant with SIS Reg 4.09
    pub fn is_compliant(&self) -> bool {
        self.considers_risk
            && self.considers_diversification
            && self.considers_liquidity
            && self.considers_insurance
            && self.considers_member_circumstances
            && self.documented_objectives
            && self.documented_asset_allocation
    }

    /// Get missing elements
    pub fn missing_elements(&self) -> Vec<String> {
        let mut missing = Vec::new();
        if !self.considers_risk {
            missing.push("Risk profile consideration".to_string());
        }
        if !self.considers_diversification {
            missing.push("Diversification consideration".to_string());
        }
        if !self.considers_liquidity {
            missing.push("Liquidity consideration".to_string());
        }
        if !self.considers_insurance {
            missing.push("Insurance needs consideration".to_string());
        }
        if !self.considers_member_circumstances {
            missing.push("Member circumstances consideration".to_string());
        }
        if !self.documented_objectives {
            missing.push("Documented investment objectives".to_string());
        }
        if !self.documented_asset_allocation {
            missing.push("Documented asset allocation".to_string());
        }
        missing
    }
}

/// SMSF in-house asset calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InHouseAssetCalculation {
    /// Total fund assets
    pub total_assets: f64,
    /// In-house asset value
    pub in_house_asset_value: f64,
    /// In-house asset percentage
    pub percentage: f64,
    /// Is within limit (5%)
    pub within_limit: bool,
    /// Excess amount (if any)
    pub excess_amount: f64,
    /// In-house assets list
    pub in_house_assets: Vec<InHouseAsset>,
}

/// In-house asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InHouseAsset {
    /// Asset description
    pub description: String,
    /// Asset value
    pub value: f64,
    /// Related party
    pub related_party: String,
    /// Asset type
    pub asset_type: InHouseAssetType,
}

/// In-house asset types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InHouseAssetType {
    /// Loan to related party
    LoanToRelatedParty,
    /// Investment in related trust
    RelatedTrustInvestment,
    /// Investment in related company
    RelatedCompanyInvestment,
    /// Asset leased to related party
    AssetLeasedToRelatedParty,
}

/// In-house asset limit (5%)
pub const IN_HOUSE_ASSET_LIMIT: f64 = 0.05;

/// Calculate in-house asset position
pub fn calculate_in_house_assets(
    total_assets: f64,
    in_house_assets: &[InHouseAsset],
) -> InHouseAssetCalculation {
    let in_house_value: f64 = in_house_assets.iter().map(|a| a.value).sum();
    let percentage = if total_assets > 0.0 {
        in_house_value / total_assets
    } else {
        0.0
    };

    let within_limit = percentage <= IN_HOUSE_ASSET_LIMIT;
    let excess = if within_limit {
        0.0
    } else {
        in_house_value - (total_assets * IN_HOUSE_ASSET_LIMIT)
    };

    InHouseAssetCalculation {
        total_assets,
        in_house_asset_value: in_house_value,
        percentage,
        within_limit,
        excess_amount: excess,
        in_house_assets: in_house_assets.to_vec(),
    }
}

/// Validate SMSF trustee eligibility
pub fn validate_trustee_eligibility(
    is_individual: bool,
    age: u32,
    is_member: bool,
    disqualification_reason: Option<DisqualificationReason>,
) -> Result<()> {
    // Must be 18 or over
    if age < 18 {
        return Err(SuperannuationError::DisqualifiedTrustee {
            reason: "Trustee must be 18 years or older".to_string(),
        });
    }

    // Must be a member (for individual trustees)
    if is_individual && !is_member {
        return Err(SuperannuationError::SmsfTrusteeBreach {
            breach: "Individual trustee must also be a member of the SMSF".to_string(),
        });
    }

    // Check disqualification
    if let Some(reason) = disqualification_reason {
        let reason_str = match reason {
            DisqualificationReason::DishonestyConviction => "Convicted of dishonesty offence",
            DisqualificationReason::CivilPenaltyOrder => "Subject to civil penalty order",
            DisqualificationReason::UndischargedBankrupt => "Undischarged bankrupt",
            DisqualificationReason::DisqualifiedFromCorporations => {
                "Disqualified from managing corporations"
            }
            DisqualificationReason::MentallyIncapacitated => "Mentally incapacitated",
        };
        return Err(SuperannuationError::DisqualifiedTrustee {
            reason: reason_str.to_string(),
        });
    }

    Ok(())
}

/// Validate SMSF member count
pub fn validate_member_count(current_members: u32) -> Result<()> {
    // Maximum 6 members from 1 July 2021
    if current_members > 6 {
        return Err(SuperannuationError::SmsfMemberLimitExceeded {
            current: current_members,
        });
    }
    Ok(())
}

/// SMSF audit contravention report
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditContraventionReport {
    /// Auditor registration number
    pub auditor_number: String,
    /// Fund ABN
    pub fund_abn: String,
    /// Financial year
    pub financial_year: String,
    /// Contraventions
    pub contraventions: Vec<AuditContravention>,
    /// Reportable to ATO
    pub reportable_to_ato: bool,
}

/// Audit contravention
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditContravention {
    /// SIS Act section
    pub sis_section: String,
    /// Description
    pub description: String,
    /// Requires ACR lodgement
    pub requires_acr: bool,
    /// Rectified by audit date
    pub rectified: bool,
}

/// Assess SMSF compliance
pub fn assess_smsf_compliance(
    fund: &SuperannuationFund,
    smsf_details: &SmsfDetails,
    members: &[FundMember],
    in_house_assets: &[InHouseAsset],
    investment_strategy: Option<&InvestmentStrategy>,
    assessment_date: NaiveDate,
) -> SmsfComplianceAssessment {
    let mut issues = Vec::new();
    let mut is_compliant = true;

    // Check member count
    if members.len() > 6 {
        issues.push(ComplianceIssue {
            category: ComplianceCategory::MemberEligibility,
            description: format!("SMSF has {} members, maximum is 6", members.len()),
            sis_reference: "s.17A".to_string(),
            severity: IssueSeverity::Critical,
            remediation: "Reduce member count to 6 or fewer".to_string(),
        });
        is_compliant = false;
    }

    // Check investment strategy
    let investment_strategy_documented = if let Some(strategy) = investment_strategy {
        if !strategy.is_compliant() {
            let missing = strategy.missing_elements();
            issues.push(ComplianceIssue {
                category: ComplianceCategory::InvestmentStrategy,
                description: format!("Investment strategy missing: {}", missing.join(", ")),
                sis_reference: "SIS Reg 4.09".to_string(),
                severity: IssueSeverity::Moderate,
                remediation: "Update investment strategy to address missing elements".to_string(),
            });
        }
        true
    } else {
        issues.push(ComplianceIssue {
            category: ComplianceCategory::InvestmentStrategy,
            description: "No investment strategy documented".to_string(),
            sis_reference: "SIS Reg 4.09".to_string(),
            severity: IssueSeverity::Serious,
            remediation: "Document investment strategy addressing all required elements"
                .to_string(),
        });
        is_compliant = false;
        false
    };

    // Check in-house assets
    let iha = calculate_in_house_assets(fund.total_assets, in_house_assets);
    let in_house_asset_compliant = iha.within_limit;
    if !in_house_asset_compliant {
        issues.push(ComplianceIssue {
            category: ComplianceCategory::InHouseAssets,
            description: format!(
                "In-house assets at {:.1}%, exceeds 5% limit by ${:.2}",
                iha.percentage * 100.0,
                iha.excess_amount
            ),
            sis_reference: "s.82-84".to_string(),
            severity: IssueSeverity::Critical,
            remediation: "Dispose of excess in-house assets by end of financial year".to_string(),
        });
        is_compliant = false;
    }

    // Check annual return lodgement
    let annual_return_lodged = smsf_details.last_annual_return.is_some();
    if !annual_return_lodged {
        issues.push(ComplianceIssue {
            category: ComplianceCategory::Lodgement,
            description: "Annual return not lodged".to_string(),
            sis_reference: "TAA s.388-50".to_string(),
            severity: IssueSeverity::Serious,
            remediation: "Lodge annual return by due date".to_string(),
        });
    }

    // Check audit completed
    let audit_completed = smsf_details.last_audit_date.is_some_and(|audit_date| {
        // Check audit is within last 15 months
        let months_since = (assessment_date.year() - audit_date.year()) * 12
            + (assessment_date.month() as i32 - audit_date.month() as i32);
        months_since <= 15
    });
    if !audit_completed {
        issues.push(ComplianceIssue {
            category: ComplianceCategory::Documentation,
            description: "Annual audit not completed within required timeframe".to_string(),
            sis_reference: "s.35C".to_string(),
            severity: IssueSeverity::Serious,
            remediation: "Engage approved SMSF auditor to complete audit".to_string(),
        });
    }

    SmsfComplianceAssessment {
        fund_abn: fund.abn.clone(),
        assessment_date,
        is_compliant,
        issues,
        sole_purpose_compliant: true, // Simplified - would need detailed assessment
        in_house_asset_compliant,
        trustee_status_compliant: true, // Simplified
        investment_strategy_documented,
        annual_return_lodged,
        audit_completed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_investment_strategy_compliance() {
        let strategy = InvestmentStrategy {
            last_review_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            considers_risk: true,
            considers_diversification: true,
            considers_liquidity: true,
            considers_insurance: true,
            considers_member_circumstances: true,
            documented_objectives: true,
            documented_asset_allocation: true,
        };
        assert!(strategy.is_compliant());
        assert!(strategy.missing_elements().is_empty());
    }

    #[test]
    fn test_investment_strategy_missing_elements() {
        let strategy = InvestmentStrategy {
            last_review_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
            considers_risk: true,
            considers_diversification: false,
            considers_liquidity: true,
            considers_insurance: false,
            considers_member_circumstances: true,
            documented_objectives: true,
            documented_asset_allocation: true,
        };
        assert!(!strategy.is_compliant());
        let missing = strategy.missing_elements();
        assert!(missing.contains(&"Diversification consideration".to_string()));
        assert!(missing.contains(&"Insurance needs consideration".to_string()));
    }

    #[test]
    fn test_in_house_asset_within_limit() {
        let assets = vec![InHouseAsset {
            description: "Loan to member".to_string(),
            value: 40_000.0,
            related_party: "John Smith".to_string(),
            asset_type: InHouseAssetType::LoanToRelatedParty,
        }];

        let result = calculate_in_house_assets(1_000_000.0, &assets);
        assert!(result.within_limit);
        assert_eq!(result.percentage, 0.04);
    }

    #[test]
    fn test_in_house_asset_exceeds_limit() {
        let assets = vec![InHouseAsset {
            description: "Loan to related company".to_string(),
            value: 100_000.0,
            related_party: "Smith Pty Ltd".to_string(),
            asset_type: InHouseAssetType::RelatedCompanyInvestment,
        }];

        let result = calculate_in_house_assets(1_000_000.0, &assets);
        assert!(!result.within_limit);
        assert_eq!(result.percentage, 0.10);
        assert_eq!(result.excess_amount, 50_000.0);
    }

    #[test]
    fn test_validate_trustee_eligibility_valid() {
        let result = validate_trustee_eligibility(true, 35, true, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_trustee_eligibility_underage() {
        let result = validate_trustee_eligibility(true, 17, true, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_trustee_eligibility_bankrupt() {
        let result = validate_trustee_eligibility(
            true,
            35,
            true,
            Some(DisqualificationReason::UndischargedBankrupt),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_member_count() {
        assert!(validate_member_count(4).is_ok());
        assert!(validate_member_count(6).is_ok());
        assert!(validate_member_count(7).is_err());
    }
}
