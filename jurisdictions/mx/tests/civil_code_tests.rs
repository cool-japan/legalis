//! Integration tests for civil code

use chrono::Utc;
use legalis_mx::civil_code::*;
use legalis_mx::common::MexicanCurrency;

#[test]
fn test_natural_person() {
    let person = NaturalPerson::new(
        "Juan Pérez González".to_string(),
        Some("PEGJ800101XXX".to_string()),
        Some("PEGJ800101HDFRXN00".to_string()),
    );

    assert!(person.has_full_capacity());
    assert!(person.validate().is_ok());
}

#[test]
fn test_juridical_person() {
    let person = JuridicalPerson::new(
        "Empresa SA de CV".to_string(),
        "EMP010101XXX".to_string(),
        EntityType::PrivateCorporation,
    );

    assert!(person.validate().is_ok());
}

#[test]
fn test_contract_creation() {
    let parties = vec![
        Party {
            nombre: "Comprador".to_string(),
            rol: PartyRole::Buyer,
            capacidad: true,
        },
        Party {
            nombre: "Vendedor".to_string(),
            rol: PartyRole::Seller,
            capacidad: true,
        },
    ];

    let contract = Contract::new(
        parties,
        "Venta de inmueble".to_string(),
        ContractType::Sale,
        Utc::now(),
    );

    assert!(contract.validate().is_ok());
    assert!(contract.is_bilateral());
}

#[test]
fn test_give_obligation() {
    let obligation = GiveObligation::new(
        "Entregar mercancía".to_string(),
        Some(MexicanCurrency::from_pesos(10_000)),
    );

    assert!(obligation.validate().is_ok());
}

#[test]
fn test_immovable_property() {
    let property = ImmovableProperty::new("Casa en Ciudad de México".to_string());
    assert!(property.validate().is_ok());
}

#[test]
fn test_movable_property() {
    let property = MovableProperty::new("Vehículo Nissan Sentra 2024".to_string());
    assert!(property.validate().is_ok());
}
