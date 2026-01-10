//! Consumer Rights Act 2015 Types
//!
//! Core data structures for UK consumer law covering:
//! - Goods contracts (Part 1 Chapter 2)
//! - Services contracts (Part 1 Chapter 4)
//! - Digital content (Part 1 Chapter 3)
//! - Remedies (tiered system)
//! - Unfair terms (Part 2)

#![allow(missing_docs)]

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Consumer contract type under CRA 2015
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumerContract {
    /// Goods contract (CRA 2015 Part 1 Chapter 2)
    Goods(GoodsContract),

    /// Services contract (CRA 2015 Part 1 Chapter 4)
    Services(ServicesContract),

    /// Digital content contract (CRA 2015 Part 1 Chapter 3)
    DigitalContent(DigitalContentContract),

    /// Mixed contract (goods + services)
    Mixed {
        goods: Box<GoodsContract>,
        services: Box<ServicesContract>,
    },
}

/// Goods contract under CRA 2015 Part 1 Chapter 2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsContract {
    /// Goods description
    pub description: String,

    /// Price paid
    pub price_gbp: f64,

    /// Date of purchase
    pub purchase_date: NaiveDate,

    /// Trader (seller)
    pub trader: Trader,

    /// Consumer (buyer)
    pub consumer: Consumer,

    /// Statutory rights under CRA 2015
    pub statutory_rights: Vec<GoodsStatutoryRight>,

    /// Current remedy stage
    pub remedy_stage: Option<RemedyStage>,
}

/// Services contract under CRA 2015 Part 1 Chapter 4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesContract {
    /// Service description
    pub description: String,

    /// Price agreed (if any)
    pub price_gbp: Option<f64>,

    /// Date service commenced
    pub commencement_date: NaiveDate,

    /// Date service completed (if applicable)
    pub completion_date: Option<NaiveDate>,

    /// Trader (service provider)
    pub trader: Trader,

    /// Consumer
    pub consumer: Consumer,

    /// Statutory rights under CRA 2015
    pub statutory_rights: Vec<ServicesStatutoryRight>,
}

/// Digital content contract under CRA 2015 Part 1 Chapter 3
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalContentContract {
    /// Digital content description
    pub description: String,

    /// Price paid (£0 if free)
    pub price_gbp: f64,

    /// Date of supply
    pub supply_date: NaiveDate,

    /// Trader (supplier)
    pub trader: Trader,

    /// Consumer
    pub consumer: Consumer,

    /// Statutory rights under CRA 2015
    pub statutory_rights: Vec<DigitalContentStatutoryRight>,

    /// Type of digital content
    pub content_type: DigitalContentType,
}

/// Trader (business)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trader {
    /// Trading name
    pub name: String,

    /// Business address
    pub address: String,

    /// Contact details
    pub contact: String,

    /// Company registration number (if limited company)
    pub company_number: Option<String>,
}

/// Consumer (individual acting outside business)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consumer {
    /// Name
    pub name: String,

    /// Address
    pub address: String,

    /// Contact details
    pub contact: String,
}

/// Statutory rights for goods under CRA 2015 ss.9-11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoodsStatutoryRight {
    /// s.9: Satisfactory quality
    /// Goods must meet standard that reasonable person would regard as satisfactory
    SatisfactoryQuality,

    /// s.10: Fit for particular purpose
    /// Where consumer makes known particular purpose
    FitForPurpose,

    /// s.11: As described
    /// Goods must match description given
    AsDescribed,

    /// s.12: Match sample
    /// Where goods sold by sample
    MatchSample,

    /// s.13: Match model seen
    /// Where goods shown before purchase
    MatchModel,
}

/// Statutory rights for services under CRA 2015 ss.49-52
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServicesStatutoryRight {
    /// s.49: Reasonable care and skill
    /// Service must be performed with reasonable care and skill
    ReasonableCareAndSkill,

    /// s.50: Information binding
    /// Information given about trader/service is binding
    InformationBinding,

    /// s.51: Reasonable price
    /// Where price not agreed, must be reasonable
    ReasonablePrice,

    /// s.52: Reasonable time
    /// Service performed within reasonable time
    ReasonableTime,
}

