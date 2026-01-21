//! Financial Advice Validators

use super::error::{AdviceError, Result};
use super::types::*;

/// Validate best interests duty compliance (s.961B)
pub fn validate_best_interests_duty(assessment: &BestInterestsAssessment) -> Result<()> {
    // Only applies to personal advice
    if !assessment.advice_type.best_interests_duty_applies() {
        return Ok(());
    }

    // Check safe harbour steps
    for step in &assessment.safe_harbour_steps {
        if !step.is_completed() {
            return Err(AdviceError::SafeHarbourStepMissing {
                step: step.description().to_string(),
            });
        }
    }

    // Check objectives identified
    if !assessment.objectives_identified {
        return Err(AdviceError::BestInterestsDutyBreach {
            client_name: assessment.client_name.clone(),
            details: "Client objectives not identified".to_string(),
        });
    }

    // Check financial situation assessed
    if !assessment.financial_situation_assessed {
        return Err(AdviceError::BestInterestsDutyBreach {
            client_name: assessment.client_name.clone(),
            details: "Client financial situation not assessed".to_string(),
        });
    }

    // Check needs identified
    if !assessment.needs_identified {
        return Err(AdviceError::BestInterestsDutyBreach {
            client_name: assessment.client_name.clone(),
            details: "Client needs not identified".to_string(),
        });
    }

    // Check product investigation
    if !assessment.product_investigation {
        return Err(AdviceError::BestInterestsDutyBreach {
            client_name: assessment.client_name.clone(),
            details: "Reasonable product investigation not conducted".to_string(),
        });
    }

    // Check priority given to client
    if !assessment.client_priority {
        return Err(AdviceError::PriorityRuleBreach {
            details: format!(
                "Adviser did not give priority to {}'s interests",
                assessment.client_name
            ),
        });
    }

    // Check overall compliance
    if !assessment.compliant {
        return Err(AdviceError::BestInterestsDutyBreach {
            client_name: assessment.client_name.clone(),
            details: assessment
                .non_compliance_details
                .clone()
                .unwrap_or_else(|| "Non-compliant advice".to_string()),
        });
    }

    Ok(())
}

/// Validate Financial Services Guide (s.941A-942C)
pub fn validate_fsg(fsg: &FinancialServicesGuide) -> Result<()> {
    // Check provided to client
    if !fsg.provided_to_client {
        return Err(AdviceError::FsgNotProvided);
    }

    // Check required content
    if !fsg.services_described {
        return Err(AdviceError::FsgDeficient {
            deficiency: "Services not described".to_string(),
        });
    }

    if !fsg.remuneration_disclosed {
        return Err(AdviceError::FsgDeficient {
            deficiency: "Remuneration not disclosed".to_string(),
        });
    }

    if !fsg.dispute_resolution_info {
        return Err(AdviceError::FsgDeficient {
            deficiency: "Dispute resolution information not included".to_string(),
        });
    }

    if !fsg.compensation_arrangements {
        return Err(AdviceError::FsgDeficient {
            deficiency: "Compensation arrangements not disclosed".to_string(),
        });
    }

    Ok(())
}

/// Validate Product Disclosure Statement (s.1012A-1013L)
pub fn validate_pds(pds: &ProductDisclosureStatement) -> Result<()> {
    // Check provided to client
    if !pds.provided_to_client {
        return Err(AdviceError::PdsNotProvided);
    }

    // Check required content
    if !pds.features_described {
        return Err(AdviceError::PdsDeficient {
            deficiency: "Product features not described".to_string(),
        });
    }

    if !pds.fees_disclosed {
        return Err(AdviceError::PdsDeficient {
            deficiency: "Fees not disclosed".to_string(),
        });
    }

    if !pds.risks_disclosed {
        return Err(AdviceError::PdsDeficient {
            deficiency: "Risks not disclosed".to_string(),
        });
    }

    if !pds.complaints_handling {
        return Err(AdviceError::PdsDeficient {
            deficiency: "Complaints handling information not included".to_string(),
        });
    }

    Ok(())
}

