//! Book III: Obligations (ພັນທະ) - Articles 432-672
//!
//! This module implements obligations law of the Lao Civil Code 2020,
//! covering general obligations, contracts, torts, and unjust enrichment.
//!
//! ## Structure
//! - Chapter 1: General Provisions on Obligations (Articles 432-480)
//! - Chapter 2: Contracts (Articles 481-580)
//! - Chapter 3: Torts (Articles 581-630)
//! - Chapter 4: Unjust Enrichment (Articles 631-660)
//! - Chapter 5: Management of Affairs without Mandate (Articles 661-672)
//!
//! ## Comparative Law Notes
//! - Structure follows Japanese saiken-hō (債権法) with French obligations influence
//! - Contract law based on Japanese Civil Code Book III (2017 reform)
//! - Tort law influenced by Japanese Articles 709-724

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for obligations law
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ObligationsError {
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    #[error("Breach of contract: {0}")]
    BreachOfContract(String),

    #[error("Tort liability: {0}")]
    TortLiability(String),

    #[error("Unjust enrichment: {0}")]
    UnjustEnrichment(String),

    #[error("Performance failure: {0}")]
    PerformanceFailure(String),
}

pub type Result<T> = std::result::Result<T, ObligationsError>;

/// Type of obligation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationType {
    /// Obligation to give
    ToGive,
    /// Obligation to do
    ToDo,
    /// Obligation not to do
    NotToDo,
}

/// Article 432-480: General Obligations
///
/// An obligation is a legal relationship whereby one person (obligor) is bound to
/// perform for another person (obligee).
///
/// Comparative: Japanese Civil Code Articles 399-520 (債権総則)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub obligor: String,
    pub obligee: String,
    pub obligation_type: ObligationType,
    pub description: String,
    pub amount: Option<u64>,
    pub due_date: Option<DateTime<Utc>>,
}

/// Article 432: Nature of Obligations
///
/// An obligation binds the obligor to perform for the obligee.
///
/// # Japanese Influence
/// Based on Japanese Civil Code Articles 399-400 on nature of obligations
pub fn article432(obligation: &Obligation) -> Result<()> {
    if obligation.obligor.is_empty() || obligation.obligee.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Obligor and obligee must be specified".to_string(),
        ));
    }

    if obligation.description.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Obligation must have description".to_string(),
        ));
    }

    Ok(())
}

/// Type of contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale
    Sale { price: u64, subject: String },
    /// Lease
    Lease {
        rent: u64,
        duration_days: u32,
        subject: String,
    },
    /// Loan
    Loan { amount: u64, interest_rate: f64 },
    /// Service
    Service { fee: u64, description: String },
    /// Other
    Other(String),
}

/// Article 481-580: Contracts (ສັນຍາ)
///
/// A contract is an agreement between parties creating obligations.
///
/// Comparative: Japanese Civil Code Articles 521-696 (契約各則),
///              French Code civil Articles 1103-1231
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub parties: Vec<String>,
    pub contract_type: ContractType,
    pub offer: String,
    pub acceptance: bool,
    pub consideration: Option<u64>,
    pub lawful_purpose: bool,
    pub capacity_verified: bool,
    pub free_consent: bool,
    pub concluded_at: DateTime<Utc>,
}

