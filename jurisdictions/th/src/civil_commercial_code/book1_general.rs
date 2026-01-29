//! Book I: General Provisions - Civil and Commercial Code B.E. 2535
//!
//! Book I (มาตรา 1-248) covers fundamental principles of Thai civil law including:
//! - Legal personality (บุคคล)
//! - Juristic acts (นิติกรรม)
//! - Limitation periods (อายุความ)
//! - Prescription (การได้มาโดยครอบครอง)

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
#[cfg(test)]
use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Civil and Commercial Code reference
pub fn ccc() -> ThaiAct {
    ThaiAct::new(
        "ประมวลกฎหมายแพ่งและพาณิชย์",
        "Civil and Commercial Code",
        BuddhistYear::from_be(2535),
    )
}

/// Legal capacity under Thai law (ความสามารถในทางกฎหมาย)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (บรรลุนิติภาวะ) - 20 years or older (Section 19)
    Full,

    /// Limited capacity (ผู้เยาว์) - 7-19 years (Sections 20-27)
    Limited,

    /// No capacity (ไร้ความสามารถ) - under 7 years, adjudged incompetent
    None,

    /// Quasi-incompetent (เสมือนไร้ความสามารถ) - prodigal, habitual drunkard (Section 29)
    QuasiIncompetent,
}

impl LegalCapacity {
    /// Determine legal capacity based on age
    pub fn from_age(age: u32) -> Self {
        if age >= 20 {
            Self::Full
        } else if age >= 7 {
            Self::Limited
        } else {
            Self::None
        }
    }

    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Full => "บุคคลผู้มีความสามารถ",
            Self::Limited => "ผู้เยาว์",
            Self::None => "ผู้ไร้ความสามารถ",
            Self::QuasiIncompetent => "ผู้เสมือนไร้ความสามารถ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Full => "Person with Full Capacity",
            Self::Limited => "Minor",
            Self::None => "Incompetent Person",
            Self::QuasiIncompetent => "Quasi-Incompetent Person",
        }
    }

    /// Can perform juristic acts independently
    pub fn can_act_independently(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// Requires consent of legal representative
    pub fn requires_consent(&self) -> bool {
        matches!(self, Self::Limited | Self::None | Self::QuasiIncompetent)
    }
}

/// Juristic act validity (ความสมบูรณ์ของนิติกรรม)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JuristicActValidity {
    /// Valid (สมบูรณ์) - all requirements met
    Valid,

    /// Voidable (อาจเพิกถอนได้) - can be cancelled by one party (Section 147)
    Voidable,

    /// Void (เป็นโมฆะ) - no legal effect (Section 150)
    Void,
}

impl JuristicActValidity {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Valid => "นิติกรรมสมบูรณ์",
            Self::Voidable => "นิติกรรมอาจเพิกถอนได้",
            Self::Void => "นิติกรรมเป็นโมฆะ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Valid => "Valid Juristic Act",
            Self::Voidable => "Voidable Juristic Act",
            Self::Void => "Void Juristic Act",
        }
    }

    /// Can be ratified
    pub fn can_ratify(&self) -> bool {
        matches!(self, Self::Voidable)
    }
}

/// Grounds for voidable juristic acts (เหตุทำให้นิติกรรมอาจเพิกถอนได้)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoidableGround {
    /// Mistake (ความผิดพลาดในสาระสำคัญ) - Section 157
    Mistake,

    /// Fraud (การฉ้อฉล) - Section 159
    Fraud,

    /// Duress (การบังคับขู่เข็ญ) - Section 160
    Duress,

    /// Lack of capacity (ขาดความสามารถ) - Section 147
    LackOfCapacity,
}

impl VoidableGround {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Mistake => "ความผิดพลาดในสาระสำคัญ",
            Self::Fraud => "การฉ้อฉล",
            Self::Duress => "การบังคับขู่เข็ญ",
            Self::LackOfCapacity => "ขาดความสามารถในการทำนิติกรรม",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Mistake => "Mistake",
            Self::Fraud => "Fraud",
            Self::Duress => "Duress",
            Self::LackOfCapacity => "Lack of Legal Capacity",
        }
    }

    /// Get CCC section number
    pub fn section(&self) -> u32 {
        match self {
            Self::Mistake => 157,
            Self::Fraud => 159,
            Self::Duress => 160,
            Self::LackOfCapacity => 147,
        }
    }
}

/// Limitation periods (อายุความ) under Thai law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationPeriod {
    /// 10 years - general limitation (Section 193/33)
    TenYears,

    /// 5 years - contracts, torts (Section 193/34)
    FiveYears,

    /// 2 years - periodic payments (Section 193/36)
    TwoYears,

    /// 1 year - defamation, assault (Section 193/37)
    OneYear,

    /// 20 years - ownership claims (Section 1382)
    TwentyYears,
}

