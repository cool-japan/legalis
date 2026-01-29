//! Book III: Specific Contracts - Civil and Commercial Code B.E. 2535
//!
//! Book III (มาตรา 538-925) covers specific types of contracts including:
//! - Sale (ซื้อขาย)
//! - Hire of work (จ้างทำของ)
//! - Lease (เช่าทรัพย์)
//! - Loan (ยืม)
//! - Suretyship (ค้ำประกัน)
//! - Pledge (จำนำ)
//! - Mortgage (จำนอง)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Types of specific contracts (ประเภทสัญญาเฉพาะ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificContract {
    /// Sale (ซื้อขาย) - Sections 453-502
    Sale,

    /// Exchange (แลกเปลี่ยน) - Section 503
    Exchange,

    /// Gift (ให้) - Sections 522-529
    Gift,

    /// Hire of work (จ้างทำของ) - Sections 587-603
    HireOfWork,

    /// Hire of service (จ้างแรงงาน) - Sections 575-586
    HireOfService,

    /// Lease (เช่าทรัพย์) - Sections 537-574
    Lease,

    /// Loan for use (ยืมใช้) - Sections 641-650
    LoanForUse,

    /// Loan for consumption (ยืมใช้สิ้นเปลือง) - Sections 651-657
    LoanForConsumption,

    /// Suretyship (ค้ำประกัน) - Sections 680-706
    Suretyship,

    /// Pledge (จำนำ) - Sections 747-782
    Pledge,

    /// Mortgage (จำนอง) - Sections 702-746
    Mortgage,

    /// Partnership (หุ้นส่วน) - Sections 1012-1078
    Partnership,
}

impl SpecificContract {
    /// Get Thai name
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Sale => "ซื้อขาย",
            Self::Exchange => "แลกเปลี่ยน",
            Self::Gift => "ให้",
            Self::HireOfWork => "จ้างทำของ",
            Self::HireOfService => "จ้างแรงงาน",
            Self::Lease => "เช่าทรัพย์",
            Self::LoanForUse => "ยืมใช้",
            Self::LoanForConsumption => "ยืมใช้สิ้นเปลือง",
            Self::Suretyship => "ค้ำประกัน",
            Self::Pledge => "จำนำ",
            Self::Mortgage => "จำนอง",
            Self::Partnership => "หุ้นส่วน",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Sale => "Sale",
            Self::Exchange => "Exchange",
            Self::Gift => "Gift",
            Self::HireOfWork => "Hire of Work",
            Self::HireOfService => "Hire of Service",
            Self::Lease => "Lease",
            Self::LoanForUse => "Loan for Use",
            Self::LoanForConsumption => "Loan for Consumption",
            Self::Suretyship => "Suretyship",
            Self::Pledge => "Pledge",
            Self::Mortgage => "Mortgage",
            Self::Partnership => "Partnership",
        }
    }

    /// Get starting section
    pub fn section_start(&self) -> u32 {
        match self {
            Self::Sale => 453,
            Self::Exchange => 503,
            Self::Gift => 522,
            Self::HireOfWork => 587,
            Self::HireOfService => 575,
            Self::Lease => 537,
            Self::LoanForUse => 641,
            Self::LoanForConsumption => 651,
            Self::Suretyship => 680,
            Self::Pledge => 747,
            Self::Mortgage => 702,
            Self::Partnership => 1012,
        }
    }

    /// Check if requires writing
    pub fn requires_writing(&self) -> bool {
        matches!(
            self,
            Self::Mortgage | Self::Pledge | Self::Suretyship | Self::Partnership
        )
    }
}

/// Seller's obligations (หน้าที่ของผู้ขาย)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SellerObligation {
    /// Deliver the goods (ส่งมอบสินค้า) - Section 456
    Deliver,

    /// Transfer ownership (โอนกรรมสิทธิ์) - Section 456
    TransferOwnership,

    /// Warrant against defects (รับประกันความชำรุดบกพร่อง) - Section 473
    WarrantDefects,

    /// Warrant against eviction (รับประกันจากการถูกริบ) - Section 466
    WarrantEviction,
}

impl SellerObligation {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Deliver => "ส่งมอบสินค้า",
            Self::TransferOwnership => "โอนกรรมสิทธิ์",
            Self::WarrantDefects => "รับประกันความชำรุดบกพร่อง",
            Self::WarrantEviction => "รับประกันจากการถูกริบ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Deliver => "Deliver Goods",
            Self::TransferOwnership => "Transfer Ownership",
            Self::WarrantDefects => "Warrant Against Defects",
            Self::WarrantEviction => "Warrant Against Eviction",
        }
    }
}

/// Buyer's obligations (หน้าที่ของผู้ซื้อ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuyerObligation {
    /// Pay the price (ชำระราคา) - Section 457
    PayPrice,

    /// Take delivery (รับมอบสินค้า) - Section 458
    TakeDelivery,

    /// Inspect goods (ตรวจสอบสินค้า) - Section 474
    InspectGoods,
}

impl BuyerObligation {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::PayPrice => "ชำระราคา",
            Self::TakeDelivery => "รับมอบสินค้า",
            Self::InspectGoods => "ตรวจสอบสินค้า",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PayPrice => "Pay the Price",
            Self::TakeDelivery => "Take Delivery",
            Self::InspectGoods => "Inspect Goods",
        }
    }
}

/// Lease types (ประเภทการเช่า)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseType {
    /// Immovable property lease (เช่าอสังหาริมทรัพย์) - Section 538
    Immovable,

    /// Movable property lease (เช่าสังหาริมทรัพย์) - Section 537
    Movable,

    /// Hire-purchase (เช่าซื้อ) - Section 574/1
    HirePurchase,
}

