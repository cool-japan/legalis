//! Tourism Law Module (ກົດໝາຍທ່ອງທ່ຽວ)
//!
//! This module provides comprehensive support for Lao tourism law based on:
//! - **Tourism Law 2013** (Law No. 32/NA, effective September 2013)
//! - **Tourism Development Strategic Plan**
//! - **ASEAN Tourism Agreement**
//!
//! # Legal Framework
//!
//! The Tourism Law 2013 is the primary legislation governing tourism
//! in the Lao People's Democratic Republic. It establishes standards for:
//!
//! ## Key Provisions
//!
//! ### Tourism Enterprise Categories (ປະເພດວິສາຫະກິດທ່ອງທ່ຽວ)
//! - **Article 22**: Tourism enterprises are classified into categories
//! - **Article 23**: All tourism enterprises must be registered and licensed
//! - **Article 25**: Foreign ownership limits (49% for most activities)
//! - **Article 26**: License validity is 3 years
//!
//! ### Hotel Classification (ການຈັດລະດັບໂຮງແຮມ)
//! - **Article 30**: Hotels must be classified according to star ratings
//! - **Article 31**: Minimum room requirements per star rating
//! - **Article 32**: Facility requirements for each star level
//! - **Article 33**: Classification validity is 3 years
//!
//! ### Tour Guide Licensing (ໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວ)
//! - **Article 35**: Tour guides must be licensed
//! - **Article 36**: Language proficiency requirements (Lao + foreign language)
//! - **Article 37**: Training certification requirements (120 hours minimum)
//! - **Article 38**: License validity is 2 years
//! - **Article 39**: License scope (national vs provincial)
//!
//! ### Tourism Zones (ເຂດທ່ອງທ່ຽວ)
//! - **Article 44**: Tourism zone classifications
//! - **Article 45**: Special permits for certain zones
//! - **Article 46**: Carrying capacity limits
//!
//! ### Tourist Rights & Protection (ສິດນັກທ່ອງທ່ຽວ)
//! - **Article 50**: Complaint mechanisms (15-day response deadline)
//! - **Article 51**: Travel insurance requirements
//! - **Article 52**: Consumer protection
//! - **Article 53**: Emergency assistance obligations
//!
//! ### Sustainable Tourism (ການທ່ອງທ່ຽວແບບຍືນຍົງ)
//! - **Article 54**: Environmental impact requirements
//! - **Article 55**: Cultural heritage protection
//! - **Article 56**: Community-based tourism framework
//!
//! ### ASEAN Integration (ການເຊື່ອມໂຍງອາຊຽນ)
//! - **Article 65**: ASEAN MRA compliance for tourism professionals
//! - **Article 66**: Cross-border tourism packages
//!
//! # Tourism Enterprise Categories
//!
//! The law recognizes the following enterprise categories:
//!
//! 1. **Accommodation** (ທີ່ພັກ) - Hotels, guesthouses, resorts, eco-lodges
//! 2. **Tour Operators** (ຜູ້ປະກອບການທົວ) - Inbound, outbound, domestic
//! 3. **Travel Agencies** (ຕົວແທນທ່ອງທ່ຽວ) - Booking and ticketing
//! 4. **Tourism Transport** (ການຂົນສົ່ງທ່ອງທ່ຽວ) - Dedicated tourism vehicles
//! 5. **Tourist Guide Services** (ການບໍລິການໄກດ໌) - Guide companies
//! 6. **Tourism Attractions** (ສະຖານທີ່ທ່ອງທ່ຽວ) - Parks, museums, sites
//!
//! # Foreign Ownership Rules
//!
//! Foreign ownership is subject to the following limits:
//! - **Hotels/Accommodation**: Up to 100% foreign ownership
//! - **Tour Operators (Inbound/Outbound)**: Maximum 49% foreign ownership
//! - **Tour Operators (Domestic)**: 0% - Lao nationals only
//! - **Travel Agencies**: Maximum 49% foreign ownership
//! - **Tour Guide Services**: 0% - Lao nationals only
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for tourism law compliance
//! - **Comprehensive Coverage**: All major aspects of Tourism Law 2013
//! - **ASEAN Integration**: Support for ASEAN tourism professional recognition
//!
//! # Examples
//!
//! ## Validating a Tourism Enterprise License
//!
//! ```rust
//! use legalis_la::tourism_law::*;
//! use chrono::{Utc, Duration};
//!
//! let enterprise = TourismEnterprise {
//!     name_lao: "ບໍລິສັດທົວທົດສອບ".to_string(),
//!     name_en: "Test Tours Co.".to_string(),
//!     category: TourismEnterpriseCategory::TourOperatorInbound,
//!     registration_number: "REG-2024-001".to_string(),
//!     tourism_license_number: "TL-2024-001".to_string(),
//!     license_status: LicenseStatus::Active,
//!     license_issue_date: Utc::now(),
//!     license_expiry_date: Utc::now() + Duration::days(365 * 3),
//!     province: "Vientiane".to_string(),
//!     district: "Chanthabouly".to_string(),
//!     address: "123 Tourism Street".to_string(),
//!     contact_phone: Some("021-123456".to_string()),
//!     email: Some("info@testtours.la".to_string()),
//!     website: Some("www.testtours.la".to_string()),
//!     foreign_ownership_percent: 30.0,
//!     registered_capital_lak: 500_000_000,
//!     employee_count: 15,
//!     lao_employee_count: 12,
//! };
//!
//! match validate_enterprise_license(&enterprise) {
//!     Ok(()) => println!("Enterprise license is valid"),
//!     Err(e) => {
//!         println!("English: {}", e.english_message());
//!         println!("Lao: {}", e.lao_message());
//!     }
//! }
//! ```
//!
//! ## Validating a Tour Guide License
//!
//! ```rust
//! use legalis_la::tourism_law::*;
//! use chrono::{Utc, Duration};
//!
//! let guide = TourGuide {
//!     name: "Somchai Vongsa".to_string(),
//!     name_lao: Some("ສົມໃຈ ວົງສາ".to_string()),
//!     license_number: "GL-2024-001".to_string(),
//!     license_category: GuideLicenseCategory::National,
//!     license_status: LicenseStatus::Active,
//!     license_issue_date: Utc::now(),
//!     license_expiry_date: Utc::now() + Duration::days(365 * 2),
//!     language_skills: vec![
//!         LanguageSkill {
//!             language: "Lao".to_string(),
//!             proficiency: LanguageProficiency::Native,
//!             certification: None,
//!         },
//!         LanguageSkill {
//!             language: "English".to_string(),
//!             proficiency: LanguageProficiency::Advanced,
//!             certification: Some("TOEFL 90".to_string()),
//!         },
//!     ],
//!     training_certifications: vec!["National Guide Certificate".to_string()],
//!     training_hours: 150,
//!     province: None,
//!     affiliated_company: Some("Lao Tours".to_string()),
//!     years_of_experience: 8,
//!     nationality: "Lao".to_string(),
//!     date_of_birth: Utc::now() - Duration::days(365 * 35),
//!     id_card_number: "ID-123456789".to_string(),
//! };
//!
//! assert!(validate_guide_license(&guide).is_ok());
//! assert!(validate_guide_language(&guide).is_ok());
//! assert!(validate_guide_training(&guide).is_ok());
//! ```
//!
//! ## Validating Hotel Classification
//!
//! ```rust
//! use legalis_la::tourism_law::*;
//! use chrono::{Utc, Duration};
//!
//! // Create base enterprise
//! let enterprise = TourismEnterprise {
//!     name_lao: "ໂຮງແຮມທົດສອບ".to_string(),
//!     name_en: "Test Hotel".to_string(),
//!     category: TourismEnterpriseCategory::Accommodation,
//!     registration_number: "REG-2024-001".to_string(),
//!     tourism_license_number: "TL-2024-001".to_string(),
//!     license_status: LicenseStatus::Active,
//!     license_issue_date: Utc::now(),
//!     license_expiry_date: Utc::now() + Duration::days(365 * 3),
//!     province: "Vientiane".to_string(),
//!     district: "Chanthabouly".to_string(),
//!     address: "456 Hotel Road".to_string(),
//!     contact_phone: Some("021-654321".to_string()),
//!     email: None,
//!     website: None,
//!     foreign_ownership_percent: 0.0,
//!     registered_capital_lak: 1_000_000_000,
//!     employee_count: 50,
//!     lao_employee_count: 45,
//! };
//!
//! let hotel = Accommodation {
//!     enterprise,
//!     accommodation_type: AccommodationType::Hotel,
//!     star_rating: Some(StarRating::ThreeStar),
//!     classification_status: HotelClassificationStatus::Classified {
//!         classification_date: Utc::now(),
//!         expiry_date: Utc::now() + Duration::days(365 * 3),
//!         authority: "Ministry of ICT".to_string(),
//!     },
//!     room_count: 35,
//!     bed_count: 70,
//!     facilities: vec![
//!         HotelFacility::Reception24Hour,
//!         HotelFacility::Restaurant,
//!         HotelFacility::AirConditioning,
//!         HotelFacility::WiFi,
//!     ],
//!     check_in_time: Some("14:00".to_string()),
//!     check_out_time: Some("12:00".to_string()),
//!     average_room_rate_lak: Some(500_000),
//! };
//!
//! assert!(validate_hotel_classification(&hotel).is_ok());
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::tourism_law::*;
//!
//! let error = TourismLawError::GuideUnlicensed {
//!     guide_name: "Test Guide".to_string(),
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Tour guide is unlicensed: Test Guide (Article 35)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ໄກດ໌ນຳທ່ຽວບໍ່ມີໃບອະນຸຍາດ: Test Guide (ມາດຕາ 35)"
//! ```
//!
//! # Compliance Notes
//!
//! When implementing tourism law compliance in Laos:
//!
//! 1. **Enterprise Licensing**: All tourism businesses must be licensed before operation
//! 2. **Foreign Ownership**: Verify ownership percentages against category limits
//! 3. **Hotel Classification**: Star-rated hotels must meet minimum requirements
//! 4. **Tour Guides**: Must be licensed, trained, and proficient in languages
//! 5. **Zone Access**: Some zones require special permits
//! 6. **Tourist Protection**: Complaints must be addressed within 15 days
//! 7. **ASEAN MRA**: Tourism professionals may work across ASEAN with MRA certification
//! 8. **Statistics**: Regular reporting of tourism statistics is mandatory

