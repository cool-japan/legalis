//! Construction & Real Estate Edge Case Tests
use chrono::{Duration, NaiveDate, Utc};
use legalis_jp::construction_real_estate::*;

#[test]
fn test_construction_valid_general() {
    let license = ConstructionBusinessLicense {
        license_number: "1".to_string(),
        business_name: "A".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![ConstructionType::Civil],
        registered_capital_jpy: 5_000_000,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        managers: vec![Manager {
            name: "M".to_string(),
            qualification: ManagerQualification::CivilEngineer,
            certification_number: "C".to_string(),
            certification_date: Utc::now().date_naive(),
        }],
    };
    assert!(validate_construction_license(&license).is_ok());
}

#[test]
fn test_construction_below_minimum() {
    let license = ConstructionBusinessLicense {
        license_number: "2".to_string(),
        business_name: "B".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![ConstructionType::Architecture],
        registered_capital_jpy: 4_999_999,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        managers: vec![],
    };
    assert!(validate_construction_license(&license).is_err());
}

#[test]
fn test_construction_special_valid() {
    let license = ConstructionBusinessLicense {
        license_number: "3".to_string(),
        business_name: "C".to_string(),
        license_type: ConstructionLicenseType::Special,
        construction_types: vec![ConstructionType::Architecture],
        registered_capital_jpy: 20_000_000,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        managers: vec![Manager {
            name: "M".to_string(),
            qualification: ManagerQualification::FirstClassArchitect,
            certification_number: "A".to_string(),
            certification_date: Utc::now().date_naive(),
        }],
    };
    assert!(validate_construction_license(&license).is_ok());
}

#[test]
fn test_construction_special_below() {
    let license = ConstructionBusinessLicense {
        license_number: "4".to_string(),
        business_name: "D".to_string(),
        license_type: ConstructionLicenseType::Special,
        construction_types: vec![ConstructionType::Electrical],
        registered_capital_jpy: 19_999_999,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        managers: vec![],
    };
    assert!(validate_construction_license(&license).is_err());
}

#[test]
fn test_construction_zero_capital() {
    let license = ConstructionBusinessLicense {
        license_number: "5".to_string(),
        business_name: "E".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![ConstructionType::Carpentry],
        registered_capital_jpy: 0,
        issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        expiration_date: NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        managers: vec![],
    };
    assert!(validate_construction_license(&license).is_err());
}

#[test]
fn test_construction_expired() {
    let license = ConstructionBusinessLicense {
        license_number: "6".to_string(),
        business_name: "F".to_string(),
        license_type: ConstructionLicenseType::General,
        construction_types: vec![ConstructionType::PlumbingHeating],
        registered_capital_jpy: 10_000_000,
        issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        expiration_date: Utc::now().date_naive() - Duration::days(1),
        managers: vec![],
    };
    assert!(validate_construction_license(&license).is_err());
}

#[test]
fn test_real_estate_missing_important() {
    let transaction = RealEstateTransaction {
        transaction_id: "1".to_string(),
        transaction_type: TransactionType::Sale,
        property: Property {
            property_type: PropertyType::Land,
            address: "A".to_string(),
            area_sqm: 100.0,
            price_jpy: 10_000_000,
            description: None,
        },
        buyer: Party {
            name: "B".to_string(),
            address: "B".to_string(),
            contact: None,
        },
        seller: Party {
            name: "S".to_string(),
            address: "S".to_string(),
            contact: None,
        },
        broker: None,
        important_matters_explained: false,
        contract_date: Utc::now().date_naive(),
    };
    assert!(validate_real_estate_transaction(&transaction).is_err());
}

#[test]
fn test_real_estate_valid() {
    let transaction = RealEstateTransaction {
        transaction_id: "2".to_string(),
        transaction_type: TransactionType::Sale,
        property: Property {
            property_type: PropertyType::Building,
            address: "A".to_string(),
            area_sqm: 80.0,
            price_jpy: 15_000_000,
            description: Some("2LDK".to_string()),
        },
        buyer: Party {
            name: "B".to_string(),
            address: "B".to_string(),
            contact: Some("1".to_string()),
        },
        seller: Party {
            name: "S".to_string(),
            address: "S".to_string(),
            contact: Some("2".to_string()),
        },
        broker: None,
        important_matters_explained: true,
        contract_date: Utc::now().date_naive(),
    };
    assert!(validate_real_estate_transaction(&transaction).is_ok());
}