/// Article 500: Formation of Contract
///
/// A contract is formed when offer meets acceptance.
///
/// # Japanese Influence
/// Based on Japanese Civil Code Article 522 (2017 reform):
/// "契約は、契約の内容を示してその締結を申し入れる意思表示に対して
/// 相手方が承諾をしたときに成立する"
pub fn article500(contract: &Contract) -> Result<()> {
    // Must have at least two parties
    if contract.parties.len() < 2 {
        return Err(ObligationsError::InvalidContract(
            "Contract requires at least two parties".to_string(),
        ));
    }

    // Must have valid offer
    if contract.offer.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Contract requires valid offer".to_string(),
        ));
    }

    // Must have acceptance
    if !contract.acceptance {
        return Err(ObligationsError::InvalidContract(
            "Contract requires acceptance".to_string(),
        ));
    }

    // Must have lawful purpose
    if !contract.lawful_purpose {
        return Err(ObligationsError::InvalidContract(
            "Contract purpose must be lawful".to_string(),
        ));
    }

    // Parties must have capacity
    if !contract.capacity_verified {
        return Err(ObligationsError::InvalidContract(
            "Party capacity must be verified".to_string(),
        ));
    }

    // Consent must be free
    if !contract.free_consent {
        return Err(ObligationsError::InvalidContract(
            "Consent must be freely given".to_string(),
        ));
    }

    Ok(())
}

/// Validates contract formation requirements
pub fn validate_contract_formation(contract: &Contract) -> Result<()> {
    article432(&Obligation {
        obligor: contract.parties.first().cloned().unwrap_or_default(),
        obligee: contract.parties.get(1).cloned().unwrap_or_default(),
        obligation_type: ObligationType::ToGive,
        description: contract.offer.clone(),
        amount: contract.consideration,
        due_date: None,
    })?;

    article500(contract)?;
    Ok(())
}

/// Type of breach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Non-performance
    NonPerformance,
    /// Defective performance
    DefectivePerformance,
    /// Delayed performance
    DelayedPerformance,
}

/// Contract breach
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractBreach {
    pub contract: Contract,
    pub breach_type: BreachType,
    pub breaching_party: String,
    pub damages: Option<u64>,
    pub occurred_at: DateTime<Utc>,
}

/// Validates contract breach claim
pub fn validate_breach_claim(breach: &ContractBreach) -> Result<u64> {
    // Verify contract is valid
    validate_contract_formation(&breach.contract)?;

    // Verify breaching party is a contract party
    if !breach.contract.parties.contains(&breach.breaching_party) {
        return Err(ObligationsError::BreachOfContract(
            "Breaching party must be a contract party".to_string(),
        ));
    }

    // Calculate damages if not specified
    if let Some(damages) = breach.damages {
        Ok(damages)
    } else {
        // Default damage calculation based on contract type
        match &breach.contract.contract_type {
            ContractType::Sale { price, .. } => Ok(*price),
            ContractType::Lease {
                rent,
                duration_days,
                ..
            } => Ok(rent * (*duration_days as u64 / 30)),
            ContractType::Loan {
                amount,
                interest_rate,
            } => Ok(((*amount as f64) * (1.0 + interest_rate)) as u64),
            ContractType::Service { fee, .. } => Ok(*fee),
            ContractType::Other(_) => Ok(0),
        }
    }
}

/// Type of tort
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortType {
    /// Negligence
    Negligence,
    /// Intentional harm
    Intentional,
    /// Strict liability
    StrictLiability,
}

/// Article 581-630: Torts (ການກະທຳຜິດລະເບຽບ)
///
/// A person who intentionally or negligently causes harm to another must compensate.
///
/// Comparative: Japanese Civil Code Articles 709-724 (不法行為),
///              French Code civil Articles 1240-1242
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tort {
    pub tortfeasor: String,
    pub victim: String,
    pub tort_type: TortType,
    pub wrongful_act: String,
    pub harm_caused: String,
    pub causation: bool,
    pub fault: bool,
    pub damages: u64,
    pub occurred_at: DateTime<Utc>,
}

