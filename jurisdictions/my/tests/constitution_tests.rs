//! Integration tests for Federal Constitution.

use legalis_my::constitution::*;

#[test]
fn test_fundamental_liberties() {
    assert_eq!(FundamentalLiberty::LibertyOfPerson.article(), 5);
    assert_eq!(FundamentalLiberty::Equality.article(), 8);
    assert_eq!(FundamentalLiberty::FreedomOfExpression.article(), 10);
}

#[test]
fn test_federal_state_matters() {
    assert!(ConstitutionalValidator::is_federal_matter("company law"));
    assert!(ConstitutionalValidator::is_federal_matter("criminal law"));
    assert!(ConstitutionalValidator::is_state_matter("land matters"));
    assert!(ConstitutionalValidator::is_state_matter("islamic law"));
}

#[test]
fn test_state_classification() {
    assert!(State::KualaLumpur.is_federal_territory());
    assert!(!State::Selangor.is_federal_territory());
    assert!(State::Selangor.has_syariah_jurisdiction());
    assert!(!State::Putrajaya.has_syariah_jurisdiction());
}
