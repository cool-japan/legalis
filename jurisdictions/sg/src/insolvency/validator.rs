//! Insolvency, Restructuring and Dissolution Act 2018 (IRDA) - Validation Logic
//!
//! Comprehensive validation and assessment for:
//! 1. Corporate winding up - grounds and the "unable to pay debts" deeming tests
//!    (IRDA s. 125)
//! 2. Judicial management applications (IRDA s. 89(1))
//! 3. Schemes of arrangement - class approval thresholds (IRDA s. 210(3AB)) and
//!    the s. 64 moratorium
//! 4. Bankruptcy applications and Debt Repayment Scheme eligibility (IRDA Part 16,
//!    Part 14)

use super::error::{InsolvencyError, Result};
use super::types::*;

/// Outcome of assessing whether a company is deemed unable to pay its debts.
#[derive(Debug, Clone, PartialEq)]
pub struct InabilityAssessment {
    /// Whether the company is deemed unable to pay its debts.
    pub deemed_unable: bool,
    /// The deeming test satisfied (if any).
    pub test: Option<InabilityToPayTest>,
    /// Whether the debt exceeded the prescribed sum.
    pub debt_sufficient: bool,
    /// Whether the 3-week statutory period had expired.
    pub period_expired: bool,
    /// Explanatory notes for the assessment.
    pub notes: Vec<String>,
}

/// Report on the validity of a winding up petition.
#[derive(Debug, Clone)]
pub struct WindingUpReport {
    /// Whether the petition is properly grounded.
    pub is_valid: bool,
    /// Errors preventing the petition from succeeding.
    pub errors: Vec<InsolvencyError>,
    /// Non-fatal warnings to be addressed.
    pub warnings: Vec<String>,
    /// Inability-to-pay assessment (where the ground is s. 125(1)(e)).
    pub inability: Option<InabilityAssessment>,
}

/// Report on the validity of a judicial management application.
#[derive(Debug, Clone)]
pub struct JudicialManagementReport {
    /// Whether the application is properly grounded.
    pub is_grounded: bool,
    /// Whether the insolvency limb is satisfied.
    pub insolvency_limb_satisfied: bool,
    /// Statutory purposes relied upon.
    pub purposes: Vec<JudicialManagementPurpose>,
    /// Errors preventing the application from succeeding.
    pub errors: Vec<InsolvencyError>,
}

/// Report on a scheme of arrangement's class voting and procedural validity.
#[derive(Debug, Clone)]
pub struct SchemeReport {
    /// Whether every class approved the scheme.
    pub all_classes_approved: bool,
    /// Per-class approval outcomes (class name, approved flag).
    pub class_results: Vec<SchemeClassResult>,
    /// Errors arising from the scheme.
    pub errors: Vec<InsolvencyError>,
    /// Warnings to be addressed.
    pub warnings: Vec<String>,
}

/// Per-class result within a [`SchemeReport`].
#[derive(Debug, Clone, PartialEq)]
pub struct SchemeClassResult {
    /// Name of the class.
    pub class_name: String,
    /// Whether the class approved the scheme.
    pub approved: bool,
    /// Whether the majority-in-number test was met.
    pub majority_in_number: bool,
    /// Whether the 75%-in-value test was met.
    pub seventy_five_percent_in_value: bool,
    /// Percentage in value voting in favour.
    pub value_percentage: f64,
}

