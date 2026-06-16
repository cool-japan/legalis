//! Securities and Futures Act 2001 - Assessment Logic
//!
//! Assessment functions that apply the Securities and Futures Act 2001 to the
//! typed models in [`super::types`], [`super::offerings`] and
//! [`super::misconduct`]:
//!
//! - [`assess_prospectus_requirement`] - Part 13 offers and exemptions.
//! - [`assess_insider_trading`], [`assess_false_trading`],
//!   [`assess_market_manipulation`], [`assess_misleading_statement`],
//!   [`assess_fraudulent_inducement`] - Part 12 market conduct.
//! - [`assess_licensing`], [`assess_representative`] - Part 4 licensing.
//! - [`assess_collective_investment_scheme`] - CIS authorisation/recognition.
//! - [`compute_civil_penalty_cents`] - s. 232 civil penalty after the cap.
//!
//! Each `assess_*` function returns `Ok(())` where no contravention is made out,
//! and a [`SecuritiesError`] flagging the relevant provision otherwise.

use super::error::{Result, SecuritiesError};
use super::misconduct::{
    FalseTradingClaim, FraudulentInducementClaim, InsiderTradingClaim, MarketManipulationClaim,
    MisleadingStatementClaim, max_civil_penalty_cents,
};
use super::offerings::{OfferingExemption, SecuritiesOffering};
use super::types::{
    AppointedRepresentative, CapitalMarketsServicesLicence, CollectiveInvestmentScheme,
    RegulatedActivity,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Part 13 - offers and prospectus
// ============================================================================

/// Outcome of a Part 13 offering assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfferingReport {
    /// Identifier of the offering assessed.
    pub offering_id: String,
    /// Whether a registered prospectus is required for the offer to proceed.
    pub prospectus_required: bool,
    /// The exemption claimed, if any.
    pub exemption: Option<OfferingExemption>,
    /// Whether the claimed exemption is made out on the recorded facts.
    pub exemption_made_out: bool,
    /// Whether the offering, as recorded, may lawfully proceed.
    pub compliant: bool,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses an offer of securities against the Part 13 prospectus regime.
///
/// Logic:
/// 1. Products that do not engage the prospectus regime (pure derivatives,
///    leveraged spot FX) are not caught - returns `Ok`.
/// 2. A valid exemption (small offer s. 272A, private placement s. 272B,
///    institutional s. 274, accredited s. 275) takes the offer outside the
///    requirement - returns `Ok`.
/// 3. Otherwise a prospectus is required: a defective prospectus yields
///    [`SecuritiesError::DefectiveProspectus`]; an unregistered prospectus
///    yields [`SecuritiesError::ProspectusNotRegistered`]; the absence of any
///    prospectus yields [`SecuritiesError::ProspectusRequired`].
pub fn assess_prospectus_requirement(offering: &SecuritiesOffering) -> Result<()> {
    if !offering.product.engages_prospectus_regime() {
        return Ok(());
    }

    if offering.exemption.is_some() && offering.exemption_made_out() {
        return Ok(());
    }

    match &offering.prospectus {
        Some(prospectus) if prospectus.is_defective() => {
            Err(SecuritiesError::DefectiveProspectus {
                detail: "false/misleading statement or omission in the registered prospectus"
                    .to_string(),
            })
        }
        Some(prospectus) if !prospectus.registered_with_mas => {
            Err(SecuritiesError::ProspectusNotRegistered)
        }
        Some(_) => Ok(()),
        None => Err(SecuritiesError::ProspectusRequired {
            product: offering.product.description().to_string(),
            reason: match offering.exemption {
                Some(exemption) => format!(
                    "claimed exemption not made out ({})",
                    exemption.statute_reference()
                ),
                None => "public offer with no applicable exemption".to_string(),
            },
        }),
    }
}

/// Produces a detailed Part 13 offering report (never returns `Err` for a
/// contravention; instead records it in the report).
pub fn assess_offering_report(offering: &SecuritiesOffering) -> OfferingReport {
    let mut notes = Vec::new();
    let engages = offering.product.engages_prospectus_regime();
    let exemption_made_out = offering.exemption_made_out();

    if !engages {
        notes.push(format!(
            "{} does not engage the Part 13 prospectus regime",
            offering.product.description()
        ));
    }

    if let Some(exemption) = offering.exemption {
        if exemption_made_out {
            notes.push(format!(
                "Exemption made out: {} ({})",
                exemption.description(),
                exemption.statute_reference()
            ));
        } else {
            notes.push(format!(
                "Exemption claimed but NOT made out: {} ({})",
                exemption.description(),
                exemption.statute_reference()
            ));
        }
    }

    let result = assess_prospectus_requirement(offering);
    let compliant = result.is_ok();
    let prospectus_required = engages && !(offering.exemption.is_some() && exemption_made_out);

    if let Err(err) = &result {
        notes.push(err.to_string());
    } else if compliant && prospectus_required {
        notes.push("Registered prospectus present (SFA s. 240/s. 246)".to_string());
    }

    OfferingReport {
        offering_id: offering.offering_id.clone(),
        prospectus_required,
        exemption: offering.exemption,
        exemption_made_out,
        compliant,
        notes,
    }
}

// ============================================================================
// Part 12 - market conduct
// ============================================================================

/// Assesses a claim of insider trading (SFA s. 218/s. 219).
///
/// # Errors
///
/// Returns [`SecuritiesError::InsiderTrading`] where the elements are made out.
pub fn assess_insider_trading(claim: &InsiderTradingClaim) -> Result<()> {
    if claim.is_made_out() {
        return Err(SecuritiesError::InsiderTrading {
            section: claim.applicable_section().to_string(),
            detail: claim.conduct.description().to_string(),
        });
    }
    Ok(())
}

/// Assesses a claim of false trading or market rigging (SFA s. 197).
///
/// # Errors
///
/// Returns [`SecuritiesError::FalseTrading`] where the conduct is made out.
pub fn assess_false_trading(claim: &FalseTradingClaim) -> Result<()> {
    if claim.is_made_out() {
        let detail = if claim.wash_trade_or_matched_orders {
            "wash trades / matched orders creating a false appearance of active trading"
        } else if claim.false_appearance_of_market_or_price {
            "false or misleading appearance as to the market or price"
        } else {
            "false or misleading appearance of active trading"
        };
        return Err(SecuritiesError::FalseTrading {
            detail: detail.to_string(),
        });
    }
    Ok(())
}

/// Assesses a claim of employing a manipulative or deceptive device (SFA s. 201).
///
/// # Errors
///
/// Returns [`SecuritiesError::MarketManipulation`] where the conduct is made out.
pub fn assess_market_manipulation(claim: &MarketManipulationClaim) -> Result<()> {
    if claim.is_made_out() {
        return Err(SecuritiesError::MarketManipulation {
            detail: "manipulative or deceptive device in connection with capital markets products"
                .to_string(),
        });
    }
    Ok(())
}

/// Assesses a claim of making a false or misleading statement (SFA s. 199).
///
/// # Errors
///
/// Returns [`SecuritiesError::MisleadingStatement`] where the conduct is made out.
pub fn assess_misleading_statement(claim: &MisleadingStatementClaim) -> Result<()> {
    if claim.is_made_out() {
        return Err(SecuritiesError::MisleadingStatement {
            detail: "false or misleading statement likely to induce dealing or affect price"
                .to_string(),
        });
    }
    Ok(())
}

/// Assesses a claim of fraudulent inducement to deal (SFA s. 200).
///
/// # Errors
///
/// Returns [`SecuritiesError::FraudulentInducement`] where the conduct is made
/// out.
pub fn assess_fraudulent_inducement(claim: &FraudulentInducementClaim) -> Result<()> {
    if claim.is_made_out() {
        return Err(SecuritiesError::FraudulentInducement {
            detail: "dishonest or fraudulent means inducing another person to deal".to_string(),
        });
    }
    Ok(())
}

/// A consolidated report on possible market misconduct (Part 12).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketConductReport {
    /// Whether any market abuse contravention was found.
    pub is_market_abuse: bool,
    /// The contraventions found (as display strings).
    pub contraventions: Vec<String>,
}

