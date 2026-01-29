//! Book VI: Succession - Civil and Commercial Code B.E. 2535
//!
//! Book VI (มาตรา 1599-1754) covers inheritance law including:
//! - Statutory succession (การรับมรดกโดยธรรม)
//! - Testamentary succession (การรับมรดกโดยพินัยกรรม)
//! - Administration of estates (การจัดการมรดก)

use serde::{Deserialize, Serialize};

/// Classes of statutory heirs (ทายาทโดยธรรม) - Section 1629
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HeirClass {
    /// Class 1: Descendants (ผู้สืบสันดาน) - children, grandchildren
    Descendants = 1,

    /// Class 2: Parents (บิดามารดา)
    Parents = 2,

    /// Class 3: Full siblings (พี่น้องร่วมบิดามารดาเดียวกัน)
    FullSiblings = 3,

    /// Class 4: Half siblings (พี่น้องร่วมบิดาหรือมารดาเดียวกัน)
    HalfSiblings = 4,

    /// Class 5: Grandparents (ปู่ย่าตายาย)
    Grandparents = 5,

    /// Class 6: Uncles/Aunts (ลุง ป้า น้า อา)
    UnclesAunts = 6,
}

impl HeirClass {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Descendants => "ผู้สืบสันดาน",
            Self::Parents => "บิดามารดา",
            Self::FullSiblings => "พี่น้องร่วมบิดามารดาเดียวกัน",
            Self::HalfSiblings => "พี่น้องร่วมบิดาหรือมารดาเดียวกัน",
            Self::Grandparents => "ปู่ย่าตายาย",
            Self::UnclesAunts => "ลุง ป้า น้า อา",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Descendants => "Descendants",
            Self::Parents => "Parents",
            Self::FullSiblings => "Full Siblings",
            Self::HalfSiblings => "Half Siblings",
            Self::Grandparents => "Grandparents",
            Self::UnclesAunts => "Uncles and Aunts",
        }
    }

    /// Get inheritance share description
    pub fn share_description_th(&self) -> &'static str {
        match self {
            Self::Descendants => "แบ่งเท่าๆ กัน",
            Self::Parents => "แบ่งเท่าๆ กัน",
            Self::FullSiblings => "แบ่งเท่าๆ กัน",
            Self::HalfSiblings => "ได้เป็นครึ่งหนึ่งของพี่น้องร่วมบิดามารดา",
            Self::Grandparents => "แบ่งเท่าๆ กัน",
            Self::UnclesAunts => "แบ่งเท่าๆ กัน",
        }
    }
}

/// Spouse's share in statutory succession (ส่วนแบ่งของคู่สมรส) - Section 1635
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpouseShare {
    /// Same share as descendants (เท่ากับผู้สืบสันดาน)
    SameAsDescendants,

    /// Half of estate (ครึ่งหนึ่งของมรดก)
    HalfOfEstate,

    /// Entire estate if no other heirs (ทั้งหมด)
    EntireEstate,
}

impl SpouseShare {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::SameAsDescendants => "เท่ากับผู้สืบสันดาน",
            Self::HalfOfEstate => "ครึ่งหนึ่งของมรดก",
            Self::EntireEstate => "มรดกทั้งหมด",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SameAsDescendants => "Same as Each Descendant",
            Self::HalfOfEstate => "Half of Estate",
            Self::EntireEstate => "Entire Estate",
        }
    }

    /// Determine spouse's share based on heir class
    pub fn from_heir_class(heir_class: Option<HeirClass>) -> Self {
        match heir_class {
            Some(HeirClass::Descendants) => Self::SameAsDescendants,
            Some(_) => Self::HalfOfEstate,
            None => Self::EntireEstate,
        }
    }
}

/// Will types (ประเภทพินัยกรรม)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Holographic will (พินัยกรรมที่ผู้ทำพินัยกรรมเขียนเอง) - Section 1657
    Holographic,

    /// Public document will (พินัยกรรมที่ทำต่อหน้าพยาน) - Section 1656
    PublicDocument,

    /// Secret will (พินัยกรรมลับ) - Section 1658
    Secret,

    /// Oral will (พินัยกรรมด้วยวาจา) - Section 1659 (emergency only)
    Oral,
}