pub mod error;
pub mod types;
pub mod validator;

// Re-export error types
pub use error::{Result, TourismLawError};

// Re-export constants
pub use types::{
    ASEAN_VISA_FREE_DAYS, COMPLAINT_RESPONSE_DEADLINE_DAYS, FOREIGN_OWNERSHIP_GENERAL_MAX_PERCENT,
    FOREIGN_OWNERSHIP_HOTEL_MAX_PERCENT, GUIDE_LICENSE_VALIDITY_YEARS, MAX_ROOMS_BOUTIQUE,
    MAX_ROOMS_GUESTHOUSE, MIN_GUIDE_TRAINING_HOURS, MIN_ROOMS_1_STAR, MIN_ROOMS_2_STAR,
    MIN_ROOMS_3_STAR, MIN_ROOMS_4_STAR, MIN_ROOMS_5_STAR, MIN_ROOMS_BOUTIQUE, MIN_ROOMS_GUESTHOUSE,
    STAR_RATING_VALIDITY_YEARS, TOURISM_DEVELOPMENT_FUND_RATE_PERCENT,
    TOURISM_LICENSE_VALIDITY_YEARS, VISA_ON_ARRIVAL_VALIDITY_DAYS,
};

// Re-export tourism enterprise types
pub use types::{LicenseStatus, TourismEnterprise, TourismEnterpriseCategory};

