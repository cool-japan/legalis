//! Mining and resources validation
//!
//! Implements validation for mining tenures, environmental requirements,
//! and native title compliance.

use super::error::{MiningError, Result};
use super::types::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Tenure validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenureValidationResult {
    /// Tenure ID
    pub tenure_id: String,
    /// Is valid
    pub is_valid: bool,
    /// Validation issues
    pub issues: Vec<ValidationIssue>,
    /// Native title compliant
    pub native_title_compliant: bool,
    /// Environmental compliant
    pub environmental_compliant: bool,
    /// Rental current
    pub rental_current: bool,
    /// Days until expiry
    pub days_until_expiry: Option<i64>,
}

/// Validation issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Issue category
    pub category: IssueCategory,
    /// Description
    pub description: String,
    /// Severity
    pub severity: IssueSeverity,
    /// Remediation
    pub remediation: String,
}

/// Issue category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IssueCategory {
    /// Tenure status
    TenureStatus,
    /// Native title
    NativeTitle,
    /// Environmental
    Environmental,
    /// Financial
    Financial,
    /// Safety
    Safety,
    /// Reporting
    Reporting,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Information only
    Info,
    /// Warning
    Warning,
    /// Error - must be addressed
    Error,
    /// Critical - immediate action required
    Critical,
}

/// Validate mining tenure
pub fn validate_tenure(tenure: &MiningTenure, current_date: NaiveDate) -> TenureValidationResult {
    let mut issues = Vec::new();
    let mut is_valid = true;

    // Check status
    if !matches!(
        tenure.status,
        TenureStatus::Current | TenureStatus::UnderRenewal
    ) {
        issues.push(ValidationIssue {
            category: IssueCategory::TenureStatus,
            description: format!("Tenure status is {:?}, not current", tenure.status),
            severity: IssueSeverity::Critical,
            remediation: "Apply for renewal or new tenure".to_string(),
        });
        is_valid = false;
    }

    // Check expiry
    let days_until_expiry = (tenure.expiry_date - current_date).num_days();
    if days_until_expiry < 0 {
        issues.push(ValidationIssue {
            category: IssueCategory::TenureStatus,
            description: "Tenure has expired".to_string(),
            severity: IssueSeverity::Critical,
            remediation: "Apply for renewal immediately".to_string(),
        });
        is_valid = false;
    } else if days_until_expiry < 90 {
        issues.push(ValidationIssue {
            category: IssueCategory::TenureStatus,
            description: format!("Tenure expires in {} days", days_until_expiry),
            severity: IssueSeverity::Warning,
            remediation: "Submit renewal application before expiry".to_string(),
        });
    }

    // Check rental
    if !tenure.rental_paid {
        issues.push(ValidationIssue {
            category: IssueCategory::Financial,
            description: "Rental not paid".to_string(),
            severity: IssueSeverity::Error,
            remediation: "Pay outstanding rental to avoid forfeiture".to_string(),
        });
        is_valid = false;
    }

    // Check native title
    let native_title_compliant = check_native_title_compliance(tenure);
    if !native_title_compliant {
        issues.push(ValidationIssue {
            category: IssueCategory::NativeTitle,
            description: "Native title requirements not met".to_string(),
            severity: IssueSeverity::Error,
            remediation: "Complete native title future act process".to_string(),
        });
        is_valid = false;
    }

    // Check environmental approvals for mining leases
    let environmental_compliant = if tenure.tenure_type.allows_production() {
        check_environmental_compliance(tenure)
    } else {
        true
    };
    if !environmental_compliant {
        issues.push(ValidationIssue {
            category: IssueCategory::Environmental,
            description: "Required environmental approvals not in place".to_string(),
            severity: IssueSeverity::Error,
            remediation: "Obtain required environmental approvals before production".to_string(),
        });
        is_valid = false;
    }

    // Check financial assurance for production tenures
    if tenure.tenure_type.allows_production() && tenure.financial_assurance.is_none() {
        issues.push(ValidationIssue {
            category: IssueCategory::Financial,
            description: "Financial assurance/rehabilitation bond not provided".to_string(),
            severity: IssueSeverity::Error,
            remediation: "Provide required financial assurance before operations".to_string(),
        });
        is_valid = false;
    }

    TenureValidationResult {
        tenure_id: tenure.tenure_id.clone(),
        is_valid,
        issues,
        native_title_compliant,
        environmental_compliant,
        rental_current: tenure.rental_paid,
        days_until_expiry: Some(days_until_expiry),
    }
}

