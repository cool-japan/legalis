//! Contract law module (Module de droit des contrats)
//!
//! This module provides comprehensive support for French contract law under the Code civil,
//! as reformed by Ordonnance n° 2016-131 (February 10, 2016).
//!
//! ## Structure
//!
//! The module is organized following French contract law's structure:
//!
//! - **Formation**: Articles 1103-1122 (consent, capacity, content)
//! - **Validity**: Articles 1128-1171 (validity requirements, defects)
//! - **Performance**: Articles 1193-1216 (execution, modification)
//! - **Breach**: Articles 1217-1231-7 (remedies, damages)
//!
//! ## Key Articles Implemented
//!
//! - **Article 1103**: Binding force of contracts
//! - **Article 1128**: Three validity requirements (consent, capacity, content)
//! - **Article 1217**: Menu of breach remedies (5 remedies)
//! - **Article 1231**: Damages for breach (no fault required)
//!
//! ## Examples
//!
//! ### Validate a contract
//!
//! ```
//! use legalis_fr::contract::{Contract, ContractType, validate_contract_validity};
//!
//! let contract = Contract::new()
//!     .with_type(ContractType::Sale {
//!         price: 100_000,
//!         subject: "Machine industrielle".to_string()
//!     })
//!     .with_parties(vec!["Acheteur SARL".to_string(), "Vendeur SA".to_string()])
//!     .with_consent(true);
//!
//! match validate_contract_validity(&contract) {
//!     Ok(_) => println!("✅ Contract is valid"),
//!     Err(e) => println!("❌ Invalid: {}", e),
//! }
//! ```
//!
//! ### Calculate damages
//!
//! ```
//! use legalis_fr::contract::calculate_contract_damages;
//!
//! // Without penalty clause
//! let damages = calculate_contract_damages(
//!     100_000,  // Contract value
//!     80_000,   // Actual loss
//!     None,     // No penalty clause
//! );
//! assert_eq!(damages, 80_000);
//!
//! // With penalty clause (takes precedence)
//! let damages = calculate_contract_damages(
//!     100_000,
//!     80_000,
//!     Some(25_000), // Penalty clause
//! );
//! assert_eq!(damages, 25_000);
//! ```
//!
//! ## References
//!
//! - [Code civil - Légifrance](https://www.legifrance.gouv.fr/)
//! - Ordonnance n° 2016-131 du 10 février 2016 portant réforme du droit des contrats
//!
//! ## Comparison with Japanese Law
//!
//! | Aspect | France | Japan |
//! |--------|--------|-------|
//! | Validity requirements | 3 (Art. 1128) | Similar but not enumerated |
//! | Breach liability | No fault (Art. 1231) | No fault (民法415条) |
//! | Primary remedy | Specific performance | Specific performance (414条) |
//! | Good faith | Explicit (Art. 1104) | Implicit (信義則 - 1条2項) |

pub mod article1103;
pub mod article1128;
pub mod article1217;
pub mod article1231;
pub mod error;
pub mod types;
pub mod validator;

// Re-export key types
pub use error::{ContractError, ValidationResult};
pub use types::{
    BreachType, Contract, ContractType, DuressLevel, ObligationType, RemedyType, ValidityDefect,
};
pub use validator::{
    calculate_contract_damages, calculate_damages_with_force_majeure, validate_breach_claim,
    validate_contract_validity,
};

// Re-export article functions
pub use article1103::article1103;
pub use article1128::article1128;
pub use article1217::article1217;
pub use article1231::article1231;
