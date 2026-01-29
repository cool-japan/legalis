//! Korean Labor Law (근로기준법 등)
//!
//! # 대한민국 노동법 / Labor Law of the Republic of Korea
//!
//! Covers:
//! - Labor Standards Act (근로기준법)
//! - Employment Insurance Act (고용보험법)
//! - Industrial Accident Compensation Insurance Act (산업재해보상보험법)

pub mod employment_insurance;
pub mod labor_standards;
pub mod workers_compensation;

pub use employment_insurance::*;
pub use labor_standards::*;
pub use workers_compensation::*;
