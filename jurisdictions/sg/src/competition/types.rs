//! Competition Act 2004 - Type Definitions
//!
//! Type-safe models of Singapore's competition law framework under the
//! Competition Act 2004, administered by the Competition and Consumer
//! Commission of Singapore (CCCS).
//!
//! # The Three Prohibitions
//!
//! - **Section 34**: agreements between undertakings, decisions by associations
//!   of undertakings, and concerted practices which have as their object or
//!   effect the prevention, restriction or distortion of competition within
//!   Singapore.
//! - **Section 47**: conduct on the part of one or more undertakings which
//!   amounts to the abuse of a dominant position in any market in Singapore.
//! - **Section 54**: mergers that have resulted, or may be expected to result,
//!   in a substantial lessening of competition (SLC) within any market in
//!   Singapore.
//!
//! Monetary values are stored as **SGD cents** (`u64`), matching the convention
//! used by the `banking` module.

use serde::{Deserialize, Serialize};

// ============================================================================
// Statutory / guideline constants
// ============================================================================

/// CCCS guideline share above which a single undertaking is *likely* to be
/// dominant. This figure is **indicative, not conclusive** - dominance is
/// ultimately assessed on the relevant market as a whole (Competition Act
/// s. 47; CCCS Guidelines on the Section 47 Prohibition).
pub const DOMINANCE_INDICATIVE_SHARE_PERCENT: u8 = 60;

/// CCCS indicative merger threshold: the merged entity's market share at or
/// above this percentage may raise competition concerns (Competition Act s. 54;
/// CCCS Guidelines on the Substantive Assessment of Mergers).
pub const MERGER_SINGLE_SHARE_THRESHOLD_PERCENT: u8 = 40;

/// CCCS indicative merger threshold: combined share of the three largest firms
/// (CR3) at or above this percentage, taken together with
/// [`MERGER_COMBINED_MERGED_SHARE_THRESHOLD_PERCENT`].
pub const MERGER_CR3_THRESHOLD_PERCENT: u8 = 70;

/// CCCS indicative merger threshold: where the CR3 threshold is met, a merged
/// entity share at or above this percentage may raise concerns.
pub const MERGER_COMBINED_MERGED_SHARE_THRESHOLD_PERCENT: u8 = 20;

/// Maximum financial penalty as a percentage of Singapore turnover, per year of
/// infringement (Competition Act s. 69(4)).
pub const MAX_PENALTY_TURNOVER_PERCENT: u64 = 10;

/// Maximum number of years of infringement that may be counted when computing a
/// financial penalty (Competition Act s. 69(4)).
pub const MAX_PENALTY_YEARS: u32 = 3;

// ============================================================================
// Section 34 - anti-competitive agreements
// ============================================================================

/// Forms of anti-competitive conduct caught by the Section 34 prohibition.
///
/// Section 34(2) lists illustrative examples; bid rigging is treated as a
/// classic "by object" infringement of s. 34 in CCCS practice even though it
/// is not separately enumerated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiCompetitiveConduct {
    /// Directly or indirectly fixing purchase or selling prices or other
    /// trading conditions (s. 34(2)(a)).
    PriceFixing,
    /// Limiting or controlling production, markets, technical development or
    /// investment (s. 34(2)(b)).
    OutputLimitation,
    /// Sharing markets or sources of supply (s. 34(2)(c)).
    MarketSharing,
    /// Collusive tendering / bid rigging - a classic object infringement of
    /// s. 34.
    BidRigging,
    /// Applying dissimilar conditions to equivalent transactions, placing some
    /// parties at a competitive disadvantage (s. 34(2)(d)).
    Discrimination,
    /// Making the conclusion of contracts subject to acceptance of
    /// supplementary obligations with no connection to the subject of the
    /// contract - tying (s. 34(2)(e)).
    Tying,
    /// Exchange of commercially sensitive information between competitors,
    /// which may amount to a concerted practice under s. 34.
    InformationExchange,
}

