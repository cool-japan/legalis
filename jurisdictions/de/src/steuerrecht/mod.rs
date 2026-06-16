//! German Tax Law (Steuerrecht) - EStG, UStG, AO
//!
//! Type-safe representations and computations for selected provisions of German
//! tax law:
//!
//! - [`estg`] - Income Tax Act (Einkommensteuergesetz): types of income
//!   (Einkunftsarten, § 2 EStG) and the income-tax tariff (§ 32a EStG).
//! - [`ustg`] - VAT Act (Umsatzsteuergesetz): taxability (Steuerbarkeit, § 1) and
//!   the standard / reduced tax rates (§ 12 UStG).
//! - [`ao`] - Fiscal Code (Abgabenordnung): tax assessment (Steuerbescheid),
//!   periods (Fristen) and the limitation of assessment (Festsetzungsverjährung,
//!   §§ 169-171 AO).

pub mod ao;
pub mod error;
pub mod estg;
pub mod ustg;

pub use ao::*;
pub use error::{Result, SteuerError};
pub use estg::*;
pub use ustg::*;
