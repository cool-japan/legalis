//! UK Family Law - Divorce and Dissolution
//!
//! Implementation of divorce and civil partnership dissolution under:
//! - Divorce, Dissolution and Separation Act 2020 (no-fault divorce)
//! - Matrimonial Causes Act 1973
//! - Civil Partnership Act 2004
//!
//! # Key Changes under DDSA 2020
//!
//! The Divorce, Dissolution and Separation Act 2020 came into force on 6 April 2022
//! and fundamentally changed divorce law in England and Wales:
//!
//! - **No-fault divorce**: No need to prove behaviour, adultery, desertion, or separation
//! - **Irretrievable breakdown**: Statement of irretrievable breakdown is conclusive
//! - **Joint applications**: Either sole or joint applications permitted
//! - **Minimum timescale**: 20-week reflection period + 6 weeks before final order
//! - **New terminology**: "Decree nisi" → "Conditional order", "Decree absolute" → "Final order"
//! - **No defence**: Respondent cannot contest the divorce (except on jurisdiction/validity)

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::error::{FamilyLawError, Result};
use super::types::{Marriage, PersonDetails, RelationshipType};

// ============================================================================
// Divorce Application Types
// ============================================================================

/// Type of divorce/dissolution application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationType {
    /// Sole application (one party applies)
    Sole,
    /// Joint application (both parties apply together)
    Joint,
}

/// Stage of divorce proceedings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceStage {
    /// Application submitted
    ApplicationSubmitted,
    /// 20-week period running
    ReflectionPeriod,
    /// Conditional order applied for
    ConditionalOrderApplied,
    /// Conditional order granted
    ConditionalOrderGranted,
    /// Final order applied for
    FinalOrderApplied,
    /// Final order granted (divorce complete)
    FinalOrderGranted,
}

/// Divorce application under DDSA 2020
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivorceApplication {
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Application type (sole or joint)
    pub application_type: ApplicationType,
    /// Applicant 1 (or sole applicant)
    pub applicant1: PersonDetails,
    /// Applicant 2 (joint applications only)
    pub applicant2: Option<PersonDetails>,
    /// Respondent (sole applications only)
    pub respondent: Option<PersonDetails>,
    /// Date of marriage/civil partnership
    pub relationship_date: NaiveDate,
    /// Date application submitted
    pub application_date: NaiveDate,
    /// Statement of irretrievable breakdown made
    pub statement_of_breakdown: bool,
    /// Current stage
    pub stage: DivorceStage,
    /// Key dates
    pub dates: DivorceDates,
    /// Financial remedy application
    pub financial_remedy_claim: bool,
    /// Children of the family
    pub children: Vec<String>,
    /// Jurisdiction basis
    pub jurisdiction: JurisdictionBasis,
}

/// Key dates in divorce proceedings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivorceDates {
    /// Application submission date
    pub application_date: NaiveDate,
    /// Date 20-week period ends
    pub reflection_period_end: Option<NaiveDate>,
    /// Conditional order application date
    pub conditional_order_applied: Option<NaiveDate>,
    /// Conditional order granted date
    pub conditional_order_granted: Option<NaiveDate>,
    /// Final order application date
    pub final_order_applied: Option<NaiveDate>,
    /// Final order granted date
    pub final_order_granted: Option<NaiveDate>,
}

/// Jurisdiction basis for divorce (Brussels II / domestic rules)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JurisdictionBasis {
    /// Both habitually resident in England & Wales
    BothHabituallyResident,
    /// Last habitually resident, one still resident
    LastHabitualResident,
    /// Respondent habitually resident
    RespondentHabituallyResident,
    /// Applicant habitually resident (12 months)
    ApplicantHabituallyResident12Months,
    /// Applicant habitually resident (6 months + domiciled)
    ApplicantHabituallyResident6MonthsDomiciled,
    /// Both domiciled in England & Wales
    BothDomiciled,
    /// Applicant domiciled
    ApplicantDomiciled,
}

// ============================================================================
// Divorce Analysis
// ============================================================================

/// Analysis of divorce application validity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivorceApplicationAnalysis {
    /// Is application valid?
    pub valid: bool,
    /// Marriage duration requirements met?
    pub marriage_duration_met: bool,
    /// Statement of breakdown made?
    pub statement_of_breakdown: bool,
    /// Jurisdiction established?
    pub jurisdiction_established: bool,
    /// Application type
    pub application_type: ApplicationType,
    /// Issues identified
    pub issues: Vec<String>,
    /// Key dates calculated
    pub key_dates: CalculatedDates,
    /// Analysis
    pub analysis: String,
}

