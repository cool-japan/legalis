//! Company Law Types
//!
//! # 公司法数据类型

#![allow(missing_docs)]

use crate::common::currency::CnyAmount;
use crate::i18n::BilingualText;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Company type (公司类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// 有限责任公司 / Limited Liability Company
    LimitedLiabilityCompany,
    /// 一人有限责任公司 / Single-shareholder LLC
    SingleShareholderLlc,
    /// 股份有限公司 / Joint Stock Company
    JointStockCompany,
    /// 国有独资公司 / Wholly State-Owned Company
    WhollyStateOwned,
    /// 外商独资企业 / Wholly Foreign-Owned Enterprise
    WhollyForeignOwned,
    /// 中外合资企业 / Sino-Foreign Joint Venture
    JointVenture,
    /// 中外合作企业 / Sino-Foreign Cooperative Enterprise
    CooperativeEnterprise,
}

impl CompanyType {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::LimitedLiabilityCompany => "有限责任公司",
            Self::SingleShareholderLlc => "一人有限责任公司",
            Self::JointStockCompany => "股份有限公司",
            Self::WhollyStateOwned => "国有独资公司",
            Self::WhollyForeignOwned => "外商独资企业",
            Self::JointVenture => "中外合资企业",
            Self::CooperativeEnterprise => "中外合作企业",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            Self::LimitedLiabilityCompany => "Limited Liability Company",
            Self::SingleShareholderLlc => "Single-shareholder LLC",
            Self::JointStockCompany => "Joint Stock Company (Co., Ltd.)",
            Self::WhollyStateOwned => "Wholly State-Owned Company",
            Self::WhollyForeignOwned => "Wholly Foreign-Owned Enterprise",
            Self::JointVenture => "Sino-Foreign Joint Venture",
            Self::CooperativeEnterprise => "Sino-Foreign Cooperative Enterprise",
        }
    }

    /// Minimum shareholders/promoters required
    pub fn min_shareholders(&self) -> u32 {
        match self {
            Self::SingleShareholderLlc | Self::WhollyStateOwned | Self::WhollyForeignOwned => 1,
            Self::LimitedLiabilityCompany | Self::JointVenture | Self::CooperativeEnterprise => 2,
            Self::JointStockCompany => 2, // 2023 revision: 2+ promoters
        }
    }

    /// Maximum shareholders for LLC
    pub fn max_shareholders(&self) -> Option<u32> {
        match self {
            Self::LimitedLiabilityCompany => Some(50),
            Self::SingleShareholderLlc => Some(1),
            _ => None, // No maximum
        }
    }

    /// Whether shares can be publicly traded
    pub fn publicly_tradeable(&self) -> bool {
        matches!(self, Self::JointStockCompany)
    }
}

/// Company registration information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyRegistration {
    /// Unified Social Credit Code (统一社会信用代码)
    pub uscc: String,
    /// Company name (Chinese)
    pub name_zh: String,
    /// Company name (English, if any)
    pub name_en: Option<String>,
    /// Company type
    pub company_type: CompanyType,
    /// Registered capital (注册资本)
    pub registered_capital: CnyAmount,
    /// Subscribed capital (认缴资本)
    pub subscribed_capital: CnyAmount,
    /// Paid-in capital (实缴资本)
    pub paid_in_capital: CnyAmount,
    /// Establishment date
    pub establishment_date: NaiveDate,
    /// Registered address
    pub registered_address: String,
    /// Business scope (经营范围)
    pub business_scope: String,
    /// Legal representative
    pub legal_representative: String,
    /// Business term (years, None for indefinite)
    pub business_term_years: Option<u32>,
    /// Industry classification code
    pub industry_code: Option<String>,
}

