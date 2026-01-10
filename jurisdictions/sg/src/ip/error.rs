//! Intellectual Property Laws - Error Types
//!
//! This module defines error types for Singapore IP law violations with trilingual messages:
//! - English (legal and business language)
//! - Chinese/华语 (Chinese community, 74% of population)
//! - Malay/Bahasa Melayu (national language, 13% of population)
//!
//! Covers four main IP statutes:
//! - Patents Act (Cap. 221)
//! - Trade Marks Act (Cap. 332)
//! - Copyright Act 2021
//! - Registered Designs Act (Cap. 266)

use thiserror::Error;

/// Result type for IP operations
pub type Result<T> = std::result::Result<T, IpError>;

/// Intellectual Property error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum IpError {
    // ===== Patents Act (Cap. 221) =====
    /// Patent lacks novelty (prior art exists)
    #[error(
        "Patent lacks novelty - prior art exists: {description} (Patents Act s. 14(2))\n\
         专利缺乏新颖性 - 存在现有技术: {description} (专利法第14(2)条)\n\
         Paten tidak novel - seni terdahulu wujud: {description} (Akta Paten s. 14(2))"
    )]
    LacksNovelty { description: String },

    /// Patent lacks inventive step
    #[error(
        "Patent lacks inventive step - obvious to person skilled in the art (Patents Act s. 15)\n\
         专利缺乏创造性 - 对本领域技术人员显而易见 (专利法第15条)\n\
         Paten tidak mempunyai langkah inventif - jelas kepada orang mahir (Akta Paten s. 15)"
    )]
    LacksInventiveStep,

    /// Not capable of industrial application
    #[error(
        "Invention not capable of industrial application (Patents Act s. 16)\n\
         发明不能工业应用 (专利法第16条)\n\
         Ciptaan tidak boleh digunakan secara industri (Akta Paten s. 16)"
    )]
    NotIndustriallyApplicable,

    /// Patent term expired
    #[error(
        "Patent term expired (filed {years_ago} years ago, max 20 years) (Patents Act s. 36)\n\
         专利期限已过期 ({years_ago}年前申请, 最长20年) (专利法第36条)\n\
         Tempoh paten tamat ({years_ago} tahun lalu, maksimum 20 tahun) (Akta Paten s. 36)"
    )]
    PatentExpired { years_ago: u32 },

    /// Patent infringement detected
    #[error(
        "Patent infringement: {description} (Patents Act s. 66)\n\
         专利侵权: {description} (专利法第66条)\n\
         Pelanggaran paten: {description} (Akta Paten s. 66)"
    )]
    PatentInfringement { description: String },

    // ===== Trade Marks Act (Cap. 332) =====
    /// Trademark not distinctive
    #[error(
        "Trademark lacks distinctiveness for Class {class} (Trade Marks Act s. 7(1)(b))\n\
         商标在第{class}类缺乏显著性 (商标法第7(1)(b)条)\n\
         Cap dagangan tidak tersendiri untuk Kelas {class} (Akta Cap Dagangan s. 7(1)(b))"
    )]
    NotDistinctive { class: u8 },

    /// Trademark is descriptive
    #[error(
        "Trademark is purely descriptive: {description} (Trade Marks Act s. 7(1)(c))\n\
         商标纯粹描述性: {description} (商标法第7(1)(c)条)\n\
         Cap dagangan adalah deskriptif semata-mata: {description} (Akta Cap Dagangan s. 7(1)(c))"
    )]
    PurelyDescriptive { description: String },

    /// Trademark too similar to existing mark
    #[error(
        "Trademark similar to '{existing}' in Class {class}, similarity score: {score}% (Trade Marks Act s. 8(2))\n\
         商标与第{class}类的 '{existing}' 相似, 相似度: {score}% (商标法第8(2)条)\n\
         Cap dagangan serupa dengan '{existing}' dalam Kelas {class}, skor persamaan: {score}% (Akta Cap Dagangan s. 8(2))"
    )]
    TrademarkTooSimilar {
        existing: String,
        class: u8,
        score: u8,
    },

    /// Trademark is deceptive
    #[error(
        "Trademark is deceptive or likely to cause confusion: {reason} (Trade Marks Act s. 7(4))\n\
         商标具有欺骗性或可能引起混淆: {reason} (商标法第7(4)条)\n\
         Cap dagangan menipu atau mungkin mengelirukan: {reason} (Akta Cap Dagangan s. 7(4))"
    )]
    Deceptive { reason: String },

    /// Trademark contains prohibited elements
    #[error(
        "Trademark contains prohibited element: {element} (Trade Marks Act s. 7(5))\n\
         商标包含禁用元素: {element} (商标法第7(5)条)\n\
         Cap dagangan mengandungi elemen terlarang: {element} (Akta Cap Dagangan s. 7(5))"
    )]
    ProhibitedElement { element: String },

    /// Trademark infringement
    #[error(
        "Trademark infringement: unauthorized use of '{mark}' in Class {class} (Trade Marks Act s. 27)\n\
         商标侵权: 未经授权使用第{class}类的 '{mark}' (商标法第27条)\n\
         Pelanggaran cap dagangan: penggunaan tanpa kebenaran '{mark}' dalam Kelas {class} (Akta Cap Dagangan s. 27)"
    )]
    TrademarkInfringement { mark: String, class: u8 },

    /// Trademark registration expired
    #[error(
        "Trademark registration expired (registered {years_ago} years ago, 10-year terms) (Trade Marks Act s. 18)\n\
         商标注册已过期 ({years_ago}年前注册, 10年期限) (商标法第18条)\n\
         Pendaftaran cap dagangan tamat ({years_ago} tahun lalu, tempoh 10 tahun) (Akta Cap Dagangan s. 18)"
    )]
    TrademarkExpired { years_ago: u32 },

    // ===== Copyright Act 2021 =====
    /// Copyright term expired
    #[error(
        "Copyright term expired ({years} years since {event}) (Copyright Act s. 28)\n\
         版权期限已过期 ({event}后{years}年) (版权法第28条)\n\
         Tempoh hak cipta tamat ({years} tahun sejak {event}) (Akta Hak Cipta s. 28)"
    )]
    CopyrightExpired { years: u32, event: String },

    /// Copyright infringement detected
    #[error(
        "Copyright infringement: {act} without authorization (Copyright Act s. 103)\n\
         版权侵权: 未经授权{act} (版权法第103条)\n\
         Pelanggaran hak cipta: {act} tanpa kebenaran (Akta Hak Cipta s. 103)"
    )]
    CopyrightInfringement { act: String },

    /// Work not eligible for copyright
    #[error(
        "Work not eligible for copyright protection: {reason} (Copyright Act s. 27)\n\
         作品不符合版权保护: {reason} (版权法第27条)\n\
         Karya tidak layak untuk perlindungan hak cipta: {reason} (Akta Hak Cipta s. 27)"
    )]
    NotCopyrightable { reason: String },

    /// Fair dealing defense does not apply
    #[error(
        "Fair dealing defense does not apply: {reason} (Copyright Act s. 35-42)\n\
         合理使用抗辩不适用: {reason} (版权法第35-42条)\n\
         Pembelaan urusan saksama tidak terpakai: {reason} (Akta Hak Cipta s. 35-42)"
    )]
    FairDealingNotApplicable { reason: String },

    // ===== Registered Designs Act (Cap. 266) =====
    /// Design lacks novelty
    #[error(
        "Design lacks novelty - similar design exists: {description} (Registered Designs Act s. 5(2))\n\
         设计缺乏新颖性 - 存在类似设计: {description} (注册外观设计法第5(2)条)\n\
         Reka bentuk tidak novel - reka bentuk serupa wujud: {description} (Akta Reka Bentuk Berdaftar s. 5(2))"
    )]
    DesignLacksNovelty { description: String },

    /// Design lacks individual character
    #[error(
        "Design lacks individual character (Registered Designs Act s. 5(3))\n\
         设计缺乏独特性 (注册外观设计法第5(3)条)\n\
         Reka bentuk tidak mempunyai watak tersendiri (Akta Reka Bentuk Berdaftar s. 5(3))"
    )]
    LacksIndividualCharacter,

    /// Design is purely functional
    #[error(
        "Design is purely functional - not registrable (Registered Designs Act s. 6)\n\
         设计纯粹功能性 - 不可注册 (注册外观设计法第6条)\n\
         Reka bentuk adalah fungsional semata-mata - tidak boleh didaftar (Akta Reka Bentuk Berdaftar s. 6)"
    )]
    PurelyFunctional,

    /// Design registration expired
    #[error(
        "Design registration expired ({years} years since registration, max 15 years) (Registered Designs Act s. 21)\n\
         设计注册已过期 (注册后{years}年, 最长15年) (注册外观设计法第21条)\n\
         Pendaftaran reka bentuk tamat ({years} tahun sejak pendaftaran, maksimum 15 tahun) (Akta Reka Bentuk Berdaftar s. 21)"
    )]
    DesignExpired { years: u32 },

    /// Design infringement
    #[error(
        "Design infringement: unauthorized use of registered design (Registered Designs Act s. 30)\n\
         设计侵权: 未经授权使用注册设计 (注册外观设计法第30条)\n\
         Pelanggaran reka bentuk: penggunaan tanpa kebenaran reka bentuk berdaftar (Akta Reka Bentuk Berdaftar s. 30)"
    )]
    DesignInfringement,

    // ===== General IP Errors =====
    /// Invalid Nice Classification class (1-45)
    #[error(
        "Invalid trademark class: {class} (must be 1-45 per Nice Classification)\n\
         无效的商标类别: {class} (尼斯分类必须为1-45)\n\
         Kelas cap dagangan tidak sah: {class} (mesti 1-45 mengikut Klasifikasi Nice)"
    )]
    InvalidClass { class: u32 },

    /// IPOS filing deadline missed
    #[error(
        "IPOS filing deadline missed by {days} days\n\
         IPOS申请截止日期已过{days}天\n\
         Tarikh akhir pemfailan IPOS terlepas {days} hari"
    )]
    FilingDeadlineMissed { days: u32 },

    /// Insufficient documentation
    #[error(
        "Insufficient documentation for registration: {missing}\n\
         注册文件不足: {missing}\n\
         Dokumentasi tidak mencukupi untuk pendaftaran: {missing}"
    )]
    InsufficientDocumentation { missing: String },

    /// Generic validation error
    #[error(
        "IP validation error: {message}\n\
         知识产权验证错误: {message}\n\
         Ralat pengesahan IP: {message}"
    )]
    ValidationError { message: String },
}

