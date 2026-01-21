//! Indonesian Civil Code (KUHPerdata) - Kitab Undang-Undang Hukum Perdata
//!
//! The Indonesian Civil Code (KUHPerdata, Staatsblad 1847 No. 23) is based on
//! the Dutch Burgerlijk Wetboek of 1838, adapted for Indonesian conditions.
//!
//! ## Structure (Books)
//!
//! 1. **Book I - Persons (Orang)**: Legal personality, capacity, domicile, marriage, family
//! 2. **Book II - Things (Benda)**: Property, ownership, servitudes
//! 3. **Book III - Obligations (Perikatan)**: Contracts, torts, unjust enrichment
//! 4. **Book IV - Evidence and Limitation (Pembuktian dan Daluwarsa)**
//!
//! ## Key Contract Principles (Pasal 1320)
//!
//! Four requirements for valid contract:
//! 1. Agreement (kesepakatan/sepakat)
//! 2. Capacity (kecakapan/cakap)
//! 3. Specific object (suatu hal tertentu)
//! 4. Lawful cause (sebab yang halal)

mod error;
mod types;
mod validator;

pub use error::{CivilCodeError, CivilCodeResult};
pub use types::{
    Contract, ContractFormation, ContractTermination, ContractValidity, ContractValidityStatus,
    LegalCapacity, ObligationType, PropertyRight, PropertyType,
};
pub use validator::{
    ContractCompliance, get_contract_checklist, validate_contract_compliance,
    validate_contract_formation, validate_contract_validity, validate_legal_capacity,
};