/// Article 600: General Tort Liability
///
/// A person who intentionally or negligently infringes the rights of another
/// and causes damage shall be liable to compensate for such damage.
///
/// # Japanese Influence
/// Based on Japanese Civil Code Article 709:
/// "故意又は過失によって他人の権利又は法律上保護される利益を
/// 侵害した者は、これによって生じた損害を賠償する責任を負う"
///
/// # French Influence
/// Also reflects French Code civil Article 1240 (former Article 1382):
/// "Tout fait quelconque de l'homme, qui cause à autrui un dommage,
/// oblige celui par la faute duquel il est arrivé à le réparer"
pub fn article600(tort: &Tort) -> Result<()> {
    // Must have identified tortfeasor and victim
    if tort.tortfeasor.is_empty() || tort.victim.is_empty() {
        return Err(ObligationsError::TortLiability(
            "Tortfeasor and victim must be identified".to_string(),
        ));
    }

    // Must have wrongful act
    if tort.wrongful_act.is_empty() {
        return Err(ObligationsError::TortLiability(
            "Wrongful act must be specified".to_string(),
        ));
    }

    // Must have harm
    if tort.harm_caused.is_empty() {
        return Err(ObligationsError::TortLiability(
            "Harm must be specified".to_string(),
        ));
    }

    // Must have causation (except strict liability)
    if !tort.causation && tort.tort_type != TortType::StrictLiability {
        return Err(ObligationsError::TortLiability(
            "Causation must be established".to_string(),
        ));
    }

    // Must have fault (except strict liability)
    if !tort.fault && tort.tort_type != TortType::StrictLiability {
        return Err(ObligationsError::TortLiability(
            "Fault must be established".to_string(),
        ));
    }

    Ok(())
}

/// Validates tort claim
pub fn validate_tort_claim(tort: &Tort) -> Result<u64> {
    article600(tort)?;
    Ok(tort.damages)
}

/// Article 631-660: Unjust Enrichment (ການຮັ່ງມີໂດຍບໍ່ມີສິດ)
///
/// A person who is enriched without legal cause at the expense of another
/// must return the benefit.
///
/// Comparative: Japanese Civil Code Articles 703-708 (不当利得),
///              French Code civil Articles 1303-1303-4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnjustEnrichment {
    pub enriched_party: String,
    pub impoverished_party: String,
    pub benefit_received: u64,
    pub no_legal_cause: bool,
    pub at_expense_of: bool,
    pub occurred_at: DateTime<Utc>,
}