impl IpError {
    /// Returns the statute reference for this error
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            // Patents Act
            IpError::LacksNovelty { .. } => Some("Patents Act s. 14(2)"),
            IpError::LacksInventiveStep => Some("Patents Act s. 15"),
            IpError::NotIndustriallyApplicable => Some("Patents Act s. 16"),
            IpError::PatentExpired { .. } => Some("Patents Act s. 36"),
            IpError::PatentInfringement { .. } => Some("Patents Act s. 66"),

            // Trade Marks Act
            IpError::NotDistinctive { .. } => Some("Trade Marks Act s. 7(1)(b)"),
            IpError::PurelyDescriptive { .. } => Some("Trade Marks Act s. 7(1)(c)"),
            IpError::TrademarkTooSimilar { .. } => Some("Trade Marks Act s. 8(2)"),
            IpError::Deceptive { .. } => Some("Trade Marks Act s. 7(4)"),
            IpError::ProhibitedElement { .. } => Some("Trade Marks Act s. 7(5)"),
            IpError::TrademarkInfringement { .. } => Some("Trade Marks Act s. 27"),
            IpError::TrademarkExpired { .. } => Some("Trade Marks Act s. 18"),

            // Copyright Act
            IpError::CopyrightExpired { .. } => Some("Copyright Act s. 28"),
            IpError::CopyrightInfringement { .. } => Some("Copyright Act s. 103"),
            IpError::NotCopyrightable { .. } => Some("Copyright Act s. 27"),
            IpError::FairDealingNotApplicable { .. } => Some("Copyright Act s. 35-42"),

