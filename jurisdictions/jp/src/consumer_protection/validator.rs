//! Consumer Protection Validators (消費者保護法バリデータ)
//!
//! Validation functions for Japanese consumer protection law compliance.

use super::error::{ConsumerProtectionError, Result};
use super::types::*;
use chrono::Utc;

// ============================================================================
// Consumer Contract Validation (消費者契約法検証)
// ============================================================================

/// Validate consumer contract (消費者契約の検証)
///
/// Validates compliance with Consumer Contract Act, checking for:
/// - Unfair exemption clauses (Article 8)
/// - Excessive penalties/cancellation fees (Article 9)
/// - Consumer disadvantage clauses (Article 10)
///
/// # Arguments
/// * `contract` - Consumer contract to validate
///
/// # Returns
/// * `Ok(())` if contract is valid
/// * `Err(ConsumerProtectionError)` if contract contains unfair terms
pub fn validate_consumer_contract(contract: &ConsumerContract) -> Result<()> {
    // Validate required fields
    if contract.title.trim().is_empty() {
        return Err(ConsumerProtectionError::MissingRequiredField {
            field_name: "title".to_string(),
        });
    }

    if contract.business_name.trim().is_empty() {
        return Err(ConsumerProtectionError::MissingRequiredField {
            field_name: "business_name".to_string(),
        });
    }

    if contract.consumer_name.trim().is_empty() {
        return Err(ConsumerProtectionError::MissingRequiredField {
            field_name: "consumer_name".to_string(),
        });
    }

    if contract.terms.is_empty() {
        return Err(ConsumerProtectionError::InsufficientInformation {
            description: "Contract must have at least one term".to_string(),
        });
    }

    // Check each term for unfairness
    for term in &contract.terms {
        validate_contract_term(term)?;
    }

    // Check cancellation policy if present (Article 9)
    if let Some(cancellation) = &contract.cancellation_policy {
        if cancellation.is_fee_excessive(contract.contract_amount_jpy) {
            let percentage = if let Some(pct) = cancellation.cancellation_fee_percentage {
                pct * 100.0
            } else {
                (cancellation.cancellation_fee_jpy as f64 / contract.contract_amount_jpy as f64)
                    * 100.0
            };

            return Err(ConsumerProtectionError::ExcessiveCancellationFee {
                fee: cancellation.cancellation_fee_jpy,
                percentage,
            });
        }
    }

    // Check penalty clause if present (Article 9)
    if let Some(penalty) = &contract.penalty_clause {
        // Estimate average damages as 10% of contract value (simplified)
        let average_damages = contract.contract_amount_jpy / 10;

        if penalty.is_penalty_excessive(average_damages) {
            return Err(ConsumerProtectionError::ExcessivePenalty {
                penalty: penalty.penalty_amount_jpy,
                average: average_damages,
            });
        }
    }

    Ok(())
}

/// Validate individual contract term (契約条項の検証)
///
/// Checks if a contract term is potentially unfair.
///
/// # Arguments
/// * `term` - Contract term to validate
///
/// # Returns
/// * `Ok(())` if term appears fair
/// * `Err(ConsumerProtectionError)` if term is unfair
pub fn validate_contract_term(term: &ContractTerm) -> Result<()> {
    if term.text.trim().is_empty() {
        return Err(ConsumerProtectionError::InvalidContractTerms {
            reason: "Term text cannot be empty".to_string(),
        });
    }

    // Check if term is marked as potentially unfair
    if term.potentially_unfair {
        if let Some(unfair_type) = term.unfair_type {
            return match unfair_type {
                UnfairTermType::FullExemption => {
                    Err(ConsumerProtectionError::FullExemptionClause {
                        description: format!("Term {}: {}", term.term_number, term.text),
                    })
                }
                UnfairTermType::PartialExemption => {
                    Err(ConsumerProtectionError::PartialExemptionClause {
                        description: format!("Term {}: {}", term.term_number, term.text),
                    })
                }
                UnfairTermType::ConsumerDisadvantage => {
                    Err(ConsumerProtectionError::ConsumerDisadvantageClause {
                        description: format!("Term {}: {}", term.term_number, term.text),
                    })
                }
                UnfairTermType::UnreasonableBurden => {
                    Err(ConsumerProtectionError::UnreasonableBurden {
                        description: format!("Term {}: {}", term.term_number, term.text),
                    })
                }
                _ => Ok(()), // Already handled via other checks
            };
        }
    }

    // Check risk score
    if term.risk_score > 70 {
        return Err(ConsumerProtectionError::ConsumerDisadvantageClause {
            description: format!(
                "Term {} has high risk score: {}",
                term.term_number, term.risk_score
            ),
        });
    }

    Ok(())
}

