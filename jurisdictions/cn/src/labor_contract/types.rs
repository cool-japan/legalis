//! Labor Contract Law Types
//!
//! # 劳动合同法数据类型

#![allow(missing_docs)]

use crate::common::currency::CnyAmount;
use crate::i18n::BilingualText;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Contract type (合同类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// 固定期限劳动合同 / Fixed-term contract
    FixedTerm,
    /// 无固定期限劳动合同 / Open-ended contract
    OpenEnded,
    /// 以完成一定工作任务为期限 / Task-based contract
    TaskBased,
}

impl ContractType {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::FixedTerm => "固定期限劳动合同",
            Self::OpenEnded => "无固定期限劳动合同",
            Self::TaskBased => "以完成一定工作任务为期限的劳动合同",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            Self::FixedTerm => "Fixed-term Labor Contract",
            Self::OpenEnded => "Open-ended Labor Contract",
            Self::TaskBased => "Task-based Labor Contract",
        }
    }
}

/// Employment relationship status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    /// 试用期 / Probation period
    Probation,
    /// 正式员工 / Regular employment
    Regular,
    /// 派遣员工 / Dispatched worker
    Dispatched,
    /// 非全日制 / Part-time
    PartTime,
    /// 合同期满 / Contract expired
    Expired,
    /// 已离职 / Terminated
    Terminated,
}

impl EmploymentStatus {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Probation => "试用期",
            Self::Regular => "正式员工",
            Self::Dispatched => "派遣员工",
            Self::PartTime => "非全日制用工",
            Self::Expired => "合同期满",
            Self::Terminated => "已离职",
        }
    }
}

/// Probation period limits based on contract duration (Article 19)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProbationLimit {
    /// No probation (contract < 3 months or task-based)
    NotAllowed,
    /// Up to 1 month (3 months to 1 year contract)
    OneMonth,
    /// Up to 2 months (1-3 year contract)
    TwoMonths,
    /// Up to 6 months (3+ year or open-ended contract)
    SixMonths,
}

impl ProbationLimit {
    /// Determine probation limit based on contract duration
    pub fn from_contract_months(months: Option<u32>) -> Self {
        match months {
            None => Self::SixMonths, // Open-ended
            Some(m) if m < 3 => Self::NotAllowed,
            Some(m) if m < 12 => Self::OneMonth,
            Some(m) if m < 36 => Self::TwoMonths,
            Some(_) => Self::SixMonths,
        }
    }

    pub fn max_days(&self) -> Option<u32> {
        match self {
            Self::NotAllowed => Some(0),
            Self::OneMonth => Some(30),
            Self::TwoMonths => Some(60),
            Self::SixMonths => Some(180),
        }
    }

    pub fn description_zh(&self) -> &str {
        match self {
            Self::NotAllowed => "不得约定试用期",
            Self::OneMonth => "试用期不得超过一个月",
            Self::TwoMonths => "试用期不得超过二个月",
            Self::SixMonths => "试用期不得超过六个月",
        }
    }
}

/// Termination reason categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationReason {
    // == Employee-initiated (Article 37-38) ==
    /// 员工辞职 / Employee resignation (30 days notice)
    EmployeeResignation,
    /// 试用期辞职 / Resignation during probation (3 days notice)
    ProbationResignation,
    /// 即时解除（用人单位违法）/ Immediate termination due to employer violation
    EmployerViolation,

    // == Employer-initiated with notice (Article 40) ==
    /// 医疗期满不能从事工作 / Unable to work after medical treatment period
    MedicalInability,
    /// 不能胜任工作 / Incompetence after training or adjustment
    Incompetence,
    /// 客观情况重大变化 / Major change in objective circumstances
    ObjectiveChange,

    // == Employer-initiated without notice (Article 39) ==
    /// 试用期不符合录用条件 / Failed probation requirements
    FailedProbation,
    /// 严重违反规章制度 / Serious violation of rules
    SeriousViolation,
    /// 严重失职 / Serious dereliction of duty
    SeriousNeglect,
    /// 双重劳动关系 / Dual employment affecting work
    DualEmployment,
    /// 劳动合同无效 / Contract invalidation
    ContractInvalid,
    /// 刑事责任 / Criminal liability
    CriminalLiability,

    // == Mass layoff (Article 41) ==
    /// 经济性裁员 / Economic layoff
    EconomicLayoff,

    // == Mutual agreement ==
    /// 协商一致解除 / Mutual agreement
    MutualAgreement,

    // == Contract expiration ==
    /// 合同期满 / Contract expiration
    ContractExpiration,
}

