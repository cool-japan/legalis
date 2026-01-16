//! UK Family Law - Children Law
//!
//! Implementation of children law under the Children Act 1989 and related legislation.
//!
//! # Key Legislation
//!
//! ## Children Act 1989
//!
//! The Children Act 1989 is the primary statute governing children law in England and Wales.
//!
//! ### Key Principles
//!
//! - **Welfare principle (s.1(1))**: The child's welfare is the paramount consideration
//! - **No delay principle (s.1(2))**: Delay is likely to prejudice the child's welfare
//! - **No order principle (s.1(5))**: Court should only make order if better than no order
//!
//! ### Welfare Checklist (s.1(3))
//!
//! When contested, court must have regard to:
//! - (a) Ascertainable wishes and feelings of the child (in light of age and understanding)
//! - (b) Child's physical, emotional and educational needs
//! - (c) Likely effect of any change in circumstances
//! - (d) Age, sex, background and any relevant characteristics
//! - (e) Any harm suffered or at risk of suffering
//! - (f) How capable parents (and others) are of meeting needs
//! - (g) Range of powers available to the court
//!
//! ### Section 8 Orders
//!
//! - **Child arrangements order**: With whom child lives and spends time
//! - **Specific issue order**: Determines specific question about upbringing
//! - **Prohibited steps order**: Prevents certain steps being taken
//!
//! # Parental Responsibility
//!
//! - Mother: Automatic (s.2(1))
//! - Married father: Automatic (s.2(1))
//! - Unmarried father: By agreement, court order, or birth registration (post-1 Dec 2003)
//! - Step-parent: By agreement or court order (s.4A)
//! - Civil partner: By agreement or court order (s.4A)
//! - Second female parent: Under HFEA 2008

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::error::{FamilyLawError, Result};
use super::types::{
    ChildDetails, OtherContactArrangement, PRAcquisitionMethod, ParentalResponsibility,
    SpendingTimeArrangement,
};

// ============================================================================
// Welfare Checklist
// ============================================================================

/// Factor from the welfare checklist (CA 1989 s.1(3))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WelfareChecklistFactor {
    /// (a) Ascertainable wishes and feelings of the child
    WishesAndFeelings,
    /// (b) Physical, emotional and educational needs
    PhysicalEmotionalEducationalNeeds,
    /// (c) Likely effect of any change in circumstances
    LikelyEffectOfChange,
    /// (d) Age, sex, background and any relevant characteristics
    AgeBackgroundCharacteristics,
    /// (e) Any harm suffered or at risk of suffering
    HarmSufferedOrRisk,
    /// (f) Capability of parents (and others) of meeting needs
    CapabilityOfParents,
    /// (g) Range of powers available to the court
    RangeOfPowers,
}

impl WelfareChecklistFactor {
    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::WishesAndFeelings => "CA 1989 s.1(3)(a)",
            Self::PhysicalEmotionalEducationalNeeds => "CA 1989 s.1(3)(b)",
            Self::LikelyEffectOfChange => "CA 1989 s.1(3)(c)",
            Self::AgeBackgroundCharacteristics => "CA 1989 s.1(3)(d)",
            Self::HarmSufferedOrRisk => "CA 1989 s.1(3)(e)",
            Self::CapabilityOfParents => "CA 1989 s.1(3)(f)",
            Self::RangeOfPowers => "CA 1989 s.1(3)(g)",
        }
    }

    /// Get all factors
    pub fn all() -> Vec<Self> {
        vec![
            Self::WishesAndFeelings,
            Self::PhysicalEmotionalEducationalNeeds,
            Self::LikelyEffectOfChange,
            Self::AgeBackgroundCharacteristics,
            Self::HarmSufferedOrRisk,
            Self::CapabilityOfParents,
            Self::RangeOfPowers,
        ]
    }
}

/// Assessment of a welfare checklist factor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WelfareFactorAssessment {
    /// Factor being assessed
    pub factor: WelfareChecklistFactor,
    /// Assessment for each party/proposal
    pub findings: Vec<String>,
    /// Weight given to this factor
    pub weight: FactorWeight,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Weight given to a factor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorWeight {
    /// Determinative
    Determinative,
    /// Significant
    Significant,
    /// Moderate
    Moderate,
    /// Minor
    Minor,
    /// Neutral
    Neutral,
}