/// Validates unjust enrichment claim
pub fn validate_unjust_enrichment(enrichment: &UnjustEnrichment) -> Result<u64> {
    if enrichment.enriched_party.is_empty() || enrichment.impoverished_party.is_empty() {
        return Err(ObligationsError::UnjustEnrichment(
            "Parties must be identified".to_string(),
        ));
    }

    if !enrichment.no_legal_cause {
        return Err(ObligationsError::UnjustEnrichment(
            "Enrichment must lack legal cause".to_string(),
        ));
    }

    if !enrichment.at_expense_of {
        return Err(ObligationsError::UnjustEnrichment(
            "Enrichment must be at expense of another".to_string(),
        ));
    }

    if enrichment.benefit_received == 0 {
        return Err(ObligationsError::UnjustEnrichment(
            "Benefit must be quantified".to_string(),
        ));
    }

    Ok(enrichment.benefit_received)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article432_obligation() {
        let obligation = Obligation {
            obligor: "Debtor".to_string(),
            obligee: "Creditor".to_string(),
            obligation_type: ObligationType::ToGive,
            description: "Pay 10,000,000 LAK".to_string(),
            amount: Some(10_000_000),
            due_date: Some(Utc::now()),
        };
        assert!(article432(&obligation).is_ok());

        let invalid = Obligation {
            obligor: "".to_string(),
            ..obligation.clone()
        };
        assert!(article432(&invalid).is_err());
    }

    #[test]
    fn test_article500_contract_formation() {
        let contract = Contract {
            parties: vec!["Buyer".to_string(), "Seller".to_string()],
            contract_type: ContractType::Sale {
                price: 100_000_000,
                subject: "Land".to_string(),
            },
            offer: "Sale of land for 100M LAK".to_string(),
            acceptance: true,
            consideration: Some(100_000_000),
            lawful_purpose: true,
            capacity_verified: true,
            free_consent: true,
            concluded_at: Utc::now(),
        };
        assert!(article500(&contract).is_ok());

        // Contract without acceptance
        let no_acceptance = Contract {
            acceptance: false,
            ..contract.clone()
        };
        assert!(article500(&no_acceptance).is_err());

        // Contract with unlawful purpose
        let unlawful = Contract {
            lawful_purpose: false,
            ..contract.clone()
        };
        assert!(article500(&unlawful).is_err());
    }

    #[test]
    fn test_validate_contract_formation() {
        let contract = Contract {
            parties: vec!["Party A".to_string(), "Party B".to_string()],
            contract_type: ContractType::Service {
                fee: 5_000_000,
                description: "Legal consultation".to_string(),
            },
            offer: "Provide legal services".to_string(),
            acceptance: true,
            consideration: Some(5_000_000),
            lawful_purpose: true,
            capacity_verified: true,
            free_consent: true,
            concluded_at: Utc::now(),
        };
        assert!(validate_contract_formation(&contract).is_ok());
    }

    #[test]
    fn test_validate_breach_claim() {
        let contract = Contract {
            parties: vec!["Buyer".to_string(), "Seller".to_string()],
            contract_type: ContractType::Sale {
                price: 50_000_000,
                subject: "Car".to_string(),
            },
            offer: "Sale of car".to_string(),
            acceptance: true,
            consideration: Some(50_000_000),
            lawful_purpose: true,
            capacity_verified: true,
            free_consent: true,
            concluded_at: Utc::now(),
        };

        let breach = ContractBreach {
            contract: contract.clone(),
            breach_type: BreachType::NonPerformance,
            breaching_party: "Seller".to_string(),
            damages: Some(50_000_000),
            occurred_at: Utc::now(),
        };

        let damages = validate_breach_claim(&breach).unwrap();
        assert_eq!(damages, 50_000_000);
    }

    #[test]
    fn test_article600_tort() {
        let tort = Tort {
            tortfeasor: "Defendant".to_string(),
            victim: "Plaintiff".to_string(),
            tort_type: TortType::Negligence,
            wrongful_act: "Negligent driving causing collision".to_string(),
            harm_caused: "Personal injury and property damage".to_string(),
            causation: true,
            fault: true,
            damages: 20_000_000,
            occurred_at: Utc::now(),
        };
        assert!(article600(&tort).is_ok());

        // Negligence without fault
        let no_fault = Tort {
            fault: false,
            ..tort.clone()
        };
        assert!(article600(&no_fault).is_err());

        // Strict liability doesn't require fault
        let strict = Tort {
            tort_type: TortType::StrictLiability,
            fault: false,
            ..tort.clone()
        };
        assert!(article600(&strict).is_ok());
    }

    #[test]
    fn test_validate_tort_claim() {
        let tort = Tort {
            tortfeasor: "Driver".to_string(),
            victim: "Pedestrian".to_string(),
            tort_type: TortType::Negligence,
            wrongful_act: "Running red light".to_string(),
            harm_caused: "Broken leg".to_string(),
            causation: true,
            fault: true,
            damages: 15_000_000,
            occurred_at: Utc::now(),
        };

        let damages = validate_tort_claim(&tort).unwrap();
        assert_eq!(damages, 15_000_000);
    }

    #[test]
    fn test_validate_unjust_enrichment() {
        let enrichment = UnjustEnrichment {
            enriched_party: "Recipient".to_string(),
            impoverished_party: "Payer".to_string(),
            benefit_received: 5_000_000,
            no_legal_cause: true,
            at_expense_of: true,
            occurred_at: Utc::now(),
        };

        let amount = validate_unjust_enrichment(&enrichment).unwrap();
        assert_eq!(amount, 5_000_000);

        // With legal cause
        let with_cause = UnjustEnrichment {
            no_legal_cause: false,
            ..enrichment.clone()
        };
        assert!(validate_unjust_enrichment(&with_cause).is_err());
    }
}