impl TerminationReason {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::EmployeeResignation => "员工辞职",
            Self::ProbationResignation => "试用期辞职",
            Self::EmployerViolation => "用人单位违法解除",
            Self::MedicalInability => "医疗期满不能从事工作",
            Self::Incompetence => "不能胜任工作",
            Self::ObjectiveChange => "客观情况重大变化",
            Self::FailedProbation => "试用期不符合录用条件",
            Self::SeriousViolation => "严重违反规章制度",
            Self::SeriousNeglect => "严重失职",
            Self::DualEmployment => "双重劳动关系",
            Self::ContractInvalid => "劳动合同无效",
            Self::CriminalLiability => "被追究刑事责任",
            Self::EconomicLayoff => "经济性裁员",
            Self::MutualAgreement => "协商一致解除",
            Self::ContractExpiration => "合同期满",
        }
    }

    /// Whether this termination requires severance pay
    pub fn requires_severance(&self) -> bool {
        matches!(
            self,
            Self::EmployerViolation
                | Self::MedicalInability
                | Self::Incompetence
                | Self::ObjectiveChange
                | Self::EconomicLayoff
                | Self::MutualAgreement
                | Self::ContractExpiration
        )
    }

    /// Whether employer can terminate without notice
    pub fn immediate_termination(&self) -> bool {
        matches!(
            self,
            Self::FailedProbation
                | Self::SeriousViolation
                | Self::SeriousNeglect
                | Self::DualEmployment
                | Self::ContractInvalid
                | Self::CriminalLiability
        )
    }

    /// Article reference
    pub fn article(&self) -> u8 {
        match self {
            Self::EmployeeResignation | Self::ProbationResignation => 37,
            Self::EmployerViolation => 38,
            Self::FailedProbation
            | Self::SeriousViolation
            | Self::SeriousNeglect
            | Self::DualEmployment
            | Self::ContractInvalid
            | Self::CriminalLiability => 39,
            Self::MedicalInability | Self::Incompetence | Self::ObjectiveChange => 40,
            Self::EconomicLayoff => 41,
            Self::MutualAgreement => 36,
            Self::ContractExpiration => 44,
        }
    }
}

/// Labor contract entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaborContract {
    /// Contract ID
    pub id: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Employer name
    pub employer: BilingualText,
    /// Employee name
    pub employee_name: String,
    /// Employee ID number
    pub employee_id: String,
    /// Position/job title
    pub position: BilingualText,
    /// Work location
    pub work_location: String,
    /// Contract start date
    pub start_date: NaiveDate,
    /// Contract end date (None for open-ended)
    pub end_date: Option<NaiveDate>,
    /// Probation end date
    pub probation_end_date: Option<NaiveDate>,
    /// Monthly salary
    pub monthly_salary: CnyAmount,
    /// Working hours type
    pub working_hours: WorkingHoursType,
    /// Social insurance enrolled
    pub social_insurance: SocialInsuranceStatus,
    /// Housing fund contribution
    pub housing_fund: HousingFundStatus,
    /// Current status
    pub status: EmploymentStatus,
    /// Renewal count
    pub renewal_count: u32,
}

impl LaborContract {
    /// Calculate contract duration in months
    pub fn duration_months(&self) -> Option<u32> {
        self.end_date.map(|end| {
            let months = (end.year() - self.start_date.year()) * 12
                + (end.month() as i32 - self.start_date.month() as i32);
            months.max(0) as u32
        })
    }

    /// Check if eligible for open-ended contract
    /// (Article 14: after 2 renewals or 10 years continuous service)
    pub fn eligible_for_open_ended(&self, years_of_service: f64) -> bool {
        self.renewal_count >= 2 || years_of_service >= 10.0
    }

    /// Get maximum allowed probation period
    pub fn max_probation(&self) -> ProbationLimit {
        ProbationLimit::from_contract_months(self.duration_months())
    }
}

/// Working hours arrangement (Article 36)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkingHoursType {
    /// 标准工时制 / Standard hours (8h/day, 40h/week)
    Standard,
    /// 综合计算工时制 / Comprehensive working hours
    Comprehensive,
    /// 不定时工作制 / Flexible hours
    Flexible,
    /// 非全日制 / Part-time (< 4h/day, < 24h/week average)
    PartTime,
}

