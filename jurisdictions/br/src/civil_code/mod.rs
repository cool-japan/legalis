//! # Civil Code - Código Civil (Lei nº 10.406/2002)
//!
//! Brazil's comprehensive civil code, enacted in 2002, replacing the 1916 code.
//!
//! ## Overview
//!
//! The Brazilian Civil Code is divided into two major parts:
//!
//! | Part | Articles | Coverage |
//! |------|----------|----------|
//! | General Part | 1-232 | Persons, property, legal acts |
//! | Special Part | 233-2027 | Obligations, contracts, property rights, family, succession |
//!
//! ## Structure
//!
//! ### Part I - General Part (Parte Geral)
//!
//! | Book | Articles | Content |
//! |------|----------|---------|
//! | Persons | 1-78 | Natural/legal persons, capacity, domicile |
//! | Property | 79-103 | Movable/immovable, public/private |
//! | Legal Acts | 104-232 | Requirements, defects, invalidity |
//!
//! ### Part II - Special Part (Parte Especial)
//!
//! | Book | Articles | Content |
//! |------|----------|---------|
//! | Obligations | 233-420 | Sources, types, extinction |
//! | Contracts | 421-853 | General provisions, specific contracts |
//! | Property Rights | 1196-1510 | Possession, ownership, limited rights |
//! | Family Law | 1511-1783 | Marriage, relations, protection |
//! | Succession | 1784-2027 | Inheritance, wills, inventory |
//!
//! ## Key Principles
//!
//! | Principle | Article | Description |
//! |-----------|---------|-------------|
//! | Sociability | Throughout | Prioritizes collective over individual interests |
//! | Good Faith | Art. 422 | Objective good faith in contracts |
//! | Social Function | Art. 421 | Contracts must serve social function |
//! | Operability | Throughout | Practical, effective norms |
//!
//! ## Major Innovations (2002 Code)
//!
//! - **Unification of obligations**: Civil and commercial obligations unified
//! - **Good faith principle**: Explicit requirement in contracts (Art. 422)
//! - **Social function**: Property and contracts must serve society
//! - **De facto union**: Recognition of stable unions (Art. 1723)
//! - **Solidarity liability**: Enhanced consumer/worker protection
//!
//! ## Usage Example
//!
//! ```rust
//! use legalis_br::civil_code::*;
//!
//! // Natural person capacity
//! let person = NaturalPerson::new("João Silva", 25);
//! assert!(person.is_fully_capable());
//!
//! // Contract validity requirements
//! let contract = Contract::new(ContractType::Sale);
//! assert!(contract.requires_good_faith());
//! ```
//!
//! ## References
//!
//! - [Lei nº 10.406/2002](http://www.planalto.gov.br/ccivil_03/leis/2002/l10406compilada.htm)
//! - [Superior Court of Justice (STJ)](https://www.stj.jus.br/)

pub mod contracts;
pub mod family;
pub mod general_part;
pub mod obligations;
pub mod property;
pub mod succession;

pub use contracts::*;
pub use family::*;
pub use general_part::*;
pub use obligations::*;
pub use property::*;
pub use succession::*;
