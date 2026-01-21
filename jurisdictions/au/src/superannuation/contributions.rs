//! Superannuation Contributions
//!
//! Implements the Superannuation Guarantee (SG) system and contribution rules.
//!
//! ## Key Legislation
//!
//! - Superannuation Guarantee (Administration) Act 1992
//! - Income Tax Assessment Act 1997 (contribution caps)
//!
//! ## SG Rate Schedule
//!
//! | Year | Rate |
//! |------|------|
//! | 2024-25 | 11.5% |
//! | 2025-26+ | 12.0% |

use super::error::{Result, SuperannuationError};
use super::types::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// SG calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SgCalculation {
    /// Ordinary time earnings (OTE)
    pub ordinary_time_earnings: f64,
    /// SG rate applied
    pub sg_rate: f64,
    /// SG amount required
    pub sg_amount: f64,
    /// Maximum super contribution base (if capped)
    pub capped_at_maximum: bool,
    /// Quarterly cap applied (if any)
    pub quarterly_cap: Option<f64>,
}

/// Calculate SG contribution required
pub fn calculate_sg_contribution(
    ordinary_time_earnings: f64,
    financial_year: &str,
    apply_maximum_base: bool,
) -> SgCalculation {
    let sg_rate = sg_rate_for_year(financial_year) / 100.0;

    // Check if we need to cap at maximum super contribution base
    let (ote, capped) = if apply_maximum_base
        && ordinary_time_earnings > MAX_SUPER_CONTRIBUTION_BASE_QUARTERLY_2024_25
    {
        (MAX_SUPER_CONTRIBUTION_BASE_QUARTERLY_2024_25, true)
    } else {
        (ordinary_time_earnings, false)
    };

    let sg_amount = ote * sg_rate;

    SgCalculation {
        ordinary_time_earnings,
        sg_rate,
        sg_amount,
        capped_at_maximum: capped,
        quarterly_cap: if capped {
            Some(MAX_SUPER_CONTRIBUTION_BASE_QUARTERLY_2024_25)
        } else {
            None
        },
    }
}

/// Employee SG eligibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmployeeSgEligibility {
    /// Employee identifier
    pub employee_id: String,
    /// Is eligible for SG
    pub is_eligible: bool,
    /// Reason if not eligible
    pub ineligibility_reason: Option<String>,
    /// Monthly OTE threshold met
    pub ote_threshold_met: bool,
    /// Employment type
    pub employment_type: EmploymentType,
}

/// Employment type for SG purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Full-time employee
    FullTime,
    /// Part-time employee
    PartTime,
    /// Casual employee
    Casual,
    /// Contractor (employee for SG purposes)
    ContractorEmployee,
    /// Contractor (not employee)
    ContractorNotEmployee,
    /// Domestic/private worker
    DomesticWorker,
}

/// Check employee eligibility for SG
pub fn check_sg_eligibility(
    _employee_age: u32,
    monthly_ote: f64,
    employment_type: EmploymentType,
) -> EmployeeSgEligibility {
    let mut is_eligible = true;
    let mut reason = None;

    // Age limit removed from 1 July 2022
    // Previously: employees aged 70+ were exempt

    // $450/month threshold removed from 1 July 2022
    // Previously: employees earning <$450/month were exempt
    let ote_threshold_met = monthly_ote > 0.0;

    // Check employment type
    if employment_type == EmploymentType::ContractorNotEmployee {
        is_eligible = false;
        reason = Some("Contractor not an employee for SG purposes".to_string());
    }

    EmployeeSgEligibility {
        employee_id: String::new(),
        is_eligible,
        ineligibility_reason: reason,
        ote_threshold_met,
        employment_type,
    }
}

/// SG shortfall calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SgShortfallCalculation {
    /// Quarter
    pub quarter: SgQuarter,
    /// Financial year
    pub financial_year: String,
    /// SG required
    pub sg_required: f64,
    /// SG paid
    pub sg_paid: f64,
    /// Shortfall amount
    pub shortfall: f64,
    /// Nominal interest (SG charge component)
    pub nominal_interest: f64,
    /// Administration fee ($20 per employee)
    pub administration_fee: f64,
    /// Total SG charge
    pub total_sg_charge: f64,
}

