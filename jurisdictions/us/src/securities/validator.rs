//! Securities Law Validators (Securities Act 1933, Securities Exchange Act 1934)
//!
//! Validation functions for securities law compliance

#![allow(missing_docs)]

use super::error::{Result, SecuritiesError};
use super::types::*;
use chrono::{Duration, NaiveDate, Utc};

/// Validate that a security is properly registered or has a valid exemption
pub fn validate_registration(security: &Security) -> Result<()> {
    match &security.registration_status {
        RegistrationStatus::Registered { .. } => Ok(()),
        RegistrationStatus::Exempt { exemption_basis } => {
            if exemption_basis.is_empty() && security.exemptions.is_empty() {
                Err(SecuritiesError::NoValidExemption {
                    offering_amount: security
                        .offering
                        .as_ref()
                        .map(|o| o.offering_size)
                        .unwrap_or(0.0),
                })
            } else {
                Ok(())
            }
        }
        RegistrationStatus::Pending { .. } => Err(SecuritiesError::RegistrationNotEffective {
            filing_date: "pending".to_string(),
        }),
        RegistrationStatus::Required => Err(SecuritiesError::NotRegistered {
            security_name: security.name.clone(),
        }),
        RegistrationStatus::NotApplicable => Ok(()),
    }
}

/// Validate Regulation D offering compliance
pub fn validate_regulation_d(
    offering: &Offering,
    exemption: &Exemption,
    _accredited_count: usize,
    non_accredited_count: usize,
) -> Result<()> {
    if let Exemption::RegulationD {
        rule,
        offering_amount,
        accredited_only,
        general_solicitation,
        filing_form_d,
    } = exemption
    {
        match rule {
            RegulationDRule::Rule504 => {
                // Rule 504: Up to $10 million in 12 months
                if *offering_amount > 10_000_000.0 {
                    return Err(SecuritiesError::RegulationDViolation {
                        rule: "504".to_string(),
                        details: format!(
                            "Offering amount ${} exceeds Rule 504 limit of $10,000,000",
                            offering_amount
                        ),
                        consequence: "Must use different exemption or register".to_string(),
                    });
                }
            }
            RegulationDRule::Rule506B => {
                // Rule 506(b): Unlimited amount, up to 35 non-accredited, no general solicitation
                if non_accredited_count > 35 {
                    return Err(SecuritiesError::RegulationDViolation {
                        rule: "506(b)".to_string(),
                        details: format!(
                            "Number of non-accredited investors ({}) exceeds Rule 506(b) limit of 35",
                            non_accredited_count
                        ),
                        consequence: "Exemption lost; offering may be deemed public offering"
                            .to_string(),
                    });
                }

                if *general_solicitation {
                    return Err(SecuritiesError::GeneralSolicitationProhibited {
                        solicitation_type: "Rule 506(b) offering with general solicitation"
                            .to_string(),
                    });
                }
            }
            RegulationDRule::Rule506C => {
                // Rule 506(c): Unlimited amount, accredited only, general solicitation allowed
                if !*accredited_only || non_accredited_count > 0 {
                    return Err(SecuritiesError::NonAccreditedIn506C {
                        investor_name: "one or more investors".to_string(),
                        verification_method: "insufficient".to_string(),
                    });
                }
            }
        }

        // Form D filing requirement
        if !*filing_form_d && offering.number_of_investors > 0 {
            return Err(SecuritiesError::FormDNotFiled);
        }

        Ok(())
    } else {
        Err(SecuritiesError::ValidationError {
            message: "Exemption is not Regulation D".to_string(),
        })
    }
}

/// Validate Regulation A offering compliance
pub fn validate_regulation_a(_offering: &Offering, exemption: &Exemption) -> Result<()> {
    if let Exemption::RegulationA {
        tier,
        offering_amount,
        offering_circular_qualified,
    } = exemption
    {
        let limit = match tier {
            RegulationATier::Tier1 => 20_000_000.0,
            RegulationATier::Tier2 => 75_000_000.0,
        };

        if *offering_amount > limit {
            return Err(SecuritiesError::RegulationALimitExceeded {
                tier: format!("{:?}", tier),
                max_amount: limit,
                offered_amount: *offering_amount,
            });
        }

        if !*offering_circular_qualified {
            return Err(SecuritiesError::ValidationError {
                message: "Regulation A offering circular not yet qualified by SEC".to_string(),
            });
        }

        Ok(())
    } else {
        Err(SecuritiesError::ValidationError {
            message: "Exemption is not Regulation A".to_string(),
        })
    }
}

