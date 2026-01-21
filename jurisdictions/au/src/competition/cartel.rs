//! Cartel Conduct Analysis (CCA Part IV, Division 1, ss.45AD-45AS)
//!
//! This module implements comprehensive analysis of cartel conduct under
//! Australian competition law. Cartel conduct is both a criminal offence
//! and a civil prohibition.
//!
//! ## Four Types of Cartel Conduct
//!
//! 1. **Price Fixing** (s.45AD(1)) - Fixing, controlling, or maintaining prices
//! 2. **Output Restriction** (s.45AD(2)) - Preventing, restricting, or limiting
//!    production, capacity, or supply
//! 3. **Market Allocation** (s.45AD(3)) - Allocating customers, suppliers, or
//!    geographic areas
//! 4. **Bid Rigging** (s.45AD(4)) - Rigging bids in competitive processes
//!
//! ## Criminal vs Civil
//!
//! | Element | Criminal (s.45AF) | Civil (s.45AJ) |
//! |---------|-------------------|----------------|
//! | Fault | Knowledge/belief | Strict liability |
//! | Penalty (individual) | 10 yrs / $444K | $500,000 |
//! | Penalty (corporate) | See below | See below |
//!
//! Corporate penalty: Greater of:
//! - $10 million
//! - 3 times the benefit obtained
//! - 10% of annual turnover
//!
//! ## Key Defences and Exceptions
//!
//! - **Joint venture exception** (ss.45AO-45AQ): Conduct in furtherance of
//!   qualifying joint venture
//! - **Authorisation**: ACCC may authorise if public benefit outweighs detriment
//! - **Notification** (exclusive dealing only): Can notify conduct to ACCC
//!
//! ## Leading Cases
//!
//! - ACCC v Visy (2007) - $36M penalty for cardboard box cartel
//! - ACCC v Flight Centre (2016) - Travel agency price fixing
//! - ACCC v NSK (2017) - Automotive parts cartel

use serde::{Deserialize, Serialize};

pub use super::error::CartelType;
use super::types::{RelevantMarket, Undertaking};

/// Cartel conduct instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartelConduct {
    /// Type of cartel conduct
    pub cartel_type: CartelType,
    /// Parties to the cartel
    pub parties: Vec<Undertaking>,
    /// Relevant market
    pub market: Option<RelevantMarket>,
    /// Description of the conduct
    pub description: String,
    /// Start date of conduct (if known)
    pub start_date: Option<String>,
    /// End date of conduct (if known)
    pub end_date: Option<String>,
    /// Evidence of agreement/arrangement/understanding
    pub evidence: CartelEvidence,
    /// Estimated gain/loss from conduct (AUD)
    pub estimated_gain_aud: Option<f64>,
}

impl CartelConduct {
    /// Create new price fixing conduct
    pub fn price_fixing(parties: Vec<Undertaking>, description: impl Into<String>) -> Self {
        Self {
            cartel_type: CartelType::PriceFixing,
            parties,
            market: None,
            description: description.into(),
            start_date: None,
            end_date: None,
            evidence: CartelEvidence::default(),
            estimated_gain_aud: None,
        }
    }

    /// Create new bid rigging conduct
    pub fn bid_rigging(parties: Vec<Undertaking>, description: impl Into<String>) -> Self {
        Self {
            cartel_type: CartelType::BidRigging,
            parties,
            market: None,
            description: description.into(),
            start_date: None,
            end_date: None,
            evidence: CartelEvidence::default(),
            estimated_gain_aud: None,
        }
    }

    /// Create new market allocation conduct
    pub fn market_allocation(parties: Vec<Undertaking>, description: impl Into<String>) -> Self {
        Self {
            cartel_type: CartelType::MarketAllocation,
            parties,
            market: None,
            description: description.into(),
            start_date: None,
            end_date: None,
            evidence: CartelEvidence::default(),
            estimated_gain_aud: None,
        }
    }

