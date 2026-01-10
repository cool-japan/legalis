//! Japanese Civil Code Article 709 implementation
//!
//! Provides builder pattern API for constructing and validating tort claims
//! under Article 709 (不法行為による損害賠償).

use crate::tort::error::{TortClaimError, ValidationError};
use crate::tort::types::{CausalLink, Damage, Intent, ProtectedInterest};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Article 709 tort claim builder
///
/// Represents a tort liability claim under Article 709 of the Japanese Civil Code.
///
/// ## Article 709 (民法第709条)
///
/// > 故意又は過失によって他人の権利又は法律上保護される利益を侵害した者は、
/// > これによって生じた損害を賠償する責任を負う。
///
/// English: A person who has intentionally or negligently infringed any right of another
/// person, or a legally protected interest, shall be liable to compensate the other
/// person for damage arising therefrom.
///
/// ## Example
///
/// ```rust
/// use legalis_jp::tort::{Article709, Intent, Damage, CausalLink, ProtectedInterest};
///
/// let claim = Article709::new()
///     .with_act("交通事故で相手の車に衝突")
///     .with_intent(Intent::Negligence)
///     .with_victim_interest(ProtectedInterest::Property("車両所有権"))
///     .with_damage(Damage::new(500_000, "修理費"))
///     .with_causal_link(CausalLink::Direct);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub struct Article709<'a> {
    /// Tortious act (加害行為)
    pub act: Option<String>,

    /// Intent or negligence (故意・過失)
    pub intent: Option<Intent>,

    /// Victim's protected interest (被害法益)
    pub victim_interest: Option<ProtectedInterest<'a>>,

    /// Damage (損害)
    pub damage: Option<Damage>,

    /// Causal link (因果関係)
    pub causal_link: Option<CausalLink<'a>>,

    /// Plaintiff (原告)
    pub plaintiff: Option<String>,

    /// Defendant/tortfeasor (被告/加害者)
    pub defendant: Option<String>,

    /// Responsibility capacity (責任能力) - default true
    pub responsibility_capacity: bool,
}

impl<'a> Article709<'a> {
    /// Create a new Article 709 tort claim builder
    pub fn new() -> Self {
        Self {
            act: None,
            intent: None,
            victim_interest: None,
            damage: None,
            causal_link: None,
            plaintiff: None,
            defendant: None,
            responsibility_capacity: true,
        }
    }

    /// Create a builder (alias for new())
    pub fn builder() -> Self {
        Self::new()
    }

    /// Set the tortious act (加害行為)
    pub fn with_act(mut self, act: impl Into<String>) -> Self {
        self.act = Some(act.into());
        self
    }

    /// Set the act (alternative method name)
    pub fn act(mut self, act: impl Into<String>) -> Self {
        self.act = Some(act.into());
        self
    }

    /// Set intent or negligence (故意・過失)
    pub fn with_intent(mut self, intent: Intent) -> Self {
        self.intent = Some(intent);
        self
    }

    /// Set intent (alternative method name)
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = Some(intent);
        self
    }

    /// Set victim's protected interest (被害法益)
    pub fn with_victim_interest(mut self, interest: ProtectedInterest<'a>) -> Self {
        self.victim_interest = Some(interest);
        self
    }

    /// Set injured interest (alternative method name)
    pub fn injured_interest(mut self, interest: ProtectedInterest<'a>) -> Self {
        self.victim_interest = Some(interest);
        self
    }

    /// Set damage (損害)
    pub fn with_damage(mut self, damage: Damage) -> Self {
        self.damage = Some(damage);
        self
    }

    /// Set damage (alternative method name)
    pub fn damage(mut self, damage: Damage) -> Self {
        self.damage = Some(damage);
        self
    }

    /// Set causal link (因果関係)
    pub fn with_causal_link(mut self, link: CausalLink<'a>) -> Self {
        self.causal_link = Some(link);
        self
    }

    /// Set causal link (alternative method name)
    pub fn causal_link(mut self, link: CausalLink<'a>) -> Self {
        self.causal_link = Some(link);
        self
    }

    /// Set plaintiff (原告)
    pub fn plaintiff(mut self, plaintiff: impl Into<String>) -> Self {
        self.plaintiff = Some(plaintiff.into());
        self
    }

    /// Set defendant/tortfeasor (被告)
    pub fn defendant(mut self, defendant: impl Into<String>) -> Self {
        self.defendant = Some(defendant.into());
        self
    }

    /// Set responsibility capacity (責任能力)
    pub fn responsibility_capacity(mut self, has_capacity: bool) -> Self {
        self.responsibility_capacity = has_capacity;
        self
    }

    /// Build the tort claim (finalizes the builder)
    pub fn build(self) -> Result<Article709<'a>, TortClaimError> {
        // Validate that all required fields are present
        if self.act.is_none() {
            return Err(TortClaimError::MissingField("act".to_string()));
        }
        if self.intent.is_none() {
            return Err(TortClaimError::MissingField("intent".to_string()));
        }
        if self.victim_interest.is_none() {
            return Err(TortClaimError::MissingField("victim_interest".to_string()));
        }
        if self.damage.is_none() {
            return Err(TortClaimError::MissingField("damage".to_string()));
        }
        if self.causal_link.is_none() {
            return Err(TortClaimError::MissingField("causal_link".to_string()));
        }

        Ok(self)
    }

    /// Validate the tort claim
    pub fn validate(&self) -> Result<(), ValidationError> {
        crate::tort::validator::validate_tort_claim(self).map(|_| ())
    }

    /// Check if tortfeasor has full responsibility capacity
    ///
    /// In Japanese law, children under 12 generally lack responsibility capacity.
    pub fn has_full_capacity(&self) -> bool {
        if !self.responsibility_capacity {
            return false;
        }

        // Check age from intent if applicable
        if let Some(Intent::Intentional { age }) = &self.intent {
            *age >= 12
        } else {
            true
        }
    }

    /// Estimate compensation amount (if damage is set)
    pub fn estimated_compensation(&self) -> u64 {
        self.damage.as_ref().map(|d| d.amount).unwrap_or(0)
    }
}

