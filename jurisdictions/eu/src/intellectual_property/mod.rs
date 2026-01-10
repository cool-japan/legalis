//! European Union Intellectual Property Law Module
//!
//! This module provides comprehensive support for EU intellectual property law,
//! implementing key EU IP regulations and directives.
//!
//! # EU IP Framework
//!
//! The EU has harmonized IP law through several regulations and directives:
//!
//! ## EU Trademark Regulation (EU) 2017/1001
//!
//! - Unitary trademark protection across all EU member states
//! - EUTM (European Union Trademark) registration via EUIPO
//! - Protection period: 10 years, renewable indefinitely
//! - Nice Classification (Classes 1-45)
//!
//! ## Community Design Regulation (EC) No 6/2002
//!
//! - Registered Community Design (RCD)
//! - Unregistered Community Design (UCD) - 3 years protection
//! - Protection period (RCD): 25 years (5-year renewals)
//! - Covers appearance of products
//!
//! ## Copyright Directives
//!
//! ### InfoSoc Directive 2001/29/EC
//! - Harmonizes copyright and related rights
//! - Reproduction, communication, distribution rights
//! - Exceptions and limitations (quotation, parody, etc.)
//!
//! ### DSM Directive (EU) 2019/790
//! - Digital Single Market copyright rules
//! - Article 17: Online content-sharing service provider liability
//! - Articles 15-16: Press publisher rights
//!
//! ### Software Directive 2009/24/EC
//! - Legal protection of computer programs
//! - Protection as literary works
//!
//! ### Database Directive 96/9/EC
//! - Sui generis database protection
//! - 15-year protection for substantial investment
//!
//! ## Trade Secrets Directive (EU) 2016/943
//!
//! - Harmonized protection of undisclosed know-how
//! - Defines misappropriation and remedies
//! - Complementary to patent protection
//!
//! ## Patent Protection (Note)
//!
//! EU patents are primarily governed by:
//! - European Patent Convention (EPO) - not EU but covers Europe
//! - National patent laws (member state-specific)
//! - Unified Patent Court (UPC) - planned unitary patent system
//!
//! This module focuses on harmonized EU IP rights (trademarks, designs, copyright, trade secrets).
//!
//! # Features
//!
//! - EU Trademark registration validation
//! - Community Design protection assessment
//! - Copyright compliance checking (InfoSoc, DSM directives)
//! - Trade secret protection validation
//! - Infringement analysis
//! - Builder pattern for type safety
//!
//! # Examples
//!
//! ## EU Trademark Validation
//!
//! ```rust,ignore
//! use legalis_eu::intellectual_property::*;
//!
//! let trademark = EuTrademark::new()
//!     .with_mark_text("EXAMPLE")
//!     .with_applicant("Example GmbH")
//!     .add_nice_class(9) // Electronics
//!     .with_mark_type(MarkType::WordMark);
//!
//! match trademark.validate() {
//!     Ok(validation) => println!("✅ Trademark valid"),
//!     Err(e) => println!("❌ Error: {}", e),
//! }
//! ```
//!
//! ## Copyright Work Protection
//!
//! ```rust,ignore
//! use legalis_eu::intellectual_property::*;
//!
//! let work = CopyrightWork::new()
//!     .with_title("Software Application")
//!     .with_author("Jane Developer")
//!     .with_work_type(WorkType::Software)
//!     .with_originality_requirement(true);
//!
//! match work.validate_protection() {
//!     Ok(period) => println!("Protected for: {} years", period),
//!     Err(e) => println!("❌ Error: {}", e),
//! }
//! ```

pub mod copyright;
pub mod design;
pub mod error;
pub mod trade_secrets;
pub mod trademark;
pub mod types;

// Re-exports
pub use copyright::*;
pub use design::*;
pub use error::IpError;
pub use trade_secrets::*;
pub use trademark::*;
pub use types::*;