/// Complete welfare checklist analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WelfareChecklistAnalysis {
    /// Child details
    pub child: ChildDetails,
    /// Assessment of each factor
    pub factor_assessments: Vec<WelfareFactorAssessment>,
    /// Overall welfare analysis
    pub overall_analysis: String,
    /// Recommendation
    pub recommendation: String,
    /// Is checklist complete?
    pub checklist_complete: bool,
}

impl WelfareChecklistAnalysis {
    /// Create new analysis
    pub fn new(child: ChildDetails) -> Self {
        Self {
            child,
            factor_assessments: Vec::new(),
            overall_analysis: String::new(),
            recommendation: String::new(),
            checklist_complete: false,
        }
    }

    /// Add factor assessment
    pub fn add_assessment(&mut self, assessment: WelfareFactorAssessment) {
        self.factor_assessments.push(assessment);
    }

    /// Check if all factors assessed
    pub fn is_complete(&self) -> bool {
        let assessed_factors: Vec<_> = self.factor_assessments.iter().map(|a| &a.factor).collect();

        WelfareChecklistFactor::all()
            .iter()
            .all(|f| assessed_factors.contains(&f))
    }

    /// Finalize analysis
    pub fn finalize(&mut self, overall_analysis: String, recommendation: String) {
        self.checklist_complete = self.is_complete();
        self.overall_analysis = overall_analysis;
        self.recommendation = recommendation;
    }
}

// ============================================================================
// Parental Responsibility Analysis
// ============================================================================

/// Analysis of parental responsibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentalResponsibilityAnalysis {
    /// Child
    pub child: String,
    /// Current PR holders
    pub current_holders: Vec<ParentalResponsibility>,
    /// Proposed PR holder (if application)
    pub proposed_holder: Option<String>,
    /// Can PR be acquired?
    pub can_acquire: bool,
    /// Method of acquisition available
    pub acquisition_method: Option<PRAcquisitionMethod>,
    /// Requirements to be met
    pub requirements: Vec<String>,
    /// Analysis
    pub analysis: String,
}

impl ParentalResponsibilityAnalysis {
    /// Analyze unmarried father's PR application
    pub fn analyze_unmarried_father(
        child: &str,
        father_name: &str,
        on_birth_certificate: bool,
        birth_registered_after_dec_2003: bool,
        mother_consents: bool,
    ) -> Self {
        let mut requirements = Vec::new();

        let (can_acquire, acquisition_method) =
            if on_birth_certificate && birth_registered_after_dec_2003 {
                // Automatic PR for fathers on birth certificate after 1 Dec 2003
                (true, Some(PRAcquisitionMethod::BirthRegistration))
            } else if mother_consents {
                // PR agreement available
                requirements.push(
                    "PR agreement must be in prescribed form and registered with \
                 Principal Registry"
                        .to_string(),
                );
                (true, Some(PRAcquisitionMethod::Agreement))
            } else {
                // Court order required
                requirements.push("Application to court under CA 1989 s.4(1)(a)".to_string());
                requirements.push(
                    "Court will consider: degree of commitment to child, attachment \
                 between father and child, reasons for application (Re H [1998])"
                        .to_string(),
                );
                (true, Some(PRAcquisitionMethod::CourtOrder))
            };

        let analysis = if on_birth_certificate && birth_registered_after_dec_2003 {
            format!(
                "Unmarried father {} has AUTOMATIC parental responsibility for {} \
                 by virtue of being registered on birth certificate after 1 December 2003 \
                 (CA 1989 s.4(1)(a) as amended).",
                father_name, child
            )
        } else if mother_consents {
            format!(
                "Unmarried father {} can acquire PR for {} by PR agreement with mother \
                 (CA 1989 s.4(1)(b)). Agreement must be in Form C(PRA1) and registered.",
                father_name, child
            )
        } else {
            format!(
                "Unmarried father {} can apply for PR order for {} under CA 1989 s.4(1)(a). \
                 Court will apply Re H factors: commitment, attachment, reasons.",
                father_name, child
            )
        };

        Self {
            child: child.to_string(),
            current_holders: Vec::new(),
            proposed_holder: Some(father_name.to_string()),
            can_acquire,
            acquisition_method,
            requirements,
            analysis,
        }
    }

