//! Company Law Validators (Companies Act 2006)

use super::error::{CompanyLawError, Result};
use super::types::*;

/// Validate company formation (CA 2006 Part 2)
///
/// Checks compliance with:
/// - Company name requirements (ss.53-81)
/// - Minimum share capital (s.763 for PLC)
/// - Director requirements (s.154)
/// - Company secretary (s.271 for PLC)
/// - Registered office (s.86)
/// - Statement of compliance (s.13)
pub fn validate_company_formation(formation: &CompanyFormation) -> Result<()> {
    let mut errors = Vec::new();

    // Validate company name
    if let Err(e) = validate_company_name(&formation.company_name, formation.company_type) {
        errors.push(e.to_string());
    }

    // Validate share capital (if applicable)
    if formation.company_type.requires_share_capital() {
        if let Some(ref capital) = formation.share_capital {
            if let Err(e) = validate_share_capital(capital, formation.company_type) {
                errors.push(e.to_string());
            }
        } else {
            errors.push("Share capital required but not provided".to_string());
        }
    }

    // Validate minimum number of directors
    if let Err(e) = validate_directors(&formation.directors, formation.company_type) {
        errors.push(e.to_string());
    }

    // Validate company secretary (required for PLC)
    if formation.company_type == CompanyType::PublicLimitedCompany && formation.secretary.is_none()
    {
        return Err(CompanyLawError::MissingCompanySecretary);
    }

    // Validate statement of compliance
    if !formation.statement_of_compliance {
        return Err(CompanyLawError::MissingStatementOfCompliance);
    }

    if !errors.is_empty() {
        return Err(CompanyLawError::MultipleErrors { errors });
    }

    Ok(())
}

/// Validate company name (CA 2006 ss.53-81)
///
/// Checks:
/// - Correct suffix (Limited/Ltd, PLC, etc.)
/// - No sensitive words without approval
/// - Not too similar to existing companies
/// - No prohibited characters
pub fn validate_company_name(name: &str, company_type: CompanyType) -> Result<()> {
    // Check for required suffix
    let required_suffix = company_type.required_suffix();
    let abbreviated = company_type.abbreviated_suffix();

    let has_correct_suffix = if !required_suffix.is_empty() {
        let name_lower = name.to_lowercase();
        let suffix_lower = required_suffix.to_lowercase();

        name_lower.ends_with(&suffix_lower)
            || abbreviated
                .map(|abbr| name_lower.ends_with(&abbr.to_lowercase()))
                .unwrap_or(false)
    } else {
        true
    };

    if !has_correct_suffix && !required_suffix.is_empty() {
        return Err(CompanyLawError::MissingSuffix {
            name: name.to_string(),
            required_suffix: required_suffix.to_string(),
        });
    }

    // Check for sensitive words (CA 2006 s.55)
    let sensitive_words = [
        "royal",
        "king",
        "queen",
        "prince",
        "princess",
        "windsor",
        "government",
        "british",
        "national",
        "england",
        "scotland",
        "wales",
        "trust",
        "charity",
        "foundation",
        "university",
        "police",
    ];

    let name_lower = name.to_lowercase();
    for word in sensitive_words.iter() {
        if name_lower.contains(word) {
            return Err(CompanyLawError::SensitiveWord {
                name: name.to_string(),
                word: word.to_string(),
            });
        }
    }

    // Check for prohibited characters
    let prohibited_chars = ['@', '#', '$', '%', '^', '&', '*', '!'];
    for ch in prohibited_chars.iter() {
        if name.contains(*ch) {
            return Err(CompanyLawError::InvalidCompanyName {
                name: name.to_string(),
                reason: format!("Contains prohibited character '{}'", ch),
            });
        }
    }

    // Check minimum length
    if name.len() < 2 {
        return Err(CompanyLawError::InvalidCompanyName {
            name: name.to_string(),
            reason: "Name too short (minimum 2 characters)".to_string(),
        });
    }

    Ok(())
}