    /// Create new output restriction conduct
    pub fn output_restriction(parties: Vec<Undertaking>, description: impl Into<String>) -> Self {
        Self {
            cartel_type: CartelType::OutputRestriction,
            parties,
            market: None,
            description: description.into(),
            start_date: None,
            end_date: None,
            evidence: CartelEvidence::default(),
            estimated_gain_aud: None,
        }
    }

    /// Set the relevant market
    pub fn with_market(mut self, market: RelevantMarket) -> Self {
        self.market = Some(market);
        self
    }

    /// Set evidence
    pub fn with_evidence(mut self, evidence: CartelEvidence) -> Self {
        self.evidence = evidence;
        self
    }

    /// Set estimated gain
    pub fn with_estimated_gain(mut self, gain_aud: f64) -> Self {
        self.estimated_gain_aud = Some(gain_aud);
        self
    }
}

/// Evidence of cartel conduct
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CartelEvidence {
    /// Direct evidence (documents, communications)
    pub direct_evidence: Vec<DirectEvidence>,
    /// Circumstantial evidence
    pub circumstantial_evidence: Vec<CircumstantialEvidence>,
    /// Witness statements
    pub witness_statements: Vec<WitnessStatement>,
    /// Leniency applicant evidence
    pub leniency_evidence: Option<LeniencyEvidence>,
}

impl CartelEvidence {
    /// Check if evidence is sufficient for civil case
    pub fn sufficient_for_civil(&self) -> bool {
        // Civil: balance of probabilities
        !self.direct_evidence.is_empty()
            || (self.circumstantial_evidence.len() >= 3 && !self.witness_statements.is_empty())
            || self.leniency_evidence.is_some()
    }

    /// Check if evidence is sufficient for criminal case
    pub fn sufficient_for_criminal(&self) -> bool {
        // Criminal: beyond reasonable doubt, plus mens rea
        let has_strong_evidence = self.direct_evidence.iter().any(|e| e.is_strong())
            || self
                .leniency_evidence
                .as_ref()
                .map(|l| l.full_cooperation)
                .unwrap_or(false);

        // Leniency with full cooperation implies confession of knowledge
        let has_mens_rea_evidence = self.direct_evidence.iter().any(|e| e.shows_knowledge)
            || self.witness_statements.iter().any(|w| w.shows_knowledge)
            || self
                .leniency_evidence
                .as_ref()
                .map(|l| l.full_cooperation)
                .unwrap_or(false);

        has_strong_evidence && has_mens_rea_evidence
    }
}

/// Direct evidence of cartel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectEvidence {
    /// Type of evidence
    pub evidence_type: DirectEvidenceType,
    /// Description
    pub description: String,
    /// Date of evidence
    pub date: Option<String>,
    /// Source
    pub source: String,
    /// Shows knowledge/belief (for criminal)
    pub shows_knowledge: bool,
}

impl DirectEvidence {
    /// Check if evidence is strong
    pub fn is_strong(&self) -> bool {
        matches!(
            self.evidence_type,
            DirectEvidenceType::WrittenAgreement
                | DirectEvidenceType::Email
                | DirectEvidenceType::RecordedConversation
        )
    }
}

/// Types of direct evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectEvidenceType {
    /// Written agreement
    WrittenAgreement,
    /// Email communications
    Email,
    /// Text messages
    TextMessage,
    /// Recorded conversation
    RecordedConversation,
    /// Meeting notes/minutes
    MeetingNotes,
    /// Handwritten notes
    HandwrittenNotes,
    /// Price lists/schedules
    PriceLists,
}

/// Circumstantial evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircumstantialEvidence {
    /// Type of circumstantial evidence
    pub evidence_type: CircumstantialEvidenceType,
    /// Description
    pub description: String,
    /// Strength of inference
    pub inference_strength: InferenceStrength,
}