/// Shareholder information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Shareholder name
    pub name: BilingualText,
    /// Shareholder type
    pub shareholder_type: ShareholderType,
    /// ID number (身份证号) or USCC (for corporate shareholders)
    pub id_number: String,
    /// Subscribed contribution (认缴出资额)
    pub subscribed_contribution: CnyAmount,
    /// Paid-in contribution (实缴出资额)
    pub paid_in_contribution: CnyAmount,
    /// Contribution method
    pub contribution_method: ContributionMethod,
    /// Shareholding percentage
    pub shareholding_pct: f64,
    /// Investment date
    pub investment_date: NaiveDate,
    /// Deadline for capital contribution
    pub contribution_deadline: Option<NaiveDate>,
}

/// Shareholder type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderType {
    /// 自然人股东 / Natural person
    NaturalPerson,
    /// 法人股东 / Corporate shareholder
    LegalEntity,
    /// 合伙企业 / Partnership
    Partnership,
    /// 国有资产监督管理机构 / State-owned asset supervisor
    StateAssetSupervisor,
    /// 外国投资者 / Foreign investor
    ForeignInvestor,
}

impl ShareholderType {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::NaturalPerson => "自然人股东",
            Self::LegalEntity => "法人股东",
            Self::Partnership => "合伙企业",
            Self::StateAssetSupervisor => "国有资产监督管理机构",
            Self::ForeignInvestor => "外国投资者",
        }
    }
}

/// Capital contribution method (出资方式)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionMethod {
    /// 货币 / Monetary
    Monetary,
    /// 实物 / Physical assets
    PhysicalAssets,
    /// 知识产权 / Intellectual property
    IntellectualProperty,
    /// 土地使用权 / Land use rights
    LandUseRights,
    /// 股权 / Equity
    Equity,
    /// 债权 / Debt claims
    DebtClaims,
}

impl ContributionMethod {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Monetary => "货币",
            Self::PhysicalAssets => "实物",
            Self::IntellectualProperty => "知识产权",
            Self::LandUseRights => "土地使用权",
            Self::Equity => "股权",
            Self::DebtClaims => "债权",
        }
    }

    /// Whether valuation is required
    pub fn requires_valuation(&self) -> bool {
        !matches!(self, Self::Monetary)
    }
}

/// Board of Directors (董事会)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardOfDirectors {
    /// Number of directors
    pub director_count: u32,
    /// Chairman
    pub chairman: Option<Director>,
    /// Directors
    pub directors: Vec<Director>,
    /// Independent directors (for JSC)
    pub independent_directors: Vec<Director>,
    /// Board term (years)
    pub term_years: u32,
}

impl BoardOfDirectors {
    /// Check if board composition is valid
    pub fn is_valid_for(&self, company_type: CompanyType) -> bool {
        match company_type {
            CompanyType::LimitedLiabilityCompany | CompanyType::SingleShareholderLlc => {
                // LLC: 0-13 directors, can have executive director instead
                self.director_count <= 13
            }
            CompanyType::JointStockCompany => {
                // JSC: 5-19 directors
                self.director_count >= 5 && self.director_count <= 19
            }
            _ => true,
        }
    }

    /// Check independent director requirement (JSC)
    pub fn independent_directors_sufficient(&self, is_listed: bool) -> bool {
        if !is_listed {
            return true;
        }
        // Listed companies: at least 1/3 independent directors
        let required = (self.director_count as f64 / 3.0).ceil() as usize;
        self.independent_directors.len() >= required
    }
}

/// Director information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    /// Name
    pub name: String,
    /// ID number
    pub id_number: String,
    /// Position
    pub position: DirectorPosition,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Term end date
    pub term_end_date: Option<NaiveDate>,
    /// Is independent director
    pub is_independent: bool,
    /// Is employee director
    pub is_employee_director: bool,
}

/// Director position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorPosition {
    /// 董事长 / Chairman
    Chairman,
    /// 副董事长 / Vice Chairman
    ViceChairman,
    /// 董事 / Director
    Director,
    /// 独立董事 / Independent Director
    IndependentDirector,
    /// 职工董事 / Employee Director
    EmployeeDirector,
    /// 执行董事 / Executive Director (for small LLCs)
    ExecutiveDirector,
}

