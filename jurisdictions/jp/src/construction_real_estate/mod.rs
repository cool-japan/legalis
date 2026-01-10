//! Construction Business Act and Real Estate Transactions Act
//!
//! Implementation of:
//! - Construction Business Act (建設業法 Act No. 100 of 1949)
//! - Real Estate Transactions Act (宅地建物取引業法 Act No. 176 of 1952)
//!
//! ## Construction Business Act (建設業法)
//!
//! Key provisions:
//! - **Article 3**: License requirement (一般建設業/特定建設業)
//! - **Article 7**: Capital requirements (¥5M for general, ¥20M for special)
//! - **Article 8**: Qualified manager requirements (技術者)
//! - **Article 3-3**: 5-year license validity period
//!
//! ## Real Estate Transactions Act (宅地建物取引業法)
//!
//! Key provisions:
//! - **Article 3**: License requirement and 5-year validity
//! - **Article 35**: Important matters explanation (重要事項説明)
//! - **Article 46**: Commission limits (3-5% depending on price)
//! - Licensed real estate agents (宅地建物取引士) required
//!
//! ## Examples
//!
//! ### Construction License Validation
//!
//! ```
//! use legalis_jp::construction_real_estate::{
//!     ConstructionBusinessLicense, ConstructionLicenseType, ConstructionType,
//!     Manager, ManagerQualification, validate_construction_license
//! };
//! use chrono::{Utc, Duration};
//!
//! let manager = Manager {
//!     name: "田中太郎".to_string(),
//!     qualification: ManagerQualification::FirstClassArchitect,
//!     certification_number: "CERT-001".to_string(),
//!     certification_date: Utc::now().date_naive(),
//! };
//!
//! let license = ConstructionBusinessLicense {
//!     license_number: "建-001".to_string(),
//!     business_name: "株式会社テスト建設".to_string(),
//!     license_type: ConstructionLicenseType::General,
//!     construction_types: vec![ConstructionType::Architecture],
//!     registered_capital_jpy: 10_000_000,
//!     issue_date: Utc::now().date_naive(),
//!     expiration_date: Utc::now().date_naive() + Duration::days(365 * 5),
//!     managers: vec![manager],
//! };
//!
//! let report = validate_construction_license(&license)?;
//! assert!(report.is_valid());
//! # Ok::<(), legalis_jp::construction_real_estate::ConstructionRealEstateError>(())
//! ```
//!
//! ### Real Estate Transaction Validation
//!
//! ```
//! use legalis_jp::construction_real_estate::{
//!     RealEstateTransaction, TransactionType, Property, PropertyType,
//!     Party, validate_real_estate_transaction
//! };
//! use chrono::Utc;
//!
//! let transaction = RealEstateTransaction {
//!     transaction_id: "TX-001".to_string(),
//!     transaction_type: TransactionType::Sale,
//!     property: Property {
//!         property_type: PropertyType::Building,
//!         address: "東京都渋谷区1-1-1".to_string(),
//!         area_sqm: 100.0,
//!         price_jpy: 50_000_000,
//!         description: Some("新築マンション".to_string()),
//!     },
//!     buyer: Party {
//!         name: "買主".to_string(),
//!         address: "東京都".to_string(),
//!         contact: Some("03-1234-5678".to_string()),
//!     },
//!     seller: Party {
//!         name: "売主".to_string(),
//!         address: "東京都".to_string(),
//!         contact: None,
//!     },
//!     broker: None,
//!     important_matters_explained: true,
//!     contract_date: Utc::now().date_naive(),
//! };
//!
//! let report = validate_real_estate_transaction(&transaction)?;
//! assert!(report.is_valid());
//! # Ok::<(), legalis_jp::construction_real_estate::ConstructionRealEstateError>(())
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{ConstructionRealEstateError, Result};
pub use types::{
    ConstructionBusinessLicense, ConstructionLicenseType, ConstructionType, LicensedAgent,
    LicensedBroker, Manager, ManagerQualification, Party, Property, PropertyType,
    RealEstateLicense, RealEstateTransaction, TransactionType,
};
pub use validator::{
    quick_validate_construction, quick_validate_real_estate, validate_construction_license,
    validate_real_estate_transaction,
};
