//! Consumer Rights Act 2015 Validation Functions

use chrono::Utc;

use super::error::{ConsumerRightsError, Result};
use super::types::*;

/// Validate goods contract compliance with CRA 2015 Part 1 Chapter 2
///
/// Checks statutory rights under ss.9-17:
/// - s.9: Satisfactory quality
/// - s.10: Fitness for particular purpose
/// - s.11: As described
/// - s.12: Sample
/// - s.13: Model
///
/// # Example
/// ```ignore
/// let contract = GoodsContract { ... };
/// validate_goods_contract(&contract)?;
/// ```
pub fn validate_goods_contract(contract: &GoodsContract) -> Result<()> {
    // Validate trader
    validate_trader(&contract.trader)?;

    // Validate consumer
    validate_consumer(&contract.consumer)?;

    // Validate price is non-negative
    if contract.price_gbp < 0.0 {
        return Err(ConsumerRightsError::invalid_value(
            "price_gbp",
            "Price cannot be negative",
        ));
    }

    // Validate description exists
    if contract.description.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("description"));
    }

    // Check statutory rights are present
    if contract.statutory_rights.is_empty() {
        return Err(ConsumerRightsError::invalid_value(
            "statutory_rights",
            "At least one statutory right must apply (s.9, s.10, or s.11)",
        ));
    }

    Ok(())
}

/// Validate services contract compliance with CRA 2015 Part 1 Chapter 4
///
/// Checks statutory rights under ss.49-52:
/// - s.49: Reasonable care and skill
/// - s.50: Information binding
/// - s.51: Reasonable price
/// - s.52: Reasonable time
pub fn validate_services_contract(contract: &ServicesContract) -> Result<()> {
    validate_trader(&contract.trader)?;
    validate_consumer(&contract.consumer)?;

    if contract.description.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("description"));
    }

    // Validate price if specified
    if let Some(price) = contract.price_gbp {
        if price < 0.0 {
            return Err(ConsumerRightsError::invalid_value(
                "price_gbp",
                "Price cannot be negative",
            ));
        }
    }

    if contract.statutory_rights.is_empty() {
        return Err(ConsumerRightsError::invalid_value(
            "statutory_rights",
            "At least one statutory right must apply",
        ));
    }

    Ok(())
}

/// Validate digital content contract compliance with CRA 2015 Part 1 Chapter 3
///
/// Checks statutory rights under ss.34-37:
/// - s.34: Satisfactory quality
/// - s.35: Fitness for particular purpose
/// - s.36: As described
/// - s.37: Other pre-contract information
pub fn validate_digital_content_contract(contract: &DigitalContentContract) -> Result<()> {
    validate_trader(&contract.trader)?;
    validate_consumer(&contract.consumer)?;

    if contract.description.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("description"));
    }

    if contract.price_gbp < 0.0 {
        return Err(ConsumerRightsError::invalid_value(
            "price_gbp",
            "Price cannot be negative (use £0 for free content)",
        ));
    }

    if contract.statutory_rights.is_empty() {
        return Err(ConsumerRightsError::invalid_value(
            "statutory_rights",
            "At least one statutory right must apply",
        ));
    }

    Ok(())
}

/// Validate trader details
pub fn validate_trader(trader: &Trader) -> Result<()> {
    if trader.name.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("trader.name"));
    }

    if trader.address.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("trader.address"));
    }

    if trader.contact.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("trader.contact"));
    }

    Ok(())
}

/// Validate consumer details
pub fn validate_consumer(consumer: &Consumer) -> Result<()> {
    if consumer.name.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("consumer.name"));
    }

    if consumer.address.trim().is_empty() {
        return Err(ConsumerRightsError::missing_field("consumer.address"));
    }

    Ok(())
}

/// Validate satisfactory quality for goods (CRA 2015 s.9)
///
/// s.9(2): Quality includes:
/// - Fitness for common purposes
/// - Appearance and finish
/// - Freedom from minor defects
/// - Safety
/// - Durability
///
/// s.9(3): Takes into account:
/// - Any description
/// - Price
/// - All other relevant circumstances
pub fn validate_satisfactory_quality(
    goods_description: &str,
    defect_description: &str,
    price_gbp: f64,
    is_satisfactory: bool,
) -> Result<()> {
    if !is_satisfactory {
        let remedy = if price_gbp > 0.0 {
            "Short-term right to reject (s.22), or repair/replacement (s.23), \
             or price reduction/final rejection (s.24)"
        } else {
            "Free goods still protected by CRA 2015 s.9"
        };

        return Err(ConsumerRightsError::GoodsNotSatisfactoryQuality {
            description: format!("{}: {}", goods_description, defect_description),
            remedy: remedy.to_string(),
        });
    }

    Ok(())
}

/// Validate fitness for particular purpose (CRA 2015 s.10)
///
/// s.10(1): Applies where consumer makes known to trader particular purpose
/// s.10(3): Goods must be fit for that purpose
pub fn validate_fit_for_purpose(
    purpose_made_known: &str,
    is_fit: bool,
    reason_not_fit: &str,
) -> Result<()> {
    if !is_fit {
        return Err(ConsumerRightsError::GoodsNotFitForPurpose {
            purpose: purpose_made_known.to_string(),
            reason: reason_not_fit.to_string(),
            remedy: "Repair, replacement, or rejection".to_string(),
        });
    }

    Ok(())
}