/// Statutory rights for digital content under CRA 2015 ss.34-37
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalContentStatutoryRight {
    /// s.34: Satisfactory quality
    SatisfactoryQuality,

    /// s.35: Fit for particular purpose
    FitForPurpose,

    /// s.36: As described
    AsDescribed,

    /// s.37: Other pre-contract information included
    OtherInformationIncluded,
}

/// Type of digital content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalContentType {
    /// Software/apps
    Software,

    /// Music/audio
    Music,

    /// Video/films
    Video,

    /// E-books
    Ebooks,

    /// Games
    Games,

    /// Other digital content
    Other,
}

/// Remedy stage in the tiered remedy system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemedyStage {
    /// First tier: Short-term right to reject (30 days)
    /// CRA 2015 s.22
    ShortTermRightToReject {
        /// Deadline to reject (30 days from delivery)
        deadline: NaiveDate,
        /// Whether goods accepted
        accepted: bool,
    },

    /// Second tier: Repair or replacement (one attempt)
    /// CRA 2015 s.23
    RepairOrReplacement {
        /// Choice of remedy
        choice: SecondTierChoice,
        /// Whether repair/replacement attempted
        attempted: bool,
        /// Date of repair/replacement
        date: Option<NaiveDate>,
        /// Whether successful
        successful: Option<bool>,
    },

    /// Third tier: Price reduction or final right to reject
    /// CRA 2015 s.24
    FinalRemedy {
        /// Choice of final remedy
        choice: FinalRemedyChoice,
        /// Amount of price reduction (if applicable)
        price_reduction_gbp: Option<f64>,
    },
}

/// Second tier remedy choice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecondTierChoice {
    /// Repair the goods
    Repair,

    /// Replace the goods
    Replacement,
}

/// Final remedy choice (third tier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinalRemedyChoice {
    /// Price reduction
    PriceReduction,

    /// Final right to reject (full refund)
    FinalRightToReject,
}

/// Consumer remedy under CRA 2015
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerRemedy {
    /// Type of contract
    pub contract_type: ContractType,

    /// Breach details
    pub breach: ContractBreach,

    /// Available remedies
    pub available_remedies: Vec<RemedyType>,

    /// Time limits
    pub time_limits: Vec<TimeLimit>,
}

/// Contract type for remedy purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Goods
    Goods,

    /// Services
    Services,

    /// Digital content
    DigitalContent,
}

/// Contract breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractBreach {
    /// Description of breach
    pub description: String,

    /// Statutory right breached
    pub statutory_right: String,

    /// Date of discovery
    pub discovery_date: NaiveDate,

    /// Severity
    pub severity: BreachSeverity,
}

/// Breach severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachSeverity {
    /// Minor defect
    Minor,

    /// Moderate defect
    Moderate,

    /// Serious defect
    Serious,
}

/// Remedy type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemedyType {
    /// Short-term right to reject (30 days)
    ShortTermReject,

    /// Repair
    Repair,

    /// Replacement
    Replacement,

    /// Price reduction
    PriceReduction,

    /// Final right to reject
    FinalReject,

    /// Repeat performance (services)
    RepeatPerformance,
}

/// Time limit for remedy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeLimit {
    /// Remedy type
    pub remedy: RemedyType,

    /// Deadline
    pub deadline: NaiveDate,

    /// Expired
    pub expired: bool,
}

/// Unfair contract term under CRA 2015 Part 2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfairTerm {
    /// Term text
    pub term_text: String,

    /// Assessment
    pub assessment: UnfairTermAssessment,
}

/// Unfair term assessment under CRA 2015 s.62
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfairTermAssessment {
    /// Contrary to requirement of good faith
    pub contrary_to_good_faith: bool,

    /// Causes significant imbalance in parties' rights/obligations
    pub significant_imbalance: bool,

    /// To detriment of consumer
    pub detriment_to_consumer: bool,

    /// On grey list (Schedule 2)
    pub on_grey_list: Option<GreyListItem>,

    /// Transparent and prominent
    pub transparent_and_prominent: bool,
}

