//! Administrative Law of Lao PDR (ກົດໝາຍບໍລິຫານ)
//!
//! This module implements administrative law types, validation, and error handling
//! for the Lao People's Democratic Republic, based on:
//!
//! - **Administrative Procedure Law** (ກົດໝາຍວ່າດ້ວຍຂັ້ນຕອນການບໍລິຫານ)
//! - **State Liability Law** (ກົດໝາຍວ່າດ້ວຍຄວາມຮັບຜິດຊອບຂອງລັດ)
//! - **Law on People's Courts** (ກົດໝາຍວ່າດ້ວຍສານປະຊາຊົນ)
//!
//! ## Legal Framework
//!
//! Administrative law in Lao PDR governs the relationship between citizens
//! and the state, including:
//!
//! ### Administrative Decisions (ການຕັດສິນໃຈບໍລິຫານ)
//! - Licensing and permits
//! - Administrative approvals and denials
//! - Revocations and suspensions
//! - Administrative orders and fines
//!
//! ### Administrative Procedure (ຂັ້ນຕອນການບໍລິຫານ)
//! - Proper authority verification
//! - Legal basis requirements
//! - Notification to affected parties
//! - Appeal deadlines and procedures
//!
//! ### Administrative Appeals (ການອຸທອນບໍລິຫານ)
//! - Administrative reconsideration (30 days)
//! - Appeal to superior authority
//! - Judicial review (60 days)
//!
//! ### State Liability (ຄວາມຮັບຜິດຊອບຂອງລັດ)
//! - Wrongful administrative decisions
//! - Procedural violations
//! - Negligence and excess of authority
//! - 2-year claim deadline
//!
//! ## Authority Hierarchy
//!
//! Administrative decisions are issued at four levels:
//! - **Central** (ສູນກາງ): Ministries and central agencies
//! - **Provincial** (ແຂວງ): Provincial government offices
//! - **District** (ເມືອງ): District government offices
//! - **Village** (ບ້ານ): Village administrative units
//!
//! ## Appeal Deadlines
//!
//! - Administrative appeal: 30 days from notification
//! - Court appeal: 60 days from administrative decision
//! - State liability claim: 2 years from wrongful act
//!
//! ## Example Usage
//!
//! ### Creating an Administrative Decision
//!
//! ```
//! use legalis_la::administrative_law::{
//!     AdministrativeDecision, AdministrativeLevel, DecisionType,
//!     LicenseType, LegalBasis, AffectedParty, PartyType,
//! };
//!
//! let decision = AdministrativeDecision::builder()
//!     .decision_number("DEC-2024-001".to_string())
//!     .issuing_authority(AdministrativeLevel::Central {
//!         ministry: "Ministry of Industry and Commerce".to_string(),
//!     })
//!     .decision_date("2024-01-15".to_string())
//!     .subject_lao("ການອອກໃບອະນຸຍາດປະກອບທຸລະກິດ".to_string())
//!     .subject_en("Business License Issuance".to_string())
//!     .decision_type(DecisionType::License {
//!         license_type: LicenseType::BusinessLicense,
//!     })
//!     .legal_basis(LegalBasis {
//!         law_name_lao: "ກົດໝາຍວ່າດ້ວຍວິສາຫະກິດ".to_string(),
//!         law_name_en: "Enterprise Law".to_string(),
//!         article_number: 15,
//!         paragraph: Some(1),
//!     })
//!     .affected_party(AffectedParty {
//!         party_name: "ABC Company Ltd.".to_string(),
//!         party_type: PartyType::LegalEntity,
//!         notification_date: Some("2024-01-15".to_string()),
//!         is_notified: true,
//!     })
//!     .is_final(false)
//!     .appeal_deadline_days(Some(30))
//!     .build();
//!
//! assert!(decision.is_ok());
//! ```
//!
//! ### Validating an Appeal Deadline
//!
//! ```
//! use legalis_la::administrative_law::validate_appeal_deadline;
//!
//! // Appeal filed within 30 days - valid
//! assert!(validate_appeal_deadline(25, 30).is_ok());
//!
//! // Appeal filed after deadline - invalid
//! assert!(validate_appeal_deadline(35, 30).is_err());
//! ```
//!
//! ### Filing a State Liability Claim
//!
//! ```
//! use legalis_la::administrative_law::{
//!     StateLiability, LiabilityType, ClaimStatus,
//!     AdministrativeLevel, AffectedParty, PartyType,
//! };
//!
//! let claim = StateLiability {
//!     claim_number: "SLC-2024-001".to_string(),
//!     claimant: AffectedParty {
//!         party_name: "John Doe".to_string(),
//!         party_type: PartyType::Individual,
//!         notification_date: None,
//!         is_notified: false,
//!     },
//!     responsible_authority: AdministrativeLevel::Provincial {
//!         province: "Vientiane Capital".to_string(),
//!     },
//!     liability_type: LiabilityType::WrongfulDecision,
//!     damage_description_lao: "ຄວາມເສຍຫາຍຈາກການຕັດສິນໃຈທີ່ຜິດກົດໝາຍ".to_string(),
//!     damage_description_en: "Damage from wrongful administrative decision".to_string(),
//!     claimed_amount_lak: 50_000_000,
//!     claim_status: ClaimStatus::Filed,
//!     wrongful_act_date: Some("2024-01-15".to_string()),
//!     filing_date: Some("2024-02-01".to_string()),
//!     supporting_evidence: vec!["Document A".to_string(), "Document B".to_string()],
//! };
//! ```
//!
//! ## Bilingual Support
//!
//! All types and error messages support both Lao (ພາສາລາວ) and English languages,
//! reflecting the bilingual nature of legal practice in Lao PDR.

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{AdministrativeLawError, AdministrativeLawResult};
pub use types::{
    // Constants
    ADMINISTRATIVE_APPEAL_DEADLINE_DAYS,
    // Administrative appeals
    AdministrativeAppeal,
    AdministrativeAppealBuilder,
    // Administrative decision types
    AdministrativeDecision,
    AdministrativeDecisionBuilder,
    // Administrative authority levels
    AdministrativeLevel,
    // Administrative sanctions
    AdministrativeSanction,
    AdministrativeSanctionBuilder,
    // Affected parties
    AffectedParty,
    AppealGround,
    AppealLevel,
    AppealOutcome,
    AppealStatus,
    COURT_APPEAL_DEADLINE_DAYS,
    ClaimStatus,
    DecisionType,
    // Legal basis
    LegalBasis,
    LiabilityType,
    // License types
    LicenseType,
    MAXIMUM_SUSPENSION_DAYS,
    MINIMUM_FINE_AMOUNT_LAK,
    // Order types
    OrderType,
    PartyType,
    // Permit types
    PermitType,
    STATE_LIABILITY_CLAIM_DEADLINE_YEARS,
    SanctionType,
    // State liability
    StateLiability,
};
pub use validator::{
    validate_administrative_appeal, validate_administrative_decision, validate_appeal_deadline,
    validate_authority_jurisdiction, validate_legal_basis, validate_license_application,
    validate_notification, validate_permit_application, validate_proportionality,
    validate_sanction, validate_state_liability_claim,
};
