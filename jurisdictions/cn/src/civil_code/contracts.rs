//! Book III: Contracts (合同编)
//!
//! Articles 463-988 of the Civil Code
//!
//! Covers:
//! - General provisions on contracts
//! - Formation, validity, performance, modification, termination
//! - 29 specific contract types
//! - Quasi-contracts

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Contract formation status (合同成立)
///
/// Articles 469-502
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractFormationStatus {
    /// Offer made (要约)
    OfferMade,
    /// Acceptance made (承诺)
    AcceptanceMade,
    /// Contract formed (合同成立)
    Formed,
    /// Contract not formed
    NotFormed,
}

/// Contract validity status (合同效力)
///
/// Articles 143-157, 502-508
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractValidityStatus {
    /// Valid (有效)
    Valid,
    /// Void (无效)
    Void,
    /// Voidable (可撤销)
    Voidable,
    /// Effective pending ratification (效力待定)
    EffectivePendingRatification,
}

impl ContractValidityStatus {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Valid => BilingualText::new("有效", "Valid"),
            Self::Void => BilingualText::new("无效", "Void"),
            Self::Voidable => BilingualText::new("可撤销", "Voidable"),
            Self::EffectivePendingRatification => {
                BilingualText::new("效力待定", "Effective pending ratification")
            }
        }
    }
}

/// Contract type (合同类型)
///
/// Articles 595-988 - 29 specific contract types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale contract (买卖合同) - Articles 595-645
    Sale,
    /// Electricity, water, gas, heat supply (电、水、气、热力供应合同) - Article 646
    UtilitySupply,
    /// Gift contract (赠与合同) - Articles 657-666
    Gift,
    /// Loan contract (借款合同) - Articles 667-680
    Loan,
    /// Secured loan (保证合同) - Articles 681-702
    Guarantee,
    /// Lease contract (租赁合同) - Articles 703-734
    Lease,
    /// Finance lease (融资租赁合同) - Articles 735-760
    FinanceLease,
    /// Factoring (保理合同) - Articles 761-769
    Factoring,
    /// Contract for work (承揽合同) - Articles 770-787
    ContractForWork,
    /// Construction contract (建设工程合同) - Articles 788-808
    Construction,
    /// Transportation contract (运输合同) - Articles 809-842
    Transportation,
    /// Technology contract (技术合同) - Articles 843-887
    Technology,
    /// Custody contract (保管合同) - Articles 888-903
    Custody,
    /// Warehousing contract (仓储合同) - Articles 904-916
    Warehousing,
    /// Commission contract (委托合同) - Articles 919-936
    Commission,
    /// Brokerage contract (行纪合同) - Articles 951-960
    Brokerage,
    /// Intermediary contract (中介合同) - Articles 961-966
    Intermediary,
    /// Partnership contract (合伙合同) - Articles 967-978
    Partnership,
    /// Other
    Other,
}

impl ContractType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Sale => BilingualText::new("买卖合同", "Sale contract"),
            Self::UtilitySupply => {
                BilingualText::new("电、水、气、热力供应合同", "Utility supply contract")
            }
            Self::Gift => BilingualText::new("赠与合同", "Gift contract"),
            Self::Loan => BilingualText::new("借款合同", "Loan contract"),
            Self::Guarantee => BilingualText::new("保证合同", "Guarantee contract"),
            Self::Lease => BilingualText::new("租赁合同", "Lease contract"),
            Self::FinanceLease => BilingualText::new("融资租赁合同", "Finance lease contract"),
            Self::Factoring => BilingualText::new("保理合同", "Factoring contract"),
            Self::ContractForWork => BilingualText::new("承揽合同", "Contract for work"),
            Self::Construction => BilingualText::new("建设工程合同", "Construction contract"),
            Self::Transportation => BilingualText::new("运输合同", "Transportation contract"),
            Self::Technology => BilingualText::new("技术合同", "Technology contract"),
            Self::Custody => BilingualText::new("保管合同", "Custody contract"),
            Self::Warehousing => BilingualText::new("仓储合同", "Warehousing contract"),
            Self::Commission => BilingualText::new("委托合同", "Commission contract"),
            Self::Brokerage => BilingualText::new("行纪合同", "Brokerage contract"),
            Self::Intermediary => BilingualText::new("中介合同", "Intermediary contract"),
            Self::Partnership => BilingualText::new("合伙合同", "Partnership contract"),
            Self::Other => BilingualText::new("其他合同", "Other contract"),
        }
    }
}

