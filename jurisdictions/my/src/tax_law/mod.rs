//! Malaysian Tax Law
//!
//! Comprehensive tax system including:
//!
//! - **Income Tax Act 1967**: Personal and corporate income tax
//! - **Sales Tax Act 2018**: Sales tax on manufactured goods (5-10%)
//! - **Service Tax Act 2018**: Service tax on prescribed services (6%)
//! - **Stamp Duty Act 1949**: Stamp duty on instruments
//!
//! # Tax Administration
//!
//! - **LHDN**: Lembaga Hasil Dalam Negeri (Inland Revenue Board of Malaysia)
//! - **Royal Malaysian Customs**: Administers SST

pub mod income_tax;
pub mod sst;
pub mod stamp_duty;

pub use income_tax::{IncomeTax, IncomeTaxBracket, calculate_income_tax};
pub use sst::{SalesTax, ServiceTax, calculate_sales_tax, calculate_service_tax};
pub use stamp_duty::{StampDuty, StampDutyType, calculate_stamp_duty};