impl MarketConductReport {
    /// Whether the conduct is clean (no contraventions found).
    pub fn is_clean(&self) -> bool {
        self.contraventions.is_empty()
    }
}

/// Builds a [`MarketConductReport`] from the optional Part 12 claims supplied.
///
/// Each claim that is made out contributes a contravention entry; the report
/// records whether any market abuse was found without short-circuiting.
pub fn assess_market_conduct(
    insider: Option<&InsiderTradingClaim>,
    false_trading: Option<&FalseTradingClaim>,
    manipulation: Option<&MarketManipulationClaim>,
    statement: Option<&MisleadingStatementClaim>,
    inducement: Option<&FraudulentInducementClaim>,
) -> MarketConductReport {
    let mut contraventions = Vec::new();

    if let Some(c) = insider
        && let Err(e) = assess_insider_trading(c)
    {
        contraventions.push(e.to_string());
    }
    if let Some(c) = false_trading
        && let Err(e) = assess_false_trading(c)
    {
        contraventions.push(e.to_string());
    }
    if let Some(c) = manipulation
        && let Err(e) = assess_market_manipulation(c)
    {
        contraventions.push(e.to_string());
    }
    if let Some(c) = statement
        && let Err(e) = assess_misleading_statement(c)
    {
        contraventions.push(e.to_string());
    }
    if let Some(c) = inducement
        && let Err(e) = assess_fraudulent_inducement(c)
    {
        contraventions.push(e.to_string());
    }

    MarketConductReport {
        is_market_abuse: !contraventions.is_empty(),
        contraventions,
    }
}

