//! ePrivacy Directive (Directive 2002/58/EC) Implementation
//!
//! The ePrivacy Directive complements the GDPR by providing specific rules
//! for electronic communications, including cookies, direct marketing, and
//! confidentiality of communications.
//!
//! ## Overview
//!
//! The ePrivacy Directive (also known as the "Cookie Directive") regulates:
//!
//! 1. **Confidentiality of communications** (Article 5)
//! 2. **Traffic data and billing** (Article 6)
//! 3. **Calling line identification** (Article 8)
//! 4. **Location data** (Article 9)
//! 5. **Cookies and similar technologies** (Article 5(3))
//! 6. **Direct marketing** (Article 13)
//!
//! ## Cookie Consent (Article 5(3))
//!
//! The most well-known provision of the ePrivacy Directive requires prior consent
//! before storing or accessing information on a user's terminal equipment.
//!
//! ### Cookie Categories
//!
//! - **Strictly necessary**: Exempt from consent (e.g., shopping cart, authentication)
//! - **Functional**: Enhance user experience (requires consent)
//! - **Performance/Analytics**: Measure website performance (requires consent)
//! - **Targeting/Advertising**: Personalized ads (requires consent)
//!
//! ### Consent Requirements
//!
//! - Must be obtained **before** cookies are placed
//! - Must be **specific and informed**
//! - Users must be able to **withdraw consent easily**
//! - **Granular control**: Users should be able to accept/reject categories
//! - **Cookie walls** (forcing acceptance) are generally not compliant
//!
//! ### Exemptions
//!
//! Cookies are exempt from consent if they are:
//! 1. **Strictly necessary** for transmission of communication, OR
//! 2. **Strictly necessary** for a service explicitly requested by the user
//!
//! ## Direct Marketing (Article 13)
//!
//! ### Opt-In Requirement
//!
//! For electronic marketing (email, SMS, automated calls):
//! - **Prior consent required** (opt-in)
//! - Exception: "soft opt-in" for existing customers (products/services)
//!
//! ### Requirements
//!
//! - Sender identity must be clear
//! - Valid postal or electronic address for opt-out
//! - Opt-out must be free of charge
//! - Respect existing "do not call" registers
//!
//! ## Location Data (Article 9)
//!
//! Processing of location data requires:
//! - **Anonymization**, OR
//! - **User consent** (with easy withdrawal)
//! - Information about processing purposes
//! - Information about risks
//!
//! ## Relationship with GDPR
//!
//! The ePrivacy Directive is a **lex specialis** (special law) that:
//! - Applies specifically to electronic communications
//! - Takes precedence over GDPR in its scope
//! - Works alongside GDPR (both apply)
//!
//! ```text
//! ePrivacy Directive          GDPR
//! (specific)                  (general)
//!        ↓                       ↓
//!   Cookie consent         Data processing
//!   requirements           legal basis
//!        ↓                       ↓
//!      BOTH MUST BE SATISFIED
//! ```
//!
//! ## Example Usage
//!
//! ### Cookie Consent
//!
//! ```rust
//! use legalis_eu::eprivacy::*;
//! use chrono::Utc;
//!
//! // Strictly necessary cookie (exempt)
//! let session_cookie = CookieConsent {
//!     category: CookieCategory::StrictlyNecessary,
//!     purpose: "Maintain user session and shopping cart".to_string(),
//!     duration: CookieDuration::Session,
//!     consent_obtained: false, // Not required
//!     consent_timestamp: None,
//!     exempt: true,
//!     exemption_reason: Some(CookieExemption::StrictlyNecessaryForService),
//! };
//!
//! // Analytics cookie (requires consent)
//! let analytics_cookie = CookieConsent {
//!     category: CookieCategory::Performance,
//!     purpose: "Measure website performance and user behavior".to_string(),
//!     duration: CookieDuration::Persistent { days: 365 },
//!     consent_obtained: true,
//!     consent_timestamp: Some(Utc::now()),
//!     exempt: false,
//!     exemption_reason: None,
//! };
//! ```
//!
//! ### Cookie Banner
//!
//! ```rust
//! use legalis_eu::eprivacy::*;
//!
//! let banner = CookieBanner {
//!     shown_before_cookies: true, // MUST be shown before non-exempt cookies
//!     granular_control: true,     // Users can choose categories
//!     accept_reject_all: true,    // "Accept All" and "Reject All" buttons
//!     cookie_wall: false,         // No forced acceptance
//!     information_provided: CookieInformation {
//!         purpose_explained: true,
//!         duration_disclosed: true,
//!         third_parties_identified: true,
//!         cookie_policy_link: true,
//!     },
//! };
//! ```
//!
//! ### Direct Marketing
//!
//! ```rust
//! use legalis_eu::eprivacy::*;
//! use chrono::Utc;
//!
//! let email_marketing = DirectMarketing {
//!     channel: MarketingChannel::Email,
//!     consent_obtained: true, // Opt-in required
//!     consent_timestamp: Some(Utc::now()),
//!     opt_out_available: true, // Must provide easy opt-out
//!     sender_identity_disclosed: true,
//! };
//! ```
//!
//! ### Location Data
//!
//! ```rust
//! use legalis_eu::eprivacy::*;
//!
//! let location_processing = LocationDataProcessing {
//!     data_type: LocationDataType::GpsCoordinates,
//!     purpose: "Provide location-based recommendations".to_string(),
//!     anonymized: false,
//!     consent_obtained: true, // Required for non-anonymized data
//!     easy_withdrawal: true,  // User can withdraw consent easily
//!     risk_information_provided: true,
//! };
//! ```
//!
//! ## Future: ePrivacy Regulation
//!
//! The EU is working on replacing the ePrivacy Directive with an **ePrivacy Regulation**:
//! - Direct applicability (no national transposition needed)
//! - Broader scope (over-the-top services like WhatsApp, Messenger)
//! - Aligned with GDPR
//! - Harmonized enforcement across EU
//!
//! ## Penalties
//!
//! Penalties are set by Member States and vary. In many countries, ePrivacy violations
//! can result in fines comparable to GDPR (up to €20M or 4% of turnover).

pub mod types;

// Re-exports
pub use types::{
    CallingLineIdentification, CommunicationConfidentiality, ConsentType, CookieBanner,
    CookieCategory, CookieConsent, CookieDuration, CookieExemption, CookieInformation,
    DirectMarketing, LocationDataProcessing, LocationDataType, MarketingChannel, NetworkSecurity,
    RetentionPurpose, TrafficDataRetention, TrafficDataType,
};
