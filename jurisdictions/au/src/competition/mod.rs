//! Australian Competition Law (Part IV, Competition and Consumer Act 2010)
//!
//! This module implements Australia's competition law framework, covering:
//! - **Part IV Division 1** - Cartel conduct (ss.45AD-45AS)
//! - **Section 46** - Misuse of market power
//! - **Section 47** - Exclusive dealing
//! - **Section 50** - Mergers and acquisitions
//! - **Part IIIA** - Access regime for essential facilities
//!
//! ## Regulatory Authority
//!
//! The Australian Competition and Consumer Commission (ACCC) administers the CCA,
//! with the Australian Competition Tribunal (ACT) handling appeals and authorisation
//! applications.
//!
//! ## Key Principles
//!
//! ### Cartel Conduct (ss.45AD-45AS)
//!
//! Cartel conduct is a per se criminal offence in Australia. Four types:
//! 1. **Price fixing** - Agreements on price, discount, allowance, rebate, or credit
//! 2. **Output restriction** - Limiting production, supply, or acquisition
//! 3. **Market allocation** - Dividing customers, suppliers, or geographic areas
//! 4. **Bid rigging** - Coordinating or not submitting competitive bids
//!
//! **Penalties:**
//! - Criminal: Up to 10 years imprisonment for individuals
//! - Civil: Greater of $10M, 3x benefit, or 10% of annual turnover
//!
//! **Defences:**
//! - Joint venture exception (s.45AO-45AQ)
//! - Collective bargaining authorisation
//! - Notification for exclusive dealing
//!
//! ### Misuse of Market Power (s.46)
//!
//! Post-2017 Harper Review reforms introduced effects-based test:
//! - Prohibition on conduct by corporation with substantial market power
//! - That has purpose, effect, or likely effect of substantially lessening competition
//!
//! **No longer requires:**
//! - Taking advantage of market power
//! - Specific anti-competitive purpose (effect sufficient)
//!
//! ### Mergers (s.50)
//!
//! Prohibits acquisitions that would substantially lessen competition (SLC test).
//!
//! **ACCC considers:**
//! - Market concentration
//! - Barriers to entry
//! - Import competition
//! - Countervailing power
//! - Likelihood of collusion
//! - Vertical integration effects
//!
//! **Notification regime:**
//! - Informal clearance (most common)
//! - Formal merger authorisation
//! - Merger undertakings (divestiture conditions)
//!
//! ## Leading Cases
//!
//! - ACCC v Boral Besser Masonry Ltd (2003) - Predatory pricing
//! - ACCC v Flight Centre (2016) - Price fixing/agency
//! - ACCC v TPG Telecom (2020) - s.46 effects-based test
//! - ACCC v Qantas (2014) - Attempted predation
//! - ACCC v Pacific National (2020) - Market power in rail
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_au::competition::{CartelAnalyzer, CartelConduct, MarketPowerAnalyzer};
//!
//! // Analyze potential cartel conduct
//! let conduct = CartelConduct::PriceFixing {
//!     parties: vec!["Company A".into(), "Company B".into()],
//!     product_market: "Cement supply".into(),
//!     agreement_type: AgreementType::PriceFixing,
//! };
//!
//! let result = CartelAnalyzer::analyze(&conduct);
//! assert!(result.is_cartel_conduct);
//! assert!(result.criminal_liability);
//! ```

pub mod cartel;
pub mod error;
pub mod exclusive_dealing;
pub mod market_power;
pub mod mergers;
pub mod types;
pub mod validator;

// Re-exports
pub use cartel::{CartelAnalysisResult, CartelAnalyzer, CartelDefence, CartelType};
pub use error::{CompetitionError, Result};
pub use exclusive_dealing::{ExclusiveDealingAnalyzer, ExclusiveDealingType};
pub use market_power::{MarketPowerAnalyzer, SubstantialMarketPower};
pub use mergers::{MergerAnalyzer, MergerResult, MergerType};
pub use types::{
    AcquirerType, AntiCompetitiveEffect, Competitor, GeographicMarket, MarketDefinition,
    MarketPlayer, MarketShare, RelevantMarket, StateTerritory as CompetitionState, Undertaking,
};
pub use validator::{CompetitionValidator, ValidationResult};
