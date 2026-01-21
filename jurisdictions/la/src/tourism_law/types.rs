//! Tourism Law Types (ປະເພດກົດໝາຍທ່ອງທ່ຽວ)
//!
//! Comprehensive type definitions for Lao PDR tourism law system.
//!
//! ## Legal Basis
//!
//! - **Tourism Law 2013** (Law No. 32/NA, effective September 2013)
//! - **Tourism Development Strategic Plan**
//! - **ASEAN Tourism Agreement**
//!
//! ## Tourism Industry Structure in Lao PDR
//!
//! The tourism industry is organized into several categories:
//! - **Accommodation**: Hotels, guesthouses, resorts, eco-lodges
//! - **Tour Operators**: Inbound, outbound, domestic operators
//! - **Travel Agencies**: Booking and ticketing services
//! - **Tourism Transport**: Dedicated tourism vehicles
//! - **Tour Guides**: National and provincial guides
//! - **Tourism Attractions**: Natural and cultural sites

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants (ຄ່າຄົງທີ່)
// ============================================================================

/// Tourism business license validity period in years (Article 26)
/// ໄລຍະເວລາໃບອະນຸຍາດທຸລະກິດທ່ອງທ່ຽວ (ປີ)
pub const TOURISM_LICENSE_VALIDITY_YEARS: u8 = 3;

/// Tour guide license validity period in years (Article 38)
/// ໄລຍະເວລາໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວ (ປີ)
pub const GUIDE_LICENSE_VALIDITY_YEARS: u8 = 2;

/// Star rating classification validity period in years (Article 33)
/// ໄລຍະເວລາການຈັດລະດັບດາວ (ປີ)
pub const STAR_RATING_VALIDITY_YEARS: u8 = 3;

/// Maximum foreign ownership percentage for general tourism (Article 25)
/// ອັດຕາການເປັນເຈົ້າຂອງຕ່າງປະເທດສູງສຸດສຳລັບການທ່ອງທ່ຽວທົ່ວໄປ
pub const FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT: f64 = 49.0;

/// Maximum foreign ownership percentage for hotels (Article 25)
/// ອັດຕາການເປັນເຈົ້າຂອງຕ່າງປະເທດສູງສຸດສຳລັບໂຮງແຮມ
pub const FOREIGN_OWNERSHIP_HOTEL_MAX_PERCENT: f64 = 100.0;

/// Minimum rooms for 1-star hotel (Article 31)
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມ 1 ດາວ
pub const MIN_ROOMS_1_STAR: u32 = 10;

/// Minimum rooms for 2-star hotel (Article 31)
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມ 2 ດາວ
pub const MIN_ROOMS_2_STAR: u32 = 15;

/// Minimum rooms for 3-star hotel (Article 31)
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມ 3 ດາວ
pub const MIN_ROOMS_3_STAR: u32 = 30;

/// Minimum rooms for 4-star hotel (Article 31)
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມ 4 ດາວ
pub const MIN_ROOMS_4_STAR: u32 = 50;

/// Minimum rooms for 5-star hotel (Article 31)
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມ 5 ດາວ
pub const MIN_ROOMS_5_STAR: u32 = 100;

/// Minimum rooms for boutique hotel
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບໂຮງແຮມບູຕິກ
pub const MIN_ROOMS_BOUTIQUE: u32 = 5;

/// Maximum rooms for boutique hotel
/// ຈຳນວນຫ້ອງສູງສຸດສຳລັບໂຮງແຮມບູຕິກ
pub const MAX_ROOMS_BOUTIQUE: u32 = 50;

/// Minimum rooms for guesthouse
/// ຈຳນວນຫ້ອງຂັ້ນຕ່ຳສຳລັບເຮືອນພັກ
pub const MIN_ROOMS_GUESTHOUSE: u32 = 3;

/// Maximum rooms for guesthouse
/// ຈຳນວນຫ້ອງສູງສຸດສຳລັບເຮືອນພັກ
pub const MAX_ROOMS_GUESTHOUSE: u32 = 15;

/// Complaint response deadline in days (Article 50)
/// ກຳນົດເວລາຕອບຄຳຮ້ອງທຸກ (ມື້)
pub const COMPLAINT_RESPONSE_DEADLINE_DAYS: u32 = 15;

/// Tourism development fund contribution rate percent
/// ອັດຕາປະກອບສ່ວນກອງທຶນພັດທະນາການທ່ອງທ່ຽວ
pub const TOURISM_DEVELOPMENT_FUND_RATE_PERCENT: f64 = 1.0;

/// Minimum guide training hours (Article 37)
/// ຊົ່ວໂມງຝຶກອົບຮົມໄກດ໌ຂັ້ນຕ່ຳ
pub const MIN_GUIDE_TRAINING_HOURS: u32 = 120;

/// Visa on arrival validity days for tourists
/// ມື້ທີ່ວີຊາເມື່ອມາຮອດໃຊ້ໄດ້ສຳລັບນັກທ່ອງທ່ຽວ
pub const VISA_ON_ARRIVAL_VALIDITY_DAYS: u32 = 30;

/// ASEAN tourist visa-free stay days
/// ມື້ພັກຟຣີວີຊານັກທ່ອງທ່ຽວອາຊຽນ
pub const ASEAN_VISA_FREE_DAYS: u32 = 30;

// ============================================================================
// Tourism Enterprise Types (ປະເພດວິສາຫະກິດທ່ອງທ່ຽວ)
// ============================================================================

/// Tourism enterprise category (ປະເພດວິສາຫະກິດທ່ອງທ່ຽວ)
///
/// Tourism Law 2013, Article 22: Tourism enterprises are classified as follows
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TourismEnterpriseCategory {
    /// Accommodation (ທີ່ພັກ)
    Accommodation,

    /// Tour operator - inbound (ຜູ້ປະກອບການທົວ - ຂາເຂົ້າ)
    TourOperatorInbound,

    /// Tour operator - outbound (ຜູ້ປະກອບການທົວ - ຂາອອກ)
    TourOperatorOutbound,

    /// Tour operator - domestic (ຜູ້ປະກອບການທົວ - ພາຍໃນ)
    TourOperatorDomestic,

    /// Travel agency (ຕົວແທນທ່ອງທ່ຽວ)
    TravelAgency,

    /// Tourism transport (ການຂົນສົ່ງທ່ອງທ່ຽວ)
    TourismTransport,

    /// Tourist guide service (ການບໍລິການໄກດ໌ນຳທ່ຽວ)
    TouristGuideService,

    /// Tourism attraction (ສະຖານທີ່ທ່ອງທ່ຽວ)
    TourismAttraction,

    /// Restaurant and entertainment (ຮ້ານອາຫານ ແລະ ການບັນເທີງ)
    RestaurantEntertainment,

    /// Tourism souvenir (ຂອງທີ່ລະນຶກທ່ອງທ່ຽວ)
    TourismSouvenir,
}