/// Check native title compliance
fn check_native_title_compliance(tenure: &MiningTenure) -> bool {
    matches!(
        tenure.native_title_status,
        NativeTitleStatus::NotApplicable
            | NativeTitleStatus::Extinguished
            | NativeTitleStatus::IluaInPlace
    )
}

/// Check environmental compliance
fn check_environmental_compliance(tenure: &MiningTenure) -> bool {
    // For production tenures, need at least state approval
    tenure.environmental_approvals.iter().any(|a| {
        matches!(
            a.approval_type,
            EnvironmentalApprovalType::StateEiaApproval
                | EnvironmentalApprovalType::EnvironmentalAuthority
        )
    })
}

/// Environmental impact assessment requirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EiaRequirement {
    /// Is EPBC referral required
    pub epbc_referral_required: bool,
    /// EPBC triggers
    pub epbc_triggers: Vec<EpbcTrigger>,
    /// State assessment level
    pub state_assessment_level: StateAssessmentLevel,
    /// Reason for assessment level
    pub assessment_reason: String,
}

/// EPBC Act trigger (Matter of National Environmental Significance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpbcTrigger {
    /// World heritage properties
    WorldHeritage,
    /// National heritage places
    NationalHeritage,
    /// Wetlands of international importance (Ramsar)
    Ramsar,
    /// Listed threatened species and communities
    ThreatenedSpecies,
    /// Listed migratory species
    MigratorySpecies,
    /// Nuclear actions
    NuclearActions,
    /// Great Barrier Reef Marine Park
    GreatBarrierReef,
    /// Water resources (coal seam gas/large coal mining)
    WaterResources,
}

/// State-level environmental assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateAssessmentLevel {
    /// No assessment required
    None,
    /// Standard environmental authority
    Standard,
    /// Environmental impact statement (EIS)
    Eis,
    /// Coordinated assessment (with Commonwealth)
    Coordinated,
    /// Major project declaration
    MajorProject,
}

/// Determine EIA requirements
pub fn determine_eia_requirements(
    project: &MiningProject,
    minerals: &[MineralType],
    area_hectares: f64,
    near_protected_area: bool,
    potential_species_impact: bool,
) -> EiaRequirement {
    let mut epbc_triggers = Vec::new();
    let mut epbc_required = false;

    // Check for EPBC triggers
    if minerals.contains(&MineralType::Uranium) {
        epbc_triggers.push(EpbcTrigger::NuclearActions);
        epbc_required = true;
    }

    if minerals.contains(&MineralType::Coal) && project.primary_mineral == MineralType::Coal {
        epbc_triggers.push(EpbcTrigger::WaterResources);
        epbc_required = true;
    }

    if potential_species_impact {
        epbc_triggers.push(EpbcTrigger::ThreatenedSpecies);
        epbc_required = true;
    }

    if near_protected_area {
        epbc_triggers.push(EpbcTrigger::NationalHeritage);
        epbc_required = true;
    }

    // Determine state assessment level
    let state_assessment_level = if area_hectares > 1000.0
        || minerals.contains(&MineralType::Uranium)
        || minerals.contains(&MineralType::Coal)
    {
        StateAssessmentLevel::Eis
    } else if area_hectares > 100.0 {
        StateAssessmentLevel::Standard
    } else {
        StateAssessmentLevel::None
    };

    let assessment_reason = if epbc_required {
        format!(
            "EPBC referral required due to: {}",
            epbc_triggers
                .iter()
                .map(|t| format!("{:?}", t))
                .collect::<Vec<_>>()
                .join(", ")
        )
    } else if state_assessment_level != StateAssessmentLevel::None {
        format!(
            "State {:?} assessment required based on project scale and type",
            state_assessment_level
        )
    } else {
        "Standard approvals only".to_string()
    };

    EiaRequirement {
        epbc_referral_required: epbc_required,
        epbc_triggers,
        state_assessment_level,
        assessment_reason,
    }
}

