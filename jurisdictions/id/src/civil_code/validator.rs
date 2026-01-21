//! Validation functions for Indonesian Civil Code

use super::error::{CivilCodeError, CivilCodeResult};
use super::types::*;
use serde::{Deserialize, Serialize};

/// Validate legal capacity to contract - Pasal 1329-1331
pub fn validate_legal_capacity(capacity: &LegalCapacity) -> CivilCodeResult<()> {
    match capacity {
        LegalCapacity::Full => Ok(()),
        LegalCapacity::Minor { age, has_guardian } => {
            if *age >= LegalCapacity::minimum_age() {
                Ok(())
            } else if !has_guardian {
                Err(CivilCodeError::Incapacity {
                    description: format!(
                        "Orang yang belum dewasa (usia {} tahun) dan tidak mempunyai wali",
                        age
                    ),
                })
            } else {
                Err(CivilCodeError::Incapacity {
                    description: format!(
                        "Orang yang belum dewasa (usia {} tahun, di bawah perwalian)",
                        age
                    ),
                })
            }
        }
        LegalCapacity::UnderGuardianship { reason } => Err(CivilCodeError::Incapacity {
            description: format!("Orang yang ditaruh di bawah pengampuan: {:?}", reason),
        }),
        LegalCapacity::MarriedWoman => {
            // Historical - now equal capacity
            Ok(())
        }
    }
}

/// Validate contract validity requirements - Pasal 1320
pub fn validate_contract_validity(validity: &ContractValidity) -> CivilCodeResult<()> {
    // Check requirement 1: Agreement
    if !validity.has_agreement {
        return Err(CivilCodeError::MissingAgreement {
            description: "Tidak ada kata sepakat".to_string(),
        });
    }

    if !validity.agreement_free_from_defects {
        return Err(CivilCodeError::MissingAgreement {
            description: "Kesepakatan mengandung cacat kehendak".to_string(),
        });
    }

    // Check requirement 2: Capacity
    if !validity.parties_have_capacity {
        return Err(CivilCodeError::Incapacity {
            description: "Salah satu pihak tidak cakap".to_string(),
        });
    }

    // Check requirement 3: Specific object (objective)
    if !validity.has_specific_object || !validity.object_is_determinable {
        return Err(CivilCodeError::NoSpecificObject);
    }

    // Check requirement 4: Lawful cause (objective)
    if !validity.has_lawful_cause {
        return Err(CivilCodeError::UnlawfulCause {
            description: "Perjanjian tidak mempunyai sebab".to_string(),
        });
    }

    if !validity.not_contrary_to_law {
        return Err(CivilCodeError::UnlawfulCause {
            description: "Sebab bertentangan dengan undang-undang".to_string(),
        });
    }

    Ok(())
}

/// Validate contract formation
pub fn validate_contract_formation(formation: &ContractFormation) -> CivilCodeResult<()> {
    // Check if acceptance exists
    if formation.acceptance_date.is_none() {
        return Err(CivilCodeError::MissingAgreement {
            description: "Penawaran belum diterima".to_string(),
        });
    }

    // Check consideration
    if formation.consideration.is_empty() || formation.counter_consideration.is_empty() {
        return Err(CivilCodeError::NoSpecificObject);
    }

    Ok(())
}

/// Contract compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCompliance {
    /// Overall compliance
    pub compliant: bool,
    /// Validity status
    pub validity_status: String,
    /// Agreement valid
    pub agreement_valid: bool,
    /// Parties have capacity
    pub parties_capable: bool,
    /// Object specific
    pub object_specific: bool,
    /// Cause lawful
    pub cause_lawful: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive contract compliance check
