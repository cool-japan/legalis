//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::types::{CoverageInfo, ImpactAssessment, RiskLevel, StatuteConflict, StatuteVerifier};
use super::types_3::{ConflictType, PrincipleCheckResult};
use super::types_4::{ComplexityMetrics, ImpactLevel, Severity};
use super::types_5::{ComplexityLevel, VerificationResult};
/// Performs a comprehensive equality check on a statute.
///
/// Checks for potential discrimination based on protected attributes
/// like age, gender, race, etc.
pub fn check_equality(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    for condition in &statute.preconditions {
        if let legalis_core::Condition::Age { operator, value } = condition {
            use legalis_core::ComparisonOp;
            match operator {
                ComparisonOp::GreaterThan | ComparisonOp::GreaterOrEqual if *value > 65 => {
                    issues.push(format!(
                        "Potential age discrimination: requires age > {}",
                        value
                    ));
                }
                ComparisonOp::LessThan | ComparisonOp::LessOrEqual if *value < 18 => {
                    issues.push(format!(
                        "Potential age discrimination: requires age < {}",
                        value
                    ));
                }
                _ => {}
            }
        }
    }
    for condition in &statute.preconditions {
        if let legalis_core::Condition::HasAttribute { key } = condition {
            let key_lower = key.to_lowercase();
            if key_lower.contains("gender")
                || key_lower.contains("race")
                || key_lower.contains("religion")
                || key_lower.contains("nationality")
            {
                issues.push(format!(
                    "Potential discrimination based on protected attribute: {}",
                    key
                ));
            }
        }
    }
    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues).with_suggestion(
            "Review for potential discrimination and ensure legitimate justification exists",
        )
    }
}
/// Performs due process verification on a statute.
///
/// Checks that adequate procedural safeguards are in place.
pub fn check_due_process(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let has_discretion = statute.discretion_logic.is_some();
    use legalis_core::EffectType;
    match statute.effect.effect_type {
        EffectType::Revoke | EffectType::Prohibition if !has_discretion => {
            issues.push(
                "Statute revokes/prohibits without discretionary review mechanism".to_string(),
            );
        }
        _ => {}
    }
    if statute.preconditions.is_empty() && !has_discretion {
        match statute.effect.effect_type {
            EffectType::Revoke | EffectType::Prohibition => {
                issues.push("No preconditions or review process for punitive action".to_string());
            }
            _ => {}
        }
    }
    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues)
            .with_suggestion("Add discretionary review or appeal mechanism for fairness")
    }
}
/// Performs privacy impact assessment on a statute.
///
/// Checks if the statute might impact individual privacy.
pub fn check_privacy_impact(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::HasAttribute { key }
            | legalis_core::Condition::AttributeEquals { key, .. } => {
                let key_lower = key.to_lowercase();
                if key_lower.contains("medical")
                    || key_lower.contains("health")
                    || key_lower.contains("financial")
                    || key_lower.contains("biometric")
                    || key_lower.contains("location")
                    || key_lower.contains("address")
                {
                    issues.push(format!(
                        "Processes potentially sensitive personal data: {}",
                        key
                    ));
                    suggestions
                        .push("Ensure data minimization and appropriate safeguards".to_string());
                }
            }
            legalis_core::Condition::Geographic { .. } => {
                issues.push("Uses geographic/location data which may be sensitive".to_string());
                suggestions.push("Consider privacy implications of location tracking".to_string());
            }
            _ => {}
        }
    }
    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Performs proportionality checking on a statute.
///
/// Checks if the statute's effects are proportional to its conditions.
pub fn check_proportionality(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    use legalis_core::EffectType;
    let condition_count = statute.preconditions.len();
    match statute.effect.effect_type {
        EffectType::Revoke | EffectType::Prohibition if condition_count < 2 => {
            issues.push(format!(
                "Severe effect ({:?}) based on only {} condition(s) - may be disproportionate",
                statute.effect.effect_type, condition_count
            ));
        }
        EffectType::Obligation if condition_count == 0 => {
            issues.push(
                "Obligation imposed without any preconditions - may be disproportionate"
                    .to_string(),
            );
        }
        _ => {}
    }
    if matches!(statute.effect.effect_type, EffectType::Grant) && condition_count > 5 {
        issues.push(format!(
            "Simple grant has {} conditions - may create unnecessary barriers",
            condition_count
        ));
    }
    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues)
            .with_suggestion("Ensure effects are proportional to conditions and legitimate aims")
    }
}
/// Performs accessibility verification on a statute.
///
/// Checks if the statute creates barriers for people with disabilities
/// or other accessibility concerns.
pub fn check_accessibility(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::HasAttribute { key } => {
                let key_lower = key.to_lowercase();
                if key_lower.contains("physical")
                    || key_lower.contains("mobility")
                    || key_lower.contains("vision")
                    || key_lower.contains("hearing")
                {
                    issues.push(format!(
                        "Condition based on physical ability may create accessibility barrier: {}",
                        key
                    ));
                    suggestions.push(
                        "Consider reasonable accommodations for people with disabilities"
                            .to_string(),
                    );
                }
                if key_lower.contains("digital")
                    || key_lower.contains("online")
                    || key_lower.contains("internet")
                {
                    issues
                        .push(
                            format!(
                                "Digital requirement may create barrier for those without internet access: {}",
                                key
                            ),
                        );
                    suggestions
                        .push("Provide alternative non-digital methods of compliance".to_string());
                }
                if key_lower.contains("language") || key_lower.contains("english") {
                    issues.push(format!("Language requirement may create barrier: {}", key));
                    suggestions
                        .push("Provide translation services or multilingual support".to_string());
                }
            }
            legalis_core::Condition::Geographic { region_id, .. } if !region_id.is_empty() => {
                issues.push(format!(
                    "Geographic restriction may limit accessibility: {}",
                    region_id
                ));
                suggestions.push(
                    "Consider remote participation options or multiple locations".to_string(),
                );
            }
            legalis_core::Condition::Income { operator, value } => {
                use legalis_core::ComparisonOp;
                if matches!(
                    operator,
                    ComparisonOp::GreaterThan | ComparisonOp::GreaterOrEqual
                ) {
                    issues.push(format!(
                        "Income requirement may create financial barrier: requires income >= {}",
                        value
                    ));
                    suggestions.push(
                        "Consider fee waivers or sliding scale based on ability to pay".to_string(),
                    );
                }
            }
            _ => {}
        }
    }
    if statute.discretion_logic.is_some() {
        suggestions
            .push("Discretion allows for individual accessibility accommodations".to_string());
    }
    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Checks if the statute violates the principle of non-retroactivity (ex post facto).
