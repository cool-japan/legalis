//! `Statute`-based models of major Indian legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-in` crate (contract, companies, data protection, information
//! technology, competition, insolvency, arbitration and goods-and-services tax
//! law) into the jurisdiction-neutral [`legalis_core::Statute`] abstraction.
//! Each builder encodes a *real* statutory provision — accurate Act name, year
//! and operative rule — as an [`Effect`] with a meaningful [`Condition`]
//! precondition where the underlying law turns on a quantifiable trigger (a
//! duration, a monetary threshold expressed as an income/turnover figure, or a
//! status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Indian rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Act |
//! |---------|-----|
//! | [`indian_contract_act_statute`] | Indian Contract Act, 1872, s.10 |
//! | [`companies_act_csr_statute`] | Companies Act, 2013, s.135 |
//! | [`dpdp_act_statute`] | Digital Personal Data Protection Act, 2023, s.8(6) |
//! | [`it_act_statute`] | Information Technology Act, 2000, s.43A |
//! | [`competition_act_statute`] | Competition Act, 2002, s.3 |
//! | [`insolvency_code_statute`] | Insolvency and Bankruptcy Code, 2016, s.7 |
//! | [`arbitration_act_statute`] | Arbitration and Conciliation Act, 1996, s.29A |
//! | [`cgst_act_statute`] | Central Goods and Services Tax Act, 2017, s.22 |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Indian advocate.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Indian Contract Act, 1872, s.10 — what agreements are contracts.
///
/// Section 10 provides that all agreements are contracts if they are made by
/// the free consent of parties competent to contract, for a lawful
/// consideration and with a lawful object, and are not expressly declared to be
/// void. The provision is the gateway requirement that confers enforceability
/// on an agreement, so it is modelled as a Grant of contractual enforceability
/// conditioned on the agreement satisfying the essential validity requirements.
///
/// Real source: Indian Contract Act, 1872 (Act 9 of 1872), s.10.
#[must_use]
pub fn indian_contract_act_statute() -> Statute {
    Statute::new(
        "IN-CONTRACT-1872-S10",
        "Agreements That Are Contracts (Indian Contract Act, 1872, s.10)",
        Effect::new(
            EffectType::Grant,
            "An agreement made by the free consent of parties competent to contract, for \
             a lawful consideration and with a lawful object, is an enforceable contract",
        )
        .with_parameter("act_number", "9")
        .with_parameter("act_year", "1872")
        .with_parameter("section", "10"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "free_consent_lawful_consideration_and_object".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("IN")
}

/// Companies Act, 2013, s.135 — Corporate Social Responsibility obligation.
///
/// Section 135 requires every company having a net worth of Rs. 500 crore or
/// more, or a turnover of Rs. 1000 crore or more, or a net profit of Rs. 5 crore
/// or more during the immediately preceding financial year to constitute a CSR
/// Committee and spend, in every financial year, at least 2% of the average net
/// profits of the three immediately preceding financial years on CSR activities
/// listed in Schedule VII. Modelled as the spending obligation triggered once a
/// company's net profit crosses the Rs. 5 crore qualifying threshold.
///
/// Real source: Companies Act, 2013 (Act 18 of 2013), s.135(1) and (5).
#[must_use]
pub fn companies_act_csr_statute() -> Statute {
    Statute::new(
        "IN-COMPANIES-2013-S135",
        "Corporate Social Responsibility (Companies Act, 2013, s.135)",
        Effect::new(
            EffectType::Obligation,
            "A qualifying company must spend at least 2% of the average net profits of \
             the three preceding financial years on CSR activities under Schedule VII",
        )
        .with_parameter("act_number", "18")
        .with_parameter("act_year", "2013")
        .with_parameter("section", "135")
        .with_parameter("csr_spend_pct", "2")
        .with_parameter("net_profit_threshold_inr", "50000000"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 50_000_000, // Rs. 5 crore net profit
    })
    .with_jurisdiction("IN")
}

/// Digital Personal Data Protection Act, 2023, s.8(6) — personal data breach
/// notification.
///
/// Section 8(6) requires a Data Fiduciary, in the event of a personal data
/// breach, to give the Data Protection Board of India and each affected Data
/// Principal intimation of the breach in such form and manner as may be
/// prescribed. The duty is triggered on the occurrence of a breach and is
/// modelled as the resulting notification obligation.
///
/// Real source: Digital Personal Data Protection Act, 2023 (Act 22 of 2023),
/// s.8(6).
#[must_use]
pub fn dpdp_act_statute() -> Statute {
    Statute::new(
        "IN-DPDP-2023-S8",
        "Personal Data Breach Notification (DPDP Act, 2023, s.8(6))",
        Effect::new(
            EffectType::Obligation,
            "On a personal data breach, a Data Fiduciary must notify the Data Protection \
             Board of India and each affected Data Principal in the prescribed manner",
        )
        .with_parameter("act_number", "22")
        .with_parameter("act_year", "2023")
        .with_parameter("section", "8(6)"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_data_breach".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("IN")
}

/// Information Technology Act, 2000, s.43A — compensation for failure to protect
/// sensitive personal data.
///
/// Section 43A provides that where a body corporate possessing, dealing or
/// handling any sensitive personal data or information in a computer resource
/// which it owns, controls or operates is negligent in implementing and
/// maintaining reasonable security practices and procedures and thereby causes
/// wrongful loss or wrongful gain to any person, it is liable to pay damages by
/// way of compensation to the person so affected. Modelled as the obligation to
/// compensate, triggered by negligent handling of sensitive personal data.
///
/// Real source: Information Technology Act, 2000 (Act 21 of 2000), s.43A.
#[must_use]
pub fn it_act_statute() -> Statute {
    Statute::new(
        "IN-ITACT-2000-S43A",
        "Compensation for Failure to Protect Data (IT Act, 2000, s.43A)",
        Effect::new(
            EffectType::Obligation,
            "A body corporate negligent in maintaining reasonable security practices for \
             sensitive personal data must pay compensation to the affected person",
        )
        .with_parameter("act_number", "21")
        .with_parameter("act_year", "2000")
        .with_parameter("section", "43A"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "negligent_handling_of_sensitive_personal_data".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("IN")
}

/// Competition Act, 2002, s.3 — prohibition of anti-competitive agreements.
///
/// Section 3(1) prohibits any agreement in respect of production, supply,
/// distribution, storage, acquisition or control of goods or services which
/// causes or is likely to cause an appreciable adverse effect on competition
/// within India, and s.3(2) renders any such agreement void. Section 3(3)
/// presumes an appreciable adverse effect for horizontal agreements between
/// competitors involving price fixing, output limitation, market sharing or bid
/// rigging. Enforced by the Competition Commission of India.
///
/// Real source: Competition Act, 2002 (Act 12 of 2003), s.3.
#[must_use]
pub fn competition_act_statute() -> Statute {
    Statute::new(
        "IN-COMPETITION-2002-S3",
        "Anti-Competitive Agreements (Competition Act, 2002, s.3)",
        Effect::new(
            EffectType::Prohibition,
            "Agreements between competitors involving price fixing, output limitation, \
             market sharing or bid rigging are presumed to harm competition and are void",
        )
        .with_parameter("act_number", "12")
        .with_parameter("act_year", "2003")
        .with_parameter("section", "3(3)"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "horizontal_agreement_between_competitors".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("IN")
}

/// Insolvency and Bankruptcy Code, 2016, s.7 — initiation of the corporate
/// insolvency resolution process by a financial creditor.
///
/// Section 7 permits a financial creditor to initiate the corporate insolvency
/// resolution process (CIRP) against a corporate debtor before the National
/// Company Law Tribunal when a default has occurred. By the notification of the
/// Ministry of Corporate Affairs dated 24 March 2020, the minimum amount of
/// default for initiating CIRP is Rs. 1 crore. Admission of the application
/// triggers the moratorium under s.14, so the provision is modelled as a status
/// change placing the corporate debtor into CIRP once the default threshold is
/// met.
///
/// Real source: Insolvency and Bankruptcy Code, 2016 (Act 31 of 2016), s.7,
/// read with MCA Notification S.O. 1205(E) dated 24 March 2020.
#[must_use]
pub fn insolvency_code_statute() -> Statute {
    Statute::new(
        "IN-IBC-2016-S7",
        "Corporate Insolvency Resolution Process (IBC, 2016, s.7)",
        Effect::new(
            EffectType::StatusChange,
            "A financial creditor may initiate the corporate insolvency resolution \
             process against a corporate debtor where the default is at least Rs. 1 crore",
        )
        .with_parameter("act_number", "31")
        .with_parameter("act_year", "2016")
        .with_parameter("section", "7")
        .with_parameter("minimum_default_inr", "10000000"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 10_000_000, // Rs. 1 crore minimum default
    })
    .with_jurisdiction("IN")
}

/// Arbitration and Conciliation Act, 1996, s.29A — time limit for the arbitral
/// award.
///
/// Section 29A requires the award in matters other than international commercial
/// arbitration to be made by the arbitral tribunal within a period of twelve
/// months from the date of completion of pleadings under s.23(4). The parties
/// may by consent extend that period by a further six months, after which any
/// extension requires an order of the court. Modelled as the tribunal's
/// obligation to render the award within the twelve-month statutory window.
///
/// Real source: Arbitration and Conciliation Act, 1996 (Act 26 of 1996),
/// s.29A(1).
#[must_use]
pub fn arbitration_act_statute() -> Statute {
    Statute::new(
        "IN-ARBITRATION-1996-S29A",
        "Time Limit for Arbitral Award (Arbitration and Conciliation Act, 1996, s.29A)",
        Effect::new(
            EffectType::Obligation,
            "The arbitral tribunal must make its award within 12 months from the date of \
             completion of pleadings in arbitrations other than international ones",
        )
        .with_parameter("act_number", "26")
        .with_parameter("act_year", "1996")
        .with_parameter("section", "29A")
        .with_parameter("award_months", "12"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 12,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("IN")
}

/// Central Goods and Services Tax Act, 2017, s.22 — persons liable to register.
///
/// Section 22(1) makes every supplier liable to be registered under the Act in
/// the State or Union territory from which a taxable supply of goods or services
/// is made if the aggregate turnover in a financial year exceeds the prescribed
/// threshold. For suppliers of goods in normal-category States the threshold is
/// Rs. 40 lakh (Rs. 20 lakh for special-category States and for services).
/// Modelled as the compulsory registration obligation triggered when aggregate
/// turnover crosses the Rs. 40 lakh goods threshold.
///
/// Real source: Central Goods and Services Tax Act, 2017 (Act 12 of 2017),
/// s.22(1), read with Notification No. 10/2019-Central Tax.
#[must_use]
pub fn cgst_act_statute() -> Statute {
    Statute::new(
        "IN-CGST-2017-S22",
        "Compulsory GST Registration (CGST Act, 2017, s.22)",
        Effect::new(
            EffectType::Obligation,
            "A supplier of goods whose aggregate turnover in a financial year exceeds \
             Rs. 40 lakh in a normal-category State must register under the CGST Act",
        )
        .with_parameter("act_number", "12")
        .with_parameter("act_year", "2017")
        .with_parameter("section", "22")
        .with_parameter("goods_threshold_inr", "4000000"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 4_000_000, // Rs. 40 lakh aggregate turnover (goods)
    })
    .with_jurisdiction("IN")
}

/// Returns every modelled Indian statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        indian_contract_act_statute(),
        companies_act_csr_statute(),
        dpdp_act_statute(),
        it_act_statute(),
        competition_act_statute(),
        insolvency_code_statute(),
        arbitration_act_statute(),
        cgst_act_statute(),
    ]
}

/// Renders every modelled Indian statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_statutes();
        assert!(!statutes.is_empty(), "IN must model at least one statute");
        assert_eq!(statutes.len(), 8, "IN must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the IN
        // adapters use: Income, Duration, AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
