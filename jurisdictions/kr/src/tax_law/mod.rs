//! Korean Tax Law (세법)
//!
//! # 대한민국 세법 / Tax Law of the Republic of Korea
//!
//! Covers:
//! - Income Tax Act (소득세법)
//! - Corporate Tax Act (법인세법)
//! - Value-Added Tax Act (부가가치세법)

pub mod corporate_tax;
pub mod income_tax;
pub mod vat;

pub use corporate_tax::*;
pub use income_tax::*;
pub use vat::*;