/// Determines whether a company is deemed unable to pay its debts on the basis
/// of a statutory-demand scenario (IRDA s. 125(2)(a)).
///
/// The company is deemed unable to pay where:
/// - the debt exceeds the prescribed sum (SGD 15,000); AND
/// - the demand has remained unsatisfied for at least 3 weeks (21 days).
///
/// # Parameters
/// - `debt_cents`: the debt demanded, in SGD cents.
/// - `days_unsatisfied`: number of days the demand has been outstanding.
/// - `prescribed_minimum_cents`: the prescribed minimum debt, in SGD cents
///   (ordinarily [`COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS`]).
pub fn assess_statutory_demand_inability(
    debt_cents: u64,
    days_unsatisfied: u32,
    prescribed_minimum_cents: u64,
) -> InabilityAssessment {
    let debt_sufficient = debt_cents > prescribed_minimum_cents;
    let period_expired = days_unsatisfied >= STATUTORY_DEMAND_PERIOD_DAYS;
    let mut notes = Vec::new();

    if !debt_sufficient {
        notes.push(format!(
            "Debt SGD {} does not exceed the prescribed sum of SGD {}",
            debt_cents / 100,
            prescribed_minimum_cents / 100
        ));
    }
    if !period_expired {
        notes.push(format!(
            "Statutory demand outstanding for {} of {} required days",
            days_unsatisfied, STATUTORY_DEMAND_PERIOD_DAYS
        ));
    }

    let deemed_unable = debt_sufficient && period_expired;
    if deemed_unable {
        notes.push("Company deemed unable to pay its debts (IRDA s. 125(2)(a))".to_string());
    }

    InabilityAssessment {
        deemed_unable,
        test: if deemed_unable {
            Some(InabilityToPayTest::StatutoryDemand)
        } else {
            None
        },
        debt_sufficient,
        period_expired,
        notes,
    }
}

/// Validates a statutory demand and returns the inability assessment, or an
/// error explaining why the demand cannot found a winding up.
///
/// # Errors
/// - [`InsolvencyError::DebtBelowPrescribedSum`] if the debt is too small.
/// - [`InsolvencyError::StatutoryDemandNotRipe`] if the 3-week period has not
///   expired.
pub fn validate_statutory_demand(demand: &StatutoryDemand) -> Result<InabilityAssessment> {
    if demand.disputed {
        return Err(InsolvencyError::ValidationError {
            message: format!(
                "Debt is disputed on substantial grounds; a statutory demand is inappropriate for {}",
                demand.company_name
            ),
        });
    }

    if !demand.exceeds_prescribed_sum() {
        return Err(InsolvencyError::DebtBelowPrescribedSum {
            debt_sgd: demand.debt_in_sgd(),
            minimum_sgd: COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS / 100,
        });
    }

    if !demand.period_expired() {
        return Err(InsolvencyError::StatutoryDemandNotRipe {
            days_elapsed: demand.days_unsatisfied,
            days_required: STATUTORY_DEMAND_PERIOD_DAYS,
        });
    }

    Ok(assess_statutory_demand_inability(
        demand.debt_cents,
        demand.days_unsatisfied,
        COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS,
    ))
}

/// Validates a winding up petition, producing a detailed report.
///
/// # Checks Performed
/// 1. Compulsory winding up requires a ground under s. 125(1).
/// 2. Where the ground is s. 125(1)(e) and a statutory demand is relied upon,
///    the demand must be valid.
/// 3. A members' voluntary winding up requires a declaration of solvency
///    (s. 161).
pub fn validate_winding_up_petition(petition: &WindingUpPetition) -> Result<WindingUpReport> {
    let mut report = WindingUpReport {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        inability: None,
    };

    match petition.mode {
        WindingUpMode::CompulsoryByCourt => {
            let Some(ground) = petition.ground else {
                report.is_valid = false;
                report.errors.push(InsolvencyError::NoWindingUpGround {
                    reason: "Compulsory winding up requires a ground under s. 125(1)".to_string(),
                });
                return Ok(report);
            };

            if ground == WindingUpGround::UnableToPayDebts {
                if let Some(ref demand) = petition.statutory_demand {
                    match validate_statutory_demand(demand) {
                        Ok(assessment) => {
                            report.inability = Some(assessment);
                        }
                        Err(error) => {
                            report.is_valid = false;
                            report.errors.push(error);
                        }
                    }
                } else if petition.inability_test.is_none() {
                    report.warnings.push(
                        "Ground is s. 125(1)(e) but no inability-to-pay test was specified"
                            .to_string(),
                    );
                }
            }
        }
        WindingUpMode::MembersVoluntary => {
            if !petition.declaration_of_solvency {
                report.is_valid = false;
                report
                    .errors
                    .push(InsolvencyError::MissingDeclarationOfSolvency);
            }
        }
        WindingUpMode::CreditorsVoluntary => {
            if petition.declaration_of_solvency {
                report.warnings.push(
                    "Creditors' voluntary winding up but a declaration of solvency was made; \
                     consider a members' voluntary winding up instead"
                        .to_string(),
                );
            }
        }
    }

    Ok(report)
}