/// Calculated key dates for divorce
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatedDates {
    /// Earliest conditional order date (20 weeks from application)
    pub earliest_conditional_order: NaiveDate,
    /// Earliest final order date (26 weeks from application)
    pub earliest_final_order: NaiveDate,
}

impl DivorceApplicationAnalysis {
    /// Analyze a divorce application
    pub fn analyze(
        relationship_date: NaiveDate,
        application_date: NaiveDate,
        application_type: ApplicationType,
        statement_of_breakdown: bool,
        jurisdiction: &JurisdictionBasis,
    ) -> Self {
        let mut issues = Vec::new();

        // Check marriage duration (minimum 1 year under MCA 1973 s.3)
        let marriage_years = application_date.year() - relationship_date.year();
        let marriage_duration_met = if marriage_years > 1 {
            true
        } else if marriage_years == 1 {
            application_date.ordinal() >= relationship_date.ordinal()
        } else {
            false
        };

        if !marriage_duration_met {
            issues.push(
                "Marriage must be at least one year old before divorce application \
                 (MCA 1973 s.3)."
                    .to_string(),
            );
        }

        // Check statement of breakdown
        if !statement_of_breakdown {
            issues
                .push("Statement of irretrievable breakdown required (DDSA 2020 s.1).".to_string());
        }

        // Check jurisdiction
        let jurisdiction_established = matches!(
            jurisdiction,
            JurisdictionBasis::BothHabituallyResident
                | JurisdictionBasis::LastHabitualResident
                | JurisdictionBasis::RespondentHabituallyResident
                | JurisdictionBasis::ApplicantHabituallyResident12Months
                | JurisdictionBasis::ApplicantHabituallyResident6MonthsDomiciled
                | JurisdictionBasis::BothDomiciled
                | JurisdictionBasis::ApplicantDomiciled
        );

        // Calculate key dates
        let earliest_conditional_order = add_weeks(application_date, 20);
        let earliest_final_order = add_weeks(application_date, 26);

        let key_dates = CalculatedDates {
            earliest_conditional_order,
            earliest_final_order,
        };

        let valid = marriage_duration_met && statement_of_breakdown && jurisdiction_established;

        let analysis = if valid {
            format!(
                "Divorce application VALID under DDSA 2020. {:?} application. \
                 Earliest conditional order: {}. Earliest final order: {}.",
                application_type, earliest_conditional_order, earliest_final_order
            )
        } else {
            format!("Divorce application INVALID. Issues: {}", issues.join(" "))
        };

        Self {
            valid,
            marriage_duration_met,
            statement_of_breakdown,
            jurisdiction_established,
            application_type,
            issues,
            key_dates,
            analysis,
        }
    }
}

/// Add weeks to a date
fn add_weeks(date: NaiveDate, weeks: i64) -> NaiveDate {
    date + chrono::Duration::weeks(weeks)
}

// ============================================================================
// Conditional Order Analysis
// ============================================================================

/// Analysis of conditional order application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalOrderAnalysis {
    /// Can conditional order be granted?
    pub can_be_granted: bool,
    /// 20-week period observed?
    pub reflection_period_complete: bool,
    /// Weeks elapsed since application
    pub weeks_elapsed: i64,
    /// All procedural requirements met?
    pub procedural_requirements_met: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Analysis
    pub analysis: String,
}

impl ConditionalOrderAnalysis {
    /// Analyze conditional order application
    pub fn analyze(
        application_date: NaiveDate,
        conditional_order_date: NaiveDate,
        application_type: &ApplicationType,
        service_effected: bool,
        acknowledgement_filed: bool,
    ) -> Self {
        let mut issues = Vec::new();

        // Calculate weeks elapsed
        let days_elapsed = (conditional_order_date - application_date).num_days();
        let weeks_elapsed = days_elapsed / 7;

        // Check 20-week reflection period
        let reflection_period_complete = weeks_elapsed >= 20;
        if !reflection_period_complete {
            issues.push(format!(
                "20-week reflection period not complete. Only {} weeks elapsed.",
                weeks_elapsed
            ));
        }

        // Check procedural requirements for sole applications
        let procedural_requirements_met = match application_type {
            ApplicationType::Sole => {
                let mut met = true;
                if !service_effected {
                    issues.push("Application not served on respondent.".to_string());
                    met = false;
                }
                if !acknowledgement_filed {
                    // Note: acknowledgement not strictly required if served
                    issues.push(
                        "Acknowledgement of service not filed (may proceed if service proved)."
                            .to_string(),
                    );
                }
                met
            }
            ApplicationType::Joint => true, // No service required for joint
        };

        let can_be_granted = reflection_period_complete && procedural_requirements_met;

        let analysis = if can_be_granted {
            format!(
                "Conditional order CAN be granted. {} weeks since application (20 required). \
                 All procedural requirements met.",
                weeks_elapsed
            )
        } else {
            format!(
                "Conditional order CANNOT be granted yet. Issues: {}",
                issues.join(" ")
            )
        };

        Self {
            can_be_granted,
            reflection_period_complete,
            weeks_elapsed,
            procedural_requirements_met,
            issues,
            analysis,
        }
    }
}

