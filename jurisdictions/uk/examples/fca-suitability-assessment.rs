//! FCA Suitability Assessment Examples (COBS 9)
//!
//! Demonstrates suitability assessments for retail clients under FCA rules.

use chrono::NaiveDate;
use legalis_uk::financial_services::*;

fn main() {
    println!("=== UK Financial Services: FCA Suitability Assessment (COBS 9) ===\n");

    example_1_suitable_recommendation();
    example_2_risk_mismatch();
    example_3_insufficient_knowledge();
    example_4_excessive_investment();
    example_5_professional_client_no_assessment();
}

/// Example 1: Suitable recommendation for retail client
fn example_1_suitable_recommendation() {
    println!("Example 1: Suitable Recommendation for Retail Client");
    println!("─────────────────────────────────────────────────────");

    let assessment = SuitabilityAssessment {
        client_name: "Sarah Johnson".to_string(),
        client_category: ClientCategory::RetailClient,
        assessment_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        knowledge_experience: KnowledgeExperience {
            familiar_instruments: vec![
                InvestmentType::Shares,
                InvestmentType::Bonds,
                InvestmentType::CollectiveInvestmentSchemes,
            ],
            years_experience: 5,
            financial_education: EducationLevel::Undergraduate,
            professional_experience: false,
        },
        financial_situation: FinancialSituation {
            regular_income_gbp: 60_000.0,
            net_assets_gbp: 150_000.0,
            source_of_funds: "Salary and savings".to_string(),
            financial_commitments_gbp: 30_000.0,
            investment_amount_gbp: 50_000.0,
            can_afford_loss: true,
        },
        investment_objectives: InvestmentObjectives {
            primary_objective: InvestmentObjective::Growth,
            risk_tolerance: RiskTolerance::Medium,
            time_horizon_years: 10,
            liquidity_needs: LiquidityNeeds::LongTerm,
        },
        recommendation: Some(InvestmentRecommendation {
            product_name: "Balanced Growth Fund".to_string(),
            investment_type: InvestmentType::CollectiveInvestmentSchemes,
            amount_gbp: 30_000.0,
            risk_rating: RiskRating::Medium,
            expected_return_percent: 6.5,
            charges_percent: 1.2,
        }),
        suitable: true,
        reasons: "Recommendation aligns with client's medium risk tolerance, 10-year time horizon, and growth objectives. Client has adequate knowledge of collective investment schemes and sufficient financial resources. Investment amount (£30k) is 20% of net assets, within prudent limits.".to_string(),
    };

    match validate_suitability_assessment(&assessment) {
        Ok(_) => {
            println!("✓ Suitability assessment PASSED");
            println!("  Client: {}", assessment.client_name);
            println!("  Category: {:?}", assessment.client_category);
            println!(
                "  Recommendation: {}",
                assessment.recommendation.as_ref().unwrap().product_name
            );
            println!(
                "  Amount: £{:.2}",
                assessment.recommendation.as_ref().unwrap().amount_gbp
            );
            println!(
                "  Risk Rating: {:?}",
                assessment.recommendation.as_ref().unwrap().risk_rating
            );
            println!(
                "  Client Risk Tolerance: {:?}",
                assessment.investment_objectives.risk_tolerance
            );
            println!("\n  Suitability Factors Assessed:");
            println!(
                "  • Knowledge & Experience: {} years, familiar with {:?}",
                assessment.knowledge_experience.years_experience,
                assessment.knowledge_experience.familiar_instruments
            );
            println!(
                "  • Financial Situation: £{:.2} available (£{:.2} net assets)",
                assessment.financial_situation.investment_amount_gbp,
                assessment.financial_situation.net_assets_gbp
            );
            println!(
                "  • Investment Objectives: {:?}, {} year horizon",
                assessment.investment_objectives.primary_objective,
                assessment.investment_objectives.time_horizon_years
            );
            println!("\n  Reference: COBS 9 (Suitability)");
        }
        Err(e) => println!("✗ Unsuitable: {}", e),
    }
    println!();
}

