//! Consumer Protection - Validation Logic

use super::error::{ConsumerError, Result};
use super::types::*;

/// Validates a consumer contract
pub fn validate_consumer_contract(contract: &ConsumerContract) -> Result<()> {
    // Check risk score
    if contract.risk_score > 70 {
        return Err(ConsumerError::HighRiskContract {
            risk_score: contract.risk_score,
        });
    }

    // Check for potentially unfair terms
    for term in &contract.terms {
        if term.is_potentially_unfair {
            return Err(ConsumerError::UnfairTerm {
                term_description: term.description.clone(),
            });
        }
    }

    // Check SCT eligibility if amount is large
    let amount_sgd = contract.amount_cents / 100;
    if amount_sgd > 20_000 {
        return Err(ConsumerError::ExceedsSctLimit { amount: amount_sgd });
    }

    Ok(())
}

/// Detects unfair practices in a contract
pub fn detect_unfair_practices(contract: &ConsumerContract) -> Vec<UnfairPractice> {
    let mut practices = Vec::new();
    let desc = contract.description.to_lowercase();

    // Check for keywords indicating false representation
    if desc.contains("guaranteed") || desc.contains("100%") || desc.contains("miracle") {
        let mut practice = UnfairPractice::new(
            format!("ufp-{}", practices.len() + 1),
            UnfairPracticeType::FalseRepresentation,
            "Potentially exaggerated claims in description",
        );
        practice.add_evidence(format!("Description contains: {}", contract.description));
        practices.push(practice);
    }

    // Check for bait advertising indicators
    if desc.contains("limited time") || desc.contains("while stocks last") {
        // This is actually legitimate, but we flag for review
        let mut practice = UnfairPractice::new(
            format!("ufp-{}", practices.len() + 1),
            UnfairPracticeType::BaitAdvertising,
            "Time-limited or stock-limited offers",
        );
        practice.severity = 3; // Lower severity
        practice.add_evidence("Description mentions limited availability");
        practices.push(practice);
    }

    // Check for unconscionable conduct indicators in terms
    for term in &contract.terms {
        if matches!(term.category, TermCategory::LiabilityLimitation) {
            let term_lower = term.description.to_lowercase();
            if term_lower.contains("no liability") || term_lower.contains("not responsible") {
                let mut practice = UnfairPractice::new(
                    format!("ufp-{}", practices.len() + 1),
                    UnfairPracticeType::UnconscionableConduct,
                    "Potentially excessive liability limitation",
                );
                practice.add_evidence(format!("Term: {}", term.description));
                practices.push(practice);
            }
        }

        if matches!(term.category, TermCategory::ReturnRefund) {
            let term_lower = term.description.to_lowercase();
            if term_lower.contains("no refund") || term_lower.contains("non-refundable") {
                let mut practice = UnfairPractice::new(
                    format!("ufp-{}", practices.len() + 1),
                    UnfairPracticeType::UnconscionableConduct,
                    "Potentially unfair refund restriction",
                );
                practice.severity = 4;
                practice.add_evidence(format!("Term: {}", term.description));
                practices.push(practice);
            }
        }
    }

    practices
}

/// Validates a sale of goods contract
pub fn validate_sale_of_goods(sale: &SaleOfGoods) -> Result<()> {
    // Check if goods are defective
    if sale.is_defective
        && let Some(ref defect) = sale.defect_description
    {
        // Check if Lemon Law applies
        if sale.is_lemon_law_applicable() {
            return Err(ConsumerError::DefectDiscovered {
                description: defect.clone(),
            });
        }

        // Check implied terms
        if sale
            .implied_terms
            .contains(&ImpliedTerm::MerchantableQuality)
        {
            return Err(ConsumerError::NotMerchantable {
                description: defect.clone(),
            });
        }
    }

    Ok(())
}

/// Validates implied terms compliance
pub fn validate_implied_terms(sale: &SaleOfGoods) -> Result<Vec<ImpliedTerm>> {
    let mut applicable_terms = Vec::new();

    // s. 13 always applies
    applicable_terms.push(ImpliedTerm::CorrespondsToDescription);

    // s. 14(2) applies if seller in business
    if sale.seller_in_business {
        applicable_terms.push(ImpliedTerm::MerchantableQuality);
    }

    // s. 14(3) applies if particular purpose communicated
    if sale.particular_purpose.is_some() {
        applicable_terms.push(ImpliedTerm::FitnessForPurpose);
    }

    // s. 15 applies if sale by sample
    if sale.sale_by_sample {
        applicable_terms.push(ImpliedTerm::SaleBySample);
    }

    Ok(applicable_terms)
}

/// Validates warranty coverage
pub fn validate_warranty(warranty: &WarrantyTerms, days_since_purchase: u32) -> Result<()> {
    if days_since_purchase > warranty.duration_days {
        return Err(ConsumerError::WarrantyExpired {
            days_ago: days_since_purchase - warranty.duration_days,
        });
    }

    Ok(())
}