// ============================================================================
// Final Order Analysis
// ============================================================================

/// Analysis of final order application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinalOrderAnalysis {
    /// Can final order be granted?
    pub can_be_granted: bool,
    /// Conditional order granted?
    pub conditional_order_granted: bool,
    /// 6-week period observed?
    pub six_week_period_complete: bool,
    /// Weeks since conditional order
    pub weeks_since_conditional: i64,
    /// Financial remedies resolved?
    pub financial_matters_resolved: FinancialMattersStatus,
    /// Issues
    pub issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Analysis
    pub analysis: String,
}

/// Status of financial matters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinancialMattersStatus {
    /// No claim made
    NoClaimMade,
    /// Claim pending
    ClaimPending,
    /// Consent order made
    ConsentOrderMade,
    /// Contested order made
    ContestedOrderMade,
    /// Dismissed
    Dismissed,
}

impl FinalOrderAnalysis {
    /// Analyze final order application
    pub fn analyze(
        conditional_order_granted: bool,
        conditional_order_date: Option<NaiveDate>,
        final_order_date: NaiveDate,
        application_type: &ApplicationType,
        financial_matters: FinancialMattersStatus,
        is_respondent_applying: bool,
    ) -> Self {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check conditional order granted
        if !conditional_order_granted {
            issues.push("Conditional order not yet granted.".to_string());
        }

        // Calculate weeks since conditional order
        let weeks_since_conditional = conditional_order_date
            .map(|co_date| (final_order_date - co_date).num_days() / 7)
            .unwrap_or(0);

        // Check 6-week period
        let six_week_period_complete = weeks_since_conditional >= 6;
        if !six_week_period_complete && conditional_order_granted {
            issues.push(format!(
                "6-week period not complete. Only {} weeks since conditional order.",
                weeks_since_conditional
            ));
        }

        // Special rules for respondent applying (sole applications)
        if is_respondent_applying && *application_type == ApplicationType::Sole {
            // Respondent can apply after 3 months from earliest date applicant could apply
            let months_since_earliest = weeks_since_conditional / 4;
            if months_since_earliest < 3 {
                issues.push(
                    "Respondent cannot apply for final order until 3 months after \
                     earliest date applicant could have applied."
                        .to_string(),
                );
            }
        }

        // Warn about financial matters
        match financial_matters {
            FinancialMattersStatus::ClaimPending => {
                warnings.push(
                    "Financial remedy claim pending. Consider whether to delay final order \
                     to protect pension rights and other financial claims."
                        .to_string(),
                );
            }
            FinancialMattersStatus::NoClaimMade => {
                warnings.push(
                    "No financial remedy claim made. Final order will end right to claim \
                     financial remedies (unless variation application later)."
                        .to_string(),
                );
            }
            _ => {}
        }

        let can_be_granted =
            conditional_order_granted && six_week_period_complete && issues.is_empty();

        let analysis = if can_be_granted {
            "Final order CAN be granted. All requirements met. Divorce will be complete."
                .to_string()
        } else {
            format!(
                "Final order CANNOT be granted. Issues: {}",
                issues.join(" ")
            )
        };

        Self {
            can_be_granted,
            conditional_order_granted,
            six_week_period_complete,
            weeks_since_conditional,
            financial_matters_resolved: financial_matters,
            issues,
            warnings,
            analysis,
        }
    }
}

// ============================================================================
// Divorce Timeline
// ============================================================================

/// Complete divorce timeline analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivorceTimeline {
    /// Application date
    pub application_date: NaiveDate,
    /// Reflection period end (20 weeks)
    pub reflection_period_end: NaiveDate,
    /// Earliest conditional order
    pub earliest_conditional_order: NaiveDate,
    /// Earliest final order (6 weeks after conditional)
    pub earliest_final_order: NaiveDate,
    /// Minimum total duration in weeks
    pub minimum_weeks: u32,
    /// Analysis
    pub analysis: String,
}