/// Types of circumstantial evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircumstantialEvidenceType {
    /// Parallel pricing
    ParallelPricing,
    /// Simultaneous price increases
    SimultaneousPriceIncreases,
    /// Information exchange
    InformationExchange,
    /// Trade association meetings
    TradeAssociationMeetings,
    /// Market division pattern
    MarketDivisionPattern,
    /// Unusual bidding patterns
    UnusualBiddingPatterns,
    /// Consistent losing bidder
    ConsistentLosingBidder,
    /// Price leadership
    PriceLeadership,
}

/// Strength of inference from circumstantial evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InferenceStrength {
    /// Strong inference
    Strong,
    /// Moderate inference
    Moderate,
    /// Weak inference
    Weak,
}

/// Witness statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WitnessStatement {
    /// Witness role
    pub witness_role: WitnessRole,
    /// Summary of statement
    pub summary: String,
    /// Whether statement shows knowledge
    pub shows_knowledge: bool,
    /// Whether witness is willing to testify
    pub willing_to_testify: bool,
}

/// Role of witness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WitnessRole {
    /// Cartel participant
    Participant,
    /// Employee of cartel member
    Employee,
    /// Customer affected by cartel
    Customer,
    /// Competitor (non-cartel member)
    Competitor,
    /// Industry expert
    Expert,
}

/// Evidence from leniency applicant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeniencyEvidence {
    /// Company name
    pub applicant: String,
    /// Type of immunity/leniency
    pub leniency_type: LeniencyType,
    /// Full cooperation
    pub full_cooperation: bool,
    /// First in (for immunity)
    pub first_in: bool,
    /// Description of evidence provided
    pub evidence_description: String,
}

/// Type of ACCC leniency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeniencyType {
    /// Full immunity (first in)
    FullImmunity,
    /// Partial immunity
    PartialImmunity,
    /// Civil cooperation policy
    CivilCooperation,
}

/// Cartel defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CartelDefence {
    /// Joint venture exception (ss.45AO-45AQ)
    JointVentureException(JointVentureDefence),
    /// Authorisation
    Authorisation {
        /// ACCC determination number
        determination_number: String,
        /// Public benefits
        public_benefits: Vec<String>,
    },
    /// Conduct not within cartel provision
    NotCartelConduct {
        /// Reason
        reason: String,
    },
    /// Lack of knowledge (criminal only)
    LackOfKnowledge {
        /// Explanation
        explanation: String,
    },
    /// Vertical arrangement (not horizontal)
    VerticalArrangement {
        /// Description
        description: String,
    },
}

/// Joint venture defence details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointVentureDefence {
    /// Joint venture description
    pub joint_venture_description: String,
    /// Qualifying joint venture (s.45AO)
    pub qualifying_joint_venture: bool,
    /// Conduct in furtherance of JV
    pub in_furtherance_of_jv: bool,
    /// Participants in JV
    pub participants: Vec<String>,
    /// Activities of JV
    pub jv_activities: Vec<String>,
    /// Provision notified (s.45AQ)
    pub provision_notified: bool,
}

/// Cartel analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartelAnalysisResult {
    /// Whether conduct constitutes cartel conduct
    pub is_cartel_conduct: bool,
    /// Type of cartel (if applicable)
    pub cartel_type: Option<CartelType>,
    /// Criminal liability likely
    pub criminal_liability: bool,
    /// Civil liability likely
    pub civil_liability: bool,
    /// Applicable defences
    pub applicable_defences: Vec<CartelDefence>,
    /// Recommended penalty range (AUD)
    pub penalty_range: Option<PenaltyRange>,
    /// Reasoning
    pub reasoning: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Penalty range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenaltyRange {
    /// Minimum penalty (AUD)
    pub min_aud: f64,
    /// Maximum penalty (AUD)
    pub max_aud: f64,
    /// Basis for calculation
    pub basis: PenaltyBasis,
}

/// Basis for penalty calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyBasis {
    /// Fixed maximum
    FixedMaximum,
    /// Three times benefit
    ThreeTimesBenefit,
    /// Ten percent of turnover
    TenPercentTurnover,
}

/// Cartel conduct analyzer
pub struct CartelAnalyzer;

