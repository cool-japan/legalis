//! Core types for Digital Services Act (DSA) and Digital Markets Act (DMA)
//!
//! This module defines the fundamental types for EU digital services regulation.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Platform type classification under DSA
///
/// Different platform types have different obligations under the DSA.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PlatformType {
    /// Intermediary service provider (Article 3)
    /// Basic caching, mere conduit services
    IntermediaryService,

    /// Hosting service provider (Article 4-5)
    /// Stores information provided by recipients
    HostingService {
        /// Average monthly active recipients in the EU
        monthly_active_recipients: u64,
    },

    /// Online platform (Article 6-13)
    /// Allows consumers and traders to conclude distance contracts
    OnlinePlatform {
        /// Average monthly active recipients in the EU
        monthly_active_recipients: u64,
        /// Whether platform allows search functionality
        has_search: bool,
    },

    /// Very Large Online Platform (VLOP) (Article 33)
    /// Platforms with 45M+ average monthly active recipients in the EU
    VeryLargeOnlinePlatform {
        /// Average monthly active recipients in the EU (must be >= 45M)
        monthly_active_recipients: u64,
        /// Date designated as VLOP by Commission
        designation_date: DateTime<Utc>,
        /// Whether designated for systemic risk assessment
        systemic_risk_designation: bool,
    },

    /// Very Large Online Search Engine (VLOSE) (Article 33)
    /// Search engines with 45M+ average monthly active recipients in the EU
    VeryLargeOnlineSearchEngine {
        /// Average monthly active recipients in the EU (must be >= 45M)
        monthly_active_recipients: u64,
        /// Date designated as VLOSE by Commission
        designation_date: DateTime<Utc>,
        /// Whether designated for systemic risk assessment
        systemic_risk_designation: bool,
    },
}

/// Illegal content categories under DSA
///
/// Content that is illegal under EU or Member State law
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum IllegalContent {
    /// Child sexual abuse material (CSAM)
    ChildSexualAbuseMaterial,

    /// Terrorist content
    TerroristContent,

    /// Incitement to violence or hatred
    IncitementToViolenceOrHatred {
        /// Target group (e.g., race, religion, nationality)
        target_group: String,
    },

    /// Intellectual property infringement
    IntellectualPropertyInfringement {
        /// Type of IP right violated
        ip_type: String,
    },

    /// Defamatory content
    Defamation,

    /// Consumer protection violations
    ConsumerProtectionViolation {
        /// Specific violation type
        violation_type: String,
    },

    /// Sale of illegal goods or services
    IllegalGoodsOrServices {
        /// Description of goods/services
        description: String,
    },

    /// Privacy violations
    PrivacyViolation,

    /// Other illegal content
    Other {
        /// Description of illegality
        description: String,
        /// Legal basis (EU or Member State law)
        legal_basis: String,
    },
}

/// Notice and action mechanism (Article 16)
///
/// Users can submit notices of illegal content
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct IllegalContentNotice {
    /// Unique identifier for the notice
    pub notice_id: String,
    /// Date and time notice was submitted
    pub submission_date: DateTime<Utc>,
    /// Type of illegal content
    pub content_type: IllegalContent,
    /// URL or identifier of content
    pub content_location: String,
    /// Explanation of why content is illegal
    pub explanation: String,
    /// Contact information of notifier
    pub notifier_contact: String,
    /// Whether notifier is a trusted flagger (Article 22)
    pub is_trusted_flagger: bool,
}

/// Platform response to illegal content notice (Article 17)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct NoticeResponse {
    /// Reference to original notice
    pub notice_id: String,
    /// Date and time of response
    pub response_date: DateTime<Utc>,
    /// Decision taken
    pub decision: NoticeDecision,
    /// Reasoning for decision
    pub reasoning: String,
    /// Information about redress (Article 20)
    pub redress_information: String,
}

/// Decision on illegal content notice
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum NoticeDecision {
    /// Content removed or disabled
    ContentRemoved {
        /// Date and time of removal
        removal_date: DateTime<Utc>,
    },
    /// Content restricted (e.g., age-gated)
    ContentRestricted {
        /// Type of restriction applied
        restriction_type: String,
    },
    /// Notice rejected
    NoticeRejected {
        /// Reason for rejection
        reason: String,
    },
    /// Under review
    UnderReview {
        /// Expected decision date
        expected_decision_date: Option<DateTime<Utc>>,
    },
}

/// Statement of reasons (Article 17)
///
/// Platforms must provide clear reasoning for content moderation decisions
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatementOfReasons {
    /// Unique identifier
    pub statement_id: String,
    /// Date issued
    pub issue_date: DateTime<Utc>,
    /// Decision taken
    pub decision: ModerationDecision,
    /// Facts and circumstances relied upon
    pub facts_and_circumstances: String,
    /// Information about use of automated means
    pub automated_decision_info: Option<AutomatedDecisionInfo>,
    /// Information about redress mechanisms
    pub redress_mechanisms: Vec<RedressMechanism>,
}

