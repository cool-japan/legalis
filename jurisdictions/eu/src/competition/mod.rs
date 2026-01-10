//! Competition Law (Articles 101-102 TFEU)
//!
//! This module implements the EU's competition law framework, covering:
//! - **Article 101 TFEU**: Anti-competitive agreements and concerted practices
//! - **Article 102 TFEU**: Abuse of dominant position
//!
//! ## Key Concepts
//!
//! ### Article 101 TFEU
//!
//! Prohibits agreements between undertakings, decisions by associations, and concerted
//! practices that prevent, restrict, or distort competition within the internal market.
//!
//! **Three elements required:**
//! 1. Agreement/concerted practice between undertakings
//! 2. Appreciable effect on competition
//! 3. Effect on trade between Member States
//!
//! **Exemptions under Article 101(3):**
//! - Improves production/distribution OR promotes technical/economic progress
//! - Benefits consumers
//! - Restrictions indispensable
//! - No elimination of competition
//!
//! ### Article 102 TFEU
//!
//! Prohibits abuse of dominant position within the internal market or substantial part thereof.
//!
//! **Two elements required:**
//! 1. Dominant position (market share typically >40%)
//! 2. Abuse of that position
//!
//! **Types of abuse:**
//! - Exploitative (excessive pricing, limiting production)
//! - Exclusionary (predatory pricing, refusal to deal, tying)

pub mod article101;
pub mod article102;
pub mod error;
pub mod types;

// Re-exports
pub use article101::{
    Article101Agreement, Article101Exemption, Article101Validation, ConcertedPractice,
    MarketAllocation,
};
pub use article102::{Article102Conduct, Article102Validation, DominanceAssessment};
pub use error::CompetitionError;
pub use types::{
    AbuseType, ExclusionaryAbuse, ExploitativeAbuse, GeographicMarket, MemberState, RelevantMarket,
    Undertaking,
};
