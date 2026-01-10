//! Construction and Real Estate Validator
//!
//! Validation logic for construction licenses and real estate transactions.

use crate::construction_real_estate::{
    error::{ConstructionRealEstateError, Result},
    types::{
        ConstructionBusinessLicense, ConstructionType, ManagerQualification, RealEstateTransaction,
    },
};
use crate::egov::ValidationReport;

/// Validate construction business license
pub fn validate_construction_license(
    license: &ConstructionBusinessLicense,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Article 7: Capital requirement
    if !license.meets_capital_requirement() {
        return Err(ConstructionRealEstateError::InsufficientCapital {
            actual: license.registered_capital_jpy,
            required: license.license_type.minimum_capital(),
        });
    }

    // Article 8: Manager requirements
    if license.managers.is_empty() {
        report.add_error("At least one qualified manager required (建設業法第8条)");
    }

    // Check managers have appropriate qualifications for construction types
    for construction_type in &license.construction_types {
        if !has_qualified_manager_for_type(&license.managers, construction_type) {
            report.add_warning(format!(
                "No qualified manager for {:?} (建設業法第8条推奨)",
                construction_type
            ));
        }
    }

    // Check license validity
    if !license.is_valid() {
        let expiry = license.expiration_date.format("%Y-%m-%d").to_string();
        return Err(ConstructionRealEstateError::LicenseExpired {
            expiration_date: expiry,
        });
    }

    // Warn if expiring soon (within 90 days)
    let days_left = license.days_until_expiration();
    if days_left > 0 && days_left < 90 {
        report.add_warning(format!(
            "License expires in {} days - renewal recommended",
            days_left
        ));
    }

    // Check license validity period (should be 5 years)
    let validity_years = (license.expiration_date - license.issue_date).num_days() / 365;
    if validity_years != 5 {
        report.add_warning("License validity should be 5 years (建設業法第3条3項)");
    }

    Ok(report)
}

/// Check if license has qualified manager for construction type
fn has_qualified_manager_for_type(
    managers: &[crate::construction_real_estate::types::Manager],
    construction_type: &ConstructionType,
) -> bool {
    managers.iter().any(|manager| {
        matches!(
            (&manager.qualification, construction_type),
            (
                ManagerQualification::FirstClassArchitect,
                ConstructionType::Architecture
            ) | (
                ManagerQualification::SecondClassArchitect,
                ConstructionType::Architecture
            ) | (ManagerQualification::CivilEngineer, ConstructionType::Civil)
                | (ManagerQualification::ConstructionManager, _)
        )
    })
}

/// Validate real estate transaction
pub fn validate_real_estate_transaction(
    transaction: &RealEstateTransaction,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Article 35: Important matters explanation required
    if !transaction.important_matters_explained {
        return Err(ConstructionRealEstateError::ImportantMattersNotExplained);
    }

    // Article 46: Commission limits
    if let Some(broker) = &transaction.broker {
        let max_commission = transaction.calculate_max_commission();
        if broker.commission_jpy > max_commission {
            return Err(ConstructionRealEstateError::CommissionExceedsLimit {
                actual: broker.commission_jpy,
                max: max_commission,
            });
        }

        // Check agent registration is valid
        if !broker.agent.is_registration_valid() {
            report.add_error("Licensed agent registration has expired (宅地建物取引士)");
        }
    }

    // Basic field validation
    if transaction.property.address.is_empty() {
        report.add_error("Property address is required");
    }

    if transaction.property.price_jpy == 0 {
        report.add_error("Property price must be greater than zero");
    }

    if transaction.buyer.name.is_empty() {
        report.add_error("Buyer name is required");
    }

    if transaction.seller.name.is_empty() {
        report.add_error("Seller name is required");
    }

    Ok(report)
}

/// Quick validation for construction license
pub fn quick_validate_construction(license: &ConstructionBusinessLicense) -> bool {
    validate_construction_license(license)
        .map(|report| report.is_valid())
        .unwrap_or(false)
}

/// Quick validation for real estate transaction
pub fn quick_validate_real_estate(transaction: &RealEstateTransaction) -> bool {
    validate_real_estate_transaction(transaction)
        .map(|report| report.is_valid())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::construction_real_estate::types::*;
    use chrono::{Duration, Utc};

    fn create_test_license() -> ConstructionBusinessLicense {
        ConstructionBusinessLicense {
            license_number: "TEST-001".to_string(),
            business_name: "Test Construction Co.".to_string(),
            license_type: ConstructionLicenseType::General,
            construction_types: vec![ConstructionType::Architecture],
            registered_capital_jpy: 10_000_000,
            issue_date: Utc::now().date_naive(),
            expiration_date: Utc::now().date_naive() + Duration::days(365 * 5),
            managers: vec![Manager {
                name: "Manager".to_string(),
                qualification: ManagerQualification::FirstClassArchitect,
                certification_number: "CERT-001".to_string(),
                certification_date: Utc::now().date_naive(),
            }],
        }
    }

    #[test]
    fn test_validate_valid_license() {
        let license = create_test_license();
        let report = validate_construction_license(&license).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_insufficient_capital() {
        let mut license = create_test_license();
        license.license_type = ConstructionLicenseType::Special;
        license.registered_capital_jpy = 10_000_000; // Less than required ¥20M

        let result = validate_construction_license(&license);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstructionRealEstateError::InsufficientCapital { .. }
        ));
    }

    #[test]
    fn test_validate_no_managers() {
        let mut license = create_test_license();
        license.managers.clear();

        let report = validate_construction_license(&license).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("manager")));
    }

    #[test]
    fn test_validate_expired_license() {
        let mut license = create_test_license();
        license.expiration_date = Utc::now().date_naive() - Duration::days(1);

        let result = validate_construction_license(&license);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstructionRealEstateError::LicenseExpired { .. }
        ));
    }

    fn create_test_transaction() -> RealEstateTransaction {
        RealEstateTransaction {
            transaction_id: "TX-001".to_string(),
            transaction_type: TransactionType::Sale,
            property: Property {
                property_type: PropertyType::Building,
                address: "Tokyo".to_string(),
                area_sqm: 100.0,
                price_jpy: 5_000_000,
                description: None,
            },
            buyer: Party {
                name: "Buyer".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            seller: Party {
                name: "Seller".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            broker: None,
            important_matters_explained: true,
            contract_date: Utc::now().date_naive(),
        }
    }

    #[test]
    fn test_validate_valid_transaction() {
        let transaction = create_test_transaction();
        let report = validate_real_estate_transaction(&transaction).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_no_important_matters() {
        let mut transaction = create_test_transaction();
        transaction.important_matters_explained = false;

        let result = validate_real_estate_transaction(&transaction);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstructionRealEstateError::ImportantMattersNotExplained
        ));
    }

    #[test]
    fn test_validate_excessive_commission() {
        let agent = LicensedAgent {
            name: "Agent".to_string(),
            registration_number: "AG-001".to_string(),
            registration_date: Utc::now().date_naive(),
        };

        let broker = LicensedBroker {
            company_name: "Test Realty".to_string(),
            license_number: "LIC-001".to_string(),
            agent,
            commission_jpy: 10_000_000, // Excessive
        };

        let mut transaction = create_test_transaction();
        transaction.broker = Some(broker);

        let result = validate_real_estate_transaction(&transaction);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstructionRealEstateError::CommissionExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_quick_validate() {
        let license = create_test_license();
        assert!(quick_validate_construction(&license));

        let transaction = create_test_transaction();
        assert!(quick_validate_real_estate(&transaction));
    }
}
