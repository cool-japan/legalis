//! Chinese Name Handling
//!
//! Handles Chinese personal and company names according to legal conventions.
//!
//! # 姓名处理 / Name Handling
//!
//! Chinese names: Family name (姓) + Given name (名)
//! Order: Family name first, then given name (opposite of Western convention)

use crate::i18n::BilingualText;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Chinese personal name
///
/// # 中国人名
///
/// Format: 姓 + 名 (Family name + Given name)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChineseName {
    /// Family name (姓)
    pub family_name: String,
    /// Given name (名)
    pub given_name: String,
    /// Romanized/English name (optional)
    pub romanized: Option<String>,
}

impl ChineseName {
    /// Create a new Chinese name
    pub fn new(family_name: impl Into<String>, given_name: impl Into<String>) -> Self {
        Self {
            family_name: family_name.into(),
            given_name: given_name.into(),
            romanized: None,
        }
    }

    /// Create with romanized version
    pub fn with_romanized(
        family_name: impl Into<String>,
        given_name: impl Into<String>,
        romanized: impl Into<String>,
    ) -> Self {
        Self {
            family_name: family_name.into(),
            given_name: given_name.into(),
            romanized: Some(romanized.into()),
        }
    }

    /// Get full name in Chinese order (family + given)
    pub fn full_name(&self) -> String {
        format!("{}{}", self.family_name, self.given_name)
    }

    /// Get full name in Western order (given + family)
    pub fn western_order(&self) -> String {
        format!("{} {}", self.given_name, self.family_name)
    }

    /// Get formal display (with romanized if available)
    pub fn formal_display(&self) -> String {
        if let Some(ref roman) = self.romanized {
            format!("{} ({})", self.full_name(), roman)
        } else {
            self.full_name()
        }
    }
}

impl fmt::Display for ChineseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Chinese company/organization name
///
/// # 公司名称 / Company Name
///
/// Format: \[区域\] + 字号 + \[行业\] + 组织形式
/// Example: 北京字节跳动科技有限公司
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanyName {
    /// Administrative region (省/市)
    pub region: Option<String>,
    /// Trade name / brand name (字号)
    pub trade_name: String,
    /// Industry description (行业)
    pub industry: Option<String>,
    /// Organization form (组织形式)
    pub organization_form: OrganizationForm,
    /// English name (optional)
    pub english_name: Option<String>,
}

impl CompanyName {
    /// Create a new company name
    pub fn new(trade_name: impl Into<String>, organization_form: OrganizationForm) -> Self {
        Self {
            region: None,
            trade_name: trade_name.into(),
            industry: None,
            organization_form,
            english_name: None,
        }
    }

    /// Add region
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Add industry
    pub fn with_industry(mut self, industry: impl Into<String>) -> Self {
        self.industry = Some(industry.into());
        self
    }

    /// Add English name
    pub fn with_english(mut self, english: impl Into<String>) -> Self {
        self.english_name = Some(english.into());
        self
    }

    /// Get full formal name
    pub fn full_name(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref region) = self.region {
            parts.push(region.as_str());
        }

        parts.push(&self.trade_name);

        if let Some(ref industry) = self.industry {
            parts.push(industry.as_str());
        }

        parts.push(self.organization_form.name_zh());

        parts.join("")
    }

    /// Get bilingual name
    pub fn bilingual(&self) -> BilingualText {
        let en = self
            .english_name
            .clone()
            .unwrap_or_else(|| self.full_name());
        BilingualText::new(self.full_name(), en)
    }
}

impl fmt::Display for CompanyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Organization forms in Chinese law
///
/// # 组织形式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationForm {
    /// 有限责任公司 / Limited Liability Company
    LimitedLiabilityCompany,
    /// 股份有限公司 / Joint Stock Company
    JointStockCompany,
    /// 有限合伙企业 / Limited Partnership
    LimitedPartnership,
    /// 普通合伙企业 / General Partnership
    GeneralPartnership,
    /// 个人独资企业 / Sole Proprietorship
    SoleProprietorship,
    /// 分公司 / Branch
    Branch,
    /// 子公司 / Subsidiary
    Subsidiary,
    /// 代表处 / Representative Office
    RepresentativeOffice,
    /// 中外合资经营企业 / Sino-Foreign Equity Joint Venture
    EquityJointVenture,
    /// 中外合作经营企业 / Sino-Foreign Cooperative Joint Venture
    CooperativeJointVenture,
    /// 外商独资企业 / Wholly Foreign-Owned Enterprise (WFOE)
    WhollyForeignOwned,
}

