//! Thai Tax Law - กฎหมายภาษีอากร
//!
//! Comprehensive tax law framework covering:
//! - Revenue Code (CIT, PIT, VAT, SBT, WHT)
//! - Customs duties
//! - Excise tax

pub mod customs;
pub mod revenue_code;

pub use customs::{CustomsDutyType, CustomsProcedure, PreferentialScheme, ValuationMethod};
pub use revenue_code::{
    CITRate, FilingPeriod, PITBracket, TaxType, VAT_REGISTRATION_THRESHOLD, VATRate,
};