impl WorkingHoursType {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Standard => "标准工时制",
            Self::Comprehensive => "综合计算工时制",
            Self::Flexible => "不定时工作制",
            Self::PartTime => "非全日制用工",
        }
    }

    /// Maximum daily hours
    pub fn max_daily_hours(&self) -> Option<u8> {
        match self {
            Self::Standard => Some(8),
            Self::Comprehensive => None, // Calculated over period
            Self::Flexible => None,
            Self::PartTime => Some(4),
        }
    }

    /// Maximum weekly hours
    pub fn max_weekly_hours(&self) -> Option<u8> {
        match self {
            Self::Standard => Some(40),
            Self::Comprehensive => Some(40), // Average over period
            Self::Flexible => None,
            Self::PartTime => Some(24),
        }
    }
}

/// Overtime rates (Article 44)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OvertimeType {
    /// 延长工作时间 / Extended hours (weekday)
    ExtendedHours,
    /// 休息日加班 / Rest day work
    RestDay,
    /// 法定节假日加班 / Statutory holiday work
    StatutoryHoliday,
}

impl OvertimeType {
    /// Overtime pay rate multiplier
    pub fn rate_multiplier(&self) -> f64 {
        match self {
            Self::ExtendedHours => 1.5,
            Self::RestDay => 2.0,
            Self::StatutoryHoliday => 3.0,
        }
    }

    pub fn name_zh(&self) -> &str {
        match self {
            Self::ExtendedHours => "延长工作时间",
            Self::RestDay => "休息日加班",
            Self::StatutoryHoliday => "法定节假日加班",
        }
    }
}

/// Social insurance status (五险)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SocialInsuranceStatus {
    /// 养老保险 / Pension insurance
    pub pension: bool,
    /// 医疗保险 / Medical insurance
    pub medical: bool,
    /// 失业保险 / Unemployment insurance
    pub unemployment: bool,
    /// 工伤保险 / Work injury insurance
    pub work_injury: bool,
    /// 生育保险 / Maternity insurance
    pub maternity: bool,
}

impl SocialInsuranceStatus {
    /// Check if fully enrolled (五险)
    pub fn is_complete(&self) -> bool {
        self.pension && self.medical && self.unemployment && self.work_injury && self.maternity
    }

    /// Missing insurance types
    pub fn missing(&self) -> Vec<&str> {
        let mut missing = Vec::new();
        if !self.pension {
            missing.push("养老保险");
        }
        if !self.medical {
            missing.push("医疗保险");
        }
        if !self.unemployment {
            missing.push("失业保险");
        }
        if !self.work_injury {
            missing.push("工伤保险");
        }
        if !self.maternity {
            missing.push("生育保险");
        }
        missing
    }
}

/// Housing provident fund status (住房公积金)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HousingFundStatus {
    /// Enrolled
    pub enrolled: bool,
    /// Monthly contribution base
    pub contribution_base: Option<CnyAmount>,
    /// Contribution rate (employer + employee, typically 5-12% each)
    pub contribution_rate: Option<f64>,
}

/// Severance calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeveranceCalculation {
    /// Years of service (rounded up for 6+ months)
    pub years_of_service: f64,
    /// Number of months for calculation
    pub months_count: f64,
    /// Monthly calculation base
    pub monthly_base: CnyAmount,
    /// Whether capped at 3x average wage
    pub is_capped: bool,
    /// Cap amount (if applicable)
    pub cap_amount: Option<CnyAmount>,
    /// Total severance amount
    pub total_amount: CnyAmount,
    /// Legal basis
    pub legal_basis: BilingualText,
}

/// Annual leave entitlement (Article 3 of Annual Leave Regulations)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnualLeaveEntitlement {
    /// Cumulative years of work experience
    pub cumulative_years: u32,
    /// Annual leave days entitled
    pub days: u32,
}

impl AnnualLeaveEntitlement {
    /// Calculate annual leave based on cumulative work experience
    pub fn from_years(years: u32) -> Self {
        let days = match years {
            0 => 0,        // Less than 1 year: no statutory annual leave
            1..=9 => 5,    // 1-10 years: 5 days
            10..=19 => 10, // 10-20 years: 10 days
            _ => 15,       // 20+ years: 15 days
        };
        Self {
            cumulative_years: years,
            days,
        }
    }
}

/// Non-compete agreement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonCompeteAgreement {
    /// Duration (max 2 years per Article 24)
    pub duration_months: u32,
    /// Geographic scope
    pub geographic_scope: String,
    /// Industry scope
    pub industry_scope: String,
    /// Monthly compensation (must be paid during non-compete period)
    pub monthly_compensation: CnyAmount,
    /// Penalty for breach
    pub breach_penalty: Option<CnyAmount>,
}

impl NonCompeteAgreement {
    /// Check if duration is valid (max 2 years)
    pub fn is_duration_valid(&self) -> bool {
        self.duration_months <= 24
    }
}