/// Validate Regulation Crowdfunding compliance
pub fn validate_crowdfunding(
    offering_amount: f64,
    prior_crowdfunding_amount: f64,
    investor_income: Option<f64>,
    investor_net_worth: Option<f64>,
    investment_amount: f64,
) -> Result<()> {
    // Offering limit: $5 million in 12-month period
    const CROWDFUNDING_LIMIT: f64 = 5_000_000.0;

    if offering_amount + prior_crowdfunding_amount > CROWDFUNDING_LIMIT {
        return Err(SecuritiesError::CrowdfundingLimitExceeded {
            current_amount: offering_amount,
            prior_amount: prior_crowdfunding_amount,
        });
    }

    // Investor investment limits
    let financial_threshold = investor_income
        .or(investor_net_worth)
        .unwrap_or(0.0)
        .max(investor_net_worth.unwrap_or(0.0));

    let investment_limit = if financial_threshold < 124_000.0 {
        // Greater of $2,500 or 5% of the greater of income or net worth
        (financial_threshold * 0.05).max(2_500.0)
    } else {
        // 10% of the greater of income or net worth, not to exceed $124,000
        (financial_threshold * 0.10).min(124_000.0)
    };

    if investment_amount > investment_limit {
        return Err(SecuritiesError::CrowdfundingInvestmentLimitExceeded {
            investor_name: "investor".to_string(),
            financial_threshold,
            limit: investment_limit,
            attempted: investment_amount,
        });
    }

    Ok(())
}

/// Validate accredited investor status
pub fn validate_accredited_investor(investor: &AccreditedInvestor) -> Result<()> {
    if !investor.is_accredited {
        return Err(SecuritiesError::NotAccreditedInvestor {
            investor_name: "investor".to_string(),
            income: None,
            net_worth: None,
        });
    }

    if investor.accreditation_basis.is_empty() {
        return Err(SecuritiesError::InsufficientAccreditedVerification {
            investor_name: "investor".to_string(),
            method: format!("{:?}", investor.verification_method),
        });
    }

    // For Rule 506(c), self-certification is not sufficient
    if matches!(
        investor.verification_method,
        VerificationMethod::SelfCertification
    ) {
        return Err(SecuritiesError::InsufficientAccreditedVerification {
            investor_name: "investor".to_string(),
            method: "Self-certification not sufficient for Rule 506(c)".to_string(),
        });
    }

    Ok(())
}

/// Validate Rule 144 holding period requirement
pub fn validate_rule_144_holding_period(
    acquisition_date: NaiveDate,
    proposed_sale_date: NaiveDate,
    is_reporting_company: bool,
) -> Result<()> {
    let days_held = (proposed_sale_date - acquisition_date).num_days() as u32;
    let required_days = if is_reporting_company { 180 } else { 365 }; // 6 months or 1 year

    if days_held < required_days {
        return Err(SecuritiesError::HoldingPeriodNotSatisfied {
            acquisition_date: acquisition_date.to_string(),
            days_held,
            required_days,
            issuer_type: if is_reporting_company {
                "reporting".to_string()
            } else {
                "non-reporting".to_string()
            },
        });
    }

    Ok(())
}

/// Validate Rule 144 volume limitation
pub fn validate_rule_144_volume(
    shares_to_sell: u64,
    outstanding_shares: u64,
    four_week_average_volume: u64,
) -> Result<()> {
    let one_percent_outstanding = outstanding_shares / 100;
    let volume_limit = one_percent_outstanding.max(four_week_average_volume);

    if shares_to_sell > volume_limit {
        return Err(SecuritiesError::VolumeLimitationExceeded {
            shares_to_sell,
            volume_limit,
        });
    }

    Ok(())
}

/// Validate qualified institutional buyer (QIB) status for Rule 144A
pub fn validate_qib_status(qib: &QualifiedInstitutionalBuyer) -> Result<()> {
    let threshold = match qib.qib_type {
        QibType::BrokerDealer => 10_000_000.0, // $10M for broker-dealers
        _ => 100_000_000.0,                    // $100M for others
    };

    if qib.securities_owned < threshold {
        return Err(SecuritiesError::NotQualifiedInstitutionalBuyer {
            purchaser_name: qib.name.clone(),
        });
    }

    Ok(())
}

