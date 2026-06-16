//! Consumer Protection Law Validators (ການກວດສອບກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ)
//!
//! Validation functions for Lao consumer protection law based on the
//! **Law on Consumer Protection (Lao PDR), No. 02/NA, 2010**.
//!
//! Each validator returns `Ok(())` on compliance, or a
//! [`ConsumerProtectionError`] carrying bilingual messages and the governing
//! statute citation.

use crate::consumer_protection_law::error::{ConsumerProtectionError, ConsumerProtectionResult};
use crate::consumer_protection_law::types::*;

// ============================================================================
// Product Labelling Validators - ການກວດສອບສະຫຼາກສິນຄ້າ
// ============================================================================

/// Validate a product label.
/// ກວດສອບສະຫຼາກສິນຄ້າ
///
/// Enforces mandatory Lao-language labelling, manufacturer identification,
/// presence of required safety warnings, and chronological consistency of the
/// manufacture/expiry dates.
pub fn validate_product_label(label: &ProductLabel) -> ConsumerProtectionResult<()> {
    if label.product_name.trim().is_empty() {
        return Err(ConsumerProtectionError::InvalidProductLabel {
            provision: "product information",
            message_lao: "ຕ້ອງລະບຸຊື່ສິນຄ້າ".to_string(),
            message_en: "Product name is required on the label".to_string(),
        });
    }

    if !label.has_lao_language() {
        return Err(ConsumerProtectionError::MissingLaoLanguageLabel {
            message_lao: "ສະຫຼາກສິນຄ້າຕ້ອງມີພາສາລາວ".to_string(),
            message_en: "Product labelling must include the Lao language".to_string(),
        });
    }

    if !label.has_manufacturer_info {
        return Err(ConsumerProtectionError::InvalidProductLabel {
            provision: "product information",
            message_lao: "ສະຫຼາກຕ້ອງລະບຸຜູ້ຜະລິດ ຫຼື ຜູ້ນຳເຂົ້າ".to_string(),
            message_en: "The label must identify the manufacturer or importer".to_string(),
        });
    }

    if label.requires_safety_warnings && !label.has_safety_warnings {
        return Err(ConsumerProtectionError::InvalidProductLabel {
            provision: "consumer right to safety",
            message_lao: "ສິນຄ້ານີ້ຕ້ອງມີຄຳເຕືອນຄວາມປອດໄພ".to_string(),
            message_en: "This product requires safety warnings on the label".to_string(),
        });
    }

    if let (Some(made), Some(expiry)) = (&label.manufacture_date, &label.expiry_date)
        && expiry.as_str() <= made.as_str()
    {
        return Err(ConsumerProtectionError::InvalidProductLabel {
            provision: "product information",
            message_lao: "ວັນໝົດອາຍຸຕ້ອງຫຼັງວັນທີຜະລິດ".to_string(),
            message_en: "Expiry date must be after the manufacture date".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Advertising Validators - ການກວດສອບການໂຄສະນາ
// ============================================================================

/// Validate advertising for prohibited (false or misleading) content.
/// ກວດສອບການໂຄສະນາ
///
/// # Arguments
/// * `claim_substantiated` - whether the advertising claims are substantiated
/// * `contains_false_statement` - whether the advertisement contains a false statement
pub fn validate_advertising(
    claim_substantiated: bool,
    contains_false_statement: bool,
) -> ConsumerProtectionResult<()> {
    if contains_false_statement {
        return Err(ConsumerProtectionError::ProhibitedAdvertising {
            practice: "false advertising",
            message_lao: "ການໂຄສະນາມີຂໍ້ຄວາມທີ່ຕົວະ".to_string(),
            message_en: "Advertisement contains a false statement".to_string(),
        });
    }

    if !claim_substantiated {
        return Err(ConsumerProtectionError::ProhibitedAdvertising {
            practice: "misleading advertising",
            message_lao: "ການອ້າງອີງໃນການໂຄສະນາບໍ່ມີຫຼັກຖານຢືນຢັນ".to_string(),
            message_en: "Advertising claims are not substantiated".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Consumer Contract Validators - ການກວດສອບສັນຍາ
// ============================================================================

/// Validate a consumer contract for unfair terms and language accessibility.
/// ກວດສອບສັນຍາຜູ້ບໍລິໂພກ
pub fn validate_consumer_contract(contract: &ConsumerContract) -> ConsumerProtectionResult<()> {
    if contract.subject.trim().is_empty() {
        return Err(ConsumerProtectionError::ValidationError {
            message_lao: "ຕ້ອງລະບຸລາຍລະອຽດສິນຄ້າ/ບໍລິການ".to_string(),
            message_en: "Contract subject is required".to_string(),
        });
    }

    if !contract.available_in_lao {
        return Err(ConsumerProtectionError::SupplierObligationBreach {
            obligation: "provide accurate information",
            message_lao: "ສັນຍາຕ້ອງມີໃຫ້ເປັນພາສາລາວ".to_string(),
            message_en: "The contract must be made available in the Lao language".to_string(),
        });
    }

    if let Some(unfair) = contract.terms.iter().find(|term| term.is_unfair()) {
        return Err(ConsumerProtectionError::UnfairContractTerm {
            message_lao: format!("ສັນຍາມີຂໍ້ກຳນົດທີ່ບໍ່ເປັນທຳ: {}", unfair.lao_name()),
            message_en: format!("Contract contains an unfair term: {:?}", unfair),
        });
    }

    Ok(())
}

// ============================================================================
// Product Safety Validators - ການກວດສອບຄວາມປອດໄພ
// ============================================================================

/// Validate that a product is safe for sale to consumers.
/// ກວດສອບຄວາມປອດໄພຂອງສິນຄ້າ
pub fn validate_product_safety(
    assessment: &ProductSafetyAssessment,
) -> ConsumerProtectionResult<()> {
    if !assessment.meets_safety_standard {
        return Err(ConsumerProtectionError::UnsafeProduct {
            message_lao: format!("ສິນຄ້າ '{}' ບໍ່ໄດ້ມາດຕະຖານຄວາມປອດໄພ", assessment.product_name),
            message_en: format!(
                "Product '{}' does not meet the applicable safety standard",
                assessment.product_name
            ),
        });
    }

    if assessment.hazard_severity.requires_recall() && !assessment.recalled {
        return Err(ConsumerProtectionError::UnsafeProduct {
            message_lao: format!(
                "ສິນຄ້າ '{}' ມີ{} ແຕ່ຍັງບໍ່ໄດ້ເກັບຄືນ",
                assessment.product_name,
                assessment.hazard_severity.lao_name()
            ),
            message_en: format!(
                "Product '{}' presents a {:?} hazard and must be recalled",
                assessment.product_name, assessment.hazard_severity
            ),
        });
    }

    Ok(())
}

/// Validate a product recall.
/// ກວດສອບການເກັບຄືນສິນຄ້າ
pub fn validate_product_recall(recall: &ProductRecall) -> ConsumerProtectionResult<()> {
    if !recall.consumers_notified {
        return Err(ConsumerProtectionError::InvalidRecall {
            message_lao: "ການເກັບຄືນຕ້ອງແຈ້ງໃຫ້ຜູ້ບໍລິໂພກຮັບຮູ້".to_string(),
            message_en: "A recall must include public notification to consumers".to_string(),
        });
    }

    if recall.remedy == RedressType::None {
        return Err(ConsumerProtectionError::InvalidRecall {
            message_lao: "ການເກັບຄືນຕ້ອງສະເໜີການແກ້ໄຂໃຫ້ຜູ້ບໍລິໂພກ".to_string(),
            message_en: "A recall must offer a remedy to affected consumers".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Complaint, Redress & Dispute Validators - ການກວດສອບຄຳຮ້ອງທຸກ
// ============================================================================

/// Validate a consumer complaint is well-formed.
/// ກວດສອບຄຳຮ້ອງທຸກຂອງຜູ້ບໍລິໂພກ
pub fn validate_complaint(complaint: &ConsumerComplaint) -> ConsumerProtectionResult<()> {
    if complaint.consumer_name.trim().is_empty() || complaint.supplier_name.trim().is_empty() {
        return Err(ConsumerProtectionError::InvalidComplaint {
            provision: "consumer right to be heard",
            message_lao: "ຕ້ອງລະບຸຊື່ຜູ້ບໍລິໂພກ ແລະ ຜູ້ສະໜອງ".to_string(),
            message_en: "Both consumer and supplier must be identified".to_string(),
        });
    }

    if complaint.description_lao.trim().is_empty() && complaint.description_en.trim().is_empty() {
        return Err(ConsumerProtectionError::InvalidComplaint {
            provision: "consumer right to be heard",
            message_lao: "ຕ້ອງມີລາຍລະອຽດຄຳຮ້ອງທຸກ".to_string(),
            message_en: "The complaint must include a description of the grievance".to_string(),
        });
    }

    Ok(())
}

/// Validate that redress is adequate and internally consistent.
/// ກວດສອບການແກ້ໄຂ
///
/// A refund may not exceed the purchase price; monetary remedies must be
/// positive; offering no remedy is not valid redress.
pub fn validate_redress(redress: &Redress) -> ConsumerProtectionResult<()> {
    match redress.redress_type {
        RedressType::None => Err(ConsumerProtectionError::InvalidRedress {
            message_lao: "ຕ້ອງສະເໜີການແກ້ໄຂທີ່ເໝາະສົມ".to_string(),
            message_en: "A valid remedy must be offered".to_string(),
        }),
        RedressType::Refund => {
            if redress.amount_lak == 0 || redress.amount_lak > redress.purchase_price_lak {
                return Err(ConsumerProtectionError::InvalidRedress {
                    message_lao: "ຈຳນວນເງິນຄືນຕ້ອງຫຼາຍກວ່າ 0 ແລະ ບໍ່ເກີນລາຄາຊື້".to_string(),
                    message_en: "Refund must be positive and not exceed the purchase price"
                        .to_string(),
                });
            }
            Ok(())
        }
        RedressType::Compensation => {
            if redress.amount_lak == 0 {
                return Err(ConsumerProtectionError::InvalidRedress {
                    message_lao: "ຄ່າຊົດເຊີຍຕ້ອງຫຼາຍກວ່າ 0".to_string(),
                    message_en: "Compensation amount must be positive".to_string(),
                });
            }
            Ok(())
        }
        RedressType::Repair | RedressType::Replacement => Ok(()),
    }
}

/// Validate the escalation between two dispute resolution methods.
/// ກວດສອບການຍົກລະດັບການແກ້ໄຂຂໍ້ຂັດແຍ່ງ
///
/// Escalation must move to a strictly more formal method; a dispute cannot be
/// escalated "backwards" to a less formal one.
pub fn validate_dispute_escalation(
    current: DisputeResolutionMethod,
    next: DisputeResolutionMethod,
) -> ConsumerProtectionResult<()> {
    if next.escalation_order() <= current.escalation_order() {
        return Err(ConsumerProtectionError::ImproperDisputeEscalation {
            message_lao: format!(
                "ບໍ່ສາມາດຍົກລະດັບຈາກ {} ໄປ {}",
                current.lao_name(),
                next.lao_name()
            ),
            message_en: format!(
                "Cannot escalate from {:?} to {:?}; escalation must increase formality",
                current, next
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Supplier Obligation & Sanction Validators - ການກວດສອບພັນທະ ແລະ ການລົງໂທດ
// ============================================================================

/// Validate that a supplier has fulfilled a specific obligation.
/// ກວດສອບການປະຕິບັດພັນທະຂອງຜູ້ສະໜອງ
pub fn validate_supplier_obligation(
    obligation: SupplierObligation,
    fulfilled: bool,
) -> ConsumerProtectionResult<()> {
    if !fulfilled {
        return Err(ConsumerProtectionError::SupplierObligationBreach {
            obligation: obligation.english_name(),
            message_lao: format!("ຜູ້ສະໜອງບໍ່ໄດ້ປະຕິບັດພັນທະ: {}", obligation.lao_name()),
            message_en: format!("Supplier failed to {}", obligation.english_name()),
        });
    }

    Ok(())
}

/// Validate the proportionality of an administrative sanction.
/// ກວດສອບຄວາມສົມເຫດສົມຜົນຂອງການລົງໂທດ
///
/// The most severe sanctions (licence revocation, criminal referral) are
/// reserved for serious practices (e.g. unsafe goods) or repeat offences; a
/// first, minor infringement should attract a warning or fine.
pub fn validate_sanction(
    practice: ProhibitedPractice,
    sanction: SanctionType,
    is_repeat_offense: bool,
) -> ConsumerProtectionResult<()> {
    let practice_is_serious = matches!(
        practice,
        ProhibitedPractice::UnsafeGoods | ProhibitedPractice::ConcealmentOfDefects
    );

    let severe_sanction = matches!(
        sanction,
        SanctionType::LicenceRevocation | SanctionType::CriminalReferral
    );

    if severe_sanction && !practice_is_serious && !is_repeat_offense {
        return Err(ConsumerProtectionError::InvalidSanction {
            message_lao: "ການລົງໂທດຮ້າຍແຮງສະຫງວນໄວ້ສຳລັບການກະທຳຜິດຮ້າຍແຮງ ຫຼື ການກະທຳຜິດຊ້ຳ".to_string(),
            message_en: "Severe sanctions are reserved for serious or repeat infringements"
                .to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lao_label() -> ProductLabel {
        ProductLabel {
            product_name: "Bottled water".to_string(),
            languages: vec!["Lao".to_string()],
            has_manufacturer_info: true,
            manufacture_date: Some("2025-01-01".to_string()),
            expiry_date: Some("2027-01-01".to_string()),
            has_net_quantity: true,
            has_usage_instructions: true,
            has_safety_warnings: false,
            requires_safety_warnings: false,
        }
    }

    #[test]
    fn test_valid_label_passes() {
        assert!(validate_product_label(&lao_label()).is_ok());
    }

    #[test]
    fn test_missing_lao_language_fails() {
        let mut label = lao_label();
        label.languages = vec!["English".to_string()];
        let err = validate_product_label(&label).unwrap_err();
        assert!(matches!(
            err,
            ConsumerProtectionError::MissingLaoLanguageLabel { .. }
        ));
    }

    #[test]
    fn test_expiry_before_manufacture_fails() {
        let mut label = lao_label();
        label.manufacture_date = Some("2027-01-01".to_string());
        label.expiry_date = Some("2025-01-01".to_string());
        assert!(validate_product_label(&label).is_err());
    }

    #[test]
    fn test_required_safety_warning_missing_fails() {
        let mut label = lao_label();
        label.requires_safety_warnings = true;
        label.has_safety_warnings = false;
        assert!(validate_product_label(&label).is_err());
    }

    #[test]
    fn test_advertising_false_statement_fails() {
        assert!(validate_advertising(true, true).is_err());
    }

    #[test]
    fn test_advertising_unsubstantiated_fails() {
        assert!(validate_advertising(false, false).is_err());
    }

    #[test]
    fn test_advertising_ok() {
        assert!(validate_advertising(true, false).is_ok());
    }

    #[test]
    fn test_unfair_contract_term_rejected() {
        let contract = ConsumerContract {
            subject: "Smartphone purchase".to_string(),
            price_lak: 5_000_000,
            terms: vec![
                ContractTermType::Standard,
                ContractTermType::WaiverOfRedress,
            ],
            available_in_lao: true,
        };
        let err = validate_consumer_contract(&contract).unwrap_err();
        assert!(matches!(
            err,
            ConsumerProtectionError::UnfairContractTerm { .. }
        ));
    }

    #[test]
    fn test_contract_not_in_lao_rejected() {
        let contract = ConsumerContract {
            subject: "Smartphone purchase".to_string(),
            price_lak: 5_000_000,
            terms: vec![ContractTermType::Standard],
            available_in_lao: false,
        };
        assert!(validate_consumer_contract(&contract).is_err());
    }

    #[test]
    fn test_fair_contract_ok() {
        let contract = ConsumerContract {
            subject: "Smartphone purchase".to_string(),
            price_lak: 5_000_000,
            terms: vec![ContractTermType::Standard],
            available_in_lao: true,
        };
        assert!(validate_consumer_contract(&contract).is_ok());
    }

    #[test]
    fn test_unsafe_product_rejected() {
        let assessment = ProductSafetyAssessment {
            product_name: "Toy".to_string(),
            hazard_severity: HazardSeverity::Critical,
            meets_safety_standard: false,
            recalled: false,
        };
        assert!(validate_product_safety(&assessment).is_err());
    }

    #[test]
    fn test_serious_hazard_requires_recall() {
        let assessment = ProductSafetyAssessment {
            product_name: "Heater".to_string(),
            hazard_severity: HazardSeverity::Serious,
            meets_safety_standard: true,
            recalled: false,
        };
        assert!(validate_product_safety(&assessment).is_err());
    }

    #[test]
    fn test_safe_product_ok() {
        let assessment = ProductSafetyAssessment {
            product_name: "Notebook".to_string(),
            hazard_severity: HazardSeverity::None,
            meets_safety_standard: true,
            recalled: false,
        };
        assert!(validate_product_safety(&assessment).is_ok());
    }

    #[test]
    fn test_recall_requires_notification_and_remedy() {
        let recall = ProductRecall {
            product_name: "Heater".to_string(),
            hazard_severity: HazardSeverity::Serious,
            consumers_notified: false,
            remedy: RedressType::Refund,
        };
        assert!(validate_product_recall(&recall).is_err());

        let recall_no_remedy = ProductRecall {
            product_name: "Heater".to_string(),
            hazard_severity: HazardSeverity::Serious,
            consumers_notified: true,
            remedy: RedressType::None,
        };
        assert!(validate_product_recall(&recall_no_remedy).is_err());
    }

    #[test]
    fn test_valid_recall_ok() {
        let recall = ProductRecall {
            product_name: "Heater".to_string(),
            hazard_severity: HazardSeverity::Serious,
            consumers_notified: true,
            remedy: RedressType::Replacement,
        };
        assert!(validate_product_recall(&recall).is_ok());
    }

    #[test]
    fn test_complaint_validation() {
        let complaint = ConsumerComplaint {
            consumer_name: "Somchai".to_string(),
            supplier_name: "ABC Store".to_string(),
            description_lao: "ສິນຄ້າເພ".to_string(),
            description_en: "Defective product".to_string(),
            right_invoked: ConsumerRight::Redress,
            claimed_loss_lak: 1_000_000,
            resolution_method: DisputeResolutionMethod::Negotiation,
            requested_remedy: RedressType::Refund,
            status: ComplaintStatus::Received,
        };
        assert!(validate_complaint(&complaint).is_ok());
    }

    #[test]
    fn test_complaint_missing_parties_rejected() {
        let complaint = ConsumerComplaint {
            consumer_name: String::new(),
            supplier_name: "ABC Store".to_string(),
            description_lao: "ສິນຄ້າເພ".to_string(),
            description_en: "Defective product".to_string(),
            right_invoked: ConsumerRight::Redress,
            claimed_loss_lak: 1_000_000,
            resolution_method: DisputeResolutionMethod::Negotiation,
            requested_remedy: RedressType::Refund,
            status: ComplaintStatus::Received,
        };
        assert!(validate_complaint(&complaint).is_err());
    }

    #[test]
    fn test_refund_exceeding_price_rejected() {
        let redress = Redress {
            redress_type: RedressType::Refund,
            purchase_price_lak: 1_000_000,
            amount_lak: 2_000_000,
        };
        assert!(validate_redress(&redress).is_err());
    }

    #[test]
    fn test_valid_refund_ok() {
        let redress = Redress {
            redress_type: RedressType::Refund,
            purchase_price_lak: 1_000_000,
            amount_lak: 1_000_000,
        };
        assert!(validate_redress(&redress).is_ok());
    }

    #[test]
    fn test_no_remedy_is_invalid_redress() {
        let redress = Redress {
            redress_type: RedressType::None,
            purchase_price_lak: 1_000_000,
            amount_lak: 0,
        };
        assert!(validate_redress(&redress).is_err());
    }

    #[test]
    fn test_dispute_escalation_forward_ok() {
        assert!(
            validate_dispute_escalation(
                DisputeResolutionMethod::Negotiation,
                DisputeResolutionMethod::Mediation
            )
            .is_ok()
        );
    }

    #[test]
    fn test_dispute_escalation_backward_rejected() {
        assert!(
            validate_dispute_escalation(
                DisputeResolutionMethod::Litigation,
                DisputeResolutionMethod::Negotiation
            )
            .is_err()
        );
    }

    #[test]
    fn test_supplier_obligation_breach() {
        assert!(validate_supplier_obligation(SupplierObligation::ProductSafety, false).is_err());
        assert!(validate_supplier_obligation(SupplierObligation::ProductSafety, true).is_ok());
    }

    #[test]
    fn test_disproportionate_sanction_rejected() {
        // Severe sanction for a minor first offence is disproportionate.
        let err = validate_sanction(
            ProhibitedPractice::ShortMeasure,
            SanctionType::LicenceRevocation,
            false,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ConsumerProtectionError::InvalidSanction { .. }
        ));
    }

    #[test]
    fn test_severe_sanction_for_serious_practice_ok() {
        assert!(
            validate_sanction(
                ProhibitedPractice::UnsafeGoods,
                SanctionType::CriminalReferral,
                false
            )
            .is_ok()
        );
    }

    #[test]
    fn test_severe_sanction_for_repeat_offense_ok() {
        assert!(
            validate_sanction(
                ProhibitedPractice::FalseAdvertising,
                SanctionType::LicenceRevocation,
                true
            )
            .is_ok()
        );
    }
}