///
/// The principle of non-retroactivity means laws should not apply to conduct
/// that occurred before the law came into effect. This is especially important for:
/// - Criminal laws and prohibitions
/// - New obligations and duties
/// - Revocation of rights
pub fn check_retroactivity(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    if let Some(effective_date) = statute.temporal_validity.effective_date {
        let is_restrictive_effect = matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
                | legalis_core::EffectType::Revoke
        );
        if is_restrictive_effect {
            let effect_desc_lower = statute.effect.description.to_lowercase();
            if effect_desc_lower.contains("retroactive")
                || effect_desc_lower.contains("retrospective")
                || effect_desc_lower.contains("prior to")
                || effect_desc_lower.contains("before")
            {
                issues.push(format!(
                    "{:?} effect appears to apply retroactively: '{}'",
                    statute.effect.effect_type, statute.effect.description
                ));
                suggestions.push(
                    "Consider prospective application only (applying from effective date forward)"
                        .to_string(),
                );
            }
            if let Some(retroactive_param) = statute.effect.parameters.get("retroactive")
                && (retroactive_param.to_lowercase() == "true"
                    || retroactive_param.to_lowercase() == "yes")
            {
                issues
                    .push(
                        format!(
                            "{:?} effect is marked as retroactive, which may violate ex post facto principles",
                            statute.effect.effect_type
                        ),
                    );
                suggestions.push(format!(
                    "Ensure effective date ({}) is not applied to conduct before that date",
                    effective_date
                ));
            }
            if let Some(application_date_str) = statute.effect.parameters.get("application_date")
                && let Ok(application_date) =
                    chrono::NaiveDate::parse_from_str(application_date_str, "%Y-%m-%d")
                && application_date < effective_date
            {
                issues
                        .push(
                            format!(
                                "Application date ({}) precedes effective date ({}), creating retroactive effect",
                                application_date, effective_date
                            ),
                        );
                suggestions.push("Align application date with or after effective date".to_string());
            }
        }
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::MonetaryTransfer
        ) {
            let effect_desc_lower = statute.effect.description.to_lowercase();
            let is_penalty = effect_desc_lower.contains("fine")
                || effect_desc_lower.contains("penalty")
                || effect_desc_lower.contains("sanction");
            if is_penalty
                && let Some(retroactive_param) = statute.effect.parameters.get("retroactive")
                && retroactive_param.to_lowercase() == "true"
            {
                issues
                    .push(
                        "Monetary penalty appears to apply retroactively, violating ex post facto principles"
                            .to_string(),
                    );
                suggestions.push(
                    "Apply penalties only to violations occurring after the effective date"
                        .to_string(),
                );
            }
        }
    } else {
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
                | legalis_core::EffectType::Revoke
        ) {
            suggestions.push(
                "Consider specifying an effective date to ensure prospective application"
                    .to_string(),
            );
        }
    }
    if let (Some(enacted_at), Some(effective_date)) = (
        statute.temporal_validity.enacted_at,
        statute.temporal_validity.effective_date,
    ) {
        let enacted_date = enacted_at.date_naive();
        if effective_date < enacted_date {
            issues
                .push(
                    format!(
                        "Effective date ({}) is before enactment date ({}), creating improper retroactive application",
                        effective_date, enacted_date
                    ),
                );
            suggestions.push("Effective date should be on or after enactment date".to_string());
        } else if effective_date == enacted_date
            && matches!(
                statute.effect.effect_type,
                legalis_core::EffectType::Prohibition | legalis_core::EffectType::Obligation
            )
        {
            suggestions
                .push(
                    "Consider providing a grace period between enactment and effective date for compliance"
                        .to_string(),
                );
        }
    }
    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Checks freedom of expression principles.
