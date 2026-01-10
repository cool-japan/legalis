//! Validation functions for French evidence law

use super::error::{EvidenceLawError, EvidenceLawResult};
use super::types::{BurdenOfProof, Evidence, EvidenceType, PresumptionType};

/// Validates evidence according to French evidence law
pub fn validate_evidence(evidence: &Evidence) -> EvidenceLawResult<()> {
    if evidence.description.is_empty() {
        return Err(EvidenceLawError::InvalidEvidenceType {
            evidence_type: format!("{:?}", evidence.evidence_type),
            reason: "Evidence must have description".to_string(),
        });
    }

    match &evidence.evidence_type {
        EvidenceType::WrittenDocument { electronic, signed } => {
            if *electronic && !evidence.authenticity_verified {
                return Err(EvidenceLawError::ElectronicEvidenceDefect {
                    defect: "Electronic evidence not verified (Articles 1366-1378)".to_string(),
                });
            }
            if *signed && !evidence.authenticity_verified {
                return Err(EvidenceLawError::ElectronicEvidenceDefect {
                    defect: "Signed document authenticity not verified".to_string(),
                });
            }
        }
        EvidenceType::WitnessTestimony { witness } => {
            if witness.witness_name.is_empty() {
                return Err(EvidenceLawError::WitnessCredibilityIssue {
                    witness: "unknown".to_string(),
                    issue: "Witness name required".to_string(),
                });
            }
        }
        EvidenceType::ExpertReport { expert } => {
            if expert.expert_name.is_empty() {
                return Err(EvidenceLawError::ExpertReportDefect {
                    expert: "unknown".to_string(),
                    defect: "Expert name required (CPC 227-229)".to_string(),
                });
            }
            if expert.field.is_empty() {
                return Err(EvidenceLawError::ExpertReportDefect {
                    expert: expert.expert_name.clone(),
                    defect: "Expert field of expertise required".to_string(),
                });
            }
        }
        EvidenceType::Confession { party, .. } => {
            if party.is_empty() {
                return Err(EvidenceLawError::InvalidConfession {
                    reason: "Confessing party must be identified".to_string(),
                });
            }
        }
        EvidenceType::Oath { party, .. } => {
            if party.is_empty() {
                return Err(EvidenceLawError::OathProcedureViolation {
                    violation: "Party taking oath must be identified".to_string(),
                });
            }
        }
        EvidenceType::Presumption { .. } => {
            // Presumption validation done by validate_presumption
        }
    }

    Ok(())
}

/// Validates burden of proof allocation (Article 1353)
pub fn validate_burden_of_proof(burden: &BurdenOfProof) -> EvidenceLawResult<()> {
    if burden.claimant_must_prove.is_empty() && burden.defendant_must_prove.is_empty() {
        return Err(EvidenceLawError::BurdenNotMet {
            party: "both".to_string(),
            missing_facts: vec!["No burden allocation specified".to_string()],
        });
    }

    // Claimant typically has initial burden under Article 1353
    if burden.claimant_must_prove.is_empty() {
        return Err(EvidenceLawError::BurdenNotMet {
            party: "claimant".to_string(),
            missing_facts: vec![
                "Claimant must prove existence of obligation (Article 1353)".to_string(),
            ],
        });
    }

    Ok(())
}

/// Validates presumption applicability (Article 1354)
pub fn validate_presumption(
    presumption_type: PresumptionType,
    basis_fact_proven: bool,
) -> EvidenceLawResult<()> {
    if !basis_fact_proven {
        return Err(EvidenceLawError::PresumptionNotApplicable {
            presumption: format!("{:?}", presumption_type),
            reason: "Basis fact not proven (Article 1354 requires basis fact)".to_string(),
        });
    }

    // All presumption types are valid if basis fact proven
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::types::{ExpertReport, WitnessTestimony};
    use chrono::NaiveDate;

    #[test]
    fn test_validate_written_document() {
        let evidence = Evidence::new(
            EvidenceType::WrittenDocument {
                electronic: true,
                signed: true,
            },
            "Contract".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_ok());
    }

    #[test]
    fn test_validate_electronic_not_verified() {
        let evidence = Evidence::new(
            EvidenceType::WrittenDocument {
                electronic: true,
                signed: false,
            },
            "Email".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            false,
        );
        let result = validate_evidence(&evidence);
        assert!(result.is_err());
        match result.unwrap_err() {
            EvidenceLawError::ElectronicEvidenceDefect { .. } => {}
            _ => panic!("Expected ElectronicEvidenceDefect"),
        }
    }

    #[test]
    fn test_validate_witness_testimony() {
        let witness = WitnessTestimony::new(
            "Jean Dupont".to_string(),
            true,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        );
        let evidence = Evidence::new(
            EvidenceType::WitnessTestimony { witness },
            "Testimony".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_ok());
    }

    #[test]
    fn test_validate_witness_no_name() {
        let witness = WitnessTestimony::new(
            "".to_string(),
            true,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        );
        let evidence = Evidence::new(
            EvidenceType::WitnessTestimony { witness },
            "Testimony".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_err());
    }

    #[test]
    fn test_validate_expert_report() {
        let expert = ExpertReport::new(
            "Dr. Smith".to_string(),
            "Engineering".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        let evidence = Evidence::new(
            EvidenceType::ExpertReport { expert },
            "Expert report".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_ok());
    }

    #[test]
    fn test_validate_expert_no_field() {
        let expert = ExpertReport::new(
            "Dr. Smith".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        let evidence = Evidence::new(
            EvidenceType::ExpertReport { expert },
            "Expert report".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_err());
    }

    #[test]
    fn test_validate_burden_of_proof_valid() {
        let burden = BurdenOfProof::new()
            .with_claimant_burden("Contract existence".to_string())
            .with_defendant_burden("Payment".to_string());
        assert!(validate_burden_of_proof(&burden).is_ok());
    }

    #[test]
    fn test_validate_burden_empty() {
        let burden = BurdenOfProof::new();
        assert!(validate_burden_of_proof(&burden).is_err());
    }

    #[test]
    fn test_validate_burden_no_claimant() {
        let burden = BurdenOfProof::new().with_defendant_burden("Payment".to_string());
        assert!(validate_burden_of_proof(&burden).is_err());
    }

    #[test]
    fn test_validate_presumption_simple() {
        assert!(validate_presumption(PresumptionType::Simple, true).is_ok());
    }

    #[test]
    fn test_validate_presumption_no_basis() {
        let result = validate_presumption(PresumptionType::Simple, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            EvidenceLawError::PresumptionNotApplicable { .. } => {}
            _ => panic!("Expected PresumptionNotApplicable"),
        }
    }

    #[test]
    fn test_validate_confession() {
        let evidence = Evidence::new(
            EvidenceType::Confession {
                party: "Defendant".to_string(),
                judicial: true,
            },
            "Confession".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_ok());
    }

    #[test]
    fn test_validate_confession_no_party() {
        let evidence = Evidence::new(
            EvidenceType::Confession {
                party: "".to_string(),
                judicial: true,
            },
            "Confession".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_err());
    }

    #[test]
    fn test_validate_oath() {
        let evidence = Evidence::new(
            EvidenceType::Oath {
                party: "Plaintiff".to_string(),
                decisive: true,
            },
            "Oath".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(validate_evidence(&evidence).is_ok());
    }
}