/// Validate goods match description (CRA 2015 s.11)
pub fn validate_as_described(description: &str, actual: &str, matches: bool) -> Result<()> {
    if !matches {
        return Err(ConsumerRightsError::GoodsNotAsDescribed {
            description: description.to_string(),
            actual: actual.to_string(),
            remedy: "Full refund under short-term right to reject".to_string(),
        });
    }

    Ok(())
}

/// Validate service performed with reasonable care and skill (CRA 2015 s.49)
pub fn validate_reasonable_care_and_skill(
    service_description: &str,
    performed_with_care: bool,
    failure_description: &str,
) -> Result<()> {
    if !performed_with_care {
        return Err(ConsumerRightsError::ServiceNotReasonableCareAndSkill {
            service: service_description.to_string(),
            failure: failure_description.to_string(),
        });
    }

    Ok(())
}

/// Validate service completed in reasonable time (CRA 2015 s.52)
pub fn validate_reasonable_time(
    service_description: &str,
    commencement_date: chrono::NaiveDate,
    completion_date: Option<chrono::NaiveDate>,
    reasonable_time_days: u32,
) -> Result<()> {
    let completion = completion_date.unwrap_or_else(|| Utc::now().date_naive());
    let elapsed_days = (completion - commencement_date).num_days() as u32;

    if elapsed_days > reasonable_time_days {
        return Err(ConsumerRightsError::ServiceNotReasonableTime {
            service: service_description.to_string(),
            time_elapsed_days: elapsed_days,
        });
    }

    Ok(())
}

/// Validate unfair contract term under CRA 2015 Part 2
///
/// s.62: Term unfair if:
/// - Contrary to requirement of good faith, AND
/// - Causes significant imbalance in parties' rights/obligations, AND
/// - To detriment of consumer
///
/// s.62(1): Unfair term is NOT BINDING on consumer
pub fn validate_unfair_term(assessment: &UnfairTermAssessment, term_text: &str) -> Result<()> {
    if assessment.is_unfair() {
        let grey_list = if let Some(item) = assessment.on_grey_list {
            format!("• On grey list: {:?}", item)
        } else {
            "• Not on grey list".to_string()
        };

        return Err(ConsumerRightsError::UnfairContractTerm {
            term: term_text.to_string(),
            contrary_to_good_faith: assessment.contrary_to_good_faith,
            significant_imbalance: assessment.significant_imbalance,
            detriment_to_consumer: assessment.detriment_to_consumer,
            grey_list,
        });
    }

    Ok(())
}

/// Validate remedy time limits
pub fn validate_remedy_time_limit(time_limit: &TimeLimit) -> Result<()> {
    if time_limit.expired {
        return Err(ConsumerRightsError::RemedyTimeLimitExpired {
            remedy: format!("{:?}", time_limit.remedy),
            deadline: time_limit.deadline.format("%Y-%m-%d").to_string(),
        });
    }

    Ok(())
}

/// Check if short-term right to reject is available
pub fn check_short_term_reject_available(purchase_date: chrono::NaiveDate) -> Result<()> {
    let days_elapsed = (Utc::now().date_naive() - purchase_date).num_days() as u32;

    if days_elapsed > 30 {
        return Err(ConsumerRightsError::ShortTermRejectExpired {
            purchase_date: purchase_date.format("%Y-%m-%d").to_string(),
            days_elapsed,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_satisfactory_quality_pass() {
        let result = validate_satisfactory_quality("Laptop", "", 500.0, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_satisfactory_quality_fail() {
        let result =
            validate_satisfactory_quality("Laptop", "Screen has dead pixels", 500.0, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::GoodsNotSatisfactoryQuality { .. }
        ));
    }

    #[test]
    fn test_validate_fit_for_purpose_fail() {
        let result = validate_fit_for_purpose(
            "Heavy gaming",
            false,
            "Graphics card insufficient for modern games",
        );
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::GoodsNotFitForPurpose { .. }
        ));
    }

    #[test]
    fn test_validate_as_described_fail() {
        let result = validate_as_described("16GB RAM", "8GB RAM", false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::GoodsNotAsDescribed { .. }
        ));
    }

    #[test]
    fn test_validate_unfair_term() {
        let assessment = UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: Some(GreyListItem::ExcludeLiabilityDeathInjury),
            transparent_and_prominent: false,
        };

        let result = validate_unfair_term(&assessment, "We exclude all liability");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::UnfairContractTerm { .. }
        ));
    }

    #[test]
    fn test_short_term_reject_expired() {
        let purchase_date = Utc::now().date_naive() - chrono::Duration::days(35);
        let result = check_short_term_reject_available(purchase_date);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConsumerRightsError::ShortTermRejectExpired { .. }
        ));
    }

    #[test]
    fn test_validate_goods_contract() {
        let contract = GoodsContract {
            description: "Laptop".to_string(),
            price_gbp: 500.0,
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            trader: Trader {
                name: "Tech Shop".to_string(),
                address: "123 High St".to_string(),
                contact: "tech@shop.com".to_string(),
                company_number: None,
            },
            consumer: Consumer {
                name: "John Doe".to_string(),
                address: "456 Main St".to_string(),
                contact: "john@example.com".to_string(),
            },
            statutory_rights: vec![GoodsStatutoryRight::SatisfactoryQuality],
            remedy_stage: None,
        };

        assert!(validate_goods_contract(&contract).is_ok());
    }
}