impl DirectorPosition {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Chairman => "董事长",
            Self::ViceChairman => "副董事长",
            Self::Director => "董事",
            Self::IndependentDirector => "独立董事",
            Self::EmployeeDirector => "职工董事",
            Self::ExecutiveDirector => "执行董事",
        }
    }
}

/// Supervisory Board (监事会)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupervisoryBoard {
    /// Number of supervisors
    pub supervisor_count: u32,
    /// Chairman of supervisory board
    pub chairman: Option<Supervisor>,
    /// Supervisors
    pub supervisors: Vec<Supervisor>,
    /// Employee supervisors
    pub employee_supervisors: Vec<Supervisor>,
}

impl SupervisoryBoard {
    /// Check if composition is valid
    pub fn is_valid_for(&self, company_type: CompanyType) -> bool {
        match company_type {
            CompanyType::JointStockCompany => {
                // JSC: at least 3 supervisors
                self.supervisor_count >= 3
            }
            CompanyType::LimitedLiabilityCompany => {
                // LLC: can have 1+ supervisors
                self.supervisor_count >= 1
            }
            _ => true,
        }
    }

    /// Check employee supervisor ratio (at least 1/3)
    pub fn employee_ratio_sufficient(&self) -> bool {
        if self.supervisor_count == 0 {
            return true;
        }
        let required = (self.supervisor_count as f64 / 3.0).ceil() as usize;
        self.employee_supervisors.len() >= required
    }
}

/// Supervisor information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Supervisor {
    /// Name
    pub name: String,
    /// ID number
    pub id_number: String,
    /// Is employee supervisor
    pub is_employee_supervisor: bool,
    /// Appointment date
    pub appointment_date: NaiveDate,
}

/// Shareholder meeting resolution types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionType {
    /// 普通决议 / Ordinary resolution (simple majority)
    Ordinary,
    /// 特别决议 / Special resolution (2/3 majority)
    Special,
}

impl ResolutionType {
    pub fn required_majority(&self) -> f64 {
        match self {
            Self::Ordinary => 0.5,
            Self::Special => 2.0 / 3.0,
        }
    }

    pub fn name_zh(&self) -> &str {
        match self {
            Self::Ordinary => "普通决议",
            Self::Special => "特别决议",
        }
    }
}

/// Matters requiring special resolution (2/3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialResolutionMatter {
    /// 修改章程 / Amendment of articles
    AmendArticles,
    /// 增加或减少注册资本 / Capital increase/decrease
    CapitalChange,
    /// 公司合并、分立 / Merger or division
    MergerDivision,
    /// 解散公司 / Dissolution
    Dissolution,
    /// 变更公司形式 / Change of company form
    ChangeCompanyForm,
    /// 发行公司债券 / Issue company bonds
    IssueBonds,
}

impl SpecialResolutionMatter {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::AmendArticles => "修改公司章程",
            Self::CapitalChange => "增加或者减少注册资本",
            Self::MergerDivision => "公司合并、分立",
            Self::Dissolution => "解散公司",
            Self::ChangeCompanyForm => "变更公司形式",
            Self::IssueBonds => "发行公司债券",
        }
    }

    pub fn article(&self) -> u8 {
        match self {
            Self::AmendArticles
            | Self::CapitalChange
            | Self::MergerDivision
            | Self::Dissolution
            | Self::ChangeCompanyForm => 66, // LLC
            Self::IssueBonds => 66,
        }
    }
}

/// Shareholder rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderRight {
    /// 分红权 / Dividend rights
    Dividend,
    /// 优先认购权 / Preemptive rights
    Preemptive,
    /// 知情权 / Right to information
    Information,
    /// 表决权 / Voting rights
    Voting,
    /// 查阅权 / Right to inspect
    Inspection,
    /// 提议召开股东会 / Right to propose meeting
    ProposeMeeting,
    /// 派生诉讼权 / Derivative action rights
    DerivativeAction,
    /// 退出权 / Exit rights (for dissenters)
    Exit,
}

