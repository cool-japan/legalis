//! Thai Civil and Commercial Code (CCC) - ประมวลกฎหมายแพ่งและพาณิชย์ พ.ศ. 2535
//!
//! The CCC is Thailand's primary civil law code enacted in B.E. 2535 (1992 CE).
//! It is organized into six books covering all aspects of private law.
//!
//! ## Structure
//!
//! | Book | Thai Name | Sections | Coverage |
//! |------|-----------|----------|----------|
//! | I | บททั่วไป | 1-248 | General Provisions |
//! | II | หนี้ | 249-537 | Obligations |
//! | III | ลักษณะแห่งสัญญาต่างๆ | 538-925 | Specific Contracts |
//! | IV | พาณิชย์ | 926-1011 | Commercial Law |
//! | V | ครอบครัว | 1448-1598/46 | Family Law |
//! | VI | มรดก | 1599-1754 | Succession |

pub mod book1_general;
pub mod book2_obligations;
pub mod book3_property;
pub mod book4_commercial;
pub mod book5_family;
pub mod book6_succession;

// Re-export common types from each book
pub use book1_general::{
    DomicileType, JuristicActValidity, LegalCapacity, LimitationPeriod, Person, VoidableGround,
};

pub use book2_obligations::{
    Contract, ContractRemedy, ContractRequirement, DamagesType, InterpretationPrinciple,
    ObligationPerformance, ObligationSource, TortLiability,
};

pub use book3_property::{
    BuyerObligation, LeaseContract, LeaseType, SaleContract, SecurityType, SellerObligation,
    SpecificContract,
};

pub use book4_commercial::{
    AgentType, CommercialAct, CommercialBusiness, CommercialRegistrationType, CommercialSaleFeature,
};

pub use book5_family::{
    AdoptionRequirement, DivorceGround, Marriage, MarriageRequirement, PropertyRegime,
};

pub use book6_succession::{DisinheritanceGround, Estate, HeirClass, SpouseShare, WillType};
