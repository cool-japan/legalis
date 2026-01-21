//! Superannuation Benefits
//!
//! Implements benefit payment rules, preservation, and conditions of release.
//!
//! ## Key Legislation
//!
//! - Superannuation Industry (Supervision) Act 1993 Part 6 (Preservation)
//! - SIS Regulations Schedule 1 (Conditions of Release)
//!
//! ## Preservation
//!
//! Benefits are preserved until a condition of release is met:
//! - Retirement (ceased gainful employment after preservation age)
//! - Reaching age 65
//! - Permanent incapacity
//! - Terminal medical condition
//! - Death

use super::error::{Result, SuperannuationError};
use super::types::*;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Benefit release assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenefitReleaseAssessment {
    /// Member ID
    pub member_id: String,
    /// Condition of release claimed
    pub condition: ConditionOfRelease,
    /// Is condition met
    pub condition_met: bool,
    /// Reason if not met
    pub reason: Option<String>,
    /// Maximum amount releasable
    pub releasable_amount: f64,
    /// Can be taken as lump sum
    pub lump_sum_available: bool,
    /// Can be taken as pension
    pub pension_available: bool,
    /// Tax treatment
    pub tax_treatment: BenefitTaxTreatment,
}

/// Tax treatment of superannuation benefit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenefitTaxTreatment {
    /// Tax-free component
    pub tax_free_component: f64,
    /// Taxable component (taxed element)
    pub taxable_taxed_element: f64,
    /// Taxable component (untaxed element)
    pub taxable_untaxed_element: f64,
    /// Low rate cap applicable
    pub low_rate_cap_applicable: bool,
    /// Low rate cap remaining
    pub low_rate_cap_remaining: f64,
}

/// Low rate cap amount (2024-25)
pub const LOW_RATE_CAP_2024_25: f64 = 235_000.0;

/// Assess benefit release eligibility
pub fn assess_benefit_release(
    member: &FundMember,
    condition: ConditionOfRelease,
    assessment_date: NaiveDate,
    additional_evidence: Option<&str>,
) -> Result<BenefitReleaseAssessment> {
    let age = member.age_at(assessment_date);
    let preservation_age = member.preservation_age();

    let (condition_met, reason) =
        check_condition_met(condition, age, preservation_age, additional_evidence);

    // Determine releasable amount
    let releasable_amount = if condition_met {
        calculate_releasable_amount(member, condition)
    } else {
        0.0
    };

    // Check payment type availability
    let lump_sum_available = condition_met && condition.allows_lump_sum();
    let pension_available = condition_met
        && matches!(
            condition,
            ConditionOfRelease::Retirement
                | ConditionOfRelease::Age65
                | ConditionOfRelease::TransitionToRetirement
        );

    // Calculate tax treatment
    let tax_treatment = calculate_tax_treatment(member, age, preservation_age);

    Ok(BenefitReleaseAssessment {
        member_id: member.member_id.clone(),
        condition,
        condition_met,
        reason,
        releasable_amount,
        lump_sum_available,
        pension_available,
        tax_treatment,
    })
}