/// Validate exploration programme
pub fn validate_exploration_programme(
    tenure: &MiningTenure,
    proposed_activities: &[ExplorationActivity],
    budget_aud: f64,
) -> Result<ExplorationValidationResult> {
    let mut issues = Vec::new();
    let mut requires_additional_approval = false;
    let mut required_approvals = Vec::new();

    // Check tenure allows exploration
    if !tenure.tenure_type.allows_exploration() {
        return Err(MiningError::ExplorationProgrammeNotApproved {
            state: format!("{:?}", tenure.jurisdiction),
            missing: "Tenure does not allow exploration activities".to_string(),
        });
    }

    // Check minimum expenditure (varies by jurisdiction, using indicative)
    let min_expenditure_per_ha = 50.0; // $50/ha indicative
    let required_expenditure = tenure.area_hectares * min_expenditure_per_ha;
    if budget_aud < required_expenditure {
        issues.push(format!(
            "Budget ${:.2} below minimum commitment ${:.2}",
            budget_aud, required_expenditure
        ));
    }

    // Check activities for additional approvals
    for activity in proposed_activities {
        if activity.requires_native_veg_clearing {
            requires_additional_approval = true;
            required_approvals.push("Native vegetation clearing permit".to_string());
        }
        if activity.requires_water_licence {
            requires_additional_approval = true;
            required_approvals.push("Water licence/permit".to_string());
        }
        if activity.ground_disturbance_area_hectares > 10.0 {
            requires_additional_approval = true;
            required_approvals.push("Work programme approval".to_string());
        }
    }

    Ok(ExplorationValidationResult {
        is_valid: issues.is_empty(),
        issues,
        minimum_expenditure_met: budget_aud >= required_expenditure,
        requires_additional_approval,
        required_approvals,
    })
}

/// Exploration activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplorationActivity {
    /// Activity type
    pub activity_type: ExplorationActivityType,
    /// Ground disturbance area
    pub ground_disturbance_area_hectares: f64,
    /// Requires native vegetation clearing
    pub requires_native_veg_clearing: bool,
    /// Requires water licence
    pub requires_water_licence: bool,
    /// Drilling metres (if drilling)
    pub drilling_metres: Option<f64>,
}

/// Exploration activity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExplorationActivityType {
    /// Geological mapping
    GeologicalMapping,
    /// Geochemical sampling
    GeochemicalSampling,
    /// Geophysical survey
    GeophysicalSurvey,
    /// Drilling (RC)
    DrillingRc,
    /// Drilling (Diamond)
    DrillingDiamond,
    /// Costeaning/trenching
    Costeaning,
    /// Bulk sampling
    BulkSampling,
}

/// Exploration validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplorationValidationResult {
    /// Is valid
    pub is_valid: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Minimum expenditure met
    pub minimum_expenditure_met: bool,
    /// Requires additional approval
    pub requires_additional_approval: bool,
    /// Required approvals
    pub required_approvals: Vec<String>,
}

/// Calculate royalty
pub fn calculate_royalty(
    mineral: MineralType,
    jurisdiction: MiningJurisdiction,
    quantity: f64,
    unit_value: f64,
) -> RoyaltyCalculation {
    // Simplified royalty rates (actual rates vary significantly)
    let (royalty_type, rate) = match (mineral, jurisdiction) {
        (MineralType::Gold, MiningJurisdiction::WA) => (RoyaltyType::AdValorem, 0.025), // 2.5%
        (MineralType::IronOre, MiningJurisdiction::WA) => (RoyaltyType::AdValorem, 0.075), // 7.5%
        (MineralType::Coal, MiningJurisdiction::QLD) => (RoyaltyType::AdValorem, 0.07), // 7% base
        (MineralType::Coal, MiningJurisdiction::NSW) => (RoyaltyType::AdValorem, 0.085), // 8.5%
        (MineralType::Lithium, _) => (RoyaltyType::AdValorem, 0.05),                    // 5%
        _ => (RoyaltyType::AdValorem, 0.05), // 5% default
    };

    let base_value = quantity * unit_value;
    let royalty_amount = base_value * rate;

    RoyaltyCalculation {
        mineral,
        royalty_type,
        rate,
        base_value,
        royalty_amount,
        period: String::new(),
    }
}

