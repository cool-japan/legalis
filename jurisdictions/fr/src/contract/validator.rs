//! Contract validation logic (Logique de validation des contrats)
//!
//! Validation functions for French contract law under the Code civil.

use super::error::{ContractError, ValidationResult};
use super::types::Contract;

/// Validate contract validity (Validité du contrat)
///
/// Validates the three requirements of Article 1128:
/// 1. Consent of the parties (Consentement des parties)
/// 2. Capacity to contract (Capacité de contracter)
/// 3. Lawful and certain content (Contenu licite et certain)
///
/// # Arguments
///
/// * `contract` - The contract to validate
///
/// # Returns
///
/// * `Ok(())` if the contract is valid
/// * `Err(ContractError)` if validation fails
///
/// # Example
///
/// ```
/// use legalis_fr::contract::{Contract, ContractType, validate_contract_validity};
///
/// let contract = Contract::new()
///     .with_type(ContractType::Sale {
///         price: 100_000,
///         subject: "Machine".to_string()
///     })
///     .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
///     .with_consent(true);
///
/// assert!(validate_contract_validity(&contract).is_ok());
/// ```
pub fn validate_contract_validity(contract: &Contract) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Article 1128, Requirement 1: Consent of the parties (Consentement)
    if !contract.consent_given {
        errors.push(ContractError::NoConsent);
    }

    // Check for validity defects (vices du consentement)
    // Articles 1130-1171: Error, fraud, or duress
    if !contract.validity_defects.is_empty() {
        errors.push(ContractError::ValidityDefect(
            contract.validity_defects.clone(),
        ));
    }

    // Article 1128, Requirement 2: Capacity to contract (Capacité)
    // For now, we assume all parties have capacity (adults, not under guardianship)
    // In a real implementation, this would check party ages and legal status

    // Article 1128, Requirement 3: Lawful and certain content (Contenu licite et certain)
    if contract.contract_type.is_none() {
        errors.push(ContractError::NoContractType);
    }

    // Must have at least 2 parties
    if !contract.has_sufficient_parties() {
        errors.push(ContractError::InsufficientParties);
    }

    // Article 1104: Good faith (Bonne foi)
    // Contracts must be negotiated, formed, and performed in good faith
    if !contract.good_faith {
        errors.push(ContractError::BadFaith);
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(ContractError::MultipleErrors(errors))
    }
}

/// Calculate contract damages (Calcul des dommages-intérêts)
///
/// Calculates damages for breach of contract according to Article 1231.
///
/// # Priority of calculation:
/// 1. If a penalty clause exists (Article 1231-5), use that amount
/// 2. Otherwise, use the actual loss suffered
///
/// # Arguments
///
/// * `contract_value` - The total value of the contract
/// * `actual_loss` - The actual loss suffered by the creditor
/// * `penalty_clause` - Optional penalty clause amount (clause pénale)
///
/// # Returns
///
/// The amount of damages in euros
///
/// # Example
///
/// ```
/// use legalis_fr::contract::calculate_contract_damages;
///
/// // Without penalty clause
/// let damages = calculate_contract_damages(100_000, 80_000, None);
/// assert_eq!(damages, 80_000);
///
/// // With penalty clause
/// let damages = calculate_contract_damages(100_000, 80_000, Some(25_000));
/// assert_eq!(damages, 25_000);
/// ```
#[must_use]
pub fn calculate_contract_damages(
    _contract_value: u64,
    actual_loss: u64,
    penalty_clause: Option<u64>,
) -> u64 {
    // Article 1231-5: Penalty clause (Clause pénale)
    // If a penalty clause exists, it takes precedence
    // Note: In practice, French courts can reduce excessive penalties (modération judiciaire)
    if let Some(penalty) = penalty_clause {
        return penalty;
    }

    // Article 1231: Compensatory damages (Dommages-intérêts compensatoires)
    // The debtor is liable for damages for non-performance, defective performance,
    // or delayed performance, unless prevented by force majeure
    actual_loss
}