/// Check if condition of release is met
fn check_condition_met(
    condition: ConditionOfRelease,
    age: u32,
    preservation_age: u32,
    evidence: Option<&str>,
) -> (bool, Option<String>) {
    match condition {
        ConditionOfRelease::Age65 => {
            if age >= 65 {
                (true, None)
            } else {
                (
                    false,
                    Some(format!("Member aged {} has not reached 65", age)),
                )
            }
        }
        ConditionOfRelease::Retirement => {
            if age >= preservation_age {
                // Would need employment cessation evidence in practice
                (true, None)
            } else {
                (
                    false,
                    Some(format!(
                        "Member aged {} has not reached preservation age {}",
                        age, preservation_age
                    )),
                )
            }
        }
        ConditionOfRelease::TransitionToRetirement => {
            if age >= preservation_age && age < 65 {
                (true, None)
            } else if age >= 65 {
                (
                    false,
                    Some("Age 65+ - use retirement condition".to_string()),
                )
            } else {
                (
                    false,
                    Some(format!(
                        "Member aged {} has not reached preservation age {}",
                        age, preservation_age
                    )),
                )
            }
        }
        ConditionOfRelease::PermanentIncapacity => {
            // Would need two medical certificates in practice
            if evidence.is_some() {
                (true, None)
            } else {
                (
                    false,
                    Some("Two medical practitioner certificates required".to_string()),
                )
            }
        }
        ConditionOfRelease::TerminalMedicalCondition => {
            // Would need two medical certificates, life expectancy < 24 months
            if evidence.is_some() {
                (true, None)
            } else {
                (
                    false,
                    Some(
                        "Medical certification required (life expectancy < 24 months)".to_string(),
                    ),
                )
            }
        }
        ConditionOfRelease::SevereFinancialHardship => {
            // Would need Centrelink evidence + fund assessment
            if evidence.is_some() {
                (true, None)
            } else {
                (
                    false,
                    Some("Centrelink evidence and fund assessment required".to_string()),
                )
            }
        }
        ConditionOfRelease::CompassionateGrounds => {
            // ATO approval required
            if evidence.is_some() {
                (true, None)
            } else {
                (false, Some("ATO approval required".to_string()))
            }
        }
        ConditionOfRelease::Death => (true, None),
        ConditionOfRelease::DepartingTemporaryResident => {
            // Would verify visa status
            if evidence.is_some() {
                (true, None)
            } else {
                (
                    false,
                    Some("Visa cancellation/departure evidence required".to_string()),
                )
            }
        }
        ConditionOfRelease::FirstHomeSuperSaver => {
            // FHSSS determination from ATO required
            if evidence.is_some() {
                (true, None)
            } else {
                (false, Some("ATO FHSSS determination required".to_string()))
            }
        }
        ConditionOfRelease::LostMemberAccount => {
            // Would check account balance and inactivity
            (true, None)
        }
        ConditionOfRelease::ReleaseAuthority => {
            // ATO release authority required
            if evidence.is_some() {
                (true, None)
            } else {
                (false, Some("ATO release authority required".to_string()))
            }
        }
    }
}

/// Calculate releasable amount based on condition
fn calculate_releasable_amount(member: &FundMember, condition: ConditionOfRelease) -> f64 {
    match condition {
        // Full access conditions
        ConditionOfRelease::Retirement
        | ConditionOfRelease::Age65
        | ConditionOfRelease::PermanentIncapacity
        | ConditionOfRelease::TerminalMedicalCondition
        | ConditionOfRelease::Death => member.account_balance,

        // TTR - restricted non-preserved only
        ConditionOfRelease::TransitionToRetirement => {
            member.unrestricted_non_preserved + member.restricted_non_preserved
        }

        // Severe financial hardship - limited release
        ConditionOfRelease::SevereFinancialHardship => {
            // Up to $10,000 maximum
            member.account_balance.min(10_000.0)
        }

        // Others - varies
        ConditionOfRelease::DepartingTemporaryResident => member.account_balance,
        ConditionOfRelease::CompassionateGrounds => member.account_balance,
        ConditionOfRelease::FirstHomeSuperSaver => {
            // Up to $50,000 FHSSS limit
            50_000.0_f64.min(member.account_balance)
        }
        ConditionOfRelease::LostMemberAccount => member.account_balance,
        ConditionOfRelease::ReleaseAuthority => member.account_balance,
    }
}

/// Calculate tax treatment of benefit
fn calculate_tax_treatment(
    member: &FundMember,
    age: u32,
    _preservation_age: u32,
) -> BenefitTaxTreatment {
    // Simplified - in practice would track components
    let total = member.account_balance;
    let tax_free_proportion = 0.10; // Simplified assumption
    let tax_free = total * tax_free_proportion;
    let taxable = total - tax_free;

    // Low rate cap applies between preservation age and 60
    let low_rate_cap_applicable = age < 60;

    BenefitTaxTreatment {
        tax_free_component: tax_free,
        taxable_taxed_element: taxable,
        taxable_untaxed_element: 0.0,
        low_rate_cap_applicable,
        low_rate_cap_remaining: LOW_RATE_CAP_2024_25,
    }
}

/// Pension drawdown rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionDrawdown {
    /// Account balance at 1 July
    pub opening_balance: f64,
    /// Member's age at 1 July
    pub age: u32,
    /// Minimum drawdown percentage
    pub minimum_percentage: f64,
    /// Minimum annual payment
    pub minimum_payment: f64,
    /// Maximum drawdown (if TTR)
    pub maximum_percentage: Option<f64>,
    /// Maximum annual payment (if TTR)
    pub maximum_payment: Option<f64>,
}