/// Validate mine closure plan
pub fn validate_closure_plan(
    plan: &MineClosurePlan,
    total_disturbance_hectares: f64,
    current_rehabilitation_hectares: f64,
) -> ClosurePlanValidationResult {
    let mut issues = Vec::new();

    // Check rehabilitation progress
    let expected_progress = total_disturbance_hectares * 0.1; // 10% progressive
    if current_rehabilitation_hectares < expected_progress {
        issues.push(format!(
            "Progressive rehabilitation behind schedule: {:.1}ha vs {:.1}ha expected",
            current_rehabilitation_hectares, expected_progress
        ));
    }

    // Check cost estimate adequacy
    let min_cost_per_hectare = 50_000.0; // $50k/ha indicative
    let min_closure_cost = plan.rehabilitation_area_hectares * min_cost_per_hectare;
    let cost_adequate = plan.estimated_closure_cost >= min_closure_cost * 0.8;
    if !cost_adequate {
        issues.push(format!(
            "Closure cost estimate may be understated: ${:.0} vs ${:.0} minimum",
            plan.estimated_closure_cost, min_closure_cost
        ));
    }

    // Check completion criteria
    if !plan.completion_criteria_defined {
        issues.push("Completion criteria not defined".to_string());
    }

    // Check post-closure monitoring
    if plan.post_closure_monitoring_years < 10 {
        issues.push(format!(
            "Post-closure monitoring period {} years may be insufficient (10+ recommended)",
            plan.post_closure_monitoring_years
        ));
    }

    ClosurePlanValidationResult {
        is_adequate: issues.is_empty(),
        issues,
        progressive_rehabilitation_on_track: current_rehabilitation_hectares >= expected_progress,
        cost_estimate_adequate: cost_adequate,
        completion_criteria_defined: plan.completion_criteria_defined,
    }
}

/// Closure plan validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosurePlanValidationResult {
    /// Is adequate
    pub is_adequate: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Progressive rehabilitation on track
    pub progressive_rehabilitation_on_track: bool,
    /// Cost estimate adequate
    pub cost_estimate_adequate: bool,
    /// Completion criteria defined
    pub completion_criteria_defined: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tenure() -> MiningTenure {
        MiningTenure {
            tenure_id: "EL12345".to_string(),
            tenure_type: TenureType::ExplorationLicence,
            jurisdiction: MiningJurisdiction::WA,
            holders: vec![TenureHolder {
                name: "Test Mining Pty Ltd".to_string(),
                interest_percentage: 100.0,
                abn: Some("12345678901".to_string()),
                is_operator: true,
            }],
            grant_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            expiry_date: NaiveDate::from_ymd_opt(2027, 1, 1).unwrap(),
            area_hectares: 200.0,
            minerals: vec![MineralType::Gold, MineralType::Copper],
            status: TenureStatus::Current,
            native_title_status: NativeTitleStatus::IluaInPlace,
            environmental_approvals: vec![],
            rental_paid: true,
            financial_assurance: None,
        }
    }

    #[test]
    fn test_validate_tenure_valid() {
        let tenure = create_test_tenure();
        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let result = validate_tenure(&tenure, current);
        assert!(result.is_valid);
        assert!(result.native_title_compliant);
    }

    #[test]
    fn test_validate_tenure_expired() {
        let mut tenure = create_test_tenure();
        tenure.expiry_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = validate_tenure(&tenure, current);

        assert!(!result.is_valid);
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.description.contains("expired"))
        );
    }

    #[test]
    fn test_determine_eia_requirements_uranium() {
        let project = MiningProject {
            name: "Test Project".to_string(),
            operator: "Test Mining".to_string(),
            phase: ProjectPhase::Feasibility,
            tenures: vec!["ML12345".to_string()],
            primary_mineral: MineralType::Uranium,
            estimated_mine_life: Some(15),
            annual_production_capacity: None,
            workforce: Some(200),
            rehabilitation_liability: 50_000_000.0,
            financial_assurance_held: 25_000_000.0,
        };

        let result =
            determine_eia_requirements(&project, &[MineralType::Uranium], 500.0, false, false);

        assert!(result.epbc_referral_required);
        assert!(result.epbc_triggers.contains(&EpbcTrigger::NuclearActions));
    }

    #[test]
    fn test_calculate_royalty_gold_wa() {
        let result = calculate_royalty(MineralType::Gold, MiningJurisdiction::WA, 1000.0, 2500.0);

        assert_eq!(result.royalty_type, RoyaltyType::AdValorem);
        assert_eq!(result.rate, 0.025);
        assert_eq!(result.base_value, 2_500_000.0);
        assert_eq!(result.royalty_amount, 62_500.0);
    }

    #[test]
    fn test_validate_closure_plan() {
        let plan = MineClosurePlan {
            version: "1.0".to_string(),
            approval_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            estimated_closure_date: Some(NaiveDate::from_ymd_opt(2040, 1, 1).unwrap()),
            rehabilitation_area_hectares: 500.0,
            estimated_closure_cost: 30_000_000.0,
            progressive_rehabilitation_percent: 15.0,
            completion_criteria_defined: true,
            post_closure_monitoring_years: 15,
        };

        let result = validate_closure_plan(&plan, 400.0, 50.0);
        assert!(result.cost_estimate_adequate);
        assert!(result.completion_criteria_defined);
    }
}
