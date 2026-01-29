//! Integration tests for tax law

use legalis_mx::common::MexicanCurrency;
use legalis_mx::tax_law::*;

#[test]
fn test_iva_standard_rate() {
    let base = MexicanCurrency::from_pesos(1000);
    let iva = calculate_iva(base, IVARate::Standard);

    // 16% of 1,000 = 160
    assert_eq!(iva.pesos(), 160);
}

#[test]
fn test_iva_with_total() {
    let base = MexicanCurrency::from_pesos(1000);
    let total = calculate_with_iva(base, IVARate::Standard);

    // 1,000 + 160 = 1,160
    assert_eq!(total.pesos(), 1160);
}

#[test]
fn test_iva_extraction() {
    let total = MexicanCurrency::from_pesos(1160);
    let iva = extract_iva_from_total(total, IVARate::Standard);

    // Should extract approximately 160
    assert!(iva.pesos() >= 159 && iva.pesos() <= 161);
}

#[test]
fn test_corporate_isr() {
    let income = MexicanCurrency::from_pesos(1_000_000);
    let isr = calculate_corporate_isr(income);

    // 30% of 1,000,000 = 300,000
    assert_eq!(isr.pesos(), 300_000);
}

#[test]
fn test_ieps_beer() {
    let base = MexicanCurrency::from_pesos(100);
    let ieps = calculate_ieps(base, IEPSCategory::Beer);

    // 26.5% of 100 = 26.5
    assert_eq!(ieps.pesos(), 26);
}

#[test]
fn test_ieps_sugary_drinks() {
    let ieps = calculate_sugary_drinks_ieps(2.0); // 2 liters
    assert_eq!(ieps.pesos(), 2); // 1 peso per liter
}

#[test]
fn test_taxpayer_validation() {
    let taxpayer = Taxpayer::new(
        "XAXX010101000".to_string(),
        "Test Company".to_string(),
        TaxpayerType::Corporation,
    );

    assert!(validate_taxpayer(&taxpayer).is_ok());
}