/// Content moderation decision types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ModerationDecision {
    /// Content removal
    Removal,
    /// Content demotion (reduced visibility)
    Demotion,
    /// Content restriction
    Restriction,
    /// Account suspension
    AccountSuspension {
        /// Duration of suspension
        duration_days: Option<u32>,
    },
    /// Account termination
    AccountTermination,
    /// Monetary claim
    MonetaryClaim,
}

/// Information about automated decision-making (Article 17)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AutomatedDecisionInfo {
    /// Whether decision was solely automated
    pub solely_automated: bool,
    /// Type of automated system used
    pub system_type: String,
    /// Information about human review, if any
    pub human_review: Option<String>,
}

/// Redress mechanisms (Article 20)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RedressMechanism {
    /// Internal complaint-handling system (Article 20)
    InternalComplaintSystem {
        /// URL or contact for complaints
        complaint_contact: String,
    },
    /// Out-of-court dispute settlement (Article 21)
    OutOfCourtSettlement {
        /// Certified dispute settlement body
        settlement_body: String,
    },
    /// Judicial redress
    JudicialRedress {
        /// Competent court information
        court_information: String,
    },
}

/// Systemic risks for VLOPs/VLOSEs (Article 34)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum SystemicRisk {
    /// Dissemination of illegal content
    IllegalContentDissemination,

    /// Negative effects on fundamental rights
    FundamentalRightsImpact {
        /// Right affected
        right: String,
    },

    /// Manipulative or deceptive use of service
    ManipulativeUse {
        /// Type of manipulation
        manipulation_type: String,
    },

    /// Negative effects on civic discourse and electoral processes
    CivicDiscourseImpact,

    /// Negative effects on gender-based violence
    GenderBasedViolence,

    /// Protection of minors
    MinorProtection,

    /// Effects on public health
    PublicHealthImpact,

    /// Other systemic risk
    Other {
        /// Description
        description: String,
    },
}

/// Risk mitigation measures (Article 35)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RiskMitigationMeasure {
    /// Type of measure
    pub measure_type: MitigationMeasureType,
    /// Description of measure
    pub description: String,
    /// Implementation date
    pub implementation_date: DateTime<Utc>,
    /// Expected effectiveness
    pub effectiveness_assessment: Option<String>,
}

/// Types of risk mitigation measures
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MitigationMeasureType {
    /// Adaptation of content moderation systems
    ContentModerationAdaptation,
    /// Algorithmic recommendation system changes
    RecommendationSystemChanges,
    /// Terms of service adjustments
    TermsOfServiceAdjustment,
    /// User interface design changes
    InterfaceDesignChanges,
    /// Cooperation with trusted flaggers
    TrustedFlaggerCooperation,
    /// Age verification mechanisms
    AgeVerification,
    /// Transparency measures
    TransparencyMeasures,
    /// Other measure
    Other,
}

/// Transparency reporting (Article 15, 24, 42)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TransparencyReport {
    /// Reporting period start
    pub period_start: DateTime<Utc>,
    /// Reporting period end
    pub period_end: DateTime<Utc>,
    /// Average monthly active recipients
    pub monthly_active_recipients: u64,
    /// Content moderation statistics
    pub moderation_statistics: ModerationStatistics,
    /// Notice and action statistics
    pub notice_statistics: NoticeStatistics,
    /// Algorithmic transparency (for VLOPs/VLOSEs)
    pub algorithmic_transparency: Option<AlgorithmicTransparency>,
}

/// Content moderation statistics
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ModerationStatistics {
    /// Total moderation orders from authorities
    pub authority_orders: u64,
    /// Total notices from users/trusted flaggers
    pub user_notices: u64,
    /// Total content removals
    pub content_removals: u64,
    /// Total account suspensions
    pub account_suspensions: u64,
    /// Automated vs manual decisions
    pub automated_decisions_percentage: f64,
}

/// Notice and action statistics
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct NoticeStatistics {
    /// Total notices received
    pub total_notices: u64,
    /// Notices from trusted flaggers
    pub trusted_flagger_notices: u64,
    /// Notices acted upon
    pub notices_acted_upon: u64,
    /// Average processing time in hours
    pub average_processing_time_hours: f64,
}

/// Algorithmic transparency for VLOPs/VLOSEs (Article 27)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AlgorithmicTransparency {
    /// Main parameters of recommendation systems
    pub recommendation_parameters: Vec<String>,
    /// Options for users to modify/influence recommendations
    pub user_control_options: Vec<String>,
    /// Information about profiling
    pub profiling_information: Option<String>,
}

// ============================================================================
// Digital Markets Act (DMA) Types
// ============================================================================