pub fn validate_contract_compliance(contract: &Contract) -> ContractCompliance {
    let mut compliance = ContractCompliance {
        compliant: true,
        validity_status: "Valid".to_string(),
        agreement_valid: true,
        parties_capable: true,
        object_specific: true,
        cause_lawful: true,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check validity
    let validity_status = contract.validity.validity_status();
    match validity_status {
        ContractValidityStatus::Valid => {
            compliance.validity_status = "Sah (Valid)".to_string();
        }
        ContractValidityStatus::Voidable(reason) => {
            compliance.compliant = false;
            compliance.validity_status = format!("Dapat Dibatalkan: {}", reason);
            compliance.issues.push(reason.clone());
            compliance.recommendations.push(
                "Perjanjian dapat dibatalkan atas permintaan pihak yang dirugikan dalam waktu 5 tahun"
                    .to_string(),
            );
        }
        ContractValidityStatus::VoidAbInitio(reason) => {
            compliance.compliant = false;
            compliance.validity_status = format!("Batal Demi Hukum: {}", reason);
            compliance.issues.push(reason.clone());
            compliance.recommendations.push(
                "Perjanjian dianggap tidak pernah ada sejak awal (batal demi hukum)".to_string(),
            );
        }
    }

    // Check party capacities
    if !contract.party_a.capacity.can_contract() {
        compliance.compliant = false;
        compliance.parties_capable = false;
        compliance
            .issues
            .push(format!("Pihak A ({}) tidak cakap", contract.party_a.name));
    }

    if !contract.party_b.capacity.can_contract() {
        compliance.compliant = false;
        compliance.parties_capable = false;
        compliance
            .issues
            .push(format!("Pihak B ({}) tidak cakap", contract.party_b.name));
    }

    // Check formation
    if contract.formation.acceptance_date.is_none() {
        compliance.compliant = false;
        compliance.agreement_valid = false;
        compliance
            .issues
            .push("Penerimaan penawaran belum ada".to_string());
    }

    // Check object
    if contract.object.is_empty() {
        compliance.compliant = false;
        compliance.object_specific = false;
        compliance
            .issues
            .push("Objek perjanjian tidak ditentukan".to_string());
    }

    compliance
}

/// Get contract validity checklist
pub fn get_contract_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Kesepakatan kedua belah pihak",
            "Agreement of both parties",
            "Pasal 1320(1)",
        ),
        (
            "Bebas dari paksaan, penipuan, kekhilafan",
            "Free from duress, fraud, mistake",
            "Pasal 1321-1328",
        ),
        (
            "Kecakapan para pihak",
            "Capacity of parties",
            "Pasal 1320(2), 1329-1331",
        ),
        (
            "Hal tertentu sebagai objek",
            "Specific object",
            "Pasal 1320(3), 1332-1334",
        ),
        (
            "Sebab yang halal",
            "Lawful cause",
            "Pasal 1320(4), 1335-1337",
        ),
        (
            "Tidak bertentangan dengan undang-undang",
            "Not contrary to law",
            "Pasal 1337",
        ),
        (
            "Tidak bertentangan dengan kesusilaan",
            "Not contrary to morality",
            "Pasal 1337",
        ),
        (
            "Tidak bertentangan dengan ketertiban umum",
            "Not contrary to public order",
            "Pasal 1337",
        ),
        (
            "Itikad baik dalam pelaksanaan",
            "Good faith in performance",
            "Pasal 1338(3)",
        ),
        (
            "Bentuk sesuai ketentuan (jika disyaratkan)",
            "Form requirements met (if required)",
            "Various",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_legal_capacity_full() {
        let result = validate_legal_capacity(&LegalCapacity::Full);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_legal_capacity_minor() {
        let result = validate_legal_capacity(&LegalCapacity::Minor {
            age: 17,
            has_guardian: false,
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_contract_validity_valid() {
        let validity = ContractValidity {
            has_agreement: true,
            agreement_free_from_defects: true,
            parties_have_capacity: true,
            has_specific_object: true,
            object_is_determinable: true,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        assert!(validate_contract_validity(&validity).is_ok());
    }

    #[test]
    fn test_validate_contract_validity_no_agreement() {
        let validity = ContractValidity {
            has_agreement: false,
            agreement_free_from_defects: true,
            parties_have_capacity: true,
            has_specific_object: true,
            object_is_determinable: true,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        let result = validate_contract_validity(&validity);
        assert!(matches!(
            result,
            Err(CivilCodeError::MissingAgreement { .. })
        ));
    }

    #[test]
    fn test_validate_formation() {
        let formation = ContractFormation {
            offer_date: Utc::now(),
            acceptance_date: Some(Utc::now()),
            formation_date: None,
            consideration: "Payment".to_string(),
            counter_consideration: "Goods".to_string(),
            formation_method: FormationMethod::Written,
        };

        assert!(validate_contract_formation(&formation).is_ok());
    }

    #[test]
    fn test_contract_checklist() {
        let checklist = get_contract_checklist();
        assert!(!checklist.is_empty());
        assert!(
            checklist
                .iter()
                .any(|(id, _, _)| id.contains("Kesepakatan"))
        );
    }
}