/// Detect unfair terms in contract text (不当条項検出)
///
/// Analyzes contract text to detect potentially unfair terms.
/// Returns risk score (0-100) and detected unfair term type.
///
/// # Arguments
/// * `term_text` - Contract term text to analyze
///
/// # Returns
/// * `(risk_score, potentially_unfair, unfair_type)`
pub fn detect_unfair_terms(term_text: &str) -> (u32, bool, Option<UnfairTermType>) {
    let text_lower = term_text.to_lowercase();
    let mut risk_score = 0u32;
    let mut unfair_type = None;

    // Full exemption clause detection (Article 8-1-1)
    let full_exemption_keywords = [
        "一切責任を負いません",
        "責任を負わない",
        "all liability is excluded",
        "no responsibility",
        "免責",
        "責任なし",
    ];

    for keyword in &full_exemption_keywords {
        if text_lower.contains(keyword) {
            risk_score += 40;
            unfair_type = Some(UnfairTermType::FullExemption);
            break;
        }
    }

    // Partial exemption detection (Article 8-1-2/3)
    let partial_exemption_keywords = [
        "故意又は重過失を除き",
        "軽過失については",
        "except for willful",
        "一定の責任",
    ];

    if unfair_type.is_none() {
        for keyword in &partial_exemption_keywords {
            if text_lower.contains(keyword) {
                risk_score += 25;
                unfair_type = Some(UnfairTermType::PartialExemption);
                break;
            }
        }
    }

    // Excessive penalty/cancellation fee keywords
    let penalty_keywords = [
        "違約金",
        "損害賠償",
        "解除料",
        "penalty",
        "cancellation fee",
        "liquidated damages",
    ];

    let has_penalty = penalty_keywords.iter().any(|k| text_lower.contains(k));

    if has_penalty {
        // Check for specific amounts that might be excessive
        if text_lower.contains("100%") || text_lower.contains("全額") {
            risk_score += 35;
            if unfair_type.is_none() {
                unfair_type = Some(UnfairTermType::ExcessivePenalty);
            }
        } else if text_lower.contains("50%") || text_lower.contains("半額") {
            risk_score += 20;
        }
    }

    // Consumer disadvantage keywords (Article 10)
    let disadvantage_keywords = [
        "消費者の負担",
        "一方的に",
        "当社の裁量",
        "consumer bears all",
        "at our discretion",
        "消費者のみ",
        "事業者は責任を負わ",
    ];

    if unfair_type.is_none() {
        for keyword in &disadvantage_keywords {
            if text_lower.contains(keyword) {
                risk_score += 30;
                unfair_type = Some(UnfairTermType::ConsumerDisadvantage);
                break;
            }
        }
    }

    // Unreasonable burden keywords
    let burden_keywords = [
        "即時支払",
        "一括払い",
        "期限の利益喪失",
        "immediate payment",
        "lump sum",
        "全額負担",
    ];

    if unfair_type.is_none() {
        for keyword in &burden_keywords {
            if text_lower.contains(keyword) {
                risk_score += 25;
                unfair_type = Some(UnfairTermType::UnreasonableBurden);
                break;
            }
        }
    }

    // Additional risk factors
    if text_lower.len() > 500 {
        risk_score += 5; // Very long clauses can hide unfair terms
    }

    if text_lower.contains("ただし") || text_lower.contains("except") {
        risk_score += 5; // Exception clauses require careful review
    }

    let potentially_unfair = risk_score >= 30;

    (risk_score.min(100), potentially_unfair, unfair_type)
}

/// Analyze contract for unfair terms (契約の不当条項分析)
///
/// Performs comprehensive analysis of all contract terms.
///
/// # Arguments
/// * `contract` - Contract to analyze
///
/// # Returns
/// * Updated contract with risk scores and unfair term flags
pub fn analyze_contract_terms(mut contract: ConsumerContract) -> ConsumerContract {
    for term in &mut contract.terms {
        let (risk_score, potentially_unfair, unfair_type) = detect_unfair_terms(&term.text);

        term.risk_score = risk_score;
        term.potentially_unfair = potentially_unfair;

        if unfair_type.is_some() && term.unfair_type.is_none() {
            term.unfair_type = unfair_type;
        }
    }

    contract
}

// ============================================================================
// Rescission Validation (取消権検証)
// ============================================================================

