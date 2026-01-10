//! Companies Act (Cap. 50) Module
//!
//! This module provides comprehensive support for Singapore Companies Act compliance,
//! including company formation, director requirements, corporate governance, and ACRA registration.
//!
//! ## Overview
//!
//! The Companies Act (Cap. 50) is the principal legislation governing companies in Singapore.
//! It covers:
//!
//! - **Company Formation**: Registration, name requirements, share capital
//! - **Directors**: Eligibility, resident director requirement (s. 145), disqualification
//! - **Corporate Governance**: AGM requirements (s. 175), board meetings, resolutions
//! - **ACRA Compliance**: Annual returns (s. 197), filing requirements, BizFile+
//! - **Shareholders**: Rights, share allocation, ownership limits
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use legalis_sg::companies::*;
//!
//! // Create a private limited company
//! let mut company = Company::new(
//!     "202401234A",
//!     "Tech Innovations Pte Ltd",
//!     CompanyType::PrivateLimited,
//!     Address::singapore("1 Raffles Place", "048616"),
//! );
//!
//! // Add resident director (s. 145 requirement)
//! company.directors.push(Director::new("John Tan", "S1234567A", true));
//!
//! // Validate company formation
//! match validate_company_formation(&company) {
//!     Ok(report) if report.is_valid => {
//!         println!("✅ Company structure is valid");
//!     }
//!     Ok(report) => {
//!         eprintln!("⚠️  Validation warnings:");
//!         for warning in report.warnings {
//!             eprintln!("  - {}", warning);
//!         }
//!     }
//!     Err(e) => {
//!         eprintln!("❌ Validation failed: {}", e);
//!     }
//! }
//! ```
//!
//! ## Key Requirements
//!
//! ### Section 145: Resident Director
//!
//! Every company must have at least one director who is ordinarily resident in Singapore.
//!
//! ```rust,ignore
//! use legalis_sg::companies::*;
//!
//! let directors = vec![
//!     Director::new("John Tan", "S1234567A", true),  // ✅ Resident
//!     Director::new("Jane Doe", "P1234567", false),  // Non-resident
//! ];
//!
//! validate_resident_director_requirement(&directors)?;
//! ```
//!
//! ### Section 175: Annual General Meeting (AGM)
//!
//! - **First AGM**: Within 18 months of incorporation
//! - **Subsequent AGMs**: Within 15 months of previous AGM, within 6 months of FYE
//!
//! ```rust,ignore
//! use legalis_sg::companies::*;
//! use chrono::Utc;
//!
//! let incorporation_date = Utc::now();
//! let first_agm_deadline = governance::calculate_first_agm_deadline(incorporation_date);
//! println!("First AGM due by: {}", first_agm_deadline);
//! ```
//!
//! ### Section 197: Annual Return
//!
//! Annual return must be filed within 7 months of financial year end.
//!
//! ```rust,ignore
//! use legalis_sg::companies::*;
//!
//! let deadline = validate_annual_return_deadline(&company)?;
//! println!("Annual return due by: {}", deadline);
//! ```
//!
//! ## Module Structure
//!
//! - [`types`]: Core data types (Company, Director, ShareCapital, etc.)
//! - [`error`]: Error types with bilingual messages and statute references
//! - [`validator`]: Validation functions for compliance checking
//! - [`acra`]: ACRA registration and BizFile+ utilities
//! - [`governance`]: AGM, board meetings, resolutions
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples:
//!
//! - `acra_company_registration.rs` - Complete company registration flow
//! - `director_compliance_check.rs` - Director eligibility and s. 145 validation
//! - `annual_compliance_checklist.rs` - AGM and annual return deadlines
//! - `share_issuance.rs` - Share allocation and dilution calculations

pub mod acra;
pub mod error;
pub mod governance;
pub mod types;
pub mod validator;

// Re-export commonly used types for convenience
pub use error::{CompaniesError, ErrorSeverity, Result};
pub use types::{
    Address, Company, CompanySecretary, CompanyType, Director, DirectorQualification,
    DisqualificationStatus, DividendRights, MonthDay, ShareAllocation, ShareCapital, ShareClass,
    Shareholder,
};
pub use validator::{
    ValidationReport, validate_agm_requirement, validate_annual_return_deadline,
    validate_company_formation, validate_director_eligibility,
    validate_resident_director_requirement, validate_share_capital, validate_shareholder_ownership,
};

// Re-export ACRA utilities
pub use acra::{
    BizFileStatus, BizFileSubmission, FilingRequirement, FilingType, RegistrationStatus, UenType,
    generate_uen, is_valid_company_name, validate_company_name, validate_uen,
};

// Re-export governance utilities
pub use governance::{
    AgendaItem, AnnualGeneralMeeting, Attendee, AttendeeRole, BoardMeeting, NoticeRequirement,
    Resolution, ResolutionType, VotingResult, calculate_annual_return_deadline,
    calculate_first_agm_deadline, calculate_subsequent_agm_deadline, days_until_agm_due,
    is_agm_overdue, is_annual_return_overdue, is_sufficient_notice,
};
