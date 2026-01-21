//! Multi-Level Protection Scheme (MLPS 2.0)
//!
//! # 网络安全等级保护制度 / Cybersecurity Multi-Level Protection Scheme
//!
//! Implements GB/T 22239-2019 and related standards.
//!
//! ## MLPS 2.0 Framework
//!
//! - Level 1: User self-protection
//! - Level 2: System audit protection  
//! - Level 3: Security label protection (most common for enterprises)
//! - Level 4: Structured protection
//! - Level 5: Access verification protection (national security)

#![allow(missing_docs)]

use super::types::MlpsLevel;
use crate::i18n::BilingualText;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// MLPS assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MlpsAssessment {
    /// System name
    pub system_name: String,
    /// Determined protection level
    pub level: MlpsLevel,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Assessor (third-party for Level 3+)
    pub assessor: Option<ThirdPartyAssessor>,
    /// Filing status
    pub filing_status: FilingStatus,
    /// Filing number
    pub filing_number: Option<String>,
    /// Security controls assessment
    pub controls_assessment: ControlsAssessment,
    /// Validity period (years)
    pub validity_years: u32,
    /// Next assessment due
    pub next_assessment_due: NaiveDate,
}

impl MlpsAssessment {
    /// Check if assessment is currently valid
    pub fn is_valid(&self, as_of: NaiveDate) -> bool {
        as_of <= self.next_assessment_due && self.filing_status == FilingStatus::Filed
    }

    /// Calculate compliance score
    pub fn compliance_score(&self) -> f64 {
        self.controls_assessment.overall_score()
    }
}

/// Third-party assessor (required for Level 3+)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThirdPartyAssessor {
    /// Assessor organization name
    pub name: String,
    /// Qualification certificate number
    pub certificate_number: String,
    /// Qualification level
    pub qualification_level: AssessorQualification,
    /// Valid until
    pub valid_until: NaiveDate,
}

/// Assessor qualification level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessorQualification {
    /// 一级 / Level 1 (can assess up to MLPS Level 3)
    Level1,
    /// 二级 / Level 2 (can assess up to MLPS Level 4)
    Level2,
    /// 三级 / Level 3 (can assess all levels including Level 5)
    Level3,
}

impl AssessorQualification {
    pub fn can_assess(&self, mlps_level: MlpsLevel) -> bool {
        match self {
            Self::Level1 => mlps_level.level_number() <= 3,
            Self::Level2 => mlps_level.level_number() <= 4,
            Self::Level3 => true,
        }
    }
}

/// Filing status with public security bureau
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilingStatus {
    /// 未备案 / Not Filed
    NotFiled,
    /// 备案中 / Filing in Progress
    FilingInProgress,
    /// 已备案 / Filed
    Filed,
    /// 已撤销 / Revoked
    Revoked,
}

/// Security controls assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlsAssessment {
    /// Physical security controls
    pub physical_security: ControlDomain,
    /// Network security controls
    pub network_security: ControlDomain,
    /// Host security controls
    pub host_security: ControlDomain,
    /// Application security controls
    pub application_security: ControlDomain,
    /// Data security controls
    pub data_security: ControlDomain,
    /// Security management center
    pub management_center: ControlDomain,
    /// Security management system
    pub management_system: ControlDomain,
    /// Security management organization
    pub management_organization: ControlDomain,
    /// Security management personnel
    pub management_personnel: ControlDomain,
    /// Security construction management
    pub construction_management: ControlDomain,
    /// Security operation management
    pub operation_management: ControlDomain,
}

impl ControlsAssessment {
    /// Calculate overall compliance score
    pub fn overall_score(&self) -> f64 {
        let domains = [
            &self.physical_security,
            &self.network_security,
            &self.host_security,
            &self.application_security,
            &self.data_security,
            &self.management_center,
            &self.management_system,
            &self.management_organization,
            &self.management_personnel,
            &self.construction_management,
            &self.operation_management,
        ];

        let total: f64 = domains.iter().map(|d| d.score).sum();
        total / domains.len() as f64
    }

    /// Check if meets minimum threshold for level
    pub fn meets_threshold(&self, level: MlpsLevel) -> bool {
        let threshold = match level {
            MlpsLevel::Level1 => 60.0,
            MlpsLevel::Level2 => 70.0,
            MlpsLevel::Level3 => 75.0,
            MlpsLevel::Level4 => 80.0,
            MlpsLevel::Level5 => 90.0,
        };
        self.overall_score() >= threshold
    }

    /// Get non-compliant domains
    pub fn non_compliant_domains(&self, threshold: f64) -> Vec<BilingualText> {
        let mut result = Vec::new();
        let domains = [
            (&self.physical_security, "物理安全", "Physical Security"),
            (&self.network_security, "网络安全", "Network Security"),
            (&self.host_security, "主机安全", "Host Security"),
            (
                &self.application_security,
                "应用安全",
                "Application Security",
            ),
            (&self.data_security, "数据安全", "Data Security"),
            (
                &self.management_center,
                "安全管理中心",
                "Security Management Center",
            ),
            (
                &self.management_system,
                "安全管理制度",
                "Security Management System",
            ),
            (
                &self.management_organization,
                "安全管理机构",
                "Security Management Organization",
            ),
            (
                &self.management_personnel,
                "安全管理人员",
                "Security Management Personnel",
            ),
            (
                &self.construction_management,
                "安全建设管理",
                "Security Construction Management",
            ),
            (
                &self.operation_management,
                "安全运维管理",
                "Security Operation Management",
            ),
        ];

        for (domain, zh, en) in domains {
            if domain.score < threshold {
                result.push(BilingualText::new(zh, en));
            }
        }

        result
    }
}