impl TourismEnterpriseCategory {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Accommodation => "ທີ່ພັກ",
            Self::TourOperatorInbound => "ຜູ້ປະກອບການທົວຂາເຂົ້າ",
            Self::TourOperatorOutbound => "ຜູ້ປະກອບການທົວຂາອອກ",
            Self::TourOperatorDomestic => "ຜູ້ປະກອບການທົວພາຍໃນ",
            Self::TravelAgency => "ຕົວແທນທ່ອງທ່ຽວ",
            Self::TourismTransport => "ການຂົນສົ່ງທ່ອງທ່ຽວ",
            Self::TouristGuideService => "ການບໍລິການໄກດ໌ນຳທ່ຽວ",
            Self::TourismAttraction => "ສະຖານທີ່ທ່ອງທ່ຽວ",
            Self::RestaurantEntertainment => "ຮ້ານອາຫານແລະການບັນເທີງ",
            Self::TourismSouvenir => "ຂອງທີ່ລະນຶກທ່ອງທ່ຽວ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Accommodation => "Accommodation",
            Self::TourOperatorInbound => "Tour Operator (Inbound)",
            Self::TourOperatorOutbound => "Tour Operator (Outbound)",
            Self::TourOperatorDomestic => "Tour Operator (Domestic)",
            Self::TravelAgency => "Travel Agency",
            Self::TourismTransport => "Tourism Transport",
            Self::TouristGuideService => "Tourist Guide Service",
            Self::TourismAttraction => "Tourism Attraction",
            Self::RestaurantEntertainment => "Restaurant and Entertainment",
            Self::TourismSouvenir => "Tourism Souvenir",
        }
    }

    /// Get maximum foreign ownership percentage
    pub fn max_foreign_ownership_percent(&self) -> f64 {
        match self {
            Self::Accommodation => FOREIGN_OWNERSHIP_HOTEL_MAX_PERCENT,
            Self::TourOperatorInbound => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::TourOperatorOutbound => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::TourOperatorDomestic => 0.0, // Domestic only for Lao nationals
            Self::TravelAgency => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::TourismTransport => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::TouristGuideService => 0.0, // Lao nationals only
            Self::TourismAttraction => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::RestaurantEntertainment => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
            Self::TourismSouvenir => FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
        }
    }
}

/// License status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LicenseStatus {
    /// Active license (ໃບອະນຸຍາດໃຊ້ໄດ້)
    Active,

    /// Expired license (ໃບອະນຸຍາດໝົດອາຍຸ)
    Expired {
        /// Expiry date
        expired_on: DateTime<Utc>,
    },

    /// Suspended license (ໃບອະນຸຍາດຖືກລະງັບ)
    Suspended {
        /// Suspension date
        suspended_on: DateTime<Utc>,
        /// Reason for suspension
        reason: String,
        /// Expected reinstatement date
        reinstatement_date: Option<DateTime<Utc>>,
    },

    /// Revoked license (ໃບອະນຸຍາດຖືກຍົກເລີກ)
    Revoked {
        /// Revocation date
        revoked_on: DateTime<Utc>,
        /// Reason for revocation
        reason: String,
    },

    /// Pending approval (ລໍຖ້າອະນຸມັດ)
    Pending {
        /// Application date
        application_date: DateTime<Utc>,
    },

    /// Pending renewal (ລໍຖ້າຕໍ່ອາຍຸ)
    PendingRenewal {
        /// Renewal application date
        application_date: DateTime<Utc>,
    },
}

/// Tourism enterprise (ວິສາຫະກິດທ່ອງທ່ຽວ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TourismEnterprise {
    /// Enterprise name in Lao (ຊື່ວິສາຫະກິດເປັນພາສາລາວ)
    pub name_lao: String,

    /// Enterprise name in English (ຊື່ວິສາຫະກິດເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Enterprise category (ປະເພດວິສາຫະກິດ)
    pub category: TourismEnterpriseCategory,

    /// Business registration number (ເລກທີລົງທະບຽນທຸລະກິດ)
    pub registration_number: String,

    /// Tourism license number (ເລກທີໃບອະນຸຍາດທ່ອງທ່ຽວ)
    pub tourism_license_number: String,

    /// License status (ສະຖານະໃບອະນຸຍາດ)
    pub license_status: LicenseStatus,

    /// License issue date (ວັນທີອອກໃບອະນຸຍາດ)
    pub license_issue_date: DateTime<Utc>,

    /// License expiry date (ວັນທີໝົດອາຍຸໃບອະນຸຍາດ)
    pub license_expiry_date: DateTime<Utc>,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Address (ທີ່ຢູ່)
    pub address: String,

    /// Contact phone (ເບີໂທລະສັບ)
    pub contact_phone: Option<String>,

    /// Email (ອີເມວ)
    pub email: Option<String>,

    /// Website (ເວັບໄຊທ໌)
    pub website: Option<String>,

    /// Foreign ownership percentage (ອັດຕາການເປັນເຈົ້າຂອງຕ່າງປະເທດ)
    pub foreign_ownership_percent: f64,

    /// Registered capital in LAK (ທຶນຈົດທະບຽນເປັນກີບ)
    pub registered_capital_lak: u64,

    /// Number of employees (ຈຳນວນພະນັກງານ)
    pub employee_count: u32,

    /// Lao employee count (ຈຳນວນພະນັກງານລາວ)
    pub lao_employee_count: u32,
}

impl TourismEnterprise {
    /// Check if license is currently valid
    pub fn is_license_valid(&self) -> bool {
        matches!(self.license_status, LicenseStatus::Active)
            && Utc::now() < self.license_expiry_date
    }

    /// Get days until license expiry
    pub fn days_until_expiry(&self) -> i64 {
        (self.license_expiry_date - Utc::now()).num_days()
    }

    /// Check if license needs renewal soon (within 90 days)
    pub fn needs_renewal_soon(&self) -> bool {
        let days = self.days_until_expiry();
        days > 0 && days <= 90
    }

    /// Check if foreign ownership is within limits
    pub fn is_foreign_ownership_valid(&self) -> bool {
        self.foreign_ownership_percent <= self.category.max_foreign_ownership_percent()
    }
}

// ============================================================================
// Accommodation Types (ປະເພດທີ່ພັກ)
// ============================================================================

