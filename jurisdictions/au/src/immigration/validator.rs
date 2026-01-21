//! Immigration validation functions
//!
//! Validators for Migration Act 1958 (Cth) and Australian Citizenship Act 2007 (Cth).

use chrono::NaiveDate;

use super::error::{ImmigrationError, Result};
use super::types::*;

/// Points test pass mark (current as of 2024)
pub const POINTS_PASS_MARK: u32 = 65;

/// Days required in Australia for citizenship (4 years = 1460 days)
pub const CITIZENSHIP_RESIDENCE_DAYS: u32 = 1460;

/// Days required in last 12 months for citizenship (at least 270)
pub const CITIZENSHIP_LAST_12_MONTHS_DAYS: u32 = 270;

/// Maximum absences in last 12 months for citizenship
pub const CITIZENSHIP_MAX_ABSENCES_12_MONTHS: u32 = 90;

/// Significant cost threshold for health requirement (as of 2024)
pub const SIGNIFICANT_COST_THRESHOLD: f64 = 86_000.0;

// =============================================================================
// Points Test Assessment
// =============================================================================

/// Calculate points for age
pub fn calculate_age_points(age: u32) -> u32 {
    match age {
        18..=24 => 25,
        25..=32 => 30,
        33..=39 => 25,
        40..=44 => 15,
        45..=49 => 0, // Can still apply but 0 age points
        _ => 0,       // Under 18 or 50+ not eligible
    }
}

/// Calculate points for English language ability
pub fn calculate_english_points(test_result: &EnglishTestResult) -> u32 {
    if test_result.meets_superior() {
        20
    } else if test_result.meets_proficient() {
        10
    } else {
        // Competent is minimum, no bonus points; below competent also scores 0
        0
    }
}

/// Calculate points for overseas skilled employment
pub fn calculate_overseas_employment_points(years: f32) -> u32 {
    match years {
        y if y >= 8.0 => 15,
        y if y >= 5.0 => 10,
        y if y >= 3.0 => 5,
        _ => 0,
    }
}

/// Calculate points for Australian skilled employment
pub fn calculate_australian_employment_points(years: f32) -> u32 {
    match years {
        y if y >= 8.0 => 20,
        y if y >= 5.0 => 15,
        y if y >= 3.0 => 10,
        y if y >= 1.0 => 5,
        _ => 0,
    }
}

/// Calculate points for education
pub fn calculate_education_points(qualification: &str) -> u32 {
    match qualification.to_lowercase().as_str() {
        q if q.contains("doctorate") || q.contains("phd") => 20,
        q if q.contains("bachelor") || q.contains("master") => 15,
        q if q.contains("diploma") || q.contains("trade") => 10,
        q if q.contains("qualification") => 10,
        _ => 0,
    }
}

/// Assess points test result
#[allow(clippy::too_many_arguments)] // Immigration points test requires all these assessment factors
pub fn assess_points_test(
    age: u32,
    english_result: &EnglishTestResult,
    overseas_employment_years: f32,
    australian_employment_years: f32,
    qualification: &str,
    has_australian_study: bool,
    has_specialist_education: bool,
    has_community_language: bool,
    has_professional_year: bool,
    partner_has_skills: bool,
    is_single_applicant: bool,
    nomination_type: Option<&str>,
) -> PointsTestResult {
    let age_points = calculate_age_points(age);
    let english_points = calculate_english_points(english_result);
    let overseas_employment_points =
        calculate_overseas_employment_points(overseas_employment_years);
    let australian_employment_points =
        calculate_australian_employment_points(australian_employment_years);
    let education_points = calculate_education_points(qualification);
    let australian_study_points = if has_australian_study { 5 } else { 0 };
    let specialist_education_points = if has_specialist_education { 10 } else { 0 };
    let community_language_points = if has_community_language { 5 } else { 0 };
    let professional_year_points = if has_professional_year { 5 } else { 0 };

    // Partner skill points: single applicants and those with skilled partners get 10 points;
    // partners with competent English only (no skilled occupation) get 5 points
    #[allow(clippy::if_same_then_else)] // Intentional: different policy reasons for same points
    let partner_points = if is_single_applicant {
        10 // Single applicant bonus
    } else if partner_has_skills {
        10 // Partner with competent English and skilled occupation
    } else {
        5 // Partner with competent English only
    };

    let nomination_points = match nomination_type {
        Some("state") => 5,     // Subclass 190
        Some("regional") => 15, // Subclass 491
        _ => 0,
    };

    let total_points = age_points
        + english_points
        + overseas_employment_points
        + australian_employment_points
        + education_points
        + australian_study_points
        + specialist_education_points
        + community_language_points
        + professional_year_points
        + partner_points
        + nomination_points;

    PointsTestResult {
        age_points,
        english_points,
        overseas_employment_points,
        australian_employment_points,
        education_points,
        australian_study_points,
        specialist_education_points,
        community_language_points,
        professional_year_points,
        partner_points,
        nomination_points,
        total_points,
        pass_mark: POINTS_PASS_MARK,
        passed: total_points >= POINTS_PASS_MARK,
    }
}