/// Calculate damages with force majeure check
///
/// Article 1218 provides that force majeure exempts from liability.
///
/// # Arguments
///
/// * `contract_value` - The total value of the contract
/// * `actual_loss` - The actual loss suffered
/// * `penalty_clause` - Optional penalty clause
/// * `force_majeure` - Whether force majeure prevented performance
///
/// # Returns
///
/// The amount of damages, or 0 if force majeure applies
#[must_use]
pub fn calculate_damages_with_force_majeure(
    contract_value: u64,
    actual_loss: u64,
    penalty_clause: Option<u64>,
    force_majeure: bool,
) -> u64 {
    // Article 1218: Force majeure
    // "Il y a force majeure en matière contractuelle lorsqu'un événement
    // échappant au contrôle du débiteur, qui ne pouvait être raisonnablement
    // prévu lors de la conclusion du contrat et dont les effets ne peuvent
    // être évités par des mesures appropriées, empêche l'exécution de son
    // obligation par le débiteur."
    if force_majeure {
        return 0;
    }

    calculate_contract_damages(contract_value, actual_loss, penalty_clause)
}

/// Validate breach claim (Validation d'une réclamation pour inexécution)
///
/// Validates that a breach claim has sufficient information for remedies.
///
/// # Arguments
///
/// * `contract` - The contract with alleged breach
///
/// # Returns
///
/// * `Ok(())` if the breach claim is valid
/// * `Err(ContractError)` if validation fails
pub fn validate_breach_claim(contract: &Contract) -> ValidationResult<()> {
    // Must have a breach specified
    if contract.breach.is_none() {
        return Err(ContractError::NoBreach);
    }

    // For damages calculation, need either contract_value and actual_loss, or penalty_clause
    if contract.penalty_clause.is_none() && contract.actual_loss.is_none() {
        return Err(ContractError::InsufficientDamageInfo {
            missing: "actual_loss or penalty_clause".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::types::{BreachType, ContractType, ValidityDefect};

    #[test]
    fn test_valid_contract() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Voiture".to_string(),
            })
            .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
            .with_consent(true)
            .with_good_faith(true);

        assert!(validate_contract_validity(&contract).is_ok());
    }

    #[test]
    fn test_no_consent() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Voiture".to_string(),
            })
            .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
            .with_consent(false);

        let result = validate_contract_validity(&contract);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ContractError::NoConsent));
    }

    #[test]
    fn test_validity_defect() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Voiture".to_string(),
            })
            .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
            .with_consent(true)
            .with_validity_defect(ValidityDefect::Fraud {
                by_contracting_party: true,
                description: "Fausses informations".to_string(),
            });

        let result = validate_contract_validity(&contract);
        assert!(result.is_err());
    }

    #[test]
    fn test_insufficient_parties() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Voiture".to_string(),
            })
            .with_parties(vec!["Seule partie".to_string()])
            .with_consent(true);

        let result = validate_contract_validity(&contract);
        assert!(result.is_err());
    }

    #[test]
    fn test_bad_faith() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Voiture".to_string(),
            })
            .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
            .with_consent(true)
            .with_good_faith(false);

        let result = validate_contract_validity(&contract);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_damages_without_penalty() {
        let damages = calculate_contract_damages(100_000, 80_000, None);
        assert_eq!(damages, 80_000);
    }

    #[test]
    fn test_calculate_damages_with_penalty() {
        let damages = calculate_contract_damages(100_000, 80_000, Some(25_000));
        assert_eq!(damages, 25_000);
    }

    #[test]
    fn test_calculate_damages_force_majeure() {
        // Without force majeure
        let damages = calculate_damages_with_force_majeure(100_000, 80_000, None, false);
        assert_eq!(damages, 80_000);

        // With force majeure - no liability
        let damages = calculate_damages_with_force_majeure(100_000, 80_000, None, true);
        assert_eq!(damages, 0);
    }

    #[test]
    fn test_validate_breach_claim_valid() {
        let contract = Contract::new()
            .with_breach(BreachType::NonPerformance)
            .with_contract_value(100_000)
            .with_actual_loss(80_000);

        assert!(validate_breach_claim(&contract).is_ok());
    }

    #[test]
    fn test_validate_breach_claim_no_breach() {
        let contract = Contract::new()
            .with_contract_value(100_000)
            .with_actual_loss(80_000);

        let result = validate_breach_claim(&contract);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ContractError::NoBreach));
    }

    #[test]
    fn test_validate_breach_claim_insufficient_info() {
        let contract = Contract::new()
            .with_breach(BreachType::NonPerformance)
            .with_contract_value(100_000);
        // Missing actual_loss and penalty_clause

        let result = validate_breach_claim(&contract);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_breach_with_penalty_only() {
        let contract = Contract::new()
            .with_breach(BreachType::NonPerformance)
            .with_penalty_clause(25_000);
        // No actual_loss needed when penalty clause exists

        assert!(validate_breach_claim(&contract).is_ok());
    }
}
