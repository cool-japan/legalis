//! Japanese jurisdiction support for Legalis-RS.
//!
//! This crate provides:
//! - Japanese era (和暦) handling
//! - e-Gov XML law parser
//! - Japanese Constitution support
//! - Bilingual (Japanese/English) statute handling

pub mod constitution;
pub mod egov;
pub mod era;
pub mod law;

pub use constitution::{Constitution, ConstitutionArticle, ConstitutionChapter};
pub use egov::{EGovArticle, EGovLaw, EGovLawParser};
pub use era::{Era, EraError, JapaneseDate};
pub use law::{JapaneseLaw, LawNumber, LawType};