/// Accommodation type (ປະເພດທີ່ພັກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccommodationType {
    /// Hotel (ໂຮງແຮມ)
    Hotel,

    /// Resort (ລີສອດ)
    Resort,

    /// Boutique hotel (ໂຮງແຮມບູຕິກ)
    BoutiqueHotel,

    /// Guesthouse (ເຮືອນພັກ)
    Guesthouse,

    /// Eco-lodge (ອີໂຄລອດຈ໌)
    EcoLodge,

    /// Hostel (ໂຮສເທລ)
    Hostel,

    /// Homestay (ໂຮມສະເຕ)
    Homestay,

    /// Serviced apartment (ອາພາດເມັນບໍລິການ)
    ServicedApartment,

    /// Motel (ໂມເທລ)
    Motel,

    /// Camping site (ສະຖານທີ່ຕັ້ງແຄັມ)
    CampingSite,
}

impl AccommodationType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Hotel => "ໂຮງແຮມ",
            Self::Resort => "ລີສອດ",
            Self::BoutiqueHotel => "ໂຮງແຮມບູຕິກ",
            Self::Guesthouse => "ເຮືອນພັກ",
            Self::EcoLodge => "ອີໂຄລອດຈ໌",
            Self::Hostel => "ໂຮສເທລ",
            Self::Homestay => "ໂຮມສະເຕ",
            Self::ServicedApartment => "ອາພາດເມັນບໍລິການ",
            Self::Motel => "ໂມເທລ",
            Self::CampingSite => "ສະຖານທີ່ຕັ້ງແຄັມ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Hotel => "Hotel",
            Self::Resort => "Resort",
            Self::BoutiqueHotel => "Boutique Hotel",
            Self::Guesthouse => "Guesthouse",
            Self::EcoLodge => "Eco-Lodge",
            Self::Hostel => "Hostel",
            Self::Homestay => "Homestay",
            Self::ServicedApartment => "Serviced Apartment",
            Self::Motel => "Motel",
            Self::CampingSite => "Camping Site",
        }
    }

    /// Check if star rating is applicable
    pub fn star_rating_applicable(&self) -> bool {
        matches!(self, Self::Hotel | Self::Resort | Self::BoutiqueHotel)
    }
}

/// Star rating (ລະດັບດາວ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StarRating {
    /// 1 star (1 ດາວ)
    OneStar = 1,

    /// 2 stars (2 ດາວ)
    TwoStar = 2,

    /// 3 stars (3 ດາວ)
    ThreeStar = 3,

    /// 4 stars (4 ດາວ)
    FourStar = 4,

    /// 5 stars (5 ດາວ)
    FiveStar = 5,
}

impl StarRating {
    /// Get minimum room count for this rating
    pub fn minimum_rooms(&self) -> u32 {
        match self {
            Self::OneStar => MIN_ROOMS_1_STAR,
            Self::TwoStar => MIN_ROOMS_2_STAR,
            Self::ThreeStar => MIN_ROOMS_3_STAR,
            Self::FourStar => MIN_ROOMS_4_STAR,
            Self::FiveStar => MIN_ROOMS_5_STAR,
        }
    }

    /// Get the numeric value
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Create from numeric value
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::OneStar),
            2 => Some(Self::TwoStar),
            3 => Some(Self::ThreeStar),
            4 => Some(Self::FourStar),
            5 => Some(Self::FiveStar),
            _ => None,
        }
    }
}

/// Hotel facility (ສິ່ງອຳນວຍຄວາມສະດວກໂຮງແຮມ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HotelFacility {
    /// Restaurant (ຮ້ານອາຫານ)
    Restaurant,

    /// Swimming pool (ສະລອຍນ້ຳ)
    SwimmingPool,

    /// Spa (ສະປາ)
    Spa,

    /// Fitness center (ສູນອອກກຳລັງກາຍ)
    FitnessCenter,

    /// Business center (ສູນທຸລະກິດ)
    BusinessCenter,

    /// Conference room (ຫ້ອງປະຊຸມ)
    ConferenceRoom,

    /// Airport shuttle (ລົດຮັບສົ່ງສະໜາມບິນ)
    AirportShuttle,

    /// Room service (ບໍລິການຫ້ອງພັກ)
    RoomService,

    /// Laundry service (ບໍລິການຊັກລີດ)
    LaundryService,

    /// WiFi (ວາຍຟາຍ)
    WiFi,

    /// Parking (ບ່ອນຈອດລົດ)
    Parking,

    /// Air conditioning (ເຄື່ອງປັບອາກາດ)
    AirConditioning,

    /// 24-hour reception (ຕ້ອນຮັບ 24 ຊົ່ວໂມງ)
    Reception24Hour,

    /// Elevator (ລິຟ)
    Elevator,

    /// Bar/Lounge (ບາ/ລາວຈ໌)
    BarLounge,

    /// Currency exchange (ແລກປ່ຽນເງິນຕາ)
    CurrencyExchange,

    /// Gift shop (ຮ້ານຂອງຂວັນ)
    GiftShop,

    /// Garden (ສວນ)
    Garden,

    /// Kids club (ສະໂມສອນເດັກ)
    KidsClub,

    /// Medical service (ບໍລິການແພດ)
    MedicalService,
}

impl HotelFacility {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Restaurant => "ຮ້ານອາຫານ",
            Self::SwimmingPool => "ສະລອຍນ້ຳ",
            Self::Spa => "ສະປາ",
            Self::FitnessCenter => "ສູນອອກກຳລັງກາຍ",
            Self::BusinessCenter => "ສູນທຸລະກິດ",
            Self::ConferenceRoom => "ຫ້ອງປະຊຸມ",
            Self::AirportShuttle => "ລົດຮັບສົ່ງສະໜາມບິນ",
            Self::RoomService => "ບໍລິການຫ້ອງພັກ",
            Self::LaundryService => "ບໍລິການຊັກລີດ",
            Self::WiFi => "ວາຍຟາຍ",
            Self::Parking => "ບ່ອນຈອດລົດ",
            Self::AirConditioning => "ເຄື່ອງປັບອາກາດ",
            Self::Reception24Hour => "ຕ້ອນຮັບ 24 ຊົ່ວໂມງ",
            Self::Elevator => "ລິຟ",
            Self::BarLounge => "ບາ/ລາວຈ໌",
            Self::CurrencyExchange => "ແລກປ່ຽນເງິນຕາ",
            Self::GiftShop => "ຮ້ານຂອງຂວັນ",
            Self::Garden => "ສວນ",
            Self::KidsClub => "ສະໂມສອນເດັກ",
            Self::MedicalService => "ບໍລິການແພດ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Restaurant => "Restaurant",
            Self::SwimmingPool => "Swimming Pool",
            Self::Spa => "Spa",
            Self::FitnessCenter => "Fitness Center",
            Self::BusinessCenter => "Business Center",
            Self::ConferenceRoom => "Conference Room",
            Self::AirportShuttle => "Airport Shuttle",
            Self::RoomService => "Room Service",
            Self::LaundryService => "Laundry Service",
            Self::WiFi => "WiFi",
            Self::Parking => "Parking",
            Self::AirConditioning => "Air Conditioning",
            Self::Reception24Hour => "24-Hour Reception",
            Self::Elevator => "Elevator",
            Self::BarLounge => "Bar/Lounge",
            Self::CurrencyExchange => "Currency Exchange",
            Self::GiftShop => "Gift Shop",
            Self::Garden => "Garden",
            Self::KidsClub => "Kids Club",
            Self::MedicalService => "Medical Service",
        }
    }
}

