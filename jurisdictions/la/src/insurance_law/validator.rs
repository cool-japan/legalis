//! Insurance Law Validators (ການກວດສອບກົດໝາຍປະກັນໄພ)
//!
//! Validation functions for Lao insurance law based on the
//! **Law on Insurance (Lao PDR), No. 06/NA, 2011**.
//!
//! Each validator returns `Ok(())` on compliance, or an [`InsuranceLawError`]
//! carrying bilingual messages and the governing statute citation.

use crate::insurance_law::error::{InsuranceLawError, InsuranceResult};
use crate::insurance_law::types::*;

// ============================================================================
// Insurer Licensing & Solvency Validators - ການກວດສອບໃບອະນຸຍາດ ແລະ ຄວາມສາມາດຊຳລະໜີ້
// ============================================================================

/// Validate that an insurer is properly licensed, capitalised and solvent.
/// ກວດສອບໃບອະນຸຍາດ, ທຶນ ແລະ ຄວາມສາມາດຊຳລະໜີ້ຂອງບໍລິສັດປະກັນໄພ
///
/// Enforces that the insurer is licensed by the Ministry of Finance, holds
/// positive registered capital, and satisfies the solvency principle.
pub fn validate_insurer_license(insurer: &Insurer) -> InsuranceResult<()> {
    if insurer.name.trim().is_empty() {
        return Err(InsuranceLawError::InvalidInsurerLicense {
            provision: "insurer licensing",
            message_lao: "ຕ້ອງລະບຸຊື່ບໍລິສັດປະກັນໄພ".to_string(),
            message_en: "Insurer name is required".to_string(),
        });
    }

    if !insurer.licensed {
        return Err(InsuranceLawError::InvalidInsurerLicense {
            provision: "insurer licensing",
            message_lao: "ບໍລິສັດປະກັນໄພຕ້ອງໄດ້ຮັບໃບອະນຸຍາດຈາກກະຊວງການເງິນ".to_string(),
            message_en: "Insurer must be licensed by the Ministry of Finance".to_string(),
        });
    }

    if insurer.registered_capital_lak == 0 {
        return Err(InsuranceLawError::InvalidInsurerLicense {
            provision: "registered capital",
            message_lao: "ບໍລິສັດປະກັນໄພຕ້ອງມີທຶນຈົດທະບຽນຫຼາຍກວ່າ 0".to_string(),
            message_en: "Insurer must hold positive registered capital".to_string(),
        });
    }

    validate_solvency(insurer)?;

    Ok(())
}

