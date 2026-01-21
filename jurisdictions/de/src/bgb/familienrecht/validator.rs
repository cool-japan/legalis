//! Validation functions for German Family Law
//!
//! Implements BGB Book 4 validation logic for marriage, divorce, maintenance, and custody.

use chrono::Utc;

use super::error::{FamilyLawError, Result};
use super::types::*;

/// Validate a person's basic information
pub fn validate_person(person: &Person) -> Result<()> {
    if person.name.trim().is_empty() {
        return Err(FamilyLawError::EmptyName);
    }

    // Check if birth date is in the future
    if person.date_of_birth > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }

    Ok(())
}

/// Validate marriage formation under §§1303-1311 BGB
pub fn validate_marriage(marriage: &Marriage) -> Result<()> {
    // Validate both spouses
    validate_person(&marriage.spouse1)?;
    validate_person(&marriage.spouse2)?;

    // Check that spouses are different people
    if marriage.spouse1.name == marriage.spouse2.name
        && marriage.spouse1.date_of_birth == marriage.spouse2.date_of_birth
    {
        return Err(FamilyLawError::SpousesIdentical);
    }

    // Check marriage date
    if marriage.marriage_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }

    // Check registrar office
    if marriage.registrar_office.trim().is_empty() {
        return Err(FamilyLawError::NoRegistrarOffice);
    }

    // Validate marriage requirements if status is Valid
    if marriage.status == MarriageStatus::Valid {
        // Check for impediments
        if !marriage.impediments.is_empty() {
            return Err(FamilyLawError::MarriageImpedimentsExist);
        }

        // §1303 BGB - Minimum age requirement (18 years)
        let spouse1_age = marriage.spouse1.age_at(marriage.marriage_date);
        let spouse2_age = marriage.spouse2.age_at(marriage.marriage_date);

        if spouse1_age < 18 {
            return Err(FamilyLawError::BelowMarriageAge {
                actual_age: spouse1_age,
            });
        }
        if spouse2_age < 18 {
            return Err(FamilyLawError::BelowMarriageAge {
                actual_age: spouse2_age,
            });
        }
    }

    // If marriage has impediments, status cannot be Valid
    if !marriage.impediments.is_empty() && marriage.status == MarriageStatus::Valid {
        return Err(FamilyLawError::MarriageImpedimentsExist);
    }

    // Check for specific impediments in the list
    if marriage
        .impediments
        .contains(&MarriageImpediment::ExistingMarriage)
    {
        return Err(FamilyLawError::ExistingMarriage);
    }
    if marriage
        .impediments
        .contains(&MarriageImpediment::Consanguinity)
    {
        return Err(FamilyLawError::Consanguinity);
    }
    if marriage
        .impediments
        .contains(&MarriageImpediment::LackOfCapacity)
    {
        return Err(FamilyLawError::LackOfCapacity);
    }

    Ok(())
}

/// Validate matrimonial property agreement under §§1408-1410 BGB
pub fn validate_matrimonial_property_agreement(
    agreement: &MatrimonialPropertyAgreement,
) -> Result<()> {
    // Validate spouses
    validate_person(&agreement.spouses.0)?;
    validate_person(&agreement.spouses.1)?;

    // Check that spouses are different
    if agreement.spouses.0.name == agreement.spouses.1.name
        && agreement.spouses.0.date_of_birth == agreement.spouses.1.date_of_birth
    {
        return Err(FamilyLawError::AgreementNotBetweenSpouses);
    }

    // §1410 BGB - Agreement must be notarized
    if !agreement.notarized {
        return Err(FamilyLawError::AgreementNotNotarized);
    }

    // Check agreement date
    if agreement.agreement_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }

    Ok(())
}

/// Validate accrued gains calculation under §§1372-1390 BGB
pub fn validate_accrued_gains_calculation(calculation: &AccruedGainsCalculation) -> Result<()> {
    // Check dates
    if calculation.marriage_start_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if calculation.marriage_end_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if calculation.marriage_end_date < calculation.marriage_start_date {
        return Err(FamilyLawError::InvalidDate {
            date_type: "End date before start date".to_string(),
        });
    }

    // Validate assets are not negative
    validate_assets(&calculation.spouse1_initial_assets)?;
    validate_assets(&calculation.spouse1_final_assets)?;
    validate_assets(&calculation.spouse2_initial_assets)?;
    validate_assets(&calculation.spouse2_final_assets)?;

    Ok(())
}

/// Validate assets structure
fn validate_assets(_assets: &Assets) -> Result<()> {
    // All asset values must be non-negative (already guaranteed by Capital type using u64)
    // This is a placeholder for any additional asset validation logic
    Ok(())
}