/// Calculate SG shortfall and charge
pub fn calculate_sg_shortfall(
    sg_required: f64,
    sg_paid: f64,
    quarter: SgQuarter,
    financial_year: &str,
    number_of_employees: u32,
    days_late: u32,
) -> SgShortfallCalculation {
    let shortfall = (sg_required - sg_paid).max(0.0);

    // Nominal interest: 10% per annum, calculated daily
    let daily_rate = 0.10 / 365.0;
    let nominal_interest = shortfall * daily_rate * days_late as f64;

    // Administration fee: $20 per employee with shortfall
    let administration_fee = 20.0 * number_of_employees as f64;

    // Total SG charge (non-deductible)
    let total_sg_charge = shortfall + nominal_interest + administration_fee;

    SgShortfallCalculation {
        quarter,
        financial_year: financial_year.to_string(),
        sg_required,
        sg_paid,
        shortfall,
        nominal_interest,
        administration_fee,
        total_sg_charge,
    }
}

/// Contribution cap assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributionCapAssessment {
    /// Financial year
    pub financial_year: String,
    /// Total super balance at 30 June prior year
    pub total_super_balance: f64,
    /// Concessional contributions
    pub concessional_contributions: f64,
    /// Non-concessional contributions
    pub non_concessional_contributions: f64,
    /// Concessional cap
    pub concessional_cap: f64,
    /// Non-concessional cap
    pub non_concessional_cap: f64,
    /// Concessional excess
    pub concessional_excess: f64,
    /// Non-concessional excess
    pub non_concessional_excess: f64,
    /// Carry-forward available (concessional)
    pub carry_forward_available: f64,
    /// Bring-forward available (non-concessional)
    pub bring_forward_available: bool,
}

/// Assess contribution cap position
pub fn assess_contribution_caps(
    total_super_balance: f64,
    concessional_contributions: f64,
    non_concessional_contributions: f64,
    carry_forward_amounts: &[f64], // Up to 5 years
    financial_year: &str,
    age: u32,
) -> ContributionCapAssessment {
    let caps = ContributionCaps::fy_2024_25();

    // Concessional cap with carry-forward
    let base_concessional_cap = caps.concessional_cap;
    let carry_forward: f64 = carry_forward_amounts.iter().sum();
    let effective_concessional_cap = base_concessional_cap + carry_forward;
    let concessional_excess = (concessional_contributions - effective_concessional_cap).max(0.0);

    // Non-concessional cap (depends on total super balance)
    let base_ncc_cap = if total_super_balance >= caps.total_super_balance_threshold {
        0.0 // Cannot make NCC if TSB >= threshold
    } else {
        caps.non_concessional_cap
    };

    // Bring-forward available if under 75 and TSB < threshold
    let bring_forward_available =
        age < 75 && total_super_balance < caps.total_super_balance_threshold;

    let non_concessional_excess = (non_concessional_contributions - base_ncc_cap).max(0.0);

    ContributionCapAssessment {
        financial_year: financial_year.to_string(),
        total_super_balance,
        concessional_contributions,
        non_concessional_contributions,
        concessional_cap: effective_concessional_cap,
        non_concessional_cap: base_ncc_cap,
        concessional_excess,
        non_concessional_excess,
        carry_forward_available: carry_forward,
        bring_forward_available,
    }
}

/// Validate contribution
pub fn validate_contribution(
    contribution: &Contribution,
    member: &FundMember,
    current_date: NaiveDate,
) -> Result<()> {
    let age = member.age_at(current_date);

    // Work test for ages 67-74 (relaxed from 75)
    if matches!(
        contribution.contribution_type,
        ContributionType::PersonalDeductible | ContributionType::PersonalNonDeductible
    ) && (67..75).contains(&age)
    {
        // Work test applies - 40 hours in 30 consecutive days
        // In practice, would need to verify work test is met
        // This is a simplification
    }

    // Age 75+ restrictions
    if age >= 75 {
        match contribution.contribution_type {
            ContributionType::PersonalNonDeductible | ContributionType::PersonalDeductible => {
                return Err(SuperannuationError::InvalidContribution {
                    reason: "Personal contributions not accepted for members aged 75+".to_string(),
                });
            }
            ContributionType::SpouseContribution => {
                return Err(SuperannuationError::InvalidContribution {
                    reason: "Spouse contributions not accepted for members aged 75+".to_string(),
                });
            }
            _ => {}
        }
    }

    // Downsizer contribution age requirement (65+)
    if contribution.contribution_type == ContributionType::Downsizer && age < 55 {
        return Err(SuperannuationError::InvalidContribution {
            reason: "Downsizer contributions require member to be 55+".to_string(),
        });
    }

    Ok(())
}

