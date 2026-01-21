//! Cybersecurity Law Types
//!
//! # 网络安全法数据类型

#![allow(missing_docs)]

use crate::i18n::BilingualText;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Network operator category (Article 21)
///
/// # 网络运营者类别
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkOperatorCategory {
    /// 一般网络运营者 / General Network Operator
    General,
    /// 关键信息基础设施运营者 / CII Operator
    CriticalInformationInfrastructure,
    /// 网络产品和服务提供者 / Network Product/Service Provider
    ProductServiceProvider,
    /// 网络平台运营者 / Platform Operator
    PlatformOperator,
}

impl NetworkOperatorCategory {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::General => "一般网络运营者",
            Self::CriticalInformationInfrastructure => "关键信息基础设施运营者",
            Self::ProductServiceProvider => "网络产品和服务提供者",
            Self::PlatformOperator => "网络平台运营者",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            Self::General => "General Network Operator",
            Self::CriticalInformationInfrastructure => "CII Operator",
            Self::ProductServiceProvider => "Network Product/Service Provider",
            Self::PlatformOperator => "Platform Operator",
        }
    }

    pub fn is_cii(&self) -> bool {
        matches!(self, Self::CriticalInformationInfrastructure)
    }
}

/// Critical Information Infrastructure sectors (Article 31)
///
/// # 关键信息基础设施行业
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiiSector {
    /// 公共通信和信息服务 / Public Communication and Information Services
    PublicCommunication,
    /// 能源 / Energy
    Energy,
    /// 交通 / Transportation
    Transportation,
    /// 水利 / Water Conservancy
    WaterConservancy,
    /// 金融 / Finance
    Finance,
    /// 公共服务 / Public Services
    PublicServices,
    /// 电子政务 / E-Government
    EGovernment,
    /// 国防科技工业 / Defense and Technology Industry
    DefenseTech,
    /// 其他重要行业 / Other Important Sectors
    Other(String),
}

impl CiiSector {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::PublicCommunication => "公共通信和信息服务",
            Self::Energy => "能源",
            Self::Transportation => "交通",
            Self::WaterConservancy => "水利",
            Self::Finance => "金融",
            Self::PublicServices => "公共服务",
            Self::EGovernment => "电子政务",
            Self::DefenseTech => "国防科技工业",
            Self::Other(_) => "其他重要行业",
        }
    }

    pub fn regulator(&self) -> &str {
        match self {
            Self::PublicCommunication => "工业和信息化部",
            Self::Energy => "国家能源局",
            Self::Transportation => "交通运输部",
            Self::WaterConservancy => "水利部",
            Self::Finance => "中国人民银行/银保监会/证监会",
            Self::PublicServices => "相关主管部门",
            Self::EGovernment => "国家互联网信息办公室",
            Self::DefenseTech => "国防科工局",
            Self::Other(_) => "相关主管部门",
        }
    }
}

/// Network operator entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkOperator {
    /// Operator name
    pub name: BilingualText,
    /// Unified Social Credit Code
    pub uscc: Option<String>,
    /// Category
    pub category: NetworkOperatorCategory,
    /// CII sector (if applicable)
    pub cii_sector: Option<CiiSector>,
    /// MLPS protection level
    pub mlps_level: Option<MlpsLevel>,
    /// Contact for security matters
    pub security_contact: Option<SecurityContact>,
    /// Last security assessment date
    pub last_assessment: Option<NaiveDate>,
}

/// MLPS protection level (1-5)
///
/// # 等级保护级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MlpsLevel {
    /// 第一级 / Level 1: User-managed protection
    Level1,
    /// 第二级 / Level 2: System audit protection
    Level2,
    /// 第三级 / Level 3: Security label protection
    Level3,
    /// 第四级 / Level 4: Structural protection
    Level4,
    /// 第五级 / Level 5: Access verification protection
    Level5,
}

