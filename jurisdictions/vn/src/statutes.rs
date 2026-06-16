//! `Statute`-based models of major Vietnamese laws (Pháp luật Việt Nam).
//!
//! This module lifts the most significant provisions of the Vietnamese legal
//! system into the jurisdiction-agnostic [`legalis_core::Statute`] abstraction,
//! so they can be reasoned over, queried, conflict-checked, and — via
//! [`statutes_as_dsl`] — rendered as `legalis-dsl` source text for inspection,
//! diffing, and tooling (LSP, documentation generation).
//!
//! Each builder is grounded in a real, named Vietnamese law (with its
//! `số .../năm/QH...` citation and the relevant Điều / Article), mirroring the
//! substantive modules already implemented in this crate ([`crate::civil_code`],
//! [`crate::enterprise`], [`crate::labor_code`], [`crate::investment`],
//! [`crate::competition_law`], [`crate::cybersecurity_law`], [`crate::land_law`],
//! [`crate::tax_law`]).
//!
//! All statutes are tagged with jurisdiction code `"VN"`.
//!
//! # Examples
//!
//! ```
//! use legalis_vn::statutes::{all_statutes, statutes_as_dsl};
//!
//! let statutes = all_statutes();
//! assert_eq!(statutes.len(), 8);
//!
//! let dsl = statutes_as_dsl();
//! assert!(dsl.contains("VN-CIVIL-2015"));
//! ```

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Bộ luật Dân sự 2015 — Civil Code 2015 (Law No. 91/2015/QH13), Điều 20.
///
/// Article 20 grants a natural person who has reached full eighteen (18) years
/// of age **full civil act capacity** (năng lực hành vi dân sự đầy đủ), enabling
/// them to establish and perform civil transactions on their own, unless they
/// lose or have restricted capacity under Articles 22–24.
#[must_use]
pub fn civil_code_capacity_statute() -> Statute {
    Statute::new(
        "VN-CIVIL-2015-A20",
        "Full Civil Act Capacity at 18 (Bộ luật Dân sự 2015, Điều 20)",
        Effect::new(
            EffectType::Grant,
            "Natural person of 18 years or older has full civil act capacity to \
             enter into and perform civil transactions independently",
        )
        .with_parameter("law_no", "91/2015/QH13")
        .with_parameter("article", "20"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("VN")
}

/// Luật Doanh nghiệp 2020 — Law on Enterprises 2020 (Law No. 59/2020/QH14),
/// Điều 8 & Điều 26.
///
/// Enterprises must lawfully register their business and obtain an Enterprise
/// Registration Certificate (Giấy chứng nhận đăng ký doanh nghiệp) from the
/// business registration authority before commencing operations. Article 8
/// imposes the general obligation to register and to operate within the
/// registered business lines; Article 26 governs the registration procedure.
#[must_use]
pub fn enterprise_registration_statute() -> Statute {
    Statute::new(
        "VN-ENTERPRISE-2020-A26",
        "Mandatory Enterprise Registration (Luật Doanh nghiệp 2020, Điều 8, 26)",
        Effect::new(
            EffectType::Obligation,
            "An enterprise must obtain an Enterprise Registration Certificate \
             before commencing business and must operate within its registered \
             business lines",
        )
        .with_parameter("law_no", "59/2020/QH14")
        .with_parameter("article", "8, 26"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "carries_on_business".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Bộ luật Lao động 2019 — Labor Code 2019 (Law No. 45/2019/QH14), Điều 105.
///
/// Article 105 caps **normal working hours** at 8 hours per day and 48 hours per
/// week. Employers are prohibited from scheduling normal working time beyond
/// this weekly ceiling; hours worked above it are governed by the separate
/// overtime regime (Điều 107, max 40 hours overtime/month, 200–300 hours/year).
#[must_use]
pub fn labor_working_hours_statute() -> Statute {
    Statute::new(
        "VN-LABOR-2019-A105",
        "Maximum Normal Working Hours 48/week (Bộ luật Lao động 2019, Điều 105)",
        Effect::new(
            EffectType::Prohibition,
            "Normal working time must not exceed 8 hours per day and 48 hours \
             per week; excess is permitted only under the overtime regime",
        )
        .with_parameter("law_no", "45/2019/QH14")
        .with_parameter("article", "105")
        .with_parameter("max_hours_per_day", "8")
        .with_parameter("max_hours_per_week", "48"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "is_normal_working_time".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Luật Đầu tư 2020 — Law on Investment 2020 (Law No. 61/2020/QH14), Điều 37.
///
/// Article 37 requires a foreign investor to obtain an **Investment Registration
/// Certificate** (Giấy chứng nhận đăng ký đầu tư, IRC) for an investment project
/// before implementation. Domestic investors are generally exempt from the IRC
/// requirement (Điều 37.2), so the obligation is conditioned on foreign-investor
/// status.
#[must_use]
pub fn investment_irc_statute() -> Statute {
    Statute::new(
        "VN-INVEST-2020-A37",
        "Investment Registration Certificate for Foreign Investors \
         (Luật Đầu tư 2020, Điều 37)",
        Effect::new(
            EffectType::Obligation,
            "A foreign investor must obtain an Investment Registration \
             Certificate (IRC) for an investment project before implementation",
        )
        .with_parameter("law_no", "61/2020/QH14")
        .with_parameter("article", "37"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "investor_type".to_string(),
        value: "foreign".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Luật Cạnh tranh 2018 — Law on Competition 2018 (Law No. 23/2018/QH14),
/// Điều 33 (thresholds in Điều 31, Nghị định 35/2020/NĐ-CP).
///
/// Economic concentrations (mergers, acquisitions, consolidations) that reach a
/// notification threshold — e.g. combined total assets or combined turnover on
/// the Vietnamese market of **VND 3,000 billion or more**, or a combined market
/// share of **20% or more** on the relevant market — must be **notified** to the
/// National Competition Commission before completion.
#[must_use]
pub fn competition_merger_notification_statute() -> Statute {
    Statute::new(
        "VN-COMPETITION-2018-A33",
        "Economic Concentration Notification Threshold \
         (Luật Cạnh tranh 2018, Điều 33)",
        Effect::new(
            EffectType::Obligation,
            "Parties to an economic concentration must notify the National \
             Competition Commission before completion where combined assets or \
             turnover reach VND 3,000 billion or combined market share reaches 20%",
        )
        .with_parameter("law_no", "23/2018/QH14")
        .with_parameter("article", "33")
        .with_parameter("asset_threshold_vnd", "3000000000000")
        .with_parameter("market_share_threshold_pct", "20"),
    )
    .with_precondition(Condition::Percentage {
        operator: ComparisonOp::GreaterOrEqual,
        value: 20,
        context: "combined_market_share".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Luật An ninh mạng 2018 — Law on Cybersecurity 2018 (Law No. 24/2018/QH14),
/// Điều 26.
///
/// Article 26 obliges domestic and foreign enterprises providing services on
/// telecom networks or the internet in Vietnam that **collect, exploit, analyse
/// or process** personal data, data on user relationships, or data generated by
/// service users in Vietnam to **store that data within Vietnam** (data
/// localisation) and, for foreign enterprises, to establish a branch or
/// representative office in Vietnam.
#[must_use]
pub fn cybersecurity_data_localization_statute() -> Statute {
    Statute::new(
        "VN-CYBER-2018-A26",
        "Data Localisation Requirement (Luật An ninh mạng 2018, Điều 26)",
        Effect::new(
            EffectType::Obligation,
            "Service providers processing personal or user data of users in \
             Vietnam must store such data within Vietnam, and foreign providers \
             must establish a local branch or representative office",
        )
        .with_parameter("law_no", "24/2018/QH14")
        .with_parameter("article", "26"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "processes_vietnam_user_data".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Luật Đất đai 2024 — Land Law 2024 (Law No. 31/2024/QH15), Điều 171–172.
///
/// All land is owned by the entire people with the State as representative
/// owner; users hold **land use rights**. Residential land allocated to
/// households and individuals is granted with **stable, long-term use**
/// (sử dụng đất ổn định lâu dài), whereas most other allocated/leased
/// non-agricultural and agricultural land is subject to a definite term
/// (commonly up to 50 years; up to 70 years for certain projects).
#[must_use]
pub fn land_use_right_statute() -> Statute {
    Statute::new(
        "VN-LAND-2024-A172",
        "Stable Long-Term Residential Land Use Rights (Luật Đất đai 2024, Điều 171-172)",
        Effect::new(
            EffectType::Grant,
            "Residential land allocated to households and individuals is granted \
             with stable, long-term land use rights; other allocated land is \
             generally subject to a definite term of up to 50-70 years",
        )
        .with_parameter("law_no", "31/2024/QH15")
        .with_parameter("article", "171-172")
        .with_parameter("max_definite_term_years", "70"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "land_category".to_string(),
        value: "residential".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Luật Thuế giá trị gia tăng — Law on Value Added Tax (Law No. 13/2008/QH12,
/// amended by Laws 31/2013/QH13 and 106/2016/QH13), Điều 8.
///
/// Article 8 sets the **standard VAT rate at 10%** on most goods and services
/// (with 0% for exports/international transport and 5% for listed essential
/// goods). VAT is a monetary transfer collected on the value added at each
/// stage of production and circulation.
#[must_use]
pub fn vat_standard_rate_statute() -> Statute {
    Statute::new(
        "VN-VAT-2008-A8",
        "Standard Value Added Tax Rate 10% (Luật Thuế GTGT, Điều 8)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Value added tax is levied at the standard rate of 10% on most \
             taxable goods and services (0% for exports, 5% for listed \
             essential goods)",
        )
        .with_parameter("law_no", "13/2008/QH12")
        .with_parameter("article", "8")
        .with_parameter("standard_rate_pct", "10"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "supply_category".to_string(),
        value: "standard_rated".to_string(),
    })
    .with_jurisdiction("VN")
}

/// Returns every modelled major Vietnamese statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_code_capacity_statute(),
        enterprise_registration_statute(),
        labor_working_hours_statute(),
        investment_irc_statute(),
        competition_merger_notification_statute(),
        cybersecurity_data_localization_statute(),
        land_use_right_statute(),
        vat_standard_rate_statute(),
    ]
}

/// Renders every modelled Vietnamese statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`], suitable for inspection, diffing, and
/// consumption by the DSL tooling.
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
        assert!(!statutes.is_empty(), "VN must model at least one statute");
        assert_eq!(statutes.len(), 8, "VN must model exactly 8 major statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one (covers the full range of condition
        // kinds the VN statutes use: Age, AttributeEquals, Percentage).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
