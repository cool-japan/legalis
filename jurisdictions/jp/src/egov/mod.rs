//! e-Gov Electronic Filing System
//!
//! Provides comprehensive support for Japanese government e-Gov electronic filing,
//! including XML/JSON format parsing, validation, and application management.
//!
//! ## Modules
//!
//! - `error`: Error types and result aliases for e-Gov operations
//! - `types`: Core data structures for applications, metadata, and attachments
//! - `validator`: Pre-submission validation logic
//! - `xml_parser`: XML format parsing and serialization (legacy e-Gov format)
//! - `json_format`: JSON format parsing and serialization (modern format)
//!
//! ## Examples
//!
//! ### Creating an Application
//!
//! ```
//! use legalis_jp::egov_filing::{EgovApplication, GovernmentAgency};
//!
//! let app = EgovApplication::new(
//!     "APP-2026-001",
//!     "administrative_procedure",
//!     "Tanaka Corporation",
//!     GovernmentAgency::MinistryOfJustice,
//! );
//! ```
//!
//! ### Validating an Application
//!
//! ```
//! use legalis_jp::egov_filing::{EgovApplication, GovernmentAgency, validate_application};
//!
//! let app = EgovApplication::new(
//!     "APP-2026-001",
//!     "administrative_procedure",
//!     "Tanaka Corporation",
//!     GovernmentAgency::MinistryOfJustice,
//! );
//!
//! let report = validate_application(&app)?;
//! if report.is_valid() {
//!     println!("Application is valid!");
//! }
//! # Ok::<(), legalis_jp::egov_filing::EgovError>(())
//! ```
//!
//! ### Parsing XML
//!
//! ```
//! use legalis_jp::egov_filing::EgovXmlParser;
//!
//! let parser = EgovXmlParser::new();
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <application>
//!   <metadata>
//!     <application_id>APP-001</application_id>
//!     <applicant_name>Test User</applicant_name>
//!     <!-- ... -->
//!   </metadata>
//! </application>"#;
//!
//! let app = parser.parse_application(xml)?;
//! # Ok::<(), legalis_jp::egov_filing::EgovError>(())
//! ```

pub mod error;
pub mod json_format;
pub mod types;
pub mod validator;
pub mod xml_parser;

// Re-export commonly used types
pub use error::{ApplicationStatus, EgovError, Result};
pub use json_format::EgovJsonFormatter;
pub use types::{
    ApplicationMetadata, Attachment, EgovApplication, EgovFieldValue, GovernmentAgency,
    ValidationReport,
};
pub use validator::{quick_validate, validate_application, validate_status_transition};
pub use xml_parser::EgovXmlParser;