impl AntiCompetitiveConduct {
    /// Returns the statutory reference for this form of conduct.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            AntiCompetitiveConduct::PriceFixing => "Competition Act s. 34(2)(a)",
            AntiCompetitiveConduct::OutputLimitation => "Competition Act s. 34(2)(b)",
            AntiCompetitiveConduct::MarketSharing => "Competition Act s. 34(2)(c)",
            AntiCompetitiveConduct::BidRigging => "Competition Act s. 34",
            AntiCompetitiveConduct::Discrimination => "Competition Act s. 34(2)(d)",
            AntiCompetitiveConduct::Tying => "Competition Act s. 34(2)(e)",
            AntiCompetitiveConduct::InformationExchange => "Competition Act s. 34",
        }
    }

    /// Returns a plain-language description of the conduct.
    pub fn description(&self) -> &'static str {
        match self {
            AntiCompetitiveConduct::PriceFixing => {
                "Directly or indirectly fixing purchase or selling prices or other trading conditions"
            }
            AntiCompetitiveConduct::OutputLimitation => {
                "Limiting or controlling production, markets, technical development or investment"
            }
            AntiCompetitiveConduct::MarketSharing => "Sharing markets or sources of supply",
            AntiCompetitiveConduct::BidRigging => {
                "Collusive tendering (bid rigging) among competitors"
            }
            AntiCompetitiveConduct::Discrimination => {
                "Applying dissimilar conditions to equivalent transactions"
            }
            AntiCompetitiveConduct::Tying => {
                "Tying the conclusion of contracts to unrelated supplementary obligations"
            }
            AntiCompetitiveConduct::InformationExchange => {
                "Exchange of commercially sensitive information amounting to a concerted practice"
            }
        }
    }

    /// Whether this conduct is, in CCCS practice, a "hardcore" restriction that
    /// infringes by **object** - that is, without proof of anti-competitive
    /// effect or any consideration of market share.
    ///
    /// Hardcore restrictions are price fixing, market sharing, bid rigging and
    /// output limitation.
    pub fn is_hardcore(&self) -> bool {
        matches!(
            self,
            AntiCompetitiveConduct::PriceFixing
                | AntiCompetitiveConduct::MarketSharing
                | AntiCompetitiveConduct::BidRigging
                | AntiCompetitiveConduct::OutputLimitation
        )
    }
}

/// Whether a restriction of competition arises by its object or only by its
/// effect.
///
/// "By object" infringements (hardcore cartels) require no proof of effect.
/// "By effect" infringements require an appreciable impact on competition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictionType {
    /// The agreement has the prevention, restriction or distortion of
    /// competition as its object - no proof of effect required.
    ByObject,
    /// The agreement restricts competition only by its effect - appreciability
    /// is relevant.
    ByEffect,
}

impl RestrictionType {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            RestrictionType::ByObject => {
                "Restriction by object - inherently harmful, no proof of effect required"
            }
            RestrictionType::ByEffect => {
                "Restriction by effect - requires an appreciable impact on competition"
            }
        }
    }
}

/// An exclusion or exemption that may take an agreement or conduct outside the
/// prohibitions (Third Schedule to the Competition Act).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Exclusion {
    /// Net economic benefit: the agreement improves production or distribution,
    /// or promotes technical or economic progress, while allowing consumers a
    /// fair share of the benefit and not eliminating competition (Third
    /// Schedule, para 9).
    NetEconomicBenefit,
    /// Services of general economic interest entrusted to an undertaking (Third
    /// Schedule, para 6).
    ServicesOfGeneralEconomicInterest,
    /// An agreement made to comply with a legal requirement (Third Schedule,
    /// para 5).
    LegalRequirement,
    /// Vertical agreements (historically excluded by order under the Third
    /// Schedule).
    VerticalAgreement,
    /// A restriction ancillary to, and directly related to and necessary for,
    /// a legitimate main transaction.
    Ancillary,
}

