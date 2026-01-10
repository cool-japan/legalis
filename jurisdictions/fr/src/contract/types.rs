//! Contract law types (Types de droit des contrats)
//!
//! This module provides type definitions for French contract law under the Code civil,
//! as reformed in 2016 (Ordonnance n° 2016-131).

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Contract type (Type de contrat)
///
/// Represents the different types of contracts recognized under French law.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContractType {
    /// Sale (Vente) - Articles 1582-1685
    /// Transfer of ownership in exchange for a price
    Sale { price: u64, subject: String },

    /// Lease (Bail) - Articles 1709-1762
    /// Temporary enjoyment of a thing in exchange for rent
    Lease {
        duration_months: u32,
        rent_per_month: u64,
        subject: String,
    },

    /// Service contract (Contrat de prestation de services)
    /// Provision of services for remuneration
    Service {
        description: String,
        remuneration: u64,
    },

    /// Employment contract (Contrat de travail)
    /// See labor law module for detailed types
    Employment { description: String },

    /// Mandate (Mandat) - Articles 1984-2010
    /// Agency relationship where one person acts for another
    Mandate { scope: String, remunerated: bool },

    /// Loan (Prêt) - Articles 1874-1914
    /// Transfer of fungible things to be returned
    Loan {
        principal: u64,
        interest_rate: Option<f64>,
    },

    /// Other contract type
    Other(String),
}

/// Breach type (Type d'inexécution)
///
/// Categories of contract breach under Articles 1217+
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BreachType {
    /// Complete non-performance (Inexécution totale)
    /// The obligation was not performed at all
    NonPerformance,

    /// Defective performance (Inexécution imparfaite)
    /// The obligation was performed but not correctly
    DefectivePerformance,

    /// Delayed performance (Retard d'exécution)
    /// The obligation was performed late
    DelayedPerformance,

    /// Partial performance (Exécution partielle)
    /// Only part of the obligation was performed
    PartialPerformance,
}

/// Remedy type (Type de sanction de l'inexécution)
///
/// Available remedies for contract breach under Article 1217
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RemedyType {
    /// Specific performance (Exécution forcée en nature) - Article 1221
    /// Compel the debtor to perform the obligation
    SpecificPerformance,

    /// Price reduction (Réduction du prix) - Article 1223
    /// Reduce the price proportionally to the defect
    PriceReduction,

    /// Termination (Résolution) - Article 1224
    /// Terminate the contract for breach
    Termination,

    /// Damages (Dommages-intérêts) - Article 1231
    /// Monetary compensation for harm
    Damages,

    /// Exception of non-performance (Exception d'inexécution) - Article 1219
    /// Suspend own performance when other party fails to perform
    ExceptionOfNonPerformance,
}

/// Validity defect (Vice du consentement)
///
/// Defects that can invalidate consent under Articles 1130-1171
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidityDefect {
    /// Error (Erreur) - Articles 1132-1136
    /// Mistake about an essential element
    Error {
        /// Is the error about an essential quality? (Article 1133)
        essential_quality: bool,
        /// Description of the error
        description: String,
    },

    /// Fraud (Dol) - Articles 1137-1139
    /// Intentional deception by a party or third party
    Fraud {
        /// Was fraud committed by the other contracting party?
        by_contracting_party: bool,
        /// Description of the fraudulent acts
        description: String,
    },

    /// Duress (Violence) - Articles 1140-1143
    /// Threat or coercion that vitiated consent
    Duress {
        /// Severity level of the duress
        severity: DuressLevel,
        /// Description of the duress
        description: String,
    },
}

/// Duress severity level (Degré de violence)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DuressLevel {
    /// Minor threat (Menace légère)
    Minor,
    /// Moderate threat (Menace modérée)
    Moderate,
    /// Severe threat (Menace grave)
    Severe,
}

/// Obligation type (Type d'obligation)
///
/// Categories of contractual obligations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObligationType {
    /// Obligation to give (Obligation de donner)
    /// Transfer ownership or a right
    Give,

    /// Obligation to do (Obligation de faire)
    /// Perform a positive act
    Do,

    /// Obligation not to do (Obligation de ne pas faire)
    /// Refrain from an act
    NotToDo,
}

/// Contract (Contrat)
///
/// Builder pattern for creating contract claims under French law.
///
/// # Example
///
/// ```
/// use legalis_fr::contract::{Contract, ContractType, BreachType};
///
/// let contract = Contract::new()
///     .with_type(ContractType::Sale {
///         price: 50_000,
///         subject: "Machine industrielle".to_string()
///     })
///     .with_parties(vec!["Acheteur SARL".to_string(), "Vendeur SA".to_string()])
///     .with_consent(true)
///     .with_breach(BreachType::NonPerformance);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Contract {
    /// Type of contract
    pub contract_type: Option<ContractType>,

    /// Parties to the contract (at least 2 required)
    pub parties: Vec<String>,

    /// Was consent given? (Article 1128 requirement 1)
    pub consent_given: bool,

    /// Validity defects affecting consent (vices du consentement)
    pub validity_defects: Vec<ValidityDefect>,

    /// Date the contract was formed
    pub formation_date: Option<NaiveDate>,

    /// Date when performance is due
    pub performance_date: Option<NaiveDate>,

    /// Type of breach, if any
    pub breach: Option<BreachType>,

    /// Contract value (for damages calculation)
    pub contract_value: Option<u64>,

    /// Actual loss suffered due to breach
    pub actual_loss: Option<u64>,

    /// Penalty clause amount, if any (clause pénale - Article 1231-5)
    pub penalty_clause: Option<u64>,

    /// Good faith principle respected? (Article 1104)
    pub good_faith: bool,
}