    /// Analyze step-parent PR application
    pub fn analyze_step_parent(
        child: &str,
        step_parent_name: &str,
        married_to_parent: bool,
        parent_has_pr: bool,
        other_parent_consents: bool,
    ) -> Self {
        let mut requirements = Vec::new();
        let mut can_acquire = false;
        let mut acquisition_method = None;

        if !married_to_parent {
            requirements.push(
                "Step-parent must be married to or civil partner of parent with PR".to_string(),
            );
        } else if !parent_has_pr {
            requirements.push("Parent spouse/civil partner must have PR".to_string());
        } else if other_parent_consents {
            can_acquire = true;
            acquisition_method = Some(PRAcquisitionMethod::Agreement);
            requirements.push(
                "PR agreement must be in prescribed form with consent of all PR holders"
                    .to_string(),
            );
        } else {
            can_acquire = true;
            acquisition_method = Some(PRAcquisitionMethod::CourtOrder);
            requirements.push("Application to court under CA 1989 s.4A(1)(a)".to_string());
        }

        let analysis = if can_acquire {
            format!(
                "Step-parent {} can acquire PR for {} under CA 1989 s.4A. \
                 Method: {:?}. Requirements: {}",
                step_parent_name,
                child,
                acquisition_method,
                requirements.join("; ")
            )
        } else {
            format!(
                "Step-parent {} cannot currently acquire PR for {}. \
                 Requirements not met: {}",
                step_parent_name,
                child,
                requirements.join("; ")
            )
        };

        Self {
            child: child.to_string(),
            current_holders: Vec::new(),
            proposed_holder: Some(step_parent_name.to_string()),
            can_acquire,
            acquisition_method,
            requirements,
            analysis,
        }
    }
}

// ============================================================================
// Section 8 Order Applications
// ============================================================================

/// Standing to apply for s.8 orders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicantCategory {
    /// Parent (entitled to apply - s.10(4))
    Parent,
    /// Guardian (entitled to apply - s.10(4))
    Guardian,
    /// Special guardian (entitled to apply - s.10(4))
    SpecialGuardian,
    /// Person with child arrangements order (lives with) (entitled - s.10(4))
    ChildArrangementsOrderHolder,
    /// Person child has lived with for 3 years (entitled for CAO - s.10(5))
    ThreeYearCarer,
    /// Person with consent of all PR holders (s.10(5)(c))
    WithConsent,
    /// Local authority foster carer (with consent or leave)
    FosterCarer,
    /// Relative (needs leave unless exception)
    Relative,
    /// Other (needs leave - s.10(2))
    Other,
}

/// Analysis of standing to apply
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingAnalysis {
    /// Applicant
    pub applicant: String,
    /// Category
    pub category: ApplicantCategory,
    /// Order sought
    pub order_type: Section8OrderType,
    /// Has automatic standing?
    pub automatic_standing: bool,
    /// Needs leave?
    pub needs_leave: bool,
    /// Leave criteria (if applicable)
    pub leave_criteria: Vec<String>,
    /// Analysis
    pub analysis: String,
}

/// Type of s.8 order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section8OrderType {
    /// Child arrangements order
    ChildArrangements,
    /// Specific issue order
    SpecificIssue,
    /// Prohibited steps order
    ProhibitedSteps,
}

impl StandingAnalysis {
    /// Analyze standing to apply
    pub fn analyze(
        applicant: &str,
        category: ApplicantCategory,
        order_type: Section8OrderType,
    ) -> Self {
        let (automatic_standing, needs_leave, leave_criteria) = match category {
            ApplicantCategory::Parent
            | ApplicantCategory::Guardian
            | ApplicantCategory::SpecialGuardian
            | ApplicantCategory::ChildArrangementsOrderHolder => (true, false, vec![]),

            ApplicantCategory::ThreeYearCarer => {
                // Entitled for CAO if lived with child for 3 years
                match order_type {
                    Section8OrderType::ChildArrangements => (true, false, vec![]),
                    _ => (
                        false,
                        true,
                        vec![
                            "Nature of application".to_string(),
                            "Applicant's connection with child".to_string(),
                            "Risk of disruption to child's life".to_string(),
                        ],
                    ),
                }
            }

            ApplicantCategory::WithConsent => (true, false, vec![]),

            ApplicantCategory::FosterCarer => (
                false,
                true,
                vec![
                    "Must have local authority consent or court leave".to_string(),
                    "Child must have lived with foster carer for at least 1 year".to_string(),
                ],
            ),

            ApplicantCategory::Relative => (
                false,
                true,
                vec![
                    "Nature of application".to_string(),
                    "Applicant's connection with child".to_string(),
                    "Risk of disruption to child's life (CA 1989 s.10(9))".to_string(),
                ],
            ),

            ApplicantCategory::Other => (
                false,
                true,
                vec![
                    "Nature of application (s.10(9)(a))".to_string(),
                    "Applicant's connection with child (s.10(9)(b))".to_string(),
                    "Risk of harm/disruption to child (s.10(9)(c))".to_string(),
                ],
            ),
        };

        let analysis = if automatic_standing {
            format!(
                "{} has automatic standing to apply for {:?} as {:?} (CA 1989 s.10(4)/(5)).",
                applicant, order_type, category
            )
        } else {
            format!(
                "{} as {:?} needs leave of court to apply for {:?}. \
                 Court will consider: {}",
                applicant,
                category,
                order_type,
                leave_criteria.join("; ")
            )
        };

        Self {
            applicant: applicant.to_string(),
            category,
            order_type,
            automatic_standing,
            needs_leave,
            leave_criteria,
            analysis,
        }
    }
}

