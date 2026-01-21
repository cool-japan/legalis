//! Commercial Law (ກົດໝາຍການຄ້າ)
//!
//! This module implements Lao commercial law provisions, including:
//! - **Enterprise Law 2013** (Law No. 46/NA, effective June 21, 2014)
//! - **Investment Promotion Law 2016** (Law No. 14/NA, amended 2017)
//!
//! ## Historical Context
//!
//! Lao PDR's commercial law framework has evolved significantly since the country's
//! transition to a market economy in 1986 (New Economic Mechanism - ນະໂຍບາຍເສດຖະກິດໃໝ່).
//! The Enterprise Law 2013 replaced the previous Enterprise Law 2005, modernizing
//! the legal framework for business entities and aligning it with international standards.
//!
//! The Investment Promotion Law 2016 consolidated previous investment regulations,
//! establishing a clearer framework for both domestic and foreign investment with
//! emphasis on Special Economic Zones (SEZs) and targeted sector development.
//!
//! ## Enterprise Law 2013 Overview
//!
//! The Enterprise Law recognizes five main types of business entities:
//!
//! 1. **Individual Enterprise** (ວິສາຫະກິດສ່ວນບຸກຄົນ)
//!    - Owned by one individual
//!    - No minimum capital requirement
//!    - Unlimited personal liability
//!    - Simplest form of business registration
//!
//! 2. **Partnership** (ຫ້າງຫຸ້ນສ່ວນ)
//!    - Ordinary partnership: All partners have unlimited liability
//!    - Limited partnership: General partners (unlimited) + Limited partners (limited)
//!    - No minimum capital requirement
//!    - Requires partnership agreement
//!
//! 3. **Limited Company** (ບໍລິສັດຈໍາກັດ)
//!    - Private limited company
//!    - Minimum registered capital: 50,000,000 LAK
//!    - Minimum paid-up capital: 30% of registered capital
//!    - 1-30 shareholders
//!    - Limited liability to shareholders
//!
//! 4. **Public Company** (ບໍລິສັດມະຫາຊົນ)
//!    - Can be listed on Lao Securities Exchange
//!    - Minimum registered capital: 1,000,000,000 LAK
//!    - Minimum paid-up capital: 30% of registered capital
//!    - Minimum 15 shareholders
//!    - Stricter governance and disclosure requirements
//!
//! 5. **State-Owned Enterprise** (ວິສາຫະກິດລັດ)
//!    - Wholly or partially owned by the state
//!    - Governed by separate SOE Law
//!
//! ## Investment Promotion Law 2016 Overview
//!
//! The Investment Promotion Law provides:
//!
//! ### Investment Incentives
//! - **Profit tax exemption**: 2-10 years depending on sector and location
//! - **Import duty exemption**: For machinery, equipment, and raw materials
//! - **Export duty exemption**: For certain products
//! - **Land rental fee reduction**: Up to 75% in promoted zones
//! - **Fast-track approval**: For priority sectors
//!
//! ### Investment Classifications
//! - **Promoted sectors**: Agriculture, manufacturing, tourism, technology
//! - **General sectors**: Services, trade, real estate
//! - **Restricted sectors**: Banking, insurance, telecom (max 49% foreign ownership)
//! - **Prohibited sectors**: Media, domestic postal services
//!
//! ### Special Economic Zones (SEZs)
//! Lao PDR has established several SEZs offering enhanced incentives:
//! - Savan-Seno SEZ (Savannakhet)
//! - Golden Triangle SEZ (Bokeo)
//! - Thakhek SEZ (Khammouane)
//! - Phoukhyo SEZ (Champasak)
//!
//! ## Example Usage
//!
//! ```
//! use legalis_la::commercial_law::{
//!     EnterpriseType, LimitedCompany, Shareholder, BoardOfDirectors, Director,
//!     DirectorPosition, validate_enterprise_formation, validate_board_composition,
//! };
//! use chrono::Utc;
//!
//! // Create a limited company
//! let company = LimitedCompany {
//!     name_en: "Lao Tech Solutions Ltd".to_string(),
//!     name_lo: "ບໍລິສັດ ວິທະຍາສາດ ລາວ ຈໍາກັດ".to_string(),
//!     registered_capital: 100_000_000, // 100 million LAK
//!     paid_up_capital: 30_000_000,     // 30 million LAK (30%)
//!     shareholders: vec![
//!         Shareholder {
//!             name: "Local Investor".to_string(),
//!             id: "L123456".to_string(),
//!             shares: 6000,
//!             ownership_percentage: 60.0,
//!             is_foreign: false,
//!             nationality: None,
//!         },
//!         Shareholder {
//!             name: "Foreign Investor".to_string(),
//!             id: "F789012".to_string(),
//!             shares: 4000,
//!             ownership_percentage: 40.0,
//!             is_foreign: true,
//!             nationality: Some("Singapore".to_string()),
//!         },
//!     ],
//!     board: BoardOfDirectors {
//!         directors: vec![
//!             Director {
//!                 name: "Chairperson Name".to_string(),
//!                 id: "D001".to_string(),
//!                 position: DirectorPosition::Chairperson,
//!                 nationality: "LAO".to_string(),
//!                 is_foreign: false,
//!                 appointed_at: Utc::now(),
//!             },
//!             Director {
//!                 name: "Managing Director".to_string(),
//!                 id: "D002".to_string(),
//!                 position: DirectorPosition::ManagingDirector,
//!                 nationality: "LAO".to_string(),
//!                 is_foreign: false,
//!                 appointed_at: Utc::now(),
//!             },
//!             Director {
//!                 name: "Director 3".to_string(),
//!                 id: "D003".to_string(),
//!                 position: DirectorPosition::Director,
//!                 nationality: "Singapore".to_string(),
//!                 is_foreign: true,
//!                 appointed_at: Utc::now(),
//!             },
//!         ],
//!         meetings_per_year: 4,
//!         last_meeting: None,
//!     },
//!     activities: vec!["Software development".to_string(), "IT consulting".to_string()],
//!     registration_number: "REG-2024-001".to_string(),
//!     registered_at: Utc::now(),
//!     foreign_ownership_percentage: 40.0,
//! };
//!
//! // Validate enterprise formation
//! let result = validate_enterprise_formation(
//!     &EnterpriseType::LimitedCompany,
//!     &company.name_en,
//!     &company.name_lo,
//!     company.registered_capital,
//!     Some(&company.shareholders),
//! );
//! assert!(result.is_ok());
//!
//! // Validate board composition
//! let result = validate_board_composition(&company.board);
//! assert!(result.is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export error types
pub use error::{CommercialLawError, Result};

// Re-export enterprise types
pub use types::{
    BoardOfDirectors, BusinessSector, CapitalRequirements, Concession, ConcessionType, Copyright,
    CopyrightWorkType, Director, DirectorPosition, DomesticInvestment, EnterpriseType,
    ForeignInvestment, IPStatus, IndividualEnterprise, IndustrialDesign, IntellectualProperty,
    InvestmentIncentive, InvestmentType, LiabilityType, LimitedCompany, MeetingType, Partner,
    PartnerType, PartnershipType, Patent, PublicCompany, Resolution, Shareholder,
    ShareholdersMeeting, Trademark,
};

// Re-export partnership types
pub use types::Partnership;

// Re-export restricted sector types
pub use types::{ApprovalStatus, ConditionalSector, ProhibitedSector, RestrictedSector};

// Re-export validators
pub use validator::{
    validate_board_composition, validate_business_name, validate_capital_requirements,
    validate_copyright, validate_enterprise_formation, validate_foreign_investment,
    validate_industrial_design, validate_ip_registration, validate_partnership, validate_patent,
    validate_restricted_sector, validate_trademark,
};
