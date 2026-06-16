//! Consumer Protection Law Module for Lao PDR (ກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ)
//!
//! This module models the **Law on Consumer Protection (Lao PDR), No. 02/NA,
//! 30 June 2010** (ກົດໝາຍວ່າດ້ວຍການປົກປ້ອງຜູ້ບໍລິໂພກ).
//!
//! # Legal Framework
//!
//! The Law on Consumer Protection establishes the rights of consumers, the
//! obligations of business operators (suppliers), prohibited trade practices,
//! product information and safety requirements, and the mechanisms for resolving
//! consumer disputes in the Lao People's Democratic Republic. It is administered
//! primarily through the Ministry of Industry and Commerce.
//!
//! # Key Provisions Modelled
//!
//! - **Consumer rights** — the eight internationally recognised fundamental
//!   consumer rights adopted as the framework of the Lao regime
//!   ([`ConsumerRight`]).
//! - **Supplier obligations** — accurate information, Lao-language labelling,
//!   product safety, fair contract terms, warranties and redress
//!   ([`SupplierObligation`]).
//! - **Prohibited practices** — false advertising, unfair contract terms, unsafe
//!   goods, hoarding/price manipulation, short measure, forced sales
//!   ([`ProhibitedPractice`]).
//! - **Product labelling** — mandatory Lao-language labelling and required
//!   information ([`ProductLabel`], [`validate_product_label`]).
//! - **Product safety and recalls** ([`ProductSafetyAssessment`],
//!   [`ProductRecall`]).
//! - **Complaints, redress and dispute resolution** ([`ConsumerComplaint`],
//!   [`Redress`], [`DisputeResolutionMethod`]).
//! - **Administrative sanctions** with a proportionality check
//!   ([`SanctionType`], [`validate_sanction`]).
//!
//! # Legal Accuracy Note
//!
//! Where the precise internal article numbers of the 2010 law cannot be
//! independently verified by this crate, provisions are cited by the law's name
//! and year ([`CONSUMER_PROTECTION_LAW_CITATION`]) together with a documented
//! topic descriptor, and quantifiable requirements are encoded as named
//! constants (for example [`REQUIRED_LABEL_LANGUAGE`]) rather than as fabricated
//! article references.
//!
//! # Example
//!
//! ```
//! use legalis_la::consumer_protection_law::*;
//!
//! let label = ProductLabel {
//!     product_name: "Padaek (fermented fish)".to_string(),
//!     languages: vec!["Lao".to_string(), "English".to_string()],
//!     has_manufacturer_info: true,
//!     manufacture_date: Some("2025-03-01".to_string()),
//!     expiry_date: Some("2026-03-01".to_string()),
//!     has_net_quantity: true,
//!     has_usage_instructions: true,
//!     has_safety_warnings: false,
//!     requires_safety_warnings: false,
//! };
//!
//! assert!(validate_product_label(&label).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{
    CONSUMER_PROTECTION_LAW_CITATION, ConsumerProtectionError, ConsumerProtectionResult,
};

pub use types::{
    // Complaint & status
    ComplaintStatus,
    // Consumer rights & supplier obligations
    ConsumerComplaint,
    ConsumerContract,
    ConsumerRight,
    ContractTermType,
    DisputeResolutionMethod,
    // Constants
    FUNDAMENTAL_CONSUMER_RIGHTS_COUNT,
    HazardSeverity,
    ProductLabel,
    ProductRecall,
    ProductSafetyAssessment,
    ProhibitedPractice,
    REQUIRED_LABEL_LANGUAGE,
    Redress,
    RedressType,
    SanctionType,
    SupplierObligation,
};

pub use validator::{
    validate_advertising, validate_complaint, validate_consumer_contract,
    validate_dispute_escalation, validate_product_label, validate_product_recall,
    validate_product_safety, validate_redress, validate_sanction, validate_supplier_obligation,
};