/// Contract (合同)
///
/// Articles 463-502
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub id: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Title
    pub title: BilingualText,
    /// Party A
    pub party_a: String,
    /// Party B
    pub party_b: String,
    /// Formation date
    pub formation_date: DateTime<Utc>,
    /// Formation status
    pub formation_status: ContractFormationStatus,
    /// Validity status
    pub validity_status: ContractValidityStatus,
    /// Subject matter (标的)
    pub subject_matter: BilingualText,
    /// Price/Consideration
    pub price: Option<f64>,
    /// Currency
    pub currency: Option<String>,
    /// Performance period
    pub performance_period: Option<PerformancePeriod>,
}

/// Performance period (履行期限)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePeriod {
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: DateTime<Utc>,
}

impl PerformancePeriod {
    /// Check if performance period has started
    pub fn has_started(&self, current_date: DateTime<Utc>) -> bool {
        current_date >= self.start_date
    }

    /// Check if performance period has expired
    pub fn has_expired(&self, current_date: DateTime<Utc>) -> bool {
        current_date > self.end_date
    }

    /// Check if within performance period
    pub fn is_within_period(&self, current_date: DateTime<Utc>) -> bool {
        self.has_started(current_date) && !self.has_expired(current_date)
    }
}

/// Breach of contract (违约)
///
/// Articles 577-592
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachOfContract {
    /// Breaching party
    pub breaching_party: String,
    /// Non-breaching party
    pub non_breaching_party: String,
    /// Description of breach
    pub breach_description: BilingualText,
    /// Date of breach
    pub breach_date: DateTime<Utc>,
    /// Breach type
    pub breach_type: BreachType,
}

/// Type of breach (违约类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Non-performance (不履行)
    NonPerformance,
    /// Delayed performance (迟延履行)
    DelayedPerformance,
    /// Defective performance (不完全履行)
    DefectivePerformance,
    /// Anticipatory breach (预期违约)
    AnticipatoryBreach,
}

impl BreachType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::NonPerformance => BilingualText::new("不履行", "Non-performance"),
            Self::DelayedPerformance => BilingualText::new("迟延履行", "Delayed performance"),
            Self::DefectivePerformance => BilingualText::new("不完全履行", "Defective performance"),
            Self::AnticipatoryBreach => BilingualText::new("预期违约", "Anticipatory breach"),
        }
    }
}

/// Remedy for breach (违约救济)
///
/// Articles 577-592
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachRemedy {
    /// Continued performance (继续履行) - Article 579
    ContinuedPerformance,
    /// Remedial measures (采取补救措施) - Article 582
    RemedialMeasures,
    /// Damages (损害赔偿) - Article 584
    Damages,
    /// Liquidated damages (违约金) - Article 585
    LiquidatedDamages,
    /// Termination (解除合同) - Article 563
    Termination,
}

impl BreachRemedy {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::ContinuedPerformance => BilingualText::new("继续履行", "Continued performance"),
            Self::RemedialMeasures => BilingualText::new("采取补救措施", "Remedial measures"),
            Self::Damages => BilingualText::new("损害赔偿", "Damages"),
            Self::LiquidatedDamages => BilingualText::new("违约金", "Liquidated damages"),
            Self::Termination => BilingualText::new("解除合同", "Termination"),
        }
    }
}

/// Sale contract (买卖合同)
///
/// Articles 595-645
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleContract {
    /// Base contract
    pub base: Contract,
    /// Seller (出卖人)
    pub seller: String,
    /// Buyer (买受人)
    pub buyer: String,
    /// Goods description (标的物)
    pub goods: BilingualText,
    /// Quantity
    pub quantity: f64,
    /// Unit
    pub unit: String,
    /// Total price
    pub total_price: f64,
    /// Currency
    pub currency: String,
    /// Delivery date
    pub delivery_date: DateTime<Utc>,
    /// Delivery location
    pub delivery_location: String,
}

/// Lease contract (租赁合同)
///
/// Articles 703-734
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseContract {
    /// Base contract
    pub base: Contract,
    /// Lessor (出租人)
    pub lessor: String,
    /// Lessee (承租人)
    pub lessee: String,
    /// Leased property (租赁物)
    pub leased_property: BilingualText,
    /// Rent amount per period
    pub rent_per_period: f64,
    /// Currency
    pub currency: String,
    /// Payment period (monthly, yearly, etc.)
    pub payment_period: BilingualText,
    /// Lease term (years)
    pub lease_term_years: f64,
    /// Start date
    pub start_date: DateTime<Utc>,
}

impl LeaseContract {
    /// Get end date
    pub fn end_date(&self) -> DateTime<Utc> {
        self.start_date + chrono::Duration::days((365.0 * self.lease_term_years) as i64)
    }

