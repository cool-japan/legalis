//! Managed Investments Validators

use super::error::{ManagedInvestmentsError, Result};
use super::types::*;

/// Validate managed investment scheme
pub fn validate_scheme(scheme: &ManagedInvestmentScheme) -> Result<()> {
    // Validate responsible entity
    validate_responsible_entity(&scheme.responsible_entity)?;

    // Validate compliance plan
    validate_compliance_plan(&scheme.compliance_plan)?;

    Ok(())
}

/// Validate responsible entity (s.601FC)
pub fn validate_responsible_entity(re: &ResponsibleEntity) -> Result<()> {
    // Check adequate resources
    if !re.adequate_resources {
        return Err(ManagedInvestmentsError::ReRequirementsNotMet {
            re_name: re.name.clone(),
            reason: "Inadequate resources".to_string(),
        });
    }

    // Check risk management
    if !re.risk_management {
        return Err(ManagedInvestmentsError::ReRequirementsNotMet {
            re_name: re.name.clone(),
            reason: "No risk management framework".to_string(),
        });
    }

    // Check compliance framework
    if !re.compliance_framework {
        return Err(ManagedInvestmentsError::ReRequirementsNotMet {
            re_name: re.name.clone(),
            reason: "No compliance framework".to_string(),
        });
    }

    // Check RG 259 compliance
    if !re.rg259_compliant {
        return Err(ManagedInvestmentsError::ReRequirementsNotMet {
            re_name: re.name.clone(),
            reason: "Does not comply with ASIC RG 259 requirements".to_string(),
        });
    }

    Ok(())
}

/// Validate compliance plan (s.601HA)
pub fn validate_compliance_plan(plan: &CompliancePlan) -> Result<()> {
    // Check lodged with ASIC
    if !plan.lodged_with_asic {
        return Err(ManagedInvestmentsError::CompliancePlanNotLodged);
    }

    // Check act compliance measures
    if !plan.act_compliance_measures {
        return Err(ManagedInvestmentsError::CompliancePlanDeficient {
            deficiency: "No measures to ensure compliance with Corporations Act".to_string(),
        });
    }

    // Check constitution compliance measures
    if !plan.constitution_compliance_measures {
        return Err(ManagedInvestmentsError::CompliancePlanDeficient {
            deficiency: "No measures to ensure compliance with constitution".to_string(),
        });
    }

    // Check regular reviews
    if !plan.regular_reviews {
        return Err(ManagedInvestmentsError::CompliancePlanDeficient {
            deficiency: "No regular review process".to_string(),
        });
    }

    // Check auditor appointed
    if !plan.auditor_appointed {
        return Err(ManagedInvestmentsError::CompliancePlanDeficient {
            deficiency: "No compliance plan auditor appointed".to_string(),
        });
    }

    // Validate compliance committee if present
    if let Some(ref committee) = plan.compliance_committee
        && !committee.has_external_majority()
    {
        return Err(ManagedInvestmentsError::ComplianceCommitteeComposition);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_valid_re() -> ResponsibleEntity {
        ResponsibleEntity {
            name: "Test RE Pty Ltd".to_string(),
            abn: "12345678901".to_string(),
            afsl_number: "123456".to_string(),
            registration_date: NaiveDate::from_ymd_opt(2015, 1, 1).unwrap(),
            adequate_resources: true,
            risk_management: true,
            compliance_framework: true,
            rg259_compliant: true,
        }
    }

    fn create_valid_compliance_plan() -> CompliancePlan {
        CompliancePlan {
            lodged_with_asic: true,
            lodgement_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            act_compliance_measures: true,
            constitution_compliance_measures: true,
            regular_reviews: true,
            review_frequency_months: 12,
            last_review_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            compliance_committee: Some(ComplianceCommittee {
                members: 3,
                external_members: 2,
                meets_quarterly: true,
                reports_to_board: true,
            }),
            auditor_appointed: true,
            auditor_name: Some("Big4 Audit".to_string()),
        }
    }

    #[test]
    fn test_validate_responsible_entity_success() {
        let re = create_valid_re();
        assert!(validate_responsible_entity(&re).is_ok());
    }

    #[test]
    fn test_validate_responsible_entity_no_resources() {
        let mut re = create_valid_re();
        re.adequate_resources = false;

        let result = validate_responsible_entity(&re);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ManagedInvestmentsError::ReRequirementsNotMet { .. })
        ));
    }

    #[test]
    fn test_validate_compliance_plan_success() {
        let plan = create_valid_compliance_plan();
        assert!(validate_compliance_plan(&plan).is_ok());
    }

    #[test]
    fn test_validate_compliance_plan_not_lodged() {
        let mut plan = create_valid_compliance_plan();
        plan.lodged_with_asic = false;

        let result = validate_compliance_plan(&plan);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ManagedInvestmentsError::CompliancePlanNotLodged)
        ));
    }

    #[test]
    fn test_validate_compliance_committee_composition() {
        let mut plan = create_valid_compliance_plan();
        plan.compliance_committee = Some(ComplianceCommittee {
            members: 4,
            external_members: 2, // Not majority
            meets_quarterly: true,
            reports_to_board: true,
        });

        let result = validate_compliance_plan(&plan);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ManagedInvestmentsError::ComplianceCommitteeComposition)
        ));
    }
}