// =============================================================================
// Visa Eligibility Assessment
// =============================================================================

/// Visa eligibility assessment result
#[derive(Debug, Clone)]
pub struct VisaEligibilityResult {
    /// Whether eligible
    pub eligible: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Points (for skilled visas)
    pub points: Option<u32>,
}

/// Assess eligibility for skilled visa
pub fn assess_skilled_visa_eligibility(
    visa_subclass: VisaSubclass,
    age: u32,
    points_result: &PointsTestResult,
    occupation_assessment: &OccupationAssessment,
    english_result: &EnglishTestResult,
    assessment_date: NaiveDate,
) -> Result<VisaEligibilityResult> {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();

    // Age requirement (under 45)
    if age >= 45 {
        issues.push("Age must be under 45 at time of invitation".to_string());
    }

    // Points requirement
    if !points_result.passed {
        issues.push(format!(
            "Points test not satisfied. Required: {} points. Obtained: {} points",
            POINTS_PASS_MARK, points_result.total_points
        ));
        if points_result.total_points >= 60 {
            recommendations.push(
                "Consider state nomination (5 points) or regional nomination (15 points)"
                    .to_string(),
            );
        }
    }

    // Occupation assessment
    if occupation_assessment.outcome != AssessmentOutcome::Suitable
        && occupation_assessment.outcome != AssessmentOutcome::SuitableWithConditions
    {
        issues.push("Occupation assessment not suitable".to_string());
    }

    // Check occupation list
    match visa_subclass {
        VisaSubclass::SkilledIndependent189 => {
            if occupation_assessment.occupation_list != SkilledOccupationList::Mltssl {
                issues.push("Occupation must be on MLTSSL for subclass 189".to_string());
            }
        }
        VisaSubclass::SkilledNominated190 => {
            if occupation_assessment.occupation_list != SkilledOccupationList::Mltssl
                && occupation_assessment.occupation_list != SkilledOccupationList::Stsol
            {
                issues.push("Occupation must be on MLTSSL or STSOL for subclass 190".to_string());
            }
        }
        _ => {}
    }

    // Assessment validity
    if occupation_assessment.valid_until < assessment_date {
        issues.push("Occupation assessment has expired".to_string());
        recommendations.push("Renew skills assessment before lodging".to_string());
    }

    // English requirement
    if !english_result.meets_competent() {
        issues.push("Minimum competent English required".to_string());
    }

    Ok(VisaEligibilityResult {
        eligible: issues.is_empty(),
        issues,
        recommendations,
        points: Some(points_result.total_points),
    })
}