// ============================================================================
// Child Arrangements Order Analysis
// ============================================================================

/// Analysis of child arrangements order application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildArrangementsAnalysis {
    /// Child
    pub child: ChildDetails,
    /// Proposed arrangements
    pub proposed: ProposedArrangements,
    /// Welfare checklist analysis
    pub welfare_analysis: Option<WelfareChecklistAnalysis>,
    /// Status quo assessment
    pub status_quo: StatusQuoAssessment,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
    /// Recommendation
    pub recommendation: String,
    /// Analysis
    pub analysis: String,
}

/// Proposed child arrangements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposedArrangements {
    /// Who child would live with
    pub lives_with: Vec<String>,
    /// Spending time arrangements
    pub spends_time_with: Vec<SpendingTimeArrangement>,
    /// Other contact
    pub other_contact: Vec<OtherContactArrangement>,
    /// Shared care?
    pub shared_care: bool,
}

/// Assessment of status quo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusQuoAssessment {
    /// Current living arrangements
    pub current_arrangements: String,
    /// How long current arrangements in place
    pub duration: String,
    /// Is child settled?
    pub child_settled: bool,
    /// Change required?
    pub change_required: bool,
    /// Impact of change
    pub impact_of_change: String,
}

/// Risk assessment for children proceedings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Domestic abuse concerns
    pub domestic_abuse_concerns: bool,
    /// Safeguarding concerns
    pub safeguarding_concerns: bool,
    /// Specific risks identified
    pub risks: Vec<IdentifiedRisk>,
    /// Risk mitigation measures
    pub mitigation: Vec<String>,
    /// PD12J applies?
    pub pd12j_applies: bool,
}

/// Identified risk
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentifiedRisk {
    /// Risk type
    pub risk_type: RiskType,
    /// Description
    pub description: String,
    /// Likelihood
    pub likelihood: RiskLikelihood,
    /// Impact
    pub impact: RiskImpact,
    /// Person at risk
    pub person_at_risk: String,
}

/// Type of risk
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskType {
    /// Physical harm
    PhysicalHarm,
    /// Emotional harm
    EmotionalHarm,
    /// Sexual abuse
    SexualAbuse,
    /// Neglect
    Neglect,
    /// Domestic abuse (witnessing)
    WitnessingDomesticAbuse,
    /// Abduction risk
    Abduction,
    /// Alienation
    Alienation,
    /// Substance abuse
    SubstanceAbuse,
    /// Mental health
    MentalHealth,
    /// Other
    Other,
}

/// Risk likelihood
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLikelihood {
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
}

/// Risk impact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskImpact {
    /// Severe
    Severe,
    /// Significant
    Significant,
    /// Moderate
    Moderate,
    /// Minor
    Minor,
}

// ============================================================================
// Special Guardianship
// ============================================================================

/// Special guardianship order analysis (CA 1989 s.14A-14G)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialGuardianshipAnalysis {
    /// Child
    pub child: String,
    /// Proposed special guardian
    pub proposed_guardian: String,
    /// Standing requirements met?
    pub standing_met: bool,
    /// Notice given to local authority (3 months)?
    pub notice_given: bool,
    /// Local authority report prepared?
    pub la_report_prepared: bool,
    /// Suitability assessment
    pub suitability: GuardianSuitability,
    /// Welfare analysis
    pub welfare_favours: bool,
    /// Orders to be discharged
    pub orders_discharged: Vec<String>,
    /// Support services available
    pub support_services: Vec<String>,
    /// Analysis
    pub analysis: String,
}

