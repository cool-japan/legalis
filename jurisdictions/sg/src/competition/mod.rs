//! Competition Act 2004 (Singapore)
//!
//! Type-safe modelling of Singapore's general competition law framework under
//! the **Competition Act 2004**, administered and enforced by the **Competition
//! and Consumer Commission of Singapore (CCCS)** (formerly the Competition
//! Commission of Singapore).
//!
//! # Overview
//!
//! The Competition Act 2004 establishes a three-pronged general competition
//! regime modelled on European Union competition law. It applies to
//! undertakings - any entity engaged in economic activity - whose conduct
//! affects competition *within Singapore*.
//!
//! ## The Three Prohibitions
//!
//! ### Section 34 - Anti-competitive agreements
//!
//! Section 34(1) prohibits agreements between undertakings, decisions by
//! associations of undertakings, and concerted practices which have as their
//! object or effect the prevention, restriction or distortion of competition
//! within Singapore. Section 34(2) lists illustrative examples:
//!
//! - **(a)** directly or indirectly fixing purchase or selling prices or other
//!   trading conditions;
//! - **(b)** limiting or controlling production, markets, technical development
//!   or investment;
//! - **(c)** sharing markets or sources of supply;
//! - **(d)** applying dissimilar conditions to equivalent transactions; and
//! - **(e)** tying.
//!
//! *Hardcore* restrictions - price fixing, market sharing, bid rigging and
//! output limitation - are infringements "by object" and require no proof of
//! effect or any consideration of market share. Other restrictions are caught
//! only where they have an appreciable anti-competitive *effect*.
//!
//! ### Section 47 - Abuse of a dominant position
//!
//! Section 47(1) prohibits conduct on the part of one or more undertakings
//! which amounts to the abuse of a dominant position in any market in
//! Singapore. Section 47(2) gives examples including predatory pricing,
//! limiting production/markets/technical development to the prejudice of
//! consumers, applying dissimilar conditions to equivalent transactions, and
//! tying. The CCCS treats a market share above roughly
//! [`DOMINANCE_INDICATIVE_SHARE_PERCENT`]% as *indicative* (not conclusive) of
//! dominance.
//!
//! ### Section 54 - Mergers
//!
//! Section 54 prohibits mergers that have resulted, or may be expected to
//! result, in a substantial lessening of competition (SLC) within any market in
//! Singapore. The CCCS's indicative thresholds are a merged-entity share of
//! [`MERGER_SINGLE_SHARE_THRESHOLD_PERCENT`]% or more, or a combined CR3 share
//! of [`MERGER_CR3_THRESHOLD_PERCENT`]% or more together with a merged-entity
//! share of [`MERGER_COMBINED_MERGED_SHARE_THRESHOLD_PERCENT`]% or more.
//!
//! ## Enforcement & Penalties
//!
//! The CCCS may impose a financial penalty of up to 10% of the turnover of the
//! business in Singapore for each year of infringement, up to a maximum of 3
//! years (s. 69(4)) - see [`max_financial_penalty_cents`]. A **leniency
//! programme** offers immunity or a reduction to cartel members that come
//! forward and cooperate - see [`LeniencyStatus`].
//!
//! ## Exclusions & Exemptions
//!
//! The **Third Schedule** excludes certain agreements from the s. 34 and s. 47
//! prohibitions, including agreements with a **net economic benefit**
//! (para 9), services of general economic interest, agreements made to comply
//! with legal requirements, and (historically) vertical agreements. Block
//! exemptions (s. 36) and individual exemptions are also provided for. See
//! [`Exclusion`] and [`ExemptionKind`].
//!
//! # Example
//!
//! ```rust
//! use legalis_sg::competition::*;
//!
//! // A price-fixing cartel between two competitors.
//! let parties = vec![
//!     Undertaking::new("Alpha Pte Ltd").with_uen("201801234A"),
//!     Undertaking::new("Beta Pte Ltd"),
//! ];
//! let agreement = AntiCompetitiveAgreement::new(
//!     "agr-001",
//!     parties,
//!     AntiCompetitiveConduct::PriceFixing,
//! );
//!
//! // Hardcore restrictions infringe regardless of market share.
//! match assess_section_34(&agreement) {
//!     Err(CompetitionError::HardcoreRestriction { conduct }) => {
//!         println!("Infringement by object: {conduct}");
//!     }
//!     other => panic!("expected a hardcore infringement, got {other:?}"),
//! }
//!
//! // The CCCS may fine up to 10% of Singapore turnover per year (max 3 years).
//! // SGD 10,000,000 turnover, infringing for 2 years.
//! let cap = max_financial_penalty_cents(1_000_000_000, 2);
//! assert_eq!(cap, 200_000_000); // SGD 2,000,000
//!
//! // The first cartel member to apply for leniency obtains full immunity.
//! let payable = compute_penalty_cents(
//!     1_000_000_000,
//!     2,
//!     150_000_000,
//!     LeniencyStatus::FirstToApply,
//! )
//! .expect("within cap");
//! assert_eq!(payable, 0);
//! ```
//!
//! # Statute References
//!
//! - `Competition Act s. 34` - anti-competitive agreements
//! - `Competition Act s. 47` - abuse of a dominant position
//! - `Competition Act s. 54` - mergers and SLC
//! - `Competition Act s. 69(4)` - financial penalties
//! - `Competition Act Third Schedule` - exclusions
//!
//! # Module Structure
//!
//! - [`error`] - error types for the three prohibitions and enforcement
//! - [`types`] - undertakings, agreements, dominance, mergers, penalties
//! - [`validator`] - assessment functions and report structs

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agreement_json_roundtrip() {
        let parties = vec![
            Undertaking::new("Alpha Pte Ltd").with_uen("201801234A"),
            Undertaking::new("Beta Pte Ltd"),
        ];
        let agreement =
            AntiCompetitiveAgreement::new("agr-001", parties, AntiCompetitiveConduct::PriceFixing)
                .with_market_share(30)
                .with_exclusion(Exclusion::NetEconomicBenefit);

        let json = serde_json::to_string(&agreement).expect("serialize");
        let decoded: AntiCompetitiveAgreement = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(agreement, decoded);
    }

    #[test]
    fn test_abuse_json_roundtrip() {
        let undertaking = DominantUndertaking::new(Undertaking::new("BigCo"), "Cement", 70)
            .with_high_barriers_to_entry(true);
        let claim = AbuseOfDominance::new("c-1", undertaking, AbuseType::PredatoryPricing);

        let json = serde_json::to_string(&claim).expect("serialize");
        let decoded: AbuseOfDominance = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(claim, decoded);
    }

    #[test]
    fn test_merger_json_roundtrip() {
        let parties = vec![Undertaking::new("MergeA"), Undertaking::new("MergeB")];
        let merger = MergerNotification::new("m-1", parties, "Telecoms", 45)
            .with_top3_share(80)
            .with_sg_turnover_cents(2_000_000_000);

        let json = serde_json::to_string(&merger).expect("serialize");
        let decoded: MergerNotification = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(merger, decoded);
    }

    #[test]
    fn test_end_to_end_cartel_assessment() {
        let parties = vec![Undertaking::new("Alpha"), Undertaking::new("Beta")];
        let agreement =
            AntiCompetitiveAgreement::new("agr-e2e", parties, AntiCompetitiveConduct::BidRigging);
        assert!(assess_section_34(&agreement).is_err());

        let report = assess_section_34_report(&agreement).expect("report");
        assert!(report.is_infringement);
        assert!(report.is_by_object);
    }
}
