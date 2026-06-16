//! Construction Law Module for Lao PDR (ກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ)
//!
//! This module models the **Law on Construction (Lao PDR), No. 05/NA, 2009**
//! (ກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ).
//!
//! # Legal Framework
//!
//! The Law on Construction governs the planning, permitting, execution,
//! supervision, inspection and acceptance of construction works in the Lao
//! People's Democratic Republic, together with the licensing and grading of
//! contractors and their responsibility for defects after completion. It is
//! administered primarily through the Ministry of Public Works and Transport.
//!
//! # Key Provisions Modelled
//!
//! - **Building permits** — a construction/building permit must be obtained
//!   before construction begins ([`BuildingPermit`], [`validate_building_permit`],
//!   [`validate_construction_authorized`]).
//! - **Construction categories** — works classified by scale/category
//!   ([`ConstructionCategory`]).
//! - **Contractor licensing and grading** — contractors must be licensed and are
//!   graded by capacity, which limits the value of projects they may undertake
//!   ([`Contractor`], [`ContractorGrade`], [`validate_contractor_license`],
//!   [`validate_contractor_grade`]).
//! - **Technical standards and on-site safety** — a safety plan is required
//!   ([`validate_safety_plan`]).
//! - **Staged inspection and acceptance** — foundation, structure and completion
//!   inspections in sequence, with acceptance/handover before occupancy
//!   ([`InspectionStage`], [`Inspection`], [`WorkAcceptance`],
//!   [`validate_inspection_sequence`], [`validate_work_acceptance`]).
//! - **Defects-liability (warranty) period** — the contractor remains responsible
//!   for defects during a statutory period ([`ConstructionContract`],
//!   [`validate_defects_liability`], [`DEFECTS_LIABILITY_PERIOD_MONTHS`]).
//!
//! # Legal Accuracy Note
//!
//! Where the precise internal article numbers of the 2009 law cannot be
//! independently verified by this crate, provisions are cited by the law's name
//! and year ([`CONSTRUCTION_LAW_CITATION`]) together with a documented topic
//! descriptor, and quantifiable requirements are encoded as named constants (for
//! example [`DEFECTS_LIABILITY_PERIOD_MONTHS`]) or as validated fields (for
//! example the `max_project_value_lak` field on [`Contractor`]) rather than as
//! fabricated article references or asserted statutory figures.
//!
//! # Example
//!
//! ```
//! use legalis_la::construction_law::*;
//!
//! let permit = BuildingPermit {
//!     project_name: "Vientiane riverside housing".to_string(),
//!     category: ConstructionCategory::Medium,
//!     applicant: "Mekong Developments Co.".to_string(),
//!     issued: true,
//!     status: PermitStatus::Approved,
//! };
//!
//! assert!(validate_building_permit(&permit).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{CONSTRUCTION_LAW_CITATION, ConstructionLawError, ConstructionResult};

pub use types::{
    // Permits
    BuildingPermit,
    // Classification
    ConstructionCategory,
    // Projects & contracts
    ConstructionContract,
    ConstructionProject,
    // Contractors
    Contractor,
    ContractorGrade,
    // Constants
    DEFECTS_LIABILITY_PERIOD_MONTHS,
    // Inspection & acceptance
    Inspection,
    InspectionStage,
    PermitStatus,
    ProjectType,
    WorkAcceptance,
};

pub use validator::{
    validate_building_permit, validate_construction_authorized, validate_contractor_grade,
    validate_contractor_license, validate_defects_liability, validate_inspection_sequence,
    validate_safety_plan, validate_work_acceptance,
};