/// Validates a judicial management application (IRDA s. 89(1)).
///
/// The application must satisfy both limbs:
/// 1. the company is, or is likely to become, unable to pay its debts; AND
/// 2. there is a reasonable probability of achieving at least one statutory
///    purpose.
pub fn validate_judicial_management(
    application: &JudicialManagementApplication,
) -> Result<JudicialManagementReport> {
    let mut report = JudicialManagementReport {
        is_grounded: true,
        insolvency_limb_satisfied: application.is_or_likely_unable_to_pay,
        purposes: application.purposes.clone(),
        errors: Vec::new(),
    };

    if !application.is_or_likely_unable_to_pay {
        report.is_grounded = false;
        report
            .errors
            .push(InsolvencyError::JudicialManagementNotGrounded {
                reason: "Company is not, and is not likely to become, unable to pay its debts"
                    .to_string(),
            });
    }

    if application.purposes.is_empty() {
        report.is_grounded = false;
        report
            .errors
            .push(InsolvencyError::NoJudicialManagementPurpose);
    }

    Ok(report)
}

/// Determines whether a single scheme class has approved the scheme of
/// arrangement (IRDA s. 210(3AB)).
///
/// Approval requires BOTH:
/// - a **majority in number** of the creditors present and voting; AND
/// - **75% in value** of the creditors present and voting.
///
/// # Parameters
/// - `votes_in_favour_count`: number of creditors voting in favour.
/// - `total_voting_count`: total number of creditors present and voting.
/// - `value_in_favour_cents`: value (in SGD cents) voting in favour.
/// - `total_value_cents`: total value (in SGD cents) present and voting.
///
/// # Returns
/// `true` only if both the majority-in-number and the 75%-in-value tests are met.
pub fn scheme_class_approved(
    votes_in_favour_count: u32,
    total_voting_count: u32,
    value_in_favour_cents: u64,
    total_value_cents: u64,
) -> bool {
    if total_voting_count == 0 || total_value_cents == 0 {
        return false;
    }

    // Majority in number: strictly more than half of those present and voting.
    let majority_in_number = (votes_in_favour_count as u64) * 2 > (total_voting_count as u64);

    // 75% in value: value in favour is at least three quarters of the total.
    // Use integer arithmetic to avoid floating-point error at the boundary:
    //   value_in_favour >= 0.75 * total  <=>  4 * value_in_favour >= 3 * total.
    let seventy_five_percent =
        value_in_favour_cents.saturating_mul(4) >= total_value_cents.saturating_mul(3);

    majority_in_number && seventy_five_percent
}