/// Hotel classification status (ສະຖານະການຈັດລະດັບໂຮງແຮມ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HotelClassificationStatus {
    /// Classified (ຈັດລະດັບແລ້ວ)
    Classified {
        /// Classification date
        classification_date: DateTime<Utc>,
        /// Expiry date
        expiry_date: DateTime<Utc>,
        /// Classifying authority
        authority: String,
    },

    /// Pending classification (ລໍຖ້າການຈັດລະດັບ)
    Pending {
        /// Application date
        application_date: DateTime<Utc>,
    },

    /// Classification expired (ການຈັດລະດັບໝົດອາຍຸ)
    Expired {
        /// Previous rating
        previous_rating: StarRating,
        /// Expiry date
        expired_on: DateTime<Utc>,
    },

    /// Not classified (ບໍ່ໄດ້ຈັດລະດັບ)
    NotClassified,
}

/// Accommodation establishment (ສະຖານທີ່ພັກ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Accommodation {
    /// Base enterprise information
    pub enterprise: TourismEnterprise,

    /// Accommodation type (ປະເພດທີ່ພັກ)
    pub accommodation_type: AccommodationType,

    /// Star rating (if applicable) (ລະດັບດາວ)
    pub star_rating: Option<StarRating>,

    /// Classification status (ສະຖານະການຈັດລະດັບ)
    pub classification_status: HotelClassificationStatus,

    /// Number of rooms (ຈຳນວນຫ້ອງ)
    pub room_count: u32,

    /// Number of beds (ຈຳນວນຕຽງ)
    pub bed_count: u32,

    /// Available facilities (ສິ່ງອຳນວຍຄວາມສະດວກ)
    pub facilities: Vec<HotelFacility>,

    /// Check-in time (ເວລາເຊັກອິນ)
    pub check_in_time: Option<String>,

    /// Check-out time (ເວລາເຊັກເອົ້າ)
    pub check_out_time: Option<String>,

    /// Average room rate in LAK (ລາຄາຫ້ອງສະເລ່ຍ)
    pub average_room_rate_lak: Option<u64>,
}

impl Accommodation {
    /// Check if room count meets star rating requirement
    pub fn meets_room_requirement(&self) -> bool {
        if let Some(rating) = self.star_rating {
            self.room_count >= rating.minimum_rooms()
        } else {
            true // No rating means no room requirement
        }
    }

    /// Check if classification is valid
    pub fn is_classification_valid(&self) -> bool {
        matches!(
            self.classification_status,
            HotelClassificationStatus::Classified { .. }
        )
    }
}

/// Accommodation builder for constructing Accommodation instances
#[derive(Debug, Default)]
pub struct AccommodationBuilder {
    enterprise: Option<TourismEnterprise>,
    accommodation_type: Option<AccommodationType>,
    star_rating: Option<StarRating>,
    classification_status: Option<HotelClassificationStatus>,
    room_count: Option<u32>,
    bed_count: Option<u32>,
    facilities: Vec<HotelFacility>,
    check_in_time: Option<String>,
    check_out_time: Option<String>,
    average_room_rate_lak: Option<u64>,
}

impl AccommodationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the enterprise
    pub fn enterprise(mut self, enterprise: TourismEnterprise) -> Self {
        self.enterprise = Some(enterprise);
        self
    }

    /// Set the accommodation type
    pub fn accommodation_type(mut self, accommodation_type: AccommodationType) -> Self {
        self.accommodation_type = Some(accommodation_type);
        self
    }

    /// Set the star rating
    pub fn star_rating(mut self, rating: StarRating) -> Self {
        self.star_rating = Some(rating);
        self
    }

    /// Set the classification status
    pub fn classification_status(mut self, status: HotelClassificationStatus) -> Self {
        self.classification_status = Some(status);
        self
    }

    /// Set the room count
    pub fn room_count(mut self, count: u32) -> Self {
        self.room_count = Some(count);
        self
    }

    /// Set the bed count
    pub fn bed_count(mut self, count: u32) -> Self {
        self.bed_count = Some(count);
        self
    }

    /// Add facilities
    pub fn facilities(mut self, facilities: Vec<HotelFacility>) -> Self {
        self.facilities = facilities;
        self
    }

    /// Set check-in time
    pub fn check_in_time(mut self, time: String) -> Self {
        self.check_in_time = Some(time);
        self
    }

    /// Set check-out time
    pub fn check_out_time(mut self, time: String) -> Self {
        self.check_out_time = Some(time);
        self
    }

    /// Set average room rate
    pub fn average_room_rate_lak(mut self, rate: u64) -> Self {
        self.average_room_rate_lak = Some(rate);
        self
    }

    /// Build the Accommodation
    pub fn build(self) -> Option<Accommodation> {
        Some(Accommodation {
            enterprise: self.enterprise?,
            accommodation_type: self.accommodation_type?,
            star_rating: self.star_rating,
            classification_status: self
                .classification_status
                .unwrap_or(HotelClassificationStatus::NotClassified),
            room_count: self.room_count?,
            bed_count: self.bed_count.unwrap_or(0),
            facilities: self.facilities,
            check_in_time: self.check_in_time,
            check_out_time: self.check_out_time,
            average_room_rate_lak: self.average_room_rate_lak,
        })
    }
}

// ============================================================================
// Tour Guide Types (ປະເພດໄກດ໌ນຳທ່ຽວ)
// ============================================================================

/// Tour guide license category (ປະເພດໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວ)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GuideLicenseCategory {
    /// National guide (ໄກດ໌ລະດັບຊາດ)
    National,

    /// Provincial guide (ໄກດ໌ລະດັບແຂວງ)
    Provincial,

    /// Specialized guide (ໄກດ໌ສະເພາະ)
    Specialized {
        /// Specialization type
        specialization: String,
    },

    /// Community guide (ໄກດ໌ຊຸມຊົນ)
    Community,
}