/// Assess eligibility for employer sponsored visa (482/494/186)
pub fn assess_employer_sponsored_eligibility(
    visa_subclass: VisaSubclass,
    sponsor: &Sponsor,
    occupation_assessment: &OccupationAssessment,
    english_result: &EnglishTestResult,
    experience_years: f32,
    assessment_date: NaiveDate,
) -> Result<VisaEligibilityResult> {
    let mut issues = Vec::new();
    let recommendations = Vec::new();

    // Sponsor status
    if sponsor.compliance_history == SponsorComplianceHistory::Barred {
        issues.push("Sponsor is barred from sponsoring visa applicants".to_string());
    }

    if let Some(expiry) = sponsor.expiry_date
        && expiry < assessment_date
    {
        issues.push("Sponsor approval has expired".to_string());
    }

    // Occupation assessment
    if occupation_assessment.outcome != AssessmentOutcome::Suitable
        && occupation_assessment.outcome != AssessmentOutcome::SuitableWithConditions
    {
        // Some subclasses may have exemptions
        if !matches!(visa_subclass, VisaSubclass::TemporarySkillShortage482) {
            issues.push("Occupation assessment required".to_string());
        }
    }

    // Experience requirement
    let required_experience = match visa_subclass {
        VisaSubclass::TemporarySkillShortage482 => 2.0,
        VisaSubclass::SkilledEmployerSponsoredRegional494 => 3.0,
        VisaSubclass::EmployerNominationScheme186 => 3.0,
        _ => 2.0,
    };

    if experience_years < required_experience {
        issues.push(format!(
            "Minimum {} years experience required. Have: {} years",
            required_experience, experience_years
        ));
    }

    // English requirement
    let required_level = match visa_subclass {
        VisaSubclass::TemporarySkillShortage482 => EnglishLanguageLevel::Competent,
        _ => EnglishLanguageLevel::Competent,
    };

    match required_level {
        EnglishLanguageLevel::Competent if !english_result.meets_competent() => {
            issues.push("Minimum competent English required".to_string());
        }
        EnglishLanguageLevel::Proficient if !english_result.meets_proficient() => {
            issues.push("Minimum proficient English required".to_string());
        }
        _ => {}
    }

    Ok(VisaEligibilityResult {
        eligible: issues.is_empty(),
        issues,
        recommendations,
        points: None,
    })
}

// =============================================================================
// Character Test Assessment
// =============================================================================

/// Character test assessment result
#[derive(Debug, Clone)]
pub struct CharacterTestResult {
    /// Whether character test passed
    pub passed: bool,
    /// Character concerns identified
    pub concerns: Vec<CharacterConcern>,
    /// Whether mandatory cancellation applies
    pub mandatory_cancellation: bool,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Character concern
#[derive(Debug, Clone)]
pub struct CharacterConcern {
    /// Ground under s.501(6)
    pub ground: CharacterTestGround,
    /// Description
    pub description: String,
    /// Severity
    pub severity: CharacterConcernSeverity,
}

/// Character concern severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterConcernSeverity {
    /// Low - unlikely to result in refusal/cancellation
    Low,
    /// Medium - may result in refusal/cancellation
    Medium,
    /// High - likely to result in refusal/cancellation
    High,
    /// Critical - mandatory cancellation applies
    Critical,
}