impl CartelAnalyzer {
    /// Analyze potential cartel conduct
    pub fn analyze(conduct: &CartelConduct) -> CartelAnalysisResult {
        let is_cartel = Self::is_cartel_conduct(conduct);
        let criminal = is_cartel && conduct.evidence.sufficient_for_criminal();
        let civil = is_cartel && conduct.evidence.sufficient_for_civil();
        let defences = Self::identify_defences(conduct);
        let penalty = if is_cartel {
            Self::estimate_penalty(conduct)
        } else {
            None
        };
        let reasoning = Self::build_reasoning(conduct, is_cartel, criminal, &defences);
        let recommendations = Self::generate_recommendations(conduct, is_cartel, &defences);

        CartelAnalysisResult {
            is_cartel_conduct: is_cartel,
            cartel_type: if is_cartel {
                Some(conduct.cartel_type)
            } else {
                None
            },
            criminal_liability: criminal,
            civil_liability: civil,
            applicable_defences: defences,
            penalty_range: penalty,
            reasoning,
            recommendations,
        }
    }

    /// Check if conduct constitutes cartel conduct
    fn is_cartel_conduct(conduct: &CartelConduct) -> bool {
        // Must have at least 2 parties
        if conduct.parties.len() < 2 {
            return false;
        }

        // Parties must be competitors (or potential competitors)
        // Simplified check - in practice would need market analysis
        true
    }

    /// Identify applicable defences
    fn identify_defences(conduct: &CartelConduct) -> Vec<CartelDefence> {
        let mut defences = Vec::new();

        // Check if conduct might be covered by joint venture exception
        // This is a simplified check
        if conduct.parties.len() == 2
            && conduct.description.to_lowercase().contains("joint venture")
        {
            defences.push(CartelDefence::JointVentureException(JointVentureDefence {
                joint_venture_description: "Potential joint venture".into(),
                qualifying_joint_venture: false, // Would need further analysis
                in_furtherance_of_jv: false,
                participants: conduct.parties.iter().map(|p| p.name.clone()).collect(),
                jv_activities: Vec::new(),
                provision_notified: false,
            }));
        }

        defences
    }

    /// Estimate penalty range
    fn estimate_penalty(conduct: &CartelConduct) -> Option<PenaltyRange> {
        // Base on estimated gain if available
        if let Some(gain) = conduct.estimated_gain_aud {
            let three_times = gain * 3.0;
            Some(PenaltyRange {
                min_aud: 10_000_000.0_f64.min(three_times * 0.5),
                max_aud: 10_000_000.0_f64.max(three_times),
                basis: PenaltyBasis::ThreeTimesBenefit,
            })
        } else {
            // Default range
            Some(PenaltyRange {
                min_aud: 1_000_000.0,
                max_aud: 10_000_000.0,
                basis: PenaltyBasis::FixedMaximum,
            })
        }
    }