impl WillType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Holographic => "พินัยกรรมที่ผู้ทำพินัยกรรมเขียนเอง",
            Self::PublicDocument => "พินัยกรรมที่ทำต่อหน้าพยาน",
            Self::Secret => "พินัยกรรมลับ",
            Self::Oral => "พินัยกรรมด้วยวาจา",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Holographic => "Holographic Will",
            Self::PublicDocument => "Public Document Will",
            Self::Secret => "Secret Will",
            Self::Oral => "Oral Will",
        }
    }

    /// Get CCC section
    pub fn section(&self) -> u32 {
        match self {
            Self::Holographic => 1657,
            Self::PublicDocument => 1656,
            Self::Secret => 1658,
            Self::Oral => 1659,
        }
    }

    /// Check if requires witnesses
    pub fn requires_witnesses(&self) -> bool {
        matches!(self, Self::PublicDocument | Self::Secret | Self::Oral)
    }

    /// Number of witnesses required
    pub fn witness_count(&self) -> u32 {
        match self {
            Self::Holographic => 0,
            Self::PublicDocument => 2,
            Self::Secret => 2,
            Self::Oral => 2,
        }
    }
}

/// Grounds for disinheritance (เหตุให้ตัดมรดก) - Section 1607
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisinheritanceGround {
    /// Murder or attempted murder (ฆ่าหรือพยายามฆ่า) - Section 1607(1)
    MurderOrAttempt,

    /// Serious assault (ทำร้ายร่างกายอย่างร้ายแรง) - Section 1607(2)
    SeriousAssault,

    /// Defamation (หมิ่นประมาท) - Section 1607(3)
    Defamation,

    /// Conviction of serious crime (ต้องโทษจำคุกในความผิดร้ายแรง) - Section 1607(4)
    SeriousCrime,

    /// Failure to support (ไม่อุปการะเลี้ยงดู) - Section 1607(5)
    FailureToSupport,

    /// Misconduct (ประพฤติชั่วร้ายแรง) - Section 1607(6)
    Misconduct,
}

impl DisinheritanceGround {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::MurderOrAttempt => "ฆ่าหรือพยายามฆ่าผู้ตาย",
            Self::SeriousAssault => "ทำร้ายร่างกายผู้ตายอย่างร้ายแรง",
            Self::Defamation => "หมิ่นประมาทผู้ตายอย่างร้ายแรง",
            Self::SeriousCrime => "ต้องโทษจำคุกในความผิดร้ายแรง",
            Self::FailureToSupport => "ไม่อุปการะเลี้ยงดูผู้ตาย",
            Self::Misconduct => "ประพฤติชั่วร้ายแรง",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::MurderOrAttempt => "Murder or Attempted Murder of Deceased",
            Self::SeriousAssault => "Serious Assault on Deceased",
            Self::Defamation => "Serious Defamation of Deceased",
            Self::SeriousCrime => "Conviction of Serious Crime",
            Self::FailureToSupport => "Failure to Support Deceased",
            Self::Misconduct => "Serious Misconduct",
        }
    }
}

/// Estate administration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estate {
    /// Deceased person's name
    pub deceased: String,

    /// Total estate value (THB)
    pub total_value: u64,

    /// List of heirs with their classes
    pub heirs: Vec<(String, HeirClass)>,

    /// Spouse if any
    pub spouse: Option<String>,

    /// Has will
    pub has_will: bool,

    /// Will type if any
    pub will_type: Option<WillType>,
}

impl Estate {
    /// Create a new estate
    pub fn new(deceased: String, total_value: u64) -> Self {
        Self {
            deceased,
            total_value,
            heirs: Vec::new(),
            spouse: None,
            has_will: false,
            will_type: None,
        }
    }