/// Assess character test
pub fn assess_character_test(
    has_criminal_record: bool,
    total_sentence_months: u32,
    is_on_parole: bool,
    has_criminal_association: bool,
    has_security_assessment: bool,
    past_visa_refusals: u32,
    past_visa_cancellations: u32,
) -> CharacterTestResult {
    let mut concerns = Vec::new();
    let mut mandatory_cancellation = false;
    let mut legal_references = vec![
        "Migration Act 1958 (Cth) s.501".to_string(),
        "Migration Regulations 1994 Sch 4 PIC 4001".to_string(),
    ];

    // Substantial criminal record (s.501(7))
    if has_criminal_record {
        if total_sentence_months >= 12 {
            concerns.push(CharacterConcern {
                ground: CharacterTestGround::SubstantialCriminalRecord,
                description: format!(
                    "Substantial criminal record - sentenced to {} months imprisonment (12+ months)",
                    total_sentence_months
                ),
                severity: CharacterConcernSeverity::Critical,
            });
            mandatory_cancellation = true;
            legal_references.push("s.501(7)(c) - sentenced to 12+ months".to_string());
        } else if total_sentence_months > 0 {
            concerns.push(CharacterConcern {
                ground: CharacterTestGround::SubstantialCriminalRecord,
                description: format!(
                    "Criminal record - sentenced to {} months imprisonment",
                    total_sentence_months
                ),
                severity: CharacterConcernSeverity::High,
            });
        }
    }

    // Currently on parole
    if is_on_parole {
        concerns.push(CharacterConcern {
            ground: CharacterTestGround::SubstantialCriminalRecord,
            description: "Currently on parole or licence".to_string(),
            severity: CharacterConcernSeverity::High,
        });
    }

    // Criminal association (s.501(6)(b))
    if has_criminal_association {
        concerns.push(CharacterConcern {
            ground: CharacterTestGround::CriminalAssociation,
            description: "Association with persons involved in criminal conduct".to_string(),
            severity: CharacterConcernSeverity::High,
        });
        legal_references.push("s.501(6)(b) - criminal association".to_string());
    }

    // Adverse security assessment
    if has_security_assessment {
        concerns.push(CharacterConcern {
            ground: CharacterTestGround::AdverseSecurityAssessment,
            description: "Adverse ASIO security assessment".to_string(),
            severity: CharacterConcernSeverity::Critical,
        });
        mandatory_cancellation = true;
    }

    // Past visa history
    if past_visa_cancellations > 0 {
        concerns.push(CharacterConcern {
            ground: CharacterTestGround::PastCriminalConduct,
            description: format!("Past visa cancellation(s): {}", past_visa_cancellations),
            severity: if past_visa_cancellations > 1 {
                CharacterConcernSeverity::High
            } else {
                CharacterConcernSeverity::Medium
            },
        });
        legal_references.push("PIC 4013/4014 - past visa refusal/cancellation".to_string());
    }

    if past_visa_refusals > 1 {
        concerns.push(CharacterConcern {
            ground: CharacterTestGround::PastCriminalConduct,
            description: format!("Multiple past visa refusals: {}", past_visa_refusals),
            severity: CharacterConcernSeverity::Medium,
        });
    }

    CharacterTestResult {
        passed: concerns.is_empty()
            || concerns
                .iter()
                .all(|c| matches!(c.severity, CharacterConcernSeverity::Low)),
        concerns,
        mandatory_cancellation,
        legal_references,
    }
}

// =============================================================================
// Citizenship Eligibility Assessment
// =============================================================================