impl LeaseType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Immovable => "เช่าอสังหาริมทรัพย์",
            Self::Movable => "เช่าสังหาริมทรัพย์",
            Self::HirePurchase => "เช่าซื้อ",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Immovable => "Lease of Immovable Property",
            Self::Movable => "Lease of Movable Property",
            Self::HirePurchase => "Hire-Purchase",
        }
    }

    /// Maximum duration without registration
    pub fn max_duration_years(&self) -> Option<u32> {
        match self {
            Self::Immovable => Some(3), // Section 538 - over 3 years requires registration
            Self::Movable => None,
            Self::HirePurchase => None,
        }
    }
}

/// Security types (ประเภทหลักประกัน)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Mortgage (จำนอง) - Sections 702-746
    Mortgage,

    /// Pledge (จำนำ) - Sections 747-782
    Pledge,

    /// Suretyship (ค้ำประกัน) - Sections 680-706
    Suretyship,

    /// Lien (สิทธิยึดหน่วง) - Section 243
    Lien,
}

impl SecurityType {
    /// Get Thai description
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Mortgage => "จำนอง",
            Self::Pledge => "จำนำ",
            Self::Suretyship => "ค้ำประกัน",
            Self::Lien => "สิทธิยึดหน่วง",
        }
    }

    /// Get English description
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Mortgage => "Mortgage",
            Self::Pledge => "Pledge",
            Self::Suretyship => "Suretyship",
            Self::Lien => "Lien",
        }
    }

    /// Check if requires possession
    pub fn requires_possession(&self) -> bool {
        matches!(self, Self::Pledge | Self::Lien)
    }

    /// Check if requires registration
    pub fn requires_registration(&self) -> bool {
        matches!(self, Self::Mortgage)
    }
}

/// Sale contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleContract {
    /// Seller name
    pub seller: String,

    /// Buyer name
    pub buyer: String,

    /// Subject matter
    pub subject_matter: String,

    /// Price (THB)
    pub price: u64,

    /// Delivery date
    pub delivery_date: Option<NaiveDate>,

    /// Payment date
    pub payment_date: Option<NaiveDate>,

    /// Is in writing
    pub in_writing: bool,
}

impl SaleContract {
    /// Create a new sale contract
    pub fn new(seller: String, buyer: String, subject_matter: String, price: u64) -> Self {
        Self {
            seller,
            buyer,
            subject_matter,
            price,
            delivery_date: None,
            payment_date: None,
            in_writing: false,
        }
    }

    /// Check if price is specified
    pub fn has_price(&self) -> bool {
        self.price > 0
    }
}

/// Lease contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseContract {
    /// Lessor name
    pub lessor: String,

    /// Lessee name
    pub lessee: String,

    /// Property description
    pub property: String,

    /// Lease type
    pub lease_type: LeaseType,

    /// Monthly rent (THB)
    pub monthly_rent: u64,

    /// Duration in months
    pub duration_months: u32,

    /// Start date
    pub start_date: NaiveDate,

    /// Registered
    pub registered: bool,
}

impl LeaseContract {
    /// Create a new lease contract
    pub fn new(
        lessor: String,
        lessee: String,
        property: String,
        lease_type: LeaseType,
        monthly_rent: u64,
        duration_months: u32,
        start_date: NaiveDate,
    ) -> Self {
        Self {
            lessor,
            lessee,
            property,
            lease_type,
            monthly_rent,
            duration_months,
            start_date,
            registered: false,
        }
    }

    /// Calculate total rent
    pub fn total_rent(&self) -> u64 {
        self.monthly_rent * u64::from(self.duration_months)
    }

    /// Check if registration required
    pub fn requires_registration(&self) -> bool {
        if let Some(max_years) = self.lease_type.max_duration_years() {
            self.duration_months >= max_years * 12
        } else {
            false
        }
    }

    /// Calculate end date
    pub fn end_date(&self) -> Option<NaiveDate> {
        self.start_date
            .checked_add_months(chrono::Months::new(self.duration_months))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_contracts() {
        assert_eq!(SpecificContract::Sale.section_start(), 453);
        assert!(SpecificContract::Mortgage.requires_writing());
        assert!(!SpecificContract::Sale.requires_writing());
    }

    #[test]
    fn test_seller_obligations() {
        let ob = SellerObligation::Deliver;
        assert_eq!(ob.name_en(), "Deliver Goods");
    }

    #[test]
    fn test_lease_types() {
        assert_eq!(LeaseType::Immovable.max_duration_years(), Some(3));
        assert_eq!(LeaseType::Movable.max_duration_years(), None);
    }

    #[test]
    fn test_security_types() {
        assert!(SecurityType::Pledge.requires_possession());
        assert!(SecurityType::Mortgage.requires_registration());
        assert!(!SecurityType::Suretyship.requires_possession());
    }

    #[test]
    fn test_sale_contract() {
        let sale = SaleContract::new(
            "Seller A".to_string(),
            "Buyer B".to_string(),
            "Land".to_string(),
            1_000_000,
        );
        assert!(sale.has_price());
        assert_eq!(sale.price, 1_000_000);
    }

    #[test]
    fn test_lease_contract() {
        let lease = LeaseContract::new(
            "Lessor".to_string(),
            "Lessee".to_string(),
            "Office Space".to_string(),
            LeaseType::Immovable,
            50_000,
            36,
            NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
        );

        assert_eq!(lease.total_rent(), 1_800_000);
        assert!(lease.requires_registration()); // 36 months = 3 years, exactly at threshold
    }
}
