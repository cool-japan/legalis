//! EvaluationContext implementations for Japanese legal entities.
//!
//! Bridges Japanese labor law types to the legalis-core reasoning framework.
//! 日本労働法の型をlegalis-core推論フレームワークに接続

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::labor_law::types::{
    Article36Agreement, EmploymentContract, EmploymentType, MonthlyWorkingSummary,
    TerminationNotice, TerminationType, WorkPattern,
};

/// Wrapper providing EvaluationContext for Japanese labor law entities
/// 日本労働法エンティティ用EvaluationContextラッパー
pub struct JpEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> JpEvaluationContext<'a, T> {
    /// Create a new evaluation context
    #[must_use]
    pub const fn new(entity: &'a T) -> Self {
        Self { entity }
    }
}

// ============================================================================
// EvaluationContext for EmploymentContract (雇用契約)
// ============================================================================

impl EvaluationContext for JpEvaluationContext<'_, EmploymentContract> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "employment_type" | "雇用形態" => Some(match self.entity.employment_type {
                EmploymentType::IndefiniteTerm => "indefinite".to_string(),
                EmploymentType::FixedTerm => "fixed_term".to_string(),
                EmploymentType::PartTime => "part_time".to_string(),
                EmploymentType::Temporary => "temporary".to_string(),
                EmploymentType::ContractWorker => "contract".to_string(),
            }),
            "work_pattern" | "勤務形態" => Some(match self.entity.work_pattern {
                WorkPattern::Regular => "regular".to_string(),
                WorkPattern::Flextime => "flextime".to_string(),
                WorkPattern::Shift => "shift".to_string(),
                WorkPattern::Discretionary => "discretionary".to_string(),
            }),
            "employee_name" | "従業員名" => Some(self.entity.employee_name.clone()),
            "employer_name" | "使用者名" => Some(self.entity.employer_name.clone()),
            "job_description" | "職務内容" => Some(self.entity.job_description.clone()),
            "work_location" | "勤務場所" => Some(self.entity.work_location.clone()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        Some(self.entity.base_wage_jpy)
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true // JP jurisdiction
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        let now = chrono::Utc::now();
        let duration = now - self.entity.start_date;
        let days = duration.num_days().max(0) as u32;

        match unit {
            DurationUnit::Days => Some(days),
            DurationUnit::Weeks => Some(days / 7),
            DurationUnit::Months => Some(days / 30),
            DurationUnit::Years => Some(days / 365),
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hours_per_day" | "1日の労働時間" => Some(self.entity.hours_per_day as f64),
            "days_per_week" | "週の労働日数" => Some(self.entity.days_per_week as f64),
            "weekly_hours" | "週労働時間" => Some(self.entity.weekly_hours() as f64),
            "base_wage" | "基本給" => Some(self.entity.base_wage_jpy as f64),
            "renewal_count" | "契約更新回数" => Some(self.entity.renewal_count as f64),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for Article36Agreement (36協定)
// ============================================================================

impl EvaluationContext for JpEvaluationContext<'_, Article36Agreement> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "employer_name" | "使用者名" => Some(self.entity.employer_name.clone()),
            "labor_representative" | "労働者代表" => {
                Some(self.entity.labor_representative.clone())
            }
            "has_special_circumstances" | "特別条項あり" => {
                Some(self.entity.has_special_circumstances.to_string())
            }
            "is_valid" | "有効" => Some(self.entity.is_within_standard_limits().to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        match unit {
            DurationUnit::Days => {
                let duration = self.entity.expiration_date - self.entity.effective_date;
                Some(duration.num_days().max(0) as u32)
            }
            _ => None,
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "max_overtime_per_day" | "1日時間外上限" => {
                Some(self.entity.max_overtime_per_day as f64)
            }
            "max_overtime_per_month" | "月間時間外上限" => {
                Some(self.entity.max_overtime_per_month as f64)
            }
            "max_overtime_per_year" | "年間時間外上限" => {
                Some(self.entity.max_overtime_per_year as f64)
            }
            "special_max_per_month" | "特別条項月間上限" => {
                self.entity.special_max_per_month.map(|v| v as f64)
            }
            "special_months_per_year" | "特別条項適用月数" => {
                self.entity.special_months_per_year.map(|v| v as f64)
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for MonthlyWorkingSummary (月間労働時間集計)
// ============================================================================

impl EvaluationContext for JpEvaluationContext<'_, MonthlyWorkingSummary> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "exceeds_limit" | "上限超過" => {
                Some(self.entity.exceeds_overtime_limit().to_string())
            }
            "year" | "年" => Some(self.entity.year.to_string()),
            "month" | "月" => Some(self.entity.month.to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "total_hours" | "総労働時間" => Some(self.entity.total_hours),
            "overtime_hours" | "時間外労働時間" => Some(self.entity.overtime_hours),
            "late_night_hours" | "深夜労働時間" => Some(self.entity.late_night_hours),
            "holiday_hours" | "休日労働時間" => Some(self.entity.holiday_hours),
            "days_worked" | "労働日数" => Some(self.entity.days_worked as f64),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for TerminationNotice (解雇予告)
// ============================================================================

impl EvaluationContext for JpEvaluationContext<'_, TerminationNotice> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "termination_type" | "退職種別" => Some(match self.entity.termination_type {
                TerminationType::OrdinaryDismissal => "ordinary_dismissal".to_string(),
                TerminationType::DisciplinaryDismissal => "disciplinary_dismissal".to_string(),
                TerminationType::VoluntaryResignation => "voluntary_resignation".to_string(),
                TerminationType::MutualAgreement => "mutual_agreement".to_string(),
                TerminationType::ContractExpiration => "contract_expiration".to_string(),
                TerminationType::Retirement => "retirement".to_string(),
            }),
            "employee_name" | "従業員名" => Some(self.entity.employee_name.clone()),
            "reason" | "理由" => Some(self.entity.reason.clone()),
            "has_sufficient_notice" | "予告期間充足" => {
                Some(self.entity.has_sufficient_notice_period().to_string())
            }
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        self.entity.severance_pay_jpy
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        match unit {
            DurationUnit::Days => Some(self.entity.notice_period_days().max(0) as u32),
            _ => None,
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "notice_days" | "予告日数" => Some(self.entity.notice_period_days() as f64),
            "severance_pay" | "退職金" => self.entity.severance_pay_jpy.map(|v| v as f64),
            "notice_allowance" | "予告手当" => {
                self.entity.notice_allowance_jpy.map(|v| v as f64)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_employment_contract_context() {
        let contract = EmploymentContract {
            employee_name: "山田太郎".to_string(),
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

        let ctx = JpEvaluationContext::new(&contract);
        assert_eq!(
            ctx.get_attribute("employment_type"),
            Some("indefinite".to_string())
        );
        assert_eq!(ctx.get_income(), Some(300_000));
        assert_eq!(ctx.evaluate_formula("weekly_hours"), Some(40.0));
    }
}