/// Validates a scheme of arrangement, assessing each class and the moratorium.
///
/// # Checks Performed
/// 1. The scheme must have at least one creditor class (s. 210).
/// 2. Each class must satisfy the majority-in-number and 75%-in-value tests
///    (s. 210(3AB)); a cram-down (s. 70) can rescue a dissenting class.
/// 3. An automatic moratorium beyond 30 days requires a Court extension (s. 64).
pub fn validate_scheme_of_arrangement(scheme: &SchemeOfArrangement) -> Result<SchemeReport> {
    let mut report = SchemeReport {
        all_classes_approved: true,
        class_results: Vec::new(),
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    if scheme.classes.is_empty() {
        report.all_classes_approved = false;
        report.errors.push(InsolvencyError::SchemeHasNoClasses);
        return Ok(report);
    }

    // Moratorium check (s. 64): the automatic moratorium is 30 days.
    if scheme.moratorium_in_force && scheme.moratorium_days > AUTOMATIC_MORATORIUM_DAYS {
        report.warnings.push(format!(
            "Moratorium of {} days exceeds the automatic {}-day period and requires a Court extension (IRDA s. 64)",
            scheme.moratorium_days, AUTOMATIC_MORATORIUM_DAYS
        ));
    }

    for class in &scheme.classes {
        let voting_count = class.voting_count();
        let in_favour_count = class.in_favour_count();
        let total_value = class.total_voting_value_cents();
        let in_favour_value = class.in_favour_value_cents();

        let majority_in_number = (in_favour_count as u64) * 2 > (voting_count as u64);
        let value_threshold = in_favour_value.saturating_mul(4) >= total_value.saturating_mul(3);
        let value_percentage = if total_value == 0 {
            0.0
        } else {
            (in_favour_value as f64 / total_value as f64) * 100.0
        };

        let approved = majority_in_number && value_threshold;

        if !approved {
            if scheme.seeks_cram_down {
                report.warnings.push(format!(
                    "Class '{}' did not approve, but a cram-down under s. 70 is sought",
                    class.class_name
                ));
            } else {
                report.all_classes_approved = false;
                if !majority_in_number {
                    report
                        .errors
                        .push(InsolvencyError::SchemeMajorityInNumberFailed {
                            in_favour: in_favour_count,
                            total: voting_count,
                        });
                }
                if !value_threshold {
                    report
                        .errors
                        .push(InsolvencyError::SchemeValueThresholdFailed {
                            percentage: value_percentage,
                        });
                }
            }
        }

        report.class_results.push(SchemeClassResult {
            class_name: class.class_name.clone(),
            approved,
            majority_in_number,
            seventy_five_percent_in_value: value_threshold,
            value_percentage,
        });
    }

    Ok(report)
}

/// Validates the moratorium length sought on a scheme application (IRDA s. 64).
///
/// # Errors
/// - [`InsolvencyError::MoratoriumPeriodExceeded`] if the requested period
///   exceeds the automatic 30-day period (a Court extension would be required).
pub fn validate_moratorium_period(days: u32) -> Result<()> {
    if days > AUTOMATIC_MORATORIUM_DAYS {
        return Err(InsolvencyError::MoratoriumPeriodExceeded {
            days,
            limit: AUTOMATIC_MORATORIUM_DAYS,
        });
    }
    Ok(())
}

/// Validates a bankruptcy application (IRDA Part 16).
///
/// # Checks Performed (creditor's application)
/// 1. The debt must be a liquidated sum of at least SGD 15,000 (s. 311(1)(a)).
/// 2. The debtor must be unable to pay, ordinarily evidenced by an unsatisfied
///    statutory demand (s. 311(1)(c)).
///
/// A debtor's own application is not subject to the creditor's debt threshold.
pub fn validate_bankruptcy_application(application: &BankruptcyApplication) -> Result<()> {
    match application.applicant {
        BankruptcyApplicant::Creditor => {
            if !application.meets_debt_threshold() {
                return Err(InsolvencyError::BankruptcyDebtBelowThreshold {
                    debt_sgd: application.debt_in_sgd(),
                    threshold_sgd: BANKRUPTCY_DEBT_THRESHOLD_CENTS / 100,
                });
            }

            if !application.debtor_unable_to_pay && !application.statutory_demand_unsatisfied {
                return Err(InsolvencyError::DebtorNotShownUnableToPay);
            }
        }
        BankruptcyApplicant::DebtorOwn => {
            // A debtor's own application turns on inability to pay, not on the
            // creditor's minimum-debt threshold.
            if !application.debtor_unable_to_pay {
                return Err(InsolvencyError::DebtorNotShownUnableToPay);
            }
        }
    }

    Ok(())
}

/// Assesses a debtor's eligibility for the Debt Repayment Scheme (IRDA s. 289).
///
/// # Eligibility
/// - aggregate debts must not exceed SGD 150,000; AND
/// - the debtor must not be an undischarged bankrupt.
///
/// # Errors
/// - [`InsolvencyError::DebtRepaymentSchemeIneligible`] if the debt ceiling is
///   exceeded.
/// - [`InsolvencyError::ValidationError`] if the debtor is an undischarged
///   bankrupt or lacks a regular income.
pub fn assess_debt_repayment_scheme(profile: &DebtRepaymentSchemeProfile) -> Result<()> {
    if profile.is_undischarged_bankrupt {
        return Err(InsolvencyError::ValidationError {
            message: format!(
                "{} is an undischarged bankrupt and cannot use the Debt Repayment Scheme",
                profile.debtor_name
            ),
        });
    }

    if !profile.within_debt_ceiling() {
        return Err(InsolvencyError::DebtRepaymentSchemeIneligible {
            debt_sgd: profile.aggregate_debt_in_sgd(),
            ceiling_sgd: DEBT_REPAYMENT_SCHEME_CEILING_CENTS / 100,
        });
    }

    if !profile.has_regular_income {
        return Err(InsolvencyError::ValidationError {
            message: format!(
                "{} has no regular income; the Debt Repayment Scheme requires a means of repayment",
                profile.debtor_name
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_statutory_demand_inability_deemed() {
        let assessment = assess_statutory_demand_inability(
            2_000_000,
            21,
            COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS,
        );
        assert!(assessment.deemed_unable);
        assert_eq!(assessment.test, Some(InabilityToPayTest::StatutoryDemand));
        assert!(assessment.debt_sufficient);
        assert!(assessment.period_expired);
    }

    #[test]
    fn test_assess_statutory_demand_inability_debt_too_small() {
        let assessment = assess_statutory_demand_inability(
            1_000_000,
            30,
            COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS,
        );
        assert!(!assessment.deemed_unable);
        assert!(!assessment.debt_sufficient);
        assert!(assessment.period_expired);
        assert!(assessment.test.is_none());
    }

    #[test]
    fn test_assess_statutory_demand_inability_period_not_expired() {
        let assessment = assess_statutory_demand_inability(
            2_000_000,
            10,
            COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS,
        );
        assert!(!assessment.deemed_unable);
        assert!(assessment.debt_sufficient);
        assert!(!assessment.period_expired);
    }

    #[test]
    fn test_validate_statutory_demand_ok() {
        let demand = StatutoryDemand::new("sd1", "Bank A", "Acme Pte Ltd", 2_000_000)
            .with_days_unsatisfied(25);
        let assessment = validate_statutory_demand(&demand).expect("should be valid");
        assert!(assessment.deemed_unable);
    }

    #[test]
    fn test_validate_statutory_demand_below_sum() {
        let demand = StatutoryDemand::new("sd2", "Bank A", "Acme Pte Ltd", 500_000)
            .with_days_unsatisfied(30);
        match validate_statutory_demand(&demand) {
            Err(InsolvencyError::DebtBelowPrescribedSum {
                debt_sgd,
                minimum_sgd,
            }) => {
                assert_eq!(debt_sgd, 5_000);
                assert_eq!(minimum_sgd, 15_000);
            }
            other => panic!("expected DebtBelowPrescribedSum, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_statutory_demand_not_ripe() {
        let demand = StatutoryDemand::new("sd3", "Bank A", "Acme Pte Ltd", 2_000_000)
            .with_days_unsatisfied(7);
        match validate_statutory_demand(&demand) {
            Err(InsolvencyError::StatutoryDemandNotRipe {
                days_elapsed,
                days_required,
            }) => {
                assert_eq!(days_elapsed, 7);
                assert_eq!(days_required, 21);
            }
            other => panic!("expected StatutoryDemandNotRipe, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_statutory_demand_disputed() {
        let demand = StatutoryDemand::new("sd4", "Bank A", "Acme Pte Ltd", 2_000_000)
            .with_days_unsatisfied(30)
            .with_dispute(true);
        assert!(matches!(
            validate_statutory_demand(&demand),
            Err(InsolvencyError::ValidationError { .. })
        ));
    }

    #[test]
    fn test_validate_winding_up_compulsory_no_ground() {
        let petition =
            WindingUpPetition::new("p1", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt);
        let report = validate_winding_up_petition(&petition).expect("returns report");
        assert!(!report.is_valid);
        assert!(matches!(
            report.errors[0],
            InsolvencyError::NoWindingUpGround { .. }
        ));
    }

    #[test]
    fn test_validate_winding_up_compulsory_with_demand() {
        let demand = StatutoryDemand::new("sd1", "Bank A", "Acme Pte Ltd", 3_000_000)
            .with_days_unsatisfied(25);
        let petition =
            WindingUpPetition::new("p2", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt)
                .with_ground(WindingUpGround::UnableToPayDebts)
                .with_statutory_demand(demand);
        let report = validate_winding_up_petition(&petition).expect("returns report");
        assert!(report.is_valid);
        assert!(report.inability.expect("inability present").deemed_unable);
    }

    #[test]
    fn test_validate_winding_up_members_voluntary_no_declaration() {
        let petition =
            WindingUpPetition::new("p3", "Solvent Pte Ltd", WindingUpMode::MembersVoluntary);
        let report = validate_winding_up_petition(&petition).expect("returns report");
        assert!(!report.is_valid);
        assert!(matches!(
            report.errors[0],
            InsolvencyError::MissingDeclarationOfSolvency
        ));
    }

    #[test]
    fn test_validate_winding_up_members_voluntary_with_declaration() {
        let petition =
            WindingUpPetition::new("p4", "Solvent Pte Ltd", WindingUpMode::MembersVoluntary)
                .with_declaration_of_solvency(true);
        let report = validate_winding_up_petition(&petition).expect("returns report");
        assert!(report.is_valid);
    }

    #[test]
    fn test_validate_judicial_management_grounded() {
        let app = JudicialManagementApplication::new("jm1", "Distressed Pte Ltd")
            .with_insolvency_limb(true)
            .with_purpose(JudicialManagementPurpose::SurvivalAsGoingConcern);
        let report = validate_judicial_management(&app).expect("returns report");
        assert!(report.is_grounded);
        assert!(report.insolvency_limb_satisfied);
        assert_eq!(report.purposes.len(), 1);
    }

    #[test]
    fn test_validate_judicial_management_no_purpose() {
        let app = JudicialManagementApplication::new("jm2", "Distressed Pte Ltd")
            .with_insolvency_limb(true);
        let report = validate_judicial_management(&app).expect("returns report");
        assert!(!report.is_grounded);
        assert!(
            report
                .errors
                .iter()
                .any(|e| matches!(e, InsolvencyError::NoJudicialManagementPurpose))
        );
    }

    #[test]
    fn test_validate_judicial_management_no_insolvency_limb() {
        let app = JudicialManagementApplication::new("jm3", "Healthy Pte Ltd")
            .with_purpose(JudicialManagementPurpose::MoreAdvantageousRealisation);
        let report = validate_judicial_management(&app).expect("returns report");
        assert!(!report.is_grounded);
        assert!(
            report
                .errors
                .iter()
                .any(|e| matches!(e, InsolvencyError::JudicialManagementNotGrounded { .. }))
        );
    }

    #[test]
    fn test_scheme_class_approved_both_tests_met() {
        // 3 of 4 in number (majority), value 800k of 1000k (80% >= 75%).
        assert!(scheme_class_approved(3, 4, 800_000, 1_000_000));
    }

    #[test]
    fn test_scheme_class_approved_exact_75_percent() {
        // Value exactly 75%: 750k of 1000k, majority in number.
        assert!(scheme_class_approved(3, 4, 750_000, 1_000_000));
        // Just below 75%: fails the value test.
        assert!(!scheme_class_approved(3, 4, 749_999, 1_000_000));
    }

    #[test]
    fn test_scheme_class_approved_fails_number() {
        // Value is high (90%) but only 2 of 4 voted (not a majority).
        assert!(!scheme_class_approved(2, 4, 900_000, 1_000_000));
    }

    #[test]
    fn test_scheme_class_approved_fails_value() {
        // 3 of 4 (majority) but value only 60%.
        assert!(!scheme_class_approved(3, 4, 600_000, 1_000_000));
    }

    #[test]
    fn test_scheme_class_approved_zero_voters() {
        assert!(!scheme_class_approved(0, 0, 0, 0));
    }

    #[test]
    fn test_validate_scheme_all_classes_approved() {
        let scheme = SchemeOfArrangement::new("s1", "Restructure Pte Ltd").with_class(
            SchemeClass::new("Unsecured")
                .with_creditor(SchemeCreditor::new("A", 800_000).with_vote(true))
                .with_creditor(SchemeCreditor::new("B", 100_000).with_vote(true))
                .with_creditor(SchemeCreditor::new("C", 100_000).with_vote(false)),
        );
        let report = validate_scheme_of_arrangement(&scheme).expect("returns report");
        assert!(report.all_classes_approved);
        assert_eq!(report.class_results.len(), 1);
        assert!(report.class_results[0].approved);
    }

    #[test]
    fn test_validate_scheme_class_rejected() {
        let scheme = SchemeOfArrangement::new("s2", "Restructure Pte Ltd").with_class(
            SchemeClass::new("Unsecured")
                .with_creditor(SchemeCreditor::new("A", 600_000).with_vote(false))
                .with_creditor(SchemeCreditor::new("B", 400_000).with_vote(true)),
        );
        let report = validate_scheme_of_arrangement(&scheme).expect("returns report");
        assert!(!report.all_classes_approved);
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn test_validate_scheme_cram_down_rescues_class() {
        let scheme = SchemeOfArrangement::new("s3", "Restructure Pte Ltd")
            .with_cram_down(true)
            .with_class(
                SchemeClass::new("Dissenting")
                    .with_creditor(SchemeCreditor::new("A", 600_000).with_vote(false))
                    .with_creditor(SchemeCreditor::new("B", 400_000).with_vote(true)),
            );
        let report = validate_scheme_of_arrangement(&scheme).expect("returns report");
        // With cram-down sought, the dissenting class does not fail the scheme.
        assert!(report.all_classes_approved);
        assert!(report.errors.is_empty());
        assert!(!report.warnings.is_empty());
    }

    #[test]
    fn test_validate_scheme_no_classes() {
        let scheme = SchemeOfArrangement::new("s4", "Empty Pte Ltd");
        let report = validate_scheme_of_arrangement(&scheme).expect("returns report");
        assert!(!report.all_classes_approved);
        assert!(matches!(
            report.errors[0],
            InsolvencyError::SchemeHasNoClasses
        ));
    }

    #[test]
    fn test_validate_scheme_moratorium_warning() {
        let scheme = SchemeOfArrangement::new("s5", "Restructure Pte Ltd")
            .with_moratorium(60)
            .with_class(
                SchemeClass::new("Unsecured")
                    .with_creditor(SchemeCreditor::new("A", 1_000_000).with_vote(true)),
            );
        let report = validate_scheme_of_arrangement(&scheme).expect("returns report");
        assert!(
            report
                .warnings
                .iter()
                .any(|w| w.contains("moratorium") || w.contains("Moratorium"))
        );
    }

    #[test]
    fn test_validate_moratorium_period_ok() {
        assert!(validate_moratorium_period(30).is_ok());
        assert!(validate_moratorium_period(15).is_ok());
    }

    #[test]
    fn test_validate_moratorium_period_exceeded() {
        match validate_moratorium_period(45) {
            Err(InsolvencyError::MoratoriumPeriodExceeded { days, limit }) => {
                assert_eq!(days, 45);
                assert_eq!(limit, 30);
            }
            other => panic!("expected MoratoriumPeriodExceeded, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_bankruptcy_creditor_ok() {
        let app =
            BankruptcyApplication::new("b1", "John Tan", BankruptcyApplicant::Creditor, 2_000_000)
                .with_creditor_name("Bank A")
                .with_unsatisfied_statutory_demand(true);
        assert!(validate_bankruptcy_application(&app).is_ok());
    }

    #[test]
    fn test_validate_bankruptcy_creditor_below_threshold() {
        let app =
            BankruptcyApplication::new("b2", "John Tan", BankruptcyApplicant::Creditor, 1_000_000)
                .with_inability_to_pay(true);
        match validate_bankruptcy_application(&app) {
            Err(InsolvencyError::BankruptcyDebtBelowThreshold {
                debt_sgd,
                threshold_sgd,
            }) => {
                assert_eq!(debt_sgd, 10_000);
                assert_eq!(threshold_sgd, 15_000);
            }
            other => panic!("expected BankruptcyDebtBelowThreshold, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_bankruptcy_creditor_not_unable() {
        let app =
            BankruptcyApplication::new("b3", "John Tan", BankruptcyApplicant::Creditor, 2_000_000);
        assert!(matches!(
            validate_bankruptcy_application(&app),
            Err(InsolvencyError::DebtorNotShownUnableToPay)
        ));
    }

    #[test]
    fn test_validate_bankruptcy_debtor_own_ignores_threshold() {
        // A debtor's own application below the creditor threshold is fine if the
        // debtor is unable to pay.
        let app =
            BankruptcyApplication::new("b4", "John Tan", BankruptcyApplicant::DebtorOwn, 500_000)
                .with_inability_to_pay(true);
        assert!(validate_bankruptcy_application(&app).is_ok());
    }

    #[test]
    fn test_assess_drs_eligible() {
        let profile = DebtRepaymentSchemeProfile::new("Debtor A", 10_000_000);
        assert!(assess_debt_repayment_scheme(&profile).is_ok());
    }

    #[test]
    fn test_assess_drs_over_ceiling() {
        let profile = DebtRepaymentSchemeProfile::new("Debtor B", 20_000_000);
        match assess_debt_repayment_scheme(&profile) {
            Err(InsolvencyError::DebtRepaymentSchemeIneligible {
                debt_sgd,
                ceiling_sgd,
            }) => {
                assert_eq!(debt_sgd, 200_000);
                assert_eq!(ceiling_sgd, 150_000);
            }
            other => panic!("expected DebtRepaymentSchemeIneligible, got {:?}", other),
        }
    }

    #[test]
    fn test_assess_drs_undischarged_bankrupt() {
        let profile =
            DebtRepaymentSchemeProfile::new("Debtor C", 5_000_000).with_undischarged_bankrupt(true);
        assert!(matches!(
            assess_debt_repayment_scheme(&profile),
            Err(InsolvencyError::ValidationError { .. })
        ));
    }

    #[test]
    fn test_assess_drs_no_income() {
        let profile =
            DebtRepaymentSchemeProfile::new("Debtor D", 5_000_000).with_regular_income(false);
        assert!(matches!(
            assess_debt_repayment_scheme(&profile),
            Err(InsolvencyError::ValidationError { .. })
        ));
    }

    #[test]
    fn test_validation_performance_loop() {
        // A typical validation must complete quickly. We run 1000 iterations and
        // assert they all complete (no timing assertion to avoid flakiness).
        let demand =
            StatutoryDemand::new("perf", "Bank", "Co", 2_000_000).with_days_unsatisfied(25);
        let mut deemed = 0u32;
        for _ in 0..1_000 {
            let assessment = validate_statutory_demand(&demand).expect("valid");
            if assessment.deemed_unable {
                deemed += 1;
            }
        }
        assert_eq!(deemed, 1_000);
    }
}
