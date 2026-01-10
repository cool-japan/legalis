//! Commercial Law Module (商法・会社法モジュール)
//!
//! This module provides comprehensive support for Japanese commercial law,
//! including the Companies Act (会社法 - Kaisha-hō) and Commercial Code (商法 - Shōhō).
//!
//! # Features
//!
//! - Company formation and registration (会社設立)
//! - Corporate governance structures (コーポレートガバナンス)
//! - Share management and transfers (株式管理・譲渡)
//! - Shareholders meetings and resolutions (株主総会・決議)
//! - Board of directors and corporate auditors (取締役会・監査役)
//! - Commercial transactions (商取引)
//! - Type-safe validation and error handling
//!
//! # Examples
//!
//! ## Creating Articles of Incorporation
//!
//! ```rust
//! use legalis_jp::commercial_law::*;
//!
//! let articles = ArticlesOfIncorporation {
//!     company_name: "テクノロジー株式会社".to_string(),
//!     business_purposes: vec![
//!         "Software development and consulting".to_string(),
//!         "IT infrastructure management".to_string(),
//!     ],
//!     head_office_location: "Tokyo, Japan".to_string(),
//!     authorized_shares: Some(10_000),
//!     capital: Capital::new(10_000_000),
//!     fiscal_year_end_month: 3,
//!     incorporators: vec![
//!         Incorporator {
//!             name: "Founder 1".to_string(),
//!             address: "Tokyo".to_string(),
//!             shares_subscribed: Some(10_000),
//!             investment_amount_jpy: 10_000_000,
//!         },
//!     ],
//!     establishment_date: None,
//! };
//!
//! // Validate articles
//! assert!(validate_articles_of_incorporation(&articles, CompanyType::StockCompany).is_ok());
//! ```
//!
//! ## Validating Shareholders Meeting
//!
//! ```rust
//! use legalis_jp::commercial_law::*;
//! use chrono::Utc;
//!
//! let meeting = ShareholdersMeeting {
//!     meeting_type: MeetingType::OrdinaryGeneralMeeting,
//!     meeting_date: Utc::now(),
//!     agenda_items: vec![
//!         AgendaItem {
//!             item_number: 1,
//!             description: "Approve financial statements".to_string(),
//!             resolution_type: ResolutionType::OrdinaryResolution,
//!             votes_favor: 600,
//!             votes_against: 200,
//!             abstentions: 200,
//!             result: Some(ResolutionResult::Approved),
//!         },
//!     ],
//!     quorum_met: true,
//!     voting_rights_present: 1000,
//!     voting_rights_total: 1500,
//! };
//!
//! assert!(validate_shareholders_meeting_resolution(&meeting).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{CommercialLawError, Result};
pub use types::*;
pub use validator::*;