impl ShareholderRight {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::Dividend => "分红权",
            Self::Preemptive => "优先认购权",
            Self::Information => "知情权",
            Self::Voting => "表决权",
            Self::Inspection => "查阅权",
            Self::ProposeMeeting => "提议召开股东会",
            Self::DerivativeAction => "派生诉讼权",
            Self::Exit => "股东退出权",
        }
    }

    pub fn article(&self) -> u8 {
        match self {
            Self::Dividend => 34,
            Self::Preemptive => 34,
            Self::Information | Self::Inspection => 57,
            Self::Voting => 42,
            Self::ProposeMeeting => 40,
            Self::DerivativeAction => 188,
            Self::Exit => 89,
        }
    }
}

/// Equity transfer restrictions (LLC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquityTransfer {
    /// Transferor name
    pub transferor: String,
    /// Transferee name
    pub transferee: String,
    /// Transfer amount (equity value)
    pub transfer_amount: CnyAmount,
    /// Shareholding percentage being transferred
    pub transfer_pct: f64,
    /// Is internal transfer (to existing shareholder)
    pub is_internal: bool,
    /// Other shareholders notified
    pub other_shareholders_notified: bool,
    /// Notification date
    pub notification_date: Option<NaiveDate>,
    /// Consent received from shareholders
    pub consents_received: u32,
    /// Total shareholders (excluding transferor)
    pub total_other_shareholders: u32,
    /// Preemptive rights waived
    pub preemptive_rights_waived: bool,
}

impl EquityTransfer {
    /// Check if majority consent received (over 50%)
    pub fn majority_consent(&self) -> bool {
        if self.total_other_shareholders == 0 {
            return true;
        }
        (self.consents_received as f64 / self.total_other_shareholders as f64) > 0.5
    }

    /// Check if 30-day notice period satisfied
    pub fn notice_period_satisfied(&self, transfer_date: NaiveDate) -> bool {
        if let Some(notification) = self.notification_date {
            (transfer_date - notification).num_days() >= 30
        } else {
            false
        }
    }
}

/// Capital reduction method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapitalReductionMethod {
    /// 减少出资额 / Reduce contribution
    ReduceContribution,
    /// 减少出资比例 / Reduce shareholding ratio
    ReduceRatio,
    /// 取消股份 / Cancel shares
    CancelShares,
}

/// Dividend distribution record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DividendDistribution {
    /// Fiscal year
    pub fiscal_year: u32,
    /// Total distributable profit
    pub total_profit: CnyAmount,
    /// Statutory reserve contribution (10% until 50% of registered capital)
    pub statutory_reserve: CnyAmount,
    /// Optional reserve
    pub optional_reserve: CnyAmount,
    /// Dividend amount
    pub dividend_amount: CnyAmount,
    /// Distribution date
    pub distribution_date: NaiveDate,
}

impl DividendDistribution {
    /// Calculate statutory reserve (10% until reaches 50% of registered capital)
    pub fn calculate_statutory_reserve(
        profit: f64,
        current_reserve: f64,
        registered_capital: f64,
    ) -> f64 {
        let target = registered_capital * 0.5;
        if current_reserve >= target {
            0.0
        } else {
            let required = profit * 0.1;
            required.min(target - current_reserve)
        }
    }
}

/// Company dissolution reasons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DissolutionReason {
    /// 营业期限届满 / Business term expired
    TermExpired,
    /// 股东会决议 / Shareholder resolution
    ShareholderResolution,
    /// 公司合并或分立 / Merger or division
    MergerDivision,
    /// 依法被吊销或撤销 / License revoked
    LicenseRevoked,
    /// 法院判决解散 / Court-ordered dissolution
    CourtOrder,
}