/// Citizenship eligibility result
#[derive(Debug, Clone)]
pub struct CitizenshipEligibilityResult {
    /// Whether eligible
    pub eligible: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Residence requirement assessment
    pub residence_assessment: Option<ResidenceRequirement>,
    /// Character requirement met
    pub character_requirement_met: bool,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Assess citizenship eligibility by conferral
pub fn assess_citizenship_by_conferral(
    permanent_resident_since: NaiveDate,
    days_present_in_australia: u32,
    days_absent: u32,
    days_present_last_12_months: u32,
    days_absent_last_12_months: u32,
    character_result: &CharacterTestResult,
    application_date: NaiveDate,
) -> CitizenshipEligibilityResult {
    let mut issues = Vec::new();
    let legal_references = vec![
        "Australian Citizenship Act 2007 (Cth) s.21".to_string(),
        "s.21(2)(b) - residence requirement".to_string(),
        "s.21(2)(d) - good character".to_string(),
    ];

    // Calculate qualifying period
    let qualifying_period_start = application_date
        .checked_sub_months(chrono::Months::new(48))
        .unwrap_or(application_date);

    let total_days_in_period = (application_date - qualifying_period_start).num_days() as u32;

    // Permanent residence requirement
    let pr_months = (application_date - permanent_resident_since).num_days() / 30;
    if pr_months < 12 {
        issues.push(format!(
            "Must be permanent resident for at least 12 months. Current: {} months",
            pr_months
        ));
    }

    // 4 years lawful residence
    let lawful_residence_months = (application_date - qualifying_period_start).num_days() / 30;
    if lawful_residence_months < 48 {
        issues.push("Must have 4 years lawful residence in Australia".to_string());
    }

    // Total days present
    if days_present_in_australia < CITIZENSHIP_RESIDENCE_DAYS {
        issues.push(format!(
            "Insufficient days in Australia. Required: {} days. Present: {} days",
            CITIZENSHIP_RESIDENCE_DAYS, days_present_in_australia
        ));
    }

    // Days in last 12 months
    if days_present_last_12_months < CITIZENSHIP_LAST_12_MONTHS_DAYS {
        issues.push(format!(
            "Insufficient days in last 12 months. Required: {} days. Present: {} days",
            CITIZENSHIP_LAST_12_MONTHS_DAYS, days_present_last_12_months
        ));
    }

    // Absences in last 12 months
    if days_absent_last_12_months > CITIZENSHIP_MAX_ABSENCES_12_MONTHS {
        issues.push(format!(
            "Too many absences in last 12 months. Maximum: {} days. Absent: {} days",
            CITIZENSHIP_MAX_ABSENCES_12_MONTHS, days_absent_last_12_months
        ));
    }

    // Character requirement
    let character_met = character_result.passed && !character_result.mandatory_cancellation;
    if !character_met {
        issues.push("Character requirement not satisfied".to_string());
    }

    let residence_assessment = ResidenceRequirement {
        qualifying_period_start,
        qualifying_period_end: application_date,
        total_days_in_period,
        days_present: days_present_in_australia,
        days_absent,
        days_in_last_twelve_months: days_present_last_12_months,
        days_absent_in_last_twelve_months: days_absent_last_12_months,
        permanent_visa_throughout: pr_months >= 12,
        requirement_met: days_present_in_australia >= CITIZENSHIP_RESIDENCE_DAYS
            && days_present_last_12_months >= CITIZENSHIP_LAST_12_MONTHS_DAYS
            && days_absent_last_12_months <= CITIZENSHIP_MAX_ABSENCES_12_MONTHS,
    };

    CitizenshipEligibilityResult {
        eligible: issues.is_empty(),
        issues,
        residence_assessment: Some(residence_assessment),
        character_requirement_met: character_met,
        legal_references,
    }
}

/// Calculate citizenship test result
pub fn assess_citizenship_test(
    correct_answers: u32,
    total_questions: u32,
    attempts: u32,
) -> Result<bool> {
    if attempts > 3 {
        return Err(ImmigrationError::CitizenshipTestNotPassed {
            score: (correct_answers * 100) / total_questions,
            attempts,
        });
    }

    let score_percent = (correct_answers * 100) / total_questions;

    if score_percent >= 75 {
        Ok(true)
    } else {
        Err(ImmigrationError::CitizenshipTestNotPassed {
            score: score_percent,
            attempts,
        })
    }
}

// =============================================================================
// Sponsor Validation
// =============================================================================

/// Sponsor validation result
#[derive(Debug, Clone)]
pub struct SponsorValidationResult {
    /// Whether sponsor is approved
    pub approved: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
}

/// Validate sponsor eligibility
pub fn validate_sponsor(
    sponsor: &Sponsor,
    visa_subclass: VisaSubclass,
    assessment_date: NaiveDate,
) -> SponsorValidationResult {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check sponsor type matches visa subclass
    match visa_subclass {
        VisaSubclass::TemporarySkillShortage482
        | VisaSubclass::SkilledEmployerSponsoredRegional494
        | VisaSubclass::EmployerNominationScheme186 => {
            if !matches!(
                sponsor.sponsor_type,
                SponsorType::StandardBusinessSponsor
                    | SponsorType::AccreditedSponsor
                    | SponsorType::LabourAgreementSponsor
            ) {
                issues.push("Sponsor type not valid for employer sponsored visa".to_string());
            }
        }
        VisaSubclass::Partner820801 | VisaSubclass::PartnerOffshore309100 => {
            // Partner sponsors must be Australian citizen/PR
        }
        _ => {}
    }

    // Check approval status
    if let Some(expiry) = sponsor.expiry_date {
        if expiry < assessment_date {
            issues.push("Sponsor approval has expired".to_string());
        } else if (expiry - assessment_date).num_days() < 90 {
            warnings.push("Sponsor approval expiring within 90 days".to_string());
        }
    }

    // Check compliance history
    match sponsor.compliance_history {
        SponsorComplianceHistory::Barred => {
            issues.push("Sponsor is barred from sponsoring".to_string());
        }
        SponsorComplianceHistory::SignificantConcerns => {
            warnings.push("Sponsor has significant compliance concerns".to_string());
        }
        SponsorComplianceHistory::UnderMonitoring => {
            warnings.push("Sponsor is under compliance monitoring".to_string());
        }
        _ => {}
    }

    SponsorValidationResult {
        approved: issues.is_empty(),
        issues,
        warnings,
    }
}

// =============================================================================
// Visa Condition Validation
// =============================================================================

/// Validate visa condition compliance
pub fn validate_visa_condition_compliance(
    condition: VisaCondition,
    hours_worked_fortnight: Option<u32>,
    is_studying: bool,
    has_health_insurance: bool,
    working_for_sponsor: bool,
    in_regional_area: bool,
) -> Result<()> {
    match condition {
        VisaCondition::NoWork8101 => {
            if hours_worked_fortnight.is_some_and(|h| h > 0) {
                return Err(ImmigrationError::VisaConditionBreached {
                    condition_number: "8101".to_string(),
                    condition_description: "No work permitted".to_string(),
                    breach: "Work undertaken while holding visa with no work condition".to_string(),
                });
            }
        }
        VisaCondition::WorkLimitationStudent8105 => {
            if is_studying && hours_worked_fortnight.is_some_and(|h| h > 48) {
                return Err(ImmigrationError::VisaConditionBreached {
                    condition_number: "8105".to_string(),
                    condition_description: "Maximum 48 hours per fortnight during session"
                        .to_string(),
                    breach: format!(
                        "Worked {} hours in fortnight (max 48)",
                        hours_worked_fortnight.unwrap_or(0)
                    ),
                });
            }
        }
        VisaCondition::MaintainHealthInsurance8501 => {
            if !has_health_insurance {
                return Err(ImmigrationError::VisaConditionBreached {
                    condition_number: "8501".to_string(),
                    condition_description: "Must maintain adequate health insurance".to_string(),
                    breach: "Health insurance not maintained".to_string(),
                });
            }
        }
        VisaCondition::EmployerMustBeSponsoring8535 => {
            if !working_for_sponsor {
                return Err(ImmigrationError::VisaConditionBreached {
                    condition_number: "8535".to_string(),
                    condition_description: "Must work for sponsoring employer".to_string(),
                    breach: "Not working for approved sponsor".to_string(),
                });
            }
        }
        VisaCondition::RegionalWorkRequirement8607 => {
            if !in_regional_area {
                return Err(ImmigrationError::VisaConditionBreached {
                    condition_number: "8607".to_string(),
                    condition_description: "Must live, work and study in regional area".to_string(),
                    breach: "Not residing in designated regional area".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_english_result(level: EnglishLanguageLevel) -> EnglishTestResult {
        let (score, overall) = match level {
            EnglishLanguageLevel::Competent => (6.0, 6.0),
            EnglishLanguageLevel::Proficient => (7.0, 7.0),
            EnglishLanguageLevel::Superior => (8.0, 8.0),
        };
        EnglishTestResult {
            test_type: EnglishLanguageTest::Ielts,
            test_date: NaiveDate::from_ymd_opt(2025, 1, 15).expect("valid date"),
            listening: score,
            reading: score,
            writing: score,
            speaking: score,
            overall,
            level,
        }
    }

    #[test]
    fn test_age_points_calculation() {
        assert_eq!(calculate_age_points(20), 25);
        assert_eq!(calculate_age_points(28), 30);
        assert_eq!(calculate_age_points(35), 25);
        assert_eq!(calculate_age_points(42), 15);
        assert_eq!(calculate_age_points(47), 0);
        assert_eq!(calculate_age_points(50), 0);
    }

    #[test]
    fn test_english_points_calculation() {
        let competent = create_test_english_result(EnglishLanguageLevel::Competent);
        assert_eq!(calculate_english_points(&competent), 0);

        let proficient = create_test_english_result(EnglishLanguageLevel::Proficient);
        assert_eq!(calculate_english_points(&proficient), 10);

        let superior = create_test_english_result(EnglishLanguageLevel::Superior);
        assert_eq!(calculate_english_points(&superior), 20);
    }

    #[test]
    fn test_points_test_pass() {
        let english = create_test_english_result(EnglishLanguageLevel::Superior);
        let result = assess_points_test(
            28,                // Age: 30 points
            &english,          // English: 20 points
            5.0,               // Overseas: 10 points
            2.0,               // Australian: 5 points
            "Bachelor degree", // Education: 15 points
            true,              // Australian study: 5 points
            false,             // Specialist: 0
            false,             // Community language: 0
            false,             // Professional year: 0
            false,             // Partner skills
            true,              // Single: 10 points
            None,              // No nomination
        );

        assert_eq!(result.total_points, 95);
        assert!(result.passed);
    }

    #[test]
    fn test_points_test_fail() {
        let english = create_test_english_result(EnglishLanguageLevel::Competent);
        let result = assess_points_test(
            40,        // Age: 15 points
            &english,  // English: 0 points
            1.0,       // Overseas: 0 points
            0.5,       // Australian: 0 points
            "Diploma", // Education: 10 points
            false,     // Australian study: 0
            false,     // Specialist: 0
            false,     // Community language: 0
            false,     // Professional year: 0
            false,     // Partner skills
            false,     // Not single: 5 points
            None,      // No nomination
        );

        assert_eq!(result.total_points, 30);
        assert!(!result.passed);
    }

    #[test]
    fn test_character_test_clean() {
        let result = assess_character_test(false, 0, false, false, false, 0, 0);
        assert!(result.passed);
        assert!(!result.mandatory_cancellation);
        assert!(result.concerns.is_empty());
    }

    #[test]
    fn test_character_test_substantial_criminal_record() {
        let result = assess_character_test(true, 18, false, false, false, 0, 0);
        assert!(!result.passed);
        assert!(result.mandatory_cancellation);
        assert!(
            result
                .concerns
                .iter()
                .any(|c| c.ground == CharacterTestGround::SubstantialCriminalRecord)
        );
    }

    #[test]
    fn test_citizenship_eligibility() {
        let character_result = assess_character_test(false, 0, false, false, false, 0, 0);
        let application_date = NaiveDate::from_ymd_opt(2026, 6, 1).expect("valid date");
        let pr_since = NaiveDate::from_ymd_opt(2021, 1, 1).expect("valid date");

        let result = assess_citizenship_by_conferral(
            pr_since,
            1500, // Days present
            50,   // Days absent
            300,  // Days in last 12 months
            65,   // Days absent in last 12 months
            &character_result,
            application_date,
        );

        assert!(result.eligible);
        assert!(result.residence_assessment.unwrap().requirement_met);
    }

    #[test]
    fn test_citizenship_insufficient_residence() {
        let character_result = assess_character_test(false, 0, false, false, false, 0, 0);
        let application_date = NaiveDate::from_ymd_opt(2026, 6, 1).expect("valid date");
        let pr_since = NaiveDate::from_ymd_opt(2021, 1, 1).expect("valid date");

        let result = assess_citizenship_by_conferral(
            pr_since,
            1000, // Days present - insufficient
            460,  // Days absent
            200,  // Days in last 12 months - insufficient
            165,  // Days absent - too many
            &character_result,
            application_date,
        );

        assert!(!result.eligible);
        assert!(!result.residence_assessment.unwrap().requirement_met);
    }

    #[test]
    fn test_citizenship_test_pass() {
        let result = assess_citizenship_test(18, 20, 1);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_citizenship_test_fail() {
        let result = assess_citizenship_test(10, 20, 2);
        assert!(result.is_err());
        if let Err(ImmigrationError::CitizenshipTestNotPassed { score, attempts }) = result {
            assert_eq!(score, 50);
            assert_eq!(attempts, 2);
        }
    }

    #[test]
    fn test_visa_condition_work_breach() {
        let result = validate_visa_condition_compliance(
            VisaCondition::NoWork8101,
            Some(20),
            false,
            true,
            true,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_visa_condition_student_work_limit() {
        // Under limit - OK
        let result = validate_visa_condition_compliance(
            VisaCondition::WorkLimitationStudent8105,
            Some(40),
            true,
            true,
            true,
            true,
        );
        assert!(result.is_ok());

        // Over limit - breach
        let result = validate_visa_condition_compliance(
            VisaCondition::WorkLimitationStudent8105,
            Some(60),
            true,
            true,
            true,
            true,
        );
        assert!(result.is_err());
    }
}
