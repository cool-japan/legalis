//! MiFID II Validators (UK MiFID II, COBS, MAR)

use super::error::{Mifid2Error, Result};
use super::types::*;
use chrono::NaiveDate;

/// Validate transaction report (MiFID II Article 26, FCA SUP 17)
///
/// Checks transaction reporting compliance:
/// - Transaction reported to FCA
/// - Reported within T+1 deadline
/// - All required fields present (ISIN, LEI, quantity, price, venue)
pub fn validate_transaction_report(
    report: &TransactionReport,
    current_date: NaiveDate,
) -> Result<()> {
    // Check reported to FCA
    if !report.reported_to_fca {
        return Err(Mifid2Error::TransactionNotReported {
            transaction_id: report.report_id.clone(),
            transaction_date: report.transaction_date.to_string(),
            deadline_date: report.reporting_deadline.to_string(),
        });
    }

    // Check T+1 deadline compliance
    if current_date > report.reporting_deadline {
        return Err(Mifid2Error::TransactionNotReported {
            transaction_id: report.report_id.clone(),
            transaction_date: report.transaction_date.to_string(),
            deadline_date: report.reporting_deadline.to_string(),
        });
    }

    // Check ISIN format (12 characters: 2 letters + 10 alphanumeric)
    if report.instrument_isin.len() != 12 {
        return Err(Mifid2Error::TransactionReportIncomplete {
            report_id: report.report_id.clone(),
            missing_field: "Valid ISIN (12 characters)".to_string(),
        });
    }

    // Check LEI format (20 characters alphanumeric)
    if report.buyer_lei.len() != 20 {
        return Err(Mifid2Error::InvalidLei {
            lei: report.buyer_lei.clone(),
        });
    }

    if report.seller_lei.len() != 20 {
        return Err(Mifid2Error::InvalidLei {
            lei: report.seller_lei.clone(),
        });
    }

    if report.executing_entity_lei.len() != 20 {
        return Err(Mifid2Error::InvalidLei {
            lei: report.executing_entity_lei.clone(),
        });
    }

    // Check quantity > 0
    if report.quantity <= 0.0 {
        return Err(Mifid2Error::TransactionReportIncomplete {
            report_id: report.report_id.clone(),
            missing_field: "Valid quantity (> 0)".to_string(),
        });
    }

    // Check price > 0
    if report.price <= 0.0 {
        return Err(Mifid2Error::TransactionReportIncomplete {
            report_id: report.report_id.clone(),
            missing_field: "Valid price (> 0)".to_string(),
        });
    }

    // Check currency code (ISO 4217 - 3 characters)
    if report.currency.len() != 3 {
        return Err(Mifid2Error::TransactionReportIncomplete {
            report_id: report.report_id.clone(),
            missing_field: "Valid currency code (ISO 4217)".to_string(),
        });
    }

    // Check MIC code (4 characters)
    if report.venue_mic.len() != 4 {
        return Err(Mifid2Error::TransactionReportIncomplete {
            report_id: report.report_id.clone(),
            missing_field: "Valid MIC code (4 characters)".to_string(),
        });
    }

    Ok(())
}

/// Validate product governance (COBS 16A, MiFID II Article 16(3))
///
/// Checks product governance compliance:
/// - Product approved by product approval committee
/// - Target market defined
/// - Distributors notified
pub fn validate_product_governance(governance: &ProductGovernance) -> Result<()> {
    // Check product approval (COBS 16A.1.5R)
    if !governance.approved_by_committee {
        return Err(Mifid2Error::ProductNotApproved {
            product_name: governance.product_name.clone(),
        });
    }

    if governance.approval_date.is_none() {
        return Err(Mifid2Error::ProductNotApproved {
            product_name: governance.product_name.clone(),
        });
    }

    // Check target market defined (COBS 16A.1.4R)
    if governance.target_market.client_categories.is_empty() {
        return Err(Mifid2Error::TargetMarketNotDefined {
            product_name: governance.product_name.clone(),
        });
    }

    // Check distributors notified (COBS 16A.1.9R)
    if !governance.distribution_channels.is_empty() && !governance.distributor_notifications_sent {
        return Err(Mifid2Error::DistributorsNotNotified {
            product_name: governance.product_name.clone(),
        });
    }

    Ok(())
}