// ============================================================================
// Part 4 - licensing
// ============================================================================

/// Assesses whether a person may carry on a regulated activity (SFA s. 82).
///
/// # Errors
///
/// Returns [`SecuritiesError::UnlicensedRegulatedActivity`] where no licence
/// (or applicable exemption) authorises the activity.
pub fn assess_licensing(
    licence: Option<&CapitalMarketsServicesLicence>,
    activity: RegulatedActivity,
) -> Result<()> {
    match licence {
        Some(l) if l.authorises(activity) => Ok(()),
        _ => Err(SecuritiesError::UnlicensedRegulatedActivity {
            activity: activity.description().to_string(),
        }),
    }
}

/// Assesses whether a person may act as a representative for a regulated activity
/// (SFA s. 99B).
///
/// # Errors
///
/// Returns [`SecuritiesError::UnauthorisedRepresentative`] where the
/// representative is not on the MAS public register, or is not appointed for the
/// activity.
pub fn assess_representative(
    representative: &AppointedRepresentative,
    activity: RegulatedActivity,
) -> Result<()> {
    if representative.may_act(activity) {
        Ok(())
    } else {
        Err(SecuritiesError::UnauthorisedRepresentative {
            name: representative.name.clone(),
        })
    }
}

/// Assesses whether a collective investment scheme may be offered as recorded.
///
/// # Errors
///
/// Returns [`SecuritiesError::SchemeNotAuthorised`] where the scheme is offered
/// to the retail public but is neither authorised (s. 286) nor recognised
/// (s. 287).
pub fn assess_collective_investment_scheme(
    scheme: &CollectiveInvestmentScheme,
    offered_to_public: bool,
) -> Result<()> {
    if offered_to_public && !scheme.may_offer_to_public() {
        return Err(SecuritiesError::SchemeNotAuthorised {
            scheme: scheme.name.clone(),
        });
    }
    Ok(())
}

// ============================================================================
// Enforcement - civil penalty (SFA s. 232)
// ============================================================================