impl Exclusion {
    /// Returns the statutory reference for this exclusion.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            Exclusion::NetEconomicBenefit => "Competition Act Third Schedule para 9",
            Exclusion::ServicesOfGeneralEconomicInterest => "Competition Act Third Schedule para 6",
            Exclusion::LegalRequirement => "Competition Act Third Schedule para 5",
            Exclusion::VerticalAgreement => "Competition Act Third Schedule (vertical agreements)",
            Exclusion::Ancillary => "Competition Act Third Schedule (ancillary restrictions)",
        }
    }

    /// Returns a plain-language description of the exclusion.
    pub fn description(&self) -> &'static str {
        match self {
            Exclusion::NetEconomicBenefit => {
                "Net economic benefit: efficiency gains shared with consumers without eliminating competition"
            }
            Exclusion::ServicesOfGeneralEconomicInterest => {
                "Services of general economic interest entrusted to the undertaking"
            }
            Exclusion::LegalRequirement => "Agreement made to comply with a legal requirement",
            Exclusion::VerticalAgreement => {
                "Vertical agreement between undertakings at different levels of the supply chain"
            }
            Exclusion::Ancillary => {
                "Restriction ancillary and necessary to a legitimate main transaction"
            }
        }
    }
}

/// The basis on which an agreement is exempt, where applicable.
///
/// Beyond the Third Schedule exclusions, the Competition Act provides for
/// block exemptions (s. 36) and individual exemptions made by the Minister.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExemptionKind {
    /// A category of agreements exempted by a block exemption order (s. 36).
    BlockExemption,
    /// An individual exemption granted in respect of a particular agreement.
    IndividualExemption,
}

impl ExemptionKind {
    /// Returns the statutory reference for this kind of exemption.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            ExemptionKind::BlockExemption => "Competition Act s. 36",
            ExemptionKind::IndividualExemption => "Competition Act s. 34 (individual exemption)",
        }
    }
}

/// An undertaking - any entity engaged in economic activity, regardless of its
/// legal status (Competition Act s. 2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Undertaking {
    /// Name of the undertaking.
    pub name: String,
    /// Unique Entity Number (UEN), if registered in Singapore.
    pub uen: Option<String>,
}

impl Undertaking {
    /// Creates a new undertaking with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uen: None,
        }
    }

    /// Sets the UEN of the undertaking.
    pub fn with_uen(mut self, uen: impl Into<String>) -> Self {
        self.uen = Some(uen.into());
        self
    }
}

/// An agreement, decision or concerted practice assessed under Section 34.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntiCompetitiveAgreement {
    /// Identifier for the agreement.
    pub agreement_id: String,
    /// The undertakings party to the agreement.
    pub parties: Vec<Undertaking>,
    /// The form of conduct at issue.
    pub conduct: AntiCompetitiveConduct,
    /// Whether the restriction arises by object or by effect.
    pub restriction_type: RestrictionType,
    /// Combined market share of the parties, as a percentage (relevant only
    /// for "by effect" cases / appreciability).
    pub combined_market_share_percent: u8,
    /// Whether the agreement affects competition within Singapore.
    pub affects_singapore: bool,
    /// Any exclusion claimed under the Third Schedule.
    pub exclusion: Option<Exclusion>,
    /// Any exemption claimed.
    pub exemption: Option<ExemptionKind>,
}

impl AntiCompetitiveAgreement {
    /// Creates a new agreement record.
    ///
    /// The restriction type defaults to [`RestrictionType::ByObject`] when the
    /// conduct is hardcore, otherwise [`RestrictionType::ByEffect`].
    pub fn new(
        agreement_id: impl Into<String>,
        parties: Vec<Undertaking>,
        conduct: AntiCompetitiveConduct,
    ) -> Self {
        let restriction_type = if conduct.is_hardcore() {
            RestrictionType::ByObject
        } else {
            RestrictionType::ByEffect
        };
        Self {
            agreement_id: agreement_id.into(),
            parties,
            conduct,
            restriction_type,
            combined_market_share_percent: 0,
            affects_singapore: true,
            exclusion: None,
            exemption: None,
        }
    }

    /// Sets the restriction type explicitly.
    pub fn with_restriction_type(mut self, restriction_type: RestrictionType) -> Self {
        self.restriction_type = restriction_type;
        self
    }

    /// Sets the combined market share of the parties (0-100).
    pub fn with_market_share(mut self, percent: u8) -> Self {
        self.combined_market_share_percent = percent;
        self
    }

    /// Marks whether the agreement affects competition within Singapore.
    pub fn with_singapore_effect(mut self, affects: bool) -> Self {
        self.affects_singapore = affects;
        self
    }

