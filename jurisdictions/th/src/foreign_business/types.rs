//! FBA Types and Structures

use crate::calendar::BuddhistYear;
use crate::citation::ThaiAct;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Business restriction lists under FBA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessRestrictionList {
    /// List 1: Prohibited for foreigners (Section 8, Annex 1)
    List1Prohibited,

    /// List 2: Requires Cabinet approval (Section 8, Annex 2)
    List2CabinetApproval,

    /// List 3: Requires DBD license (Section 8, Annex 3)
    List3License,

    /// Not restricted
    NotRestricted,
}

impl BusinessRestrictionList {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::List1Prohibited => "บัญชี 1: ธุรกิจที่ห้ามมิให้คนต่างด้าวประกอบ",
            Self::List2CabinetApproval => "บัญชี 2: ธุรกิจที่ต้องได้รับอนุมัติจาก ครม.",
            Self::List3License => "บัญชี 3: ธุรกิจที่ต้องขออนุญาต",
            Self::NotRestricted => "ไม่อยู่ในบัญชีท้าย พ.ร.บ.",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::List1Prohibited => "List 1: Prohibited",
            Self::List2CabinetApproval => "List 2: Cabinet Approval Required",
            Self::List3License => "List 3: License Required",
            Self::NotRestricted => "Not Restricted",
        }
    }

    /// Get maximum foreign ownership allowed
    pub fn max_foreign_ownership(&self) -> f64 {
        match self {
            Self::List1Prohibited => 0.0,
            Self::List2CabinetApproval => 49.0, // With approval can be higher
            Self::List3License => 49.0,         // With license can be higher
            Self::NotRestricted => 100.0,
        }
    }
}

/// Business activities and their restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessActivity {
    /// Activity code
    pub code: String,

    /// Thai description
    pub name_th: String,

    /// English description
    pub name_en: String,

    /// Restriction list
    pub restriction_list: BusinessRestrictionList,

    /// Sub-category within list (if applicable)
    pub sub_category: Option<u32>,
}

impl BusinessActivity {
    /// Create a new business activity
    pub fn new(
        code: impl Into<String>,
        name_th: impl Into<String>,
        name_en: impl Into<String>,
        restriction_list: BusinessRestrictionList,
    ) -> Self {
        Self {
            code: code.into(),
            name_th: name_th.into(),
            name_en: name_en.into(),
            restriction_list,
            sub_category: None,
        }
    }

    /// Common List 1 activities (prohibited)
    pub fn list1_activities() -> Vec<Self> {
        vec![
            Self::new(
                "1.1",
                "การค้าที่ดิน",
                "Land Trading",
                BusinessRestrictionList::List1Prohibited,
            ),
            Self::new(
                "1.2",
                "กิจการหนังสือพิมพ์",
                "Newspaper Publishing",
                BusinessRestrictionList::List1Prohibited,
            ),
            Self::new(
                "1.3",
                "กิจการวิทยุ",
                "Radio Broadcasting",
                BusinessRestrictionList::List1Prohibited,
            ),
            Self::new(
                "1.4",
                "กิจการโทรทัศน์",
                "Television Broadcasting",
                BusinessRestrictionList::List1Prohibited,
            ),
            Self::new(
                "1.5",
                "การทำป่าไม้",
                "Forestry",
                BusinessRestrictionList::List1Prohibited,
            ),
        ]
    }

    /// Common List 2 activities (cabinet approval)
    pub fn list2_activities() -> Vec<Self> {
        vec![
            Self::new(
                "2.1",
                "การผลิตอาวุธ",
                "Weapons Manufacturing",
                BusinessRestrictionList::List2CabinetApproval,
            ),
            Self::new(
                "2.2",
                "การขนส่งทางบก",
                "Domestic Land Transport",
                BusinessRestrictionList::List2CabinetApproval,
            ),
            Self::new(
                "2.3",
                "การขนส่งทางน้ำ",
                "Domestic Water Transport",
                BusinessRestrictionList::List2CabinetApproval,
            ),
            Self::new(
                "2.4",
                "การขนส่งทางอากาศ",
                "Domestic Air Transport",
                BusinessRestrictionList::List2CabinetApproval,
            ),
        ]
    }

    /// Common List 3 activities (license required)
    pub fn list3_activities() -> Vec<Self> {
        vec![
            Self::new(
                "3.1",
                "การค้าปลีก",
                "Retail Trade",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.2",
                "การค้าส่ง",
                "Wholesale Trade",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.3",
                "การก่อสร้าง",
                "Construction",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.4",
                "บริการที่ปรึกษา",
                "Consulting Services",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.5",
                "บริการทางกฎหมาย",
                "Legal Services",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.6",
                "บริการทางบัญชี",
                "Accounting Services",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.7",
                "โรงแรม",
                "Hotel",
                BusinessRestrictionList::List3License,
            ),
            Self::new(
                "3.8",
                "ร้านอาหาร",
                "Restaurant",
                BusinessRestrictionList::List3License,
            ),
        ]
    }
}