/// Economic layoff requirements (Article 41)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EconomicLayoff {
    /// Number of employees to be laid off
    pub layoff_count: u32,
    /// Total workforce
    pub total_workforce: u32,
    /// Reason for layoff
    pub reason: LayoffReason,
    /// Notice given to union (30 days required)
    pub union_notified: bool,
    /// Notice given to all employees
    pub employees_notified: bool,
    /// Labor bureau reported
    pub labor_bureau_reported: bool,
    /// Priority retention list provided
    pub priority_retention_applied: bool,
}

impl EconomicLayoff {
    /// Check if mass layoff threshold reached (20+ or 10%+)
    pub fn is_mass_layoff(&self) -> bool {
        self.layoff_count >= 20
            || (self.total_workforce > 0
                && (self.layoff_count as f64 / self.total_workforce as f64) >= 0.1)
    }
}

/// Economic layoff reasons (Article 41)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoffReason {
    /// 企业破产重整 / Bankruptcy restructuring
    BankruptcyRestructuring,
    /// 生产经营发生严重困难 / Serious operational difficulties
    SeriousDifficulties,
    /// 企业转产、重大技术革新或者经营方式调整 / Production change or technological revolution
    BusinessChange,
    /// 客观经济情况发生重大变化 / Major change in objective economic conditions
    EconomicChange,
}

impl LayoffReason {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::BankruptcyRestructuring => "企业破产重整",
            Self::SeriousDifficulties => "生产经营发生严重困难",
            Self::BusinessChange => "企业转产、重大技术革新或者经营方式调整",
            Self::EconomicChange => "客观经济情况发生重大变化",
        }
    }
}

/// Protected employee categories (cannot be terminated under Article 40-41)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectedCategory {
    /// 从事接触职业病危害作业未进行离岗前职业健康检查
    OccupationalHazardWorker,
    /// 疑似职业病病人在诊断或者医学观察期间
    SuspectedOccupationalDisease,
    /// 在本单位患职业病或因工负伤丧失或部分丧失劳动能力
    WorkInjuryDisabled,
    /// 患病或非因工负伤，在规定的医疗期内
    MedicalTreatmentPeriod,
    /// 女职工在孕期、产期、哺乳期
    PregnancyMaternity,
    /// 在本单位连续工作满十五年，且距法定退休年龄不足五年
    NearRetirement,
}

impl ProtectedCategory {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::OccupationalHazardWorker => "从事接触职业病危害作业的劳动者",
            Self::SuspectedOccupationalDisease => "疑似职业病病人",
            Self::WorkInjuryDisabled => "因工负伤丧失或部分丧失劳动能力",
            Self::MedicalTreatmentPeriod => "在规定医疗期内",
            Self::PregnancyMaternity => "孕期、产期、哺乳期女职工",
            Self::NearRetirement => "连续工作满十五年且距退休不足五年",
        }
    }

    pub fn article(&self) -> u8 {
        42
    }
}

/// Labor dispatch (劳务派遣) restrictions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaborDispatch {
    /// Dispatching agency name
    pub agency_name: String,
    /// Agency license number
    pub agency_license: String,
    /// Position type
    pub position_type: DispatchPositionType,
    /// Duration
    pub duration_months: u32,
    /// Host company total workforce
    pub host_workforce: u32,
    /// Dispatched worker count at host
    pub dispatched_count: u32,
}

impl LaborDispatch {
    /// Check if dispatch ratio within limit (10%)
    pub fn is_ratio_compliant(&self) -> bool {
        if self.host_workforce == 0 {
            return false;
        }
        (self.dispatched_count as f64 / self.host_workforce as f64) <= 0.1
    }
}

/// Dispatch position types (临时性、辅助性、替代性)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchPositionType {
    /// 临时性 / Temporary (< 6 months)
    Temporary,
    /// 辅助性 / Auxiliary (non-core business)
    Auxiliary,
    /// 替代性 / Substitute (replacing absent employee)
    Substitute,
}

impl DispatchPositionType {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Temporary => "临时性岗位",
            Self::Auxiliary => "辅助性岗位",
            Self::Substitute => "替代性岗位",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probation_limit() {
        // Task-based: no probation
        assert_eq!(
            ProbationLimit::from_contract_months(Some(2)),
            ProbationLimit::NotAllowed
        );

        // 3 months to 1 year: 1 month max
        assert_eq!(
            ProbationLimit::from_contract_months(Some(6)),
            ProbationLimit::OneMonth
        );

        // 1-3 years: 2 months max
        assert_eq!(
            ProbationLimit::from_contract_months(Some(24)),
            ProbationLimit::TwoMonths
        );

        // 3+ years: 6 months max
        assert_eq!(
            ProbationLimit::from_contract_months(Some(48)),
            ProbationLimit::SixMonths
        );

        // Open-ended: 6 months max
        assert_eq!(
            ProbationLimit::from_contract_months(None),
            ProbationLimit::SixMonths
        );
    }