/// Validate Howey Test for investment contract analysis
pub fn validate_howey_test(analysis: &HoweyTestAnalysis) -> Result<bool> {
    // All four prongs must be satisfied for it to be a security
    let is_security = analysis.investment_of_money
        && !matches!(
            analysis.common_enterprise,
            CommonEnterpriseType::NoCommonEnterprise
        )
        && analysis.expectation_of_profits
        && analysis.efforts_of_others.essential_efforts_by_others;

    Ok(is_security)
}

/// Validate Section 16(b) short-swing profit violation
pub fn validate_section_16b(
    purchase_date: NaiveDate,
    sale_date: NaiveDate,
    is_insider: bool,
) -> Result<()> {
    if !is_insider {
        return Ok(());
    }

    let days_between = (sale_date - purchase_date).num_days().abs();

    if days_between <= 180 {
        // Within 6 months - potential Section 16(b) violation
        return Err(SecuritiesError::ShortSwingProfit {
            insider_name: "insider".to_string(),
            purchase_date: purchase_date.to_string(),
            sale_date: sale_date.to_string(),
            profit: 0.0, // Would need to calculate actual profit
        });
    }

    Ok(())
}

/// Validate beneficial ownership reporting (13D/13G) requirement
pub fn validate_beneficial_ownership_reporting(
    ownership_percentage: f64,
    has_filed: bool,
) -> Result<()> {
    if ownership_percentage > 5.0 && !has_filed {
        return Err(SecuritiesError::BeneficialOwnershipReportingRequired {
            percentage: ownership_percentage,
        });
    }

    Ok(())
}

/// Validate Exchange Act Section 12 registration requirement
pub fn validate_exchange_act_registration(
    total_assets: f64,
    total_shareholders: usize,
    non_accredited_shareholders: usize,
    is_registered: bool,
) -> Result<()> {
    if total_assets <= 10_000_000.0 {
        return Ok(()); // Below asset threshold
    }

    let requires_registration = total_shareholders >= 2000 || non_accredited_shareholders >= 500;

    if requires_registration && !is_registered {
        return Err(SecuritiesError::MissingExchangeActRegistration {
            threshold_violated: if total_shareholders >= 2000 {
                format!(
                    "2,000+ total shareholders ({} shareholders)",
                    total_shareholders
                )
            } else {
                format!(
                    "500+ non-accredited shareholders ({} non-accredited)",
                    non_accredited_shareholders
                )
            },
        });
    }

    Ok(())
}

/// Validate Investment Company Act Section 3(c)(1) exemption (100 or fewer beneficial owners)
pub fn validate_3c1_exemption(beneficial_owners: usize) -> Result<()> {
    if beneficial_owners > 100 {
        return Err(SecuritiesError::Section3C1ExceededOwners {
            owners: beneficial_owners,
        });
    }

    Ok(())
}

/// Validate blue sky compliance for multi-state offering
pub fn validate_blue_sky_compliance(
    _security: &Security,
    states: &[String],
    blue_sky_status: &[BlueSkyCompliance],
) -> Result<()> {
    for state in states {
        let state_compliance = blue_sky_status.iter().find(|bs| &bs.state == state);

        match state_compliance {
            Some(compliance) => {
                if compliance.registration_required
                    && compliance.state_exemption.is_none()
                    && !compliance.notice_filing_required
                {
                    return Err(SecuritiesError::StateRegistrationRequired {
                        state: state.clone(),
                    });
                }

                if compliance.notice_filing_required
                    && compliance.filing_status.as_deref() != Some("filed")
                {
                    return Err(SecuritiesError::NoticeFilingRequired {
                        state: state.clone(),
                    });
                }
            }
            None => {
                // No blue sky analysis for this state
                return Err(SecuritiesError::ValidationError {
                    message: format!("Blue sky compliance not assessed for state: {}", state),
                });
            }
        }
    }

    Ok(())
}

/// Validate periodic reporting requirement (10-K, 10-Q)
pub fn validate_periodic_reporting(
    form_type: &str,
    period_end: NaiveDate,
    filing_deadline: NaiveDate,
    actual_filing_date: Option<NaiveDate>,
) -> Result<()> {
    match actual_filing_date {
        Some(filed_date) => {
            if filed_date > filing_deadline {
                Err(SecuritiesError::LatePeriodicReportFiling {
                    form_type: form_type.to_string(),
                    period_end: period_end.to_string(),
                    deadline: filing_deadline.to_string(),
                })
            } else {
                Ok(())
            }
        }
        None => {
            if Utc::now().naive_utc().date() > filing_deadline {
                Err(SecuritiesError::LatePeriodicReportFiling {
                    form_type: form_type.to_string(),
                    period_end: period_end.to_string(),
                    deadline: filing_deadline.to_string(),
                })
            } else {
                Ok(())
            }
        }
    }
}