/// Validates a proposed civil penalty against the statutory cap (SFA s. 232).
///
/// The cap is computed via [`max_civil_penalty_cents`]. Returns the proposed
/// penalty if it is within the cap.
///
/// # Errors
///
/// Returns [`SecuritiesError::CivilPenaltyExceedsCap`] if `proposed_penalty_cents`
/// exceeds the statutory maximum.
pub fn compute_civil_penalty_cents(
    profit_or_loss_avoided_cents: u64,
    is_individual: bool,
    proposed_penalty_cents: u64,
) -> Result<u64> {
    let cap = max_civil_penalty_cents(profit_or_loss_avoided_cents, is_individual);
    if proposed_penalty_cents > cap {
        return Err(SecuritiesError::CivilPenaltyExceedsCap {
            proposed_cents: proposed_penalty_cents,
            maximum_cents: cap,
        });
    }
    Ok(proposed_penalty_cents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::securities::misconduct::InsiderConduct;
    use crate::securities::offerings::Prospectus;
    use crate::securities::types::{
        CapitalMarketsProduct, CisAuthorisationStatus, CmsLicenceStatus, InvestorClass,
    };

    // ---- Part 13 -----------------------------------------------------------

    #[test]
    fn test_public_offer_without_prospectus_requires_one() {
        let offering =
            SecuritiesOffering::new("o-1", CapitalMarketsProduct::Securities, 1_000_000_000);
        match assess_prospectus_requirement(&offering) {
            Err(SecuritiesError::ProspectusRequired { .. }) => {}
            other => panic!("expected ProspectusRequired, got {other:?}"),
        }
    }

    #[test]
    fn test_public_offer_with_registered_prospectus_is_ok() {
        let offering =
            SecuritiesOffering::new("o-2", CapitalMarketsProduct::Securities, 1_000_000_000)
                .with_prospectus(Prospectus::registered());
        assert!(assess_prospectus_requirement(&offering).is_ok());
    }

    #[test]
    fn test_unregistered_prospectus_is_rejected() {
        let offering =
            SecuritiesOffering::new("o-3", CapitalMarketsProduct::Securities, 1_000_000_000)
                .with_prospectus(Prospectus::unregistered());
        assert_eq!(
            assess_prospectus_requirement(&offering),
            Err(SecuritiesError::ProspectusNotRegistered)
        );
    }

    #[test]
    fn test_defective_prospectus_is_rejected() {
        let offering =
            SecuritiesOffering::new("o-4", CapitalMarketsProduct::Securities, 1_000_000_000)
                .with_prospectus(Prospectus::registered().with_false_statement());
        match assess_prospectus_requirement(&offering) {
            Err(SecuritiesError::DefectiveProspectus { .. }) => {}
            other => panic!("expected DefectiveProspectus, got {other:?}"),
        }
    }

    #[test]
    fn test_small_offer_exemption_dispenses_with_prospectus() {
        let offering =
            SecuritiesOffering::new("o-5", CapitalMarketsProduct::Securities, 300_000_000)
                .with_exemption(OfferingExemption::SmallOffer);
        assert!(assess_prospectus_requirement(&offering).is_ok());
    }

    #[test]
    fn test_failed_exemption_still_requires_prospectus() {
        // Small-offer exemption claimed but the amount exceeds SGD 5m.
        let offering =
            SecuritiesOffering::new("o-6", CapitalMarketsProduct::Securities, 600_000_000)
                .with_exemption(OfferingExemption::SmallOffer);
        match assess_prospectus_requirement(&offering) {
            Err(SecuritiesError::ProspectusRequired { reason, .. }) => {
                assert!(reason.contains("272A"));
            }
            other => panic!("expected ProspectusRequired, got {other:?}"),
        }
    }

    #[test]
    fn test_derivatives_do_not_require_prospectus() {
        let offering = SecuritiesOffering::new(
            "o-7",
            CapitalMarketsProduct::DerivativesContract,
            900_000_000,
        );
        assert!(assess_prospectus_requirement(&offering).is_ok());
    }

    #[test]
    fn test_offering_report_records_exemption() {
        let offering =
            SecuritiesOffering::new("o-8", CapitalMarketsProduct::Securities, 100_000_000)
                .with_offeree_class(InvestorClass::Accredited)
                .with_exemption(OfferingExemption::AccreditedInvestors);
        let report = assess_offering_report(&offering);
        assert!(report.compliant);
        assert!(report.exemption_made_out);
        assert!(!report.prospectus_required);
        assert!(report.notes.iter().any(|n| n.contains("275")));
    }

    // ---- Part 12 -----------------------------------------------------------

    #[test]
    fn test_insider_trading_made_out() {
        let claim = InsiderTradingClaim::new("it-1", true);
        match assess_insider_trading(&claim) {
            Err(SecuritiesError::InsiderTrading { section, .. }) => assert_eq!(section, "s. 218"),
            other => panic!("expected InsiderTrading, got {other:?}"),
        }
    }

    #[test]
    fn test_insider_trading_public_information_is_ok() {
        let claim = InsiderTradingClaim::new("it-2", true).generally_available();
        assert!(assess_insider_trading(&claim).is_ok());
    }

    #[test]
    fn test_false_trading_and_manipulation() {
        assert!(assess_false_trading(&FalseTradingClaim::new("ft-1").with_wash_trade()).is_err());
        assert!(assess_market_manipulation(&MarketManipulationClaim::new("mm-1")).is_err());
    }

    #[test]
    fn test_market_conduct_report_collects_contraventions() {
        let insider = InsiderTradingClaim::new("it-3", false).with_conduct(InsiderConduct::Dealt);
        let false_trade = FalseTradingClaim::new("ft-2").with_wash_trade();
        let report = assess_market_conduct(Some(&insider), Some(&false_trade), None, None, None);
        assert!(report.is_market_abuse);
        assert!(!report.is_clean());
        assert_eq!(report.contraventions.len(), 2);
    }

    #[test]
    fn test_market_conduct_report_clean() {
        let insider = InsiderTradingClaim::new("it-4", true).generally_available();
        let report = assess_market_conduct(Some(&insider), None, None, None, None);
        assert!(!report.is_market_abuse);
        assert!(report.is_clean());
    }

    // ---- Part 4 ------------------------------------------------------------

    #[test]
    fn test_licensing_requires_authorisation() {
        let licence = CapitalMarketsServicesLicence::new(
            "Gamma Pte Ltd",
            vec![RegulatedActivity::FundManagement],
        );
        assert!(assess_licensing(Some(&licence), RegulatedActivity::FundManagement).is_ok());
        match assess_licensing(Some(&licence), RegulatedActivity::ProductFinancing) {
            Err(SecuritiesError::UnlicensedRegulatedActivity { .. }) => {}
            other => panic!("expected UnlicensedRegulatedActivity, got {other:?}"),
        }
    }

    #[test]
    fn test_licensing_none_is_unlicensed() {
        match assess_licensing(None, RegulatedActivity::DealingInCapitalMarketsProducts) {
            Err(SecuritiesError::UnlicensedRegulatedActivity { .. }) => {}
            other => panic!("expected UnlicensedRegulatedActivity, got {other:?}"),
        }
    }

    #[test]
    fn test_suspended_licence_does_not_authorise() {
        let licence = CapitalMarketsServicesLicence::new(
            "Delta Pte Ltd",
            vec![RegulatedActivity::FundManagement],
        )
        .with_status(CmsLicenceStatus::Suspended);
        assert!(assess_licensing(Some(&licence), RegulatedActivity::FundManagement).is_err());
    }

    #[test]
    fn test_representative_off_register_is_unauthorised() {
        let rep = AppointedRepresentative::new(
            "John Lee",
            "Gamma Pte Ltd",
            vec![RegulatedActivity::FundManagement],
        )
        .not_on_register();
        match assess_representative(&rep, RegulatedActivity::FundManagement) {
            Err(SecuritiesError::UnauthorisedRepresentative { name }) => {
                assert_eq!(name, "John Lee")
            }
            other => panic!("expected UnauthorisedRepresentative, got {other:?}"),
        }
    }

    #[test]
    fn test_cis_offer_to_public_requires_authorisation() {
        let restricted = CollectiveInvestmentScheme::new(
            "Hedge Fund LP",
            false,
            CisAuthorisationStatus::Restricted,
        );
        match assess_collective_investment_scheme(&restricted, true) {
            Err(SecuritiesError::SchemeNotAuthorised { .. }) => {}
            other => panic!("expected SchemeNotAuthorised, got {other:?}"),
        }
        // The same scheme offered privately (not to the public) is fine.
        assert!(assess_collective_investment_scheme(&restricted, false).is_ok());
    }

    #[test]
    fn test_cis_authorised_scheme_is_ok() {
        let authorised = CollectiveInvestmentScheme::new(
            "SG Bond Fund",
            true,
            CisAuthorisationStatus::Authorised,
        );
        assert!(assess_collective_investment_scheme(&authorised, true).is_ok());
    }

    // ---- Enforcement -------------------------------------------------------

    #[test]
    fn test_civil_penalty_within_cap() {
        // Individual, profit SGD 1m -> cap SGD 3m. Propose SGD 2m -> ok.
        let payable =
            compute_civil_penalty_cents(100_000_000, true, 200_000_000).expect("within cap");
        assert_eq!(payable, 200_000_000);
    }

    #[test]
    fn test_civil_penalty_exceeds_cap() {
        // Individual, profit SGD 100k -> cap SGD 300k. Propose SGD 1m -> error.
        match compute_civil_penalty_cents(10_000_000, true, 100_000_000) {
            Err(SecuritiesError::CivilPenaltyExceedsCap {
                proposed_cents,
                maximum_cents,
            }) => {
                assert_eq!(proposed_cents, 100_000_000);
                assert_eq!(maximum_cents, 30_000_000);
            }
            other => panic!("expected CivilPenaltyExceedsCap, got {other:?}"),
        }
    }

    #[test]
    fn test_performance_many_assessments() {
        // A large batch of assessments completes deterministically and cheaply.
        let mut contraventions = 0usize;
        for i in 0..1000 {
            let offering = SecuritiesOffering::new(
                format!("o-{i}"),
                CapitalMarketsProduct::Securities,
                1_000_000_000,
            );
            if assess_prospectus_requirement(&offering).is_err() {
                contraventions += 1;
            }
            let claim = InsiderTradingClaim::new(format!("it-{i}"), true);
            if assess_insider_trading(&claim).is_err() {
                contraventions += 1;
            }
        }
        assert_eq!(contraventions, 2000);
    }
}