/// Validate target market suitability for client
///
/// Checks if client profile matches target market:
/// - Client category compatible
/// - Knowledge level sufficient
/// - Risk tolerance aligned
/// - Investment amount meets minimum
pub fn validate_target_market_match(
    target_market: &TargetMarket,
    client_category: ClientCategory,
    client_knowledge: KnowledgeLevel,
    client_risk_tolerance: RiskTolerance,
    investment_amount: f64,
) -> Result<()> {
    // Check client category
    if !target_market.client_categories.contains(&client_category) {
        return Err(Mifid2Error::SoldOutsideTargetMarket {
            product_name: "Product".to_string(),
            client_profile: format!("{:?}", client_category),
            target_market: format!("{:?}", target_market.client_categories),
        });
    }

    // Check knowledge level (client must have at least target level)
    let client_knowledge_score = match client_knowledge {
        KnowledgeLevel::Basic => 1,
        KnowledgeLevel::Informed => 2,
        KnowledgeLevel::Advanced => 3,
    };

    let target_knowledge_score = match target_market.knowledge_level {
        KnowledgeLevel::Basic => 1,
        KnowledgeLevel::Informed => 2,
        KnowledgeLevel::Advanced => 3,
    };

    if client_knowledge_score < target_knowledge_score {
        return Err(Mifid2Error::SoldOutsideTargetMarket {
            product_name: "Product".to_string(),
            client_profile: format!("Knowledge level: {:?}", client_knowledge),
            target_market: format!("Required knowledge: {:?}", target_market.knowledge_level),
        });
    }

    // Check risk tolerance (client tolerance must be at least target level)
    let client_risk_score = match client_risk_tolerance {
        RiskTolerance::Low => 1,
        RiskTolerance::Medium => 2,
        RiskTolerance::High => 3,
    };

    let target_risk_score = match target_market.risk_tolerance {
        RiskTolerance::Low => 1,
        RiskTolerance::Medium => 2,
        RiskTolerance::High => 3,
    };

    if client_risk_score < target_risk_score {
        return Err(Mifid2Error::SoldOutsideTargetMarket {
            product_name: "Product".to_string(),
            client_profile: format!("Risk tolerance: {:?}", client_risk_tolerance),
            target_market: format!(
                "Required risk tolerance: {:?}",
                target_market.risk_tolerance
            ),
        });
    }

    // Check minimum investment amount
    if let Some(min_amount) = target_market.min_investment_amount {
        if investment_amount < min_amount {
            return Err(Mifid2Error::SoldOutsideTargetMarket {
                product_name: "Product".to_string(),
                client_profile: format!("Investment amount: £{:.2}", investment_amount),
                target_market: format!("Minimum investment: £{:.2}", min_amount),
            });
        }
    }

    Ok(())
}

/// Validate research payment unbundling (MiFID II Article 24(8), COBS 2.3B)
///
/// Checks research unbundling compliance:
/// - Payment from research payment account (not dealing commission)
/// - Research budget approved
/// - Disclosed to clients
pub fn validate_research_payment(payment: &ResearchPayment) -> Result<()> {
    // Check payment from research account (COBS 2.3B.4R)
    if !payment.paid_from_research_account {
        return Err(Mifid2Error::ResearchPaymentBundled {
            research_provider: payment.research_provider.clone(),
            amount_gbp: payment.amount_gbp,
        });
    }

    // Check research budget approved (COBS 2.3B.5R)
    if !payment.research_budget_approved {
        return Err(Mifid2Error::ResearchBudgetNotApproved {
            research_provider: payment.research_provider.clone(),
            amount_gbp: payment.amount_gbp,
        });
    }

    // Check disclosed to clients (COBS 2.3B.9R)
    if !payment.disclosed_to_clients {
        return Err(Mifid2Error::ResearchPaymentsNotDisclosed {
            firm_name: "Firm".to_string(),
        });
    }

    Ok(())
}

