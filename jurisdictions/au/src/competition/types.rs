//! Core types for Australian Competition Law
//!
//! This module defines the fundamental data structures for analysing competition
//! law matters under Part IV of the Competition and Consumer Act 2010.
//!
//! ## Key Concepts
//!
//! ### Market Definition
//!
//! Market definition is fundamental to competition analysis. Australian courts
//! consider both product and geographic dimensions, applying the SSNIP test
//! (Small but Significant Non-transitory Increase in Price).
//!
//! ### Substantial Market Power
//!
//! Market power is the ability to profitably raise prices or reduce output
//! beyond competitive levels. Indicators include:
//! - High market share (typically >40%)
//! - Barriers to entry/expansion
//! - Vertical integration
//! - Access to superior technology
//!
//! ## Cases
//!
//! - Queensland Wire Industries v BHP (1989) - Market power definition
//! - ACCC v Metcash (2011) - Market definition methodology
//! - ACCC v Flight Centre (2016) - Relevant market for agency services

use serde::{Deserialize, Serialize};

/// Australian States and Territories for geographic market definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateTerritory {
    /// New South Wales
    NewSouthWales,
    /// Victoria
    Victoria,
    /// Queensland
    Queensland,
    /// Western Australia
    WesternAustralia,
    /// South Australia
    SouthAustralia,
    /// Tasmania
    Tasmania,
    /// Northern Territory
    NorthernTerritory,
    /// Australian Capital Territory
    AustralianCapitalTerritory,
}

impl StateTerritory {
    /// Returns all states and territories
    pub fn all() -> Vec<StateTerritory> {
        vec![
            StateTerritory::NewSouthWales,
            StateTerritory::Victoria,
            StateTerritory::Queensland,
            StateTerritory::WesternAustralia,
            StateTerritory::SouthAustralia,
            StateTerritory::Tasmania,
            StateTerritory::NorthernTerritory,
            StateTerritory::AustralianCapitalTerritory,
        ]
    }

    /// Returns abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            StateTerritory::NewSouthWales => "NSW",
            StateTerritory::Victoria => "VIC",
            StateTerritory::Queensland => "QLD",
            StateTerritory::WesternAustralia => "WA",
            StateTerritory::SouthAustralia => "SA",
            StateTerritory::Tasmania => "TAS",
            StateTerritory::NorthernTerritory => "NT",
            StateTerritory::AustralianCapitalTerritory => "ACT",
        }
    }

    /// Population (2024 estimates for economic significance)
    pub fn population(&self) -> u64 {
        match self {
            StateTerritory::NewSouthWales => 8_300_000,
            StateTerritory::Victoria => 6_800_000,
            StateTerritory::Queensland => 5_400_000,
            StateTerritory::WesternAustralia => 2_850_000,
            StateTerritory::SouthAustralia => 1_850_000,
            StateTerritory::Tasmania => 575_000,
            StateTerritory::NorthernTerritory => 255_000,
            StateTerritory::AustralianCapitalTerritory => 470_000,
        }
    }
}

/// Geographic market scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeographicMarket {
    /// National market (Australia-wide)
    National,
    /// State or territory market
    StateTerritory(StateTerritory),
    /// Multiple states/territories
    Regional(Vec<StateTerritory>),
    /// Metropolitan area (e.g., Greater Sydney)
    MetropolitanArea {
        /// State where metropolitan area is located
        state: StateTerritory,
        /// Name of the metropolitan area
        name: String,
    },
    /// Local market (suburb/town level)
    Local {
        /// State where local market is located
        state: StateTerritory,
        /// Locality name
        locality: String,
        /// Radius in kilometres
        radius_km: f64,
    },
    /// International (Australia + specific countries)
    International(Vec<String>),
}

impl GeographicMarket {
    /// Create a national market
    pub fn national() -> Self {
        GeographicMarket::National
    }

    /// Create a state market
    pub fn state(state: StateTerritory) -> Self {
        GeographicMarket::StateTerritory(state)
    }

    /// Check if market is national or broader
    pub fn is_national_or_broader(&self) -> bool {
        matches!(
            self,
            GeographicMarket::National | GeographicMarket::International(_)
        )
    }
}

/// Product or service market definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductMarket {
    /// Primary product/service description
    pub description: String,
    /// Substitutable products (demand-side substitution)
    pub demand_substitutes: Vec<String>,
    /// Supply-side substitutes (supply-side substitution)
    pub supply_substitutes: Vec<String>,
    /// Functional characteristics
    pub functional_characteristics: Vec<String>,
    /// End-use applications
    pub end_uses: Vec<String>,
}

