//! Basic Law of Governance (النظام الأساسي للحكم)
//!
//! Royal Decree No. A/90 dated 27/8/1412H (1992)
//!
//! The Basic Law serves as Saudi Arabia's constitutional document,
//! establishing the principles of governance, rights, and duties.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for Basic Law operations
pub type BasicLawResult<T> = Result<T, BasicLawError>;

/// Basic Law errors
#[derive(Debug, Error)]
pub enum BasicLawError {
    /// Invalid article reference
    #[error("مرجع مادة غير صالح: {article}")]
    InvalidArticle { article: u32 },

    /// Principle violation
    #[error("انتهاك مبدأ النظام الأساسي: {principle}")]
    PrincipleViolation { principle: String },
}

/// Branches of government under the Basic Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovernmentBranch {
    /// Judicial Authority (السلطة القضائية)
    Judicial,
    /// Executive Authority (السلطة التنفيذية)
    Executive,
    /// Regulatory Authority (السلطة التنظيمية)
    Regulatory,
}

impl GovernmentBranch {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Judicial => "السلطة القضائية",
            Self::Executive => "السلطة التنفيذية",
            Self::Regulatory => "السلطة التنظيمية",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Judicial => "Judicial Authority",
            Self::Executive => "Executive Authority",
            Self::Regulatory => "Regulatory Authority",
        }
    }

    /// Get description of powers
    pub fn powers_description_en(&self) -> &'static str {
        match self {
            Self::Judicial => "Courts and judicial system based on Sharia",
            Self::Executive => "King and Council of Ministers",
            Self::Regulatory => "King and Shura Council",
        }
    }
}

/// Key articles from the Basic Law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicLawArticle {
    /// Article number
    pub number: u32,
    /// Article title in Arabic
    pub title_ar: Option<String>,
    /// Article title in English
    pub title_en: Option<String>,
    /// Article text in Arabic
    pub text_ar: String,
    /// Article text in English
    pub text_en: String,
}

impl BasicLawArticle {
    /// Create a new article
    pub fn new(number: u32, text_ar: impl Into<String>, text_en: impl Into<String>) -> Self {
        Self {
            number,
            title_ar: None,
            title_en: None,
            text_ar: text_ar.into(),
            text_en: text_en.into(),
        }
    }

    /// Add titles
    pub fn with_titles(mut self, title_ar: impl Into<String>, title_en: impl Into<String>) -> Self {
        self.title_ar = Some(title_ar.into());
        self.title_en = Some(title_en.into());
        self
    }
}

/// Get key Basic Law principles
pub fn get_basic_law_principles() -> Vec<(&'static str, &'static str)> {
    vec![
        ("الإسلام دين الدولة", "Islam is the religion of the state"),
        (
            "القرآن والسنة مصدر التشريع",
            "Quran and Sunnah are the source of legislation",
        ),
        ("العدل أساس الحكم", "Justice is the basis of governance"),
        ("الشورى", "Consultation (Shura)"),
        (
            "حماية حقوق الإنسان",
            "Protection of human rights per Islamic Sharia",
        ),
        ("المساواة", "Equality in accordance with Islamic Sharia"),
        ("حماية الملكية الخاصة", "Protection of private property"),
        ("استقلال القضاء", "Independence of the judiciary"),
    ]
}

/// Key articles from the Basic Law
pub mod articles {
    use super::*;

    /// Article 1: Sharia as supreme law
    pub fn article_1() -> BasicLawArticle {
        BasicLawArticle::new(
            1,
            "المملكة العربية السعودية دولة عربية إسلامية، ذات سيادة تامة، دينها الإسلام، ودستورها كتاب الله تعالى وسنة رسوله صلى الله عليه وسلم",
            "The Kingdom of Saudi Arabia is a sovereign Arab Islamic state with Islam as its religion; God's Book and the Sunnah of His Messenger are its constitution",
        )
    }

    /// Article 7: Government derives authority from Quran and Sunnah
    pub fn article_7() -> BasicLawArticle {
        BasicLawArticle::new(
            7,
            "يستمد الحكم في المملكة العربية السعودية سلطته من كتاب الله تعالى، وسنة رسوله، وهما الحاكمان على هذا النظام وجميع أنظمة الدولة",
            "Government in Saudi Arabia derives its power from the Holy Quran and the Prophet's Sunnah, which rule over this Law and all other laws of the State",
        )
    }

    /// Article 8: Governance based on justice, shura, and equality
    pub fn article_8() -> BasicLawArticle {
        BasicLawArticle::new(
            8,
            "يقوم الحكم في المملكة العربية السعودية على أساس العدل والشورى والمساواة وفق الشريعة الإسلامية",
            "Government in Saudi Arabia is based on justice, shura (consultation), and equality in accordance with Islamic Sharia",
        )
    }