/// Validate that an insurer meets the solvency requirement.
/// ກວດສອບຄວາມສາມາດຊຳລະໜີ້ຂອງບໍລິສັດປະກັນໄພ
///
/// The solvency principle requires admitted assets to be at least equal to
/// liabilities (see [`MIN_SOLVENCY_RATIO_PERCENT`]).
pub fn validate_solvency(insurer: &Insurer) -> InsuranceResult<()> {
    if !insurer.is_solvent() {
        return Err(InsuranceLawError::InsolventInsurer {
            message_lao: "ຊັບສິນທີ່ຮັບຮູ້ໄດ້ຕ້ອງບໍ່ໜ້ອຍກວ່າໜີ້ສິນ (ຫຼັກການຄວາມສາມາດຊຳລະໜີ້)".to_string(),
            message_en:
                "Admitted assets must be at least equal to liabilities (solvency requirement)"
                    .to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Insurance Policy Validators - ການກວດສອບສັນຍາປະກັນໄພ
// ============================================================================

/// Validate the essential elements of an insurance policy.
/// ກວດສອບອົງປະກອບສຳຄັນຂອງສັນຍາປະກັນໄພ
///
/// Enforces the presence of an insurable interest, a positive sum insured and
/// premium, and chronological consistency of the policy term.
pub fn validate_policy(policy: &InsurancePolicy) -> InsuranceResult<()> {
    if policy.policyholder.trim().is_empty() {
        return Err(InsuranceLawError::InvalidPolicy {
            provision: "policy formation",
            message_lao: "ຕ້ອງລະບຸຊື່ຜູ້ເອົາປະກັນໄພ".to_string(),
            message_en: "Policyholder is required".to_string(),
        });
    }

    if !policy.insurable_interest {
        return Err(InsuranceLawError::NoInsurableInterest {
            message_lao: "ສັນຍາປະກັນໄພຕ້ອງມີຜົນປະໂຫຍດທີ່ສາມາດເອົາປະກັນໄພໄດ້".to_string(),
            message_en: "An insurance policy requires an insurable interest".to_string(),
        });
    }

    if policy.sum_insured_lak == 0 {
        return Err(InsuranceLawError::InvalidPolicy {
            provision: "sum insured",
            message_lao: "ຈຳນວນເງິນເອົາປະກັນໄພຕ້ອງຫຼາຍກວ່າ 0".to_string(),
            message_en: "Sum insured must be greater than zero".to_string(),
        });
    }

    if policy.premium_lak == 0 {
        return Err(InsuranceLawError::InvalidPolicy {
            provision: "premium",
            message_lao: "ເບ້ຍປະກັນໄພຕ້ອງຫຼາຍກວ່າ 0".to_string(),
            message_en: "Premium must be greater than zero".to_string(),
        });
    }

    if policy.end_date.as_str() <= policy.start_date.as_str() {
        return Err(InsuranceLawError::InvalidPolicy {
            provision: "policy duration",
            message_lao: "ວັນທີສິ້ນສຸດສັນຍາຕ້ອງຫຼັງວັນທີເລີ່ມຕົ້ນ".to_string(),
            message_en: "Policy end date must be after the start date".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Insurance Claim Validators - ການກວດສອບການຮຽກຮ້ອງຄ່າສິນໄໝ
// ============================================================================

/// Validate an insurance claim.
/// ກວດສອບການຮຽກຮ້ອງຄ່າສິນໄໝ
///
/// A fraudulent claim is rejected, a claim must be notified to the insurer, and
/// indemnity claims are additionally checked against the principle of indemnity.
pub fn validate_claim(claim: &InsuranceClaim) -> InsuranceResult<()> {
    if claim.fraudulent {
        return Err(InsuranceLawError::FraudulentClaim {
            message_lao: "ການຮຽກຮ້ອງຄ່າສິນໄໝທີ່ສໍ້ໂກງຈະຖືກປະຕິເສດ".to_string(),
            message_en: "A fraudulent claim is rejected".to_string(),
        });
    }

    if !claim.notified {
        return Err(InsuranceLawError::InvalidClaim {
            provision: "claim notification",
            message_lao: "ການຮຽກຮ້ອງຄ່າສິນໄໝຕ້ອງໄດ້ຮັບການແຈ້ງຕໍ່ບໍລິສັດປະກັນໄພ".to_string(),
            message_en: "A claim must be notified to the insurer".to_string(),
        });
    }

    if claim.is_indemnity {
        validate_indemnity_principle(claim)?;
    }

    Ok(())
}

/// Validate the principle of indemnity for an indemnity (non-life) claim.
/// ກວດສອບຫຼັກການຊົດໃຊ້ຄ່າເສຍຫາຍ
///
/// For indemnity insurance the payout may not exceed the actual loss, and may not
/// exceed the sum insured. Non-indemnity claims are not constrained by this rule.
pub fn validate_indemnity_principle(claim: &InsuranceClaim) -> InsuranceResult<()> {
    if !claim.is_indemnity {
        return Ok(());
    }

    if claim.claim_amount_lak > claim.actual_loss_lak {
        return Err(InsuranceLawError::IndemnityExceeded {
            message_lao: "ຄ່າສິນໄໝທົດແທນຕ້ອງບໍ່ເກີນຄວາມເສຍຫາຍຕົວຈິງ (ຫຼັກການຊົດໃຊ້ຄ່າເສຍຫາຍ)".to_string(),
            message_en: "Indemnity payout must not exceed the actual loss (principle of indemnity)"
                .to_string(),
        });
    }

    if claim.claim_amount_lak > claim.sum_insured_lak {
        return Err(InsuranceLawError::IndemnityExceeded {
            message_lao: "ຄ່າສິນໄໝທົດແທນຕ້ອງບໍ່ເກີນຈຳນວນເງິນເອົາປະກັນໄພ".to_string(),
            message_en: "Indemnity payout must not exceed the sum insured".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Compulsory Insurance Validators - ການກວດສອບການປະກັນໄພທີ່ບັງຄັບ
// ============================================================================

/// Validate that compulsory insurance is in place where required.
/// ກວດສອບການມີຢູ່ຂອງການປະກັນໄພທີ່ບັງຄັບ
///
/// If the class is compulsory (e.g. motor third-party liability) and no cover is
/// in place, validation fails.
pub fn validate_compulsory_insurance(
    insurance_class: InsuranceClass,
    is_insured: bool,
) -> InsuranceResult<()> {
    if insurance_class.is_compulsory() && !is_insured {
        return Err(InsuranceLawError::CompulsoryInsuranceMissing {
            provision: "compulsory motor third-party liability insurance",
            message_lao: format!("{} ເປັນການບັງຄັບ ແຕ່ບໍ່ມີການເອົາປະກັນໄພ", insurance_class.lao_name()),
            message_en: format!(
                "{} is compulsory but no cover is in place",
                insurance_class.english_name()
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Intermediary Validators - ການກວດສອບຕົວກາງປະກັນໄພ
// ============================================================================

/// Validate that an insurance intermediary is licensed.
/// ກວດສອບໃບອະນຸຍາດຂອງຕົວກາງປະກັນໄພ
pub fn validate_intermediary(intermediary: &Intermediary) -> InsuranceResult<()> {
    if intermediary.name.trim().is_empty() {
        return Err(InsuranceLawError::ValidationError {
            message_lao: "ຕ້ອງລະບຸຊື່ຕົວກາງປະກັນໄພ".to_string(),
            message_en: "Intermediary name is required".to_string(),
        });
    }

    if !intermediary.licensed {
        return Err(InsuranceLawError::UnlicensedIntermediary {
            message_lao: format!(
                "{} ຕ້ອງໄດ້ຮັບໃບອະນຸຍາດ",
                intermediary.intermediary_type.lao_name()
            ),
            message_en: format!(
                "{} must be licensed",
                intermediary.intermediary_type.english_name()
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solvent_insurer() -> Insurer {
        Insurer {
            name: "Lao Insurance Co".to_string(),
            insurer_type: InsurerType::NonLifeInsurer,
            registered_capital_lak: 50_000_000_000,
            licensed: true,
            admitted_assets_lak: 100_000_000_000,
            liabilities_lak: 60_000_000_000,
        }
    }

    fn valid_policy() -> InsurancePolicy {
        InsurancePolicy {
            policyholder: "Somchai".to_string(),
            insurance_class: InsuranceClass::Motor,
            insurable_interest: true,
            sum_insured_lak: 50_000_000,
            premium_lak: 1_200_000,
            is_indemnity: true,
            start_date: "2025-01-01".to_string(),
            end_date: "2026-01-01".to_string(),
            status: PolicyStatus::Active,
        }
    }

    fn valid_indemnity_claim() -> InsuranceClaim {
        InsuranceClaim {
            insurance_class: InsuranceClass::Motor,
            sum_insured_lak: 50_000_000,
            actual_loss_lak: 10_000_000,
            claim_amount_lak: 10_000_000,
            is_indemnity: true,
            notified: true,
            fraudulent: false,
            status: ClaimStatus::Notified,
        }
    }

    // ---- Insurer licensing & solvency -------------------------------------

    #[test]
    fn test_valid_insurer_license_ok() {
        assert!(validate_insurer_license(&solvent_insurer()).is_ok());
    }

    #[test]
    fn test_insurer_empty_name_fails() {
        let mut insurer = solvent_insurer();
        insurer.name = String::new();
        let err = validate_insurer_license(&insurer).unwrap_err();
        assert!(matches!(
            err,
            InsuranceLawError::InvalidInsurerLicense { .. }
        ));
    }

    #[test]
    fn test_unlicensed_insurer_fails() {
        let mut insurer = solvent_insurer();
        insurer.licensed = false;
        let err = validate_insurer_license(&insurer).unwrap_err();
        assert!(matches!(
            err,
            InsuranceLawError::InvalidInsurerLicense { .. }
        ));
    }

    #[test]
    fn test_zero_capital_insurer_fails() {
        let mut insurer = solvent_insurer();
        insurer.registered_capital_lak = 0;
        assert!(validate_insurer_license(&insurer).is_err());
    }

    #[test]
    fn test_insolvent_insurer_fails_license() {
        let mut insurer = solvent_insurer();
        insurer.liabilities_lak = 200_000_000_000;
        let err = validate_insurer_license(&insurer).unwrap_err();
        assert!(matches!(err, InsuranceLawError::InsolventInsurer { .. }));
    }

    #[test]
    fn test_validate_solvency_ok() {
        assert!(validate_solvency(&solvent_insurer()).is_ok());
    }

    #[test]
    fn test_validate_solvency_fails() {
        let mut insurer = solvent_insurer();
        insurer.admitted_assets_lak = 10_000_000_000;
        insurer.liabilities_lak = 60_000_000_000;
        assert!(validate_solvency(&insurer).is_err());
    }

    // ---- Policy ------------------------------------------------------------

    #[test]
    fn test_valid_policy_ok() {
        assert!(validate_policy(&valid_policy()).is_ok());
    }

    #[test]
    fn test_policy_empty_policyholder_fails() {
        let mut policy = valid_policy();
        policy.policyholder = String::new();
        assert!(validate_policy(&policy).is_err());
    }

    #[test]
    fn test_policy_no_insurable_interest_fails() {
        let mut policy = valid_policy();
        policy.insurable_interest = false;
        let err = validate_policy(&policy).unwrap_err();
        assert!(matches!(err, InsuranceLawError::NoInsurableInterest { .. }));
    }

    #[test]
    fn test_policy_zero_sum_insured_fails() {
        let mut policy = valid_policy();
        policy.sum_insured_lak = 0;
        assert!(validate_policy(&policy).is_err());
    }

    #[test]
    fn test_policy_zero_premium_fails() {
        let mut policy = valid_policy();
        policy.premium_lak = 0;
        assert!(validate_policy(&policy).is_err());
    }

    #[test]
    fn test_policy_end_before_start_fails() {
        let mut policy = valid_policy();
        policy.start_date = "2026-01-01".to_string();
        policy.end_date = "2025-01-01".to_string();
        let err = validate_policy(&policy).unwrap_err();
        assert!(matches!(err, InsuranceLawError::InvalidPolicy { .. }));
    }

    // ---- Claim -------------------------------------------------------------

    #[test]
    fn test_valid_claim_ok() {
        assert!(validate_claim(&valid_indemnity_claim()).is_ok());
    }

    #[test]
    fn test_fraudulent_claim_rejected() {
        let mut claim = valid_indemnity_claim();
        claim.fraudulent = true;
        let err = validate_claim(&claim).unwrap_err();
        assert!(matches!(err, InsuranceLawError::FraudulentClaim { .. }));
    }

    #[test]
    fn test_unnotified_claim_rejected() {
        let mut claim = valid_indemnity_claim();
        claim.notified = false;
        let err = validate_claim(&claim).unwrap_err();
        assert!(matches!(err, InsuranceLawError::InvalidClaim { .. }));
    }

    #[test]
    fn test_indemnity_exceeds_actual_loss_rejected() {
        let mut claim = valid_indemnity_claim();
        claim.actual_loss_lak = 5_000_000;
        claim.claim_amount_lak = 10_000_000;
        let err = validate_claim(&claim).unwrap_err();
        assert!(matches!(err, InsuranceLawError::IndemnityExceeded { .. }));
    }

    #[test]
    fn test_indemnity_exceeds_sum_insured_rejected() {
        let mut claim = valid_indemnity_claim();
        claim.sum_insured_lak = 8_000_000;
        claim.actual_loss_lak = 20_000_000;
        claim.claim_amount_lak = 10_000_000;
        let err = validate_indemnity_principle(&claim).unwrap_err();
        assert!(matches!(err, InsuranceLawError::IndemnityExceeded { .. }));
    }

    #[test]
    fn test_indemnity_principle_ok() {
        assert!(validate_indemnity_principle(&valid_indemnity_claim()).is_ok());
    }

    #[test]
    fn test_non_indemnity_claim_above_loss_ok() {
        // A non-indemnity (e.g. life) claim is not constrained by actual loss.
        let mut claim = valid_indemnity_claim();
        claim.insurance_class = InsuranceClass::Life;
        claim.is_indemnity = false;
        claim.actual_loss_lak = 0;
        claim.claim_amount_lak = 50_000_000;
        assert!(validate_claim(&claim).is_ok());
        assert!(validate_indemnity_principle(&claim).is_ok());
    }

    // ---- Compulsory insurance ---------------------------------------------

    #[test]
    fn test_compulsory_motor_missing_fails() {
        let err = validate_compulsory_insurance(InsuranceClass::Motor, false).unwrap_err();
        assert!(matches!(
            err,
            InsuranceLawError::CompulsoryInsuranceMissing { .. }
        ));
    }

    #[test]
    fn test_compulsory_motor_insured_ok() {
        assert!(validate_compulsory_insurance(InsuranceClass::Motor, true).is_ok());
    }

    #[test]
    fn test_non_compulsory_uninsured_ok() {
        assert!(validate_compulsory_insurance(InsuranceClass::Life, false).is_ok());
        assert!(validate_compulsory_insurance(InsuranceClass::Property, false).is_ok());
    }

    // ---- Intermediary ------------------------------------------------------

    #[test]
    fn test_valid_intermediary_ok() {
        let intermediary = Intermediary {
            name: "Vientiane Brokers".to_string(),
            intermediary_type: IntermediaryType::Broker,
            licensed: true,
        };
        assert!(validate_intermediary(&intermediary).is_ok());
    }

    #[test]
    fn test_unlicensed_intermediary_fails() {
        let intermediary = Intermediary {
            name: "Unlicensed Agent".to_string(),
            intermediary_type: IntermediaryType::Agent,
            licensed: false,
        };
        let err = validate_intermediary(&intermediary).unwrap_err();
        assert!(matches!(
            err,
            InsuranceLawError::UnlicensedIntermediary { .. }
        ));
    }

    #[test]
    fn test_intermediary_empty_name_fails() {
        let intermediary = Intermediary {
            name: "   ".to_string(),
            intermediary_type: IntermediaryType::Agent,
            licensed: true,
        };
        let err = validate_intermediary(&intermediary).unwrap_err();
        assert!(matches!(err, InsuranceLawError::ValidationError { .. }));
    }
}
