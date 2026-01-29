//! UAE Federal Penal Code - Federal Law No. 3/1987
//!
//! The UAE Penal Code (قانون العقوبات الاتحادي) governs criminal offenses
//! and punishments at the federal level.
//!
//! ## Classification of Crimes
//!
//! - **جنايات** (Felonies) - Serious crimes punishable by death, life imprisonment,
//!   or imprisonment exceeding 3 years
//! - **جنح** (Misdemeanors) - Crimes punishable by imprisonment up to 3 years
//!   or fines exceeding AED 1,000
//! - **مخالفات** (Violations) - Minor offenses punishable by fines up to AED 1,000
//!
//! ## Key Amendments
//!
//! - Federal Decree-Law No. 34/2021 (Cybercrime)
//! - Federal Decree-Law No. 36/2021 (Rumours and Electronic Crimes)

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for criminal law operations
pub type CriminalLawResult<T> = Result<T, CriminalLawError>;

/// Classification of criminal offenses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrimeClassification {
    /// Felony (جناية) - Serious offense
    Felony,
    /// Misdemeanor (جنحة) - Moderate offense
    Misdemeanor,
    /// Violation (مخالفة) - Minor offense
    Violation,
}

impl CrimeClassification {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Felony => "جناية",
            Self::Misdemeanor => "جنحة",
            Self::Violation => "مخالفة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Felony => "Felony",
            Self::Misdemeanor => "Misdemeanor",
            Self::Violation => "Violation",
        }
    }

    /// Get maximum imprisonment term (years, 0 = life/death possible)
    pub fn max_imprisonment_years(&self) -> u32 {
        match self {
            Self::Felony => 0, // Can include life/death
            Self::Misdemeanor => 3,
            Self::Violation => 0, // Typically fines only
        }
    }
}

/// Types of criminal penalties
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Penalty {
    /// Death penalty (الإعدام)
    Death,
    /// Life imprisonment (السجن المؤبد)
    LifeImprisonment,
    /// Temporary imprisonment (السجن المؤقت) - 3+ years
    TemporaryImprisonment { years: u32 },
    /// Confinement (الحبس) - up to 3 years
    Confinement { months: u32 },
    /// Fine (الغرامة)
    Fine { amount: Aed },
    /// Blood money (الدية) - for certain offenses
    BloodMoney { amount: Aed },
    /// Deportation (الإبعاد) - for non-citizens
    Deportation,
    /// Community service
    CommunityService { hours: u32 },
}

impl Penalty {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Death => "الإعدام",
            Self::LifeImprisonment => "السجن المؤبد",
            Self::TemporaryImprisonment { .. } => "السجن المؤقت",
            Self::Confinement { .. } => "الحبس",
            Self::Fine { .. } => "الغرامة",
            Self::BloodMoney { .. } => "الدية",
            Self::Deportation => "الإبعاد",
            Self::CommunityService { .. } => "الخدمة المجتمعية",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Death => "Death Penalty",
            Self::LifeImprisonment => "Life Imprisonment",
            Self::TemporaryImprisonment { .. } => "Temporary Imprisonment",
            Self::Confinement { .. } => "Confinement",
            Self::Fine { .. } => "Fine",
            Self::BloodMoney { .. } => "Blood Money (Diya)",
            Self::Deportation => "Deportation",
            Self::CommunityService { .. } => "Community Service",
        }
    }
}

/// Common criminal offenses under UAE Penal Code
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CriminalOffense {
    /// Murder (قتل عمد) - Article 332
    Murder,
    /// Manslaughter (قتل خطأ) - Article 343
    Manslaughter,
    /// Assault (اعتداء) - Article 338
    Assault,
    /// Theft (سرقة) - Article 379
    Theft,
    /// Fraud (احتيال) - Article 399
    Fraud,
    /// Embezzlement (خيانة الأمانة) - Article 404
    Embezzlement,
    /// Bribery (رشوة) - Article 234
    Bribery,
    /// Forgery (تزوير) - Article 216
    Forgery,
    /// Drug trafficking (تجارة المخدرات) - Federal Law 14/1995
    DrugTrafficking,
    /// Money laundering (غسل الأموال) - Federal Decree-Law 20/2018
    MoneyLaundering,
    /// Defamation (قذف) - Article 372
    Defamation,
    /// Insult (سب) - Article 373
    Insult,
}