impl GuideLicenseCategory {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::National => "ໄກດ໌ລະດັບຊາດ",
            Self::Provincial => "ໄກດ໌ລະດັບແຂວງ",
            Self::Specialized { .. } => "ໄກດ໌ສະເພາະ",
            Self::Community => "ໄກດ໌ຊຸມຊົນ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::National => "National Guide",
            Self::Provincial => "Provincial Guide",
            Self::Specialized { .. } => "Specialized Guide",
            Self::Community => "Community Guide",
        }
    }
}

/// Language proficiency level (ລະດັບຄວາມສາມາດທາງພາສາ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LanguageProficiency {
    /// Basic (ພື້ນຖານ)
    Basic = 1,

    /// Intermediate (ລະດັບກາງ)
    Intermediate = 2,

    /// Advanced (ລະດັບສູງ)
    Advanced = 3,

    /// Fluent (ຄ່ອງແຄ້ວ)
    Fluent = 4,

    /// Native (ພາສາແມ່)
    Native = 5,
}

impl LanguageProficiency {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::Basic => "ພື້ນຖານ",
            Self::Intermediate => "ລະດັບກາງ",
            Self::Advanced => "ລະດັບສູງ",
            Self::Fluent => "ຄ່ອງແຄ້ວ",
            Self::Native => "ພາສາແມ່",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Fluent => "Fluent",
            Self::Native => "Native",
        }
    }
}

/// Language skill (ທັກສະພາສາ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LanguageSkill {
    /// Language name (ຊື່ພາສາ)
    pub language: String,

    /// Proficiency level (ລະດັບຄວາມສາມາດ)
    pub proficiency: LanguageProficiency,

    /// Certification (if any) (ໃບຢັ້ງຢືນ)
    pub certification: Option<String>,
}

/// Tour guide (ໄກດ໌ນຳທ່ຽວ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TourGuide {
    /// Name (ຊື່)
    pub name: String,

    /// Name in Lao (ຊື່ເປັນພາສາລາວ)
    pub name_lao: Option<String>,

    /// License number (ເລກທີໃບອະນຸຍາດ)
    pub license_number: String,

    /// License category (ປະເພດໃບອະນຸຍາດ)
    pub license_category: GuideLicenseCategory,

    /// License status (ສະຖານະໃບອະນຸຍາດ)
    pub license_status: LicenseStatus,

    /// License issue date (ວັນທີອອກໃບອະນຸຍາດ)
    pub license_issue_date: DateTime<Utc>,

    /// License expiry date (ວັນທີໝົດອາຍຸໃບອະນຸຍາດ)
    pub license_expiry_date: DateTime<Utc>,

    /// Language skills (ທັກສະພາສາ)
    pub language_skills: Vec<LanguageSkill>,

    /// Training certifications (ໃບຢັ້ງຢືນການຝຶກອົບຮົມ)
    pub training_certifications: Vec<String>,

    /// Training hours completed (ຊົ່ວໂມງຝຶກອົບຮົມສຳເລັດ)
    pub training_hours: u32,

    /// Province (for provincial guides) (ແຂວງ)
    pub province: Option<String>,

    /// Affiliated company (if any) (ບໍລິສັດທີ່ສັງກັດ)
    pub affiliated_company: Option<String>,

    /// Years of experience (ປະສົບການ)
    pub years_of_experience: u32,

    /// Nationality (ສັນຊາດ)
    pub nationality: String,

    /// Date of birth (ວັນເດືອນປີເກີດ)
    pub date_of_birth: DateTime<Utc>,

    /// ID card number (ເລກບັດປະຈຳຕົວ)
    pub id_card_number: String,
}

impl TourGuide {
    /// Check if license is currently valid
    pub fn is_license_valid(&self) -> bool {
        matches!(self.license_status, LicenseStatus::Active)
            && Utc::now() < self.license_expiry_date
    }

    /// Check if guide has required Lao proficiency
    pub fn has_lao_proficiency(&self) -> bool {
        self.language_skills.iter().any(|skill| {
            skill.language.to_lowercase() == "lao"
                && skill.proficiency >= LanguageProficiency::Fluent
        })
    }

    /// Check if guide has required foreign language proficiency
    pub fn has_foreign_language(&self) -> bool {
        self.language_skills.iter().any(|skill| {
            skill.language.to_lowercase() != "lao"
                && skill.proficiency >= LanguageProficiency::Intermediate
        })
    }

    /// Check if training requirements are met
    pub fn meets_training_requirements(&self) -> bool {
        self.training_hours >= MIN_GUIDE_TRAINING_HOURS
    }
}

// ============================================================================
// Tourism Zone Types (ປະເພດເຂດທ່ອງທ່ຽວ)
// ============================================================================

/// Tourism zone type (ປະເພດເຂດທ່ອງທ່ຽວ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TourismZoneType {
    /// National tourism zone (ເຂດທ່ອງທ່ຽວແຫ່ງຊາດ)
    NationalTourismZone,

    /// Provincial tourism development area (ເຂດພັດທະນາການທ່ອງທ່ຽວແຂວງ)
    ProvincialDevelopmentArea,

    /// Heritage tourism site (ສະຖານທີ່ທ່ອງທ່ຽວມໍລະດົກ)
    HeritageSite,

    /// Ecotourism area (ເຂດທ່ອງທ່ຽວນິເວດ)
    EcotourismArea,

    /// Special economic zone for tourism (ເຂດເສດຖະກິດພິເສດການທ່ອງທ່ຽວ)
    SpecialEconomicZone,

    /// Community-based tourism area (ເຂດທ່ອງທ່ຽວໂດຍຊຸມຊົນ)
    CommunityBasedTourismArea,

    /// Protected natural area (ເຂດອະນຸລັກທຳມະຊາດ)
    ProtectedNaturalArea,

    /// Cultural tourism zone (ເຂດທ່ອງທ່ຽວວັດທະນະທຳ)
    CulturalTourismZone,

    /// Adventure tourism zone (ເຂດທ່ອງທ່ຽວຜະຈົນໄພ)
    AdventureTourismZone,

    /// Security/restricted zone (ເຂດຄວາມໝັ້ນຄົງ/ຈຳກັດ)
    SecurityZone,
}

