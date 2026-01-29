//! Book V: Family - Civil and Commercial Code B.E. 2535
//!
//! Book V (มาตรา 1448-1598/46) covers family law including:
//! - Marriage (การสมรส)
//! - Divorce (การหย่า)
//! - Adoption (การรับบุตรบุญธรรม)
//! - Parental power (อำนาจปกครอง)
//! - Guardianship (ผู้อนุบาล)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Marriage requirements (หลักเกณฑ์การสมรส)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarriageRequirement {
    /// Minimum age 17 years (Section 1448)
    MinimumAge,

    /// Consent of both parties (Section 1450)
    Consent,

    /// Parental consent if under 20 (Section 1454)
    ParentalConsent,

    /// Registration (Section 1457)
    Registration,

    /// No prohibited relationship (Section 1450)
    NoProhibitedRelationship,
}

impl MarriageRequirement {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::MinimumAge => "อายุไม่ต่ำกว่า 17 ปี",
            Self::Consent => "ความยินยอมของคู่สมรส",
            Self::ParentalConsent => "ความยินยอมของผู้ปกครอง (หากอายุต่ำกว่า 20 ปี)",
            Self::Registration => "จดทะเบียนสมรส",
            Self::NoProhibitedRelationship => "ไม่มีความสัมพันธ์ต้องห้าม",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::MinimumAge => "Minimum Age 17 Years",
            Self::Consent => "Consent of Both Parties",
            Self::ParentalConsent => "Parental Consent if Under 20",
            Self::Registration => "Registration Required",
            Self::NoProhibitedRelationship => "No Prohibited Relationship",
        }
    }

    /// Get CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::MinimumAge => 1448,
            Self::Consent => 1450,
            Self::ParentalConsent => 1454,
            Self::Registration => 1457,
            Self::NoProhibitedRelationship => 1450,
        }
    }
}

/// Grounds for divorce (เหตุหย่า)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceGround {
    /// Mutual consent (ความยินยอม) - Section 1514
    MutualConsent,

    /// Adultery (ล่วงประเวณี) - Section 1516(1)
    Adultery,

    /// Misconduct (ประพฤติชั่ว) - Section 1516(2)
    Misconduct,

    /// Imprisonment (ถูกจำคุก) - Section 1516(3)
    Imprisonment,

    /// Desertion (ละทิ้ง) - Section 1516(4)
    Desertion,

    /// Insanity (วิกลจริต) - Section 1516(5)
    Insanity,

    /// Cruelty (ทารุณกรรม) - Section 1516(6)
    Cruelty,

    /// Serious injury (ทำร้ายร่างกายหรือจิตใจ) - Section 1516(7)
    SeriousInjury,

    /// Separated over 3 years (แยกกันอยู่เกิน 3 ปี) - Section 1516(9)
    SeparationOverThreeYears,

    /// Incurable disease (โรคร้ายแรง) - Section 1516(10)
    IncurableDisease,
}

impl DivorceGround {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::MutualConsent => "ความยินยอมร่วมกัน",
            Self::Adultery => "ล่วงประเวณี",
            Self::Misconduct => "ประพฤติชั่ว",
            Self::Imprisonment => "ถูกจำคุก",
            Self::Desertion => "ละทิ้งกัน",
            Self::Insanity => "วิกลจริต",
            Self::Cruelty => "ทารุณกรรม",
            Self::SeriousInjury => "ทำร้ายร่างกายหรือจิตใจอย่างร้ายแรง",
            Self::SeparationOverThreeYears => "แยกกันอยู่เกิน 3 ปี",
            Self::IncurableDisease => "เป็นโรคร้ายแรงซึ่งรักษาไม่หาย",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::MutualConsent => "Mutual Consent",
            Self::Adultery => "Adultery",
            Self::Misconduct => "Serious Misconduct",
            Self::Imprisonment => "Imprisonment",
            Self::Desertion => "Desertion",
            Self::Insanity => "Insanity",
            Self::Cruelty => "Cruelty",
            Self::SeriousInjury => "Serious Injury to Body or Mind",
            Self::SeparationOverThreeYears => "Separation Over 3 Years",
            Self::IncurableDisease => "Incurable Communicable Disease",
        }
    }

    /// Check if requires court proceeding
    pub fn requires_court(&self) -> bool {
        !matches!(self, Self::MutualConsent)
    }
}