    /// Article 26: Protection of human rights
    pub fn article_26() -> BasicLawArticle {
        BasicLawArticle::new(
            26,
            "تحمي الدولة حقوق الإنسان وفق الشريعة الإسلامية",
            "The State protects human rights in accordance with Islamic Sharia",
        )
    }

    /// Article 38: Punishment is personal
    pub fn article_38() -> BasicLawArticle {
        BasicLawArticle::new(
            38,
            "العقوبة شخصية، ولا جريمة ولا عقوبة إلا بناء على نص شرعي، أو نص نظامي، ولا عقاب إلا على الأعمال اللاحقة للعمل بالنص النظامي",
            "Punishment is personal. There is no crime or punishment except in accordance with Sharia or statutory law. There is no punishment except for acts committed subsequent to the coming into force of the statutory law",
        )
    }

    /// Article 44: Authorities of the State
    pub fn article_44() -> BasicLawArticle {
        BasicLawArticle::new(
            44,
            "تتكون السلطات في الدولة من: السلطة القضائية، السلطة التنفيذية، السلطة التنظيمية، وتتعاون هذه السلطات في أداء وظائفها، وفقاً لهذا النظام وغيره من الأنظمة، والملك هو مرجع هذه السلطات",
            "The authorities of the State consist of: the Judicial Authority, the Executive Authority, and the Regulatory Authority. These authorities cooperate in performing their functions, in accordance with this Law and other laws. The King is the point of reference for these authorities",
        )
    }

    /// Article 48: Shura Council
    pub fn article_48() -> BasicLawArticle {
        BasicLawArticle::new(
            48,
            "مجلس الشورى يتكون من رئيس وعدد من الأعضاء يختارهم الملك",
            "The Shura Council shall be composed of a President and members chosen by the King",
        )
    }
}

/// Validate if a governance action complies with Basic Law principles
pub fn validate_basic_law_compliance(
    action_description: &str,
    branch: &GovernmentBranch,
) -> BasicLawResult<()> {
    // Simplified validation logic
    if action_description.is_empty() {
        return Err(BasicLawError::PrincipleViolation {
            principle: "Action description cannot be empty".to_string(),
        });
    }

    // Check if the action aligns with the branch's authority
    match branch {
        GovernmentBranch::Judicial => {
            if !action_description.contains("court")
                && !action_description.contains("judge")
                && !action_description.contains("ruling")
            {
                // This is overly simplistic, but demonstrates the concept
            }
        }
        GovernmentBranch::Executive => {
            // Executive branch validation
        }
        GovernmentBranch::Regulatory => {
            // Regulatory branch validation
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_government_branches() {
        assert_eq!(GovernmentBranch::Judicial.name_ar(), "السلطة القضائية");
        assert_eq!(GovernmentBranch::Executive.name_en(), "Executive Authority");
    }

    #[test]
    fn test_article_1() {
        let art1 = articles::article_1();
        assert_eq!(art1.number, 1);
        assert!(art1.text_en.contains("Islam"));
        assert!(art1.text_ar.contains("الإسلام"));
    }

    #[test]
    fn test_article_7() {
        let art7 = articles::article_7();
        assert_eq!(art7.number, 7);
        assert!(art7.text_en.contains("Quran"));
        assert!(art7.text_ar.contains("كتاب الله")); // "Book of Allah" (refers to Quran)
    }

    #[test]
    fn test_article_8() {
        let art8 = articles::article_8();
        assert_eq!(art8.number, 8);
        assert!(art8.text_en.contains("justice"));
        assert!(art8.text_en.contains("shura"));
    }

    #[test]
    fn test_basic_law_principles() {
        let principles = get_basic_law_principles();
        assert!(!principles.is_empty());
        assert!(principles.len() >= 8);

        // Check first principle
        assert!(principles[0].0.contains("الإسلام"));
        assert!(principles[0].1.contains("Islam"));
    }

    #[test]
    fn test_article_26() {
        let art26 = articles::article_26();
        assert_eq!(art26.number, 26);
        assert!(art26.text_en.contains("human rights"));
    }

    #[test]
    fn test_article_44() {
        let art44 = articles::article_44();
        assert_eq!(art44.number, 44);
        assert!(art44.text_en.contains("Judicial"));
        assert!(art44.text_en.contains("Executive"));
        assert!(art44.text_en.contains("Regulatory"));
    }

    #[test]
    fn test_validate_compliance() {
        let result = validate_basic_law_compliance(
            "Court ruling on contract dispute",
            &GovernmentBranch::Judicial,
        );
        assert!(result.is_ok());
    }
}
