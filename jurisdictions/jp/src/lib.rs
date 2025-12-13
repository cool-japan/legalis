//! Japanese jurisdiction support for Legalis-RS.
//!
//! This crate provides:
//! - Japanese era (和暦) handling
//! - e-Gov XML law parser
//! - Japanese Constitution support
//! - Civil Code (民法 - Minpo) implementation
//! - Bilingual (Japanese/English) statute handling

pub mod constitution;
pub mod egov;
pub mod era;
pub mod law;
pub mod minpo;

pub use constitution::{Constitution, ConstitutionArticle, ConstitutionChapter};
pub use egov::{EGovArticle, EGovLaw, EGovLawParser};
pub use era::{Era, EraError, JapaneseDate};
pub use law::{JapaneseLaw, LawNumber, LawType};
pub use minpo::{article_709, article_710, article_715_1};