/// Detects specific unfair practice types
pub fn detect_specific_practice(
    description: &str,
    practice_type: UnfairPracticeType,
) -> Option<UnfairPractice> {
    let desc_lower = description.to_lowercase();

    match practice_type {
        UnfairPracticeType::FalseRepresentation => {
            // Check for exaggerated claims
            if desc_lower.contains("guaranteed cure")
                || desc_lower.contains("100% effective")
                || desc_lower.contains("never fails")
            {
                let mut practice = UnfairPractice::new(
                    "detect-1",
                    UnfairPracticeType::FalseRepresentation,
                    "Potentially false or exaggerated claims",
                );
                practice.add_evidence(description.to_string());
                return Some(practice);
            }
        }
        UnfairPracticeType::Harassment => {
            // Check for pressure tactics
            if desc_lower.contains("must buy now")
                || desc_lower.contains("last chance")
                || desc_lower.contains("act immediately")
            {
                let mut practice = UnfairPractice::new(
                    "detect-2",
                    UnfairPracticeType::Harassment,
                    "Potentially coercive language",
                );
                practice.add_evidence(description.to_string());
                return Some(practice);
            }
        }
        UnfairPracticeType::BaitAdvertising => {
            // Check for bait and switch indicators
            if desc_lower.contains("price subject to change")
                || desc_lower.contains("availability not guaranteed")
            {
                let mut practice = UnfairPractice::new(
                    "detect-3",
                    UnfairPracticeType::BaitAdvertising,
                    "Potentially misleading availability claims",
                );
                practice.add_evidence(description.to_string());
                return Some(practice);
            }
        }
        _ => {}
    }

    None
}

/// Calculates recommended remedy for breach
pub fn recommend_remedy(sale: &SaleOfGoods) -> Vec<ConsumerRemedy> {
    let mut remedies = Vec::new();

    if sale.is_defective {
        // Within 6 months - Lemon Law applies
        if sale.is_lemon_law_applicable() {
            remedies.push(ConsumerRemedy::Repair);
            remedies.push(ConsumerRemedy::Replacement);
            remedies.push(ConsumerRemedy::Refund);
            remedies.push(ConsumerRemedy::PriceReduction);
        } else {
            // After 6 months - common law remedies
            remedies.push(ConsumerRemedy::Damages);
            if sale
                .implied_terms
                .contains(&ImpliedTerm::MerchantableQuality)
            {
                remedies.push(ConsumerRemedy::Repair);
            }
        }
    }

    remedies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_consumer_contract_high_risk() {
        let mut contract = ConsumerContract::new(
            "c1",
            "Seller",
            "Buyer",
            TransactionType::Services,
            100_000,
            "Service",
        );
        contract.risk_score = 85;

        match validate_consumer_contract(&contract) {
            Err(ConsumerError::HighRiskContract { risk_score }) => {
                assert_eq!(risk_score, 85);
            }
            _ => panic!("Expected HighRiskContract error"),
        }
    }

    #[test]
    fn test_validate_consumer_contract_sct_limit() {
        let contract = ConsumerContract::new(
            "c2",
            "Seller",
            "Buyer",
            TransactionType::SaleOfGoods,
            2_500_000, // SGD 25,000
            "Expensive item",
        );

        match validate_consumer_contract(&contract) {
            Err(ConsumerError::ExceedsSctLimit { amount }) => {
                assert_eq!(amount, 25_000);
            }
            _ => panic!("Expected ExceedsSctLimit error"),
        }
    }

    #[test]
    fn test_detect_unfair_practices_false_representation() {
        let contract = ConsumerContract::new(
            "c3",
            "Snake Oil Co",
            "Buyer",
            TransactionType::SaleOfGoods,
            50_000,
            "Guaranteed miracle cure for all ailments",
        );

        let practices = detect_unfair_practices(&contract);
        assert!(!practices.is_empty());
        assert_eq!(
            practices[0].practice_type,
            UnfairPracticeType::FalseRepresentation
        );
    }

    #[test]
    fn test_validate_sale_of_goods_defective() {
        let mut sale = SaleOfGoods::new("s1", true, "Television");
        sale.report_defect("Screen is cracked");

        match validate_sale_of_goods(&sale) {
            Err(ConsumerError::DefectDiscovered { .. }) => {}
            _ => panic!("Expected DefectDiscovered error"),
        }
    }

    #[test]
    fn test_validate_implied_terms() {
        let mut sale = SaleOfGoods::new("s2", true, "Laptop");
        sale.particular_purpose = Some("Video editing".to_string());
        sale.sale_by_sample = true;

        let terms = validate_implied_terms(&sale).unwrap();

        assert!(terms.contains(&ImpliedTerm::CorrespondsToDescription));
        assert!(terms.contains(&ImpliedTerm::MerchantableQuality));
        assert!(terms.contains(&ImpliedTerm::FitnessForPurpose));
        assert!(terms.contains(&ImpliedTerm::SaleBySample));
    }

    #[test]
    fn test_validate_warranty_expired() {
        let warranty = WarrantyTerms::new(365, WarrantyType::Manufacturer, "Coverage");

        match validate_warranty(&warranty, 400) {
            Err(ConsumerError::WarrantyExpired { days_ago }) => {
                assert_eq!(days_ago, 35);
            }
            _ => panic!("Expected WarrantyExpired error"),
        }
    }

    #[test]
    fn test_validate_warranty_valid() {
        let warranty = WarrantyTerms::new(365, WarrantyType::Manufacturer, "Coverage");

        assert!(validate_warranty(&warranty, 100).is_ok());
    }

    #[test]
    fn test_detect_specific_practice_harassment() {
        let description = "You must buy now or lose this deal forever!";
        let practice =
            detect_specific_practice(description, UnfairPracticeType::Harassment).unwrap();

        assert_eq!(practice.practice_type, UnfairPracticeType::Harassment);
    }

    #[test]
    fn test_recommend_remedy_lemon_law() {
        let mut sale = SaleOfGoods::new("s3", true, "Phone");
        sale.report_defect("Battery drains in 1 hour");

        let remedies = recommend_remedy(&sale);

        assert!(remedies.contains(&ConsumerRemedy::Repair));
        assert!(remedies.contains(&ConsumerRemedy::Replacement));
        assert!(remedies.contains(&ConsumerRemedy::Refund));
    }
}