    /// Build reasoning
    fn build_reasoning(
        conduct: &CartelConduct,
        is_cartel: bool,
        criminal: bool,
        defences: &[CartelDefence],
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Cartel analysis under CCA s.{}",
            conduct.cartel_type.section()
        ));

        if is_cartel {
            parts.push(format!(
                "Conduct appears to constitute {} cartel conduct",
                match conduct.cartel_type {
                    CartelType::PriceFixing => "price fixing",
                    CartelType::OutputRestriction => "output restriction",
                    CartelType::MarketAllocation => "market allocation",
                    CartelType::BidRigging => "bid rigging",
                }
            ));

            parts.push(format!(
                "{} parties involved: {}",
                conduct.parties.len(),
                conduct
                    .parties
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));

            if criminal {
                parts.push(
                    "Criminal prosecution may be pursued - evidence supports mens rea".into(),
                );
            } else {
                parts.push(
                    "Civil proceedings more likely - insufficient evidence for criminal standard"
                        .into(),
                );
            }
        } else {
            parts.push("Conduct does not appear to constitute cartel conduct".into());
        }

        if !defences.is_empty() {
            parts.push(format!(
                "{} potential defence(s) identified",
                defences.len()
            ));
        }

        parts.join(". ")
    }

    /// Generate recommendations
    fn generate_recommendations(
        _conduct: &CartelConduct,
        is_cartel: bool,
        defences: &[CartelDefence],
    ) -> Vec<String> {
        let mut recs = Vec::new();

        if is_cartel {
            recs.push("Seek immediate legal advice".into());
            recs.push("Consider ACCC immunity/leniency policy if first to disclose".into());
            recs.push("Preserve all relevant documents".into());
            recs.push("Conduct internal investigation".into());

            if !defences.is_empty() {
                recs.push("Investigate potential defences".into());
            }
        } else {
            recs.push("Document business justification for conduct".into());
            recs.push("Consider competition law compliance review".into());
        }

        recs
    }

    /// Calculate combined market share
    pub fn combined_market_share(parties: &[Undertaking]) -> Option<f64> {
        let shares: Vec<f64> = parties
            .iter()
            .filter_map(|p| p.market_share.as_ref())
            .filter_map(|s| s.primary_share())
            .collect();

        if shares.is_empty() {
            None
        } else {
            Some(shares.iter().sum())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::competition::types::MarketShare;

    #[test]
    fn test_price_fixing_creation() {
        let parties = vec![Undertaking::new("Company A"), Undertaking::new("Company B")];

        let conduct = CartelConduct::price_fixing(parties, "Agreement to fix cement prices");

        assert_eq!(conduct.cartel_type, CartelType::PriceFixing);
        assert_eq!(conduct.parties.len(), 2);
    }

    #[test]
    fn test_cartel_analysis_basic() {
        let parties = vec![Undertaking::new("Acme Corp"), Undertaking::new("Beta Inc")];

        let mut conduct = CartelConduct::price_fixing(parties, "Agreed to maintain minimum prices");

        conduct.evidence.direct_evidence.push(DirectEvidence {
            evidence_type: DirectEvidenceType::Email,
            description: "Email chain discussing prices".into(),
            date: Some("2024-01-15".into()),
            source: "Company A servers".into(),
            shows_knowledge: true,
        });

        let result = CartelAnalyzer::analyze(&conduct);

        assert!(result.is_cartel_conduct);
        assert!(result.civil_liability);
    }

    #[test]
    fn test_bid_rigging() {
        let parties = vec![
            Undertaking::new("Builder A"),
            Undertaking::new("Builder B"),
            Undertaking::new("Builder C"),
        ];

        let conduct = CartelConduct::bid_rigging(parties, "Coordinated bids for government tender");

        assert_eq!(conduct.cartel_type, CartelType::BidRigging);
    }

    #[test]
    fn test_evidence_sufficiency() {
        let mut evidence = CartelEvidence::default();

        // Initially insufficient
        assert!(!evidence.sufficient_for_civil());
        assert!(!evidence.sufficient_for_criminal());

        // Add leniency evidence
        evidence.leniency_evidence = Some(LeniencyEvidence {
            applicant: "First Mover Ltd".into(),
            leniency_type: LeniencyType::FullImmunity,
            full_cooperation: true,
            first_in: true,
            evidence_description: "Full disclosure of cartel activities".into(),
        });

        assert!(evidence.sufficient_for_civil());
        assert!(evidence.sufficient_for_criminal());
    }

    #[test]
    fn test_cartel_type_descriptions() {
        assert!(CartelType::PriceFixing.description().contains("price"));
        assert!(CartelType::BidRigging.description().contains("bid"));
        assert!(
            CartelType::MarketAllocation
                .description()
                .contains("allocate")
        );
    }

    #[test]
    fn test_combined_market_share() {
        let parties = vec![
            Undertaking::new("A").with_market_share(MarketShare::from_revenue(0.30)),
            Undertaking::new("B").with_market_share(MarketShare::from_revenue(0.25)),
        ];

        let combined = CartelAnalyzer::combined_market_share(&parties);
        assert!((combined.unwrap_or(0.0) - 0.55).abs() < 0.01);
    }
}
