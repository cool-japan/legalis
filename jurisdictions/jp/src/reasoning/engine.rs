//! Legal Reasoning Engine for Japanese Labor Law (法的推論エンジン).
//!
//! Provides automated compliance analysis and violation detection.
//! コンプライアンス分析と違反検出を自動化

use legalis_core::StatuteRegistry;

use super::context::JpEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::all_labor_statutes;
use super::types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};

use crate::labor_law::types::{
    Article36Agreement, EmploymentContract, MONTHLY_OVERTIME_LIMIT, MonthlyWorkingSummary,
    STATUTORY_HOURS_PER_WEEK, TerminationNotice,
};

/// Legal Reasoning Engine for Japanese Labor Law
/// 日本労働法の法的推論エンジン
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Create a new reasoning engine with all Japanese labor statutes
    /// 全日本労働法令を含む新しい推論エンジンを作成
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_labor_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Create with custom registry
    #[must_use]
    pub fn with_registry(registry: StatuteRegistry) -> Self {
        Self { registry }
    }

    /// Analyze an employment contract for compliance
    /// 雇用契約のコンプライアンス分析
    pub fn analyze_employment_contract(
        &self,
        contract: &EmploymentContract,
    ) -> ReasoningResult<LegalAnalysis> {
        let _ctx = JpEvaluationContext::new(contract);
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "雇用契約の法的分析 / Employment Contract Analysis",
        );

        // Check statutory working hours (Art. 32)
        // 法定労働時間チェック（第32条）
        if contract.weekly_hours() > STATUTORY_HOURS_PER_WEEK {
            analysis.add_violation(
                Violation::new(
                    "LSA_Art32",
                    "法定労働時間 / Statutory Working Hours",
                    format!(
                        "週{}時間は法定上限40時間を超過 / {} hours/week exceeds 40-hour limit",
                        contract.weekly_hours(),
                        contract.weekly_hours()
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("労働基準法第32条")
                .with_remediation("36協定を締結するか、労働時間を削減してください"),
            );
        }

        // Check for indefinite conversion eligibility (LCA Art. 18)
        // 無期転換権チェック（労働契約法第18条）
        if contract.is_eligible_for_indefinite_conversion() {
            analysis.add_violation(
                Violation::new(
                    "LCA_Art18",
                    "無期転換ルール / Indefinite Conversion Rule",
                    "有期契約5年超過により無期転換申込権が発生 / Right to convert to indefinite term has arisen",
                    ViolationSeverity::Advisory,
                )
                .with_legal_reference("労働契約法第18条"),
            );
        }

        // Update overall status
        if !analysis.violations.is_empty() {
            let has_major = analysis
                .violations
                .iter()
                .any(|v| v.severity >= ViolationSeverity::Major);

            analysis.status = if has_major {
                ComplianceStatus::NonCompliant
            } else {
                ComplianceStatus::PartiallyCompliant
            };
        }

        Ok(analysis)
    }

    /// Analyze monthly working summary for overtime violations
    /// 月間労働時間の時間外労働違反分析
    pub fn analyze_monthly_summary(
        &self,
        summary: &MonthlyWorkingSummary,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "月間労働時間の法的分析 / Monthly Working Time Analysis",
        );

        // Check overtime limit (Art. 36)
        // 時間外労働上限チェック（第36条）
        if summary.exceeds_overtime_limit() {
            let excess = summary.overtime_hours - MONTHLY_OVERTIME_LIMIT as f64;
            analysis.add_violation(
                Violation::new(
                    "OT_LIMIT",
                    "時間外労働上限 / Overtime Limit",
                    format!(
                        "月間時間外労働{:.1}時間（上限超過{:.1}時間）/ {:.1}h overtime ({:.1}h over limit)",
                        summary.overtime_hours, excess, summary.overtime_hours, excess
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("労働基準法第36条")
                .with_remediation("時間外労働を月45時間以内に削減してください"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        // Check for excessive working hours (80+ hours overtime = karoshi line)
        // 過労死ライン（月80時間超）チェック
        if summary.overtime_hours >= 80.0 {
            analysis.add_violation(
                Violation::new(
                    "KAROSHI",
                    "過労死ライン / Karoshi Line",
                    format!(
                        "月間時間外{:.1}時間は過労死認定基準を超過 / {:.1}h overtime exceeds karoshi threshold",
                        summary.overtime_hours, summary.overtime_hours
                    ),
                    ViolationSeverity::Critical,
                )
                .with_remediation("労働時間を直ちに削減し、健康診断を実施してください"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::Critical;
        }

        Ok(analysis)
    }

    /// Analyze Article 36 Agreement for validity
    /// 36協定の有効性分析
    pub fn analyze_article_36_agreement(
        &self,
        agreement: &Article36Agreement,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "36協定の法的分析 / Article 36 Agreement Analysis",
        );

        // Check agreement validity
        if let Err(msg) = agreement.validate() {
            analysis.add_violation(
                Violation::new(
                    "LSA_Art36",
                    "36協定 / Article 36 Agreement",
                    format!("36協定の不備: {} / Invalid agreement: {}", msg, msg),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("労働基準法第36条"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        // Check if special circumstances are valid
        if agreement.has_special_circumstances && !agreement.is_special_circumstances_valid() {
            analysis.add_violation(
                Violation::new(
                    "LSA_Art36_SPECIAL",
                    "特別条項 / Special Circumstances",
                    "特別条項の設定が不適切 / Invalid special circumstances configuration",
                    ViolationSeverity::Moderate,
                )
                .with_legal_reference("労働基準法第36条第5項"),
            );
            if analysis.status == ComplianceStatus::Compliant {
                analysis.status = ComplianceStatus::PartiallyCompliant;
            }
        }

        Ok(analysis)
    }

    /// Analyze termination notice for compliance
    /// 解雇予告のコンプライアンス分析
    pub fn analyze_termination_notice(
        &self,
        notice: &TerminationNotice,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "解雇予告の法的分析 / Termination Notice Analysis",
        );

        // Check sufficient notice period (Art. 20)
        // 解雇予告期間チェック（第20条）
        if !notice.has_sufficient_notice_period() {
            analysis.add_violation(
                Violation::new(
                    "LSA_Art20",
                    "解雇予告 / Dismissal Notice",
                    format!(
                        "解雇予告期間{}日は30日未満 / Notice period {} days is less than 30 days",
                        notice.notice_period_days(),
                        notice.notice_period_days()
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("労働基準法第20条")
                .with_remediation("30日分の平均賃金を予告手当として支払ってください"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        Ok(analysis)
    }

    /// Get the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::labor_law::types::{EmploymentType, WorkPattern};
    use chrono::Utc;

    #[test]
    fn test_compliant_contract() {
        let engine = LegalReasoningEngine::new();
        let contract = EmploymentContract {
            employee_name: "田中太郎".to_string(),
            employer_name: "株式会社ABC".to_string(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_pattern: WorkPattern::Regular,
            start_date: Utc::now(),
            end_date: None,
            base_wage_jpy: 300_000,
            hours_per_day: 8,
            days_per_week: 5,
            job_description: "ソフトウェアエンジニア".to_string(),
            work_location: "東京".to_string(),
            probation_period_days: Some(90),
            renewal_count: 0,
        };

        let analysis = engine.analyze_employment_contract(&contract).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_overtime_violation() {
        let engine = LegalReasoningEngine::new();
        let summary = MonthlyWorkingSummary {
            year: 2026,
            month: 1,
            total_hours: 200.0,
            overtime_hours: 70.0, // Exceeds 60h limit
            late_night_hours: 10.0,
            holiday_hours: 8.0,
            days_worked: 22,
        };

        let analysis = engine.analyze_monthly_summary(&summary).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_karoshi_line() {
        let engine = LegalReasoningEngine::new();
        let summary = MonthlyWorkingSummary {
            year: 2026,
            month: 1,
            total_hours: 240.0,
            overtime_hours: 85.0, // Exceeds karoshi line
            late_night_hours: 20.0,
            holiday_hours: 16.0,
            days_worked: 24,
        };

        let analysis = engine.analyze_monthly_summary(&summary).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }
}
