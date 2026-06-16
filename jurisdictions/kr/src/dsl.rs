//! `legalis-dsl` integration for Korean law (대한민국 법률).
//!
//! Renders every statute modelled by the `create_*_statute` builders in
//! [`crate`] as `legalis-dsl` source text, so the Civil Code / Criminal Code /
//! labour / tax / competition statutes can be inspected, formatted, diffed, and
//! consumed by the DSL tooling (LSP, documentation generation, structural
//! diffing).
//!
//! legalis-dsl 연계. 各 create_*_statute 빌더가 생성하는 Statute 를 DSL 소스
//! 텍스트로 출력한다.

use crate::{
    create_civil_code_statute, create_commercial_code_statute, create_corporate_tax_statute,
    create_criminal_code_statute, create_employment_insurance_statute, create_fair_trade_statute,
    create_income_tax_statute, create_labor_standards_statute, create_pipa_statute,
    create_vat_statute, create_workers_compensation_statute,
};
use legalis_core::Statute;

/// Collects every statute modelled by the Korean jurisdiction crate.
///
/// Each entry is produced by one of the `create_*_statute` builders defined at
/// the crate root and carries the `KR` jurisdiction tag.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        create_civil_code_statute(),
        create_criminal_code_statute(),
        create_commercial_code_statute(),
        create_labor_standards_statute(),
        create_pipa_statute(),
        create_employment_insurance_statute(),
        create_workers_compensation_statute(),
        create_income_tax_statute(),
        create_corporate_tax_statute(),
        create_vat_statute(),
        create_fair_trade_statute(),
    ]
}

/// Renders every modelled Korean statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "KR must model at least one statute");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one.
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
