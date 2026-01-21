//! Land Law Module (ກົດໝາຍທີ່ດິນ)
//!
//! This module implements the Land Law 2019 (Law No. 70/NA) of the Lao People's Democratic Republic.
//!
//! ## Legal Foundation
//!
//! **Law Name (Lao):** ກົດໝາຍທີ່ດິນ ປີ 2019
//! **Law Name (English):** Land Law 2019
//! **Law Number:** Law No. 70/NA
//! **Enacted:** October 29, 2019
//! **Effective:** October 29, 2019
//! **Previous Versions:** Land Law 2003, Land Law 1997
//!
//! ## Fundamental Principle: State Land Ownership
//!
//! ### Article 3: State Ownership Principle (ຫຼັກການທີ່ດິນເປັນຊັບສິນຂອງຊາດ)
//!
//! **LAO (ພາສາລາວ):**
//! > ທີ່ດິນທັງໝົດເປັນຊັບສິນຂອງຊາດທີ່ລັດເປັນຜູ້ຄຸ້ມຄອງ ແລະ ໃຫ້ປະຊາຊົນ, ອົງການຈັດຕັ້ງນຳໃຊ້.
//!
//! **ENGLISH:**
//! > All land is the property of the national community under state management,
//! > and is allocated to the people and organizations for use.
//!
//! This is the most fundamental principle of Lao land law. Unlike many Western legal systems
//! where individuals can own land in fee simple, in Lao PDR:
//!
//! - The **state** retains ultimate ownership of all land
//! - Individuals and organizations only receive **land use rights** (ສິດນຳໃຊ້ທີ່ດິນ)
//! - These rights can be perpetual or temporary
//! - These rights can be inherited, transferred, leased, or mortgaged
//! - But the underlying ownership always remains with the state
//!
//! ## Key Legal Concepts
//!
//! ### 1. Land Use Rights (ສິດນຳໃຊ້ທີ່ດິນ)
//!
//! There are two main types of land use rights:
//!
//! #### Perpetual Use Right (ສິດນຳໃຊ້ຖາວອນ)
//! - Available **only to Lao citizens** and domestic legal entities
//! - Can be inherited through generations
//! - Can be transferred, sold, or mortgaged (with government approval)
//! - Subject to land use regulations and zoning
//! - Most secure form of land tenure in Laos
//!
//! #### Temporary Use Right (ສິດນຳໃຊ້ຊົ່ວຄາວ)
//! - Limited duration (typically 30-50 years)
//! - Available to both citizens and foreigners
//! - May be renewable depending on agreement
//! - Common for commercial and industrial purposes
//! - Subject to stricter conditions than perpetual rights
//!
//! ### 2. Land Concessions (ສຳປະທານທີ່ດິນ)
//!
//! The government grants land concessions for large-scale economic activities:
//!
//! - **Agricultural Concessions** (ສຳປະທານກະສິກຳ): 30-50 years, up to 10,000 hectares
//! - **Industrial Concessions** (ສຳປະທານອຸດສາຫະກຳ): 50-75 years
//! - **Commercial Concessions** (ສຳປະທານການຄ້າ): 30-50 years
//! - **Mining Concessions** (ສຳປະທານບໍ່ແຮ່): Duration varies, requires EIA
//! - **Tourism Concessions** (ສຳປະທານທ່ອງທ່ຽວ): 50-99 years
//!
//! ### 3. Foreign Ownership Restrictions (ຂໍ້ຈຳກັດຊາວຕ່າງປະເທດ)
//!
//! Foreign nationals and entities face significant restrictions:
//!
//! - **Cannot** hold perpetual land use rights
//! - **Can** lease land for up to 50 years (standard) or 99 years (special economic zones)
//! - **Can** own condominium units (but not the land beneath)
//! - **Must** obtain government approval for most transactions
//! - Foreign-invested companies with >49% foreign ownership treated as foreign
//!
//! ### 4. Land Registration System (ລະບົບລົງທະບຽນທີ່ດິນ)
//!
//! The Land Law 2019 strengthens the land registration system:
//!
//! - **Full Title Deed** (ໃບຕາດິນເຕັມສິດ): Highest form of title
//! - **Temporary Certificate** (ໃບຢັ້ງຢືນຊົ່ວຄາວ): Pending full survey
//! - **Tax Receipt** (ໃບເສຍພາສີ): Legacy evidence of use (older system)
//! - **Cadastral Survey** (ການສຳຫຼວດທີ່ດິນ): Required for full title
//! - **Central Cadastre** (ທະບຽນສູນກາງ): National database
//!
//! ## Module Structure
//!
//! This module is organized into three main components:
//!
//! - **types**: Core data structures (land use rights, concessions, titles, transactions)
//! - **error**: Error types with bilingual (Lao/English) messages
//! - **validator**: Validation functions ensuring compliance with Land Law 2019
//!
//! ## Example Usage
//!
//! ### Example 1: Perpetual Land Use Right for Lao Citizen
//!
//! ```
//! use legalis_la::land_law::*;
//! use chrono::Utc;
//!
//! // Lao citizen with perpetual use right
//! let use_right = LandUseRight::PerpetualUse {
//!     holder_name: "ສົມໃຈ ວົງພະຈັນ / Somchai Vongphachan".to_string(),
//!     holder_nationality: "LAO".to_string(),
//!     granted_at: Utc::now(),
//!     parcel_id: "VTE-CH-001-2024".to_string(),
//!     area_sqm: 1200,
//!     permitted_use: LandUsePurpose::Residential,
//! };
//!
//! // Validate according to Land Law 2019
//! assert!(validate_land_use_right(&use_right).is_ok());
//! ```
//!
//! ### Example 2: Foreign National Lease (Allowed)
//!
//! ```
//! use legalis_la::land_law::*;
//! use chrono::{Utc, Duration};
//!
//! // Foreign national with temporary use right (lease)
//! let use_right = LandUseRight::TemporaryUse {
//!     holder_name: "John Smith".to_string(),
//!     holder_nationality: "USA".to_string(),
//!     granted_at: Utc::now(),
//!     expires_at: Utc::now() + Duration::days(365 * 30), // 30 years
//!     parcel_id: "VTE-CH-002-2024".to_string(),
//!     area_sqm: 800,
//!     permitted_use: LandUsePurpose::Commercial,
//!     renewable: true,
//! };
//!
//! // This is allowed under Lao law
//! assert!(validate_land_use_right(&use_right).is_ok());
//! ```
//!
//! ### Example 3: Foreign Ownership Restrictions
//!
//! ```
//! use legalis_la::land_law::*;
//!
//! let foreign_status = ForeignOwnershipStatus::ForeignNational {
//!     passport_number: "P1234567".to_string(),
//!     nationality: "Thailand".to_string(),
//!     lease_approved: true,
//! };
//!
//! // Foreign nationals CANNOT hold perpetual use rights
//! assert!(validate_foreign_ownership(&foreign_status, true).is_err());
//!
//! // But they CAN hold temporary use rights (leases)
//! assert!(validate_foreign_ownership(&foreign_status, false).is_ok());
//! ```
//!
//! ## Legal References
//!
//! - Land Law 2019 (Law No. 70/NA): Full text in Lao language
//! - Investment Promotion Law 2016: Foreign investment framework
//! - Enterprise Law 2013: Corporate land use rights
//! - Constitution of Lao PDR (2015): Article 17 on state land ownership
//!
//! ## Comparative Law Notes
//!
//! The Lao land law system differs significantly from Western systems:
//!
//! ### Comparison with Common Law (e.g., USA, UK, Australia)
//! - Common law: Fee simple absolute ownership possible
//! - Lao law: Only use rights, state retains ownership
//!
//! ### Comparison with Civil Law (e.g., France, Germany)
//! - Civil law: Private property ownership guaranteed (French Civil Code Article 544)
//! - Lao law: State ownership principle (Article 3)
//!
//! ### Similarity with Vietnam and China
//! - Vietnam: Similar state land ownership system (Land Law 2013)
//! - China: Similar "land use rights" system (中国土地管理法)
//! - All three are socialist-oriented market economies
//!
//! ## Historical Development
//!
//! - **1975-1986**: Collectivization period, no individual land rights
//! - **1986**: Doi Moi (New Economic Mechanism), gradual liberalization
//! - **1997**: First comprehensive Land Law (Law No. 01/97)
//! - **2003**: Revised Land Law (Law No. 04/03)
//! - **2019**: Current Land Law (Law No. 70/NA) - strengthened registration

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{LandLawError, Result};

pub use types::{
    CadastralSurvey, DisputeStatus, ForeignOwnershipStatus, LandCertificate, LandClassification,
    LandConcession, LandDispute, LandDisputeType, LandRegistrationStatus, LandTitle, LandTitleType,
    LandTransaction, LandTransactionType, LandUsePurpose, LandUseRight, ResolutionMethod,
    StateLand, SurveyMethod,
};

pub use validator::{
    validate_cadastral_survey, validate_foreign_ownership, validate_land_concession,
    validate_land_registration, validate_land_title, validate_land_transaction,
    validate_land_use_right,
};
