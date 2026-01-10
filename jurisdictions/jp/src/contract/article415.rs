//! Japanese Civil Code Article 415 implementation
//!
//! Provides builder pattern API for breach of obligation claims.
//!
//! ## Article 415 (民法第415条 - 債務不履行による損害賠償)
//!
//! > 債務者がその債務の本旨に従った履行をしないときは、債権者は、これによって生じた損害の賠償を請求することができる。
//! > ただし、その債務の不履行が契約その他の債務の発生原因及び取引上の社会通念に照らして債務者の責めに帰することができない事由によるものであるときは、この限りでない。
//!
//! English: If an obligor fails to perform the obligation in accordance with the purpose thereof,
//! the obligee may request damages arising therefrom; provided, however, that this does not apply
//! if the non-performance is due to grounds not attributable to the obligor in light of the
//! contract or other sources of obligation and common sense in transactions.

use crate::contract::error::ContractLiabilityError;
use crate::contract::types::{Attribution, BreachType, ObligationType};
use crate::tort::types::{CausalLink, Damage}; // Reuse from tort module

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Article 415 breach of obligation claim builder
///
/// Represents a contract breach claim under Article 415.
///
/// ## Example
///
/// ```rust
/// use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
/// use legalis_jp::tort::{Damage, CausalLink};
///
/// let claim = Article415::new()
///     .with_obligation(ObligationType::Monetary {
///         amount: 1_000_000,
///         currency: "JPY".to_string(),
///     })
///     .with_breach(BreachType::NonPerformance)
///     .with_attribution(Attribution::new(
///         AttributionType::Negligence,
///         "正当な理由なく履行を拒否"
///     ))
///     .with_damage(Damage::new(1_000_000, "契約金額"))
///     .with_causal_link(CausalLink::Direct)
///     .creditor("会社A")
///     .debtor("供給業者B");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Article415<'a> {
    /// Obligation that exists (債務の存在)
    pub obligation: Option<ObligationType>,

    /// Type of breach/non-performance (不履行の種類)
    pub breach: Option<BreachType>,

    /// Attribution to debtor (帰責事由)
    pub attribution: Option<Attribution>,

    /// Causal link between breach and damages (因果関係)
    pub causal_link: Option<CausalLink<'a>>,

    /// Damages occurred (損害)
    pub damage: Option<Damage>,

    /// Creditor/obligee (債権者)
    pub creditor: Option<String>,

    /// Debtor/obligor (債務者)
    pub debtor: Option<String>,

    /// Contract date (契約日) - optional
    pub contract_date: Option<String>,

    /// Due date for performance (履行期日) - optional
    pub due_date: Option<String>,
}

impl<'a> Article415<'a> {
    /// Create a new Article 415 claim builder
    pub fn new() -> Self {
        Self {
            obligation: None,
            breach: None,
            attribution: None,
            causal_link: None,
            damage: None,
            creditor: None,
            debtor: None,
            contract_date: None,
            due_date: None,
        }
    }

    /// Set the obligation (債務の存在)
    pub fn with_obligation(mut self, obligation: ObligationType) -> Self {
        self.obligation = Some(obligation);
        self
    }

    /// Alternative method name for setting obligation
    pub fn obligation(mut self, obligation: ObligationType) -> Self {
        self.obligation = Some(obligation);
        self
    }

    /// Set the type of breach (不履行の種類)
    pub fn with_breach(mut self, breach: BreachType) -> Self {
        self.breach = Some(breach);
        self
    }

    /// Alternative method name for setting breach
    pub fn breach(mut self, breach: BreachType) -> Self {
        self.breach = Some(breach);
        self
    }

    /// Set attribution (帰責事由)
    pub fn with_attribution(mut self, attribution: Attribution) -> Self {
        self.attribution = Some(attribution);
        self
    }

    /// Alternative method name for setting attribution
    pub fn attribution(mut self, attribution: Attribution) -> Self {
        self.attribution = Some(attribution);
        self
    }

    /// Set causal link (因果関係)
    pub fn with_causal_link(mut self, link: CausalLink<'a>) -> Self {
        self.causal_link = Some(link);
        self
    }

    /// Alternative method name for setting causal link
    pub fn causal_link(mut self, link: CausalLink<'a>) -> Self {
        self.causal_link = Some(link);
        self
    }

    /// Set damage (損害)
    pub fn with_damage(mut self, damage: Damage) -> Self {
        self.damage = Some(damage);
        self
    }

    /// Alternative method name for setting damage
    pub fn damage(mut self, damage: Damage) -> Self {
        self.damage = Some(damage);
        self
    }

    /// Set creditor/obligee (債権者)
    pub fn creditor(mut self, creditor: impl Into<String>) -> Self {
        self.creditor = Some(creditor.into());
        self
    }

