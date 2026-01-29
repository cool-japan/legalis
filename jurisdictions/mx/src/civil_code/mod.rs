//! Federal Civil Code (Código Civil Federal)
//!
//! Mexican Civil Code covering:
//! - Book 1: Persons (Personas)
//! - Book 2: Property (Bienes)
//! - Book 3: Succession (Sucesiones)
//! - Book 4: Obligations (Obligaciones) and Contracts (Contratos)

pub mod contracts;
pub mod obligations;
pub mod persons;
pub mod property;

// Re-export main types
pub use contracts::{
    ConsentDefect, Contract, ContractError, ContractType, Party, PartyRole, Term,
    ValidityRequirements,
};
pub use obligations::{
    Breach, BreachType, DoObligation, GiveObligation, NotDoObligation, ObligationError,
    ObligationSource, ObligationType, PerformanceStatus,
};
pub use persons::{
    EntityType, JuridicalPerson, LegalCapacity, NaturalPerson, PersonError, PersonType,
};
pub use property::{
    ImmovableProperty, MovableProperty, OwnershipType, PropertyError, PropertyRight, PropertyType,
    RightType,
};