impl Contract {
    /// Create a new contract builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            contract_type: None,
            parties: Vec::new(),
            consent_given: false,
            validity_defects: Vec::new(),
            formation_date: None,
            performance_date: None,
            breach: None,
            contract_value: None,
            actual_loss: None,
            penalty_clause: None,
            good_faith: true, // Presumed by default (Article 1104)
        }
    }

    /// Set the contract type
    #[must_use]
    pub fn with_type(mut self, contract_type: ContractType) -> Self {
        self.contract_type = Some(contract_type);
        self
    }

    /// Add parties to the contract
    #[must_use]
    pub fn with_parties(mut self, parties: Vec<String>) -> Self {
        self.parties = parties;
        self
    }

    /// Set whether consent was given
    #[must_use]
    pub fn with_consent(mut self, consent_given: bool) -> Self {
        self.consent_given = consent_given;
        self
    }

    /// Add a validity defect
    #[must_use]
    pub fn with_validity_defect(mut self, defect: ValidityDefect) -> Self {
        self.validity_defects.push(defect);
        self
    }

    /// Set the formation date
    #[must_use]
    pub fn with_formation_date(mut self, date: NaiveDate) -> Self {
        self.formation_date = Some(date);
        self
    }

    /// Set the performance date
    #[must_use]
    pub fn with_performance_date(mut self, date: NaiveDate) -> Self {
        self.performance_date = Some(date);
        self
    }

    /// Set the breach type
    #[must_use]
    pub fn with_breach(mut self, breach: BreachType) -> Self {
        self.breach = Some(breach);
        self
    }

    /// Set the contract value
    #[must_use]
    pub fn with_contract_value(mut self, value: u64) -> Self {
        self.contract_value = Some(value);
        self
    }

    /// Set the actual loss
    #[must_use]
    pub fn with_actual_loss(mut self, loss: u64) -> Self {
        self.actual_loss = Some(loss);
        self
    }

    /// Set the penalty clause amount
    #[must_use]
    pub fn with_penalty_clause(mut self, amount: u64) -> Self {
        self.penalty_clause = Some(amount);
        self
    }

    /// Set whether good faith was respected
    #[must_use]
    pub fn with_good_faith(mut self, good_faith: bool) -> Self {
        self.good_faith = good_faith;
        self
    }

    /// Check if the contract has at least 2 parties
    #[must_use]
    pub fn has_sufficient_parties(&self) -> bool {
        self.parties.len() >= 2
    }

    /// Check if the contract has any validity defects
    #[must_use]
    pub fn has_validity_defects(&self) -> bool {
        !self.validity_defects.is_empty()
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_builder() {
        let contract = Contract::new()
            .with_type(ContractType::Sale {
                price: 100_000,
                subject: "Véhicule".to_string(),
            })
            .with_parties(vec!["Acheteur".to_string(), "Vendeur".to_string()])
            .with_consent(true)
            .with_good_faith(true);

        assert!(contract.contract_type.is_some());
        assert_eq!(contract.parties.len(), 2);
        assert!(contract.consent_given);
        assert!(contract.good_faith);
        assert!(!contract.has_validity_defects());
    }

    #[test]
    fn test_contract_with_breach() {
        let contract = Contract::new()
            .with_breach(BreachType::NonPerformance)
            .with_contract_value(50_000)
            .with_actual_loss(45_000);

        assert_eq!(contract.breach, Some(BreachType::NonPerformance));
        assert_eq!(contract.contract_value, Some(50_000));
        assert_eq!(contract.actual_loss, Some(45_000));
    }

    #[test]
    fn test_contract_with_validity_defect() {
        let contract = Contract::new().with_validity_defect(ValidityDefect::Error {
            essential_quality: true,
            description: "Erreur sur la substance".to_string(),
        });

        assert_eq!(contract.validity_defects.len(), 1);
        assert!(contract.has_validity_defects());
    }

    #[test]
    fn test_sufficient_parties() {
        let mut contract = Contract::new();
        assert!(!contract.has_sufficient_parties());

        contract = contract.with_parties(vec!["A".to_string()]);
        assert!(!contract.has_sufficient_parties());

        contract = contract.with_parties(vec!["A".to_string(), "B".to_string()]);
        assert!(contract.has_sufficient_parties());
    }

    #[test]
    fn test_breach_types() {
        assert_eq!(BreachType::NonPerformance, BreachType::NonPerformance);
        assert_ne!(BreachType::NonPerformance, BreachType::DelayedPerformance);
    }

    #[test]
    fn test_remedy_types() {
        let remedies = [
            RemedyType::SpecificPerformance,
            RemedyType::PriceReduction,
            RemedyType::Termination,
            RemedyType::Damages,
        ];

        assert_eq!(remedies.len(), 4);
    }

    #[test]
    fn test_duress_level_ordering() {
        assert!(DuressLevel::Minor < DuressLevel::Moderate);
        assert!(DuressLevel::Moderate < DuressLevel::Severe);
    }
}
