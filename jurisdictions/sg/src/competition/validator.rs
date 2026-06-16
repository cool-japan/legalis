//! Competition Act 2004 - Assessment Logic
//!
//! Assessment functions for the three prohibitions of the Competition Act 2004:
//!
//! - [`assess_section_34`] - anti-competitive agreements
//! - [`assess_section_47`] - abuse of a dominant position
//! - [`assess_merger`] - mergers leading to a substantial lessening of competition
//!
//! Each function returns `Ok(())` (or an `Ok` report) where no infringement is
//! made out, and a [`CompetitionError`] flagging the relevant prohibition
//! otherwise.

use super::error::{CompetitionError, Result};
use super::types::*;

/// Appreciability screen for "by effect" cases.
///
/// CCCS guidance treats agreements between competitors below an aggregate
/// market share of roughly 20%, and between non-competitors below roughly 25%,
/// as generally not having an appreciable effect on competition. This module
/// applies the more conservative 20% screen for combined market share.
pub const APPRECIABILITY_SHARE_THRESHOLD_PERCENT: u8 = 20;

/// Outcome of a Section 34 assessment.
#[derive(Debug, Clone, PartialEq)]
pub struct Section34Report {
    /// Whether an infringement of the s. 34 prohibition is made out.
    pub is_infringement: bool,
    /// Whether the restriction is "by object" (hardcore).
    pub is_by_object: bool,
    /// Whether the agreement is excluded or exempt.
    pub is_excluded_or_exempt: bool,
    /// The statutory reference for the conduct assessed.
    pub statute_reference: &'static str,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses an agreement against the Section 34 prohibition.
///
/// Logic:
/// 1. The prohibition only bites where competition *within Singapore* is
///    affected (s. 34(1)); otherwise [`CompetitionError::NoSingaporeNexus`].
/// 2. An excluded or exempt agreement (Third Schedule / block / individual
///    exemption) is not caught - returns `Ok`.
/// 3. Hardcore "by object" restrictions infringe regardless of market share -
///    [`CompetitionError::HardcoreRestriction`].
/// 4. "By effect" restrictions infringe only where the combined market share
///    is appreciable (>= [`APPRECIABILITY_SHARE_THRESHOLD_PERCENT`]) -
///    [`CompetitionError::Section34Infringement`].
pub fn assess_section_34(agreement: &AntiCompetitiveAgreement) -> Result<()> {
    if !agreement.affects_singapore {
        return Err(CompetitionError::NoSingaporeNexus);
    }

    if agreement.combined_market_share_percent > 100 {
        return Err(CompetitionError::InvalidMarketShare {
            value: agreement.combined_market_share_percent as u16,
        });
    }

    // Excluded or exempt agreements fall outside the prohibition.
    if agreement.is_excluded_or_exempt() {
        return Ok(());
    }

    let conduct_desc = agreement.conduct.description().to_string();

    match agreement.restriction_type {
        RestrictionType::ByObject => Err(CompetitionError::HardcoreRestriction {
            conduct: conduct_desc,
        }),
        RestrictionType::ByEffect => {
            if agreement.combined_market_share_percent >= APPRECIABILITY_SHARE_THRESHOLD_PERCENT {
                Err(CompetitionError::Section34Infringement {
                    conduct: conduct_desc,
                })
            } else {
                Ok(())
            }
        }
    }
}

/// Produces a detailed Section 34 report (never returns `Err` for an
/// infringement; instead records it in the report).
///
/// This is useful where the caller wants the full picture rather than
/// short-circuiting on the first issue.
pub fn assess_section_34_report(agreement: &AntiCompetitiveAgreement) -> Result<Section34Report> {
    if agreement.combined_market_share_percent > 100 {
        return Err(CompetitionError::InvalidMarketShare {
            value: agreement.combined_market_share_percent as u16,
        });
    }

    let mut notes = Vec::new();
    let is_by_object = matches!(agreement.restriction_type, RestrictionType::ByObject);
    let is_excluded_or_exempt = agreement.is_excluded_or_exempt();

    if !agreement.affects_singapore {
        notes.push("No appreciable effect on competition within Singapore (s. 34(1))".to_string());
    }

    if let Some(exclusion) = agreement.exclusion {
        notes.push(format!(
            "Exclusion claimed: {} ({})",
            exclusion.description(),
            exclusion.statute_reference()
        ));
    }
    if let Some(exemption) = agreement.exemption {
        notes.push(format!(
            "Exemption claimed: {}",
            exemption.statute_reference()
        ));
    }

    let is_infringement = match assess_section_34(agreement) {
        Ok(()) => false,
        Err(CompetitionError::HardcoreRestriction { .. })
        | Err(CompetitionError::Section34Infringement { .. }) => true,
        Err(CompetitionError::NoSingaporeNexus) => false,
        Err(other) => return Err(other),
    };

    if is_by_object && is_infringement {
        notes.push(
            "Hardcore restriction by object - infringement irrespective of market share"
                .to_string(),
        );
    }

    Ok(Section34Report {
        is_infringement,
        is_by_object,
        is_excluded_or_exempt,
        statute_reference: agreement.conduct.statute_reference(),
        notes,
    })
}

/// Outcome of a Section 47 assessment.
#[derive(Debug, Clone, PartialEq)]
pub struct Section47Report {
    /// Whether an abuse of a dominant position is made out.
    pub is_abuse: bool,
    /// Whether the undertaking is (likely) dominant.
    pub is_dominant: bool,
    /// The market share assessed, as a percentage.
    pub market_share_percent: u8,
    /// Whether the conduct was objectively justified.
    pub objectively_justified: bool,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses a claim against the Section 47 prohibition.
///
/// Logic:
/// 1. The prohibition only bites where competition *within Singapore* is
///    affected (s. 47(1)); otherwise [`CompetitionError::NoSingaporeNexus`].
/// 2. There must be (likely) dominance; absent it, abuse cannot arise -
///    [`CompetitionError::NotDominant`].
/// 3. Objectively justified conduct is not abusive - returns `Ok`.
/// 4. Otherwise the abuse infringes - [`CompetitionError::Section47Abuse`].
pub fn assess_section_47(claim: &AbuseOfDominance) -> Result<()> {
    if !claim.affects_singapore {
        return Err(CompetitionError::NoSingaporeNexus);
    }

    if claim.undertaking.market_share_percent > 100 {
        return Err(CompetitionError::InvalidMarketShare {
            value: claim.undertaking.market_share_percent as u16,
        });
    }

    if !claim.undertaking.is_likely_dominant() {
        return Err(CompetitionError::NotDominant {
            market_share_percent: claim.undertaking.market_share_percent,
        });
    }

    // A dominant undertaking may still escape liability where its conduct is
    // objectively justified.
    if claim.objectively_justified {
        return Ok(());
    }

    Err(CompetitionError::Section47Abuse {
        abuse: claim.abuse.description().to_string(),
    })
}

/// Produces a detailed Section 47 report.
pub fn assess_section_47_report(claim: &AbuseOfDominance) -> Result<Section47Report> {
    if claim.undertaking.market_share_percent > 100 {
        return Err(CompetitionError::InvalidMarketShare {
            value: claim.undertaking.market_share_percent as u16,
        });
    }

    let mut notes = Vec::new();
    let is_dominant = claim.undertaking.is_likely_dominant();
    let market_share_percent = claim.undertaking.market_share_percent;

    notes.push(format!(
        "Market share {market_share_percent}% on relevant market: {}",
        claim.undertaking.relevant_market
    ));

    if claim.undertaking.high_barriers_to_entry {
        notes.push("High barriers to entry reinforce a finding of dominance".to_string());
    }
    if claim.undertaking.countervailing_buyer_power {
        notes.push("Countervailing buyer power weighs against dominance".to_string());
    }

    let is_abuse = match assess_section_47(claim) {
        Ok(()) => false,
        Err(CompetitionError::Section47Abuse { .. }) => true,
        Err(CompetitionError::NotDominant { .. }) | Err(CompetitionError::NoSingaporeNexus) => {
            false
        }
        Err(other) => return Err(other),
    };

    if claim.objectively_justified {
        notes.push("Conduct is objectively justified - no abuse".to_string());
    }

    Ok(Section47Report {
        is_abuse,
        is_dominant,
        market_share_percent,
        objectively_justified: claim.objectively_justified,
        notes,
    })
}

/// Outcome of a merger assessment under Section 54.
#[derive(Debug, Clone, PartialEq)]
pub struct MergerReport {
    /// Whether the merger may give rise to a substantial lessening of
    /// competition (SLC).
    pub may_raise_concerns: bool,
    /// Whether the single-firm indicative threshold is met.
    pub single_firm_threshold_met: bool,
    /// Whether the combined CR3 indicative threshold is met.
    pub combined_threshold_met: bool,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses a merger against the Section 54 prohibition.
///
/// CCCS indicative SLC thresholds:
/// - the merged entity has a market share of 40% or more; **or**
/// - the combined share of the three largest firms (CR3) is 70% or more **and**
///   the merged entity has 20% or more.
///
/// Where neither threshold is met, the merger is unlikely to raise concerns
/// and the function returns `Ok`. Where a threshold is met, it returns
/// [`CompetitionError::SubstantialLesseningOfCompetition`].
pub fn assess_merger(merger: &MergerNotification) -> Result<()> {
    if !merger.affects_singapore {
        return Err(CompetitionError::NoSingaporeNexus);
    }

    if merger.merged_share_percent > 100 || merger.combined_top3_share_percent > 100 {
        let value = merger
            .merged_share_percent
            .max(merger.combined_top3_share_percent) as u16;
        return Err(CompetitionError::InvalidMarketShare { value });
    }

    let single_firm = merger.merged_share_percent >= MERGER_SINGLE_SHARE_THRESHOLD_PERCENT;
    let combined = merger.combined_top3_share_percent >= MERGER_CR3_THRESHOLD_PERCENT
        && merger.merged_share_percent >= MERGER_COMBINED_MERGED_SHARE_THRESHOLD_PERCENT;

    if single_firm || combined {
        Err(CompetitionError::SubstantialLesseningOfCompetition {
            merged_share_percent: merger.merged_share_percent,
        })
    } else {
        Ok(())
    }
}

/// Produces a detailed merger report.
pub fn assess_merger_report(merger: &MergerNotification) -> Result<MergerReport> {
    if merger.merged_share_percent > 100 || merger.combined_top3_share_percent > 100 {
        let value = merger
            .merged_share_percent
            .max(merger.combined_top3_share_percent) as u16;
        return Err(CompetitionError::InvalidMarketShare { value });
    }

    let single_firm_threshold_met =
        merger.merged_share_percent >= MERGER_SINGLE_SHARE_THRESHOLD_PERCENT;
    let combined_threshold_met = merger.combined_top3_share_percent >= MERGER_CR3_THRESHOLD_PERCENT
        && merger.merged_share_percent >= MERGER_COMBINED_MERGED_SHARE_THRESHOLD_PERCENT;

    let mut notes = Vec::new();
    if single_firm_threshold_met {
        notes.push(format!(
            "Merged entity share {}% meets the {}% single-firm indicative threshold",
            merger.merged_share_percent, MERGER_SINGLE_SHARE_THRESHOLD_PERCENT
        ));
    }
    if combined_threshold_met {
        notes.push(format!(
            "CR3 share {}% with merged entity {}% meets the combined indicative threshold",
            merger.combined_top3_share_percent, merger.merged_share_percent
        ));
    }
    if !single_firm_threshold_met && !combined_threshold_met {
        notes.push("Neither indicative threshold met - SLC unlikely".to_string());
    }
    if !merger.affects_singapore {
        notes.push("No effect on competition within Singapore (s. 54)".to_string());
    }

    Ok(MergerReport {
        may_raise_concerns: (single_firm_threshold_met || combined_threshold_met)
            && merger.affects_singapore,
        single_firm_threshold_met,
        combined_threshold_met,
        notes,
    })
}

/// Computes the financial penalty for an infringement after applying the
/// statutory cap (s. 69(4)) and any leniency reduction.
///
/// The cap is computed via [`max_financial_penalty_cents`]; the leniency
/// status then scales the payable amount via
/// [`LeniencyStatus::remaining_fraction`].
///
/// Returns [`CompetitionError::PenaltyExceedsCap`] if `proposed_penalty_cents`
/// exceeds the statutory maximum (before leniency).
pub fn compute_penalty_cents(
    annual_sg_turnover_cents: u64,
    years_of_infringement: u32,
    proposed_penalty_cents: u64,
    leniency: LeniencyStatus,
) -> Result<u64> {
    let cap = max_financial_penalty_cents(annual_sg_turnover_cents, years_of_infringement);
    if proposed_penalty_cents > cap {
        return Err(CompetitionError::PenaltyExceedsCap {
            proposed_cents: proposed_penalty_cents,
            maximum_cents: cap,
        });
    }

    let fraction = leniency.remaining_fraction();
    let payable = (proposed_penalty_cents as f64 * fraction).round() as u64;
    Ok(payable)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_parties() -> Vec<Undertaking> {
        vec![
            Undertaking::new("Alpha Pte Ltd"),
            Undertaking::new("Beta Pte Ltd"),
        ]
    }

    #[test]
    fn test_section_34_hardcore_infringes_regardless_of_share() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-1",
            two_parties(),
            AntiCompetitiveConduct::PriceFixing,
        )
        .with_market_share(2); // tiny share, but hardcore