impl LimitationPeriod {
    /// Get period in years
    pub fn years(&self) -> u32 {
        match self {
            Self::TenYears => 10,
            Self::FiveYears => 5,
            Self::TwoYears => 2,
            Self::OneYear => 1,
            Self::TwentyYears => 20,
        }
    }

    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::TenYears => "อายุความ 10 ปี (ทั่วไป)",
            Self::FiveYears => "อายุความ 5 ปี (สัญญา, ละเมิด)",
            Self::TwoYears => "อายุความ 2 ปี (การชำระเป็นงวด)",
            Self::OneYear => "อายุความ 1 ปี (หมิ่นประมาท, ทำร้าย)",
            Self::TwentyYears => "อายุความ 20 ปี (กรรมสิทธิ์)",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::TenYears => "10-Year Limitation (General)",
            Self::FiveYears => "5-Year Limitation (Contracts, Torts)",
            Self::TwoYears => "2-Year Limitation (Periodic Payments)",
            Self::OneYear => "1-Year Limitation (Defamation, Assault)",
            Self::TwentyYears => "20-Year Limitation (Ownership)",
        }
    }

    /// Calculate expiry date from start date
    pub fn expiry_date(&self, start_date: NaiveDate) -> Option<NaiveDate> {
        start_date.checked_add_months(chrono::Months::new(self.years() * 12))
    }
}

/// Domicile (ภูมิลำเนา) types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomicileType {
    /// Legal domicile (ภูมิลำเนาโดยผลของกฎหมาย) - Section 41
    Legal,

    /// Voluntary domicile (ภูมิลำเนาโดยความสมัครใจ) - Section 40
    Voluntary,

    /// Domicile of choice for business (ภูมิลำเนาสำหรับกิจการพิเศษ) - Section 47
    Business,
}

impl DomicileType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Legal => "ภูมิลำเนาโดยผลของกฎหมาย",
            Self::Voluntary => "ภูมิลำเนาโดยความสมัครใจ",
            Self::Business => "ภูมิลำเนาสำหรับกิจการพิเศษ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Legal => "Legal Domicile",
            Self::Voluntary => "Voluntary Domicile",
            Self::Business => "Business Domicile",
        }
    }
}

/// Person entity (บุคคล)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Name
    pub name: String,

    /// Date of birth
    pub date_of_birth: NaiveDate,

    /// Legal capacity
    pub capacity: LegalCapacity,

    /// Domicile type
    pub domicile_type: DomicileType,
}

impl Person {
    /// Create a new person
    pub fn new(name: String, date_of_birth: NaiveDate) -> Self {
        let age = Self::calculate_age(date_of_birth);
        Self {
            name,
            date_of_birth,
            capacity: LegalCapacity::from_age(age),
            domicile_type: DomicileType::Voluntary,
        }
    }

    /// Calculate age in years
    pub fn calculate_age(date_of_birth: NaiveDate) -> u32 {
        let today = chrono::Utc::now().date_naive();
        let days = (today - date_of_birth).num_days();
        if days < 0 {
            return 0;
        }
        (days / 365) as u32
    }

    /// Get current age
    pub fn age(&self) -> u32 {
        Self::calculate_age(self.date_of_birth)
    }

    /// Check if person has reached legal age (20 years)
    pub fn is_of_legal_age(&self) -> bool {
        self.age() >= 20
    }

    /// Update legal capacity based on current age
    pub fn update_capacity(&mut self) {
        self.capacity = LegalCapacity::from_age(self.age());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_capacity_from_age() {
        assert_eq!(LegalCapacity::from_age(25), LegalCapacity::Full);
        assert_eq!(LegalCapacity::from_age(15), LegalCapacity::Limited);
        assert_eq!(LegalCapacity::from_age(5), LegalCapacity::None);
    }

    #[test]
    fn test_legal_capacity_permissions() {
        assert!(LegalCapacity::Full.can_act_independently());
        assert!(!LegalCapacity::Limited.can_act_independently());
        assert!(LegalCapacity::Limited.requires_consent());
    }

    #[test]
    fn test_juristic_act_validity() {
        assert!(JuristicActValidity::Voidable.can_ratify());
        assert!(!JuristicActValidity::Void.can_ratify());
    }

    #[test]
    fn test_voidable_ground_sections() {
        assert_eq!(VoidableGround::Fraud.section(), 159);
        assert_eq!(VoidableGround::Duress.section(), 160);
    }

    #[test]
    fn test_limitation_periods() {
        assert_eq!(LimitationPeriod::TenYears.years(), 10);
        assert_eq!(LimitationPeriod::FiveYears.years(), 5);
    }

    #[test]
    fn test_limitation_expiry() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let period = LimitationPeriod::FiveYears;
        let expiry = period.expiry_date(start).expect("valid expiry");

        assert_eq!(expiry.year(), 2025);
    }

    #[test]
    fn test_person_age_calculation() {
        let dob = NaiveDate::from_ymd_opt(2000, 1, 1).expect("valid date");
        let person = Person::new("Test".to_string(), dob);

        // Age will vary based on current date, but should be >= 24 (as of 2024)
        assert!(person.age() >= 24);
    }

    #[test]
    fn test_person_legal_age() {
        let adult = Person::new(
            "Adult".to_string(),
            NaiveDate::from_ymd_opt(2000, 1, 1).expect("valid date"),
        );
        assert!(adult.is_of_legal_age());

        let minor = Person::new(
            "Minor".to_string(),
            NaiveDate::from_ymd_opt(2015, 1, 1).expect("valid date"),
        );
        assert!(!minor.is_of_legal_age());
    }

    #[test]
    fn test_ccc_citation() {
        let code = ccc();
        assert_eq!(code.year.be_year, 2535);
    }
}