impl DivorceTimeline {
    /// Calculate divorce timeline from application date
    pub fn calculate(application_date: NaiveDate) -> Self {
        let reflection_period_end = add_weeks(application_date, 20);
        let earliest_conditional_order = reflection_period_end;
        let earliest_final_order = add_weeks(earliest_conditional_order, 6);

        let minimum_weeks = 26;

        let analysis = format!(
            "Divorce timeline from {}:\n\
             - Application submitted: {}\n\
             - 20-week reflection period ends: {}\n\
             - Earliest conditional order: {}\n\
             - 6-week period after conditional order\n\
             - Earliest final order: {}\n\
             - Minimum duration: {} weeks",
            application_date,
            application_date,
            reflection_period_end,
            earliest_conditional_order,
            earliest_final_order,
            minimum_weeks
        );

        Self {
            application_date,
            reflection_period_end,
            earliest_conditional_order,
            earliest_final_order,
            minimum_weeks,
            analysis,
        }
    }
}

// ============================================================================
// Marriage/Civil Partnership Validity
// ============================================================================

/// Analysis of marriage validity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarriageValidityAnalysis {
    /// Is marriage valid?
    pub valid: bool,
    /// Is marriage void (never existed)?
    pub void: bool,
    /// Is marriage voidable (valid until annulled)?
    pub voidable: bool,
    /// Grounds for invalidity
    pub invalidity_grounds: Vec<String>,
    /// Can marriage be annulled?
    pub can_be_annulled: bool,
    /// Time bar applies (voidable marriages - 3 years)?
    pub time_barred: bool,
    /// Analysis
    pub analysis: String,
}

