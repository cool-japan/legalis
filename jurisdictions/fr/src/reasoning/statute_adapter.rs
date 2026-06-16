//! French statute adapter — aggregates canonical article constructors from domain modules.
//!
//! This module is a thin aggregator that delegates to canonical article constructor
//! functions defined in each domain module (contract, labor, company, family).
//! It replaces the earlier ad-hoc inline definitions with direct calls to the
//! authoritative implementations.

use legalis_core::Statute;

/// Returns all French contract law statutes (6 articles).
///
/// Covers Code civil Articles 1103, 1104, 1128, 1217, 1218, and 1231.
#[must_use]
pub fn contract_law_statutes() -> Vec<Statute> {
    vec![
        crate::contract::article1103(),
        crate::contract::article1104(),
        crate::contract::article1128(),
        crate::contract::article1217(),
        crate::contract::article1218(),
        crate::contract::article1231(),
    ]
}

/// Returns all French labor law statutes (15 articles).
///
/// Covers Code du travail Articles L1221-1, L1221-19, L1242-2, L1242-8, L1242-12,
/// L1231-1, L1232-1, L1232-2, L1233-3, L1234-1, L3121-18, L3121-20, L3121-27,
/// L3121-33, and L3121-34.
#[must_use]
pub fn labor_law_statutes() -> Vec<Statute> {
    vec![
        crate::labor::article_l1221_1(),
        crate::labor::article_l1221_19(),
        crate::labor::article_l1242_2(),
        crate::labor::article_l1242_8(),
        crate::labor::article_l1242_12(),
        crate::labor::article_l1231_1(),
        crate::labor::article_l1232_1(),
        crate::labor::article_l1232_2(),
        crate::labor::article_l1233_3(),
        crate::labor::article_l1234_1(),
        crate::labor::article_l3121_18(),
        crate::labor::article_l3121_20(),
        crate::labor::article_l3121_27(),
        crate::labor::article_l3121_33(),
        crate::labor::article_l3121_34(),
    ]
}

/// Returns all French company law statutes (5 articles).
///
/// Covers Code de commerce Articles L225-1, L225-17, L225-18 (SA),
/// and L223-1, L223-3 (SARL).
#[must_use]
pub fn company_law_statutes() -> Vec<Statute> {
    vec![
        crate::company::article_l225_1(),
        crate::company::article_l225_17(),
        crate::company::article_l225_18(),
        crate::company::article_l223_1(),
        crate::company::article_l223_3(),
    ]
}

/// Returns all French family law statutes (19 articles).
///
/// Covers marriage Articles 143, 144, 146, 146-1, 147, 161, 165, 180;
/// divorce Articles 229, 230, 233, 237, 242, 247;
/// and property regime Articles 1387, 1400, 1401, 1404, 1536.
#[must_use]
pub fn family_law_statutes() -> Vec<Statute> {
    vec![
        crate::family::article143(),
        crate::family::article144(),
        crate::family::article146(),
        crate::family::article146_1(),
        crate::family::article147(),
        crate::family::article161(),
        crate::family::article165(),
        crate::family::article180(),
        crate::family::article229(),
        crate::family::article230(),
        crate::family::article233(),
        crate::family::article237(),
        crate::family::article242(),
        crate::family::article247(),
        crate::family::article1387(),
        crate::family::article1400(),
        crate::family::article1401(),
        crate::family::article1404(),
        crate::family::article1536(),
    ]
}

/// Returns all French statutes across all domains (45 articles total).
///
/// Aggregates: 6 contract + 15 labor + 5 company + 19 family = 45 statutes.
#[must_use]
pub fn all_french_statutes() -> Vec<Statute> {
    let mut all = contract_law_statutes();
    all.extend(labor_law_statutes());
    all.extend(company_law_statutes());
    all.extend(family_law_statutes());
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_law_statutes_count() {
        assert_eq!(contract_law_statutes().len(), 6);
    }

    #[test]
    fn labor_law_statutes_count() {
        assert_eq!(labor_law_statutes().len(), 15);
    }

    #[test]
    fn company_law_statutes_count() {
        assert_eq!(company_law_statutes().len(), 5);
    }

    #[test]
    fn family_law_statutes_count() {
        assert_eq!(family_law_statutes().len(), 19);
    }

    #[test]
    fn all_french_statutes_count() {
        assert_eq!(all_french_statutes().len(), 45);
    }

    #[test]
    fn all_statutes_have_fr_jurisdiction() {
        for s in all_french_statutes() {
            assert_eq!(
                s.jurisdiction,
                Some("FR".to_string()),
                "statute {} missing FR jurisdiction",
                s.id
            );
        }
    }

    #[test]
    fn contract_statutes_have_discretion() {
        // Contract statutes (authored with full detail) must all have discretion_logic
        for s in contract_law_statutes() {
            assert!(
                s.discretion_logic.is_some(),
                "contract statute {} missing discretion_logic",
                s.id
            );
            assert!(
                !s.discretion_logic.as_ref().unwrap().is_empty(),
                "contract statute {} has empty discretion_logic",
                s.id
            );
        }
    }

    #[test]
    fn company_statutes_have_discretion() {
        // Company statutes (authored with full detail) must all have discretion_logic
        for s in company_law_statutes() {
            assert!(
                s.discretion_logic.is_some(),
                "company statute {} missing discretion_logic",
                s.id
            );
            assert!(
                !s.discretion_logic.as_ref().unwrap().is_empty(),
                "company statute {} has empty discretion_logic",
                s.id
            );
        }
    }

    #[test]
    fn contract_statutes_include_expected_ids() {
        let statutes = contract_law_statutes();
        let ids: Vec<&str> = statutes.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"code-civil-1103"));
        assert!(ids.contains(&"code-civil-1104"));
        assert!(ids.contains(&"code-civil-1128"));
        assert!(ids.contains(&"code-civil-1217"));
        assert!(ids.contains(&"code-civil-1218"));
        assert!(ids.contains(&"code-civil-1231"));
    }

    #[test]
    fn company_statutes_include_sarl_articles() {
        let statutes = company_law_statutes();
        let ids: Vec<&str> = statutes.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"code-commerce-l225-1"));
        assert!(ids.contains(&"code-commerce-l225-17"));
        assert!(ids.contains(&"code-commerce-l225-18"));
        assert!(ids.contains(&"code-commerce-l223-1"));
        assert!(ids.contains(&"code-commerce-l223-3"));
    }

    #[test]
    fn all_statutes_contain_domains() {
        let statutes = all_french_statutes();
        assert!(statutes.iter().any(|s| s.id.starts_with("code-civil")));
        assert!(statutes.iter().any(|s| s.id.starts_with("code-travail")));
        assert!(statutes.iter().any(|s| s.id.starts_with("code-commerce")));
    }
}