impl OrganizationForm {
    /// Get name in Chinese
    pub fn name_zh(&self) -> &'static str {
        match self {
            Self::LimitedLiabilityCompany => "有限责任公司",
            Self::JointStockCompany => "股份有限公司",
            Self::LimitedPartnership => "有限合伙企业",
            Self::GeneralPartnership => "普通合伙企业",
            Self::SoleProprietorship => "个人独资企业",
            Self::Branch => "分公司",
            Self::Subsidiary => "子公司",
            Self::RepresentativeOffice => "代表处",
            Self::EquityJointVenture => "中外合资经营企业",
            Self::CooperativeJointVenture => "中外合作经营企业",
            Self::WhollyForeignOwned => "外商独资企业",
        }
    }

    /// Get name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::LimitedLiabilityCompany => "Limited Liability Company",
            Self::JointStockCompany => "Joint Stock Company Limited",
            Self::LimitedPartnership => "Limited Partnership",
            Self::GeneralPartnership => "General Partnership",
            Self::SoleProprietorship => "Sole Proprietorship",
            Self::Branch => "Branch",
            Self::Subsidiary => "Subsidiary",
            Self::RepresentativeOffice => "Representative Office",
            Self::EquityJointVenture => "Equity Joint Venture",
            Self::CooperativeJointVenture => "Cooperative Joint Venture",
            Self::WhollyForeignOwned => "Wholly Foreign-Owned Enterprise",
        }
    }

    /// Get abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::LimitedLiabilityCompany => "有限公司",
            Self::JointStockCompany => "股份公司",
            Self::LimitedPartnership => "有限合伙",
            Self::GeneralPartnership => "普通合伙",
            Self::SoleProprietorship => "个独",
            Self::Branch => "分公司",
            Self::Subsidiary => "子公司",
            Self::RepresentativeOffice => "代表处",
            Self::EquityJointVenture => "合资",
            Self::CooperativeJointVenture => "合作",
            Self::WhollyForeignOwned => "外资",
        }
    }

    /// Check if foreign investment vehicle
    pub fn is_foreign_invested(&self) -> bool {
        matches!(
            self,
            Self::EquityJointVenture | Self::CooperativeJointVenture | Self::WhollyForeignOwned
        )
    }

    /// Check if has legal personality
    pub fn has_legal_personality(&self) -> bool {
        !matches!(self, Self::Branch | Self::RepresentativeOffice)
    }
}

impl fmt::Display for OrganizationForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_zh())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_name() {
        let name = ChineseName::new("张", "三");
        assert_eq!(name.full_name(), "张三");
        assert_eq!(name.western_order(), "三 张");
    }

    #[test]
    fn test_chinese_name_with_romanized() {
        let name = ChineseName::with_romanized("李", "明", "Li Ming");
        assert_eq!(name.formal_display(), "李明 (Li Ming)");
    }

    #[test]
    fn test_company_name() {
        let company = CompanyName::new("字节跳动", OrganizationForm::LimitedLiabilityCompany)
            .with_region("北京")
            .with_industry("科技");

        assert_eq!(company.full_name(), "北京字节跳动科技有限责任公司");
    }

    #[test]
    fn test_organization_form() {
        let form = OrganizationForm::WhollyForeignOwned;
        assert_eq!(form.name_zh(), "外商独资企业");
        assert_eq!(form.name_en(), "Wholly Foreign-Owned Enterprise");
        assert!(form.is_foreign_invested());
    }

    #[test]
    fn test_legal_personality() {
        assert!(OrganizationForm::LimitedLiabilityCompany.has_legal_personality());
        assert!(!OrganizationForm::Branch.has_legal_personality());
        assert!(!OrganizationForm::RepresentativeOffice.has_legal_personality());
    }
}