    /// Check if lease exceeds maximum term (20 years)
    ///
    /// Article 705
    pub fn exceeds_maximum_term(&self) -> bool {
        self.lease_term_years > 20.0
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate contract formation
///
/// Articles 469-502
pub fn validate_contract_formation(contract: &Contract) -> Result<(), ContractsError> {
    // Check that both parties exist
    if contract.party_a.is_empty() || contract.party_b.is_empty() {
        return Err(ContractsError::MissingParty {
            contract_id: contract.id.clone(),
        });
    }

    // Check formation status
    if contract.formation_status != ContractFormationStatus::Formed {
        return Err(ContractsError::ContractNotFormed {
            contract_id: contract.id.clone(),
            status: contract.formation_status,
        });
    }

    Ok(())
}

/// Validate contract validity
///
/// Articles 502-508
pub fn validate_contract_validity(contract: &Contract) -> Result<(), ContractsError> {
    // Check formation first
    validate_contract_formation(contract)?;

    // Check validity status
    match contract.validity_status {
        ContractValidityStatus::Valid => Ok(()),
        ContractValidityStatus::Void => Err(ContractsError::ContractVoid {
            contract_id: contract.id.clone(),
            reason: BilingualText::new("合同无效", "Contract is void"),
        }),
        ContractValidityStatus::Voidable => Err(ContractsError::ContractVoidable {
            contract_id: contract.id.clone(),
        }),
        ContractValidityStatus::EffectivePendingRatification => {
            Err(ContractsError::ContractPendingRatification {
                contract_id: contract.id.clone(),
            })
        }
    }
}

/// Validate lease contract term
///
/// Article 705: Lease term cannot exceed 20 years
pub fn validate_lease_term(lease: &LeaseContract) -> Result<(), ContractsError> {
    if lease.exceeds_maximum_term() {
        Err(ContractsError::LeaseTermExceedsMaximum {
            term_years: lease.lease_term_years,
            max_years: 20.0,
        })
    } else {
        Ok(())
    }
}

/// Calculate damages for breach of contract
///
/// Article 584: Damages = actual losses + lost profits
pub fn calculate_damages(actual_losses: f64, lost_profits: f64, mitigation_reduction: f64) -> f64 {
    (actual_losses + lost_profits - mitigation_reduction).max(0.0)
}

/// Determine available remedies for breach
///
/// Articles 577-592
pub fn determine_breach_remedies(
    breach: &BreachOfContract,
    liquidated_damages_clause: bool,
) -> Vec<BreachRemedy> {
    let mut remedies = Vec::new();

    match breach.breach_type {
        BreachType::NonPerformance => {
            remedies.push(BreachRemedy::ContinuedPerformance);
            remedies.push(BreachRemedy::Damages);
            remedies.push(BreachRemedy::Termination);
        }
        BreachType::DelayedPerformance => {
            remedies.push(BreachRemedy::ContinuedPerformance);
            if liquidated_damages_clause {
                remedies.push(BreachRemedy::LiquidatedDamages);
            }
            remedies.push(BreachRemedy::Damages);
        }
        BreachType::DefectivePerformance => {
            remedies.push(BreachRemedy::RemedialMeasures);
            remedies.push(BreachRemedy::Damages);
        }
        BreachType::AnticipatoryBreach => {
            remedies.push(BreachRemedy::Termination);
            remedies.push(BreachRemedy::Damages);
        }
    }

    remedies
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Contracts
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ContractsError {
    /// Missing party
    #[error("Contract {contract_id} is missing a party")]
    MissingParty {
        /// Contract ID
        contract_id: String,
    },

    /// Contract not formed
    #[error("Contract {contract_id} not formed (status: {status:?})")]
    ContractNotFormed {
        /// Contract ID
        contract_id: String,
        /// Formation status
        status: ContractFormationStatus,
    },

    /// Contract is void
    #[error("Contract {contract_id} is void: {reason}")]
    ContractVoid {
        /// Contract ID
        contract_id: String,
        /// Reason
        reason: BilingualText,
    },

    /// Contract is voidable
    #[error("Contract {contract_id} is voidable")]
    ContractVoidable {
        /// Contract ID
        contract_id: String,
    },

    /// Contract pending ratification
    #[error("Contract {contract_id} is pending ratification")]
    ContractPendingRatification {
        /// Contract ID
        contract_id: String,
    },

    /// Lease term exceeds maximum
    #[error("Lease term {term_years} years exceeds maximum {max_years} years")]
    LeaseTermExceedsMaximum {
        /// Term in years
        term_years: f64,
        /// Maximum years
        max_years: f64,
    },

    /// Breach of contract
    #[error("Breach of contract: {description}")]
    BreachOfContract {
        /// Description
        description: BilingualText,
    },
}

/// Result type for Contracts operations
pub type ContractsResult<T> = Result<T, ContractsError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_formation() {
        let contract = Contract {
            id: "C001".to_string(),
            contract_type: ContractType::Sale,
            title: BilingualText::new("商品买卖合同", "Sale contract"),
            party_a: "卖方".to_string(),
            party_b: "买方".to_string(),
            formation_date: Utc::now(),
            formation_status: ContractFormationStatus::Formed,
            validity_status: ContractValidityStatus::Valid,
            subject_matter: BilingualText::new("商品", "Goods"),
            price: Some(10000.0),
            currency: Some("CNY".to_string()),
            performance_period: None,
        };

        assert!(validate_contract_formation(&contract).is_ok());
        assert!(validate_contract_validity(&contract).is_ok());
    }

    #[test]
    fn test_lease_term_validation() {
        let lease = LeaseContract {
            base: Contract {
                id: "L001".to_string(),
                contract_type: ContractType::Lease,
                title: BilingualText::new("租赁合同", "Lease contract"),
                party_a: "出租人".to_string(),
                party_b: "承租人".to_string(),
                formation_date: Utc::now(),
                formation_status: ContractFormationStatus::Formed,
                validity_status: ContractValidityStatus::Valid,
                subject_matter: BilingualText::new("房屋", "Property"),
                price: Some(5000.0),
                currency: Some("CNY".to_string()),
                performance_period: None,
            },
            lessor: "出租人".to_string(),
            lessee: "承租人".to_string(),
            leased_property: BilingualText::new("商业用房", "Commercial property"),
            rent_per_period: 5000.0,
            currency: "CNY".to_string(),
            payment_period: BilingualText::new("月", "Monthly"),
            lease_term_years: 10.0,
            start_date: Utc::now(),
        };

        // 10 years is valid
        assert!(validate_lease_term(&lease).is_ok());
        assert!(!lease.exceeds_maximum_term());
    }

    #[test]
    fn test_lease_term_exceeds_maximum() {
        let lease = LeaseContract {
            base: Contract {
                id: "L002".to_string(),
                contract_type: ContractType::Lease,
                title: BilingualText::new("租赁合同", "Lease contract"),
                party_a: "出租人".to_string(),
                party_b: "承租人".to_string(),
                formation_date: Utc::now(),
                formation_status: ContractFormationStatus::Formed,
                validity_status: ContractValidityStatus::Valid,
                subject_matter: BilingualText::new("房屋", "Property"),
                price: Some(5000.0),
                currency: Some("CNY".to_string()),
                performance_period: None,
            },
            lessor: "出租人".to_string(),
            lessee: "承租人".to_string(),
            leased_property: BilingualText::new("商业用房", "Commercial property"),
            rent_per_period: 5000.0,
            currency: "CNY".to_string(),
            payment_period: BilingualText::new("月", "Monthly"),
            lease_term_years: 25.0, // Exceeds 20 years
            start_date: Utc::now(),
        };

        assert!(validate_lease_term(&lease).is_err());
        assert!(lease.exceeds_maximum_term());
    }

    #[test]
    fn test_calculate_damages() {
        let damages = calculate_damages(10000.0, 5000.0, 1000.0);
        assert_eq!(damages, 14000.0);

        // Test mitigation exceeds losses
        let damages2 = calculate_damages(10000.0, 5000.0, 20000.0);
        assert_eq!(damages2, 0.0); // Cannot be negative
    }

    #[test]
    fn test_breach_remedies() {
        let breach = BreachOfContract {
            breaching_party: "甲方".to_string(),
            non_breaching_party: "乙方".to_string(),
            breach_description: BilingualText::new("未履行合同", "Non-performance"),
            breach_date: Utc::now(),
            breach_type: BreachType::NonPerformance,
        };

        let remedies = determine_breach_remedies(&breach, false);
        assert!(remedies.contains(&BreachRemedy::ContinuedPerformance));
        assert!(remedies.contains(&BreachRemedy::Damages));
        assert!(remedies.contains(&BreachRemedy::Termination));
    }

    #[test]
    fn test_performance_period() {
        let now = Utc::now();
        let period = PerformancePeriod {
            start_date: now - chrono::Duration::days(10),
            end_date: now + chrono::Duration::days(10),
        };

        assert!(period.has_started(now));
        assert!(!period.has_expired(now));
        assert!(period.is_within_period(now));
    }
}