///
/// Verifies that statutes do not unduly restrict speech, press, assembly,
/// or other expressive activities without compelling justification.
pub fn check_freedom_of_expression(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    if matches!(
        statute.effect.effect_type,
        legalis_core::EffectType::Prohibition | legalis_core::EffectType::Obligation
    ) {
        let effect_desc_lower = statute.effect.description.to_lowercase();
        let title_lower = statute.title.to_lowercase();
        let speech_keywords = [
            "speech",
            "speak",
            "express",
            "say",
            "publish",
            "broadcast",
            "communicate",
            "media",
            "press",
            "assembly",
            "protest",
            "demonstration",
            "petition",
            "religion",
            "belief",
            "opinion",
        ];
        let affects_expression = speech_keywords
            .iter()
            .any(|keyword| effect_desc_lower.contains(keyword) || title_lower.contains(keyword));
        if affects_expression {
            let has_justification = effect_desc_lower.contains("safety")
                || effect_desc_lower.contains("security")
                || effect_desc_lower.contains("public health")
                || effect_desc_lower.contains("emergency")
                || effect_desc_lower.contains("imminent")
                || effect_desc_lower.contains("violence")
                || statute.discretion_logic.is_some();
            if !has_justification {
                issues
                    .push(
                        format!(
                            "Statute '{}' may restrict freedom of expression without clear compelling justification",
                            statute.id
                        ),
                    );
                suggestions
                    .push(
                        "Add explicit justification for expression restrictions (e.g., public safety, imminent harm)"
                            .to_string(),
                    );
                suggestions.push(
                    "Consider narrow tailoring to minimize impact on protected speech".to_string(),
                );
            } else {
                suggestions.push(
                    "Ensure restrictions are narrowly tailored and use least restrictive means"
                        .to_string(),
                );
            }
            if effect_desc_lower.contains("prior")
                || effect_desc_lower.contains("advance")
                || effect_desc_lower.contains("pre-approval")
                || effect_desc_lower.contains("permit required")
            {
                issues
                    .push(
                        "Statute may impose prior restraint on expression, which is generally unconstitutional"
                            .to_string(),
                    );
                suggestions.push(
                    "Consider post-publication remedies instead of prior restraint".to_string(),
                );
            }
        }
    }
    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Checks property rights principles.
///
/// Verifies that statutes respect property rights and provide just compensation
/// for takings or restrictions on property use.
pub fn check_property_rights(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    let effect_desc_lower = statute.effect.description.to_lowercase();
    let title_lower = statute.title.to_lowercase();
    let property_keywords = [
        "property",
        "land",
        "real estate",
        "possession",
        "ownership",
        "seizure",
        "confiscation",
        "taking",
        "eminent domain",
        "expropriation",
        "restriction",
        "use",
        "development",
    ];
    let affects_property = property_keywords
        .iter()
        .any(|keyword| effect_desc_lower.contains(keyword) || title_lower.contains(keyword));
    if affects_property {
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Revoke
                | legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
        ) {
            let has_compensation = effect_desc_lower.contains("compensation")
                || effect_desc_lower.contains("reimbursement")
                || effect_desc_lower.contains("payment")
                || statute
                    .effect
                    .parameters
                    .contains_key("compensation_amount");
            if (effect_desc_lower.contains("taking")
                || effect_desc_lower.contains("seiz")
                || effect_desc_lower.contains("confiscat"))
                && !has_compensation
            {
                issues.push(format!(
                    "Statute '{}' may involve property taking without just compensation",
                    statute.id
                ));
                suggestions.push(
                    "Provide just compensation for property takings as required by law".to_string(),
                );
            }
            if effect_desc_lower.contains("prohibit")
                || effect_desc_lower.contains("restrict")
                || effect_desc_lower.contains("limit")
            {
                suggestions
                    .push(
                        "Consider whether regulatory restrictions constitute a taking requiring compensation"
                            .to_string(),
                    );
                suggestions.push(
                    "Ensure restrictions permit economically viable use of property".to_string(),
                );
            }
        }
        if matches!(statute.effect.effect_type, legalis_core::EffectType::Revoke) {
            let has_procedure = effect_desc_lower.contains("hearing")
                || effect_desc_lower.contains("notice")
                || effect_desc_lower.contains("appeal")
                || statute.discretion_logic.is_some();
            if !has_procedure {
                issues.push(
                    "Property deprivation may lack adequate procedural safeguards".to_string(),
                );
                suggestions.push(
                    "Provide notice and opportunity for hearing before property deprivation"
                        .to_string(),
                );
            }
        }
    }
    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Checks procedural due process principles (detailed).
///
/// Verifies that statutes provide adequate procedural safeguards when depriving
/// individuals of life, liberty, or property interests.
pub fn check_procedural_due_process(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    let requires_due_process = matches!(
        statute.effect.effect_type,
        legalis_core::EffectType::Revoke
            | legalis_core::EffectType::Prohibition
            | legalis_core::EffectType::MonetaryTransfer
    );
    if requires_due_process {
        let effect_desc_lower = statute.effect.description.to_lowercase();
        let has_notice = effect_desc_lower.contains("notice")
            || effect_desc_lower.contains("notification")
            || effect_desc_lower.contains("inform");
        let has_hearing = effect_desc_lower.contains("hearing")
            || effect_desc_lower.contains("proceeding")
            || effect_desc_lower.contains("tribunal");
        let has_representation = effect_desc_lower.contains("represent")
            || effect_desc_lower.contains("counsel")
            || effect_desc_lower.contains("attorney")
            || effect_desc_lower.contains("lawyer");
        let has_appeal = effect_desc_lower.contains("appeal")
            || effect_desc_lower.contains("review")
            || effect_desc_lower.contains("reconsideration");
        let has_evidence = effect_desc_lower.contains("evidence")
            || effect_desc_lower.contains("testimony")
            || effect_desc_lower.contains("witness");
        if !has_notice {
            issues.push("Statute lacks explicit notice requirement before deprivation".to_string());
            suggestions
                .push("Add requirement for adequate notice before taking action".to_string());
        }
        if !has_hearing {
            issues.push("Statute lacks explicit hearing or opportunity to be heard".to_string());
            suggestions
                .push("Provide opportunity for hearing before final deprivation".to_string());
        }
        if !has_representation {
            suggestions
                .push("Consider allowing right to legal representation in proceedings".to_string());
        }
        if !has_appeal {
            suggestions.push(
                "Consider providing appeal or review mechanism for adverse decisions".to_string(),
            );
        }
        if !has_evidence {
            suggestions.push(
                "Allow parties to present evidence and confront adverse evidence".to_string(),
            );
        }
        if statute.discretion_logic.is_some() {
            suggestions.push(
                "Ensure decision-makers are impartial and free from conflicts of interest"
                    .to_string(),
            );
        }
        if !effect_desc_lower.contains("timely")
            && !effect_desc_lower.contains("prompt")
            && !effect_desc_lower.contains("within")
        {
            suggestions
                .push("Specify timeframes to ensure timely resolution of proceedings".to_string());
        }
    }
    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Checks equal protection principles (comprehensive).
///
/// Verifies that statutes treat similarly situated persons equally and
/// that any differential treatment has adequate justification.
pub fn check_equal_protection(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    let mut classifications = Vec::new();
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::Age { .. } => {
                classifications.push(("age", "intermediate scrutiny"));
            }
            legalis_core::Condition::AttributeEquals { key, value } => {
                let key_lower = key.to_lowercase();
                let value_lower = value.to_lowercase();
                let combined = format!("{} {}", key_lower, value_lower);
                if combined.contains("race")
                    || combined.contains("national origin")
                    || combined.contains("ethnicity")
                {
                    classifications.push(("race/national origin", "strict scrutiny"));
                    issues
                        .push(
                            format!(
                                "Classification based on race/national origin in '{}: {}' requires strict scrutiny",
                                key, value
                            ),
                        );
                }
                if combined.contains("religion") || combined.contains("religious") {
                    classifications.push(("religion", "strict scrutiny"));
                    issues.push(format!(
                        "Classification based on religion in '{}: {}' requires strict scrutiny",
                        key, value
                    ));
                }
                if combined.contains("gender")
                    || combined.contains("sex")
                    || key_lower == "gender"
                    || key_lower == "sex"
                {
                    classifications.push(("gender/sex", "intermediate scrutiny"));
                    suggestions.push(format!(
                        "Gender classification in '{}: {}' requires substantial justification",
                        key, value
                    ));
                }
                if combined.contains("citizenship") || combined.contains("alien") {
                    classifications.push(("citizenship", "intermediate scrutiny"));
                    suggestions.push(format!(
                        "Citizenship classification in '{}: {}' may require heightened scrutiny",
                        key, value
                    ));
                }
            }
            legalis_core::Condition::Income { .. } => {
                classifications.push(("economic status", "rational basis"));
                suggestions.push("Ensure economic classifications have rational basis".to_string());
            }
            _ => {}
        }
    }
    let effect_desc_lower = statute.effect.description.to_lowercase();
    if effect_desc_lower.contains("discriminat")
        || effect_desc_lower.contains("preferential")
        || effect_desc_lower.contains("exclusive")
    {
        issues.push("Effect description suggests potential discriminatory treatment".to_string());
        suggestions.push(
            "Ensure any differential treatment serves important governmental interest".to_string(),
        );
    }
    if !classifications.is_empty() {
        suggestions.push(format!(
            "Statute contains {} classification(s) requiring review",
            classifications.len()
        ));
        for (classification, standard) in &classifications {
            if standard == &"strict scrutiny" {
                suggestions
                    .push(
                        format!(
                            "For {} classification: Must serve compelling governmental interest and be narrowly tailored",
                            classification
                        ),
                    );
            } else if standard == &"intermediate scrutiny" {
                suggestions
                    .push(
                        format!(
                            "For {} classification: Must serve important governmental interest and be substantially related",
                            classification
                        ),
                    );
            }
        }
    }
    if statute.preconditions.len() > 3 && statute.discretion_logic.is_none() {
        suggestions.push(
            "Complex preconditions without discretion logic may create arbitrary distinctions"
                .to_string(),
        );
        suggestions.push(
            "Consider adding discretion logic to explain classification rationale".to_string(),
        );
    }
    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}
