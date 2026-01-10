//! German Family Law (Familienrecht - BGB Book 4)
//!
//! This module provides type-safe representations and validation for German family law
//! under the Bürgerliches Gesetzbuch (BGB) Book 4 (§§1297-1921).
//!
//! # Legal Context
//!
//! German family law regulates:
//! - **Marriage (Ehe)**: Formation, requirements, and effects (§§1303-1362)
//! - **Matrimonial Property (Güterrecht)**: Property regimes between spouses (§§1363-1563)
//! - **Divorce (Scheidung)**: Dissolution of marriage (§§1564-1587)
//! - **Maintenance (Unterhalt)**: Support obligations (§§1569-1615)
//! - **Parentage (Abstammung)**: Parent-child legal relationships (§§1591-1600)
//! - **Parental Custody (Sorgerecht)**: Rights and duties toward children (§§1626-1698)
//!
//! # Key Concepts
//!
//! ## Marriage Formation (§§1303-1311 BGB)
//!
//! Valid marriage requires:
//! - Both parties at least 18 years old (§1303 BGB)
//! - No existing marriage (§1306 BGB)
//! - Parties not closely related (§1307 BGB)
//! - Both parties have legal capacity (§1304 BGB)
//! - Civil ceremony before registrar (Standesbeamter)
//!
//! ## Matrimonial Property Regimes (§§1363-1563 BGB)
//!
//! Three regimes exist:
//!
//! 1. **Community of Accrued Gains (Zugewinngemeinschaft)** - §§1363-1390 BGB
//!    - DEFAULT regime if no agreement
//!    - Each spouse keeps separate property during marriage
//!    - Upon divorce/death: equalization of gains accrued during marriage
//!    - Calculation: Final assets - Initial assets = Accrued gain
//!    - Spouse with lower gain entitled to half the difference
//!
//! 2. **Separation of Property (Gütertrennung)** - §1414 BGB
//!    - Complete separation
//!    - No equalization upon divorce
//!    - Requires matrimonial property agreement
//!
//! 3. **Community of Property (Gütergemeinschaft)** - §§1415-1518 BGB
//!    - Rare in practice
//!    - Joint ownership of assets
//!    - Requires matrimonial property agreement
//!
//! ## Divorce (§§1564-1587 BGB)
//!
//! Divorce requirements:
//! - Marriage breakdown (Scheitern der Ehe) - §1565 BGB
//! - Separation period:
//!   - 1 year with mutual consent (§1566 Abs. 1)
//!   - 3 years without consent (§1566 Abs. 2)
//!
//! Divorce consequences:
//! - Accrued gains equalization (§§1372-1390 BGB)
//! - Pension equalization (Versorgungsausgleich) - §§1587-1587p BGB
//! - Post-marital maintenance (§§1569-1586 BGB)
//!
//! ## Post-Marital Maintenance (§§1569-1586 BGB)
//!
//! Grounds for maintenance claims:
//! - Child care (§1570 BGB)
//! - Age (§1571 BGB)
//! - Illness (§1572 BGB)
//! - Unemployment (§1573 BGB)
//! - Additional training (§1575 BGB)
//! - Equity (§1576 BGB)
//!
//! Temporal limitation (§1578b BGB): Courts may limit duration based on circumstances
//!
//! ## Parentage (§§1591-1600 BGB)
//!
//! Legal motherhood:
//! - §1591 BGB: Mother is the woman who gave birth
//!
//! Legal fatherhood (§1592 BGB):
//! 1. Husband of mother at child's birth
//! 2. Man who acknowledged paternity
//! 3. Man determined father by court
//!
//! ## Parental Custody (§§1626-1698 BGB)
//!
//! - §1626 BGB: Parents have duty and right to care for child
//! - Joint custody (gemeinsame Sorge): DEFAULT for married parents
//! - Sole custody (Alleinsorge): One parent only
//! - Custody ends when child reaches 18 (majority)
//!
//! # Examples
//!
//! ## Valid Marriage
//!
//! ```rust
//! use legalis_de::bgb::familienrecht::*;
//! use chrono::NaiveDate;
//!
//! let marriage = Marriage {
//!     spouse1: Person {
//!         name: "Hans Mueller".to_string(),
//!         date_of_birth: NaiveDate::from_ymd_opt(1990, 5, 15).unwrap(),
//!         place_of_birth: "Berlin".to_string(),
//!         nationality: "German".to_string(),
//!         gender: Gender::Male,
//!         address: "Musterstrasse 1, 10115 Berlin".to_string(),
//!     },
//!     spouse2: Person {
//!         name: "Maria Schmidt".to_string(),
//!         date_of_birth: NaiveDate::from_ymd_opt(1992, 8, 20).unwrap(),
//!         place_of_birth: "Munich".to_string(),
//!         nationality: "German".to_string(),
//!         gender: Gender::Female,
//!         address: "Beispielweg 5, 80331 Munich".to_string(),
//!     },
//!     marriage_date: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
//!     place_of_marriage: "Berlin".to_string(),
//!     registrar_office: "Standesamt Berlin-Mitte".to_string(),
//!     status: MarriageStatus::Valid,
//!     property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
//!     impediments: vec![],
//! };
//!
//! // Validate marriage
//! assert!(validate_marriage(&marriage).is_ok());
//! ```
//!
//! ## Divorce with Accrued Gains Equalization
//!
//! ```rust
//! use legalis_de::bgb::familienrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::NaiveDate;
//! # use chrono::Utc;
//!
//! # let marriage = Marriage {
//! #     spouse1: Person {
//! #         name: "Hans".to_string(),
//! #         date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
//! #         place_of_birth: "Berlin".to_string(),
//! #         nationality: "German".to_string(),
//! #         gender: Gender::Male,
//! #         address: "Test".to_string(),
//! #     },
//! #     spouse2: Person {
//! #         name: "Maria".to_string(),
//! #         date_of_birth: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap(),
//! #         place_of_birth: "Berlin".to_string(),
//! #         nationality: "German".to_string(),
//! #         gender: Gender::Female,
//! #         address: "Test".to_string(),
//! #     },
//! #     marriage_date: NaiveDate::from_ymd_opt(2015, 6, 15).unwrap(),
//! #     place_of_marriage: "Berlin".to_string(),
//! #     registrar_office: "Standesamt Berlin-Mitte".to_string(),
//! #     status: MarriageStatus::Valid,
//! #     property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
//! #     impediments: vec![],
//! # };
//! // Calculate accrued gains
//! let calculation = AccruedGainsCalculation {
//!     spouse1_initial_assets: Assets {
//!         real_estate_value: Capital::from_euros(100_000),
//!         movable_property_value: Capital::from_euros(20_000),
//!         bank_accounts: Capital::from_euros(10_000),
//!         securities: Capital::from_euros(0),
//!         business_interests: Capital::from_euros(0),
//!         other_assets: Capital::from_euros(0),
//!         liabilities: Capital::from_euros(30_000),
//!     },
//!     spouse1_final_assets: Assets {
//!         real_estate_value: Capital::from_euros(200_000),
//!         movable_property_value: Capital::from_euros(30_000),
//!         bank_accounts: Capital::from_euros(50_000),
//!         securities: Capital::from_euros(20_000),
//!         business_interests: Capital::from_euros(0),
//!         other_assets: Capital::from_euros(0),
//!         liabilities: Capital::from_euros(50_000),
//!     },
//!     spouse2_initial_assets: Assets {
//!         real_estate_value: Capital::from_euros(0),
//!         movable_property_value: Capital::from_euros(10_000),
//!         bank_accounts: Capital::from_euros(5_000),
//!         securities: Capital::from_euros(0),
//!         business_interests: Capital::from_euros(0),
//!         other_assets: Capital::from_euros(0),
//!         liabilities: Capital::from_euros(0),
//!     },
//!     spouse2_final_assets: Assets {
//!         real_estate_value: Capital::from_euros(0),
//!         movable_property_value: Capital::from_euros(15_000),
//!         bank_accounts: Capital::from_euros(30_000),
//!         securities: Capital::from_euros(10_000),
//!         business_interests: Capital::from_euros(0),
//!         other_assets: Capital::from_euros(0),
//!         liabilities: Capital::from_euros(0),
//!     },
//!     marriage_start_date: NaiveDate::from_ymd_opt(2015, 6, 15).unwrap(),
//!     marriage_end_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//! };
//!
//! // Spouse 1 gain: €250,000 - €100,000 = €150,000
//! // Spouse 2 gain: €55,000 - €15,000 = €40,000
//! // Difference: €150,000 - €40,000 = €110,000
//! // Equalization: €110,000 / 2 = €55,000 (Spouse 2 receives)
//!
//! let (claimant, amount) = calculation.equalization_claim();
//! assert_eq!(claimant, EqualizationClaimant::Spouse2);
//! assert_eq!(amount.to_euros(), 55_000.0);
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{FamilyLawError, Result};
pub use types::*;
pub use validator::*;