    /// Add an heir
    pub fn add_heir(&mut self, name: String, class: HeirClass) {
        self.heirs.push((name, class));
    }

    /// Get highest priority heir class
    pub fn highest_priority_class(&self) -> Option<HeirClass> {
        self.heirs.iter().map(|(_, class)| *class).min()
    }

    /// Filter heirs by priority class (only heirs of highest priority class inherit)
    pub fn priority_heirs(&self) -> Vec<(String, HeirClass)> {
        if let Some(priority_class) = self.highest_priority_class() {
            self.heirs
                .iter()
                .filter(|(_, class)| *class == priority_class)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Calculate shares for statutory succession
    pub fn calculate_statutory_shares(&self) -> Vec<(String, u64)> {
        let priority_heirs = self.priority_heirs();
        if priority_heirs.is_empty() && self.spouse.is_none() {
            return Vec::new();
        }

        let mut shares = Vec::new();
        let heir_class = self.highest_priority_class();

        if let Some(spouse_name) = &self.spouse {
            let spouse_share = SpouseShare::from_heir_class(heir_class);
            let spouse_amount = match spouse_share {
                SpouseShare::EntireEstate => self.total_value,
                SpouseShare::HalfOfEstate => self.total_value / 2,
                SpouseShare::SameAsDescendants => {
                    self.total_value / (priority_heirs.len() as u64 + 1)
                }
            };
            shares.push((spouse_name.clone(), spouse_amount));

            // Calculate remaining for other heirs
            let remaining = self.total_value - spouse_amount;
            if !priority_heirs.is_empty() {
                let per_heir = remaining / priority_heirs.len() as u64;
                for (name, _) in priority_heirs {
                    shares.push((name, per_heir));
                }
            }
        } else if !priority_heirs.is_empty() {
            let per_heir = self.total_value / priority_heirs.len() as u64;
            for (name, _) in priority_heirs {
                shares.push((name, per_heir));
            }
        }

        shares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heir_classes() {
        assert!(HeirClass::Descendants < HeirClass::Parents);
        assert!(HeirClass::Parents < HeirClass::FullSiblings);
    }

    #[test]
    fn test_spouse_share() {
        assert_eq!(
            SpouseShare::from_heir_class(Some(HeirClass::Descendants)),
            SpouseShare::SameAsDescendants
        );
        assert_eq!(
            SpouseShare::from_heir_class(Some(HeirClass::Parents)),
            SpouseShare::HalfOfEstate
        );
        assert_eq!(
            SpouseShare::from_heir_class(None),
            SpouseShare::EntireEstate
        );
    }

    #[test]
    fn test_will_types() {
        assert_eq!(WillType::Holographic.witness_count(), 0);
        assert_eq!(WillType::PublicDocument.witness_count(), 2);
        assert!(!WillType::Holographic.requires_witnesses());
        assert!(WillType::PublicDocument.requires_witnesses());
    }

    #[test]
    fn test_estate_priority() {
        let mut estate = Estate::new("Deceased".to_string(), 1_000_000);
        estate.add_heir("Child 1".to_string(), HeirClass::Descendants);
        estate.add_heir("Parent 1".to_string(), HeirClass::Parents);
        estate.add_heir("Child 2".to_string(), HeirClass::Descendants);

        assert_eq!(
            estate.highest_priority_class(),
            Some(HeirClass::Descendants)
        );
        assert_eq!(estate.priority_heirs().len(), 2); // Only descendants
    }

    #[test]
    fn test_statutory_succession_shares() {
        let mut estate = Estate::new("Deceased".to_string(), 1_000_000);
        estate.add_heir("Child 1".to_string(), HeirClass::Descendants);
        estate.add_heir("Child 2".to_string(), HeirClass::Descendants);
        estate.spouse = Some("Spouse".to_string());

        let shares = estate.calculate_statutory_shares();
        assert_eq!(shares.len(), 3); // Spouse + 2 children

        // Each should get equal share (1/3 each)
        for (_, amount) in shares {
            assert_eq!(amount, 333_333);
        }
    }
}