impl DissolutionReason {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::TermExpired => "营业期限届满",
            Self::ShareholderResolution => "股东会决议解散",
            Self::MergerDivision => "因公司合并或者分立需要解散",
            Self::LicenseRevoked => "依法被吊销营业执照、责令关闭或者被撤销",
            Self::CourtOrder => "人民法院依法予以解散",
        }
    }

    pub fn article(&self) -> u8 {
        229 // Dissolution article in 2023 Company Law
    }
}

/// Company status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyStatus {
    /// 设立中 / In formation
    InFormation,
    /// 存续 / Active/Existing
    Active,
    /// 清算中 / In liquidation
    InLiquidation,
    /// 已注销 / Deregistered
    Deregistered,
    /// 吊销 / Revoked
    Revoked,
}

impl CompanyStatus {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::InFormation => "设立中",
            Self::Active => "存续",
            Self::InLiquidation => "清算中",
            Self::Deregistered => "已注销",
            Self::Revoked => "吊销",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_shareholders() {
        assert_eq!(CompanyType::LimitedLiabilityCompany.min_shareholders(), 2);
        assert_eq!(
            CompanyType::LimitedLiabilityCompany.max_shareholders(),
            Some(50)
        );
        assert_eq!(CompanyType::SingleShareholderLlc.min_shareholders(), 1);
        assert_eq!(CompanyType::JointStockCompany.min_shareholders(), 2);
    }

    #[test]
    fn test_board_validity() {
        let board = BoardOfDirectors {
            director_count: 5,
            chairman: None,
            directors: vec![],
            independent_directors: vec![],
            term_years: 3,
        };

        // Valid for JSC (5-19)
        assert!(board.is_valid_for(CompanyType::JointStockCompany));

        // Valid for LLC (0-13)
        assert!(board.is_valid_for(CompanyType::LimitedLiabilityCompany));
    }

    #[test]
    fn test_resolution_majority() {
        assert_eq!(ResolutionType::Ordinary.required_majority(), 0.5);
        assert!((ResolutionType::Special.required_majority() - 0.6666666).abs() < 0.001);
    }

    #[test]
    fn test_equity_transfer_consent() {
        let transfer = EquityTransfer {
            transferor: "张三".to_string(),
            transferee: "李四".to_string(),
            transfer_amount: CnyAmount::from_yuan(100000.0),
            transfer_pct: 20.0,
            is_internal: false,
            other_shareholders_notified: true,
            notification_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
            consents_received: 3,
            total_other_shareholders: 4,
            preemptive_rights_waived: true,
        };

        assert!(transfer.majority_consent());
    }

    #[test]
    fn test_statutory_reserve() {
        // Not yet at 50% target
        let reserve = DividendDistribution::calculate_statutory_reserve(
            1000000.0, // profit
            100000.0,  // current reserve
            1000000.0, // registered capital
        );
        assert_eq!(reserve, 100000.0); // 10% of profit

        // Already at 50% target
        let reserve =
            DividendDistribution::calculate_statutory_reserve(1000000.0, 500000.0, 1000000.0);
        assert_eq!(reserve, 0.0);
    }

    #[test]
    fn test_supervisory_board_employee_ratio() {
        let board = SupervisoryBoard {
            supervisor_count: 3,
            chairman: None,
            supervisors: vec![],
            employee_supervisors: vec![Supervisor {
                name: "员工代表".to_string(),
                id_number: "123".to_string(),
                is_employee_supervisor: true,
                appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            }],
        };

        // 1 out of 3 is exactly 1/3
        assert!(board.employee_ratio_sufficient());
    }

    #[test]
    fn test_contribution_method() {
        assert!(!ContributionMethod::Monetary.requires_valuation());
        assert!(ContributionMethod::IntellectualProperty.requires_valuation());
        assert!(ContributionMethod::LandUseRights.requires_valuation());
    }
}
