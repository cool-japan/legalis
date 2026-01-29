//! Korean Civil Code (민법)
//!
//! # 대한민국 민법 / Civil Code of the Republic of Korea
//!
//! Enacted: 1958
//! Total Articles: 1,118
//!
//! The Civil Code consists of five parts:
//! 1. General Provisions (총칙편)
//! 2. Property Law (물권법)
//! 3. Obligations (채권법)
//! 4. Family Law (가족법)
//! 5. Succession (상속법)

pub mod family;
pub mod general_provisions;
pub mod obligations;
pub mod property;
pub mod succession;

pub use family::*;
pub use general_provisions::*;
pub use obligations::*;
pub use property::*;
pub use succession::*;