/// Validate Statement of Advice (s.946A-947D)
pub fn validate_soa(soa: &StatementOfAdvice) -> Result<()> {
    // Check provided to client
    if !soa.provided_to_client {
        return Err(AdviceError::SoaNotProvided);
    }

    // Check required content
    if !soa.advice_summary {
        return Err(AdviceError::SoaDeficient {
            deficiency: "Advice summary not included".to_string(),
        });
    }

    if !soa.basis_explained {
        return Err(AdviceError::SoaDeficient {
            deficiency: "Basis for advice not explained".to_string(),
        });
    }

    if !soa.information_disclosed {
        return Err(AdviceError::SoaDeficient {
            deficiency: "Information relied on not disclosed".to_string(),
        });
    }

    if !soa.remuneration_disclosed {
        return Err(AdviceError::SoaDeficient {
            deficiency: "Remuneration not disclosed".to_string(),
        });
    }

    Ok(())
}

/// Validate conflicted remuneration (s.963E)
pub fn validate_conflicted_remuneration(remuneration: &ConflictedRemuneration) -> Result<()> {
    // Check if remuneration type is prohibited
    if remuneration.remuneration_type.is_generally_prohibited() && !remuneration.is_permitted {
        return Err(AdviceError::ConflictedRemuneration {
            description: remuneration.description.clone(),
            amount: remuneration.amount_aud,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_best_interests_duty_success() {
        let assessment = BestInterestsAssessment {
            client_name: "John Smith".to_string(),
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            advice_type: AdviceType::Personal,
            safe_harbour_steps: vec![
                SafeHarbourStep::IdentifyClientCircumstances { completed: true },
                SafeHarbourStep::IdentifySubjectMatter { completed: true },
                SafeHarbourStep::ReasonableInvestigation {
                    completed: true,
                    products_considered: 5,
                },
                SafeHarbourStep::EnsureAppropriate { completed: true },
                SafeHarbourStep::ReasonableAssessment { completed: true },
                SafeHarbourStep::ConsiderRecommendation { completed: true },
                SafeHarbourStep::OtherInquiries { completed: true },
            ],
            objectives_identified: true,
            financial_situation_assessed: true,
            needs_identified: true,
            product_investigation: true,
            recommendation_appropriate: true,
            client_priority: true,
            compliant: true,
            non_compliance_details: None,
        };

        assert!(validate_best_interests_duty(&assessment).is_ok());
    }

    #[test]
    fn test_validate_best_interests_duty_missing_step() {
        let assessment = BestInterestsAssessment {
            client_name: "John Smith".to_string(),
            assessment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            advice_type: AdviceType::Personal,
            safe_harbour_steps: vec![SafeHarbourStep::ReasonableInvestigation {
                completed: false, // Not completed
                products_considered: 0,
            }],
            objectives_identified: true,
            financial_situation_assessed: true,
            needs_identified: true,
            product_investigation: true,
            recommendation_appropriate: true,
            client_priority: true,
            compliant: true,
            non_compliance_details: None,
        };

        let result = validate_best_interests_duty(&assessment);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AdviceError::SafeHarbourStepMissing { .. })
        ));
    }

    #[test]
    fn test_validate_fsg() {
        let fsg = FinancialServicesGuide {
            issuer_name: "Test Pty Ltd".to_string(),
            afsl_number: "123456".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            services_described: true,
            remuneration_disclosed: true,
            associations_disclosed: true,
            dispute_resolution_info: true,
            compensation_arrangements: true,
            provided_to_client: true,
            provision_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
        };

        assert!(validate_fsg(&fsg).is_ok());
    }

    #[test]
    fn test_validate_conflicted_remuneration() {
        let prohibited = ConflictedRemuneration {
            description: "Volume bonus".to_string(),
            amount_aud: 5000.0,
            source: "Product issuer".to_string(),
            remuneration_type: RemunerationType::VolumeBased,
            is_permitted: false,
            exemption_reason: None,
        };

        let result = validate_conflicted_remuneration(&prohibited);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(AdviceError::ConflictedRemuneration { .. })
        ));

        let permitted = ConflictedRemuneration {
            description: "Flat fee".to_string(),
            amount_aud: 2000.0,
            source: "Client".to_string(),
            remuneration_type: RemunerationType::FlatFee,
            is_permitted: true,
            exemption_reason: None,
        };

        assert!(validate_conflicted_remuneration(&permitted).is_ok());
    }
}