/// Treaty exemptions from FBA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatyExemption {
    /// US Treaty of Amity (1966)
    UsTreatyOfAmity,

    /// ASEAN Framework Agreement
    AseanFramework,

    /// Japan-Thailand EPA
    JapanThailandEpa,

    /// Australia-Thailand FTA (TAFTA)
    AustraliaThailandFta,

    /// Other bilateral treaty
    OtherBilateral,
}

impl TreatyExemption {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::UsTreatyOfAmity => "สนธิสัญญาทางไมตรีและความสัมพันธ์ทางเศรษฐกิจไทย-สหรัฐ",
            Self::AseanFramework => "กรอบความตกลงอาเซียน",
            Self::JapanThailandEpa => "ความตกลงหุ้นส่วนเศรษฐกิจไทย-ญี่ปุ่น",
            Self::AustraliaThailandFta => "ความตกลงการค้าเสรีไทย-ออสเตรเลีย",
            Self::OtherBilateral => "สนธิสัญญาทวิภาคีอื่น",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::UsTreatyOfAmity => "US Treaty of Amity (1966)",
            Self::AseanFramework => "ASEAN Framework Agreement",
            Self::JapanThailandEpa => "Japan-Thailand EPA",
            Self::AustraliaThailandFta => "Australia-Thailand FTA (TAFTA)",
            Self::OtherBilateral => "Other Bilateral Treaty",
        }
    }

    /// Check if exemption applies to List 3
    pub fn exempts_list3(&self) -> bool {
        matches!(
            self,
            Self::UsTreatyOfAmity | Self::AseanFramework | Self::JapanThailandEpa
        )
    }

    /// Get applicable nationality
    pub fn applicable_nationality(&self) -> &'static str {
        match self {
            Self::UsTreatyOfAmity => "สหรัฐอเมริกา",
            Self::AseanFramework => "ประเทศสมาชิกอาเซียน",
            Self::JapanThailandEpa => "ญี่ปุ่น",
            Self::AustraliaThailandFta => "ออสเตรเลีย",
            Self::OtherBilateral => "ตามสนธิสัญญา",
        }
    }
}

/// Foreign investor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignInvestor {
    /// Investor name
    pub name: String,

    /// Nationality
    pub nationality: String,

    /// Share percentage
    pub share_percentage: f64,

    /// Treaty exemption if applicable
    pub treaty_exemption: Option<TreatyExemption>,

    /// BOI-promoted
    pub is_boi_promoted: bool,
}

impl ForeignInvestor {
    /// Create a new foreign investor
    pub fn new(
        name: impl Into<String>,
        nationality: impl Into<String>,
        share_percentage: f64,
    ) -> Self {
        Self {
            name: name.into(),
            nationality: nationality.into(),
            share_percentage,
            treaty_exemption: None,
            is_boi_promoted: false,
        }
    }

    /// Set treaty exemption
    pub fn with_treaty(mut self, treaty: TreatyExemption) -> Self {
        self.treaty_exemption = Some(treaty);
        self
    }

    /// Set BOI promotion
    pub fn with_boi(mut self) -> Self {
        self.is_boi_promoted = true;
        self
    }
}

/// Foreign ownership structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignOwnership {
    /// Total foreign share percentage
    pub total_foreign_percentage: f64,

    /// List of foreign investors
    pub investors: Vec<ForeignInvestor>,

    /// Whether structure uses nominees
    pub uses_nominees: bool,
}

impl ForeignOwnership {
    /// Create new ownership structure
    pub fn new(investors: Vec<ForeignInvestor>) -> Self {
        let total = investors.iter().map(|i| i.share_percentage).sum();
        Self {
            total_foreign_percentage: total,
            investors,
            uses_nominees: false,
        }
    }

    /// Check if exceeds 49% limit
    pub fn exceeds_limit(&self) -> bool {
        self.total_foreign_percentage > 49.0
    }

    /// Check if any investor has treaty exemption
    pub fn has_treaty_exemption(&self) -> bool {
        self.investors.iter().any(|i| i.treaty_exemption.is_some())
    }

    /// Check if any investor is BOI-promoted
    pub fn has_boi_promotion(&self) -> bool {
        self.investors.iter().any(|i| i.is_boi_promoted)
    }
}

/// Ownership structure analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipStructure {
    /// Foreign ownership details
    pub foreign_ownership: ForeignOwnership,

    /// Thai shareholding percentage
    pub thai_percentage: f64,

    /// Number of Thai shareholders
    pub thai_shareholder_count: u32,

    /// Minimum capital requirement (THB)
    pub minimum_capital: u64,

    /// Registered capital (THB)
    pub registered_capital: u64,
}