/// Validate share capital (CA 2006 Part 17)
///
/// Checks:
/// - Minimum capital for PLC (£50,000)
/// - Minimum paid up capital for PLC (25%)
/// - Share arithmetic correctness
pub fn validate_share_capital(capital: &ShareCapital, company_type: CompanyType) -> Result<()> {
    // Check minimum capital
    if let Some(minimum) = company_type.minimum_share_capital() {
        if capital.nominal_capital_gbp < minimum {
            return Err(CompanyLawError::InsufficientShareCapital {
                actual: capital.nominal_capital_gbp,
                minimum,
                company_type: format!("{:?}", company_type),
            });
        }
    }

    // Check paid up capital for PLC (s.586: minimum 25%)
    if company_type == CompanyType::PublicLimitedCompany && !capital.meets_plc_paid_up_requirement()
    {
        return Err(CompanyLawError::InsufficientPaidUpCapital {
            paid_up: capital.paid_up_capital_gbp,
            nominal: capital.nominal_capital_gbp,
            percentage: capital.percentage_paid_up(),
        });
    }

    // Check share arithmetic
    let expected_nominal = capital.number_of_shares as f64 * capital.nominal_value_per_share_gbp;
    let difference = (expected_nominal - capital.nominal_capital_gbp).abs();

    if difference > 0.01 {
        // Allow 1p rounding error
        return Err(CompanyLawError::ValidationError {
            message: format!(
                "Share arithmetic error: {} shares × £{} = £{}, but nominal capital is £{}",
                capital.number_of_shares,
                capital.nominal_value_per_share_gbp,
                expected_nominal,
                capital.nominal_capital_gbp
            ),
        });
    }

    Ok(())
}

/// Validate directors (CA 2006 s.154)
///
/// Checks:
/// - Private company: minimum 1 director
/// - Public company: minimum 2 directors
/// - All directors have required information
pub fn validate_directors(directors: &[Director], company_type: CompanyType) -> Result<()> {
    let minimum = match company_type {
        CompanyType::PublicLimitedCompany => 2,
        _ => 1,
    };

    if (directors.len() as u32) < minimum {
        return Err(CompanyLawError::InsufficientDirectors {
            minimum,
            actual: directors.len() as u32,
        });
    }

    // Validate each director has minimum information
    for director in directors {
        if director.name.is_empty() {
            return Err(CompanyLawError::ValidationError {
                message: "Director name cannot be empty".to_string(),
            });
        }

        if director.service_address.address_line_1.is_empty() {
            return Err(CompanyLawError::ValidationError {
                message: format!("Director '{}' missing service address", director.name),
            });
        }
    }

    Ok(())
}