/// Validate divorce proceedings under §§1564-1587 BGB
pub fn validate_divorce(divorce: &Divorce) -> Result<()> {
    // Marriage must be valid to be divorced
    if divorce.marriage.status != MarriageStatus::Valid {
        return Err(FamilyLawError::CannotDivorceInvalidMarriage);
    }

    // Validate the marriage itself
    validate_marriage(&divorce.marriage)?;

    // Check dates
    if divorce.filing_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if divorce.separation_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if let Some(decree_date) = divorce.divorce_decree_date {
        if decree_date > Utc::now().date_naive() {
            return Err(FamilyLawError::FutureDate);
        }
        if decree_date < divorce.filing_date {
            return Err(FamilyLawError::InvalidDate {
                date_type: "Decree date before filing date".to_string(),
            });
        }
    }

    // Separation date must be before filing date
    if divorce.separation_date > divorce.filing_date {
        return Err(FamilyLawError::InvalidDate {
            date_type: "Separation date after filing date".to_string(),
        });
    }

    // §1566 BGB - Check separation period requirement
    if !divorce.meets_separation_requirement() {
        let months = divorce.separation_period_months();
        let required = if divorce.mutual_consent { 12 } else { 36 };
        return Err(FamilyLawError::InsufficientSeparationPeriod {
            actual_months: months,
            required_months: required,
        });
    }

    // Validate accrued gains calculation if present
    if let Some(ref accrued_gains) = divorce.accrued_gains_equalization {
        // Only valid for community of accrued gains regime
        if divorce.marriage.property_regime != MatrimonialPropertyRegime::CommunityOfAccruedGains {
            return Err(FamilyLawError::AccruedGainsNotApplicable);
        }
        validate_accrued_gains_calculation(accrued_gains)?;
    }

    // Validate pension equalization if present
    if let Some(ref pension_eq) = divorce.pension_equalization {
        validate_pension_equalization(pension_eq)?;
    }

    Ok(())
}

/// Validate pension equalization under §§1587-1587p BGB
pub fn validate_pension_equalization(equalization: &PensionEqualization) -> Result<()> {
    // Marriage duration must be positive
    if equalization.marriage_duration_years == 0 {
        return Err(FamilyLawError::InvalidDate {
            date_type: "Marriage duration is zero".to_string(),
        });
    }

    // Pension rights values are already non-negative (Capital uses u64)
    Ok(())
}

/// Validate post-marital maintenance under §§1569-1586 BGB
pub fn validate_post_marital_maintenance(maintenance: &PostMaritalMaintenance) -> Result<()> {
    // Validate persons
    validate_person(&maintenance.claimant)?;
    validate_person(&maintenance.obligor)?;

    // Claimant and obligor must be different
    if maintenance.claimant.name == maintenance.obligor.name
        && maintenance.claimant.date_of_birth == maintenance.obligor.date_of_birth
    {
        return Err(FamilyLawError::SelfMaintenance);
    }

    // Monthly amount must be positive
    if maintenance.monthly_amount.amount_cents == 0 {
        return Err(FamilyLawError::NoMaintenanceAmount);
    }

    // Check dates
    if maintenance.start_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if let Some(end_date) = maintenance.end_date
        && end_date < maintenance.start_date
    {
        return Err(FamilyLawError::InvalidDate {
            date_type: "End date before start date".to_string(),
        });
    }

    Ok(())
}