/// Validate insider Form 4 filing timeliness
pub fn validate_form_4_timeliness(
    transaction_date: NaiveDate,
    filing_date: Option<NaiveDate>,
) -> Result<()> {
    let deadline = transaction_date + Duration::days(2); // 2 business days (simplified)

    match filing_date {
        Some(filed) => {
            if filed > deadline {
                Err(SecuritiesError::LateForm4Filing {
                    insider_name: "insider".to_string(),
                    transaction_date: transaction_date.to_string(),
                })
            } else {
                Ok(())
            }
        }
        None => {
            if Utc::now().naive_utc().date() > deadline {
                Err(SecuritiesError::LateForm4Filing {
                    insider_name: "insider".to_string(),
                    transaction_date: transaction_date.to_string(),
                })
            } else {
                Ok(())
            }
        }
    }
}

/// Validate integration of offerings
///
/// SEC considers several factors to determine if multiple offerings should be integrated:
/// 1. Are the offerings part of a single plan of financing?
/// 2. Do the offerings involve the same class of securities?
/// 3. Are the offerings made at or about the same time?
/// 4. Is the same type of consideration received?
/// 5. Are the offerings made for the same general purpose?
pub fn validate_offering_integration(
    current_offering_date: NaiveDate,
    prior_offering_date: NaiveDate,
    same_class: bool,
    same_purpose: bool,
) -> Result<()> {
    let days_between = (current_offering_date - prior_offering_date).num_days();

    // Rule 152 safe harbor: 6 months between offerings
    if days_between < 180 && same_class && same_purpose {
        return Err(SecuritiesError::IntegrationViolation {
            prior_date: prior_offering_date.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_registration_success() {
        let security = Security {
            identifier: "TEST001".to_string(),
            name: "Test Stock".to_string(),
            security_type: SecurityType::CommonStock,
            issuer: Issuer {
                name: "Test Corp".to_string(),
                jurisdiction: "Delaware".to_string(),
                cik: Some("0001234567".to_string()),
                issuer_type: IssuerType::Corporation,
                is_reporting_company: true,
                sic_code: Some("7370".to_string()),
                total_assets: Some(100_000_000.0),
            },
            registration_status: RegistrationStatus::Registered {
                registration_number: "333-123456".to_string(),
                effective_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
                form_type: RegistrationFormType::S1,
            },
            exemptions: vec![],
            issue_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).expect("valid date")),
            offering: None,
            is_restricted: false,
            trading_restrictions: vec![],
        };

        assert!(validate_registration(&security).is_ok());
    }

    #[test]
    fn test_validate_registration_failure() {
        let security = Security {
            identifier: "TEST002".to_string(),
            name: "Unregistered Stock".to_string(),
            security_type: SecurityType::CommonStock,
            issuer: Issuer {
                name: "Startup Inc".to_string(),
                jurisdiction: "Delaware".to_string(),
                cik: None,
                issuer_type: IssuerType::Corporation,
                is_reporting_company: false,
                sic_code: None,
                total_assets: Some(5_000_000.0),
            },
            registration_status: RegistrationStatus::Required,
            exemptions: vec![],
            issue_date: None,
            offering: None,
            is_restricted: true,
            trading_restrictions: vec![],
        };

        assert!(validate_registration(&security).is_err());
    }

    #[test]
    fn test_validate_regulation_d_506b_success() {
        let offering = Offering {
            offering_type: OfferingType::PrivatePlacement,
            offering_size: 5_000_000.0,
            amount_raised: 3_000_000.0,
            number_of_investors: 25,
            offering_start: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            offering_end: None,
            underwriters: vec![],
            use_of_proceeds: Some("Working capital".to_string()),
            minimum_investment: Some(100_000.0),
        };

        let exemption = Exemption::RegulationD {
            rule: RegulationDRule::Rule506B,
            offering_amount: 5_000_000.0,
            accredited_only: false,
            general_solicitation: false,
            filing_form_d: true,
        };

        assert!(validate_regulation_d(&offering, &exemption, 20, 5).is_ok());
    }

    #[test]
    fn test_validate_regulation_d_506b_too_many_non_accredited() {
        let offering = Offering {
            offering_type: OfferingType::PrivatePlacement,
            offering_size: 5_000_000.0,
            amount_raised: 3_000_000.0,
            number_of_investors: 50,
            offering_start: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            offering_end: None,
            underwriters: vec![],
            use_of_proceeds: Some("Working capital".to_string()),
            minimum_investment: Some(50_000.0),
        };

        let exemption = Exemption::RegulationD {
            rule: RegulationDRule::Rule506B,
            offering_amount: 5_000_000.0,
            accredited_only: false,
            general_solicitation: false,
            filing_form_d: true,
        };

        assert!(validate_regulation_d(&offering, &exemption, 10, 40).is_err());
    }

    #[test]
    fn test_validate_crowdfunding_limits() {
        // Should succeed
        assert!(
            validate_crowdfunding(
                3_000_000.0,
                1_000_000.0,
                Some(100_000.0),
                Some(200_000.0),
                5_000.0
            )
            .is_ok()
        );

        // Should fail - exceeds offering limit
        assert!(
            validate_crowdfunding(
                4_000_000.0,
                2_000_000.0,
                Some(100_000.0),
                Some(200_000.0),
                5_000.0
            )
            .is_err()
        );

        // Should fail - exceeds investment limit
        assert!(
            validate_crowdfunding(1_000_000.0, 0.0, Some(50_000.0), Some(60_000.0), 10_000.0)
                .is_err()
        );
    }

    #[test]
    fn test_validate_rule_144_holding_period() {
        let acquisition = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let sale_reporting = NaiveDate::from_ymd_opt(2024, 7, 1).expect("valid date"); // 6 months later
        let sale_non_reporting = NaiveDate::from_ymd_opt(2025, 1, 2).expect("valid date"); // 1 year later

        // Reporting company - 6 months OK
        assert!(validate_rule_144_holding_period(acquisition, sale_reporting, true).is_ok());

        // Non-reporting company - 6 months not enough
        assert!(validate_rule_144_holding_period(acquisition, sale_reporting, false).is_err());

        // Non-reporting company - 1 year OK
        assert!(validate_rule_144_holding_period(acquisition, sale_non_reporting, false).is_ok());
    }

    #[test]
    fn test_validate_section_16b() {
        let purchase = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let sale_within_6mo = NaiveDate::from_ymd_opt(2024, 5, 1).expect("valid date");
        let sale_after_6mo = NaiveDate::from_ymd_opt(2024, 8, 1).expect("valid date");

        // Insider trading within 6 months - violation
        assert!(validate_section_16b(purchase, sale_within_6mo, true).is_err());

        // Insider trading after 6 months - OK
        assert!(validate_section_16b(purchase, sale_after_6mo, true).is_ok());

        // Non-insider - always OK
        assert!(validate_section_16b(purchase, sale_within_6mo, false).is_ok());
    }

    #[test]
    fn test_validate_exchange_act_registration() {
        // Below asset threshold - no registration required
        assert!(validate_exchange_act_registration(5_000_000.0, 3000, 600, false).is_ok());

        // Above thresholds - registration required
        assert!(validate_exchange_act_registration(50_000_000.0, 2500, 400, false).is_err());

        // Above thresholds but registered - OK
        assert!(validate_exchange_act_registration(50_000_000.0, 2500, 400, true).is_ok());
    }

    #[test]
    fn test_validate_3c1_exemption() {
        assert!(validate_3c1_exemption(50).is_ok());
        assert!(validate_3c1_exemption(100).is_ok());
        assert!(validate_3c1_exemption(101).is_err());
    }

    #[test]
    fn test_validate_howey_test() {
        // Classic investment contract - all prongs satisfied
        let security = HoweyTestAnalysis {
            investment_of_money: true,
            common_enterprise: CommonEnterpriseType::HorizontalCommonality,
            expectation_of_profits: true,
            efforts_of_others: EffortsOfOthersAnalysis {
                essential_efforts_by_others: true,
                investor_role: InvestorRole::Passive,
                promoter_control_level: ControlLevel::Complete,
            },
            is_security: true,
            additional_factors: vec![],
        };

        assert!(validate_howey_test(&security).unwrap_or(false));

        // Not a security - investor actively manages
        let not_security = HoweyTestAnalysis {
            investment_of_money: true,
            common_enterprise: CommonEnterpriseType::HorizontalCommonality,
            expectation_of_profits: true,
            efforts_of_others: EffortsOfOthersAnalysis {
                essential_efforts_by_others: false,
                investor_role: InvestorRole::ActiveManagement,
                promoter_control_level: ControlLevel::Shared,
            },
            is_security: false,
            additional_factors: vec![],
        };

        assert!(!validate_howey_test(&not_security).unwrap_or(true));
    }
}