/// Individual control domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlDomain {
    /// Domain score (0-100)
    pub score: f64,
    /// Total controls in domain
    pub total_controls: u32,
    /// Implemented controls
    pub implemented_controls: u32,
    /// Partially implemented controls
    pub partial_controls: u32,
    /// Not implemented controls
    pub not_implemented_controls: u32,
    /// Findings
    pub findings: Vec<Finding>,
}

impl Default for ControlDomain {
    fn default() -> Self {
        Self {
            score: 0.0,
            total_controls: 0,
            implemented_controls: 0,
            partial_controls: 0,
            not_implemented_controls: 0,
            findings: Vec::new(),
        }
    }
}

/// Assessment finding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// Finding ID
    pub id: String,
    /// Severity
    pub severity: FindingSeverity,
    /// Description
    pub description: BilingualText,
    /// Recommendation
    pub recommendation: BilingualText,
    /// Control reference
    pub control_ref: String,
}

/// Finding severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingSeverity {
    /// 高 / High
    High,
    /// 中 / Medium
    Medium,
    /// 低 / Low
    Low,
}

/// MLPS level determination criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelDeterminationFactors {
    /// Business information security level
    pub business_info_level: u8,
    /// System service security level
    pub system_service_level: u8,
    /// Information system type
    pub system_type: SystemType,
    /// User count
    pub user_count: u64,
    /// Data volume
    pub data_volume: DataVolume,
    /// Impact on society
    pub social_impact: SocialImpact,
}

impl LevelDeterminationFactors {
    /// Determine recommended MLPS level
    pub fn determine_level(&self) -> MlpsLevel {
        let max_level = self.business_info_level.max(self.system_service_level);

        // Adjust based on user count and social impact
        let adjusted = if self.user_count >= 10_000_000
            || matches!(
                self.social_impact,
                SocialImpact::Significant | SocialImpact::Major
            ) {
            max_level.saturating_add(1)
        } else {
            max_level
        };

        match adjusted.min(5) {
            1 => MlpsLevel::Level1,
            2 => MlpsLevel::Level2,
            3 => MlpsLevel::Level3,
            4 => MlpsLevel::Level4,
            _ => MlpsLevel::Level5,
        }
    }
}

/// Information system type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemType {
    /// 基础设施 / Infrastructure
    Infrastructure,
    /// 业务应用 / Business Application
    BusinessApplication,
    /// 数据平台 / Data Platform
    DataPlatform,
    /// 工业控制 / Industrial Control
    IndustrialControl,
    /// 物联网 / IoT
    IoT,
    /// 云计算 / Cloud Computing
    CloudComputing,
    /// 移动互联网 / Mobile Internet
    MobileInternet,
    /// 大数据 / Big Data
    BigData,
}

/// Data volume category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataVolume {
    /// 小 / Small (< 1TB)
    Small,
    /// 中 / Medium (1-100TB)
    Medium,
    /// 大 / Large (100TB-1PB)
    Large,
    /// 特大 / Very Large (> 1PB)
    VeryLarge,
}

/// Social impact assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialImpact {
    /// 一般 / General
    General,
    /// 较大 / Significant
    Significant,
    /// 重大 / Major
    Major,
    /// 特别重大 / Particularly Major
    ParticularlyMajor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessor_qualification() {
        let level1_assessor = AssessorQualification::Level1;
        assert!(level1_assessor.can_assess(MlpsLevel::Level3));
        assert!(!level1_assessor.can_assess(MlpsLevel::Level4));

        let level3_assessor = AssessorQualification::Level3;
        assert!(level3_assessor.can_assess(MlpsLevel::Level5));
    }

    #[test]
    fn test_level_determination() {
        let factors = LevelDeterminationFactors {
            business_info_level: 2,
            system_service_level: 3,
            system_type: SystemType::BusinessApplication,
            user_count: 100_000,
            data_volume: DataVolume::Medium,
            social_impact: SocialImpact::General,
        };

        assert_eq!(factors.determine_level(), MlpsLevel::Level3);
    }

    #[test]
    fn test_level_determination_large_users() {
        let factors = LevelDeterminationFactors {
            business_info_level: 2,
            system_service_level: 2,
            system_type: SystemType::DataPlatform,
            user_count: 50_000_000,
            data_volume: DataVolume::VeryLarge,
            social_impact: SocialImpact::Major,
        };

        // Large user count should bump level up
        assert_eq!(factors.determine_level(), MlpsLevel::Level3);
    }

    #[test]
    fn test_controls_assessment_score() {
        let domain = ControlDomain {
            score: 80.0,
            total_controls: 20,
            implemented_controls: 16,
            partial_controls: 2,
            not_implemented_controls: 2,
            findings: vec![],
        };

        let assessment = ControlsAssessment {
            physical_security: domain.clone(),
            network_security: domain.clone(),
            host_security: domain.clone(),
            application_security: domain.clone(),
            data_security: domain.clone(),
            management_center: domain.clone(),
            management_system: domain.clone(),
            management_organization: domain.clone(),
            management_personnel: domain.clone(),
            construction_management: domain.clone(),
            operation_management: domain,
        };

        assert!((assessment.overall_score() - 80.0).abs() < 0.001);
        assert!(assessment.meets_threshold(MlpsLevel::Level3));
    }
}