impl CriminalOffense {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Murder => "قتل عمد",
            Self::Manslaughter => "قتل خطأ",
            Self::Assault => "اعتداء",
            Self::Theft => "سرقة",
            Self::Fraud => "احتيال",
            Self::Embezzlement => "خيانة الأمانة",
            Self::Bribery => "رشوة",
            Self::Forgery => "تزوير",
            Self::DrugTrafficking => "تجارة المخدرات",
            Self::MoneyLaundering => "غسل الأموال",
            Self::Defamation => "قذف",
            Self::Insult => "سب",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Murder => "Murder",
            Self::Manslaughter => "Manslaughter",
            Self::Assault => "Assault",
            Self::Theft => "Theft",
            Self::Fraud => "Fraud",
            Self::Embezzlement => "Embezzlement",
            Self::Bribery => "Bribery",
            Self::Forgery => "Forgery",
            Self::DrugTrafficking => "Drug Trafficking",
            Self::MoneyLaundering => "Money Laundering",
            Self::Defamation => "Defamation",
            Self::Insult => "Insult",
        }
    }

    /// Get crime classification
    pub fn classification(&self) -> CrimeClassification {
        match self {
            Self::Murder | Self::DrugTrafficking | Self::MoneyLaundering | Self::Bribery => {
                CrimeClassification::Felony
            }
            Self::Manslaughter
            | Self::Assault
            | Self::Theft
            | Self::Fraud
            | Self::Embezzlement
            | Self::Forgery => CrimeClassification::Misdemeanor,
            Self::Defamation | Self::Insult => CrimeClassification::Violation,
        }
    }

    /// Article reference in Penal Code
    pub fn article_reference(&self) -> u32 {
        match self {
            Self::Murder => 332,
            Self::Manslaughter => 343,
            Self::Assault => 338,
            Self::Theft => 379,
            Self::Fraud => 399,
            Self::Embezzlement => 404,
            Self::Bribery => 234,
            Self::Forgery => 216,
            Self::DrugTrafficking => 1, // Separate law
            Self::MoneyLaundering => 1, // Separate law
            Self::Defamation => 372,
            Self::Insult => 373,
        }
    }
}

/// Mitigating and aggravating circumstances
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CircumstanceType {
    /// Mitigating (ظروف مخففة)
    Mitigating,
    /// Aggravating (ظروف مشددة)
    Aggravating,
}

/// Specific circumstances
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Circumstance {
    /// Youth (صغر السن)
    Youth,
    /// First offense (سابقة أولى)
    FirstOffense,
    /// Provocation (استفزاز)
    Provocation,
    /// Premeditation (سبق إصرار)
    Premeditation,
    /// Use of weapon (استخدام سلاح)
    UseOfWeapon,
    /// Repeat offense (عود)
    RepeatOffense,
    /// Against public servant (ضد موظف عام)
    AgainstPublicServant,
}

impl Circumstance {
    /// Get circumstance type
    pub fn circumstance_type(&self) -> CircumstanceType {
        match self {
            Self::Youth | Self::FirstOffense | Self::Provocation => CircumstanceType::Mitigating,
            Self::Premeditation
            | Self::UseOfWeapon
            | Self::RepeatOffense
            | Self::AgainstPublicServant => CircumstanceType::Aggravating,
        }
    }

    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Youth => "صغر السن",
            Self::FirstOffense => "سابقة أولى",
            Self::Provocation => "استفزاز",
            Self::Premeditation => "سبق إصرار",
            Self::UseOfWeapon => "استخدام سلاح",
            Self::RepeatOffense => "عود",
            Self::AgainstPublicServant => "ضد موظف عام",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Youth => "Youth",
            Self::FirstOffense => "First Offense",
            Self::Provocation => "Provocation",
            Self::Premeditation => "Premeditation",
            Self::UseOfWeapon => "Use of Weapon",
            Self::RepeatOffense => "Repeat Offense",
            Self::AgainstPublicServant => "Against Public Servant",
        }
    }
}