/// Example 2: Risk mismatch - high risk product for low risk tolerance
fn example_2_risk_mismatch() {
    println!("Example 2: Risk Mismatch - Unsuitable Recommendation");
    println!("──────────────────────────────────────────────────────");

    let assessment = SuitabilityAssessment {
        client_name: "Robert Chen".to_string(),
        client_category: ClientCategory::RetailClient,
        assessment_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        knowledge_experience: KnowledgeExperience {
            familiar_instruments: vec![InvestmentType::Shares],
            years_experience: 2,
            financial_education: EducationLevel::Secondary,
            professional_experience: false,
        },
        financial_situation: FinancialSituation {
            regular_income_gbp: 35_000.0,
            net_assets_gbp: 50_000.0,
            source_of_funds: "Salary and inheritance".to_string(),
            financial_commitments_gbp: 15_000.0,
            investment_amount_gbp: 20_000.0,
            can_afford_loss: true,
        },
        investment_objectives: InvestmentObjectives {
            primary_objective: InvestmentObjective::CapitalPreservation,
            risk_tolerance: RiskTolerance::VeryLow, // VERY LOW risk tolerance
            time_horizon_years: 5,
            liquidity_needs: LiquidityNeeds::MediumTerm,
        },
        recommendation: Some(InvestmentRecommendation {
            product_name: "Emerging Markets High Growth Fund".to_string(),
            investment_type: InvestmentType::Shares,
            amount_gbp: 15_000.0,
            risk_rating: RiskRating::High, // HIGH risk product
            expected_return_percent: 12.0,
            charges_percent: 2.5,
        }),
        suitable: false,
        reasons: "Risk mismatch".to_string(),
    };

    match validate_suitability_assessment(&assessment) {
        Ok(_) => println!("✓ Suitable"),
        Err(e) => {
            println!("✗ UNSUITABLE: {}", e);
            println!("\n  Problem: RISK MISMATCH");
            println!("  Client Risk Tolerance: Very Low (capital preservation objective)");
            println!("  Product Risk Rating: High");
            println!("\n  COBS 9.2.1R: Firm must obtain necessary information regarding:");
            println!("  (a) Knowledge and experience");
            println!("  (b) Financial situation");
            println!("  (c) Investment objectives");
            println!("\n  COBS 9.2.2R: Firm must not recommend if investment is NOT SUITABLE");
            println!("\n  This recommendation BREACHES suitability requirements.");
            println!(
                "  Adviser must recommend lower-risk products (e.g., bonds, money market funds)"
            );
            println!("  that align with capital preservation objective.");
            println!("\n  Reference: COBS 9 (Suitability)");
        }
    }
    println!();
}