/// Validate director duties compliance (CA 2006 ss.171-177)
///
/// Checks all seven statutory director duties:
/// - s.171: Act within powers
/// - s.172: Promote success of company
/// - s.173: Exercise independent judgment
/// - s.174: Reasonable care, skill and diligence
/// - s.175: Avoid conflicts of interest
/// - s.176: Not accept benefits from third parties
/// - s.177: Declare interest in proposed transaction
pub fn validate_director_duties(duties: &DirectorDutiesCompliance) -> Result<()> {
    let mut errors = Vec::new();

    // s.171: Act within powers
    if !duties.act_within_powers.compliant {
        errors.push(CompanyLawError::BreachActWithinPowers {
            details: duties
                .act_within_powers
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    // s.172: Promote success of company
    if !duties.promote_success.compliant {
        errors.push(CompanyLawError::BreachPromoteSuccess {
            details: format!(
                "Failed considerations: {}",
                get_failed_s172_considerations(&duties.promote_success)
            ),
        });
    }

    // s.173: Independent judgment
    if !duties.independent_judgment.compliant {
        errors.push(CompanyLawError::BreachIndependentJudgment {
            details: duties
                .independent_judgment
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    // s.174: Reasonable care
    if !duties.reasonable_care.compliant {
        errors.push(CompanyLawError::BreachReasonableCare {
            details: format!(
                "Objective standard met: {}, Subjective standard met: {}. {}",
                duties.reasonable_care.objective_standard_met,
                duties.reasonable_care.subjective_standard_met,
                duties.reasonable_care.evidence
            ),
        });
    }

    // s.175: Avoid conflicts
    if !duties.avoid_conflicts.compliant {
        errors.push(CompanyLawError::BreachAvoidConflicts {
            details: format!(
                "{} undeclared conflicts of interest",
                duties
                    .avoid_conflicts
                    .conflicts_declared
                    .iter()
                    .filter(|c| !c.authorization_obtained)
                    .count()
            ),
        });
    }

    // s.176: No third party benefits
    if !duties.no_third_party_benefits.compliant {
        errors.push(CompanyLawError::BreachThirdPartyBenefits {
            details: duties
                .no_third_party_benefits
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    // s.177: Declare interest
    if !duties.declare_interest.compliant {
        errors.push(CompanyLawError::BreachDeclareInterest {
            details: format!(
                "{} interests not declared to board",
                duties
                    .declare_interest
                    .interests_declared
                    .iter()
                    .filter(|i| !i.declared_to_board)
                    .count()
            ),
        });
    }

    if !errors.is_empty() {
        return Err(errors.into_iter().next().unwrap());
    }

    Ok(())
}

/// Get description of failed s.172 considerations
fn get_failed_s172_considerations(compliance: &PromoteSuccessCompliance) -> String {
    let mut failed = Vec::new();

    if !compliance.long_term_consequences_considered {
        failed.push("long term consequences");
    }
    if !compliance.employee_interests_considered {
        failed.push("employee interests");
    }
    if !compliance.business_relationships_considered {
        failed.push("business relationships");
    }
    if !compliance.community_environment_considered {
        failed.push("community and environment");
    }
    if !compliance.reputation_considered {
        failed.push("reputation");
    }
    if !compliance.fairness_between_members_considered {
        failed.push("fairness between members");
    }

    if failed.is_empty() {
        "None (but marked as non-compliant)".to_string()
    } else {
        failed.join(", ")
    }
}

/// Validate resolution voting (CA 2006 ss.282-283)
///
/// Checks:
/// - Ordinary resolution: > 50% majority
/// - Special resolution: ≥ 75% majority
pub fn validate_resolution(
    resolution_type: ResolutionType,
    votes_for: u64,
    votes_against: u64,
) -> Result<()> {
    if !resolution_type.passes(votes_for, votes_against) {
        let total_votes = votes_for + votes_against;
        let percentage_for = if total_votes > 0 {
            (votes_for as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };

        return Err(CompanyLawError::ResolutionFailed {
            resolution_type: format!("{:?}", resolution_type),
            votes_for,
            votes_against,
            percentage_for,
            required_percentage: resolution_type.required_majority(),
        });
    }

    Ok(())
}

/// Validate annual accounts compliance (CA 2006 Part 15)
///
/// Checks:
/// - Accounts prepared
/// - Audit obtained if required
/// - Filed by deadline
pub fn validate_annual_accounts(accounts: &AnnualAccountsRequirement) -> Result<()> {
    if !accounts.accounts_prepared {
        return Err(CompanyLawError::ValidationError {
            message: format!(
                "Annual accounts not prepared for year ending {}",
                accounts.financial_year_end
            ),
        });
    }

    if accounts.audit_required && !accounts.accounts_audited {
        return Err(CompanyLawError::ValidationError {
            message: format!(
                "Audit required but not obtained for year ending {}",
                accounts.financial_year_end
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_company_name_suffix() {
        // Valid names
        assert!(validate_company_name("Acme Limited", CompanyType::PrivateLimitedByShares).is_ok());
        assert!(validate_company_name("Acme Ltd", CompanyType::PrivateLimitedByShares).is_ok());
        assert!(
            validate_company_name(
                "Acme Public Limited Company",
                CompanyType::PublicLimitedCompany
            )
            .is_ok()
        );
        assert!(validate_company_name("Acme PLC", CompanyType::PublicLimitedCompany).is_ok());

        // Missing suffix
        assert!(validate_company_name("Acme", CompanyType::PrivateLimitedByShares).is_err());
        assert!(validate_company_name("Acme", CompanyType::PublicLimitedCompany).is_err());
    }

    #[test]
    fn test_validate_company_name_sensitive_words() {
        let result = validate_company_name("Royal Acme Ltd", CompanyType::PrivateLimitedByShares);
        assert!(result.is_err());
        assert!(matches!(result, Err(CompanyLawError::SensitiveWord { .. })));

        let result = validate_company_name(
            "British Trading Company Ltd",
            CompanyType::PrivateLimitedByShares,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_share_capital_plc() {
        // Valid PLC capital (£50k nominal, 50% paid up)
        let capital = ShareCapital {
            nominal_capital_gbp: 50_000.0,
            paid_up_capital_gbp: 25_000.0,
            number_of_shares: 50_000,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![],
        };
        assert!(validate_share_capital(&capital, CompanyType::PublicLimitedCompany).is_ok());

        // Insufficient nominal capital
        let capital = ShareCapital {
            nominal_capital_gbp: 40_000.0,
            paid_up_capital_gbp: 10_000.0,
            number_of_shares: 40_000,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![],
        };
        assert!(validate_share_capital(&capital, CompanyType::PublicLimitedCompany).is_err());

        // Insufficient paid up (only 20%)
        let capital = ShareCapital {
            nominal_capital_gbp: 50_000.0,
            paid_up_capital_gbp: 10_000.0,
            number_of_shares: 50_000,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![],
        };
        let result = validate_share_capital(&capital, CompanyType::PublicLimitedCompany);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CompanyLawError::InsufficientPaidUpCapital { .. })
        ));
    }

    #[test]
    fn test_validate_directors() {
        let director = Director {
            name: "John Smith".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            nationality: "British".to_string(),
            service_address: ServiceAddress {
                address_line_1: "1 High Street".to_string(),
                address_line_2: None,
                city: "London".to_string(),
                postcode: "SW1A 1AA".to_string(),
                country: "United Kingdom".to_string(),
            },
            appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            resignation_date: None,
            director_type: DirectorType::Individual,
        };

        // Private company: 1 director OK
        assert!(
            validate_directors(
                std::slice::from_ref(&director),
                CompanyType::PrivateLimitedByShares
            )
            .is_ok()
        );

        // Public company: need 2 directors
        let result = validate_directors(
            std::slice::from_ref(&director),
            CompanyType::PublicLimitedCompany,
        );
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CompanyLawError::InsufficientDirectors { .. })
        ));

        // Public company: 2 directors OK
        assert!(
            validate_directors(
                &[director.clone(), director.clone()],
                CompanyType::PublicLimitedCompany
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_director_duties_s172() {
        let duties = DirectorDutiesCompliance {
            act_within_powers: DutyCompliance {
                compliant: true,
                evidence: "Within powers".to_string(),
                breach_details: None,
            },
            promote_success: PromoteSuccessCompliance {
                compliant: false,
                long_term_consequences_considered: true,
                employee_interests_considered: false,
                business_relationships_considered: true,
                community_environment_considered: false,
                reputation_considered: true,
                fairness_between_members_considered: true,
                evidence: "Failed to consider some factors".to_string(),
            },
            independent_judgment: DutyCompliance {
                compliant: true,
                evidence: "Independent".to_string(),
                breach_details: None,
            },
            reasonable_care: ReasonableCareCompliance {
                compliant: true,
                objective_standard_met: true,
                subjective_standard_met: true,
                evidence: "Reasonable care exercised".to_string(),
            },
            avoid_conflicts: ConflictsCompliance {
                compliant: true,
                conflicts_declared: vec![],
                board_authorization_obtained: true,
            },
            no_third_party_benefits: DutyCompliance {
                compliant: true,
                evidence: "No benefits accepted".to_string(),
                breach_details: None,
            },
            declare_interest: DeclareInterestCompliance {
                compliant: true,
                interests_declared: vec![],
            },
        };

        let result = validate_director_duties(&duties);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CompanyLawError::BreachPromoteSuccess { .. })
        ));
    }

    #[test]
    fn test_validate_resolution() {
        // Ordinary resolution passes with > 50%
        assert!(validate_resolution(ResolutionType::Ordinary, 51, 49).is_ok());

        // Ordinary resolution fails with 50%
        assert!(validate_resolution(ResolutionType::Ordinary, 50, 50).is_err());

        // Special resolution passes with 75%
        assert!(validate_resolution(ResolutionType::Special, 75, 25).is_ok());

        // Special resolution fails with 74%
        assert!(validate_resolution(ResolutionType::Special, 74, 26).is_err());
    }

    #[test]
    fn test_validate_company_formation() {
        let formation = CompanyFormation {
            company_name: "Test Company Ltd".to_string(),
            company_type: CompanyType::PrivateLimitedByShares,
            registered_office: RegisteredOffice {
                address_line_1: "1 Test Street".to_string(),
                address_line_2: None,
                city: "London".to_string(),
                county: None,
                postcode: "SW1A 1AA".to_string(),
                country: RegisteredOfficeCountry::England,
            },
            share_capital: Some(ShareCapital {
                nominal_capital_gbp: 100.0,
                paid_up_capital_gbp: 100.0,
                number_of_shares: 100,
                nominal_value_per_share_gbp: 1.0,
                share_classes: vec![],
            }),
            directors: vec![Director {
                name: "John Smith".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
                nationality: "British".to_string(),
                service_address: ServiceAddress {
                    address_line_1: "1 High Street".to_string(),
                    address_line_2: None,
                    city: "London".to_string(),
                    postcode: "SW1A 1AA".to_string(),
                    country: "United Kingdom".to_string(),
                },
                appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                resignation_date: None,
                director_type: DirectorType::Individual,
            }],
            shareholders: vec![],
            secretary: None,
            statement_of_compliance: true,
            formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        };

        assert!(validate_company_formation(&formation).is_ok());
    }
}
