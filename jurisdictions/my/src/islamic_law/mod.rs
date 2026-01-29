//! Islamic Law (Syariah) for Malaysia
//!
//! Syariah law applies to Muslims in Malaysia for matters under State List (Schedule 9, List II).
//!
//! # Jurisdiction
//!
//! - **Family Law**: Marriage, divorce, custody, maintenance
//! - **Inheritance**: Islamic inheritance (faraid)
//! - **Islamic Finance**: Syariah-compliant financial products
//!
//! # Syariah Courts
//!
//! - Syariah Appeal Court
//! - Syariah High Court
//! - Syariah Subordinate Court

pub mod family_law;
pub mod finance;

pub use family_law::{
    IslamicDivorce, IslamicMarriage, validate_islamic_marriage,
    validate_shariah_compliance as validate_family_law,
};
pub use finance::{
    IslamicContract, IslamicFinanceProduct, IslamicFinanceType, validate_shariah_compliance,
};
