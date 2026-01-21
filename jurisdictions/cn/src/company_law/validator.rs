//! Company Law Validation
//!
//! # 公司法合规验证

#![allow(missing_docs)]

use super::error::{CompanyLawError, CompanyLawResult};
use super::types::*;
use crate::i18n::BilingualText;
use chrono::NaiveDate;

/// Company compliance report
#[derive(Debug, Clone)]
pub struct CompanyComplianceReport {
    pub compliant: bool,
    pub violations: Vec<CompanyLawError>,
    pub warnings: Vec<BilingualText>,
}

impl Default for CompanyComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

/// Validate company formation
pub fn validate_company_formation(
    registration: &CompanyRegistration,
    shareholders: &[Shareholder],
    has_legal_representative: bool,
    has_articles: bool,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Check shareholder count
    let min_shareholders = registration.company_type.min_shareholders();
    if (shareholders.len() as u32) < min_shareholders {
        report
            .violations
            .push(CompanyLawError::InsufficientShareholders {
                company_type: registration.company_type.name_zh().to_string(),
                min: min_shareholders,
                actual: shareholders.len() as u32,
            });
        report.compliant = false;
    }

    // Check max shareholders for LLC
    if let Some(max) = registration.company_type.max_shareholders()
        && shareholders.len() as u32 > max
    {
        report
            .violations
            .push(CompanyLawError::TooManyShareholders {
                actual: shareholders.len() as u32,
            });
        report.compliant = false;
    }

    // Check legal representative
    if !has_legal_representative {
        report
            .violations
            .push(CompanyLawError::MissingLegalRepresentative);
        report.compliant = false;
    }

    // Check articles of association
    if !has_articles {
        report
            .violations
            .push(CompanyLawError::ArticlesMissingContent {
                missing: "公司章程".to_string(),
            });
        report.compliant = false;
    }

    // Check non-monetary contributions have valuation
    for shareholder in shareholders {
        if shareholder.contribution_method.requires_valuation() {
            report.warnings.push(BilingualText::new(
                format!(
                    "股东{}的{}出资应当评估作价",
                    shareholder.name.zh,
                    shareholder.contribution_method.name_zh()
                ),
                format!(
                    "Shareholder {}'s {} contribution must be valued",
                    shareholder.name.en,
                    shareholder.contribution_method.name_zh()
                ),
            ));
        }
    }

    report
}

/// Validate board of directors
pub fn validate_board(
    board: &BoardOfDirectors,
    company_type: CompanyType,
    is_listed: bool,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Check board size
    if !board.is_valid_for(company_type) {
        let (min, max) = match company_type {
            CompanyType::JointStockCompany => (5, 19),
            CompanyType::LimitedLiabilityCompany => (0, 13),
            _ => (0, 100),
        };

        report
            .violations
            .push(CompanyLawError::InvalidBoardComposition {
                article: if matches!(company_type, CompanyType::JointStockCompany) {
                    109
                } else {
                    45
                },
                reason: format!(
                    "董事人数{}不在规定范围{}-{}人",
                    board.director_count, min, max
                ),
            });
        report.compliant = false;
    }

    // Check independent directors for listed companies
    if is_listed && !board.independent_directors_sufficient(is_listed) {
        report
            .violations
            .push(CompanyLawError::InsufficientIndependentDirectors);
        report.compliant = false;
    }

    // Check term (max 3 years for directors)
    if board.term_years > 3 {
        report.warnings.push(BilingualText::new(
            "董事任期超过三年，建议调整",
            "Director term exceeds 3 years, adjustment recommended",
        ));
    }

    report
}

/// Validate supervisory board
pub fn validate_supervisory_board(
    board: &SupervisoryBoard,
    company_type: CompanyType,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Check board size
    if !board.is_valid_for(company_type) {
        report
            .violations
            .push(CompanyLawError::InvalidBoardComposition {
                article: 69,
                reason: "监事会人数不足".to_string(),
            });
        report.compliant = false;
    }

    // Check employee supervisor ratio
    if board.supervisor_count >= 3 && !board.employee_ratio_sufficient() {
        report
            .violations
            .push(CompanyLawError::InsufficientEmployeeSupervisors);
        report.compliant = false;
    }

    report
}

/// Validate equity transfer (LLC)
pub fn validate_equity_transfer(
    transfer: &EquityTransfer,
    transfer_date: NaiveDate,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Internal transfers don't need consent
    if transfer.is_internal {
        return report;
    }

    // Check notification
    if !transfer.other_shareholders_notified {
        report.warnings.push(BilingualText::new(
            "应当书面通知其他股东",
            "Other shareholders must be notified in writing",
        ));
    }

    // Check 30-day notice period
    if !transfer.notice_period_satisfied(transfer_date) {
        report
            .violations
            .push(CompanyLawError::NoticePeriodNotSatisfied);
        report.compliant = false;
    }

    // Check majority consent
    if !transfer.majority_consent() {
        report
            .violations
            .push(CompanyLawError::EquityTransferNoConsent);
        report.compliant = false;
    }

    // Check preemptive rights
    if !transfer.preemptive_rights_waived {
        report
            .violations
            .push(CompanyLawError::PreemptiveRightsViolation);
        report.compliant = false;
    }

    report
}