/// Minimum drawdown factors by age
pub fn minimum_drawdown_factor(age: u32) -> f64 {
    match age {
        0..=64 => 0.04,  // 4%
        65..=74 => 0.05, // 5%
        75..=79 => 0.06, // 6%
        80..=84 => 0.07, // 7%
        85..=89 => 0.09, // 9%
        90..=94 => 0.11, // 11%
        _ => 0.14,       // 14%
    }
}

/// Calculate pension drawdown requirements
pub fn calculate_pension_drawdown(opening_balance: f64, age: u32, is_ttr: bool) -> PensionDrawdown {
    let minimum_percentage = minimum_drawdown_factor(age);
    let minimum_payment = opening_balance * minimum_percentage;

    let (max_pct, max_payment) = if is_ttr {
        let max = 0.10; // 10% maximum for TTR
        (Some(max), Some(opening_balance * max))
    } else {
        (None, None)
    };

    PensionDrawdown {
        opening_balance,
        age,
        minimum_percentage,
        minimum_payment,
        maximum_percentage: max_pct,
        maximum_payment: max_payment,
    }
}

/// Validate benefit payment
pub fn validate_benefit_payment(
    member: &FundMember,
    amount: f64,
    condition: ConditionOfRelease,
    payment_type: BenefitPaymentType,
    payment_date: NaiveDate,
) -> Result<()> {
    let age = member.age_at(payment_date);
    let preservation_age = member.preservation_age();

    // Check condition is met
    let (met, reason) = check_condition_met(condition, age, preservation_age, None);
    if !met {
        return Err(SuperannuationError::PreservationRequirementsNotMet {
            benefit_type: format!("{:?}", payment_type),
            required_condition: reason.unwrap_or_default(),
        });
    }

    // Check amount doesn't exceed balance
    if amount > member.account_balance {
        return Err(SuperannuationError::InvalidBenefitPayment {
            reason: format!(
                "Payment amount ${:.2} exceeds account balance ${:.2}",
                amount, member.account_balance
            ),
        });
    }

    // TTR - cannot take lump sums
    if condition == ConditionOfRelease::TransitionToRetirement
        && payment_type == BenefitPaymentType::LumpSum
    {
        return Err(SuperannuationError::InvalidBenefitPayment {
            reason: "TTR pension cannot be taken as lump sum".to_string(),
        });
    }

    // Severe financial hardship - amount limits
    if condition == ConditionOfRelease::SevereFinancialHardship && amount > 10_000.0 {
        return Err(SuperannuationError::InvalidBenefitPayment {
            reason: "Severe financial hardship release limited to $10,000".to_string(),
        });
    }

    Ok(())
}

/// Death benefit distribution validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeathBenefitDistribution {
    /// Total death benefit
    pub total_benefit: f64,
    /// Beneficiary allocations
    pub allocations: Vec<BeneficiaryAllocation>,
    /// Estate allocation (if any)
    pub estate_allocation: Option<f64>,
    /// Valid nominations
    pub valid: bool,
    /// Validation issues
    pub issues: Vec<String>,
}

/// Beneficiary allocation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeneficiaryAllocation {
    /// Beneficiary name
    pub name: String,
    /// Relationship
    pub relationship: BeneficiaryRelationship,
    /// Percentage
    pub percentage: f64,
    /// Amount
    pub amount: f64,
    /// Is tax dependant
    pub is_tax_dependant: bool,
}