/// Guardian suitability factors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuardianSuitability {
    /// Applicant's relationship to child
    pub relationship: String,
    /// Living arrangements
    pub living_arrangements: String,
    /// Ability to meet child's needs
    pub ability_to_meet_needs: String,
    /// Understanding of PR implications
    pub understands_pr: bool,
    /// Commitment assessment
    pub commitment: String,
    /// Overall suitable?
    pub suitable: bool,
}

impl SpecialGuardianshipAnalysis {
    /// Analyze special guardianship application
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        child: &str,
        proposed_guardian: &str,
        notice_given: bool,
        notice_days: u32,
        la_report_prepared: bool,
        suitability: GuardianSuitability,
        welfare_favours: bool,
    ) -> Self {
        let standing_met = true; // Simplified - would check s.14A(5) in practice
        let notice_sufficient = notice_days >= 90; // 3 months

        let analysis = if !notice_sufficient {
            format!(
                "Special guardianship application: Notice to LA insufficient. \
                 {} days given, 3 months (90 days) required under CA 1989 s.14A(7).",
                notice_days
            )
        } else if !la_report_prepared {
            "Special guardianship application: LA report not yet prepared under s.14A(8)."
                .to_string()
        } else if !suitability.suitable {
            format!(
                "Special guardianship: Concerns about suitability of {} as special guardian.",
                proposed_guardian
            )
        } else if !welfare_favours {
            "Special guardianship: Welfare analysis does not favour making order.".to_string()
        } else {
            format!(
                "Special guardianship application for {} by {} meets requirements. \
                 LA report prepared, suitability confirmed, welfare favours order.",
                child, proposed_guardian
            )
        };

        Self {
            child: child.to_string(),
            proposed_guardian: proposed_guardian.to_string(),
            standing_met,
            notice_given: notice_given && notice_sufficient,
            la_report_prepared,
            suitability,
            welfare_favours,
            orders_discharged: vec!["Care order (if any)".to_string()],
            support_services: vec![
                "Special guardianship support services (CA 1989 s.14F)".to_string(),
            ],
            analysis,
        }
    }
}

// ============================================================================
// CAFCASS and Reporting
// ============================================================================

/// Type of CAFCASS report
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CafcassReportType {
    /// Safeguarding checks (initial)
    SafeguardingChecks,
    /// Section 7 welfare report
    Section7Report,
    /// Section 37 report (LA investigation)
    Section37Report,
    /// Rule 16.4 report (child joined as party)
    Rule16_4Report,
}

/// CAFCASS officer recommendation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CafcassRecommendation {
    /// Report type
    pub report_type: CafcassReportType,
    /// Officer name
    pub officer: String,
    /// Date of report
    pub date: NaiveDate,
    /// Child's wishes (if ascertained)
    pub child_wishes: Option<String>,
    /// Welfare analysis
    pub welfare_analysis: String,
    /// Recommendation
    pub recommendation: String,
    /// Risk factors identified
    pub risk_factors: Vec<String>,
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate welfare checklist is complete
pub fn validate_welfare_checklist(analysis: &WelfareChecklistAnalysis) -> Result<()> {
    if !analysis.is_complete() {
        let assessed: Vec<_> = analysis
            .factor_assessments
            .iter()
            .map(|a| format!("{:?}", a.factor))
            .collect();

        let all_factors: Vec<_> = WelfareChecklistFactor::all()
            .iter()
            .map(|f| format!("{:?}", f))
            .collect();

        let missing: Vec<_> = all_factors
            .iter()
            .filter(|f| !assessed.contains(f))
            .cloned()
            .collect();

        return Err(FamilyLawError::WelfareChecklistNotConsidered {
            missing_factors: missing,
        });
    }
    Ok(())
}

/// Validate child's wishes considered for age-appropriate child
pub fn validate_child_wishes_considered(
    child: &ChildDetails,
    current_date: NaiveDate,
    wishes_ascertained: bool,
) -> Result<()> {
    let age = child.age_at(current_date);

    // Generally children 10+ should have wishes ascertained
    if age >= 10 && !wishes_ascertained {
        return Err(FamilyLawError::ChildWishesNotAscertained { age });
    }
    Ok(())
}