/// Validate shareholder resolution
pub fn validate_resolution(
    matter: SpecialResolutionMatter,
    votes_for_pct: f64,
) -> CompanyLawResult<()> {
    let required = ResolutionType::Special.required_majority();

    if votes_for_pct < required {
        return Err(CompanyLawError::InvalidResolution {
            matter: matter.name_zh().to_string(),
        });
    }

    Ok(())
}

/// Validate capital contribution
pub fn validate_capital_contribution(
    shareholder: &Shareholder,
    as_of: NaiveDate,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Check if deadline passed
    if let Some(deadline) = shareholder.contribution_deadline
        && as_of > deadline
        && shareholder.paid_in_contribution.yuan() < shareholder.subscribed_contribution.yuan()
    {
        report
            .violations
            .push(CompanyLawError::CapitalContributionOverdue);
        report.compliant = false;
    }

    // Check valuation for non-monetary contributions
    if shareholder.contribution_method.requires_valuation() {
        report.warnings.push(BilingualText::new(
            "非货币出资应当评估作价",
            "Non-monetary contribution must be properly valued",
        ));
    }

    report
}

/// Validate dividend distribution
pub fn validate_dividend_distribution(
    distribution: &DividendDistribution,
    current_statutory_reserve: f64,
    registered_capital: f64,
) -> CompanyComplianceReport {
    let mut report = CompanyComplianceReport::default();

    // Check if statutory reserve requirement met
    let required_reserve = DividendDistribution::calculate_statutory_reserve(
        distribution.total_profit.yuan(),
        current_statutory_reserve,
        registered_capital,
    );

    if distribution.statutory_reserve.yuan() < required_reserve * 0.99 {
        // 1% tolerance
        report
            .violations
            .push(CompanyLawError::DividendBeforeReserve);
        report.compliant = false;
    }

    // Check total distribution doesn't exceed available profit
    let available =
        distribution.total_profit.yuan() - required_reserve - distribution.optional_reserve.yuan();
    if distribution.dividend_amount.yuan() > available * 1.01 {
        // 1% tolerance
        report
            .violations
            .push(CompanyLawError::IllegalProfitDistribution);
        report.compliant = false;
    }

    report
}

/// Check director eligibility (Article 146)
pub fn check_director_eligibility(
    has_criminal_record: bool,
    is_bankrupt: bool,
    has_debt_default: bool,
    previous_company_revoked: bool,
) -> CompanyLawResult<()> {
    if has_criminal_record {
        return Err(CompanyLawError::DirectorDisqualified {
            reason: "因贪污、贿赂、侵占财产、挪用财产或者破坏社会主义市场经济秩序被判处刑罚"
                .to_string(),
        });
    }

    if is_bankrupt {
        return Err(CompanyLawError::DirectorDisqualified {
            reason: "担任破产清算公司的董事或厂长、经理负有个人责任尚未逾三年".to_string(),
        });
    }

    if has_debt_default {
        return Err(CompanyLawError::DirectorDisqualified {
            reason: "个人所负数额较大的债务到期未清偿".to_string(),
        });
    }

    if previous_company_revoked {
        return Err(CompanyLawError::DirectorDisqualified {
            reason: "担任被吊销营业执照公司的法定代表人负有个人责任尚未逾三年".to_string(),
        });
    }

    Ok(())
}

/// Check if corporate veil can be pierced
pub fn check_veil_piercing_risk(
    assets_commingled: bool,
    inadequate_capitalization: bool,
    company_used_for_fraud: bool,
    no_separate_existence: bool,
) -> Vec<BilingualText> {
    let mut risks = Vec::new();

    if assets_commingled {
        risks.push(BilingualText::new(
            "股东财产与公司财产混同",
            "Shareholder and company assets commingled",
        ));
    }

    if inadequate_capitalization {
        risks.push(BilingualText::new(
            "公司资本明显不足",
            "Inadequate capitalization",
        ));
    }

    if company_used_for_fraud {
        risks.push(BilingualText::new(
            "利用公司法人人格进行欺诈",
            "Company used to perpetrate fraud",
        ));
    }

    if no_separate_existence {
        risks.push(BilingualText::new(
            "公司与股东人格不分",
            "No separate existence between company and shareholder",
        ));
    }

    risks
}

