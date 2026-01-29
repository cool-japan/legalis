//! Integration tests for common utilities

use chrono::NaiveDate;
use legalis_mx::common::*;

#[test]
fn test_mexican_currency() {
    let amount = MexicanCurrency::from_pesos(1000);
    assert_eq!(amount.pesos(), 1000);
    assert_eq!(amount.cents(), 0);
}

#[test]
fn test_mexican_currency_with_cents() {
    let amount = MexicanCurrency::from_centavos(150050);
    assert_eq!(amount.pesos(), 1500);
    assert_eq!(amount.cents(), 50);
}

#[test]
fn test_rfc_validation() {
    assert!(validate_rfc("XAXX010101000").is_ok());
    assert!(validate_rfc("ABC010101ABC").is_ok());
    assert!(validate_rfc("123").is_err());
}

#[test]
fn test_curp_validation() {
    assert!(validate_curp("XAXX010101HDFRRL00").is_ok());
    assert!(validate_curp("123").is_err());
}

#[test]
fn test_nss_validation() {
    assert!(validate_nss("12345678901").is_ok());
    assert!(validate_nss("123").is_err());
}

#[test]
fn test_mexican_states() {
    assert_eq!(MexicanState::CMX.nombre_es(), "Ciudad de México");
    assert_eq!(MexicanState::NLE.nombre_es(), "Nuevo León");
    assert_eq!(MexicanState::JAL.nombre_es(), "Jalisco");

    assert_eq!(MexicanState::CMX.region_es(), "Centro");
    assert_eq!(MexicanState::NLE.region_es(), "Noreste");
    assert_eq!(MexicanState::YUC.region_es(), "Sureste");
}

#[test]
fn test_federal_holidays() {
    let new_year = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    assert!(FederalHoliday::NewYear.is_holiday(new_year));

    let independence_day = NaiveDate::from_ymd_opt(2024, 9, 16).unwrap();
    assert!(FederalHoliday::IndependenceDay.is_holiday(independence_day));
}

#[test]
fn test_minimum_wage() {
    let wage_2024 = minimum_wage::get_minimum_wage(2024, false);
    assert!(wage_2024.is_some());
}

#[test]
fn test_uma_values() {
    let daily_uma = uma::get_uma(2024, uma::UmaPeriod::Daily);
    assert!(daily_uma.is_some());

    let monthly_uma = uma::get_uma(2024, uma::UmaPeriod::Monthly);
    assert!(monthly_uma.is_some());
}