impl TourismZoneType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::NationalTourismZone => "ເຂດທ່ອງທ່ຽວແຫ່ງຊາດ",
            Self::ProvincialDevelopmentArea => "ເຂດພັດທະນາການທ່ອງທ່ຽວແຂວງ",
            Self::HeritageSite => "ສະຖານທີ່ທ່ອງທ່ຽວມໍລະດົກ",
            Self::EcotourismArea => "ເຂດທ່ອງທ່ຽວນິເວດ",
            Self::SpecialEconomicZone => "ເຂດເສດຖະກິດພິເສດການທ່ອງທ່ຽວ",
            Self::CommunityBasedTourismArea => "ເຂດທ່ອງທ່ຽວໂດຍຊຸມຊົນ",
            Self::ProtectedNaturalArea => "ເຂດອະນຸລັກທຳມະຊາດ",
            Self::CulturalTourismZone => "ເຂດທ່ອງທ່ຽວວັດທະນະທຳ",
            Self::AdventureTourismZone => "ເຂດທ່ອງທ່ຽວຜະຈົນໄພ",
            Self::SecurityZone => "ເຂດຄວາມໝັ້ນຄົງ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::NationalTourismZone => "National Tourism Zone",
            Self::ProvincialDevelopmentArea => "Provincial Tourism Development Area",
            Self::HeritageSite => "Heritage Tourism Site",
            Self::EcotourismArea => "Ecotourism Area",
            Self::SpecialEconomicZone => "Special Economic Zone (Tourism)",
            Self::CommunityBasedTourismArea => "Community-Based Tourism Area",
            Self::ProtectedNaturalArea => "Protected Natural Area",
            Self::CulturalTourismZone => "Cultural Tourism Zone",
            Self::AdventureTourismZone => "Adventure Tourism Zone",
            Self::SecurityZone => "Security/Restricted Zone",
        }
    }

    /// Check if special permit is required
    pub fn requires_special_permit(&self) -> bool {
        matches!(
            self,
            Self::ProtectedNaturalArea | Self::SecurityZone | Self::HeritageSite
        )
    }

    /// Check if zone is open to foreign tourists
    pub fn open_to_foreign_tourists(&self) -> bool {
        !matches!(self, Self::SecurityZone)
    }
}

/// Tourism zone (ເຂດທ່ອງທ່ຽວ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TourismZone {
    /// Zone name in Lao (ຊື່ເຂດເປັນພາສາລາວ)
    pub name_lao: String,

    /// Zone name in English (ຊື່ເຂດເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Zone type (ປະເພດເຂດ)
    pub zone_type: TourismZoneType,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: Option<f64>,

    /// Carrying capacity (visitors per day) (ຄວາມສາມາດຮອງຮັບ)
    pub carrying_capacity: Option<u32>,

    /// Entrance fee for Lao nationals (LAK) (ຄ່າເຂົ້າຊົມສຳລັບຄົນລາວ)
    pub entrance_fee_lao_lak: Option<u64>,

    /// Entrance fee for foreigners (LAK) (ຄ່າເຂົ້າຊົມສຳລັບຄົນຕ່າງປະເທດ)
    pub entrance_fee_foreign_lak: Option<u64>,

    /// Special permit required (ຕ້ອງການໃບອະນຸຍາດພິເສດ)
    pub permit_required: bool,

    /// Managing authority (ໜ່ວຍງານຄຸ້ມຄອງ)
    pub managing_authority: String,

    /// UNESCO World Heritage status (ສະຖານະມໍລະດົກໂລກຢູເນສໂກ)
    pub unesco_world_heritage: bool,

    /// Description (ຄຳອະທິບາຍ)
    pub description: Option<String>,
}

impl TourismZone {
    /// Check if zone is currently open
    pub fn is_open(&self) -> bool {
        !matches!(self.zone_type, TourismZoneType::SecurityZone)
    }
}

// ============================================================================
// Tourist Rights Types (ປະເພດສິດນັກທ່ອງທ່ຽວ)
// ============================================================================

/// Tourist complaint status (ສະຖານະຄຳຮ້ອງທຸກນັກທ່ອງທ່ຽວ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComplaintStatus {
    /// Filed (ຍື່ນແລ້ວ)
    Filed {
        /// Filing date
        filed_on: DateTime<Utc>,
    },

    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation {
        /// Investigation start date
        started_on: DateTime<Utc>,
    },

    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved {
        /// Resolution date
        resolved_on: DateTime<Utc>,
        /// Resolution description
        resolution: String,
    },

    /// Dismissed (ຖືກປະຕິເສດ)
    Dismissed {
        /// Dismissal date
        dismissed_on: DateTime<Utc>,
        /// Reason for dismissal
        reason: String,
    },

    /// Pending response (ລໍຖ້າການຕອບ)
    PendingResponse,
}

/// Tourist complaint (ຄຳຮ້ອງທຸກນັກທ່ອງທ່ຽວ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TouristComplaint {
    /// Complaint ID (ລະຫັດຄຳຮ້ອງທຸກ)
    pub complaint_id: String,

    /// Tourist name (ຊື່ນັກທ່ອງທ່ຽວ)
    pub tourist_name: String,

    /// Tourist nationality (ສັນຊາດນັກທ່ອງທ່ຽວ)
    pub tourist_nationality: String,

    /// Complaint against (enterprise/person) (ຮ້ອງທຸກຕໍ່)
    pub complaint_against: String,

    /// Complaint description (ຄຳອະທິບາຍຄຳຮ້ອງທຸກ)
    pub description: String,

    /// Complaint status (ສະຖານະຄຳຮ້ອງທຸກ)
    pub status: ComplaintStatus,

    /// Filing date (ວັນທີຍື່ນ)
    pub filing_date: DateTime<Utc>,

    /// Category (ປະເພດ)
    pub category: String,

    /// Evidence provided (ຫຼັກຖານທີ່ສະໜອງ)
    pub evidence: Vec<String>,
}

/// Travel insurance (ປະກັນການເດີນທາງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TravelInsurance {
    /// Policy number (ເລກທີນະໂຍບາຍ)
    pub policy_number: String,

    /// Insurance company (ບໍລິສັດປະກັນ)
    pub insurance_company: String,

    /// Coverage amount in USD (ຈຳນວນການຄຸ້ມຄອງເປັນ USD)
    pub coverage_amount_usd: u64,

    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: DateTime<Utc>,

    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: DateTime<Utc>,

    /// Covers medical emergencies (ຄຸ້ມຄອງສຸກເສີນທາງການແພດ)
    pub covers_medical: bool,

    /// Covers evacuation (ຄຸ້ມຄອງການອົບພະຍົບ)
    pub covers_evacuation: bool,

    /// Covers trip cancellation (ຄຸ້ມຄອງການຍົກເລີກການເດີນທາງ)
    pub covers_cancellation: bool,
}

impl TravelInsurance {
    /// Check if insurance is currently valid
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.start_date && now < self.end_date
    }
}

// ============================================================================
// ASEAN Integration Types (ປະເພດການເຊື່ອມໂຍງອາຊຽນ)
// ============================================================================