    #[test]
    fn test_termination_severance() {
        // Employee resignation: no severance
        assert!(!TerminationReason::EmployeeResignation.requires_severance());

        // Serious violation: no severance
        assert!(!TerminationReason::SeriousViolation.requires_severance());

        // Economic layoff: severance required
        assert!(TerminationReason::EconomicLayoff.requires_severance());

        // Contract expiration: severance required
        assert!(TerminationReason::ContractExpiration.requires_severance());
    }

    #[test]
    fn test_overtime_rates() {
        assert_eq!(OvertimeType::ExtendedHours.rate_multiplier(), 1.5);
        assert_eq!(OvertimeType::RestDay.rate_multiplier(), 2.0);
        assert_eq!(OvertimeType::StatutoryHoliday.rate_multiplier(), 3.0);
    }

    #[test]
    fn test_social_insurance_complete() {
        let complete = SocialInsuranceStatus {
            pension: true,
            medical: true,
            unemployment: true,
            work_injury: true,
            maternity: true,
        };
        assert!(complete.is_complete());

        let incomplete = SocialInsuranceStatus {
            pension: true,
            medical: true,
            unemployment: false,
            work_injury: true,
            maternity: true,
        };
        assert!(!incomplete.is_complete());
        assert_eq!(incomplete.missing(), vec!["失业保险"]);
    }

    #[test]
    fn test_annual_leave() {
        assert_eq!(AnnualLeaveEntitlement::from_years(0).days, 0);
        assert_eq!(AnnualLeaveEntitlement::from_years(5).days, 5);
        assert_eq!(AnnualLeaveEntitlement::from_years(15).days, 10);
        assert_eq!(AnnualLeaveEntitlement::from_years(25).days, 15);
    }

    #[test]
    fn test_non_compete_duration() {
        let valid = NonCompeteAgreement {
            duration_months: 24,
            geographic_scope: "全国".to_string(),
            industry_scope: "互联网".to_string(),
            monthly_compensation: CnyAmount::from_yuan(5000.0),
            breach_penalty: None,
        };
        assert!(valid.is_duration_valid());

        let invalid = NonCompeteAgreement {
            duration_months: 36,
            geographic_scope: "全国".to_string(),
            industry_scope: "互联网".to_string(),
            monthly_compensation: CnyAmount::from_yuan(5000.0),
            breach_penalty: None,
        };
        assert!(!invalid.is_duration_valid());
    }

    #[test]
    fn test_mass_layoff_threshold() {
        // 20+ employees
        let layoff_20 = EconomicLayoff {
            layoff_count: 20,
            total_workforce: 100,
            reason: LayoffReason::SeriousDifficulties,
            union_notified: true,
            employees_notified: true,
            labor_bureau_reported: true,
            priority_retention_applied: true,
        };
        assert!(layoff_20.is_mass_layoff());

        // 10%+ of workforce
        let layoff_10pct = EconomicLayoff {
            layoff_count: 15,
            total_workforce: 100,
            reason: LayoffReason::SeriousDifficulties,
            union_notified: true,
            employees_notified: true,
            labor_bureau_reported: true,
            priority_retention_applied: true,
        };
        assert!(layoff_10pct.is_mass_layoff());

        // Below threshold
        let layoff_small = EconomicLayoff {
            layoff_count: 5,
            total_workforce: 100,
            reason: LayoffReason::SeriousDifficulties,
            union_notified: true,
            employees_notified: true,
            labor_bureau_reported: true,
            priority_retention_applied: true,
        };
        assert!(!layoff_small.is_mass_layoff());
    }

    #[test]
    fn test_dispatch_ratio() {
        let compliant = LaborDispatch {
            agency_name: "测试派遣公司".to_string(),
            agency_license: "123456".to_string(),
            position_type: DispatchPositionType::Auxiliary,
            duration_months: 12,
            host_workforce: 100,
            dispatched_count: 10,
        };
        assert!(compliant.is_ratio_compliant());

        let non_compliant = LaborDispatch {
            agency_name: "测试派遣公司".to_string(),
            agency_license: "123456".to_string(),
            position_type: DispatchPositionType::Auxiliary,
            duration_months: 12,
            host_workforce: 100,
            dispatched_count: 15,
        };
        assert!(!non_compliant.is_ratio_compliant());
    }
}