impl<'a> Default for Article709<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of tort liability validation (検証結果)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TortLiability {
    /// Reference to Article 709
    pub article: ArticleReference,

    /// Liability status
    pub status: LiabilityStatus,

    /// Detailed validation results
    pub validation_details: Vec<String>,
}

impl TortLiability {
    /// Check if liability is established
    pub fn is_liability_established(&self) -> bool {
        matches!(self.status, LiabilityStatus::Established)
    }

    /// Get recommended compensation amount (if liability established)
    pub fn recommended_compensation(&self) -> Option<u64> {
        if self.is_liability_established() {
            // This would involve complex calculation in real implementation
            // For now, return None (requires judicial determination)
            None
        } else {
            None
        }
    }
}

/// Article reference
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
            number: "709".to_string(),
            title: "不法行為による損害賠償 (Tort Liability)".to_string(),
        }
    }
}

/// Liability status under Article 709
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

/// Function to create Article 715(1) supervisor liability claim
///
/// Note: This is a placeholder for future implementation
pub fn article_715_1() -> Article715Builder {
    Article715Builder::new()
}

/// Builder for Article 715(1) - Employer/Supervisor Liability
///
/// This is a simplified placeholder for future full implementation
#[derive(Debug, Clone)]
pub struct Article715Builder {
    supervisor: Option<String>,
    duty_violation: bool,
    linked_act: Option<String>,
}

impl Article715Builder {
    pub fn new() -> Self {
        Self {
            supervisor: None,
            duty_violation: false,
            linked_act: None,
        }
    }

    pub fn supervisor(mut self, supervisor: impl Into<String>) -> Self {
        self.supervisor = Some(supervisor.into());
        self
    }

    pub fn duty_violation(mut self, violated: bool) -> Self {
        self.duty_violation = violated;
        self
    }

    pub fn link_to_child_act(mut self, _claim: &Article709) -> Self {
        self.linked_act = Some("child_tort".to_string());
        self
    }

    pub fn is_liability_established(&self) -> bool {
        self.supervisor.is_some() && self.duty_violation && self.linked_act.is_some()
    }
}

impl Default for Article715Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tort::types::*;

    #[test]
    fn test_article709_builder() {
        let claim = Article709::new()
            .with_act("交通事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(500_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        assert!(claim.act.is_some());
        assert!(claim.intent.is_some());
        assert!(claim.victim_interest.is_some());
        assert!(claim.damage.is_some());
        assert!(claim.causal_link.is_some());
    }

    #[test]
    fn test_build_with_missing_field() {
        let result = Article709::new().with_act("交通事故").build();

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err, TortClaimError::MissingField(_)));
        }
    }

    #[test]
    fn test_has_full_capacity() {
        let claim = Article709::new().with_intent(Intent::Intentional { age: 10 });

        assert!(!claim.has_full_capacity());

        let claim2 = Article709::new().with_intent(Intent::Intentional { age: 20 });

        assert!(claim2.has_full_capacity());
    }

    #[test]
    fn test_estimated_compensation() {
        let claim = Article709::new().with_damage(Damage::new(3_000_000, "治療費"));

        assert_eq!(claim.estimated_compensation(), 3_000_000);
    }
}