/// Validate death benefit distribution
pub fn validate_death_benefit(
    member: &FundMember,
    current_date: NaiveDate,
) -> DeathBenefitDistribution {
    let total_benefit = member.account_balance;
    let mut allocations = Vec::new();
    let mut issues = Vec::new();
    let mut estate_allocation = None;
    let mut total_percentage = 0.0;

    for nom in &member.beneficiaries {
        // Check if binding nomination is still valid (3-year limit)
        if nom.nomination_type == NominationType::Binding {
            if let Some(expiry) = nom.expiry_date {
                if expiry < current_date {
                    issues.push(format!(
                        "Binding nomination for {} expired on {}",
                        nom.name, expiry
                    ));
                    continue;
                }
            } else {
                // Check 3-year rule from nomination date
                let three_years_later = NaiveDate::from_ymd_opt(
                    nom.nomination_date.year() + 3,
                    nom.nomination_date.month(),
                    nom.nomination_date.day(),
                );
                if let Some(expiry) = three_years_later
                    && expiry < current_date
                {
                    issues.push(format!(
                        "Binding nomination for {} has exceeded 3-year validity",
                        nom.name
                    ));
                    continue;
                }
            }
        }

        // Check beneficiary is valid dependant or LPR
        let is_valid = matches!(
            nom.relationship,
            BeneficiaryRelationship::Spouse
                | BeneficiaryRelationship::Child
                | BeneficiaryRelationship::FinancialDependant
                | BeneficiaryRelationship::Interdependent
                | BeneficiaryRelationship::LegalPersonalRepresentative
        );

        if !is_valid {
            issues.push(format!(
                "{} is not a valid death benefit dependant or LPR",
                nom.name
            ));
            continue;
        }

        if nom.relationship == BeneficiaryRelationship::LegalPersonalRepresentative {
            estate_allocation = Some(total_benefit * nom.percentage / 100.0);
        } else {
            allocations.push(BeneficiaryAllocation {
                name: nom.name.clone(),
                relationship: nom.relationship,
                percentage: nom.percentage,
                amount: total_benefit * nom.percentage / 100.0,
                is_tax_dependant: nom.relationship.is_tax_dependant(),
            });
        }

        total_percentage += nom.percentage;
    }

    // Check total percentage
    if (total_percentage - 100.0).abs() > 0.01 && !allocations.is_empty() {
        issues.push(format!(
            "Beneficiary percentages total {:.1}%, should be 100%",
            total_percentage
        ));
    }

    let valid = issues.is_empty() && !allocations.is_empty();

    DeathBenefitDistribution {
        total_benefit,
        allocations,
        estate_allocation,
        valid,
        issues,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_member() -> FundMember {
        FundMember {
            member_id: "M001".to_string(),
            tfn_provided: true,
            date_of_birth: NaiveDate::from_ymd_opt(1965, 6, 15).unwrap(),
            join_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            category: MemberCategory::Accumulation,
            account_balance: 500_000.0,
            preserved_amount: 450_000.0,
            restricted_non_preserved: 30_000.0,
            unrestricted_non_preserved: 20_000.0,
            insurance: None,
            beneficiaries: vec![],
        }
    }

    #[test]
    fn test_assess_benefit_release_age_65() {
        let mut member = create_test_member();
        member.date_of_birth = NaiveDate::from_ymd_opt(1958, 1, 1).unwrap();

        let assessment_date = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
        let result =
            assess_benefit_release(&member, ConditionOfRelease::Age65, assessment_date, None)
                .unwrap();

        assert!(result.condition_met);
        assert_eq!(result.releasable_amount, 500_000.0);
        assert!(result.lump_sum_available);
    }

    #[test]
    fn test_assess_benefit_release_ttr() {
        let member = create_test_member();
        // Assessment after 60th birthday (born 1965-06-15, preservation age 60)
        let assessment_date = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

        let result = assess_benefit_release(
            &member,
            ConditionOfRelease::TransitionToRetirement,
            assessment_date,
            None,
        )
        .unwrap();

        assert!(result.condition_met);
        assert!(!result.lump_sum_available); // TTR cannot be lump sum
        assert!(result.pension_available);
    }

    #[test]
    fn test_minimum_drawdown_factors() {
        assert_eq!(minimum_drawdown_factor(60), 0.04);
        assert_eq!(minimum_drawdown_factor(65), 0.05);
        assert_eq!(minimum_drawdown_factor(75), 0.06);
        assert_eq!(minimum_drawdown_factor(95), 0.14);
    }

    #[test]
    fn test_calculate_pension_drawdown() {
        let result = calculate_pension_drawdown(500_000.0, 65, false);
        assert_eq!(result.minimum_percentage, 0.05);
        assert_eq!(result.minimum_payment, 25_000.0);
        assert!(result.maximum_percentage.is_none());
    }

    #[test]
    fn test_calculate_pension_drawdown_ttr() {
        let result = calculate_pension_drawdown(500_000.0, 60, true);
        assert_eq!(result.minimum_percentage, 0.04);
        assert_eq!(result.minimum_payment, 20_000.0);
        assert_eq!(result.maximum_percentage, Some(0.10));
        assert_eq!(result.maximum_payment, Some(50_000.0));
    }

    #[test]
    fn test_validate_benefit_payment_invalid_amount() {
        let member = create_test_member();
        let result = validate_benefit_payment(
            &member,
            600_000.0, // More than balance
            ConditionOfRelease::Age65,
            BenefitPaymentType::LumpSum,
            NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        );
        assert!(result.is_err());
    }
}