/// Criminal procedural rights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralRights {
    /// Right to legal representation
    pub right_to_lawyer: bool,
    /// Right to interpreter
    pub right_to_interpreter: bool,
    /// Right to remain silent
    pub right_to_silence: bool,
    /// Right to bail (if applicable)
    pub right_to_bail: bool,
    /// Right to appeal
    pub right_to_appeal: bool,
}

impl ProceduralRights {
    /// Get standard procedural rights
    pub fn standard() -> Self {
        Self {
            right_to_lawyer: true,
            right_to_interpreter: true,
            right_to_silence: true,
            right_to_bail: false, // Depends on offense
            right_to_appeal: true,
        }
    }
}

/// Criminal law errors
#[derive(Debug, Error)]
pub enum CriminalLawError {
    /// Invalid offense classification
    #[error("تصنيف الجريمة غير صحيح: {offense}")]
    InvalidOffenseClassification { offense: String },

    /// Penalty calculation error
    #[error("خطأ في حساب العقوبة: {reason}")]
    PenaltyCalculationError { reason: String },

    /// Procedural violation
    #[error("انتهاك إجرائي: {violation}")]
    ProceduralViolation { violation: String },
}

/// Get criminal procedure checklist
pub fn get_criminal_procedure_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("الاتهام الواضح", "Clear charge"),
        ("حق الدفاع", "Right to defense"),
        ("حق التمثيل القانوني", "Right to legal representation"),
        ("حق الترجمة", "Right to interpreter"),
        ("الحق في الصمت", "Right to remain silent"),
        ("افتراض البراءة", "Presumption of innocence"),
        ("محاكمة عادلة", "Fair trial"),
        ("حق الاستئناف", "Right to appeal"),
    ]
}

/// Get common offenses and penalties reference
pub fn get_common_offenses() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("قتل عمد", "Murder", "Article 332 - Death/Life"),
        (
            "قتل خطأ",
            "Manslaughter",
            "Article 343 - Diya + Imprisonment",
        ),
        ("سرقة", "Theft", "Article 379 - Imprisonment/Fine"),
        ("احتيال", "Fraud", "Article 399 - Imprisonment"),
        ("تزوير", "Forgery", "Article 216 - Imprisonment"),
        ("رشوة", "Bribery", "Article 234 - Imprisonment"),
        ("قذف", "Defamation", "Article 372 - Fine"),
        ("سب", "Insult", "Article 373 - Fine"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crime_classification() {
        assert_eq!(CrimeClassification::Felony.name_ar(), "جناية");
        assert_eq!(CrimeClassification::Misdemeanor.max_imprisonment_years(), 3);
    }

    #[test]
    fn test_criminal_offenses() {
        let murder = CriminalOffense::Murder;
        assert_eq!(murder.name_ar(), "قتل عمد");
        assert_eq!(murder.classification(), CrimeClassification::Felony);
        assert_eq!(murder.article_reference(), 332);

        let theft = CriminalOffense::Theft;
        assert_eq!(theft.classification(), CrimeClassification::Misdemeanor);
    }

    #[test]
    fn test_penalties() {
        let fine = Penalty::Fine {
            amount: Aed::from_dirhams(5_000),
        };
        assert_eq!(fine.name_ar(), "الغرامة");

        let imprisonment = Penalty::TemporaryImprisonment { years: 5 };
        assert_eq!(imprisonment.name_en(), "Temporary Imprisonment");
    }

    #[test]
    fn test_circumstances() {
        let youth = Circumstance::Youth;
        assert_eq!(youth.circumstance_type(), CircumstanceType::Mitigating);
        assert_eq!(youth.name_ar(), "صغر السن");

        let premeditation = Circumstance::Premeditation;
        assert_eq!(
            premeditation.circumstance_type(),
            CircumstanceType::Aggravating
        );
    }

    #[test]
    fn test_procedural_rights() {
        let rights = ProceduralRights::standard();
        assert!(rights.right_to_lawyer);
        assert!(rights.right_to_silence);
        assert!(rights.right_to_appeal);
    }

    #[test]
    fn test_criminal_procedure_checklist() {
        let checklist = get_criminal_procedure_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 8);
    }

    #[test]
    fn test_common_offenses() {
        let offenses = get_common_offenses();
        assert!(!offenses.is_empty());
    }
}