impl MlpsLevel {
    pub fn level_number(&self) -> u8 {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
            Self::Level5 => 5,
        }
    }

    pub fn name_zh(&self) -> &str {
        match self {
            Self::Level1 => "第一级（用户自主保护级）",
            Self::Level2 => "第二级（系统审计保护级）",
            Self::Level3 => "第三级（安全标记保护级）",
            Self::Level4 => "第四级（结构化保护级）",
            Self::Level5 => "第五级（访问验证保护级）",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            Self::Level1 => "Level 1: User-managed Protection",
            Self::Level2 => "Level 2: System Audit Protection",
            Self::Level3 => "Level 3: Security Label Protection",
            Self::Level4 => "Level 4: Structural Protection",
            Self::Level5 => "Level 5: Access Verification Protection",
        }
    }

    pub fn requires_third_party_assessment(&self) -> bool {
        matches!(self, Self::Level3 | Self::Level4 | Self::Level5)
    }

    pub fn requires_annual_assessment(&self) -> bool {
        matches!(self, Self::Level3 | Self::Level4 | Self::Level5)
    }
}

/// Security contact information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SecurityContact {
    pub name: String,
    pub title: String,
    pub phone: String,
    pub email: String,
}

/// Network security incident severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    /// 特别重大 / Particularly Major
    ParticularlyMajor,
    /// 重大 / Major
    Major,
    /// 较大 / Significant
    Significant,
    /// 一般 / General
    General,
}

impl IncidentSeverity {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::ParticularlyMajor => "特别重大网络安全事件",
            Self::Major => "重大网络安全事件",
            Self::Significant => "较大网络安全事件",
            Self::General => "一般网络安全事件",
        }
    }

    pub fn reporting_deadline_hours(&self) -> u32 {
        match self {
            Self::ParticularlyMajor => 1,
            Self::Major => 2,
            Self::Significant => 8,
            Self::General => 24,
        }
    }

    pub fn reporting_authority(&self) -> &str {
        match self {
            Self::ParticularlyMajor | Self::Major => "国家网信部门",
            Self::Significant => "省级网信部门",
            Self::General => "市级网信部门",
        }
    }
}

/// Network security incident
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub id: String,
    pub severity: IncidentSeverity,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    pub reported_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
    pub affected_systems: Vec<String>,
    pub affected_users: Option<u64>,
    pub data_breach: bool,
    pub remediation_status: RemediationStatus,
}

/// Remediation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationStatus {
    /// 处置中 / In Progress
    InProgress,
    /// 已控制 / Contained
    Contained,
    /// 已恢复 / Recovered
    Recovered,
    /// 已完成 / Completed
    Completed,
}

/// Cybersecurity review trigger (Article 35)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewTrigger {
    /// CII采购网络产品和服务 / CII Procurement
    CiiProcurement,
    /// 数据处理活动影响国家安全 / Data Processing Affecting National Security
    DataProcessingNationalSecurity,
    /// 境外上市 / Overseas Listing
    OverseasListing,
    /// 掌握超过100万用户个人信息的网络平台运营者境外上市
    OverseasListingLargeUserBase,
}

impl ReviewTrigger {
    pub fn name_zh(&self) -> &str {
        match self {
            Self::CiiProcurement => "关键信息基础设施运营者采购网络产品和服务",
            Self::DataProcessingNationalSecurity => "数据处理活动影响或可能影响国家安全",
            Self::OverseasListing => "网络平台运营者境外上市",
            Self::OverseasListingLargeUserBase => {
                "掌握超过100万用户个人信息的网络平台运营者境外上市"
            }
        }
    }

    pub fn mandatory(&self) -> bool {
        true // All triggers require mandatory review
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlps_level_ordering() {
        assert!(MlpsLevel::Level1 < MlpsLevel::Level2);
        assert!(MlpsLevel::Level3 < MlpsLevel::Level5);
    }

    #[test]
    fn test_mlps_third_party_assessment() {
        assert!(!MlpsLevel::Level1.requires_third_party_assessment());
        assert!(!MlpsLevel::Level2.requires_third_party_assessment());
        assert!(MlpsLevel::Level3.requires_third_party_assessment());
        assert!(MlpsLevel::Level4.requires_third_party_assessment());
        assert!(MlpsLevel::Level5.requires_third_party_assessment());
    }

    #[test]
    fn test_incident_severity_deadline() {
        assert_eq!(
            IncidentSeverity::ParticularlyMajor.reporting_deadline_hours(),
            1
        );
        assert_eq!(IncidentSeverity::General.reporting_deadline_hours(), 24);
    }

    #[test]
    fn test_cii_sector_regulator() {
        let finance = CiiSector::Finance;
        assert!(finance.regulator().contains("人民银行"));
    }
}