impl ProductMarket {
    /// Create a new product market
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            demand_substitutes: Vec::new(),
            supply_substitutes: Vec::new(),
            functional_characteristics: Vec::new(),
            end_uses: Vec::new(),
        }
    }

    /// Add a demand substitute
    pub fn with_demand_substitute(mut self, substitute: impl Into<String>) -> Self {
        self.demand_substitutes.push(substitute.into());
        self
    }

    /// Add a supply substitute
    pub fn with_supply_substitute(mut self, substitute: impl Into<String>) -> Self {
        self.supply_substitutes.push(substitute.into());
        self
    }
}

/// Complete relevant market definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelevantMarket {
    /// Product market
    pub product: ProductMarket,
    /// Geographic market
    pub geographic: GeographicMarket,
    /// Temporal dimension (if relevant)
    pub temporal: Option<TemporalDimension>,
    /// Total market size (AUD)
    pub market_size_aud: Option<f64>,
    /// Market growth rate (annual percentage)
    pub annual_growth_rate: Option<f64>,
}

impl RelevantMarket {
    /// Create a new relevant market
    pub fn new(product: ProductMarket, geographic: GeographicMarket) -> Self {
        Self {
            product,
            geographic,
            temporal: None,
            market_size_aud: None,
            annual_growth_rate: None,
        }
    }

    /// Set market size
    pub fn with_market_size(mut self, size_aud: f64) -> Self {
        self.market_size_aud = Some(size_aud);
        self
    }

    /// Set growth rate
    pub fn with_growth_rate(mut self, rate: f64) -> Self {
        self.annual_growth_rate = Some(rate);
        self
    }
}

/// Temporal market dimension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporalDimension {
    /// Peak season only
    PeakSeason { months: Vec<u8> },
    /// Off-peak only
    OffPeak,
    /// Contract period
    ContractPeriod { years: u32 },
    /// Spot market
    SpotMarket,
}

/// Market definition result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDefinition {
    /// The defined relevant market
    pub market: RelevantMarket,
    /// Methodology used
    pub methodology: MarketDefinitionMethodology,
    /// Key factors considered
    pub factors: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

/// Methodology for market definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketDefinitionMethodology {
    /// SSNIP test (hypothetical monopolist test)
    SsnipTest,
    /// Functional substitutability
    FunctionalSubstitutability,
    /// Cross-elasticity of demand
    CrossElasticity,
    /// Practical evidence (switching data)
    PracticalEvidence,
}

/// Market share data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketShare {
    /// Share by revenue (0.0 to 1.0)
    pub revenue_share: Option<f64>,
    /// Share by volume/units
    pub volume_share: Option<f64>,
    /// Share by capacity
    pub capacity_share: Option<f64>,
    /// Primary measure used
    pub primary_measure: ShareMeasure,
}

impl MarketShare {
    /// Create market share from revenue percentage
    pub fn from_revenue(share: f64) -> Self {
        Self {
            revenue_share: Some(share),
            volume_share: None,
            capacity_share: None,
            primary_measure: ShareMeasure::Revenue,
        }
    }

    /// Create market share from volume percentage
    pub fn from_volume(share: f64) -> Self {
        Self {
            revenue_share: None,
            volume_share: Some(share),
            capacity_share: None,
            primary_measure: ShareMeasure::Volume,
        }
    }

    /// Get primary share value
    pub fn primary_share(&self) -> Option<f64> {
        match self.primary_measure {
            ShareMeasure::Revenue => self.revenue_share,
            ShareMeasure::Volume => self.volume_share,
            ShareMeasure::Capacity => self.capacity_share,
        }
    }

    /// Check if share indicates substantial market power (>40%)
    pub fn indicates_substantial_power(&self) -> bool {
        self.primary_share().map(|s| s > 0.40).unwrap_or(false)
    }

    /// Check if share indicates very substantial power (>60%)
    pub fn indicates_very_substantial_power(&self) -> bool {
        self.primary_share().map(|s| s > 0.60).unwrap_or(false)
    }

    /// Check if share indicates dominance (>70%)
    pub fn indicates_dominance(&self) -> bool {
        self.primary_share().map(|s| s > 0.70).unwrap_or(false)
    }
}

/// Market share measurement method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareMeasure {
    /// Revenue/sales value
    Revenue,
    /// Volume/units
    Volume,
    /// Production capacity
    Capacity,
}