/// ASEAN tourism professional type (ປະເພດບຸກຄະລາກອນທ່ອງທ່ຽວອາຊຽນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AseanTourismProfessional {
    /// Hotel manager (ຜູ້ຈັດການໂຮງແຮມ)
    HotelManager,

    /// Front office manager (ຜູ້ຈັດການຕ້ອນຮັບ)
    FrontOfficeManager,

    /// Housekeeper (ແມ່ບ້ານ)
    Housekeeper,

    /// Travel consultant (ທີ່ປຶກສາການເດີນທາງ)
    TravelConsultant,

    /// Tour guide (ໄກດ໌ນຳທ່ຽວ)
    TourGuide,

    /// Food and beverage manager (ຜູ້ຈັດການອາຫານ ແລະ ເຄື່ອງດື່ມ)
    FoodBeverageManager,
}

impl AseanTourismProfessional {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::HotelManager => "ຜູ້ຈັດການໂຮງແຮມ",
            Self::FrontOfficeManager => "ຜູ້ຈັດການຕ້ອນຮັບ",
            Self::Housekeeper => "ແມ່ບ້ານ",
            Self::TravelConsultant => "ທີ່ປຶກສາການເດີນທາງ",
            Self::TourGuide => "ໄກດ໌ນຳທ່ຽວ",
            Self::FoodBeverageManager => "ຜູ້ຈັດການອາຫານແລະເຄື່ອງດື່ມ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::HotelManager => "Hotel Manager",
            Self::FrontOfficeManager => "Front Office Manager",
            Self::Housekeeper => "Housekeeper",
            Self::TravelConsultant => "Travel Consultant",
            Self::TourGuide => "Tour Guide",
            Self::FoodBeverageManager => "Food & Beverage Manager",
        }
    }
}

/// ASEAN MRA certification (ໃບຢັ້ງຢືນ MRA ອາຊຽນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AseanMraCertification {
    /// Certificate number (ເລກທີໃບຢັ້ງຢືນ)
    pub certificate_number: String,

    /// Professional type (ປະເພດບຸກຄະລາກອນ)
    pub professional_type: AseanTourismProfessional,

    /// Issue country (ປະເທດອອກ)
    pub issue_country: String,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: DateTime<Utc>,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: DateTime<Utc>,

    /// Holder name (ຊື່ຜູ້ຖື)
    pub holder_name: String,
}

impl AseanMraCertification {
    /// Check if certification is valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expiry_date
    }
}

// ============================================================================
// Visa Types (ປະເພດວີຊາ)
// ============================================================================

/// Visa type for tourism (ປະເພດວີຊາສຳລັບການທ່ອງທ່ຽວ)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TourismVisaType {
    /// Tourist visa (T-B) (ວີຊານັກທ່ອງທ່ຽວ)
    TouristVisa,

    /// Visa on arrival (ວີຊາເມື່ອມາຮອດ)
    VisaOnArrival,

    /// E-visa (ວີຊາອີເລັກໂຕຣນິກ)
    EVisa,

    /// Visa exemption (ການຍົກເວັ້ນວີຊາ)
    VisaExemption {
        /// Country eligible
        country: String,
        /// Duration in days
        duration_days: u32,
    },

    /// ASEAN visa-free (ຟຣີວີຊາອາຊຽນ)
    AseanVisaFree,

    /// Transit visa (ວີຊາຜ່ານທາງ)
    TransitVisa,
}

impl TourismVisaType {
    /// Get description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            Self::TouristVisa => "ວີຊານັກທ່ອງທ່ຽວ",
            Self::VisaOnArrival => "ວີຊາເມື່ອມາຮອດ",
            Self::EVisa => "ວີຊາອີເລັກໂຕຣນິກ",
            Self::VisaExemption { .. } => "ການຍົກເວັ້ນວີຊາ",
            Self::AseanVisaFree => "ຟຣີວີຊາອາຊຽນ",
            Self::TransitVisa => "ວີຊາຜ່ານທາງ",
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::TouristVisa => "Tourist Visa",
            Self::VisaOnArrival => "Visa on Arrival",
            Self::EVisa => "E-Visa",
            Self::VisaExemption { .. } => "Visa Exemption",
            Self::AseanVisaFree => "ASEAN Visa-Free",
            Self::TransitVisa => "Transit Visa",
        }
    }

    /// Get default validity in days
    pub fn default_validity_days(&self) -> u32 {
        match self {
            Self::TouristVisa => 30,
            Self::VisaOnArrival => VISA_ON_ARRIVAL_VALIDITY_DAYS,
            Self::EVisa => 60,
            Self::VisaExemption { duration_days, .. } => *duration_days,
            Self::AseanVisaFree => ASEAN_VISA_FREE_DAYS,
            Self::TransitVisa => 7,
        }
    }
}

// ============================================================================
// Sustainable Tourism Types (ປະເພດການທ່ອງທ່ຽວແບບຍືນຍົງ)
// ============================================================================

/// Community-based tourism project (ໂຄງການທ່ອງທ່ຽວໂດຍຊຸມຊົນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityBasedTourism {
    /// Project name (ຊື່ໂຄງການ)
    pub project_name: String,

    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,

    /// Province (ແຂວງ)
    pub province: String,

    /// District (ເມືອງ)
    pub district: String,

    /// Registration number (ເລກທີລົງທະບຽນ)
    pub registration_number: Option<String>,

    /// Participating households (ຄົວເຮືອນທີ່ເຂົ້າຮ່ວມ)
    pub participating_households: u32,

    /// Activities offered (ກິດຈະກຳທີ່ສະເໜີ)
    pub activities: Vec<String>,

    /// Revenue sharing percentage to community (ອັດຕາແບ່ງປັນລາຍຮັບໃຫ້ຊຸມຊົນ)
    pub community_revenue_share_percent: f64,

    /// Environmental protection measures (ມາດຕະການປົກປ້ອງສິ່ງແວດລ້ອມ)
    pub environmental_measures: Vec<String>,

    /// Cultural preservation activities (ກິດຈະກຳອະນຸລັກວັດທະນະທຳ)
    pub cultural_preservation: Vec<String>,

    /// Maximum visitors per day (ນັກທ່ອງທ່ຽວສູງສຸດຕໍ່ມື້)
    pub max_visitors_per_day: Option<u32>,
}

// ============================================================================
// Tourism Statistics Types (ປະເພດສະຖິຕິການທ່ອງທ່ຽວ)
// ============================================================================