/// Performs an impact assessment on a statute.
///
/// Analyzes the potential impacts of a statute on various groups and dimensions.
pub fn assess_impact(statute: &Statute) -> ImpactAssessment {
    let mut assessment = ImpactAssessment::new();
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::Age { value, .. } => {
                if *value < 18 {
                    assessment.affected_groups.push("Minors".to_string());
                } else if *value >= 65 {
                    assessment.affected_groups.push("Seniors".to_string());
                } else {
                    assessment.affected_groups.push("Adults".to_string());
                }
            }
            legalis_core::Condition::Income { .. } => {
                assessment
                    .affected_groups
                    .push("Income earners".to_string());
                assessment.economic_impact = ImpactLevel::High;
            }
            legalis_core::Condition::HasAttribute { key } => {
                let key_lower = key.to_lowercase();
                if key_lower.contains("disabled") || key_lower.contains("disability") {
                    assessment
                        .affected_groups
                        .push("People with disabilities".to_string());
                }
                if key_lower.contains("veteran") {
                    assessment.affected_groups.push("Veterans".to_string());
                }
                if key_lower.contains("student") {
                    assessment.affected_groups.push("Students".to_string());
                }
            }
            legalis_core::Condition::Geographic { region_id, .. } => {
                assessment
                    .affected_groups
                    .push(format!("Residents of {}", region_id));
            }
            _ => {}
        }
    }
    use legalis_core::EffectType;
    match statute.effect.effect_type {
        EffectType::Grant => {
            assessment
                .positive_impacts
                .push(format!("Grants benefit: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::Medium;
        }
        EffectType::Revoke => {
            assessment
                .negative_impacts
                .push(format!("Revokes benefit: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::High;
            assessment.overall_risk = RiskLevel::High;
        }
        EffectType::Obligation => {
            assessment.negative_impacts.push(format!(
                "Imposes obligation: {}",
                statute.effect.description
            ));
            assessment.social_impact = ImpactLevel::Medium;
        }
        EffectType::Prohibition => {
            assessment
                .negative_impacts
                .push(format!("Prohibits action: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::High;
            assessment.overall_risk = RiskLevel::High;
        }
        _ => {}
    }
    let equality_result = check_equality(statute);
    if !equality_result.passed {
        assessment.equity_concerns.extend(equality_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::High);
    }
    let accessibility_result = check_accessibility(statute);
    if !accessibility_result.passed {
        assessment
            .accessibility_concerns
            .extend(accessibility_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::Medium);
    }
    let privacy_result = check_privacy_impact(statute);
    if !privacy_result.passed {
        assessment.privacy_concerns.extend(privacy_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::Medium);
    }
    let total_concerns = assessment.equity_concerns.len()
        + assessment.accessibility_concerns.len()
        + assessment.privacy_concerns.len();
    if total_concerns > 5 {
        assessment.overall_risk = RiskLevel::Critical;
    } else if total_concerns > 3 {
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::High);
    }
    assessment
}
/// Performs impact assessment on multiple statutes and generates a comprehensive report.
pub fn assess_multiple_impacts(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Comprehensive Impact Assessment\n\n");
    report.push_str(&format!("Analyzed {} statute(s)\n\n", statutes.len()));
    let mut all_groups: HashSet<String> = HashSet::new();
    let mut total_positive = 0;
    let mut total_negative = 0;
    let mut max_risk = RiskLevel::Low;
    for statute in statutes {
        let assessment = assess_impact(statute);
        all_groups.extend(assessment.affected_groups.clone());
        total_positive += assessment.positive_impacts.len();
        total_negative += assessment.negative_impacts.len();
        max_risk = max_risk.max(assessment.overall_risk);
        report.push_str(&format!(
            "## Statute: {} - \"{}\"\n",
            statute.id, statute.title
        ));
        report.push_str(&assessment.report());
        report.push_str("\n---\n\n");
    }
    report.push_str("## Overall Summary\n\n");
    report.push_str(&format!("- **Maximum Risk Level**: {}\n", max_risk));
    report.push_str(&format!(
        "- **Total Affected Groups**: {}\n",
        all_groups.len()
    ));
    report.push_str(&format!(
        "- **Total Positive Impacts**: {}\n",
        total_positive
    ));
    report.push_str(&format!(
        "- **Total Negative Impacts**: {}\n\n",
        total_negative
    ));
    if !all_groups.is_empty() {
        report.push_str("### All Affected Groups:\n");
        for group in &all_groups {
            report.push_str(&format!("- {}\n", group));
        }
    }
    report
}
/// Verifies the integrity of a set of laws.
pub fn verify_integrity(laws: &[Statute]) -> Result<VerificationResult, String> {
    let verifier = StatuteVerifier::new();
    Ok(verifier.verify(laws))
}
/// Detects conflicts between statutes.
pub fn detect_statute_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    conflicts.extend(detect_id_collisions(statutes));
    conflicts.extend(detect_effect_conflicts(statutes));
    conflicts.extend(detect_jurisdictional_overlaps(statutes));
    conflicts.extend(detect_temporal_conflicts(statutes));
    conflicts
}
/// Detects ID collisions (multiple statutes with the same ID).
fn detect_id_collisions(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    let mut id_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, statute) in statutes.iter().enumerate() {
        id_map.entry(statute.id.clone()).or_default().push(idx);
    }
    for (id, indices) in id_map.iter() {
        if indices.len() > 1 {
            let statute_ids: Vec<String> = indices
                .iter()
                .map(|&i| {
                    format!(
                        "{} ({})",
                        statutes[i].id,
                        statutes[i]
                            .jurisdiction
                            .as_ref()
                            .unwrap_or(&"no jurisdiction".to_string())
                    )
                })
                .collect();
            conflicts.push(
                StatuteConflict::new(
                    ConflictType::IdCollision,
                    statute_ids.clone(),
                    format!(
                        "Multiple statutes share the same ID '{}': found {} instances",
                        id,
                        indices.len()
                    ),
                )
                .with_suggestion("Use unique IDs for each statute")
                .with_suggestion("Consider adding jurisdiction prefix to IDs"),
            );
        }
    }
    conflicts
}
/// Detects effect conflicts (overlapping conditions with contradictory effects).
pub(crate) fn detect_effect_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute1 = &statutes[i];
            let statute2 = &statutes[j];
            let same_jurisdiction = match (&statute1.jurisdiction, &statute2.jurisdiction) {
                (Some(j1), Some(j2)) => j1 == j2,
                (None, _) | (_, None) => true,
            };
            if !same_jurisdiction {
                continue;
            }
            if !temporal_validity_overlaps(&statute1.temporal_validity, &statute2.temporal_validity)
            {
                continue;
            }
            let conditions_overlap =
                conditions_overlap(&statute1.preconditions, &statute2.preconditions);
            if conditions_overlap {
                let effects_contradict = effects_contradict(&statute1.effect, &statute2.effect);
                if effects_contradict {
                    conflicts
                        .push(
                            StatuteConflict::new(
                                    ConflictType::EffectConflict,
                                    vec![statute1.id.clone(), statute2.id.clone()],
                                    format!(
                                        "Statutes '{}' and '{}' have overlapping conditions but contradictory effects",
                                        statute1.id, statute2.id
                                    ),
                                )
                                .with_suggestion(
                                    "Add more specific conditions to differentiate the statutes",
                                )
                                .with_suggestion(
                                    "Establish a priority/hierarchy relationship",
                                )
                                .with_suggestion(
                                    "Use temporal validity to separate their applicability",
                                ),
                        );
                }
            }
        }
    }
    conflicts
}
/// Checks if two temporal validity periods overlap.
pub(super) fn temporal_validity_overlaps(
    tv1: &legalis_core::TemporalValidity,
    tv2: &legalis_core::TemporalValidity,
) -> bool {
    use chrono::NaiveDate;
    let start1 = tv1.effective_date;
    let end1 = tv1.expiry_date;
    let start2 = tv2.effective_date;
    let end2 = tv2.expiry_date;
    if start1.is_none() && end1.is_none() {
        return true;
    }
    if start2.is_none() && end2.is_none() {
        return true;
    }
    let start1 = start1.unwrap_or(NaiveDate::MIN);
    let end1 = end1.unwrap_or(NaiveDate::MAX);
    let start2 = start2.unwrap_or(NaiveDate::MIN);
    let end2 = end2.unwrap_or(NaiveDate::MAX);
    start1 <= end2 && start2 <= end1
}
/// Checks if two sets of conditions overlap (have common scenarios).
pub(super) fn conditions_overlap(
    conds1: &[legalis_core::Condition],
    conds2: &[legalis_core::Condition],
) -> bool {
    if conds1.is_empty() && conds2.is_empty() {
        return true;
    }
    if conds1.is_empty() || conds2.is_empty() {
        return true;
    }
    use std::mem::discriminant;
    for c1 in conds1 {
        for c2 in conds2 {
            if discriminant(c1) == discriminant(c2) {
                return true;
            }
        }
    }
    false
}
/// Checks if two effects contradict each other.
pub(super) fn effects_contradict(
    effect1: &legalis_core::Effect,
    effect2: &legalis_core::Effect,
) -> bool {
    use legalis_core::EffectType;
    match (&effect1.effect_type, &effect2.effect_type) {
        (EffectType::Grant, EffectType::Revoke)
        | (EffectType::Revoke, EffectType::Grant)
        | (EffectType::Grant, EffectType::Prohibition)
        | (EffectType::Prohibition, EffectType::Grant) => true,
        (t1, t2) if t1 == t2 => {
            let desc1_lower = effect1.description.to_lowercase();
            let desc2_lower = effect2.description.to_lowercase();
            (desc1_lower.contains("allow") && desc2_lower.contains("prohibit"))
                || (desc1_lower.contains("prohibit") && desc2_lower.contains("allow"))
                || (desc1_lower.contains("grant") && desc2_lower.contains("deny"))
                || (desc1_lower.contains("deny") && desc2_lower.contains("grant"))
        }
        _ => false,
    }
}
/// Detects jurisdictional overlaps.
fn detect_jurisdictional_overlaps(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    let mut jurisdiction_map: HashMap<String, Vec<String>> = HashMap::new();
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            jurisdiction_map
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute.id.clone());
        }
    }
    for (jurisdiction, statute_ids) in jurisdiction_map.iter() {
        if statute_ids.len() > 20 {
            conflicts
                .push(
                    StatuteConflict::new(
                            ConflictType::JurisdictionalOverlap,
                            statute_ids.clone(),
                            format!(
                                "Jurisdiction '{}' has {} statutes, which may indicate overlap or redundancy",
                                jurisdiction, statute_ids.len()
                            ),
                        )
                        .with_suggestion(
                            "Review statutes for consolidation opportunities",
                        )
                        .with_suggestion(
                            "Consider creating sub-jurisdictions for better organization",
                        ),
                );
        }
    }
    conflicts
}
/// Detects temporal conflicts (overlapping time periods with conflicting rules).
fn detect_temporal_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute1 = &statutes[i];
            let statute2 = &statutes[j];
            let related = statute1.jurisdiction == statute2.jurisdiction
                || title_similarity(&statute1.title, &statute2.title) > 0.5;
            if !related {
                continue;
            }
            if temporal_validity_overlaps(&statute1.temporal_validity, &statute2.temporal_validity)
                && statute1.version != statute2.version
            {
                conflicts.push(
                    StatuteConflict::new(
                        ConflictType::TemporalConflict,
                        vec![statute1.id.clone(), statute2.id.clone()],
                        format!(
                            "Statutes '{}' (v{}) and '{}' (v{}) have overlapping validity periods",
                            statute1.id, statute1.version, statute2.id, statute2.version
                        ),
                    )
                    .with_suggestion(
                        "Set expiry date on older version when newer version takes effect",
                    )
                    .with_suggestion(
                        "Use version control and temporal validity to manage transitions",
                    ),
                );
            }
        }
    }
    conflicts
}
/// Calculates simple title similarity (Jaccard similarity of words).
pub(super) fn title_similarity(title1: &str, title2: &str) -> f64 {
    let words1: HashSet<&str> = title1.split_whitespace().collect();
    let words2: HashSet<&str> = title2.split_whitespace().collect();
    if words1.is_empty() && words2.is_empty() {
        return 1.0;
    }
    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}