impl MarriageValidityAnalysis {
    /// Analyze marriage validity
    pub fn analyze(marriage: &Marriage, current_date: NaiveDate) -> Self {
        let mut invalidity_grounds = Vec::new();
        let mut void = false;
        let mut voidable = false;

        if let Some(ref ground) = marriage.invalidity_grounds {
            match ground {
                // Void marriages (MCA 1973 s.11)
                super::types::MarriageInvalidityGround::ProhibitedDegrees => {
                    invalidity_grounds.push(
                        "Void - parties within prohibited degrees (MA 1949 s.1, MCA 1973 s.11(a)(i))"
                            .to_string(),
                    );
                    void = true;
                }
                super::types::MarriageInvalidityGround::AlreadyMarried => {
                    invalidity_grounds.push(
                        "Void - either party already married (MA 1949 s.1, MCA 1973 s.11(b))"
                            .to_string(),
                    );
                    void = true;
                }
                super::types::MarriageInvalidityGround::UnderageWithoutConsent => {
                    invalidity_grounds.push(
                        "Void - party under 18 (Marriage and Civil Partnership (Minimum Age) Act 2022)"
                            .to_string(),
                    );
                    void = true;
                }
                super::types::MarriageInvalidityGround::FormalityDefect => {
                    invalidity_grounds.push(
                        "Void - non-compliance with formalities (MA 1949 s.49, MCA 1973 s.11(a)(iii))"
                            .to_string(),
                    );
                    void = true;
                }
                super::types::MarriageInvalidityGround::SameSexPreLegalization => {
                    invalidity_grounds.push(
                        "Void - parties not male and female (pre-Marriage (Same Sex Couples) Act 2013)"
                            .to_string(),
                    );
                    void = true;
                }
                // Voidable marriages (MCA 1973 s.12)
                super::types::MarriageInvalidityGround::NonConsummation => {
                    invalidity_grounds
                        .push("Voidable - non-consummation (MCA 1973 s.12(a)/(b))".to_string());
                    voidable = true;
                }
                super::types::MarriageInvalidityGround::LackOfConsent => {
                    invalidity_grounds.push(
                        "Voidable - lack of valid consent (MCA 1973 s.12(c)) - duress, mistake, \
                         unsoundness of mind, or otherwise"
                            .to_string(),
                    );
                    voidable = true;
                }
                super::types::MarriageInvalidityGround::MentalDisorder => {
                    invalidity_grounds.push(
                        "Voidable - mental disorder unfitting for marriage (MCA 1973 s.12(d))"
                            .to_string(),
                    );
                    voidable = true;
                }
                super::types::MarriageInvalidityGround::PregnancyByAnother => {
                    invalidity_grounds.push(
                        "Voidable - respondent pregnant by another at time of marriage (MCA 1973 s.12(f))"
                            .to_string(),
                    );
                    voidable = true;
                }
                super::types::MarriageInvalidityGround::GenderRecognitionIssue => {
                    invalidity_grounds.push(
                        "Voidable - gender recognition issue (MCA 1973 s.12(g)/(h))".to_string(),
                    );
                    voidable = true;
                }
            }
        }

        // Check time bar for voidable marriages (3 years from marriage, with exceptions)
        let years_since_marriage = current_date.year() - marriage.date.year();
        let time_barred = voidable && years_since_marriage > 3;

        let can_be_annulled = void || (voidable && !time_barred);
        let valid = !void && !voidable;

        let analysis = if valid {
            "Marriage is VALID. No grounds for annulment identified.".to_string()
        } else if void {
            format!(
                "Marriage is VOID (never existed in law). Grounds: {}. \
                 Decree of nullity can be obtained at any time.",
                invalidity_grounds.join("; ")
            )
        } else if voidable && time_barred {
            format!(
                "Marriage was VOIDABLE but is now TIME-BARRED (over 3 years). \
                 Grounds: {}. Marriage remains valid unless other exceptions apply.",
                invalidity_grounds.join("; ")
            )
        } else {
            format!(
                "Marriage is VOIDABLE (valid until annulled). Grounds: {}. \
                 Decree of nullity can be sought within 3 years of marriage \
                 (subject to exceptions).",
                invalidity_grounds.join("; ")
            )
        };

        Self {
            valid,
            void,
            voidable,
            invalidity_grounds,
            can_be_annulled,
            time_barred,
            analysis,
        }
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate divorce application
pub fn validate_divorce_application(application: &DivorceApplication) -> Result<()> {
    // Check marriage duration (1 year minimum)
    let years = application.application_date.year() - application.relationship_date.year();
    let duration_ok = if years > 1 {
        true
    } else if years == 1 {
        application.application_date.ordinal() >= application.relationship_date.ordinal()
    } else {
        false
    };

    if !duration_ok {
        return Err(FamilyLawError::MarriageUnderOneYear {
            marriage_date: application.relationship_date.to_string(),
            application_date: application.application_date.to_string(),
        });
    }

    // Check statement of breakdown
    if !application.statement_of_breakdown {
        return Err(FamilyLawError::NoStatementOfBreakdown);
    }

    Ok(())
}

/// Validate conditional order application timing
pub fn validate_conditional_order_timing(
    application_date: NaiveDate,
    conditional_order_date: NaiveDate,
) -> Result<()> {
    let weeks = (conditional_order_date - application_date).num_days() / 7;
    if weeks < 20 {
        return Err(FamilyLawError::ApplicationPeriodNotObserved {
            weeks_elapsed: weeks as u32,
            required_weeks: 20,
        });
    }
    Ok(())
}

/// Validate final order application timing
pub fn validate_final_order_timing(
    conditional_order_granted: bool,
    conditional_order_date: Option<NaiveDate>,
    final_order_date: NaiveDate,
) -> Result<()> {
    if !conditional_order_granted {
        return Err(FamilyLawError::ConditionalOrderNotObtained);
    }

    if let Some(co_date) = conditional_order_date {
        let weeks = (final_order_date - co_date).num_days() / 7;
        if weeks < 6 {
            return Err(FamilyLawError::FinalOrderTooEarly {
                weeks_since_conditional: weeks as u32,
            });
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    #[test]
    fn test_divorce_timeline_calculation() {
        let application_date = test_date(2024, 1, 1);
        let timeline = DivorceTimeline::calculate(application_date);

        assert_eq!(timeline.minimum_weeks, 26);
        assert_eq!(timeline.reflection_period_end, test_date(2024, 5, 20));
        assert_eq!(timeline.earliest_conditional_order, test_date(2024, 5, 20));
        assert_eq!(timeline.earliest_final_order, test_date(2024, 7, 1));
    }

    #[test]
    fn test_divorce_application_analysis_valid() {
        let analysis = DivorceApplicationAnalysis::analyze(
            test_date(2020, 6, 15),
            test_date(2024, 1, 1),
            ApplicationType::Sole,
            true,
            &JurisdictionBasis::BothHabituallyResident,
        );

        assert!(analysis.valid);
        assert!(analysis.marriage_duration_met);
        assert!(analysis.statement_of_breakdown);
        assert!(analysis.jurisdiction_established);
        assert!(analysis.issues.is_empty());
    }

    #[test]
    fn test_divorce_application_under_one_year() {
        let analysis = DivorceApplicationAnalysis::analyze(
            test_date(2023, 6, 15),
            test_date(2024, 1, 1),
            ApplicationType::Sole,
            true,
            &JurisdictionBasis::BothHabituallyResident,
        );

        assert!(!analysis.valid);
        assert!(!analysis.marriage_duration_met);
        assert!(!analysis.issues.is_empty());
    }

    #[test]
    fn test_conditional_order_analysis_valid() {
        let analysis = ConditionalOrderAnalysis::analyze(
            test_date(2024, 1, 1),
            test_date(2024, 5, 27),
            &ApplicationType::Sole,
            true,
            true,
        );

        assert!(analysis.can_be_granted);
        assert!(analysis.reflection_period_complete);
        assert!(analysis.weeks_elapsed >= 20);
    }

    #[test]
    fn test_conditional_order_too_early() {
        let analysis = ConditionalOrderAnalysis::analyze(
            test_date(2024, 1, 1),
            test_date(2024, 3, 1),
            &ApplicationType::Sole,
            true,
            true,
        );

        assert!(!analysis.can_be_granted);
        assert!(!analysis.reflection_period_complete);
    }

    #[test]
    fn test_final_order_analysis_valid() {
        let analysis = FinalOrderAnalysis::analyze(
            true,
            Some(test_date(2024, 5, 20)),
            test_date(2024, 7, 8),
            &ApplicationType::Sole,
            FinancialMattersStatus::ConsentOrderMade,
            false,
        );

        assert!(analysis.can_be_granted);
        assert!(analysis.conditional_order_granted);
        assert!(analysis.six_week_period_complete);
    }

    #[test]
    fn test_final_order_too_early() {
        let analysis = FinalOrderAnalysis::analyze(
            true,
            Some(test_date(2024, 5, 20)),
            test_date(2024, 6, 1),
            &ApplicationType::Sole,
            FinancialMattersStatus::NoClaimMade,
            false,
        );

        assert!(!analysis.can_be_granted);
        assert!(!analysis.six_week_period_complete);
    }

    #[test]
    fn test_validate_divorce_application_valid() {
        let application = DivorceApplication {
            relationship_type: RelationshipType::Marriage,
            application_type: ApplicationType::Sole,
            applicant1: PersonDetails {
                name: "John Smith".to_string(),
                date_of_birth: None,
                gender: None,
                address: None,
                occupation: None,
                ni_number: None,
            },
            applicant2: None,
            respondent: Some(PersonDetails {
                name: "Jane Smith".to_string(),
                date_of_birth: None,
                gender: None,
                address: None,
                occupation: None,
                ni_number: None,
            }),
            relationship_date: test_date(2020, 6, 15),
            application_date: test_date(2024, 1, 1),
            statement_of_breakdown: true,
            stage: DivorceStage::ApplicationSubmitted,
            dates: DivorceDates {
                application_date: test_date(2024, 1, 1),
                reflection_period_end: None,
                conditional_order_applied: None,
                conditional_order_granted: None,
                final_order_applied: None,
                final_order_granted: None,
            },
            financial_remedy_claim: true,
            children: vec![],
            jurisdiction: JurisdictionBasis::BothHabituallyResident,
        };

        assert!(validate_divorce_application(&application).is_ok());
    }

    #[test]
    fn test_validate_divorce_application_under_one_year() {
        let application = DivorceApplication {
            relationship_type: RelationshipType::Marriage,
            application_type: ApplicationType::Sole,
            applicant1: PersonDetails {
                name: "John Smith".to_string(),
                date_of_birth: None,
                gender: None,
                address: None,
                occupation: None,
                ni_number: None,
            },
            applicant2: None,
            respondent: None,
            relationship_date: test_date(2023, 9, 1),
            application_date: test_date(2024, 1, 1),
            statement_of_breakdown: true,
            stage: DivorceStage::ApplicationSubmitted,
            dates: DivorceDates {
                application_date: test_date(2024, 1, 1),
                reflection_period_end: None,
                conditional_order_applied: None,
                conditional_order_granted: None,
                final_order_applied: None,
                final_order_granted: None,
            },
            financial_remedy_claim: false,
            children: vec![],
            jurisdiction: JurisdictionBasis::BothHabituallyResident,
        };

        let result = validate_divorce_application(&application);
        assert!(matches!(
            result,
            Err(FamilyLawError::MarriageUnderOneYear { .. })
        ));
    }
}