/// Validate special guardianship notice period
pub fn validate_sg_notice(notice_given: bool, notice_days: u32) -> Result<()> {
    if !notice_given || notice_days < 90 {
        return Err(FamilyLawError::SpecialGuardianshipNoticeNotGiven {
            days_given: notice_days,
            required_days: 90,
        });
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

    fn test_child() -> ChildDetails {
        ChildDetails {
            name: "Test Child".to_string(),
            date_of_birth: test_date(2016, 6, 15),
            child_of_family: true,
            biological_parent1: Some("Parent 1".to_string()),
            biological_parent2: Some("Parent 2".to_string()),
            residence: Some("123 Test Street".to_string()),
            special_needs: None,
            wishes: None,
        }
    }

    #[test]
    fn test_child_age_calculation() {
        let child = test_child();
        let current = test_date(2024, 1, 1);

        assert_eq!(child.age_at(current), 7);
        assert!(child.is_minor(current));
        assert!(!child.is_gillick_competent_age(current));
    }

    #[test]
    fn test_welfare_checklist_factors() {
        let factors = WelfareChecklistFactor::all();
        assert_eq!(factors.len(), 7);
    }

    #[test]
    fn test_welfare_checklist_analysis_incomplete() {
        let child = test_child();
        let analysis = WelfareChecklistAnalysis::new(child);

        assert!(!analysis.is_complete());
    }

    #[test]
    fn test_pr_analysis_unmarried_father_auto() {
        let analysis = ParentalResponsibilityAnalysis::analyze_unmarried_father(
            "Child", "Father", true, true, false,
        );

        assert!(analysis.can_acquire);
        assert_eq!(
            analysis.acquisition_method,
            Some(PRAcquisitionMethod::BirthRegistration)
        );
    }

    #[test]
    fn test_pr_analysis_unmarried_father_agreement() {
        let analysis = ParentalResponsibilityAnalysis::analyze_unmarried_father(
            "Child", "Father", false, false, true,
        );

        assert!(analysis.can_acquire);
        assert_eq!(
            analysis.acquisition_method,
            Some(PRAcquisitionMethod::Agreement)
        );
    }

    #[test]
    fn test_pr_analysis_unmarried_father_court() {
        let analysis = ParentalResponsibilityAnalysis::analyze_unmarried_father(
            "Child", "Father", false, false, false,
        );

        assert!(analysis.can_acquire);
        assert_eq!(
            analysis.acquisition_method,
            Some(PRAcquisitionMethod::CourtOrder)
        );
    }

    #[test]
    fn test_standing_analysis_parent() {
        let analysis = StandingAnalysis::analyze(
            "Parent",
            ApplicantCategory::Parent,
            Section8OrderType::ChildArrangements,
        );

        assert!(analysis.automatic_standing);
        assert!(!analysis.needs_leave);
    }

    #[test]
    fn test_standing_analysis_other() {
        let analysis = StandingAnalysis::analyze(
            "Other Person",
            ApplicantCategory::Other,
            Section8OrderType::ChildArrangements,
        );

        assert!(!analysis.automatic_standing);
        assert!(analysis.needs_leave);
        assert!(!analysis.leave_criteria.is_empty());
    }

    #[test]
    fn test_special_guardianship_notice_insufficient() {
        let suitability = GuardianSuitability {
            relationship: "Grandmother".to_string(),
            living_arrangements: "Suitable".to_string(),
            ability_to_meet_needs: "Good".to_string(),
            understands_pr: true,
            commitment: "High".to_string(),
            suitable: true,
        };

        let analysis = SpecialGuardianshipAnalysis::analyze(
            "Child",
            "Grandmother",
            true,
            60, // Only 60 days, need 90
            true,
            suitability,
            true,
        );

        assert!(!analysis.notice_given);
        assert!(analysis.analysis.contains("insufficient"));
    }

    #[test]
    fn test_validate_sg_notice_insufficient() {
        let result = validate_sg_notice(true, 60);
        assert!(matches!(
            result,
            Err(FamilyLawError::SpecialGuardianshipNoticeNotGiven { .. })
        ));
    }

    #[test]
    fn test_validate_sg_notice_sufficient() {
        let result = validate_sg_notice(true, 90);
        assert!(result.is_ok());
    }
}