// Re-export accommodation types
pub use types::{
    Accommodation, AccommodationBuilder, AccommodationType, HotelClassificationStatus,
    HotelFacility, StarRating,
};

// Re-export tour guide types
pub use types::{GuideLicenseCategory, LanguageProficiency, LanguageSkill, TourGuide};

// Re-export tourism zone types
pub use types::{TourismZone, TourismZoneType};

// Re-export tourist rights types
pub use types::{ComplaintStatus, TouristComplaint, TravelInsurance};

// Re-export ASEAN integration types
pub use types::{AseanMraCertification, AseanTourismProfessional};

// Re-export visa types
pub use types::TourismVisaType;

// Re-export sustainable tourism types
pub use types::CommunityBasedTourism;

// Re-export statistics types
pub use types::TourismStatisticsReport;

// Re-export validators
pub use validator::{
    validate_accommodation_comprehensive, validate_asean_mra_certification,
    validate_carrying_capacity, validate_cbt_project, validate_complaint_response,
    validate_enterprise_comprehensive, validate_enterprise_for_activity,
    validate_enterprise_license, validate_entrance_fee, validate_foreign_ownership,
    validate_guide_comprehensive, validate_guide_language, validate_guide_license,
    validate_guide_scope, validate_guide_training, validate_hotel_classification,
    validate_hotel_facilities, validate_statistics_submission, validate_tourism_visa,
    validate_travel_insurance, validate_zone_access,
};