    /// Records an exclusion claimed under the Third Schedule.
    pub fn with_exclusion(mut self, exclusion: Exclusion) -> Self {
        self.exclusion = Some(exclusion);
        self
    }

    /// Records an exemption claimed.
    pub fn with_exemption(mut self, exemption: ExemptionKind) -> Self {
        self.exemption = Some(exemption);
        self
    }

    /// Whether this agreement is excluded or exempt and so falls outside the
    /// prohibition.
    pub fn is_excluded_or_exempt(&self) -> bool {
        self.exclusion.is_some() || self.exemption.is_some()
    }
}

// ============================================================================
// Section 47 - abuse of a dominant position
// ============================================================================

/// Forms of abuse of a dominant position caught by the Section 47 prohibition.
///
/// Section 47(2) lists illustrative examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseType {
    /// Pricing below cost to eliminate or discipline a competitor.
    PredatoryPricing,
    /// Refusing to supply an essential input or to deal with a customer.
    RefusalToSupply,
    /// Requiring customers to deal exclusively with the dominant undertaking.
    ExclusiveDealing,
    /// Tying a separate product to the dominant product (s. 47(2)).
    Tying,
    /// Setting wholesale prices that squeeze the margins of downstream rivals.
    MarginSqueeze,
    /// Charging prices that bear no reasonable relation to economic value.
    ExcessivePricing,
    /// Applying dissimilar conditions to equivalent transactions (s. 47(2)).
    Discrimination,
}

impl AbuseType {
    /// Returns the statutory reference (all forms fall under s. 47).
    pub fn statute_reference(&self) -> &'static str {
        "Competition Act s. 47"
    }

    /// Returns a plain-language description of the abuse.
    pub fn description(&self) -> &'static str {
        match self {
            AbuseType::PredatoryPricing => "Predatory pricing below cost to foreclose competitors",
            AbuseType::RefusalToSupply => "Refusal to supply an essential input or to deal",
            AbuseType::ExclusiveDealing => "Imposing exclusive purchasing obligations on customers",
            AbuseType::Tying => "Tying a distinct product to the dominant product",
            AbuseType::MarginSqueeze => {
                "Margin squeeze foreclosing equally efficient downstream rivals"
            }
            AbuseType::ExcessivePricing => {
                "Charging prices with no reasonable relation to economic value"
            }
            AbuseType::Discrimination => {
                "Applying dissimilar conditions to equivalent transactions"
            }
        }
    }

    /// Whether this abuse is exclusionary (forecloses rivals) as opposed to
    /// exploitative (extracts from customers).
    pub fn is_exclusionary(&self) -> bool {
        matches!(
            self,
            AbuseType::PredatoryPricing
                | AbuseType::RefusalToSupply
                | AbuseType::ExclusiveDealing
                | AbuseType::Tying
                | AbuseType::MarginSqueeze
        )
    }
}

/// An undertaking whose dominance is being assessed under Section 47.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DominantUndertaking {
    /// The undertaking concerned.
    pub undertaking: Undertaking,
    /// Description of the relevant market.
    pub relevant_market: String,
    /// Market share on the relevant market, as a percentage (0-100).
    pub market_share_percent: u8,
    /// Whether there are high barriers to entry on the relevant market.
    pub high_barriers_to_entry: bool,
    /// Whether buyers wield significant countervailing power.
    pub countervailing_buyer_power: bool,
}

impl DominantUndertaking {
    /// Creates a new dominance assessment record.
    pub fn new(
        undertaking: Undertaking,
        relevant_market: impl Into<String>,
        market_share_percent: u8,
    ) -> Self {
        Self {
            undertaking,
            relevant_market: relevant_market.into(),
            market_share_percent,
            high_barriers_to_entry: false,
            countervailing_buyer_power: false,
        }
    }

    /// Marks the relevant market as having high barriers to entry.
    pub fn with_high_barriers_to_entry(mut self, value: bool) -> Self {
        self.high_barriers_to_entry = value;
        self
    }

    /// Marks the presence of countervailing buyer power.
    pub fn with_countervailing_buyer_power(mut self, value: bool) -> Self {
        self.countervailing_buyer_power = value;
        self
    }