            // Registered Designs Act
            IpError::DesignLacksNovelty { .. } => Some("Registered Designs Act s. 5(2)"),
            IpError::LacksIndividualCharacter => Some("Registered Designs Act s. 5(3)"),
            IpError::PurelyFunctional => Some("Registered Designs Act s. 6"),
            IpError::DesignExpired { .. } => Some("Registered Designs Act s. 21"),
            IpError::DesignInfringement => Some("Registered Designs Act s. 30"),

            _ => None,
        }
    }

    /// Returns the IP type this error relates to
    pub fn ip_type(&self) -> IpType {
        match self {
            IpError::LacksNovelty { .. }
            | IpError::LacksInventiveStep
            | IpError::NotIndustriallyApplicable
            | IpError::PatentExpired { .. }
            | IpError::PatentInfringement { .. } => IpType::Patent,

            IpError::NotDistinctive { .. }
            | IpError::PurelyDescriptive { .. }
            | IpError::TrademarkTooSimilar { .. }
            | IpError::Deceptive { .. }
            | IpError::ProhibitedElement { .. }
            | IpError::TrademarkInfringement { .. }
            | IpError::TrademarkExpired { .. }
            | IpError::InvalidClass { .. } => IpType::Trademark,

            IpError::CopyrightExpired { .. }
            | IpError::CopyrightInfringement { .. }
            | IpError::NotCopyrightable { .. }
            | IpError::FairDealingNotApplicable { .. } => IpType::Copyright,

            IpError::DesignLacksNovelty { .. }
            | IpError::LacksIndividualCharacter
            | IpError::PurelyFunctional
            | IpError::DesignExpired { .. }
            | IpError::DesignInfringement => IpType::Design,

            _ => IpType::General,
        }
    }

    /// Returns severity level (1-5)
    pub fn severity(&self) -> u8 {
        match self {
            IpError::PatentInfringement { .. }
            | IpError::TrademarkInfringement { .. }
            | IpError::CopyrightInfringement { .. }
            | IpError::DesignInfringement => 5, // Infringement is serious

            IpError::LacksNovelty { .. }
            | IpError::LacksInventiveStep
            | IpError::TrademarkTooSimilar { .. }
            | IpError::DesignLacksNovelty { .. } => 4, // Registration blockers

            IpError::PatentExpired { .. }
            | IpError::TrademarkExpired { .. }
            | IpError::CopyrightExpired { .. }
            | IpError::DesignExpired { .. } => 3, // Term expiry

            IpError::NotDistinctive { .. }
            | IpError::PurelyDescriptive { .. }
            | IpError::NotCopyrightable { .. } => 2, // Eligibility issues

            _ => 1, // Minor issues
        }
    }
}