/// Validate maintenance obligation under §§1601-1615 BGB
pub fn validate_maintenance_obligation(obligation: &MaintenanceObligation) -> Result<()> {
    // Validate persons
    validate_person(&obligation.obligor)?;
    validate_person(&obligation.beneficiary)?;

    // Obligor and beneficiary must be different
    if obligation.obligor.name == obligation.beneficiary.name
        && obligation.obligor.date_of_birth == obligation.beneficiary.date_of_birth
    {
        return Err(FamilyLawError::SelfMaintenance);
    }

    // Monthly amount must be positive
    if obligation.monthly_amount.amount_cents == 0 {
        return Err(FamilyLawError::NoMaintenanceAmount);
    }

    // Check dates
    if obligation.start_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }
    if let Some(end_date) = obligation.end_date
        && end_date < obligation.start_date
    {
        return Err(FamilyLawError::InvalidDate {
            date_type: "End date before start date".to_string(),
        });
    }

    // Validate relationship-specific rules
    match obligation.relationship {
        MaintenanceRelationship::ParentToChild => {
            // Parent must be adult
            if !obligation.obligor.is_adult() {
                return Err(FamilyLawError::CustodyHolderMinor);
            }
        }
        MaintenanceRelationship::ChildToParent => {
            // Child must be adult to owe maintenance to parent
            if !obligation.obligor.is_adult() {
                return Err(FamilyLawError::CustodyHolderMinor);
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate parent-child relationship under §§1591-1600 BGB
pub fn validate_parent_child_relationship(relationship: &ParentChildRelationship) -> Result<()> {
    // Validate persons
    validate_person(&relationship.parent)?;
    validate_person(&relationship.child)?;

    // Child cannot be older than parent
    if relationship.child.date_of_birth < relationship.parent.date_of_birth {
        return Err(FamilyLawError::ChildOlderThanParent);
    }

    // Validate parentage status rules
    match relationship.parentage_status {
        ParentageStatus::MotherByBirth => {
            // §1591 BGB - Mother must be female
            if relationship.parent.gender != Gender::Female {
                return Err(FamilyLawError::InvalidMother);
            }
        }
        ParentageStatus::FatherByMarriage
        | ParentageStatus::FatherByAcknowledgment
        | ParentageStatus::FatherByCourtDetermination => {
            // §1592 BGB - Father must be male
            if relationship.parent.gender != Gender::Male {
                return Err(FamilyLawError::InvalidFather);
            }
        }
    }

    // Check established date
    if relationship.established_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }

    // Established date should not be before child's birth
    if relationship.established_date < relationship.child.date_of_birth {
        return Err(FamilyLawError::InvalidDate {
            date_type: "Parentage established before child's birth".to_string(),
        });
    }

    Ok(())
}

/// Validate parental custody under §§1626-1698 BGB
pub fn validate_parental_custody(custody: &ParentalCustody) -> Result<()> {
    // Validate child
    validate_person(&custody.child)?;

    // §1626 BGB - Custody only for minor children
    if custody.child.is_adult() {
        return Err(FamilyLawError::ChildAdult);
    }

    // At least one custody holder required
    if custody.custody_holders.is_empty() {
        return Err(FamilyLawError::NoCustodyHolders);
    }

    // Validate all custody holders
    for holder in &custody.custody_holders {
        validate_person(holder)?;

        // Custody holder must be adult
        if !holder.is_adult() {
            return Err(FamilyLawError::CustodyHolderMinor);
        }

        // Custody holder cannot be the child
        if holder.name == custody.child.name && holder.date_of_birth == custody.child.date_of_birth
        {
            return Err(FamilyLawError::MissingPerson {
                person_type: "Valid custody holder".to_string(),
            });
        }
    }

    // §1626 BGB - Joint custody requires exactly two holders
    if custody.custody_type == CustodyType::Joint && custody.custody_holders.len() != 2 {
        return Err(FamilyLawError::InvalidJointCustody);
    }

    // Check established date
    if custody.established_date > Utc::now().date_naive() {
        return Err(FamilyLawError::FutureDate);
    }

    // Established date should not be before child's birth
    if custody.established_date < custody.child.date_of_birth {
        return Err(FamilyLawError::InvalidDate {
            date_type: "Custody established before child's birth".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate};

    fn create_valid_person(name: &str, age_years: u32) -> Person {
        let today = Utc::now().date_naive();
        Person {
            name: name.to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(
                today.year() - age_years as i32,
                today.month(),
                today.day(),
            )
            .unwrap(),
            place_of_birth: "Berlin".to_string(),
            nationality: "German".to_string(),
            gender: Gender::Male,
            address: "Test Street 1".to_string(),
        }
    }

    #[test]
    fn test_valid_marriage() {
        let marriage = Marriage {
            spouse1: create_valid_person("Hans", 30),
            spouse2: Person {
                gender: Gender::Female,
                ..create_valid_person("Maria", 28)
            },
            marriage_date: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
            place_of_marriage: "Berlin".to_string(),
            registrar_office: "Standesamt Berlin-Mitte".to_string(),
            status: MarriageStatus::Valid,
            property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            impediments: vec![],
        };

        assert!(validate_marriage(&marriage).is_ok());
    }

    #[test]
    fn test_marriage_below_minimum_age() {
        let mut marriage = Marriage {
            spouse1: create_valid_person("Hans", 30),
            spouse2: Person {
                gender: Gender::Female,
                ..create_valid_person("Maria", 17)
            },
            marriage_date: Utc::now().date_naive(),
            place_of_marriage: "Berlin".to_string(),
            registrar_office: "Standesamt Berlin-Mitte".to_string(),
            status: MarriageStatus::Valid,
            property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            impediments: vec![],
        };

        // Set spouse2 birth date to make them 17 at marriage
        marriage.spouse2.date_of_birth =
            NaiveDate::from_ymd_opt(marriage.marriage_date.year() - 17, 1, 1).unwrap();

        let result = validate_marriage(&marriage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::BelowMarriageAge { .. }
        ));
    }

    #[test]
    fn test_marriage_with_existing_marriage_impediment() {
        let marriage = Marriage {
            spouse1: create_valid_person("Hans", 30),
            spouse2: Person {
                gender: Gender::Female,
                ..create_valid_person("Maria", 28)
            },
            marriage_date: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
            place_of_marriage: "Berlin".to_string(),
            registrar_office: "Standesamt Berlin-Mitte".to_string(),
            status: MarriageStatus::Invalid,
            property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            impediments: vec![MarriageImpediment::ExistingMarriage],
        };

        let result = validate_marriage(&marriage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::ExistingMarriage
        ));
    }

    #[test]
    fn test_valid_divorce_with_mutual_consent() {
        let marriage = Marriage {
            spouse1: create_valid_person("Hans", 35),
            spouse2: Person {
                gender: Gender::Female,
                ..create_valid_person("Maria", 33)
            },
            marriage_date: NaiveDate::from_ymd_opt(2018, 6, 15).unwrap(),
            place_of_marriage: "Berlin".to_string(),
            registrar_office: "Standesamt Berlin-Mitte".to_string(),
            status: MarriageStatus::Valid,
            property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            impediments: vec![],
        };

        let divorce = Divorce {
            marriage,
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            separation_date: NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(), // 13 months ago
            ground: DivorceGround::MarriageBreakdown,
            mutual_consent: true,
            divorce_decree_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            accrued_gains_equalization: None,
            pension_equalization: None,
        };

        assert!(validate_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_divorce_insufficient_separation_with_consent() {
        let marriage = Marriage {
            spouse1: create_valid_person("Hans", 35),
            spouse2: Person {
                gender: Gender::Female,
                ..create_valid_person("Maria", 33)
            },
            marriage_date: NaiveDate::from_ymd_opt(2018, 6, 15).unwrap(),
            place_of_marriage: "Berlin".to_string(),
            registrar_office: "Standesamt Berlin-Mitte".to_string(),
            status: MarriageStatus::Valid,
            property_regime: MatrimonialPropertyRegime::CommunityOfAccruedGains,
            impediments: vec![],
        };

        let divorce = Divorce {
            marriage,
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            separation_date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(), // Only 6 months
            ground: DivorceGround::MarriageBreakdown,
            mutual_consent: true, // Requires 12 months
            divorce_decree_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            accrued_gains_equalization: None,
            pension_equalization: None,
        };

        let result = validate_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::InsufficientSeparationPeriod { .. }
        ));
    }

    #[test]
    fn test_valid_parental_custody() {
        let child = create_valid_person("Kind", 5);
        let parent1 = create_valid_person("Vater", 35);
        let parent2 = Person {
            gender: Gender::Female,
            ..create_valid_person("Mutter", 33)
        };

        let custody = ParentalCustody {
            child: child.clone(),
            custody_holders: vec![parent1, parent2],
            custody_type: CustodyType::Joint,
            established_date: child.date_of_birth,
        };

        assert!(validate_parental_custody(&custody).is_ok());
    }

    #[test]
    fn test_custody_child_adult() {
        let child = create_valid_person("Kind", 20); // Adult
        let parent = create_valid_person("Vater", 45);

        let custody = ParentalCustody {
            child,
            custody_holders: vec![parent],
            custody_type: CustodyType::Sole,
            established_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        };

        let result = validate_parental_custody(&custody);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FamilyLawError::ChildAdult));
    }

    #[test]
    fn test_invalid_joint_custody_one_holder() {
        let child = create_valid_person("Kind", 5);
        let parent = create_valid_person("Vater", 35);

        let custody = ParentalCustody {
            child: child.clone(),
            custody_holders: vec![parent],
            custody_type: CustodyType::Joint, // Joint requires 2 holders
            established_date: child.date_of_birth,
        };

        let result = validate_parental_custody(&custody);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::InvalidJointCustody
        ));
    }

    #[test]
    fn test_valid_maintenance_obligation() {
        use crate::gmbhg::Capital;

        let parent = create_valid_person("Vater", 40);
        let child = create_valid_person("Kind", 10);

        let obligation = MaintenanceObligation {
            obligor: parent,
            beneficiary: child,
            relationship: MaintenanceRelationship::ParentToChild,
            monthly_amount: Capital::from_euros(500),
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            end_date: None,
        };

        assert!(validate_maintenance_obligation(&obligation).is_ok());
    }

    #[test]
    fn test_self_maintenance_error() {
        use crate::gmbhg::Capital;

        let person = create_valid_person("Person", 30);

        let obligation = MaintenanceObligation {
            obligor: person.clone(),
            beneficiary: person,
            relationship: MaintenanceRelationship::ParentToChild,
            monthly_amount: Capital::from_euros(500),
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            end_date: None,
        };

        let result = validate_maintenance_obligation(&obligation);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::SelfMaintenance
        ));
    }
}