/// Get required procedures for company dissolution
pub fn dissolution_procedures(reason: DissolutionReason) -> Vec<BilingualText> {
    // Common procedures
    let mut procedures = vec![
        BilingualText::new(
            "成立清算组，清算组对公司财产进行清算",
            "Form liquidation committee to liquidate company assets",
        ),
        BilingualText::new(
            "通知债权人并公告",
            "Notify creditors and make public announcement",
        ),
        BilingualText::new("清偿债务", "Settle debts"),
        BilingualText::new("编制清算报告", "Prepare liquidation report"),
        BilingualText::new(
            "向公司登记机关申请注销登记",
            "Apply for deregistration with company registry",
        ),
    ];

    // Special procedures based on reason
    if matches!(reason, DissolutionReason::ShareholderResolution) {
        procedures.insert(
            0,
            BilingualText::new(
                "经代表三分之二以上表决权的股东通过",
                "Resolution passed by shareholders representing 2/3 or more voting rights",
            ),
        );
    }

    if matches!(reason, DissolutionReason::CourtOrder) {
        procedures.insert(
            0,
            BilingualText::new(
                "向人民法院申请解散公司",
                "Apply to People's Court for dissolution",
            ),
        );
    }

    procedures
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::currency::CnyAmount;

    fn create_test_shareholder(name: &str, pct: f64) -> Shareholder {
        Shareholder {
            name: BilingualText::new(name, name),
            shareholder_type: ShareholderType::NaturalPerson,
            id_number: "123456789".to_string(),
            subscribed_contribution: CnyAmount::from_yuan(100000.0 * pct / 100.0),
            paid_in_contribution: CnyAmount::from_yuan(100000.0 * pct / 100.0),
            contribution_method: ContributionMethod::Monetary,
            shareholding_pct: pct,
            investment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            contribution_deadline: None,
        }
    }

    #[test]
    fn test_validate_company_formation() {
        let registration = CompanyRegistration {
            uscc: "91310000000000000X".to_string(),
            name_zh: "测试有限公司".to_string(),
            name_en: Some("Test LLC".to_string()),
            company_type: CompanyType::LimitedLiabilityCompany,
            registered_capital: CnyAmount::from_yuan(1000000.0),
            subscribed_capital: CnyAmount::from_yuan(1000000.0),
            paid_in_capital: CnyAmount::from_yuan(500000.0),
            establishment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            registered_address: "上海市浦东新区".to_string(),
            business_scope: "技术服务".to_string(),
            legal_representative: "张三".to_string(),
            business_term_years: Some(20),
            industry_code: None,
        };

        let shareholders = vec![
            create_test_shareholder("股东1", 60.0),
            create_test_shareholder("股东2", 40.0),
        ];

        let report = validate_company_formation(&registration, &shareholders, true, true);
        assert!(report.compliant);
    }

    #[test]
    fn test_validate_insufficient_shareholders() {
        let registration = CompanyRegistration {
            uscc: "91310000000000000X".to_string(),
            name_zh: "测试有限公司".to_string(),
            name_en: None,
            company_type: CompanyType::LimitedLiabilityCompany,
            registered_capital: CnyAmount::from_yuan(1000000.0),
            subscribed_capital: CnyAmount::from_yuan(1000000.0),
            paid_in_capital: CnyAmount::from_yuan(500000.0),
            establishment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            registered_address: "上海市".to_string(),
            business_scope: "技术服务".to_string(),
            legal_representative: "张三".to_string(),
            business_term_years: None,
            industry_code: None,
        };

        // LLC needs at least 2 shareholders
        let shareholders = vec![create_test_shareholder("股东1", 100.0)];

        let report = validate_company_formation(&registration, &shareholders, true, true);
        assert!(!report.compliant);
        assert!(
            report
                .violations
                .iter()
                .any(|e| matches!(e, CompanyLawError::InsufficientShareholders { .. }))
        );
    }

    #[test]
    fn test_validate_equity_transfer() {
        let transfer = EquityTransfer {
            transferor: "张三".to_string(),
            transferee: "外部人员".to_string(),
            transfer_amount: CnyAmount::from_yuan(100000.0),
            transfer_pct: 20.0,
            is_internal: false,
            other_shareholders_notified: true,
            notification_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
            consents_received: 3,
            total_other_shareholders: 4,
            preemptive_rights_waived: true,
        };

        let transfer_date = NaiveDate::from_ymd_opt(2024, 2, 15).expect("valid date");
        let report = validate_equity_transfer(&transfer, transfer_date);
        assert!(report.compliant);
    }

    #[test]
    fn test_validate_resolution() {
        // Special resolution needs 2/3
        assert!(validate_resolution(SpecialResolutionMatter::AmendArticles, 0.7).is_ok());
        assert!(validate_resolution(SpecialResolutionMatter::AmendArticles, 0.5).is_err());
    }

    #[test]
    fn test_director_eligibility() {
        assert!(check_director_eligibility(false, false, false, false).is_ok());
        assert!(matches!(
            check_director_eligibility(true, false, false, false),
            Err(CompanyLawError::DirectorDisqualified { .. })
        ));
    }

    #[test]
    fn test_veil_piercing_risk() {
        let risks = check_veil_piercing_risk(true, true, false, false);
        assert_eq!(risks.len(), 2);
    }

    #[test]
    fn test_dissolution_procedures() {
        let procedures = dissolution_procedures(DissolutionReason::ShareholderResolution);
        assert!(procedures.len() >= 5);
        assert!(procedures[0].zh.contains("三分之二"));
    }
}