impl OwnershipStructure {
    /// Create new ownership structure
    pub fn new(
        foreign_ownership: ForeignOwnership,
        thai_shareholder_count: u32,
        registered_capital: u64,
    ) -> Self {
        let thai_percentage = 100.0 - foreign_ownership.total_foreign_percentage;
        // Minimum capital for List 3 is 3M THB
        let minimum_capital = 3_000_000;

        Self {
            foreign_ownership,
            thai_percentage,
            thai_shareholder_count,
            minimum_capital,
            registered_capital,
        }
    }

    /// Check if meets minimum capital requirement
    pub fn meets_capital_requirement(&self) -> bool {
        self.registered_capital >= self.minimum_capital
    }

    /// Check if Thai shareholding is sufficient
    pub fn thai_shareholding_sufficient(&self) -> bool {
        self.thai_percentage >= 51.0 && self.thai_shareholder_count >= 3
    }
}

/// Foreign business license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignBusinessLicense {
    /// License number
    pub license_number: String,

    /// Company name
    pub company_name: String,

    /// Business activity
    pub activity: BusinessActivity,

    /// Issue date
    pub issue_date: NaiveDate,

    /// Expiry date (if applicable)
    pub expiry_date: Option<NaiveDate>,

    /// Conditions
    pub conditions: Vec<String>,
}

impl ForeignBusinessLicense {
    /// Check if license is valid
    pub fn is_valid(&self, as_of: NaiveDate) -> bool {
        match self.expiry_date {
            Some(expiry) => as_of <= expiry,
            None => true,
        }
    }

    /// Get license citation
    pub fn citation(&self) -> String {
        let fba = ThaiAct::new(
            "ประกอบธุรกิจของคนต่างด้าว",
            "Foreign Business Act",
            BuddhistYear::from_be(2542),
        );
        fba.section(17).format_th()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restriction_list() {
        assert_eq!(
            BusinessRestrictionList::List1Prohibited.max_foreign_ownership(),
            0.0
        );
        assert_eq!(
            BusinessRestrictionList::List3License.max_foreign_ownership(),
            49.0
        );
        assert_eq!(
            BusinessRestrictionList::NotRestricted.max_foreign_ownership(),
            100.0
        );
    }

    #[test]
    fn test_treaty_exemption() {
        let treaty = TreatyExemption::UsTreatyOfAmity;
        assert!(treaty.exempts_list3());
        assert_eq!(treaty.applicable_nationality(), "สหรัฐอเมริกา");
    }

    #[test]
    fn test_foreign_ownership() {
        let investors = vec![
            ForeignInvestor::new("US Corp", "USA", 30.0),
            ForeignInvestor::new("JP Corp", "Japan", 15.0),
        ];
        let ownership = ForeignOwnership::new(investors);
        assert_eq!(ownership.total_foreign_percentage, 45.0);
        assert!(!ownership.exceeds_limit());
    }

    #[test]
    fn test_foreign_ownership_exceeds() {
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 60.0)];
        let ownership = ForeignOwnership::new(investors);
        assert!(ownership.exceeds_limit());
    }

    #[test]
    fn test_ownership_structure() {
        let investors = vec![ForeignInvestor::new("Foreign Corp", "USA", 40.0)];
        let ownership = ForeignOwnership::new(investors);
        let structure = OwnershipStructure::new(ownership, 3, 5_000_000);

        assert_eq!(structure.thai_percentage, 60.0);
        assert!(structure.thai_shareholding_sufficient());
        assert!(structure.meets_capital_requirement());
    }

    #[test]
    fn test_business_activities() {
        let list1 = BusinessActivity::list1_activities();
        assert!(!list1.is_empty());
        assert!(
            list1
                .iter()
                .all(|a| a.restriction_list == BusinessRestrictionList::List1Prohibited)
        );

        let list3 = BusinessActivity::list3_activities();
        assert!(!list3.is_empty());
        assert!(
            list3
                .iter()
                .all(|a| a.restriction_list == BusinessRestrictionList::List3License)
        );
    }

    #[test]
    fn test_investor_with_treaty() {
        let investor = ForeignInvestor::new("US Corp", "USA", 49.0)
            .with_treaty(TreatyExemption::UsTreatyOfAmity);
        assert!(investor.treaty_exemption.is_some());
    }

    #[test]
    fn test_license_validity() {
        let license = ForeignBusinessLicense {
            license_number: "FBA-001".to_string(),
            company_name: "Test Co.".to_string(),
            activity: BusinessActivity::new(
                "3.1",
                "การค้าปลีก",
                "Retail",
                BusinessRestrictionList::List3License,
            ),
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid"),
            expiry_date: Some(NaiveDate::from_ymd_opt(2029, 1, 1).expect("valid")),
            conditions: vec![],
        };

        assert!(license.is_valid(NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid")));
        assert!(!license.is_valid(NaiveDate::from_ymd_opt(2030, 1, 1).expect("valid")));
    }
}