/// Validate best execution report (COBS 11.2A.28R)
///
/// Checks best execution reporting compliance:
/// - Report published to clients
/// - Top 5 venues included
/// - Quality assessment performed
pub fn validate_best_execution_report(report: &BestExecutionReport) -> Result<()> {
    // Check published to clients (COBS 11.2A.28R)
    if !report.published_to_clients {
        return Err(Mifid2Error::Top5ReportNotPublished {
            period_start: report.period_start.to_string(),
            period_end: report.period_end.to_string(),
        });
    }

    // Check top 5 venues included
    if report.top_venues.is_empty() {
        return Err(Mifid2Error::ValidationError {
            message: "Best execution report must include top execution venues".to_string(),
        });
    }

    if report.top_venues.len() > 5 {
        return Err(Mifid2Error::ValidationError {
            message: "Best execution report should list top 5 venues maximum".to_string(),
        });
    }

    // Check quality assessment performed
    if report.quality_assessment.is_empty() {
        return Err(Mifid2Error::ValidationError {
            message: "Best execution report must include quality assessment".to_string(),
        });
    }

    // Check volume percentages sum to reasonable total
    let total_volume: f64 = report.top_venues.iter().map(|v| v.volume_percentage).sum();
    if total_volume > 100.0 {
        return Err(Mifid2Error::ValidationError {
            message: format!("Total volume percentage exceeds 100%: {:.2}%", total_volume),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_transaction_report_compliant() {
        let report = TransactionReport {
            report_id: "TR001".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            instrument_isin: "GB0002374006".to_string(),
            buyer_lei: "213800EBPD2GY84SVP41".to_string(),
            seller_lei: "529900T8BM49AURSDO55".to_string(),
            executing_entity_lei: "213800EBPD2GY84SVP41".to_string(),
            quantity: 1000.0,
            price: 100.0,
            currency: "GBP".to_string(),
            venue_mic: "XLON".to_string(),
            reported_to_fca: true,
            reporting_deadline: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        };

        let current = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        assert!(validate_transaction_report(&report, current).is_ok());
    }

    #[test]
    fn test_validate_transaction_report_not_reported() {
        let report = TransactionReport {
            report_id: "TR002".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            instrument_isin: "GB0002374006".to_string(),
            buyer_lei: "213800EBPD2GY84SVP41".to_string(),
            seller_lei: "529900T8BM49AURSDO55".to_string(),
            executing_entity_lei: "213800EBPD2GY84SVP41".to_string(),
            quantity: 1000.0,
            price: 100.0,
            currency: "GBP".to_string(),
            venue_mic: "XLON".to_string(),
            reported_to_fca: false, // NOT REPORTED
            reporting_deadline: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        };

        let current = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let result = validate_transaction_report(&report, current);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Mifid2Error::TransactionNotReported { .. })
        ));
    }

    #[test]
    fn test_validate_transaction_report_invalid_lei() {
        let report = TransactionReport {
            report_id: "TR003".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            instrument_isin: "GB0002374006".to_string(),
            buyer_lei: "INVALID".to_string(), // Invalid LEI (not 20 chars)
            seller_lei: "529900T8BM49AURSDO55".to_string(),
            executing_entity_lei: "213800EBPD2GY84SVP41".to_string(),
            quantity: 1000.0,
            price: 100.0,
            currency: "GBP".to_string(),
            venue_mic: "XLON".to_string(),
            reported_to_fca: true,
            reporting_deadline: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        };

        let current = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let result = validate_transaction_report(&report, current);
        assert!(result.is_err());
        assert!(matches!(result, Err(Mifid2Error::InvalidLei { .. })));
    }

    #[test]
    fn test_validate_product_governance_compliant() {
        let governance = ProductGovernance {
            product_name: "UK Equity Fund".to_string(),
            manufacturer: "Asset Manager Ltd".to_string(),
            target_market: TargetMarket {
                client_categories: vec![ClientCategory::Retail],
                knowledge_level: KnowledgeLevel::Informed,
                risk_tolerance: RiskTolerance::Medium,
                min_investment_amount: Some(1000.0),
                ability_to_bear_losses: AbilityToBearLosses::Limited,
                time_horizon: TimeHorizon::MediumTerm,
            },
            approved_by_committee: true,
            approval_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            distribution_channels: vec!["Direct".to_string()],
            distributor_notifications_sent: true,
        };

        assert!(validate_product_governance(&governance).is_ok());
    }

    #[test]
    fn test_validate_product_governance_not_approved() {
        let governance = ProductGovernance {
            product_name: "High Risk Bond".to_string(),
            manufacturer: "Asset Manager Ltd".to_string(),
            target_market: TargetMarket {
                client_categories: vec![ClientCategory::Professional],
                knowledge_level: KnowledgeLevel::Advanced,
                risk_tolerance: RiskTolerance::High,
                min_investment_amount: Some(10000.0),
                ability_to_bear_losses: AbilityToBearLosses::Full,
                time_horizon: TimeHorizon::LongTerm,
            },
            approved_by_committee: false, // NOT APPROVED
            approval_date: None,
            distribution_channels: vec![],
            distributor_notifications_sent: false,
        };

        let result = validate_product_governance(&governance);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Mifid2Error::ProductNotApproved { .. })
        ));
    }

    #[test]
    fn test_validate_target_market_match_compliant() {
        let target_market = TargetMarket {
            client_categories: vec![ClientCategory::Retail],
            knowledge_level: KnowledgeLevel::Informed,
            risk_tolerance: RiskTolerance::Medium,
            min_investment_amount: Some(1000.0),
            ability_to_bear_losses: AbilityToBearLosses::Limited,
            time_horizon: TimeHorizon::MediumTerm,
        };

        let result = validate_target_market_match(
            &target_market,
            ClientCategory::Retail,
            KnowledgeLevel::Advanced, // Higher than required
            RiskTolerance::Medium,
            5000.0, // Above minimum
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_target_market_match_insufficient_knowledge() {
        let target_market = TargetMarket {
            client_categories: vec![ClientCategory::Retail],
            knowledge_level: KnowledgeLevel::Advanced, // Requires advanced
            risk_tolerance: RiskTolerance::Medium,
            min_investment_amount: Some(1000.0),
            ability_to_bear_losses: AbilityToBearLosses::Limited,
            time_horizon: TimeHorizon::MediumTerm,
        };

        let result = validate_target_market_match(
            &target_market,
            ClientCategory::Retail,
            KnowledgeLevel::Basic, // Insufficient knowledge
            RiskTolerance::Medium,
            5000.0,
        );

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Mifid2Error::SoldOutsideTargetMarket { .. })
        ));
    }

    #[test]
    fn test_validate_research_payment_compliant() {
        let payment = ResearchPayment {
            research_provider: "Research House Ltd".to_string(),
            payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 10000.0,
            paid_from_research_account: true,
            research_budget_approved: true,
            disclosed_to_clients: true,
        };

        assert!(validate_research_payment(&payment).is_ok());
    }

    #[test]
    fn test_validate_research_payment_bundled() {
        let payment = ResearchPayment {
            research_provider: "Research House Ltd".to_string(),
            payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 10000.0,
            paid_from_research_account: false, // BUNDLED (not from research account)
            research_budget_approved: false,
            disclosed_to_clients: false,
        };

        let result = validate_research_payment(&payment);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Mifid2Error::ResearchPaymentBundled { .. })
        ));
    }

    #[test]
    fn test_validate_best_execution_report_compliant() {
        let report = BestExecutionReport {
            period_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            top_venues: vec![
                ExecutionVenue {
                    venue_name: "London Stock Exchange".to_string(),
                    mic_code: "XLON".to_string(),
                    volume_percentage: 45.0,
                },
                ExecutionVenue {
                    venue_name: "Cboe Europe".to_string(),
                    mic_code: "BATE".to_string(),
                    volume_percentage: 30.0,
                },
            ],
            quality_assessment: "Overall quality of execution was good".to_string(),
            published_to_clients: true,
        };

        assert!(validate_best_execution_report(&report).is_ok());
    }

    #[test]
    fn test_validate_best_execution_report_not_published() {
        let report = BestExecutionReport {
            period_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            top_venues: vec![ExecutionVenue {
                venue_name: "LSE".to_string(),
                mic_code: "XLON".to_string(),
                volume_percentage: 100.0,
            }],
            quality_assessment: "Quality assessment".to_string(),
            published_to_clients: false, // NOT PUBLISHED
        };

        let result = validate_best_execution_report(&report);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Mifid2Error::Top5ReportNotPublished { .. })
        ));
    }
}