/// Validate rescission claim (取消権行使の検証)
///
/// Validates if rescission claim is valid under Article 4-7.
///
/// # Arguments
/// * `claim` - Rescission claim to validate
///
/// # Returns
/// * `Ok(())` if rescission is valid
/// * `Err(ConsumerProtectionError)` if rescission is invalid
pub fn validate_rescission_claim(claim: &RescissionClaim) -> Result<()> {
    // Check if within rescission period (Article 7)
    if !claim.is_within_rescission_period() {
        let months_since_contract =
            (claim.rescission_date - claim.contract.contract_date).num_days() / 30;
        return Err(ConsumerProtectionError::RescissionPeriodExpired {
            months_since_contract,
        });
    }

    // Validate description is provided
    if claim.description.trim().len() < 20 {
        return Err(ConsumerProtectionError::InsufficientInformation {
            description: "Rescission grounds must be described in detail".to_string(),
        });
    }

    // Validate based on specific grounds
    match claim.ground {
        RescissionGround::Misrepresentation => {
            if !claim.description.to_lowercase().contains("misrepresent")
                && !claim.description.contains("不実")
            {
                return Err(ConsumerProtectionError::Misrepresentation {
                    description: "Misrepresentation not clearly described".to_string(),
                });
            }
        }
        RescissionGround::NonDisclosure => {
            if claim.description.to_lowercase().contains("disclose")
                || claim.description.contains("告知")
            {
                // Description mentions disclosure
            } else {
                return Err(ConsumerProtectionError::NonDisclosure {
                    description: "Non-disclosure not clearly described".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

// ============================================================================
// Cooling-Off Validation (クーリング・オフ検証)
// ============================================================================

/// Validate specified commercial transaction (特定商取引の検証)
///
/// Validates transaction under Specified Commercial Transactions Act.
///
/// # Arguments
/// * `transaction` - Transaction to validate
///
/// # Returns
/// * `Ok(())` if transaction is valid
/// * `Err(ConsumerProtectionError)` if transaction violates SCTA
pub fn validate_specified_transaction(transaction: &SpecifiedCommercialTransaction) -> Result<()> {
    // Validate required fields
    if transaction.seller_name.trim().is_empty() {
        return Err(ConsumerProtectionError::MissingRequiredField {
            field_name: "seller_name".to_string(),
        });
    }

    if transaction.purchaser_name.trim().is_empty() {
        return Err(ConsumerProtectionError::MissingRequiredField {
            field_name: "purchaser_name".to_string(),
        });
    }

    if transaction.product_description.trim().is_empty() {
        return Err(ConsumerProtectionError::InsufficientInformation {
            description: "Product/service description required".to_string(),
        });
    }

    // Check if cooling-off notice was provided when required
    if transaction.transaction_type.has_cooling_off() && !transaction.cooling_off_notice_provided {
        return Err(ConsumerProtectionError::CoolingOffNoticeNotProvided);
    }

    // Check if contract documents were provided
    if transaction.document_receipt_date.is_none()
        && transaction.transaction_type != TransactionType::MailOrder
    {
        return Err(ConsumerProtectionError::RequiredDisclosureNotMade);
    }

    Ok(())
}

/// Validate cooling-off exercise (クーリング・オフ行使の検証)
///
/// Validates if cooling-off was properly exercised.
///
/// # Arguments
/// * `exercise` - Cooling-off exercise to validate
///
/// # Returns
/// * `Ok(())` if cooling-off exercise is valid
/// * `Err(ConsumerProtectionError)` if exercise is invalid
pub fn validate_cooling_off_exercise(exercise: &CoolingOffExercise) -> Result<()> {
    // First validate the underlying transaction
    validate_specified_transaction(&exercise.transaction)?;

    // Check if cooling-off applies to this transaction type
    if !exercise.transaction.transaction_type.has_cooling_off() {
        return Err(ConsumerProtectionError::CoolingOffNotApplicable {
            transaction_type: format!("{:?}", exercise.transaction.transaction_type),
        });
    }

    // Check if exercised within the period
    if !exercise.is_timely() {
        let deadline = exercise
            .transaction
            .cooling_off_deadline()
            .map(|d| format!("{}", d))
            .unwrap_or_else(|| "unknown".to_string());

        return Err(ConsumerProtectionError::CoolingOffExpired { deadline });
    }

    // Check if notification was sent
    if !exercise.notification_sent {
        return Err(ConsumerProtectionError::ImproperCoolingOffExercise {
            reason: "Notification must be sent to seller".to_string(),
        });
    }

    // Validate notification method is reasonable
    if exercise.notification_method.trim().is_empty() {
        return Err(ConsumerProtectionError::ImproperCoolingOffExercise {
            reason: "Notification method must be specified".to_string(),
        });
    }

    Ok(())
}

/// Calculate cooling-off deadline with grace period (クーリング・オフ期限計算)
///
/// Returns cooling-off information including deadline and days remaining.
///
/// # Arguments
/// * `transaction` - Transaction to check
///
/// # Returns
/// * `(deadline, days_remaining, is_valid)`
pub fn get_cooling_off_info(
    transaction: &SpecifiedCommercialTransaction,
) -> Option<(chrono::DateTime<Utc>, i64, bool)> {
    if let Some(deadline) = transaction.cooling_off_deadline() {
        let days_remaining = transaction.cooling_off_days_remaining().unwrap_or(0);
        let is_valid = transaction.is_within_cooling_off_period();
        Some((deadline, days_remaining, is_valid))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_detect_unfair_terms_full_exemption() {
        let text = "当社は一切責任を負いません。";
        let (risk_score, potentially_unfair, unfair_type) = detect_unfair_terms(text);

        assert!(risk_score >= 40);
        assert!(potentially_unfair);
        assert_eq!(unfair_type, Some(UnfairTermType::FullExemption));
    }

    #[test]
    fn test_detect_unfair_terms_consumer_disadvantage() {
        let text = "消費者の負担において一方的に契約を変更できる。";
        let (risk_score, potentially_unfair, unfair_type) = detect_unfair_terms(text);

        assert!(risk_score >= 30);
        assert!(potentially_unfair);
        assert_eq!(unfair_type, Some(UnfairTermType::ConsumerDisadvantage));
    }

    #[test]
    fn test_detect_unfair_terms_fair_clause() {
        let text = "商品は7日以内に配送されます。";
        let (risk_score, potentially_unfair, _) = detect_unfair_terms(text);

        assert!(risk_score < 30);
        assert!(!potentially_unfair);
    }

    #[test]
    fn test_validate_consumer_contract_excessive_cancellation() {
        let contract = ConsumerContract {
            title: "Test Contract".to_string(),
            business_name: "Business".to_string(),
            consumer_name: "Consumer".to_string(),
            contract_date: Utc::now(),
            contract_amount_jpy: 100_000,
            terms: vec![ContractTerm {
                term_number: 1,
                text: "Standard term".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 10,
            }],
            cancellation_policy: Some(CancellationPolicy {
                cancellation_fee_jpy: 30_000,
                cancellation_fee_percentage: Some(0.30),
                notice_period_days: 30,
                description: "Test".to_string(),
            }),
            penalty_clause: None,
        };

        assert!(matches!(
            validate_consumer_contract(&contract),
            Err(ConsumerProtectionError::ExcessiveCancellationFee { .. })
        ));
    }

    #[test]
    fn test_validate_cooling_off_within_period() {
        let transaction = SpecifiedCommercialTransaction {
            transaction_type: TransactionType::DoorToDoor,
            seller_name: "Seller".to_string(),
            purchaser_name: "Buyer".to_string(),
            contract_date: Utc::now(),
            document_receipt_date: Some(Utc::now()),
            contract_amount_jpy: 100_000,
            product_description: "Product".to_string(),
            payment_method: "Cash".to_string(),
            cooling_off_notice_provided: true,
        };

        let exercise = CoolingOffExercise {
            transaction: transaction.clone(),
            exercise_date: Utc::now() + Duration::days(3),
            notification_method: "Registered mail".to_string(),
            notification_sent: true,
        };

        assert!(validate_cooling_off_exercise(&exercise).is_ok());
    }

    #[test]
    fn test_validate_cooling_off_expired() {
        let transaction = SpecifiedCommercialTransaction {
            transaction_type: TransactionType::DoorToDoor,
            seller_name: "Seller".to_string(),
            purchaser_name: "Buyer".to_string(),
            contract_date: Utc::now() - Duration::days(20),
            document_receipt_date: Some(Utc::now() - Duration::days(20)),
            contract_amount_jpy: 100_000,
            product_description: "Product".to_string(),
            payment_method: "Cash".to_string(),
            cooling_off_notice_provided: true,
        };

        let exercise = CoolingOffExercise {
            transaction,
            exercise_date: Utc::now(),
            notification_method: "Mail".to_string(),
            notification_sent: true,
        };

        assert!(matches!(
            validate_cooling_off_exercise(&exercise),
            Err(ConsumerProtectionError::CoolingOffExpired { .. })
        ));
    }

    #[test]
    fn test_validate_rescission_claim() {
        let contract = ConsumerContract {
            title: "Test".to_string(),
            business_name: "Business".to_string(),
            consumer_name: "Consumer".to_string(),
            contract_date: Utc::now() - Duration::days(30),
            contract_amount_jpy: 100_000,
            terms: vec![],
            cancellation_policy: None,
            penalty_clause: None,
        };

        let claim = RescissionClaim {
            contract,
            ground: RescissionGround::Misrepresentation,
            rescission_date: Utc::now(),
            description: "The seller made false misrepresentations about the product quality"
                .to_string(),
            evidence_description: Some("Photographs and expert report".to_string()),
        };

        assert!(validate_rescission_claim(&claim).is_ok());
    }
}