/// Marital property regime (ระบบทรัพย์สินระหว่างสามีภริยา)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyRegime {
    /// Sin somros (สินสมรส) - jointly acquired property (Section 1474)
    SinSomros,

    /// Sin suan tua (สินส่วนตัว) - separate property (Section 1471)
    SinSuanTua,
}

impl PropertyRegime {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::SinSomros => "สินสมรส",
            Self::SinSuanTua => "สินส่วนตัว",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SinSomros => "Marital Property (Sin Somros)",
            Self::SinSuanTua => "Separate Property (Sin Suan Tua)",
        }
    }

    /// Get CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::SinSomros => 1474,
            Self::SinSuanTua => 1471,
        }
    }
}

/// Adoption requirements (หลักเกณฑ์การรับบุตรบุญธรรม)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdoptionRequirement {
    /// Adopter at least 25 years old (Section 1598/2)
    MinimumAgeAdopter,

    /// At least 15 years older than adoptee (Section 1598/2)
    AgeGap,

    /// Consent of adoptee if over 15 (Section 1598/6)
    ConsentOfAdoptee,

    /// Consent of biological parents (Section 1598/7)
    ConsentOfBiologicalParents,

    /// Court order (Section 1598/1)
    CourtOrder,
}

impl AdoptionRequirement {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::MinimumAgeAdopter => "ผู้รับบุตรบุญธรรมต้องมีอายุไม่ต่ำกว่า 25 ปี",
            Self::AgeGap => "ต้องมีอายุมากกว่าบุตรบุญธรรมไม่น้อยกว่า 15 ปี",
            Self::ConsentOfAdoptee => "ความยินยอมของบุตรบุญธรรม (หากอายุเกิน 15 ปี)",
            Self::ConsentOfBiologicalParents => "ความยินยอมของบิดามารดาโดยธรรม",
            Self::CourtOrder => "คำสั่งศาล",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::MinimumAgeAdopter => "Adopter Minimum Age 25",
            Self::AgeGap => "At Least 15 Years Older Than Adoptee",
            Self::ConsentOfAdoptee => "Consent of Adoptee if Over 15",
            Self::ConsentOfBiologicalParents => "Consent of Biological Parents",
            Self::CourtOrder => "Court Order Required",
        }
    }
}

/// Marriage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marriage {
    /// Husband name
    pub husband: String,

    /// Wife name
    pub wife: String,

    /// Marriage date
    pub marriage_date: NaiveDate,

    /// Registration number
    pub registration_number: Option<String>,

    /// Is registered
    pub registered: bool,
}

impl Marriage {
    /// Create a new marriage
    pub fn new(husband: String, wife: String, marriage_date: NaiveDate) -> Self {
        Self {
            husband,
            wife,
            marriage_date,
            registration_number: None,
            registered: false,
        }
    }

    /// Calculate marriage duration in years
    pub fn duration_years(&self) -> u32 {
        let today = chrono::Utc::now().date_naive();
        let days = (today - self.marriage_date).num_days();
        if days < 0 {
            return 0;
        }
        (days / 365) as u32
    }

    /// Check if separated for over 3 years (divorce ground)
    pub fn separated_over_three_years(&self, separation_date: NaiveDate) -> bool {
        let today = chrono::Utc::now().date_naive();
        let days = (today - separation_date).num_days();
        days >= 3 * 365
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_requirements() {
        assert_eq!(MarriageRequirement::MinimumAge.section(), 1448);
        assert_eq!(MarriageRequirement::Registration.section(), 1457);
    }

    #[test]
    fn test_divorce_grounds() {
        assert!(!DivorceGround::MutualConsent.requires_court());
        assert!(DivorceGround::Adultery.requires_court());
    }

    #[test]
    fn test_property_regime() {
        assert_eq!(PropertyRegime::SinSomros.section(), 1474);
        assert_eq!(PropertyRegime::SinSuanTua.section(), 1471);
    }

    #[test]
    fn test_marriage_duration() {
        let marriage = Marriage::new(
            "Husband".to_string(),
            "Wife".to_string(),
            NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
        );

        // Duration will vary based on current date
        assert!(marriage.duration_years() >= 4);
    }

    #[test]
    fn test_separation_duration() {
        let marriage = Marriage::new(
            "Husband".to_string(),
            "Wife".to_string(),
            NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
        );

        let recent_separation = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        assert!(!marriage.separated_over_three_years(recent_separation));

        let old_separation = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        assert!(marriage.separated_over_three_years(old_separation));
    }
}