    /// Set debtor/obligor (債務者)
    pub fn debtor(mut self, debtor: impl Into<String>) -> Self {
        self.debtor = Some(debtor.into());
        self
    }

    /// Set contract date (契約日)
    pub fn contract_date(mut self, date: impl Into<String>) -> Self {
        self.contract_date = Some(date.into());
        self
    }

    /// Set due date for performance (履行期日)
    pub fn with_due_date(mut self, date: impl Into<String>) -> Self {
        self.due_date = Some(date.into());
        self
    }

    /// Build the claim (finalizes the builder)
    pub fn build(self) -> Result<Article415<'a>, ContractLiabilityError> {
        if self.obligation.is_none() {
            return Err(ContractLiabilityError::MissingField(
                "obligation".to_string(),
            ));
        }
        if self.breach.is_none() {
            return Err(ContractLiabilityError::MissingField("breach".to_string()));
        }
        if self.attribution.is_none() {
            return Err(ContractLiabilityError::MissingField(
                "attribution".to_string(),
            ));
        }
        if self.causal_link.is_none() {
            return Err(ContractLiabilityError::MissingField(
                "causal_link".to_string(),
            ));
        }
        if self.damage.is_none() {
            return Err(ContractLiabilityError::MissingField("damage".to_string()));
        }
        Ok(self)
    }

    /// Validate the Article 415 claim
    pub fn validate(&self) -> Result<(), ContractLiabilityError> {
        crate::contract::validator::validate_breach_claim(self).map(|_| ())
    }

    /// Get estimated damages amount
    pub fn estimated_damages(&self) -> u64 {
        self.damage.as_ref().map(|d| d.amount).unwrap_or(0)
    }
}

impl<'a> Default for Article415<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of Article 415 breach liability validation (415条検証結果)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BreachLiability {
    /// Reference to Article 415
    pub article: ArticleReference,

    /// Liability status
    pub status: LiabilityStatus,

    /// Detailed validation results
    pub validation_details: Vec<String>,

    /// Compensation basis (賠償の基礎)
    pub compensation_basis: Option<String>,
}

impl BreachLiability {
    /// Check if breach liability is established
    pub fn is_liability_established(&self) -> bool {
        matches!(self.status, LiabilityStatus::Established)
    }
}

/// Article reference for Article 415
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArticleReference {
    /// Article number
    pub number: String,

    /// Article title
    pub title: String,
}

impl Default for ArticleReference {
    fn default() -> Self {
        Self {
            number: "415".to_string(),
            title: "債務不履行による損害賠償 (Breach of Obligation Liability)".to_string(),
        }
    }
}

/// Liability status under Article 415 (責任の状態)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LiabilityStatus {
    /// Liability established (責任成立)
    Established,

    /// Liability not established (責任不成立)
    NotEstablished,

    /// Insufficient evidence (立証不足)
    InsufficientEvidence(String),

    /// Requires judicial determination (司法判断を要する)
    RequiresJudicialDetermination { reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::types::*;
    use crate::tort::types::{CausalLink, Damage};

    #[test]
    fn test_article415_builder() {
        let claim = Article415::new()
            .with_obligation(ObligationType::Monetary {
                amount: 1_000_000,
                currency: "JPY".to_string(),
            })
            .with_breach(BreachType::NonPerformance)
            .with_attribution(Attribution::new(
                AttributionType::Negligence,
                "正当な理由なく履行を拒否",
            ))
            .with_damage(Damage::new(1_000_000, "契約金額"))
            .with_causal_link(CausalLink::Direct);

        assert!(claim.obligation.is_some());
        assert!(claim.breach.is_some());
        assert!(claim.attribution.is_some());
        assert!(claim.causal_link.is_some());
        assert!(claim.damage.is_some());
    }

    #[test]
    fn test_build_with_missing_field() {
        let result = Article415::new()
            .with_obligation(ObligationType::Monetary {
                amount: 1_000_000,
                currency: "JPY".to_string(),
            })
            .build();

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err, ContractLiabilityError::MissingField(_)));
        }
    }

    #[test]
    fn test_estimated_damages() {
        let claim = Article415::new().with_damage(Damage::new(3_000_000, "契約違反による損害"));

        assert_eq!(claim.estimated_damages(), 3_000_000);
    }

    #[test]
    fn test_breach_type_variants() {
        let delayed = BreachType::DelayedPerformance { days_late: 30 };
        assert!(matches!(delayed, BreachType::DelayedPerformance { .. }));

        let defective = BreachType::DefectivePerformance {
            description: "品質不良".to_string(),
        };
        assert!(matches!(defective, BreachType::DefectivePerformance { .. }));
    }
}
