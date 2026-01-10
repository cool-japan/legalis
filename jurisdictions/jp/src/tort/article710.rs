//! Japanese Civil Code Article 710 implementation
//!
//! Provides builder pattern API for non-pecuniary damages (慰謝料) under Article 710.
//!
//! ## Article 710 (民法第710条 - 財産以外の損害の賠償)
//!
//! > 他人の身体、自由若しくは名誉を侵害した場合又は他人の財産権を侵害した場合のいずれであるかを問わず、
//! > 前条の規定により損害賠償の責任を負う者は、財産以外の損害に対しても、その賠償をしなければならない。
//!
//! English: A person who is liable to compensate for damage under the provisions of the
//! preceding Article (Article 709) shall compensate for non-pecuniary damage as well,
//! regardless of whether the victim's body, liberty, reputation, or property rights were infringed.

use crate::tort::article709::{Article709, ArticleReference};
use crate::tort::error::{TortClaimError, ValidationError};
use crate::tort::types::{HarmSeverity, NonPecuniaryDamageType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Article 710 non-pecuniary damages claim builder
///
/// Represents emotional/non-pecuniary damages under Article 710.
/// Requires Article 709 liability as a precondition.
///
/// ## Example
///
/// ```rust
/// use legalis_jp::tort::{Article709, Article710, Intent, Damage, CausalLink, ProtectedInterest};
/// use legalis_jp::tort::{NonPecuniaryDamageType, HarmSeverity};
///
/// // First establish Article 709 liability
/// let article_709_claim = Article709::new()
///     .with_act("交通事故で歩行者に衝突")
///     .with_intent(Intent::Negligence)
///     .with_victim_interest(ProtectedInterest::BodyAndHealth)
///     .with_damage(Damage::new(200_000, "治療費"))
///     .with_causal_link(CausalLink::Direct);
///
/// // Then claim Article 710 non-pecuniary damages
/// let article_710_claim = Article710::new()
///     .with_article_709(article_709_claim)
///     .damage_type(NonPecuniaryDamageType::BodyAndHealth)
///     .harm_severity(HarmSeverity::Moderate)
///     .emotional_distress("継続的な痛みと精神的苦痛")
///     .consolation_money(500_000);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Article710<'a> {
    /// Reference to the Article 709 claim (required precondition)
    pub article_709_claim: Option<Article709<'a>>,

    /// Type of non-pecuniary damage (非財産的損害の種類)
    pub damage_type: Option<NonPecuniaryDamageType>,

    /// Consolation money amount (慰謝料額)
    pub consolation_money: Option<u64>,

    /// Severity/degree of harm (被害の程度)
    pub harm_severity: Option<HarmSeverity>,

    /// Victim's comparative fault percentage (0-100) (被害者の過失割合)
    pub victim_comparative_fault: Option<u8>,

    /// Description of emotional distress (精神的苦痛の説明)
    pub emotional_distress_description: Option<String>,
}

impl<'a> Article710<'a> {
    /// Create a new Article 710 claim builder
    pub fn new() -> Self {
        Self {
            article_709_claim: None,
            damage_type: None,
            consolation_money: None,
            harm_severity: None,
            victim_comparative_fault: None,
            emotional_distress_description: None,
        }
    }

    /// Link to establishing Article 709 liability (前提条件)
    pub fn with_article_709(mut self, claim: Article709<'a>) -> Self {
        self.article_709_claim = Some(claim);
        self
    }

    /// Set the type of non-pecuniary damage
    pub fn damage_type(mut self, dtype: NonPecuniaryDamageType) -> Self {
        self.damage_type = Some(dtype);
        self
    }

    /// Set the consolation money amount (慰謝料額)
    pub fn consolation_money(mut self, amount: u64) -> Self {
        self.consolation_money = Some(amount);
        self
    }

    /// Set the severity of harm (被害の程度)
    pub fn harm_severity(mut self, severity: HarmSeverity) -> Self {
        self.harm_severity = Some(severity);
        self
    }