        match assess_section_34(&agreement) {
            Err(CompetitionError::HardcoreRestriction { .. }) => {}
            other => panic!("expected HardcoreRestriction, got {other:?}"),
        }
    }

    #[test]
    fn test_section_34_bid_rigging_is_hardcore() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-bid",
            two_parties(),
            AntiCompetitiveConduct::BidRigging,
        );
        assert!(assess_section_34(&agreement).is_err());
    }

    #[test]
    fn test_section_34_by_effect_below_appreciability_is_ok() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-2",
            two_parties(),
            AntiCompetitiveConduct::InformationExchange,
        )
        .with_restriction_type(RestrictionType::ByEffect)
        .with_market_share(10);
        assert!(assess_section_34(&agreement).is_ok());
    }

    #[test]
    fn test_section_34_by_effect_appreciable_infringes() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-3",
            two_parties(),
            AntiCompetitiveConduct::InformationExchange,
        )
        .with_restriction_type(RestrictionType::ByEffect)
        .with_market_share(35);
        match assess_section_34(&agreement) {
            Err(CompetitionError::Section34Infringement { .. }) => {}
            other => panic!("expected Section34Infringement, got {other:?}"),
        }
    }

    #[test]
    fn test_section_34_excluded_agreement_is_ok() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-4",
            two_parties(),
            AntiCompetitiveConduct::PriceFixing,
        )
        .with_exclusion(Exclusion::NetEconomicBenefit);
        assert!(assess_section_34(&agreement).is_ok());
    }

    #[test]
    fn test_section_34_no_singapore_nexus() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-5",
            two_parties(),
            AntiCompetitiveConduct::PriceFixing,
        )
        .with_singapore_effect(false);
        assert_eq!(
            assess_section_34(&agreement),
            Err(CompetitionError::NoSingaporeNexus)
        );
    }

    #[test]
    fn test_section_34_report_records_infringement() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-6",
            two_parties(),
            AntiCompetitiveConduct::MarketSharing,
        );
        let report = assess_section_34_report(&agreement).expect("report");
        assert!(report.is_infringement);
        assert!(report.is_by_object);
        assert!(!report.is_excluded_or_exempt);
        assert!(report.notes.iter().any(|n| n.contains("Hardcore")));
    }

    #[test]
    fn test_section_47_requires_dominance() {
        let undertaking = DominantUndertaking::new(Undertaking::new("SmallCo"), "Widgets", 25);
        let claim = AbuseOfDominance::new("c-1", undertaking, AbuseType::PredatoryPricing);
        match assess_section_47(&claim) {
            Err(CompetitionError::NotDominant {
                market_share_percent,
            }) => {
                assert_eq!(market_share_percent, 25);
            }
            other => panic!("expected NotDominant, got {other:?}"),
        }
    }

    #[test]
    fn test_section_47_dominant_abuse_infringes() {
        let undertaking = DominantUndertaking::new(Undertaking::new("BigCo"), "Cement", 70)
            .with_high_barriers_to_entry(true);
        let claim = AbuseOfDominance::new("c-2", undertaking, AbuseType::PredatoryPricing);
        match assess_section_47(&claim) {
            Err(CompetitionError::Section47Abuse { .. }) => {}
            other => panic!("expected Section47Abuse, got {other:?}"),
        }
    }

    #[test]
    fn test_section_47_objective_justification_is_ok() {
        let undertaking = DominantUndertaking::new(Undertaking::new("BigCo"), "Cement", 70);
        let claim = AbuseOfDominance::new("c-3", undertaking, AbuseType::RefusalToSupply)
            .with_objective_justification(true);
        assert!(assess_section_47(&claim).is_ok());
    }

    #[test]
    fn test_section_47_report() {
        let undertaking = DominantUndertaking::new(Undertaking::new("BigCo"), "Cement", 80);
        let claim = AbuseOfDominance::new("c-4", undertaking, AbuseType::ExclusiveDealing);
        let report = assess_section_47_report(&claim).expect("report");
        assert!(report.is_dominant);
        assert!(report.is_abuse);
        assert_eq!(report.market_share_percent, 80);
    }

    #[test]
    fn test_merger_single_firm_threshold() {
        let merger = MergerNotification::new("m-1", two_parties(), "Supermarkets", 45);
        match assess_merger(&merger) {
            Err(CompetitionError::SubstantialLesseningOfCompetition {
                merged_share_percent,
            }) => {
                assert_eq!(merged_share_percent, 45);
            }
            other => panic!("expected SLC, got {other:?}"),
        }
    }

    #[test]
    fn test_merger_combined_threshold() {
        // Below 40% single-firm, but CR3 >= 70% and merged >= 20%.
        let merger =
            MergerNotification::new("m-2", two_parties(), "Telecoms", 25).with_top3_share(75);
        assert!(assess_merger(&merger).is_err());
    }

    #[test]
    fn test_merger_below_thresholds_is_ok() {
        let merger = MergerNotification::new("m-3", two_parties(), "Cafes", 15).with_top3_share(40);
        assert!(assess_merger(&merger).is_ok());
    }

    #[test]
    fn test_merger_report_notes() {
        let merger = MergerNotification::new("m-4", two_parties(), "Telecoms", 45);
        let report = assess_merger_report(&merger).expect("report");
        assert!(report.may_raise_concerns);
        assert!(report.single_firm_threshold_met);
        assert!(!report.notes.is_empty());
    }

    #[test]
    fn test_penalty_within_cap_first_to_apply_is_zero() {
        // SGD 10M turnover, 2 years -> cap = 20% = SGD 2M (200_000_000 cents).
        let payable =
            compute_penalty_cents(1_000_000_000, 2, 150_000_000, LeniencyStatus::FirstToApply)
                .expect("penalty");
        assert_eq!(payable, 0);
    }

    #[test]
    fn test_penalty_within_cap_subsequent_applicant_halved() {
        let payable = compute_penalty_cents(
            1_000_000_000,
            2,
            100_000_000,
            LeniencyStatus::SubsequentApplicant,
        )
        .expect("penalty");
        assert_eq!(payable, 50_000_000);
    }

    #[test]
    fn test_penalty_exceeds_cap() {
        // Cap for SGD 1M turnover, 1 year = 10% = SGD 100k (10_000_000 cents).
        let result = compute_penalty_cents(100_000_000, 1, 50_000_000, LeniencyStatus::NotApplied);
        match result {
            Err(CompetitionError::PenaltyExceedsCap {
                proposed_cents,
                maximum_cents,
            }) => {
                assert_eq!(proposed_cents, 50_000_000);
                assert_eq!(maximum_cents, 10_000_000);
            }
            other => panic!("expected PenaltyExceedsCap, got {other:?}"),
        }
    }

    #[test]
    fn test_invalid_market_share_rejected() {
        let agreement = AntiCompetitiveAgreement::new(
            "agr-bad",
            two_parties(),
            AntiCompetitiveConduct::InformationExchange,
        )
        .with_market_share(120);
        match assess_section_34(&agreement) {
            Err(CompetitionError::InvalidMarketShare { value }) => assert_eq!(value, 120),
            other => panic!("expected InvalidMarketShare, got {other:?}"),
        }
    }

    #[test]
    fn test_performance_many_assessments() {
        // Ensure a large batch of assessments completes deterministically.
        let undertaking = DominantUndertaking::new(Undertaking::new("BigCo"), "Cement", 70);
        let mut infringements = 0usize;
        for i in 0..1000 {
            let agreement = AntiCompetitiveAgreement::new(
                format!("agr-{i}"),
                two_parties(),
                AntiCompetitiveConduct::PriceFixing,
            );
            if assess_section_34(&agreement).is_err() {
                infringements += 1;
            }

            let claim = AbuseOfDominance::new(
                format!("c-{i}"),
                undertaking.clone(),
                AbuseType::PredatoryPricing,
            );
            if assess_section_47(&claim).is_err() {
                infringements += 1;
            }
        }
        // Every iteration produces two infringements.
        assert_eq!(infringements, 2000);
    }
}