/// Example 3: Insufficient knowledge for derivatives
fn example_3_insufficient_knowledge() {
    println!("Example 3: Insufficient Knowledge for Complex Product");
    println!("───────────────────────────────────────────────────────");

    let assessment = SuitabilityAssessment {
        client_name: "Emma Thompson".to_string(),
        client_category: ClientCategory::RetailClient,
        assessment_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        knowledge_experience: KnowledgeExperience {
            familiar_instruments: vec![InvestmentType::Shares], // Not familiar with derivatives
            years_experience: 1,                                // Only 1 year experience
            financial_education: EducationLevel::None,
            professional_experience: false,
        },
        financial_situation: FinancialSituation {
            regular_income_gbp: 80_000.0,
            net_assets_gbp: 200_000.0,
            source_of_funds: "Salary".to_string(),
            financial_commitments_gbp: 40_000.0,
            investment_amount_gbp: 100_000.0,
            can_afford_loss: true,
        },
        investment_objectives: InvestmentObjectives {
            primary_objective: InvestmentObjective::Speculation,
            risk_tolerance: RiskTolerance::VeryHigh,
            time_horizon_years: 2,
            liquidity_needs: LiquidityNeeds::ShortTerm,
        },
        recommendation: Some(InvestmentRecommendation {
            product_name: "Currency Options Strategy".to_string(),
            investment_type: InvestmentType::Derivatives, // DERIVATIVES - complex product
            amount_gbp: 50_000.0,
            risk_rating: RiskRating::High,
            expected_return_percent: 20.0,
            charges_percent: 3.0,
        }),
        suitable: false,
        reasons: "Insufficient knowledge".to_string(),
    };

    match validate_suitability_assessment(&assessment) {
        Ok(_) => println!("✓ Suitable"),
        Err(e) => {
            println!("✗ UNSUITABLE: {}", e);
            println!("\n  Problem: INSUFFICIENT KNOWLEDGE & EXPERIENCE");
            println!("  Product Type: Derivatives (complex product)");
            println!("  Client Experience: 1 year (minimum 3 years required for derivatives)");
            println!("  Client Familiarity: Shares only (not familiar with derivatives)");
            println!("\n  Complex products require:");
            println!("  • Understanding of leverage");
            println!("  • Knowledge of margin requirements");
            println!("  • Awareness of potential for unlimited losses");
            println!("  • Experience with derivatives markets");
            println!("\n  Even though client has:");
            println!("  • High risk tolerance ✓");
            println!("  • Sufficient financial resources ✓");
            println!("  • Speculation objective ✓");
            println!("\n  Recommendation is STILL UNSUITABLE due to insufficient knowledge.");
            println!("\n  Adviser must either:");
            println!("  1. Provide education and reassess after client gains experience, or");
            println!("  2. Recommend simpler products aligned with client's objectives");
            println!("\n  Reference: COBS 9.2 (Assessing suitability), COBS 10 (Appropriateness)");
        }
    }
    println!();
}

/// Example 4: Excessive investment amount relative to net assets
fn example_4_excessive_investment() {
    println!("Example 4: Excessive Investment Amount");
    println!("───────────────────────────────────────");

    let assessment = SuitabilityAssessment {
        client_name: "David Wilson".to_string(),
        client_category: ClientCategory::RetailClient,
        assessment_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        knowledge_experience: KnowledgeExperience {
            familiar_instruments: vec![InvestmentType::Shares, InvestmentType::Bonds],
            years_experience: 10,
            financial_education: EducationLevel::Graduate,
            professional_experience: false,
        },
        financial_situation: FinancialSituation {
            regular_income_gbp: 40_000.0,
            net_assets_gbp: 60_000.0, // £60k net assets
            source_of_funds: "Inheritance".to_string(),
            financial_commitments_gbp: 10_000.0,
            investment_amount_gbp: 50_000.0, // Claims £50k available
            can_afford_loss: false,          // CANNOT afford to lose
        },
        investment_objectives: InvestmentObjectives {
            primary_objective: InvestmentObjective::Growth,
            risk_tolerance: RiskTolerance::Medium,
            time_horizon_years: 7,
            liquidity_needs: LiquidityNeeds::MediumTerm,
        },
        recommendation: Some(InvestmentRecommendation {
            product_name: "Global Equity Fund".to_string(),
            investment_type: InvestmentType::Shares,
            amount_gbp: 45_000.0, // 75% of net assets!
            risk_rating: RiskRating::Medium,
            expected_return_percent: 7.0,
            charges_percent: 1.5,
        }),
        suitable: false,
        reasons: "Excessive concentration".to_string(),
    };

    match validate_suitability_assessment(&assessment) {
        Ok(_) => println!("✓ Suitable"),
        Err(e) => {
            println!("✗ UNSUITABLE: {}", e);
            println!("\n  Problem: EXCESSIVE INVESTMENT AMOUNT");
            println!(
                "  Net Assets: £{:.2}",
                assessment.financial_situation.net_assets_gbp
            );
            println!(
                "  Recommended Investment: £{:.2}",
                assessment.recommendation.as_ref().unwrap().amount_gbp
            );
            println!(
                "  Percentage: {:.1}%",
                (assessment.recommendation.as_ref().unwrap().amount_gbp
                    / assessment.financial_situation.net_assets_gbp)
                    * 100.0
            );
            println!(
                "  Can Afford Loss: {}",
                assessment.financial_situation.can_afford_loss
            );
            println!(
                "\n  Prudent investment limit: 50% of net assets (£{:.2})",
                assessment.financial_situation.net_assets_gbp * 0.5
            );
            println!("\n  Issues:");
            println!("  • Client CANNOT afford to lose investment");
            println!("  • 75% concentration in single fund creates excessive risk");
            println!("  • Lack of diversification across asset classes");
            println!("\n  Recommendation:");
            println!("  • Reduce investment amount to £25,000-£30,000 (40-50%)");
            println!("  • Retain £20,000+ as emergency fund");
            println!("  • Consider lower-risk allocation given inability to afford loss");
            println!("\n  Reference: COBS 9.2 (Assessing suitability) - financial situation");
        }
    }
    println!();
}