    /// Whether the undertaking is *likely* to be dominant.
    ///
    /// Dominance turns on the ability to act independently of competitive
    /// pressures. A market share above
    /// [`DOMINANCE_INDICATIVE_SHARE_PERCENT`] is treated by the CCCS as
    /// indicative of dominance; high barriers to entry reinforce a finding of
    /// dominance, while strong countervailing buyer power may negate it. This
    /// is an indicative screen, not a conclusive legal finding.
    pub fn is_likely_dominant(&self) -> bool {
        if self.countervailing_buyer_power && self.market_share_percent < 75 {
            return false;
        }
        self.market_share_percent >= DOMINANCE_INDICATIVE_SHARE_PERCENT
    }
}

/// A claim that a dominant undertaking has abused its position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseOfDominance {
    /// Identifier for the claim.
    pub claim_id: String,
    /// The undertaking alleged to be dominant.
    pub undertaking: DominantUndertaking,
    /// The form of abuse alleged.
    pub abuse: AbuseType,
    /// Whether the conduct affects competition within Singapore.
    pub affects_singapore: bool,
    /// Whether the conduct is objectively justified (a defence to abuse).
    pub objectively_justified: bool,
}

impl AbuseOfDominance {
    /// Creates a new abuse-of-dominance claim.
    pub fn new(
        claim_id: impl Into<String>,
        undertaking: DominantUndertaking,
        abuse: AbuseType,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            undertaking,
            abuse,
            affects_singapore: true,
            objectively_justified: false,
        }
    }

    /// Marks whether the conduct affects competition within Singapore.
    pub fn with_singapore_effect(mut self, affects: bool) -> Self {
        self.affects_singapore = affects;
        self
    }

    /// Marks the conduct as objectively justified (e.g. genuine efficiency or
    /// legitimate commercial response).
    pub fn with_objective_justification(mut self, justified: bool) -> Self {
        self.objectively_justified = justified;
        self
    }
}

// ============================================================================
// Section 54 - mergers
// ============================================================================

/// A merger notification assessed for a substantial lessening of competition
/// (SLC) under Section 54.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerNotification {
    /// Identifier for the notification.
    pub notification_id: String,
    /// The merging undertakings.
    pub parties: Vec<Undertaking>,
    /// Description of the relevant market.
    pub relevant_market: String,
    /// Post-merger market share of the merged entity, as a percentage (0-100).
    pub merged_share_percent: u8,
    /// Combined share of the three largest firms post-merger (CR3), as a
    /// percentage (0-100).
    pub combined_top3_share_percent: u8,
    /// Whether the merger affects competition within Singapore.
    pub affects_singapore: bool,
    /// Annual turnover in Singapore of the merged business, in SGD cents (used
    /// for jurisdictional and penalty context).
    pub merged_sg_turnover_cents: u64,
}

impl MergerNotification {
    /// Creates a new merger notification record.
    pub fn new(
        notification_id: impl Into<String>,
        parties: Vec<Undertaking>,
        relevant_market: impl Into<String>,
        merged_share_percent: u8,
    ) -> Self {
        Self {
            notification_id: notification_id.into(),
            parties,
            relevant_market: relevant_market.into(),
            merged_share_percent,
            combined_top3_share_percent: 0,
            affects_singapore: true,
            merged_sg_turnover_cents: 0,
        }
    }

    /// Sets the combined CR3 share post-merger.
    pub fn with_top3_share(mut self, percent: u8) -> Self {
        self.combined_top3_share_percent = percent;
        self
    }

    /// Marks whether the merger affects competition within Singapore.
    pub fn with_singapore_effect(mut self, affects: bool) -> Self {
        self.affects_singapore = affects;
        self
    }

    /// Sets the merged entity's annual Singapore turnover in SGD cents.
    pub fn with_sg_turnover_cents(mut self, cents: u64) -> Self {
        self.merged_sg_turnover_cents = cents;
        self
    }
}

// ============================================================================
// Enforcement - penalties and leniency
// ============================================================================