/// Undertaking (corporation or business entity)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Undertaking {
    /// Legal name
    pub name: String,
    /// ABN (Australian Business Number)
    pub abn: Option<String>,
    /// ACN (Australian Company Number)
    pub acn: Option<String>,
    /// Parent company (if subsidiary)
    pub parent: Option<String>,
    /// Market share in relevant market
    pub market_share: Option<MarketShare>,
    /// Related bodies corporate
    pub related_bodies: Vec<String>,
}

impl Undertaking {
    /// Create a new undertaking
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            abn: None,
            acn: None,
            parent: None,
            market_share: None,
            related_bodies: Vec::new(),
        }
    }

    /// Set ABN
    pub fn with_abn(mut self, abn: impl Into<String>) -> Self {
        self.abn = Some(abn.into());
        self
    }

    /// Set market share
    pub fn with_market_share(mut self, share: MarketShare) -> Self {
        self.market_share = Some(share);
        self
    }

    /// Check if undertaking has substantial market power
    pub fn has_substantial_power(&self) -> bool {
        self.market_share
            .as_ref()
            .map(|s| s.indicates_substantial_power())
            .unwrap_or(false)
    }
}

/// Market player (competitor, supplier, or customer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketPlayer {
    /// The undertaking
    pub undertaking: Undertaking,
    /// Role in the market
    pub role: MarketRole,
    /// Competitive constraint level
    pub competitive_constraint: ConstraintLevel,
}

/// Role of a market player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketRole {
    /// Direct competitor
    Competitor,
    /// Upstream supplier
    Supplier,
    /// Downstream customer
    Customer,
    /// Potential entrant
    PotentialEntrant,
    /// Fringe player
    FringePlayer,
}

/// Level of competitive constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintLevel {
    /// Strong constraint
    Strong,
    /// Moderate constraint
    Moderate,
    /// Weak constraint
    Weak,
    /// No meaningful constraint
    None,
}

/// Competitor in the relevant market
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Competitor {
    /// The undertaking
    pub undertaking: Undertaking,
    /// Market share
    pub market_share: MarketShare,
    /// Entry date (if recent entrant)
    pub entry_date: Option<String>,
    /// Competitive strength factors
    pub strengths: Vec<String>,
    /// Competitive weaknesses
    pub weaknesses: Vec<String>,
}

impl Competitor {
    /// Create a new competitor
    pub fn new(undertaking: Undertaking, market_share: MarketShare) -> Self {
        Self {
            undertaking,
            market_share,
            entry_date: None,
            strengths: Vec::new(),
            weaknesses: Vec::new(),
        }
    }
}

/// Type of acquirer for merger analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquirerType {
    /// Horizontal competitor
    Horizontal,
    /// Vertical supplier
    VerticalUpstream,
    /// Vertical customer
    VerticalDownstream,
    /// Conglomerate (unrelated market)
    Conglomerate,
    /// Private equity
    PrivateEquity,
    /// Foreign acquirer
    ForeignInvestor,
}

/// Anti-competitive effect under s.50
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AntiCompetitiveEffect {
    /// Unilateral effects (merged firm can profitably raise prices)
    UnilateralEffects {
        /// Price increase estimate (percentage)
        estimated_price_increase: Option<f64>,
        /// Lost competition description
        lost_competition: String,
    },
    /// Coordinated effects (facilitates tacit collusion)
    CoordinatedEffects {
        /// Factors facilitating coordination
        facilitating_factors: Vec<String>,
        /// Likelihood assessment
        likelihood: CoordinationLikelihood,
    },
    /// Vertical foreclosure
    VerticalForeclosure {
        /// Input foreclosure
        input_foreclosure: bool,
        /// Customer foreclosure
        customer_foreclosure: bool,
        /// Description
        description: String,
    },
    /// Conglomerate effects (portfolio effects, bundling)
    ConglomerateEffects {
        /// Bundling capability
        bundling_capability: bool,
        /// Description
        description: String,
    },
}

/// Likelihood of coordinated effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationLikelihood {
    /// High likelihood
    High,
    /// Medium likelihood
    Medium,
    /// Low likelihood
    Low,
    /// Unlikely
    Unlikely,
}

