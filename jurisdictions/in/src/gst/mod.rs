//! Goods and Services Tax (GST) Module
//!
//! # GST in India
//!
//! This module implements the Goods and Services Tax framework under the
//! Central Goods and Services Tax Act, 2017 (CGST Act), State Goods and
//! Services Tax Acts (SGST Acts), and Integrated Goods and Services Tax
//! Act, 2017 (IGST Act).
//!
//! ## Historical Background
//!
//! - GST was introduced on July 1, 2017, replacing multiple indirect taxes
//! - Constitutional amendment: 101st Amendment (2016)
//! - Subsumed taxes: Central Excise, Service Tax, VAT, CST, Entry Tax, etc.
//!
//! ## GST Structure
//!
//! India follows a **dual GST** model:
//!
//! | Transaction | Tax Component |
//! |-------------|---------------|
//! | Intra-state | CGST + SGST |
//! | Inter-state | IGST |
//! | Import | IGST |
//! | Export | Zero-rated |
//!
//! ## Registration Thresholds
//!
//! | Category | Normal States | Special Category States |
//! |----------|---------------|-------------------------|
//! | Goods | Rs. 40 lakhs | Rs. 20 lakhs |
//! | Services | Rs. 20 lakhs | Rs. 10 lakhs |
//!
//! **Special Category States**: Arunachal Pradesh, Assam, J&K, Manipur,
//! Meghalaya, Mizoram, Nagaland, Sikkim, Tripura, Himachal Pradesh,
//! Uttarakhand, Ladakh
//!
//! ## GST Rates
//!
//! | Rate | Typical Goods/Services |
//! |------|------------------------|
//! | 0% (Nil) | Essentials, healthcare, education |
//! | 5% | Basic necessities, transport |
//! | 12% | Standard goods and services |
//! | 18% | Most goods and services |
//! | 28% | Luxury items, demerit goods |
//! | 28% + Cess | Automobiles, tobacco, aerated drinks |
//!
//! ## Key Returns
//!
//! | Return | Description | Due Date |
//! |--------|-------------|----------|
//! | GSTR-1 | Outward supplies | 11th of next month |
//! | GSTR-3B | Summary + payment | 20th of next month |
//! | GSTR-9 | Annual return | 31st December |
//! | GSTR-9C | Reconciliation | 31st December |
//!
//! ## Example: Registration Requirement
//!
//! ```rust
//! use legalis_in::gst::*;
//!
//! // Check if registration is required
//! let requires_registration = validate_registration_requirement(
//!     3_500_000.0, // Rs. 35 lakhs turnover
//!     GstState::Maharashtra,
//!     SupplyCategory::Goods,
//!     false, // No inter-state supply
//!     false, // Not e-commerce operator
//! ).expect("valid check");
//!
//! // Below Rs. 40 lakhs for goods in normal state
//! assert!(!requires_registration);
//! ```
//!
//! ## Example: Tax Calculation
//!
//! ```rust
//! use legalis_in::gst::*;
//!
//! // Calculate tax on intra-state supply
//! let liability = calculate_tax(
//!     100_000.0, // Taxable value
//!     GstRate::Rate18,
//!     SupplyType::IntraState,
//!     None, // No cess
//! );
//!
//! assert_eq!(liability.cgst, 9000.0); // 9%
//! assert_eq!(liability.sgst, 9000.0); // 9%
//! assert_eq!(liability.total_tax(), 18000.0);
//! ```
//!
//! ## Example: ITC Utilization
//!
//! ```rust
//! use legalis_in::gst::*;
//!
//! // Plan ITC utilization following Section 49 order
//! let plan = validate_itc_utilization(
//!     10000.0, // IGST credit
//!     5000.0,  // CGST credit
//!     5000.0,  // SGST credit
//!     8000.0,  // IGST liability
//!     4000.0,  // CGST liability
//!     4000.0,  // SGST liability
//! ).expect("valid plan");
//!
//! // First step: IGST for IGST
//! assert_eq!(plan.steps[0].utilization, ItcUtilization::IgstForIgst);
//! ```
//!
//! ## Input Tax Credit (ITC)
//!
//! ### Conditions for Claiming ITC (Section 16(2))
//!
//! 1. Possession of tax invoice/debit note
//! 2. Receipt of goods/services
//! 3. Tax actually paid to government
//! 4. Return filed by recipient
//!
//! ### Blocked ITC (Section 17(5))
//!
//! - Motor vehicles (except for transport/training)
//! - Food, beverages, outdoor catering
//! - Works contract for construction
//! - Membership of club, health centre
//! - Personal consumption
//! - Goods lost, stolen, destroyed
//!
//! ## Composition Scheme (Section 10)
//!
//! | Business Type | Rate | Turnover Limit |
//! |---------------|------|----------------|
//! | Manufacturers | 1% | Rs. 1.5 crore |
//! | Traders | 1% | Rs. 1.5 crore |
//! | Restaurants | 5% | Rs. 1.5 crore |
//! | Service providers | 6% | Rs. 50 lakhs |
//!
//! ### Restrictions:
//! - Cannot make inter-state supplies
//! - Cannot supply through e-commerce
//! - Cannot claim ITC
//! - Cannot issue tax invoice
//!
//! ## E-Way Bill
//!
//! Required for movement of goods exceeding Rs. 50,000 value.
//!
//! | Distance | Validity |
//! |----------|----------|
//! | < 200 km | 1 day |
//! | 200-400 km | 2 days |
//! | > 400 km | 1 day per 200 km |
//!
//! ## Late Fee and Interest
//!
//! ### Late Fee (Section 47)
//!
//! | Return | Normal | Nil Return |
//! |--------|--------|------------|
//! | GSTR-1/3B | Rs. 50/day (max Rs. 5000) | Rs. 20/day (max Rs. 500) |
//! | GSTR-9 | Rs. 200/day (max 0.25% turnover) | - |
//!
//! ### Interest (Section 50)
//!
//! | Type | Rate |
//! |------|------|
//! | Delayed payment | 18% p.a. |
//! | Wrong ITC claim | 24% p.a. |
//!
//! ## References
//!
//! - [CGST Act, 2017](https://www.cbic.gov.in/resources//htdocs-cbec/gst/CGST-updated-upto-01062020.pdf)
//! - [IGST Act, 2017](https://www.cbic.gov.in/resources//htdocs-cbec/gst/51-IGST.pdf)
//! - [GST Rules, 2017](https://www.cbic.gov.in/resources//htdocs-cbec/gst/cgst-rules.pdf)
//! - [GST Portal](https://www.gst.gov.in/)
//! - [GST Council](https://gstcouncil.gov.in/)

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
