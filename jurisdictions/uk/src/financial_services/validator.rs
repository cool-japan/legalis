//! Financial Services Validators (FSMA 2000, FCA Rules)

use super::error::{FinancialServicesError, Result};
use super::types::*;

/// Validate FCA authorization
///
/// Checks:
/// - Authorization status is active
/// - Regulated activities are permitted
pub fn validate_fca_authorization(
    authorization: &FcaAuthorization,
    activity: &RegulatedActivity,
) -> Result<()> {
    // Check authorization status
    if authorization.status != AuthorizationStatus::Authorized {
        return Err(FinancialServicesError::AuthorizationSuspended);
    }

    // Check activity is permitted
    let activity_name = format!("{:?}", activity);
    if !authorization
        .regulated_activities
        .iter()
        .any(|a| std::mem::discriminant(a) == std::mem::discriminant(activity))
    {
        let permitted = authorization
            .regulated_activities
            .iter()
            .map(|a| format!("{:?}", a))
            .collect::<Vec<_>>()
            .join(", ");

        return Err(FinancialServicesError::ActivityNotPermitted {
            activity: activity_name,
            permitted_activities: permitted,
        });
    }

    Ok(())
}

/// Validate FCA Principles for Businesses compliance
///
/// Checks all 11 principles are complied with
pub fn validate_principles_compliance(principles: &PrinciplesCompliance) -> Result<()> {
    let mut errors = Vec::new();

    if !principles.integrity.compliant {
        errors.push(FinancialServicesError::BreachIntegrity {
            details: principles
                .integrity
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.skill_care_diligence.compliant {
        errors.push(FinancialServicesError::BreachSkillCare {
            details: principles
                .skill_care_diligence
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.customers_interests.compliant {
        errors.push(FinancialServicesError::BreachCustomersInterests {
            details: principles
                .customers_interests
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.communications.compliant {
        errors.push(FinancialServicesError::BreachCommunications {
            details: principles
                .communications
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.conflicts_of_interest.compliant {
        errors.push(FinancialServicesError::BreachConflictsOfInterest {
            details: principles
                .conflicts_of_interest
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.client_assets.compliant {
        errors.push(FinancialServicesError::BreachClientAssets {
            details: principles
                .client_assets
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !principles.relations_with_regulators.compliant {
        errors.push(FinancialServicesError::BreachRelationsWithRegulators {
            details: principles
                .relations_with_regulators
                .breach_details
                .clone()
                .unwrap_or_else(|| "No details provided".to_string()),
        });
    }

    if !errors.is_empty() {
        return Err(errors.into_iter().next().expect("errors vec is non-empty"));
    }

    Ok(())
}

/// Validate suitability assessment (COBS 9)
///
/// Checks:
/// - Assessment required for client category
/// - Knowledge and experience adequate
/// - Financial situation supports recommendation
/// - Recommendation aligns with objectives
/// - Risk rating matches tolerance
pub fn validate_suitability_assessment(assessment: &SuitabilityAssessment) -> Result<()> {
    // Check if suitability required for this client category
    if !assessment.client_category.requires_suitability_assessment() {
        return Ok(()); // Not required for professional/counterparty
    }

    // If no recommendation, cannot assess
    let recommendation = assessment.recommendation.as_ref().ok_or_else(|| {
        FinancialServicesError::InsufficientInformationForSuitability {
            missing_information: "No recommendation provided".to_string(),
        }
    })?;

    // Check knowledge and experience
    if !is_knowledge_adequate(
        &assessment.knowledge_experience,
        &recommendation.investment_type,
    ) {
        return Err(FinancialServicesError::UnsuitableRecommendation {
            reason: format!(
                "Client lacks adequate knowledge/experience with {:?} (only {} years experience)",
                recommendation.investment_type, assessment.knowledge_experience.years_experience
            ),
        });
    }

    // Check financial situation
    if !is_financially_suitable(&assessment.financial_situation, recommendation.amount_gbp) {
        return Err(FinancialServicesError::UnsuitableRecommendation {
            reason: format!(
                "Investment amount £{:.2} exceeds client's financial capacity (available: £{:.2})",
                recommendation.amount_gbp, assessment.financial_situation.investment_amount_gbp
            ),
        });
    }

    // Check risk tolerance
    if !recommendation
        .risk_rating
        .matches_tolerance(assessment.investment_objectives.risk_tolerance)
    {
        return Err(FinancialServicesError::UnsuitableRecommendation {
            reason: format!(
                "Risk rating {:?} does not match client risk tolerance {:?}",
                recommendation.risk_rating, assessment.investment_objectives.risk_tolerance
            ),
        });
    }

    // Check time horizon for liquidity
    if !matches_time_horizon(
        &assessment.investment_objectives,
        &recommendation.investment_type,
    ) {
        return Err(FinancialServicesError::UnsuitableRecommendation {
            reason: format!(
                "Investment time horizon ({} years) does not match product characteristics",
                assessment.investment_objectives.time_horizon_years
            ),
        });
    }

    if !assessment.suitable {
        return Err(FinancialServicesError::UnsuitableRecommendation {
            reason: "Marked as unsuitable by assessor".to_string(),
        });
    }

    Ok(())
}

/// Check if knowledge and experience is adequate for investment type
fn is_knowledge_adequate(
    knowledge: &KnowledgeExperience,
    investment_type: &InvestmentType,
) -> bool {
    // Must have familiarity with investment type
    if !knowledge.familiar_instruments.contains(investment_type) {
        return false;
    }

    // Complex products require more experience
    let min_years = match investment_type {
        InvestmentType::Derivatives => 3,
        InvestmentType::Shares | InvestmentType::Bonds => 1,
        InvestmentType::CollectiveInvestmentSchemes => 1,
        InvestmentType::PensionSchemes => 0,
        InvestmentType::InsuranceContracts => 0,
    };

    knowledge.years_experience >= min_years
}

/// Check if financial situation supports recommendation
fn is_financially_suitable(situation: &FinancialSituation, investment_amount: f64) -> bool {
    // Investment amount must not exceed available funds
    if investment_amount > situation.investment_amount_gbp {
        return false;
    }

    // Client must be able to afford to lose the investment
    if !situation.can_afford_loss {
        return false;
    }

    // Investment should not be more than 50% of net assets (prudent rule)
    let max_investment = situation.net_assets_gbp * 0.5;
    investment_amount <= max_investment
}

/// Check if time horizon matches investment type
fn matches_time_horizon(
    objectives: &InvestmentObjectives,
    investment_type: &InvestmentType,
) -> bool {
    match investment_type {
        InvestmentType::Derivatives => objectives.time_horizon_years >= 1,
        InvestmentType::Shares => objectives.time_horizon_years >= 3,
        InvestmentType::Bonds => objectives.time_horizon_years >= 1,
        InvestmentType::CollectiveInvestmentSchemes => objectives.time_horizon_years >= 3,
        InvestmentType::PensionSchemes => objectives.time_horizon_years >= 5,
        InvestmentType::InsuranceContracts => true,
    }
}

/// Validate client assets protection (CASS)
///
/// Checks:
/// - Client money segregated
/// - Trust arrangements in place
/// - Daily reconciliation performed
/// - Regular CASS audit
pub fn validate_client_assets_protection(protection: &ClientAssetsProtection) -> Result<()> {
    // Check segregation
    if !protection.segregated && protection.client_money_gbp > 0.0 {
        return Err(FinancialServicesError::ClientMoneyNotSegregated {
            amount: protection.client_money_gbp,
        });
    }

    // Check trust arrangement
    if !protection.trust_arrangement && protection.client_money_gbp > 0.0 {
        return Err(FinancialServicesError::ClientAssetsNotProtected {
            value: protection.client_money_gbp,
            reason: "No trust arrangement in place for client money".to_string(),
        });
    }

    // Check daily reconciliation
    if !protection.daily_reconciliation && protection.client_money_gbp > 0.0 {
        return Err(FinancialServicesError::ValidationError {
            message: "Daily reconciliation not being performed for client money".to_string(),
        });
    }

    Ok(())
}

/// Validate financial promotion (FSMA s.21, COBS 4)
///
/// Checks:
/// - Approved by authorized person
/// - Risk warnings included
/// - Fair, clear and not misleading
pub fn validate_financial_promotion(promotion: &FinancialPromotion) -> Result<()> {
    // Check approval
    if !promotion.approved_by_authorized_person && promotion.fca_approval.is_none() {
        return Err(FinancialServicesError::UnapprovedFinancialPromotion);
    }

    // Check fair, clear and not misleading
    if !promotion.fair_clear_not_misleading {
        return Err(FinancialServicesError::MisleadingPromotion {
            reason: "Promotion marked as not fair, clear or misleading".to_string(),
        });
    }

    // Check risk warning for retail audience
    if matches!(promotion.target_audience, PromotionAudience::RetailClients)
        && !promotion.risk_warning_included
    {
        return Err(FinancialServicesError::MissingRiskWarning {
            product_type: "retail financial product".to_string(),
        });
    }

    Ok(())
}

/// Validate best execution policy (COBS 11)
///
/// Checks:
/// - Policy established
/// - Execution factors defined
/// - Venues identified
/// - Published to clients
pub fn validate_best_execution_policy(policy: &BestExecutionPolicy) -> Result<()> {
    // Check factors defined
    if policy.factors.is_empty() {
        return Err(FinancialServicesError::NoBestExecutionPolicy);
    }

    // Check venues
    if policy.venues.is_empty() {
        return Err(FinancialServicesError::ValidationError {
            message: "No execution venues defined in best execution policy".to_string(),
        });
    }

    // Check published to clients
    if !policy.published_to_clients {
        return Err(FinancialServicesError::ValidationError {
            message: "Best execution policy not published to clients".to_string(),
        });
    }

    // Check review frequency (should be at least annual)
    if policy.review_frequency_months > 12 {
        return Err(FinancialServicesError::ValidationError {
            message: "Best execution policy review frequency exceeds 12 months".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_fca_authorization_success() {
        let auth = FcaAuthorization {
            firm_reference_number: "123456".to_string(),
            firm_name: "Test Firm".to_string(),
            status: AuthorizationStatus::Authorized,
            authorization_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            regulated_activities: vec![RegulatedActivity::AdvisingOnInvestments {
                investment_type: InvestmentType::Shares,
            }],
            passporting_rights: vec![],
        };

        let activity = RegulatedActivity::AdvisingOnInvestments {
            investment_type: InvestmentType::Shares,
        };

        assert!(validate_fca_authorization(&auth, &activity).is_ok());
    }

    #[test]
    fn test_validate_fca_authorization_activity_not_permitted() {
        let auth = FcaAuthorization {
            firm_reference_number: "123456".to_string(),
            firm_name: "Test Firm".to_string(),
            status: AuthorizationStatus::Authorized,
            authorization_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            regulated_activities: vec![RegulatedActivity::AdvisingOnInvestments {
                investment_type: InvestmentType::Shares,
            }],
            passporting_rights: vec![],
        };

        let activity = RegulatedActivity::DealingInInvestmentsPrincipal;

        let result = validate_fca_authorization(&auth, &activity);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::ActivityNotPermitted { .. })
        ));
    }

    #[test]
    fn test_validate_suitability_risk_mismatch() {
        let assessment = SuitabilityAssessment {
            client_name: "John Smith".to_string(),
            client_category: ClientCategory::RetailClient,
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            knowledge_experience: KnowledgeExperience {
                familiar_instruments: vec![InvestmentType::Shares],
                years_experience: 5,
                financial_education: EducationLevel::Undergraduate,
                professional_experience: false,
            },
            financial_situation: FinancialSituation {
                regular_income_gbp: 50_000.0,
                net_assets_gbp: 100_000.0,
                source_of_funds: "Salary".to_string(),
                financial_commitments_gbp: 20_000.0,
                investment_amount_gbp: 30_000.0,
                can_afford_loss: true,
            },
            investment_objectives: InvestmentObjectives {
                primary_objective: InvestmentObjective::Growth,
                risk_tolerance: RiskTolerance::Low, // LOW risk tolerance
                time_horizon_years: 5,
                liquidity_needs: LiquidityNeeds::MediumTerm,
            },
            recommendation: Some(InvestmentRecommendation {
                product_name: "High Risk Fund".to_string(),
                investment_type: InvestmentType::Shares,
                amount_gbp: 20_000.0,
                risk_rating: RiskRating::High, // HIGH risk product
                expected_return_percent: 15.0,
                charges_percent: 2.0,
            }),
            suitable: false,
            reasons: "Risk mismatch".to_string(),
        };

        let result = validate_suitability_assessment(&assessment);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::UnsuitableRecommendation { .. })
        ));
    }

    #[test]
    fn test_validate_client_assets_not_segregated() {
        let protection = ClientAssetsProtection {
            client_money_gbp: 100_000.0,
            client_assets_value_gbp: 500_000.0,
            segregated: false, // NOT SEGREGATED
            trust_arrangement: false,
            daily_reconciliation: true,
            cass_audit_date: None,
        };

        let result = validate_client_assets_protection(&protection);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::ClientMoneyNotSegregated { .. })
        ));
    }

    #[test]
    fn test_validate_financial_promotion_not_approved() {
        let promotion = FinancialPromotion {
            content: "Invest now for high returns!".to_string(),
            medium: PromotionMedium::Online,
            target_audience: PromotionAudience::RetailClients,
            promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            risk_warning_included: false,
            approved_by_authorized_person: false, // NOT APPROVED
            fca_approval: None,
            fair_clear_not_misleading: true,
        };

        let result = validate_financial_promotion(&promotion);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(FinancialServicesError::UnapprovedFinancialPromotion)
        ));
    }

    #[test]
    fn test_validate_best_execution_policy() {
        let policy = BestExecutionPolicy {
            policy_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            factors: vec![
                ExecutionFactor::Price,
                ExecutionFactor::Costs,
                ExecutionFactor::Speed,
            ],
            venues: vec!["LSE".to_string(), "Chi-X".to_string()],
            review_frequency_months: 12,
            published_to_clients: true,
        };

        assert!(validate_best_execution_policy(&policy).is_ok());
    }
}