/// Core platform service designation (Article 2)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum CorePlatformService {
    /// Online intermediation services
    OnlineIntermediationServices,
    /// Online search engines
    OnlineSearchEngines,
    /// Online social networking services
    OnlineSocialNetworking,
    /// Video-sharing platform services
    VideoSharingPlatforms,
    /// Number-independent interpersonal communications services
    InterpersonalCommunications,
    /// Operating systems
    OperatingSystems,
    /// Web browsers
    WebBrowsers,
    /// Virtual assistants
    VirtualAssistants,
    /// Cloud computing services
    CloudComputingServices,
    /// Online advertising services
    OnlineAdvertisingServices,
}

/// Gatekeeper designation status (Article 3)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct GatekeeperDesignation {
    /// Name of gatekeeper company
    pub company_name: String,
    /// Core platform services designated
    pub designated_services: Vec<CorePlatformService>,
    /// Date of designation
    pub designation_date: DateTime<Utc>,
    /// Whether meets quantitative thresholds
    pub meets_quantitative_thresholds: QuantitativeThresholds,
    /// Whether designation is contested
    pub contested: bool,
}

/// Quantitative thresholds for gatekeeper designation (Article 3)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct QuantitativeThresholds {
    /// Annual EEA turnover >= €7.5 billion OR market capitalization >= €75 billion
    pub significant_impact_on_internal_market: bool,
    /// Provides core platform service in at least 3 Member States
    pub operates_in_multiple_member_states: bool,
    /// More than 45 million monthly active end users AND 10,000 yearly active business users
    pub substantial_user_base: bool,
    /// Met thresholds in each of last 3 financial years
    pub entrenched_and_durable_position: bool,
}

/// Gatekeeper obligations under DMA
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum GatekeeperObligation {
    /// Article 5(a) - No combining personal data without consent
    NoCombiningPersonalDataWithoutConsent,

    /// Article 5(b) - Allow users to un-install pre-installed software
    AllowUninstallPreinstalledSoftware,

    /// Article 5(c) - Allow third-party app stores and sideloading
    AllowThirdPartyAppStores,

    /// Article 5(d) - No leveraging data from business users
    NoLeveragingBusinessUserData,

    /// Article 5(e) - No requiring use of gatekeeper's services
    NoTyingOfServices,

    /// Article 5(f) - No preferential treatment in ranking
    NoSelfPreferencingInRanking,

    /// Article 5(g) - No restricting data portability
    EnableDataPortability,

    /// Article 5(h) - Provide access to data for business users
    ProvideBusinessUserDataAccess,

    /// Article 6(a) - Allow third-party interoperability
    AllowThirdPartyInteroperability,

    /// Article 6(b) - Provide effective data portability tools
    ProvideDataPortabilityTools,

    /// Article 6(c) - Provide business users with access to data
    ProvideBusinessUserAccessToData,

    /// Article 6(d) - Effective unsubscribe for core platform services
    EffectiveUnsubscribe,

    /// Article 6(e) - No tracking outside core platform without consent
    NoTrackingWithoutConsent,

    /// Article 6(f) - Allow end users to choose browser, search engine, etc.
    AllowUserChoiceOfDefaults,

    /// Article 6(g) - Provide advertisers and publishers with performance data
    ProvideAdvertisingPerformanceData,

    /// Article 6(h) - Fair, reasonable, non-discriminatory (FRAND) access
    FrandAccess,

    /// Article 6(i) - Provide real-time access to data for ranking queries
    ProvideRealTimeRankingData,

    /// Article 6(j) - Allow business users to promote offers to end users
    AllowBusinessUserPromotions,

    /// Article 6(k) - Apply fair and non-discriminatory terms for app stores
    FairAppStoreTerms,

    /// Article 6(l) - Enable switching between operating systems
    EnableOperatingSystemSwitching,
}

/// Interoperability requirement (Article 7)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InteroperabilityRequirement {
    /// Type of service requiring interoperability
    pub service_type: CorePlatformService,
    /// Description of interoperability obligation
    pub description: String,
    /// Free or FRAND basis
    pub access_terms: InteroperabilityAccessTerms,
    /// Timeline for implementation
    pub implementation_deadline: DateTime<Utc>,
}

/// Access terms for interoperability
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum InteroperabilityAccessTerms {
    /// Free access
    Free,
    /// Fair, Reasonable, and Non-Discriminatory (FRAND) terms
    Frand {
        /// Fee structure
        fee_structure: Option<String>,
    },
}

/// DMA compliance report
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DmaComplianceReport {
    /// Gatekeeper information
    pub gatekeeper: GatekeeperDesignation,
    /// Reporting period
    pub period_start: DateTime<Utc>,
    /// Reporting period end
    pub period_end: DateTime<Utc>,
    /// Compliance status for each obligation
    pub obligation_compliance: Vec<ObligationCompliance>,
    /// Measures taken to ensure compliance
    pub compliance_measures: Vec<String>,
}

/// Compliance status for specific obligation
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ObligationCompliance {
    /// The obligation
    pub obligation: GatekeeperObligation,
    /// Whether compliant
    pub compliant: bool,
    /// Explanation
    pub explanation: String,
}