/// IP type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpType {
    Patent,
    Trademark,
    Copyright,
    Design,
    General,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = IpError::LacksNovelty {
            description: "Prior art found".to_string(),
        };
        assert_eq!(error.statute_reference(), Some("Patents Act s. 14(2)"));

        let error2 = IpError::TrademarkTooSimilar {
            existing: "ACME".to_string(),
            class: 25,
            score: 85,
        };
        assert_eq!(error2.statute_reference(), Some("Trade Marks Act s. 8(2)"));
    }

    #[test]
    fn test_error_ip_type() {
        let patent_error = IpError::LacksInventiveStep;
        assert_eq!(patent_error.ip_type(), IpType::Patent);

        let trademark_error = IpError::NotDistinctive { class: 9 };
        assert_eq!(trademark_error.ip_type(), IpType::Trademark);

        let copyright_error = IpError::CopyrightExpired {
            years: 75,
            event: "author's death".to_string(),
        };
        assert_eq!(copyright_error.ip_type(), IpType::Copyright);
    }

    #[test]
    fn test_error_severity() {
        let infringement = IpError::PatentInfringement {
            description: "Unauthorized use".to_string(),
        };
        assert_eq!(infringement.severity(), 5);

        let lacks_novelty = IpError::LacksNovelty {
            description: "Prior art".to_string(),
        };
        assert_eq!(lacks_novelty.severity(), 4);

        let expired = IpError::TrademarkExpired { years_ago: 12 };
        assert_eq!(expired.severity(), 3);
    }

    #[test]
    fn test_error_display() {
        let error = IpError::TrademarkTooSimilar {
            existing: "APPLE".to_string(),
            class: 9,
            score: 90,
        };
        let display = format!("{}", error);
        assert!(display.contains("APPLE"));
        assert!(display.contains("Class 9"));
        assert!(display.contains("90%"));
        assert!(display.contains("商标")); // Chinese
        assert!(display.contains("Cap dagangan")); // Malay
    }
}