/// Generates a conflict detection report.
pub fn conflict_detection_report(statutes: &[Statute]) -> String {
    let conflicts = detect_statute_conflicts(statutes);
    let mut report = String::new();
    report.push_str("# Statute Conflict Detection Report\n\n");
    report.push_str(&format!("Analyzed {} statutes\n", statutes.len()));
    report.push_str(&format!("Found {} conflicts\n\n", conflicts.len()));
    if conflicts.is_empty() {
        report.push_str("✓ No conflicts detected.\n");
        return report;
    }
    let mut critical = Vec::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for conflict in &conflicts {
        match conflict.severity {
            Severity::Critical => critical.push(conflict),
            Severity::Error => errors.push(conflict),
            Severity::Warning => warnings.push(conflict),
            _ => {}
        }
    }
    if !critical.is_empty() {
        report.push_str(&format!("## Critical Conflicts ({})\n\n", critical.len()));
        for conflict in critical {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
            if !conflict.resolution_suggestions.is_empty() {
                report.push_str("**Suggestions:**\n");
                for suggestion in &conflict.resolution_suggestions {
                    report.push_str(&format!("- {}\n", suggestion));
                }
                report.push('\n');
            }
        }
    }
    if !errors.is_empty() {
        report.push_str(&format!("## Errors ({})\n\n", errors.len()));
        for conflict in errors {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
        }
    }
    if !warnings.is_empty() {
        report.push_str(&format!("## Warnings ({})\n\n", warnings.len()));
        for conflict in warnings {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
        }
    }
    report
}
/// Analyzes the complexity of a statute.
pub fn analyze_complexity(statute: &Statute) -> ComplexityMetrics {
    let mut metrics = ComplexityMetrics {
        condition_count: statute.preconditions.len(),
        ..Default::default()
    };
    let mut condition_types = HashSet::new();
    for condition in &statute.preconditions {
        let (depth, ops, types) = analyze_condition(condition);
        metrics.condition_depth = metrics.condition_depth.max(depth);
        metrics.logical_operator_count += ops;
        condition_types.extend(types);
    }
    metrics.condition_type_count = condition_types.len();
    metrics.has_discretion = statute.discretion_logic.is_some();
    metrics.cyclomatic_complexity = 1 + metrics.condition_count + metrics.logical_operator_count;
    let mut score: u32 = 0;
    if metrics.condition_count > 1 {
        score += ((metrics.condition_count - 1) * 10).min(30) as u32;
    }
    if metrics.condition_depth > 1 {
        score += ((metrics.condition_depth - 1) * 15).min(30) as u32;
    }
    score += (metrics.logical_operator_count * 8).min(24) as u32;
    if metrics.condition_type_count > 1 {
        score += ((metrics.condition_type_count - 1) * 6).min(12) as u32;
    }
    if metrics.has_discretion {
        score += 10;
    }
    metrics.complexity_score = score.min(100);
    metrics.complexity_level = match metrics.complexity_score {
        0..=25 => ComplexityLevel::Simple,
        26..=50 => ComplexityLevel::Moderate,
        51..=75 => ComplexityLevel::Complex,
        _ => ComplexityLevel::VeryComplex,
    };
    metrics
}
/// Analyzes a condition recursively.
/// Returns (depth, operator_count, condition_types)
fn analyze_condition(condition: &legalis_core::Condition) -> (usize, usize, HashSet<String>) {
    use legalis_core::Condition;
    match condition {
        Condition::Age { .. } => (1, 0, ["Age".to_string()].into_iter().collect()),
        Condition::Income { .. } => (1, 0, ["Income".to_string()].into_iter().collect()),
        Condition::HasAttribute { .. } => {
            (1, 0, ["HasAttribute".to_string()].into_iter().collect())
        }
        Condition::AttributeEquals { .. } => {
            (1, 0, ["AttributeEquals".to_string()].into_iter().collect())
        }
        Condition::DateRange { .. } => (1, 0, ["DateRange".to_string()].into_iter().collect()),
        Condition::Geographic { .. } => (1, 0, ["Geographic".to_string()].into_iter().collect()),
        Condition::EntityRelationship { .. } => (
            1,
            0,
            ["EntityRelationship".to_string()].into_iter().collect(),
        ),
        Condition::ResidencyDuration { .. } => (
            1,
            0,
            ["ResidencyDuration".to_string()].into_iter().collect(),
        ),
        Condition::Duration { .. } => (1, 0, ["Duration".to_string()].into_iter().collect()),
        Condition::Percentage { .. } => (1, 0, ["Percentage".to_string()].into_iter().collect()),
        Condition::SetMembership { .. } => {
            (1, 0, ["SetMembership".to_string()].into_iter().collect())
        }
        Condition::Pattern { .. } => (1, 0, ["Pattern".to_string()].into_iter().collect()),
        Condition::Calculation { .. } => (1, 0, ["Calculation".to_string()].into_iter().collect()),
        Condition::Composite { conditions, .. } => {
            let mut max_depth = 1;
            let mut total_ops = 0;
            let mut all_types = HashSet::new();
            all_types.insert("Composite".to_string());
            for (_weight, cond) in conditions {
                let (depth, ops, types) = analyze_condition(cond);
                max_depth = max_depth.max(depth + 1);
                total_ops += ops;
                all_types.extend(types);
            }
            (max_depth, total_ops + conditions.len() - 1, all_types)
        }
        Condition::Threshold { attributes, .. } => {
            let mut types = HashSet::new();
            types.insert("Threshold".to_string());
            (1, attributes.len().saturating_sub(1), types)
        }
        Condition::Fuzzy { .. } => (1, 0, ["Fuzzy".to_string()].into_iter().collect()),
        Condition::Probabilistic { condition, .. } => {
            let (depth, ops, mut types) = analyze_condition(condition);
            types.insert("Probabilistic".to_string());
            (1 + depth, 1 + ops, types)
        }
        Condition::Temporal { .. } => (1, 0, ["Temporal".to_string()].into_iter().collect()),
        Condition::Custom { .. } => (1, 0, ["Custom".to_string()].into_iter().collect()),
        Condition::And(left, right) => {
            let (l_depth, l_ops, l_types) = analyze_condition(left);
            let (r_depth, r_ops, r_types) = analyze_condition(right);
            let mut types = l_types;
            types.extend(r_types);
            (1 + l_depth.max(r_depth), 1 + l_ops + r_ops, types)
        }
        Condition::Or(left, right) => {
            let (l_depth, l_ops, l_types) = analyze_condition(left);
            let (r_depth, r_ops, r_types) = analyze_condition(right);
            let mut types = l_types;
            types.extend(r_types);
            (1 + l_depth.max(r_depth), 1 + l_ops + r_ops, types)
        }
        Condition::Not(inner) => {
            let (depth, ops, types) = analyze_condition(inner);
            (1 + depth, 1 + ops, types)
        }
    }
}
/// Generates a complexity report for multiple statutes.
pub fn complexity_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Complexity Report\n\n");
    let mut total_score = 0u32;
    let mut max_complexity = ComplexityLevel::Simple;
    for statute in statutes {
        let metrics = analyze_complexity(statute);
        total_score += metrics.complexity_score;
        if metrics.complexity_level as u8 > max_complexity as u8 {
            max_complexity = metrics.complexity_level;
        }
        report.push_str(&format!("## {}: \"{}\"\n", statute.id, statute.title));
        report.push_str(&format!(
            "- Complexity Level: {}\n",
            metrics.complexity_level
        ));
        report.push_str(&format!(
            "- Complexity Score: {}/100\n",
            metrics.complexity_score
        ));
        report.push_str(&format!("- Conditions: {}\n", metrics.condition_count));
        report.push_str(&format!("- Max Depth: {}\n", metrics.condition_depth));
        report.push_str(&format!(
            "- Logical Operators: {}\n",
            metrics.logical_operator_count
        ));
        report.push_str(&format!(
            "- Condition Types: {}\n",
            metrics.condition_type_count
        ));
        report.push_str(&format!("- Has Discretion: {}\n", metrics.has_discretion));
        report.push_str(&format!(
            "- Cyclomatic Complexity: {}\n\n",
            metrics.cyclomatic_complexity
        ));
    }
    let avg_score = if statutes.is_empty() {
        0
    } else {
        total_score / statutes.len() as u32
    };
    report.push_str("## Summary\n");
    report.push_str(&format!("- Total Statutes: {}\n", statutes.len()));
    report.push_str(&format!("- Average Complexity Score: {}\n", avg_score));
    report.push_str(&format!("- Maximum Complexity Level: {}\n", max_complexity));
    report
}
/// Analyzes code coverage for conditions in statutes.
///
/// This function determines which conditions have been evaluated during verification
/// and which paths through the condition logic have been taken.
pub fn analyze_coverage(statutes: &[Statute]) -> CoverageInfo {
    let mut coverage = CoverageInfo::new();
    #[cfg(feature = "smt-solver")]
    {
        use crate::smt;
        let mut smt_verifier = smt::SmtVerifier::new();
        for statute in statutes {
            let statute_id = statute.id.clone();
            let mut covered_indices = Vec::new();
            let mut uncovered_indices = Vec::new();
            for (idx, condition) in statute.preconditions.iter().enumerate() {
                coverage.total_conditions += 1;
                match smt_verifier.is_satisfiable(condition) {
                    Ok(true) => {
                        coverage.satisfiable_conditions += 1;
                        covered_indices.push(idx);
                    }
                    Ok(false) => {
                        coverage.unsatisfiable_conditions += 1;
                        uncovered_indices.push(idx);
                    }
                    Err(_) => {
                        uncovered_indices.push(idx);
                    }
                }
            }
            if !covered_indices.is_empty() {
                coverage
                    .covered_conditions
                    .insert(statute_id.clone(), covered_indices);
            }
            if !uncovered_indices.is_empty() {
                coverage
                    .uncovered_conditions
                    .insert(statute_id, uncovered_indices);
            }
        }
    }
    #[cfg(not(feature = "smt-solver"))]
    {
        for statute in statutes {
            let statute_id = statute.id.clone();
            let indices: Vec<usize> = (0..statute.preconditions.len()).collect();
            coverage.total_conditions += statute.preconditions.len();
            coverage.satisfiable_conditions += statute.preconditions.len();
            if !indices.is_empty() {
                coverage.covered_conditions.insert(statute_id, indices);
            }
        }
    }
    coverage.compute_percentage();
    coverage
}
/// Analyzes code coverage for conditions in statutes in parallel (requires 'parallel' feature).
///
/// This function determines which conditions have been evaluated during verification
/// and which paths through the condition logic have been taken. Uses parallel processing
/// to speed up analysis for large statute sets.
#[cfg(feature = "parallel")]
pub fn analyze_coverage_parallel(statutes: &[Statute]) -> CoverageInfo {
    use rayon::prelude::*;
    use std::sync::Mutex;
    let coverage = Mutex::new(CoverageInfo::new());
    #[cfg(feature = "smt-solver")]
    {
        use crate::smt;
        statutes.par_iter().for_each(|statute| {
            let mut smt_verifier = smt::SmtVerifier::new();
            let statute_id = statute.id.clone();
            let mut covered_indices = Vec::new();
            let mut uncovered_indices = Vec::new();
            let mut total = 0;
            let mut satisfiable = 0;
            let mut unsatisfiable = 0;
            for (idx, condition) in statute.preconditions.iter().enumerate() {
                total += 1;
                match smt_verifier.is_satisfiable(condition) {
                    Ok(true) => {
                        satisfiable += 1;
                        covered_indices.push(idx);
                    }
                    Ok(false) => {
                        unsatisfiable += 1;
                        uncovered_indices.push(idx);
                    }
                    Err(_) => {
                        uncovered_indices.push(idx);
                    }
                }
            }
            if let Ok(mut cov) = coverage.lock() {
                cov.total_conditions += total;
                cov.satisfiable_conditions += satisfiable;
                cov.unsatisfiable_conditions += unsatisfiable;
                if !covered_indices.is_empty() {
                    cov.covered_conditions
                        .insert(statute_id.clone(), covered_indices);
                }
                if !uncovered_indices.is_empty() {
                    cov.uncovered_conditions
                        .insert(statute_id, uncovered_indices);
                }
            }
        });
    }
    #[cfg(not(feature = "smt-solver"))]
    {
        statutes.par_iter().for_each(|statute| {
            let statute_id = statute.id.clone();
            let indices: Vec<usize> = (0..statute.preconditions.len()).collect();
            let total = statute.preconditions.len();
            if let Ok(mut cov) = coverage.lock() {
                cov.total_conditions += total;
                cov.satisfiable_conditions += total;
                if !indices.is_empty() {
                    cov.covered_conditions.insert(statute_id, indices);
                }
            }
        });
    }
    let mut final_coverage = coverage.into_inner().expect("coverage mutex poisoned");
    final_coverage.compute_percentage();
    final_coverage
}
/// Generates an HTML report for verification results.
pub fn generate_html_report(result: &VerificationResult, title: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("  <title>{}</title>\n", title));
    html.push_str("  <style>\n");
    html.push_str(
        "    body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n",
    );
    html.push_str(
        "    .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n",
    );
    html.push_str(
        "    h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }\n",
    );
    html.push_str(
        "    .status { padding: 10px; margin: 20px 0; border-radius: 4px; font-weight: bold; }\n",
    );
    html.push_str(
        "    .status.pass { background: #d4edda; color: #155724; border: 1px solid #c3e6cb; }\n",
    );
    html.push_str(
        "    .status.fail { background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; }\n",
    );
    html.push_str("    .section { margin: 20px 0; }\n");
    html.push_str("    .section h2 { color: #555; margin-bottom: 10px; }\n");
    html.push_str(
        "    .error { background: #f8d7da; padding: 10px; margin: 5px 0; border-left: 4px solid #dc3545; border-radius: 4px; }\n",
    );
    html.push_str(
        "    .warning { background: #fff3cd; padding: 10px; margin: 5px 0; border-left: 4px solid #ffc107; border-radius: 4px; }\n",
    );
    html.push_str(
        "    .suggestion { background: #d1ecf1; padding: 10px; margin: 5px 0; border-left: 4px solid #17a2b8; border-radius: 4px; }\n",
    );
    html.push_str("    .empty { color: #999; font-style: italic; }\n");
    html.push_str("    .timestamp { color: #666; font-size: 0.9em; margin-top: 20px; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("  <div class=\"container\">\n");
    html.push_str(&format!("    <h1>{}</h1>\n", title));
    let status_class = if result.passed { "pass" } else { "fail" };
    let status_text = if result.passed {
        "✓ Verification Passed"
    } else {
        "✗ Verification Failed"
    };
    html.push_str(&format!(
        "    <div class=\"status {}\">{}</div>\n",
        status_class, status_text
    ));
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Errors</h2>\n");
    if result.errors.is_empty() {
        html.push_str("      <p class=\"empty\">No errors found</p>\n");
    } else {
        for error in &result.errors {
            html.push_str(&format!(
                "      <div class=\"error\">{}</div>\n",
                html_escape(&error.to_string())
            ));
        }
    }
    html.push_str("    </div>\n");
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Warnings</h2>\n");
    if result.warnings.is_empty() {
        html.push_str("      <p class=\"empty\">No warnings found</p>\n");
    } else {
        for warning in &result.warnings {
            html.push_str(&format!(
                "      <div class=\"warning\">{}</div>\n",
                html_escape(warning)
            ));
        }
    }
    html.push_str("    </div>\n");
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Suggestions</h2>\n");
    if result.suggestions.is_empty() {
        html.push_str("      <p class=\"empty\">No suggestions</p>\n");
    } else {
        for suggestion in &result.suggestions {
            html.push_str(&format!(
                "      <div class=\"suggestion\">{}</div>\n",
                html_escape(suggestion)
            ));
        }
    }
    html.push_str("    </div>\n");
    html.push_str("    <div class=\"timestamp\">\n");
    html.push_str(&format!(
        "      Generated: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    html.push_str("    </div>\n");
    html.push_str("  </div>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");
    html
}
/// Simple HTML escaping function.
pub(crate) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