/// Calculate due date for SG payment
pub fn sg_due_date(quarter: SgQuarter, financial_year: &str) -> NaiveDate {
    let start_year: i32 = financial_year[0..4].parse().unwrap_or(2024);

    let (year, month) = match quarter {
        SgQuarter::Q1 => (start_year, 10),    // Oct of FY start year
        SgQuarter::Q2 => (start_year + 1, 1), // Jan of FY end year
        SgQuarter::Q3 => (start_year + 1, 4), // Apr of FY end year
        SgQuarter::Q4 => (start_year + 1, 7), // Jul after FY end
    };

    NaiveDate::from_ymd_opt(year, month, 28).unwrap_or_default()
}

/// Check if SG payment is on time
pub fn is_sg_payment_on_time(
    quarter: SgQuarter,
    financial_year: &str,
    payment_date: NaiveDate,
) -> bool {
    let due = sg_due_date(quarter, financial_year);
    payment_date <= due
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_calculate_sg_contribution() {
        let result = calculate_sg_contribution(10_000.0, "2024-25", false);
        assert_eq!(result.sg_rate, 0.115);
        assert_eq!(result.sg_amount, 1150.0);
        assert!(!result.capped_at_maximum);
    }

    #[test]
    fn test_calculate_sg_contribution_capped() {
        let result = calculate_sg_contribution(100_000.0, "2024-25", true);
        assert!(result.capped_at_maximum);
        assert!(result.quarterly_cap.is_some());
    }

    #[test]
    fn test_sg_eligibility_basic() {
        let result = check_sg_eligibility(35, 5000.0, EmploymentType::FullTime);
        assert!(result.is_eligible);
        assert!(result.ote_threshold_met);
    }

    #[test]
    fn test_sg_eligibility_contractor() {
        let result = check_sg_eligibility(35, 5000.0, EmploymentType::ContractorNotEmployee);
        assert!(!result.is_eligible);
    }

    #[test]
    fn test_sg_shortfall_calculation() {
        let result = calculate_sg_shortfall(1150.0, 1000.0, SgQuarter::Q1, "2024-25", 1, 30);
        assert_eq!(result.shortfall, 150.0);
        assert!(result.nominal_interest > 0.0);
        assert_eq!(result.administration_fee, 20.0);
    }

    #[test]
    fn test_contribution_cap_assessment() {
        let result = assess_contribution_caps(
            500_000.0,  // TSB
            25_000.0,   // CC contributions
            50_000.0,   // NCC contributions
            &[5_000.0], // Carry-forward
            "2024-25",
            55,
        );

        assert_eq!(result.concessional_cap, 35_000.0); // 30k + 5k carry-forward
        assert_eq!(result.concessional_excess, 0.0);
        assert!(result.bring_forward_available);
    }

    #[test]
    fn test_sg_due_date() {
        let due = sg_due_date(SgQuarter::Q1, "2024-25");
        assert_eq!(due.year(), 2024);
        assert_eq!(due.month(), 10);
        assert_eq!(due.day(), 28);
    }

    #[test]
    fn test_is_sg_payment_on_time() {
        let on_time = NaiveDate::from_ymd_opt(2024, 10, 25).unwrap();
        let late = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        assert!(is_sg_payment_on_time(SgQuarter::Q1, "2024-25", on_time));
        assert!(!is_sg_payment_on_time(SgQuarter::Q1, "2024-25", late));
    }

    #[test]
    fn test_validate_contribution_age_75() {
        let member = FundMember {
            member_id: "M001".to_string(),
            tfn_provided: true,
            date_of_birth: NaiveDate::from_ymd_opt(1948, 1, 1).unwrap(),
            join_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            category: MemberCategory::Accumulation,
            account_balance: 100_000.0,
            preserved_amount: 100_000.0,
            restricted_non_preserved: 0.0,
            unrestricted_non_preserved: 0.0,
            insurance: None,
            beneficiaries: vec![],
        };

        let contribution = Contribution {
            contribution_type: ContributionType::PersonalNonDeductible,
            amount: 10_000.0,
            date_received: NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
            financial_year: "2024-25".to_string(),
            employer: None,
        };

        let current = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let result = validate_contribution(&contribution, &member, current);
        assert!(result.is_err());
    }
}
