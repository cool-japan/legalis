//! BGB Property Law Module (Sachenrecht - Book 3)
//!
//! Comprehensive implementation of German property law under BGB Book 3 (§§854-1296 BGB).
//!
//! # Legal Context
//!
//! German property law (Sachenrecht) governs rights in things (Sachen), including:
//! - **Ownership (Eigentum)** - §§903-1011 BGB
//! - **Possession (Besitz)** - §§854-872 BGB
//! - **Transfer of Ownership** - §§929-936 (movables), §§873-902 (immovables)
//! - **Limited Real Rights** - Easements, mortgages, pledges
//!
//! ## Core Principles
//!
//! ### 1. Abstraction Principle (Abstraktionsprinzip)
//!
//! German law separates:
//! - **Obligatory agreement** (Verpflichtungsgeschäft) - creates obligation to transfer
//! - **Disposition** (Verfügungsgeschäft) - actual transfer of ownership
//!
//! Example: Sales contract (§433 BGB) creates obligation, but transfer (§929 BGB) transfers ownership.
//!
//! ### 2. Transfer of Movables (§§929-936 BGB)
//!
//! #### §929 Sentence 1 - Standard Transfer
//! Requirements:
//! - Agreement (Einigung) between transferor and transferee
//! - Delivery (Übergabe) of the thing
//! - Transferor has authority to transfer (Verfügungsbefugnis)
//!
//! #### §930 - Constructive Possession
//! Transfer without delivery when transferee already has possession through
//! a possessory relationship (e.g., lease).
//!
//! #### §931 - Assignment of Claim
//! Transfer when third party has possession - transferor assigns herausgabe claim.
//!
//! ### 3. Transfer of Immovables (§§873-902 BGB)
//!
//! Requirements:
//! - Agreement (Einigung) between transferor and transferee
//! - Land registry entry (Grundbucheintragung) - §873 Abs. 1 BGB
//! - Form requirement: Notarial certification (§925 BGB)
//!
//! ### 4. Possession (Besitz) - §§854-872 BGB
//!
//! Possession requires:
//! - **Factual control** (tatsächliche Gewalt) - §854 BGB
//! - **Possession will** (Besitzwille)
//!
//! Types:
//! - **Direct possession** (unmittelbarer Besitz) - §854
//! - **Indirect possession** (mittelbarer Besitz) - §868
//! - **Owner possession** (Eigenbesitz) - §872
//! - **Holder possession** (Fremdbesitz) - possession for another
//!
//! Possession protection (Besitzschutz):
//! - §861 BGB - Restitution claim (Rückgabe bei Entziehung)
//! - §862 BGB - Cessation/prohibition claim (Beseitigung/Unterlassung)
//! - §864 BGB - One-year limitation period
//!
//! ### 5. Good Faith Acquisition (Gutgläubiger Erwerb) - §§932-936 BGB
//!
//! Requirements (§932 BGB):
//! - Transferor not the owner
//! - Acquirer in good faith (guter Glaube)
//! - No gross negligence (keine grobe Fahrlässigkeit)
//! - Voluntary transfer (freiwillige Übertragung)
//!
//! Exception (§935 BGB):
//! - Lost or stolen things (abhanden gekommene Sachen) cannot be acquired in good faith
//! - Exception to exception: Money and bearer instruments
//!
//! ### 6. Easements (Dienstbarkeiten) - §§1018-1093 BGB
//!
//! Types:
//! - **Predial easement** (Grunddienstbarkeit §1018) - benefits land parcel
//! - **Personal easement** (beschränkte persönliche Dienstbarkeit §1090) - benefits person
//! - **Usufruct** (Nießbrauch §1030) - right to use and enjoy
//!
//! Establishment:
//! - Agreement + land registry entry
//! - Prescription (Ersitzung) - §1028 BGB
//!
//! ### 7. Mortgages and Land Charges (§§1113-1203 BGB)
//!
//! #### Mortgage (Hypothek §1113 BGB)
//! - Accessory to claim (akzessorisch)
//! - Secures specific claim
//! - Claim and mortgage inseparable
//!
//! #### Land Charge (Grundschuld §1191 BGB)
//! - Non-accessory (nicht akzessorisch)
//! - Independent of underlying claim
//! - More flexible, commonly used in practice
//!
//! #### Owner Land Charge (Eigentümergrundschuld)
//! - Land charge in favor of owner
//! - Protects priority rank
//!
//! ### 8. Pledges (Pfandrecht) - §§1204-1259 BGB
//!
//! #### Movable Pledge (§1204 BGB)
//! Requirements:
//! - Agreement between pledgor and pledgee
//! - Transfer of possession (Besitzübertragung §1205)
//! - Secured claim
//!
//! #### Rights Pledge (§1273 BGB)
//! Pledge on claims, shares, intellectual property rights.
//!
//! # Module Structure
//!
//! This module provides:
//! - Comprehensive type system for property law concepts
//! - Builder patterns for ergonomic construction
//! - Validation functions implementing BGB requirements
//! - Bilingual error messages (German/English)
//!
//! # Examples
//!
//! ## Transfer of Movable Property (§929 BGB)
//!
//! ```rust
//! use legalis_de::bgb::sachenrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Create a valid movable transfer
//! let transfer = MovableTransferBuilder::new()
//!     .transferor("Max Mustermann", "Berlin")
//!     .transferee("Erika Schmidt", "Munich")
//!     .thing("Used car - VW Golf", Capital::from_euros(15_000))
//!     .transfer_type(MovableTransferType::ActualDelivery)
//!     .agreement(Utc::now())
//!     .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
//!     .consideration(Capital::from_euros(15_000))
//!     .good_faith(true)
//!     .build()?;
//!
//! // Validate the transfer
//! validate_movable_transfer(&transfer)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Possession (§854 BGB)
//!
//! ```rust
//! use legalis_de::bgb::sachenrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! let possession = Possession {
//!     possessor: PropertyParty {
//!         name: "Hans Müller".to_string(),
//!         address: Some("Frankfurt".to_string()),
//!         date_of_birth: None,
//!         is_natural_person: true,
//!     },
//!     thing: Thing {
//!         description: "Bicycle".to_string(),
//!         property_type: PropertyType::Movable,
//!         value: Capital::from_euros(500),
//!         is_consumable: false,
//!         is_fungible: false,
//!         location: Some("Frankfurt".to_string()),
//!     },
//!     possession_type: PossessionType::DirectPossession,
//!     acquired_at: Utc::now(),
//!     factual_control: true,
//!     possession_will: true,
//! };
//!
//! validate_possession(&possession)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Good Faith Acquisition (§932 BGB)
//!
//! ```rust
//! use legalis_de::bgb::sachenrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Transfer from non-owner
//! let transfer = MovableTransferBuilder::new()
//!     .transferor("Non-Owner", "Berlin")
//!     .transferee("Good Faith Acquirer", "Munich")
//!     .thing("Laptop", Capital::from_euros(1_000))
//!     .transfer_type(MovableTransferType::ActualDelivery)
//!     .agreement(Utc::now())
//!     .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
//!     .consideration(Capital::from_euros(1_000))
//!     .good_faith(true)
//!     .build()?;
//!
//! // Good faith acquisition analysis
//! let acquisition = GoodFaithAcquisition {
//!     transfer,
//!     transferor_not_owner: true,
//!     good_faith: true,                         // §932 Abs. 1
//!     no_gross_negligence: true,                // §932 Abs. 2
//!     acquired_through_voluntary_transfer: true, // §935
//!     acquisition_valid: true,
//! };
//!
//! validate_good_faith_acquisition(&acquisition)?;
//! // Acquirer becomes owner despite transferor not being owner
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Land Parcel Transfer (§873 BGB)
//!
//! ```rust
//! use legalis_de::bgb::sachenrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::{NaiveDate, Utc};
//!
//! let land_transfer = ImmovableTransfer {
//!     transferor: PropertyParty {
//!         name: "Seller GmbH".to_string(),
//!         address: Some("Berlin".to_string()),
//!         date_of_birth: None,
//!         is_natural_person: false,
//!     },
//!     transferee: PropertyParty {
//!         name: "Buyer AG".to_string(),
//!         address: Some("Munich".to_string()),
//!         date_of_birth: None,
//!         is_natural_person: false,
//!     },
//!     land_parcel: LandParcel {
//!         parcel_number: "123/45".to_string(),
//!         land_registry_district: "Berlin-Mitte".to_string(),
//!         size_square_meters: 500,
//!         location: "Alexanderplatz 1, 10178 Berlin".to_string(),
//!         description: "Commercial property".to_string(),
//!         value: Capital::from_euros(1_000_000),
//!     },
//!     agreement: TransferAgreement {
//!         agreement_reached: true,
//!         agreed_at: Utc::now(),
//!         transfer_intent: true,
//!         acceptance_intent: true,
//!     },
//!     registration: LandRegistryEntry {
//!         registered: true,
//!         registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
//!         registry_office: "Amtsgericht Berlin-Mitte".to_string(),
//!         section: LandRegistrySection::SectionI,
//!         entry_number: Some("Bl. 1234".to_string()),
//!     },
//!     consideration: Some(Capital::from_euros(1_000_000)),
//!     transferred_at: Utc::now(),
//! };
//!
//! validate_immovable_transfer(&land_transfer)?;
//! // Ownership transferred upon land registry entry (§873 Abs. 1 BGB)
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{PropertyError, Result};
pub use types::*;
pub use validator::*;