/// An undertaking's standing under the CCCS leniency programme.
///
/// Immunity (or a substantial reduction) may be available to a cartel
/// participant that comes forward and cooperates with the CCCS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeniencyStatus {
    /// First to apply with qualifying evidence - eligible for full immunity or
    /// the greatest reduction.
    FirstToApply,
    /// A subsequent applicant - eligible for a partial reduction.
    SubsequentApplicant,
    /// Has not applied for leniency.
    NotApplied,
}

impl LeniencyStatus {
    /// Returns the proportion of the penalty that remains payable after the
    /// leniency reduction (0.0 = full immunity, 1.0 = no reduction).
    ///
    /// The first qualifying applicant is modelled as obtaining full immunity;
    /// a subsequent applicant obtains a 50% reduction; a non-applicant pays in
    /// full. These reflect the upper end of the published CCCS leniency
    /// reductions and are indicative.
    pub fn remaining_fraction(&self) -> f64 {
        match self {
            LeniencyStatus::FirstToApply => 0.0,
            LeniencyStatus::SubsequentApplicant => 0.5,
            LeniencyStatus::NotApplied => 1.0,
        }
    }

    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            LeniencyStatus::FirstToApply => {
                "First to apply - eligible for full immunity from financial penalties"
            }
            LeniencyStatus::SubsequentApplicant => {
                "Subsequent applicant - eligible for a reduction in financial penalties"
            }
            LeniencyStatus::NotApplied => "Not applied for leniency - full penalty payable",
        }
    }
}