/// Tourism statistics report (ບົດລາຍງານສະຖິຕິການທ່ອງທ່ຽວ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TourismStatisticsReport {
    /// Reporting entity (ໜ່ວຍງານລາຍງານ)
    pub reporting_entity: String,

    /// Reporting period (month/year) (ໄລຍະລາຍງານ)
    pub reporting_period: String,

    /// Total visitors (ນັກທ່ອງທ່ຽວທັງໝົດ)
    pub total_visitors: u64,

    /// Foreign visitors (ນັກທ່ອງທ່ຽວຕ່າງປະເທດ)
    pub foreign_visitors: u64,

    /// Domestic visitors (ນັກທ່ອງທ່ຽວພາຍໃນ)
    pub domestic_visitors: u64,

    /// Revenue in LAK (ລາຍຮັບເປັນກີບ)
    pub revenue_lak: u64,

    /// Occupancy rate (if hotel) (ອັດຕາເຂົ້າພັກ)
    pub occupancy_rate: Option<f64>,

    /// Submission date (ວັນທີສົ່ງ)
    pub submission_date: DateTime<Utc>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_tourism_enterprise_category_descriptions() {
        let category = TourismEnterpriseCategory::Accommodation;
        assert_eq!(category.description_lao(), "ທີ່ພັກ");
        assert_eq!(category.description_en(), "Accommodation");
    }

    #[test]
    fn test_foreign_ownership_limits() {
        assert_eq!(
            TourismEnterpriseCategory::Accommodation.max_foreign_ownership_percent(),
            100.0
        );
        assert_eq!(
            TourismEnterpriseCategory::TourOperatorDomestic.max_foreign_ownership_percent(),
            0.0
        );
        assert_eq!(
            TourismEnterpriseCategory::TravelAgency.max_foreign_ownership_percent(),
            49.0
        );
    }

    #[test]
    fn test_star_rating_minimum_rooms() {
        assert_eq!(StarRating::OneStar.minimum_rooms(), MIN_ROOMS_1_STAR);
        assert_eq!(StarRating::FiveStar.minimum_rooms(), MIN_ROOMS_5_STAR);
    }

    #[test]
    fn test_star_rating_conversion() {
        assert_eq!(StarRating::from_u8(3), Some(StarRating::ThreeStar));
        assert_eq!(StarRating::from_u8(6), None);
        assert_eq!(StarRating::FourStar.as_u8(), 4);
    }

    #[test]
    fn test_accommodation_type_star_rating() {
        assert!(AccommodationType::Hotel.star_rating_applicable());
        assert!(!AccommodationType::Hostel.star_rating_applicable());
    }

    #[test]
    fn test_guide_license_category_descriptions() {
        let category = GuideLicenseCategory::National;
        assert_eq!(category.description_lao(), "ໄກດ໌ລະດັບຊາດ");
        assert_eq!(category.description_en(), "National Guide");
    }

    #[test]
    fn test_language_proficiency_order() {
        assert!(LanguageProficiency::Fluent > LanguageProficiency::Intermediate);
        assert!(LanguageProficiency::Native > LanguageProficiency::Advanced);
    }

    #[test]
    fn test_tourism_zone_permits() {
        assert!(TourismZoneType::SecurityZone.requires_special_permit());
        assert!(!TourismZoneType::NationalTourismZone.requires_special_permit());
    }

    #[test]
    fn test_tourism_zone_foreign_access() {
        assert!(!TourismZoneType::SecurityZone.open_to_foreign_tourists());
        assert!(TourismZoneType::HeritageSite.open_to_foreign_tourists());
    }

    #[test]
    fn test_visa_type_validity() {
        assert_eq!(
            TourismVisaType::VisaOnArrival.default_validity_days(),
            VISA_ON_ARRIVAL_VALIDITY_DAYS
        );
        assert_eq!(
            TourismVisaType::AseanVisaFree.default_validity_days(),
            ASEAN_VISA_FREE_DAYS
        );
    }

    #[test]
    fn test_travel_insurance_validity() {
        let insurance = TravelInsurance {
            policy_number: "POL-001".to_string(),
            insurance_company: "Test Insurance".to_string(),
            coverage_amount_usd: 50000,
            start_date: Utc::now() - Duration::days(10),
            end_date: Utc::now() + Duration::days(20),
            covers_medical: true,
            covers_evacuation: true,
            covers_cancellation: false,
        };

        assert!(insurance.is_valid());
    }

    #[test]
    fn test_asean_mra_certification() {
        let cert = AseanMraCertification {
            certificate_number: "MRA-001".to_string(),
            professional_type: AseanTourismProfessional::TourGuide,
            issue_country: "Lao PDR".to_string(),
            issue_date: Utc::now(),
            expiry_date: Utc::now() + Duration::days(365),
            holder_name: "Test Guide".to_string(),
        };

        assert!(cert.is_valid());
        assert_eq!(cert.professional_type.description_en(), "Tour Guide");
    }

    #[test]
    fn test_constants() {
        assert_eq!(TOURISM_LICENSE_VALIDITY_YEARS, 3);
        assert_eq!(GUIDE_LICENSE_VALIDITY_YEARS, 2);
        assert_eq!(FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT, 49.0);
        assert_eq!(MIN_GUIDE_TRAINING_HOURS, 120);
    }

    #[test]
    fn test_hotel_facility_descriptions() {
        let facility = HotelFacility::SwimmingPool;
        assert_eq!(facility.description_lao(), "ສະລອຍນ້ຳ");
        assert_eq!(facility.description_en(), "Swimming Pool");
    }

    #[test]
    fn test_accommodation_builder() {
        let enterprise = TourismEnterprise {
            name_lao: "ໂຮງແຮມທົດສອບ".to_string(),
            name_en: "Test Hotel".to_string(),
            category: TourismEnterpriseCategory::Accommodation,
            registration_number: "REG-001".to_string(),
            tourism_license_number: "TL-001".to_string(),
            license_status: LicenseStatus::Active,
            license_issue_date: Utc::now(),
            license_expiry_date: Utc::now() + Duration::days(365),
            province: "Vientiane".to_string(),
            district: "Chanthabouly".to_string(),
            address: "123 Test Street".to_string(),
            contact_phone: Some("021-123456".to_string()),
            email: None,
            website: None,
            foreign_ownership_percent: 0.0,
            registered_capital_lak: 100_000_000,
            employee_count: 10,
            lao_employee_count: 10,
        };

        let accommodation = AccommodationBuilder::new()
            .enterprise(enterprise)
            .accommodation_type(AccommodationType::Hotel)
            .star_rating(StarRating::ThreeStar)
            .room_count(35)
            .bed_count(70)
            .facilities(vec![HotelFacility::Restaurant, HotelFacility::WiFi])
            .build();

        assert!(accommodation.is_some());
        let accom = accommodation.expect("Accommodation should be created");
        assert!(accom.meets_room_requirement());
    }
}