/// Grey list items from CRA 2015 Schedule 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GreyListItem {
    /// Para 1: Exclude/limit liability for death/personal injury
    ExcludeLiabilityDeathInjury,

    /// Para 2: Inappropriately exclude/limit consumer's legal rights
    ExcludeConsumerRights,

    /// Para 5: Require excessive compensation for cancellation
    ExcessiveCancellationFee,

    /// Para 6: Trader may dissolve contract without same right for consumer
    AsymmetricTermination,

    /// Para 8: Automatically extend contract unless consumer objects
    AutomaticRenewal,

    /// Para 11: Trader may change terms without valid reason
    UnilateralTermChange,

    /// Para 14: Trader determines compliance with contract
    TraderDeterminesCompliance,

    /// Para 16: Exclusive jurisdiction clause limiting consumer's right to sue
    ExclusiveJurisdiction,

    /// Para 20: Trader may increase price without consumer's right to cancel
    PriceIncreaseWithoutExit,
}

impl GoodsContract {
    /// Check if short-term right to reject is still available
    pub fn can_short_term_reject(&self) -> bool {
        let days_since_purchase = (Utc::now().date_naive() - self.purchase_date).num_days();
        days_since_purchase <= 30
    }

    /// Get applicable remedy stage
    pub fn current_remedy_stage(&self) -> RemedyStage {
        if self.can_short_term_reject() {
            RemedyStage::ShortTermRightToReject {
                deadline: self.purchase_date + chrono::Duration::days(30),
                accepted: false,
            }
        } else {
            RemedyStage::RepairOrReplacement {
                choice: SecondTierChoice::Repair,
                attempted: false,
                date: None,
                successful: None,
            }
        }
    }
}

impl UnfairTermAssessment {
    /// Check if term is unfair under CRA 2015 s.62
    pub fn is_unfair(&self) -> bool {
        // A term is unfair if:
        // 1. Contrary to good faith, AND
        // 2. Causes significant imbalance, AND
        // 3. To detriment of consumer
        // UNLESS: Core term and transparent and prominent

        let meets_unfairness_test =
            self.contrary_to_good_faith && self.significant_imbalance && self.detriment_to_consumer;

        // Grey list items are indicative (not conclusive)
        // Transparency exception does not apply to grey list items
        if self.on_grey_list.is_some() {
            return meets_unfairness_test;
        }

        // For non-grey-list terms, transparency may save them
        if self.transparent_and_prominent {
            return false;
        }

        meets_unfairness_test
    }

    /// Get unfairness score (0-100)
    pub fn unfairness_score(&self) -> u8 {
        let mut score = 0u8;

        if self.contrary_to_good_faith {
            score += 30;
        }
        if self.significant_imbalance {
            score += 30;
        }
        if self.detriment_to_consumer {
            score += 20;
        }
        if self.on_grey_list.is_some() {
            score += 15;
        }
        if !self.transparent_and_prominent {
            score += 5;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_term_reject_within_30_days() {
        let contract = GoodsContract {
            description: "Laptop".to_string(),
            price_gbp: 500.0,
            purchase_date: Utc::now().date_naive() - chrono::Duration::days(15),
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

        assert!(contract.can_short_term_reject());
    }

    #[test]
    fn test_short_term_reject_expired() {
        let contract = GoodsContract {
            description: "Laptop".to_string(),
            price_gbp: 500.0,
            purchase_date: Utc::now().date_naive() - chrono::Duration::days(35),
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

        assert!(!contract.can_short_term_reject());
    }

    #[test]
    fn test_unfair_term_meets_all_criteria() {
        let assessment = UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: None,
            transparent_and_prominent: false,
        };

        assert!(assessment.is_unfair());
        assert!(assessment.unfairness_score() >= 80);
    }

    #[test]
    fn test_unfair_term_transparent_saves_non_grey_list() {
        let assessment = UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: None,
            transparent_and_prominent: true,
        };

        // Transparent and prominent saves it (if not on grey list)
        assert!(!assessment.is_unfair());
    }

    #[test]
    fn test_unfair_term_grey_list_always_unfair() {
        let assessment = UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: Some(GreyListItem::ExcludeLiabilityDeathInjury),
            transparent_and_prominent: true, // Doesn't help on grey list
        };

        assert!(assessment.is_unfair());
    }
}