/// Example 5: Professional client - suitability assessment not required
fn example_5_professional_client_no_assessment() {
    println!("Example 5: Professional Client - No Suitability Assessment Required");
    println!("───────────────────────────────────────────────────────────────────");

    let assessment = SuitabilityAssessment {
        client_name: "Investment Management Ltd".to_string(),
        client_category: ClientCategory::ProfessionalClient { elective: false },
        assessment_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        knowledge_experience: KnowledgeExperience {
            familiar_instruments: vec![
                InvestmentType::Shares,
                InvestmentType::Bonds,
                InvestmentType::Derivatives,
            ],
            years_experience: 20,
            financial_education: EducationLevel::Professional,
            professional_experience: true,
        },
        financial_situation: FinancialSituation {
            regular_income_gbp: 5_000_000.0,
            net_assets_gbp: 50_000_000.0,
            source_of_funds: "Corporate funds".to_string(),
            financial_commitments_gbp: 1_000_000.0,
            investment_amount_gbp: 10_000_000.0,
            can_afford_loss: true,
        },
        investment_objectives: InvestmentObjectives {
            primary_objective: InvestmentObjective::Growth,
            risk_tolerance: RiskTolerance::High,
            time_horizon_years: 10,
            liquidity_needs: LiquidityNeeds::LongTerm,
        },
        recommendation: None, // No personal recommendation
        suitable: true,
        reasons: "Professional client".to_string(),
    };

    match validate_suitability_assessment(&assessment) {
        Ok(_) => {
            println!("✓ Suitability assessment NOT REQUIRED (Professional Client)");
            println!("  Client: {}", assessment.client_name);
            println!("  Category: {:?}", assessment.client_category);
            println!(
                "  Protection Level: {}",
                assessment.client_category.protection_level()
            );
            println!("\n  COBS 9 (Suitability): Applies to RETAIL CLIENTS only");
            println!("\n  Professional clients:");
            println!("  • Are presumed to have necessary knowledge and experience");
            println!("  • Can assess investment risks independently");
            println!("  • Receive reduced regulatory protections");
            println!("  • Lower conduct of business obligations apply");
            println!("\n  Requirements that DO apply:");
            println!("  • Best execution (COBS 11) ✓");
            println!("  • Client categorization (COBS 3) ✓");
            println!("  • Conflicts of interest management (PRIN 8) ✓");
            println!("\n  Requirements that DON'T apply:");
            println!("  • Suitability assessment (COBS 9) ✗");
            println!("  • Appropriateness test (COBS 10) ✗");
            println!("\n  Reference: COBS 3 (Client categorization)");
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}