    /// Set description of emotional distress (精神的苦痛の説明)
    pub fn emotional_distress(mut self, description: impl Into<String>) -> Self {
        self.emotional_distress_description = Some(description.into());
        self
    }

    /// Set victim's comparative fault percentage (被害者の過失割合)
    pub fn victim_comparative_fault(mut self, percentage: u8) -> Self {
        if percentage <= 100 {
            self.victim_comparative_fault = Some(percentage);
        }
        self
    }

    /// Build the claim (finalizes the builder)
    pub fn build(self) -> Result<Article710<'a>, TortClaimError> {
        if self.article_709_claim.is_none() {
            return Err(TortClaimError::MissingField(
                "article_709_claim".to_string(),
            ));
        }
        if self.damage_type.is_none() {
            return Err(TortClaimError::MissingField("damage_type".to_string()));
        }
        if self.harm_severity.is_none() {
            return Err(TortClaimError::MissingField("harm_severity".to_string()));
        }
        Ok(self)
    }

    /// Validate the Article 710 claim
    pub fn validate(&self) -> Result<(), ValidationError> {
        crate::tort::validator::validate_article_710(self).map(|_| ())
    }

    /// Get recommended consolation money based on severity
    pub fn recommended_consolation_money(&self) -> u64 {
        if let Some(amount) = self.consolation_money {
            return amount;
        }

        // Default recommendations based on severity (simplified)
        match self.harm_severity {
            Some(HarmSeverity::Minor) => 100_000,
            Some(HarmSeverity::Moderate) => 500_000,
            Some(HarmSeverity::Severe) => 1_500_000,
            Some(HarmSeverity::Catastrophic) => 5_000_000,
            None => 0,
        }
    }
}

impl<'a> Default for Article710<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of Article 710 liability validation (710条検証結果)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article710Liability {
    /// Reference to Article 710
    pub article: ArticleReference,

    /// Whether Article 709 liability is established
    pub based_on_article_709: bool,

    /// Liability status
    pub status: NonPecuniaryLiabilityStatus,

    /// Recommended consolation money (慰謝料額の推奨値)
    pub recommended_consolation_money: u64,

    /// Factors considered in calculation (考慮要素)
    pub calculation_factors: Vec<String>,

    /// Detailed validation results
    pub validation_details: Vec<String>,
}

impl Article710Liability {
    /// Check if non-pecuniary liability is established
    pub fn is_liability_established(&self) -> bool {
        matches!(self.status, NonPecuniaryLiabilityStatus::Established)
    }
}

/// Liability status for Article 710 (710条責任の状態)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NonPecuniaryLiabilityStatus {
    /// Liability established (責任成立)
    Established,

    /// Liability not established (責任不成立)
    NotEstablished,

    /// Requires judicial determination (司法判断を要する)
    RequiresJudicialDetermination { reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tort::types::*;

    #[test]
    fn test_article710_builder() {
        let article_709 = Article709::new()
            .with_act("交通事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::BodyAndHealth)
            .with_damage(Damage::new(200_000, "治療費"))
            .with_causal_link(CausalLink::Direct);

        let claim = Article710::new()
            .with_article_709(article_709)
            .damage_type(NonPecuniaryDamageType::BodyAndHealth)
            .harm_severity(HarmSeverity::Moderate);

        assert!(claim.article_709_claim.is_some());
        assert!(claim.damage_type.is_some());
        assert!(claim.harm_severity.is_some());
    }

    #[test]
    fn test_build_with_missing_field() {
        let result = Article710::new()
            .damage_type(NonPecuniaryDamageType::BodyAndHealth)
            .build();

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err, TortClaimError::MissingField(_)));
        }
    }

    #[test]
    fn test_recommended_consolation_money() {
        let claim = Article710::new().harm_severity(HarmSeverity::Severe);

        assert_eq!(claim.recommended_consolation_money(), 1_500_000);
    }

    #[test]
    fn test_victim_comparative_fault() {
        let claim = Article710::new().victim_comparative_fault(30);

        assert_eq!(claim.victim_comparative_fault, Some(30));
    }
}
