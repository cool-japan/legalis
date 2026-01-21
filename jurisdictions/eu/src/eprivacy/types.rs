//! Core types for ePrivacy Directive (Directive 2002/58/EC)
//!
//! The ePrivacy Directive complements GDPR by providing specific rules
//! for electronic communications.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Cookie or similar technology consent (Article 5(3))
///
/// The ePrivacy Directive requires prior consent for storing or accessing
/// information on a user's terminal equipment (cookies, local storage, etc.).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CookieConsent {
    /// Cookie category
    pub category: CookieCategory,
    /// Cookie purpose
    pub purpose: String,
    /// Cookie duration
    pub duration: CookieDuration,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Consent timestamp
    pub consent_timestamp: Option<DateTime<Utc>>,
    /// Whether exempt from consent requirement
    pub exempt: bool,
    /// Exemption reason (if exempt)
    pub exemption_reason: Option<CookieExemption>,
}

/// Cookie categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CookieCategory {
    /// Strictly necessary cookies (exempt from consent)
    StrictlyNecessary,
    /// Functional cookies
    Functional,
    /// Performance/Analytics cookies
    Performance,
    /// Targeting/Advertising cookies
    Targeting,
    /// Social media cookies
    SocialMedia,
}

/// Cookie duration
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CookieDuration {
    /// Session cookie (deleted when browser closed)
    Session,
    /// Persistent cookie with specified duration
    Persistent {
        /// Duration in days
        days: u32,
    },
}

/// Exemptions from cookie consent requirement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CookieExemption {
    /// Sole purpose is carrying out transmission of communication
    TransmissionOfCommunication,
    /// Strictly necessary for service explicitly requested by user
    StrictlyNecessaryForService,
}

/// Consent type under ePrivacy Directive
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConsentType {
    /// Explicit consent (opt-in)
    Explicit,
    /// Implied consent (typically not sufficient for ePrivacy)
    Implied,
    /// No consent (exempt cookies only)
    Exempt,
}

/// Cookie banner / consent mechanism
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CookieBanner {
    /// Whether banner shown before cookies placed
    pub shown_before_cookies: bool,
    /// Whether user can granularly control cookie categories
    pub granular_control: bool,
    /// Whether "Accept All" and "Reject All" options provided
    pub accept_reject_all: bool,
    /// Whether continuing to browse implies consent (cookie walls)
    pub cookie_wall: bool,
    /// Information provided about cookies
    pub information_provided: CookieInformation,
}

/// Information about cookies provided to users
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CookieInformation {
    /// Purpose of cookies explained
    pub purpose_explained: bool,
    /// Cookie duration disclosed
    pub duration_disclosed: bool,
    /// Third parties identified
    pub third_parties_identified: bool,
    /// Link to detailed cookie policy
    pub cookie_policy_link: bool,
}

/// Direct marketing communications (Article 13)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DirectMarketing {
    /// Communication channel
    pub channel: MarketingChannel,
    /// Whether consent obtained
    pub consent_obtained: bool,
    /// Consent timestamp
    pub consent_timestamp: Option<DateTime<Utc>>,
    /// Whether opt-out mechanism provided
    pub opt_out_available: bool,
    /// Whether sender identity disclosed
    pub sender_identity_disclosed: bool,
}

/// Marketing communication channels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MarketingChannel {
    /// Email
    Email,
    /// SMS
    Sms,
    /// Automated calling systems
    AutomatedCalling,
    /// Fax
    Fax,
    /// Other electronic communication
    Other(String),
}

/// Confidentiality of communications (Article 5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CommunicationConfidentiality {
    /// Whether communications are confidential
    pub confidential: bool,
    /// Whether interception/surveillance occurs
    pub interception: bool,
    /// Legal basis for any interception
    pub legal_basis: Option<String>,
}

/// Traffic data retention (Article 6)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TrafficDataRetention {
    /// Type of traffic data retained
    pub data_type: TrafficDataType,
    /// Purpose of retention
    pub purpose: RetentionPurpose,
    /// Retention period
    pub retention_period_days: u32,
    /// Whether user consent obtained (if required)
    pub consent_obtained: bool,
}

/// Types of traffic data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum TrafficDataType {
    /// Data necessary for billing
    BillingData,
    /// Location data
    LocationData,
    /// Other traffic data
    OtherTrafficData,
}

/// Purpose of traffic data retention
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RetentionPurpose {
    /// Billing purposes
    Billing,
    /// Interconnection payments
    InterconnectionPayments,
    /// Marketing (requires consent)
    Marketing,
    /// Value-added services (requires consent)
    ValueAddedServices,
}

/// Location data processing (Article 9)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct LocationDataProcessing {
    /// Type of location data
    pub data_type: LocationDataType,
    /// Purpose of processing
    pub purpose: String,
    /// Whether anonymized
    pub anonymized: bool,
    /// Whether user consent obtained
    pub consent_obtained: bool,
    /// Whether user can withdraw consent easily
    pub easy_withdrawal: bool,
    /// Whether user informed of risks
    pub risk_information_provided: bool,
}

/// Types of location data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LocationDataType {
    /// Cell-based location
    CellBased,
    /// GPS coordinates
    GpsCoordinates,
    /// Wi-Fi based location
    WiFiBased,
    /// Other location data
    Other(String),
}

/// Calling line identification (CLI) - Article 8
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CallingLineIdentification {
    /// Whether CLI presentation available
    pub cli_presentation: bool,
    /// Whether user can block CLI per call (free of charge)
    pub per_call_blocking: bool,
    /// Whether user can block CLI per line (free of charge)
    pub per_line_blocking: bool,
    /// Whether connected line presentation available
    pub connected_line_presentation: bool,
}

/// Security of communications networks (Article 4)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct NetworkSecurity {
    /// Technical measures implemented
    pub technical_measures: Vec<String>,
    /// Organizational measures implemented
    pub organizational_measures: Vec<String>,
    /// Whether users informed of security risks
    pub users_informed_of_risks: bool,
    /// Whether users informed of remedies
    pub remedies_disclosed: bool,
}