/// Barriers to entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarriersToEntry {
    /// Regulatory barriers
    pub regulatory: Vec<String>,
    /// Capital requirements (AUD)
    pub capital_required: Option<f64>,
    /// Economies of scale significance
    pub economies_of_scale: ScaleSignificance,
    /// Brand loyalty/switching costs
    pub switching_costs: SwitchingCostLevel,
    /// Access to distribution
    pub distribution_access: AccessDifficulty,
    /// Intellectual property barriers
    pub ip_barriers: Vec<String>,
    /// Natural barriers (geography, resources)
    pub natural_barriers: Vec<String>,
    /// Overall assessment
    pub overall_level: BarrierLevel,
}

impl Default for BarriersToEntry {
    fn default() -> Self {
        Self {
            regulatory: Vec::new(),
            capital_required: None,
            economies_of_scale: ScaleSignificance::Low,
            switching_costs: SwitchingCostLevel::Low,
            distribution_access: AccessDifficulty::Moderate,
            ip_barriers: Vec::new(),
            natural_barriers: Vec::new(),
            overall_level: BarrierLevel::Low,
        }
    }
}

/// Significance of economies of scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleSignificance {
    /// Very significant
    VeryHigh,
    /// Significant
    High,
    /// Moderate
    Moderate,
    /// Low
    Low,
    /// Not significant
    NotSignificant,
}

/// Level of switching costs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwitchingCostLevel {
    /// Very high switching costs
    VeryHigh,
    /// High switching costs
    High,
    /// Moderate switching costs
    Moderate,
    /// Low switching costs
    Low,
    /// Negligible switching costs
    Negligible,
}

/// Difficulty of access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessDifficulty {
    /// Very difficult
    VeryDifficult,
    /// Difficult
    Difficult,
    /// Moderate
    Moderate,
    /// Easy
    Easy,
    /// Very easy
    VeryEasy,
}

/// Overall barrier level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarrierLevel {
    /// Very high barriers
    VeryHigh,
    /// High barriers
    High,
    /// Moderate barriers
    Moderate,
    /// Low barriers
    Low,
    /// Minimal barriers
    Minimal,
}

/// ACCC enforcement outcome
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnforcementOutcome {
    /// No action
    NoAction { reason: String },
    /// Warning letter
    Warning,
    /// Administrative resolution
    AdministrativeResolution { undertakings: Vec<String> },
    /// Infringement notice
    InfringementNotice { penalty_aud: f64 },
    /// Court proceedings
    CourtProceedings {
        /// Federal Court or High Court
        court: String,
        /// Matter number
        matter_number: Option<String>,
    },
    /// Penalty imposed
    Penalty {
        /// Penalty amount
        amount_aud: f64,
        /// Criminal or civil
        penalty_type: PenaltyType,
    },
}

/// Type of penalty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Civil pecuniary penalty
    CivilPecuniary,
    /// Criminal fine
    CriminalFine,
    /// Criminal imprisonment
    CriminalImprisonment { years: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_share_substantial_power() {
        let share = MarketShare::from_revenue(0.45);
        assert!(share.indicates_substantial_power());
        assert!(!share.indicates_dominance());
    }

    #[test]
    fn test_market_share_dominance() {
        let share = MarketShare::from_revenue(0.75);
        assert!(share.indicates_substantial_power());
        assert!(share.indicates_very_substantial_power());
        assert!(share.indicates_dominance());
    }

    #[test]
    fn test_undertaking_builder() {
        let undertaking = Undertaking::new("Acme Pty Ltd")
            .with_abn("12 345 678 901")
            .with_market_share(MarketShare::from_revenue(0.50));

        assert_eq!(undertaking.name, "Acme Pty Ltd");
        assert!(undertaking.abn.is_some());
        assert!(undertaking.has_substantial_power());
    }

    #[test]
    fn test_geographic_market() {
        let national = GeographicMarket::national();
        assert!(national.is_national_or_broader());

        let state = GeographicMarket::state(StateTerritory::NewSouthWales);
        assert!(!state.is_national_or_broader());
    }

    #[test]
    fn test_product_market_builder() {
        let market = ProductMarket::new("Premium smartphones")
            .with_demand_substitute("Mid-range smartphones")
            .with_supply_substitute("Tablet manufacturers");

        assert_eq!(market.description, "Premium smartphones");
        assert_eq!(market.demand_substitutes.len(), 1);
        assert_eq!(market.supply_substitutes.len(), 1);
    }

    #[test]
    fn test_state_abbreviations() {
        assert_eq!(StateTerritory::NewSouthWales.abbreviation(), "NSW");
        assert_eq!(StateTerritory::Victoria.abbreviation(), "VIC");
        assert_eq!(StateTerritory::Queensland.abbreviation(), "QLD");
    }
}