/// Computes the maximum financial penalty under Section 69(4).
///
/// The CCCS may impose a penalty of up to 10% of the turnover of the business
/// in Singapore for each year of infringement, for up to a maximum of 3 years.
///
/// `years_of_infringement` is capped at [`MAX_PENALTY_YEARS`]; the penalty is
/// 10% of `annual_sg_turnover_cents` per counted year. The arithmetic is
/// saturating to avoid overflow.
///
/// # Examples
///
/// ```
/// use legalis_sg::competition::max_financial_penalty_cents;
///
/// // SGD 10,000,000 turnover (1_000_000_000 cents), 2 years of infringement.
/// let cap = max_financial_penalty_cents(1_000_000_000, 2);
/// // 10% per year * 2 years = 20% of turnover.
/// assert_eq!(cap, 200_000_000);
/// ```
pub fn max_financial_penalty_cents(
    annual_sg_turnover_cents: u64,
    years_of_infringement: u32,
) -> u64 {
    let years = years_of_infringement.min(MAX_PENALTY_YEARS) as u64;
    let per_year = annual_sg_turnover_cents.saturating_mul(MAX_PENALTY_TURNOVER_PERCENT) / 100;
    per_year.saturating_mul(years)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conduct_statute_references() {
        assert_eq!(
            AntiCompetitiveConduct::PriceFixing.statute_reference(),
            "Competition Act s. 34(2)(a)"
        );
        assert_eq!(
            AntiCompetitiveConduct::MarketSharing.statute_reference(),
            "Competition Act s. 34(2)(c)"
        );
        assert_eq!(
            AntiCompetitiveConduct::Tying.statute_reference(),
            "Competition Act s. 34(2)(e)"
        );
        assert_eq!(
            AntiCompetitiveConduct::BidRigging.statute_reference(),
            "Competition Act s. 34"
        );
    }

    #[test]
    fn test_hardcore_classification() {
        assert!(AntiCompetitiveConduct::PriceFixing.is_hardcore());
        assert!(AntiCompetitiveConduct::MarketSharing.is_hardcore());
        assert!(AntiCompetitiveConduct::BidRigging.is_hardcore());
        assert!(AntiCompetitiveConduct::OutputLimitation.is_hardcore());
        assert!(!AntiCompetitiveConduct::InformationExchange.is_hardcore());
        assert!(!AntiCompetitiveConduct::Discrimination.is_hardcore());
    }

    #[test]
    fn test_agreement_defaults_restriction_type() {
        let parties = vec![
            Undertaking::new("Alpha Pte Ltd"),
            Undertaking::new("Beta Pte Ltd"),
        ];
        let hardcore = AntiCompetitiveAgreement::new(
            "agr-1",
            parties.clone(),
            AntiCompetitiveConduct::PriceFixing,
        );
        assert_eq!(hardcore.restriction_type, RestrictionType::ByObject);

        let soft = AntiCompetitiveAgreement::new(
            "agr-2",
            parties,
            AntiCompetitiveConduct::InformationExchange,
        );
        assert_eq!(soft.restriction_type, RestrictionType::ByEffect);
    }

    #[test]
    fn test_agreement_builders() {
        let parties = vec![Undertaking::new("Gamma").with_uen("201912345A")];
        let agreement = AntiCompetitiveAgreement::new(
            "agr-3",
            parties,
            AntiCompetitiveConduct::InformationExchange,
        )
        .with_market_share(15)
        .with_exclusion(Exclusion::NetEconomicBenefit);

        assert_eq!(agreement.combined_market_share_percent, 15);
        assert!(agreement.is_excluded_or_exempt());
        assert_eq!(agreement.parties[0].uen.as_deref(), Some("201912345A"));
    }

    #[test]
    fn test_exclusion_references() {
        assert_eq!(
            Exclusion::NetEconomicBenefit.statute_reference(),
            "Competition Act Third Schedule para 9"
        );
        assert_eq!(
            Exclusion::LegalRequirement.statute_reference(),
            "Competition Act Third Schedule para 5"
        );
    }

    #[test]
    fn test_abuse_type_metadata() {
        assert_eq!(
            AbuseType::PredatoryPricing.statute_reference(),
            "Competition Act s. 47"
        );
        assert!(AbuseType::PredatoryPricing.is_exclusionary());
        assert!(!AbuseType::ExcessivePricing.is_exclusionary());
    }

    #[test]
    fn test_dominance_indicative_threshold() {
        let strong = DominantUndertaking::new(Undertaking::new("DominantCo"), "Retail fuel", 65);
        assert!(strong.is_likely_dominant());

        let weak = DominantUndertaking::new(Undertaking::new("SmallCo"), "Retail fuel", 30);
        assert!(!weak.is_likely_dominant());

        // Boundary: exactly the indicative threshold is treated as dominant.
        let boundary = DominantUndertaking::new(
            Undertaking::new("EdgeCo"),
            "Retail fuel",
            DOMINANCE_INDICATIVE_SHARE_PERCENT,
        );
        assert!(boundary.is_likely_dominant());
    }

    #[test]
    fn test_countervailing_buyer_power_negates_dominance() {
        let undertaking =
            DominantUndertaking::new(Undertaking::new("SupplierCo"), "Industrial widgets", 65)
                .with_countervailing_buyer_power(true);
        assert!(!undertaking.is_likely_dominant());
    }

    #[test]
    fn test_leniency_remaining_fraction() {
        assert_eq!(LeniencyStatus::FirstToApply.remaining_fraction(), 0.0);
        assert_eq!(
            LeniencyStatus::SubsequentApplicant.remaining_fraction(),
            0.5
        );
        assert_eq!(LeniencyStatus::NotApplied.remaining_fraction(), 1.0);
    }

    #[test]
    fn test_max_financial_penalty_caps_years() {
        // 5 years requested, but capped at 3.
        let turnover = 1_000_000_000; // SGD 10,000,000
        let cap = max_financial_penalty_cents(turnover, 5);
        // 10% * 3 years = 30%.
        assert_eq!(cap, 300_000_000);
    }

    #[test]
    fn test_max_financial_penalty_single_year() {
        let turnover = 5_000_000; // SGD 50,000
        let cap = max_financial_penalty_cents(turnover, 1);
        assert_eq!(cap, 500_000); // 10% of turnover
    }

    #[test]
    fn test_max_financial_penalty_zero_years() {
        let cap = max_financial_penalty_cents(1_000_000, 0);
        assert_eq!(cap, 0);
    }

    #[test]
    fn test_merger_builders() {
        let parties = vec![Undertaking::new("MergeA"), Undertaking::new("MergeB")];
        let merger = MergerNotification::new("mrg-1", parties, "Telecoms", 45)
            .with_top3_share(80)
            .with_sg_turnover_cents(2_000_000_000);
        assert_eq!(merger.merged_share_percent, 45);
        assert_eq!(merger.combined_top3_share_percent, 80);
        assert_eq!(merger.merged_sg_turnover_cents, 2_000_000_000);
    }
}
