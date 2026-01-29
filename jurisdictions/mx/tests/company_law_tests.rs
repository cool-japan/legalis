//! Integration tests for company law

use legalis_mx::company_law::*;

#[test]
fn test_create_sa() {
    let sa = StockCorporation::new(
        "Mi Empresa SA de CV".to_string(),
        "Comercio y servicios".to_string(),
        10_000_000, // 100,000 pesos (in cents)
        5,
    );

    assert!(sa.is_ok());
    let sa = sa.unwrap();
    assert!(sa.validate().is_ok());
    assert_eq!(sa.num_accionistas, 5);
}

#[test]
fn test_sa_minimum_capital() {
    let sa = StockCorporation::new(
        "Test SA".to_string(),
        "Test".to_string(),
        4_000_000, // Below minimum (40,000 pesos in cents)
        2,
    );

    assert!(sa.is_err());
}

#[test]
fn test_create_srl() {
    let srl = LimitedLiabilityCompany::new(
        "Mi Empresa SRL de CV".to_string(),
        "Servicios profesionales".to_string(),
        5_000_000,
        3,
    );

    assert!(srl.is_ok());
    let srl = srl.unwrap();
    assert!(srl.validate().is_ok());
}

#[test]
fn test_srl_max_partners() {
    let srl = LimitedLiabilityCompany::new(
        "Test SRL".to_string(),
        "Test".to_string(),
        5_000_000,
        51, // Exceeds maximum
    );

    assert!(srl.is_err());
}

#[test]
fn test_srl_minimum_partners() {
    let srl = LimitedLiabilityCompany::new(
        "Test SRL".to_string(),
        "Test".to_string(),
        5_000_000,
        1, // Below minimum
    );

    assert!(srl.is_err());
}
