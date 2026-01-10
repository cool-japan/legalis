//! Japanese Civil Code Article 715 implementation
//!
//! Provides builder pattern API for employer/supervisor vicarious liability.
//!
//! ## Article 715(1) (民法第715条第1項 - 使用者等の責任)
//!
//! > ある事業のために他人を使用する者は、被用者がその事業の執行について第三者に加えた損害を賠償する責任を負う。
//! > ただし、使用者が被用者の選任及びその事業の監督について相当の注意をしたとき、
//! > 又は相当の注意をしても損害が生ずべきであったときは、この限りでない。
//!
//! English: A person who employs another to engage in an undertaking is liable for damage
//! inflicted on a third party by the employee in the course of execution of that undertaking;
//! provided, however, that this does not apply if the employer exercised reasonable care
//! in appointing the employee and in supervising the undertaking, or if the damage
//! would have occurred even if the employer had exercised reasonable care.

use crate::tort::article709::{Article709, ArticleReference};
use crate::tort::error::{TortClaimError, ValidationError};
use crate::tort::types::EmploymentRelationship;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Article 715 employer liability claim builder
///
/// Represents vicarious liability of employers/supervisors under Article 715.
///
/// ## Example
///
/// ```rust
/// use legalis_jp::tort::{Article709, Article715, Intent, Damage, CausalLink, ProtectedInterest};
/// use legalis_jp::tort::{EmploymentRelationship, EmploymentType};
///
/// // Employee's tort (Article 709)
/// let employee_tort = Article709::new()
///     .with_act("配達中に歩行者と衝突")
///     .with_intent(Intent::Negligence)
///     .with_victim_interest(ProtectedInterest::BodyAndHealth)
///     .with_damage(Damage::new(500_000, "治療費"))
///     .with_causal_link(CausalLink::Direct);
///
/// // Employer's vicarious liability (Article 715)
/// let employer_liability = Article715::new()
///     .employee_tort(employee_tort)
///     .employer("株式会社ABC配送")
///     .employee("配達員X")
///     .employment_type(EmploymentType::FullTime)
///     .during_business_execution(true)
///     .business_context("配達業務中の交通事故");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Article715<'a> {
    /// Employment relationship (使用関係)
    pub employment_relationship: Option<EmploymentRelationship<'a>>,

    /// Employee's Article 709 tort (被用者の709条不法行為)
    pub employee_tort: Option<Article709<'a>>,

    /// Whether tort occurred during business execution (事業執行について)
    pub during_business_execution: Option<bool>,

    /// Business execution context explanation (事業執行との関連性)
    pub business_context: Option<String>,

    /// Defense: Reasonable care in appointment (選任の際の相当注意)
    pub reasonable_care_appointment: Option<bool>,

    /// Defense: Reasonable care in supervision (監督の際の相当注意)
    pub reasonable_care_supervision: Option<bool>,

    /// Evidence of reasonable care measures (相当注意の証拠)
    pub care_evidence: Option<String>,

    /// Whether damage would occur even with reasonable care (相当注意をしても損害発生)
    pub unavoidable_damage: Option<bool>,

    /// Employer name (使用者名) - convenience field
    employer_name: Option<String>,

    /// Employee name (被用者名) - convenience field
    employee_name: Option<String>,

    /// Employment type - convenience field
    employment_type: Option<crate::tort::types::EmploymentType>,
}

impl<'a> Article715<'a> {
    /// Create a new Article 715 claim builder
    pub fn new() -> Self {
        Self {
            employment_relationship: None,
            employee_tort: None,
            during_business_execution: None,
            business_context: None,
            reasonable_care_appointment: None,
            reasonable_care_supervision: None,
            care_evidence: None,
            unavoidable_damage: None,
            employer_name: None,
            employee_name: None,
            employment_type: None,
        }
    }

    /// Set employment relationship (使用関係)
    pub fn employment(mut self, relationship: EmploymentRelationship<'a>) -> Self {
        self.employment_relationship = Some(relationship);
        self
    }

    /// Set employer name (convenience method)
    pub fn employer(mut self, name: impl Into<String>) -> Self {
        self.employer_name = Some(name.into());
        self
    }

    /// Set employee name (convenience method)
    pub fn employee(mut self, name: impl Into<String>) -> Self {
        self.employee_name = Some(name.into());
        self
    }

    /// Set employment type (convenience method)
    pub fn employment_type(mut self, etype: crate::tort::types::EmploymentType) -> Self {
        self.employment_type = Some(etype);
        self
    }

    /// Link to employee's Article 709 tort
    pub fn employee_tort(mut self, tort: Article709<'a>) -> Self {
        self.employee_tort = Some(tort);
        self
    }

    /// Set whether tort occurred during business execution (事業執行について)
    pub fn during_business_execution(mut self, during: bool) -> Self {
        self.during_business_execution = Some(during);
        self
    }

    /// Set business context explanation
    pub fn business_context(mut self, context: impl Into<String>) -> Self {
        self.business_context = Some(context.into());
        self
    }

    /// Set defense: reasonable care in appointment
    pub fn reasonable_care_appointment(mut self, exercised: bool) -> Self {
        self.reasonable_care_appointment = Some(exercised);
        self
    }

    /// Set defense: reasonable care in supervision
    pub fn reasonable_care_supervision(mut self, exercised: bool) -> Self {
        self.reasonable_care_supervision = Some(exercised);
        self
    }

    /// Set evidence of reasonable care
    pub fn care_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.care_evidence = Some(evidence.into());
        self
    }

    /// Set whether damage was unavoidable despite reasonable care
    pub fn unavoidable_damage(mut self, unavoidable: bool) -> Self {
        self.unavoidable_damage = Some(unavoidable);
        self
    }

    /// Build the claim (finalizes the builder)
    pub fn build(self) -> Result<Article715<'a>, TortClaimError> {
        if self.employee_tort.is_none() {
            return Err(TortClaimError::MissingField("employee_tort".to_string()));
        }
        if self.during_business_execution.is_none() {
            return Err(TortClaimError::MissingField(
                "during_business_execution".to_string(),
            ));
        }

        // Build employment relationship from convenience fields if not directly set
        let mut claim = self;
        if claim.employment_relationship.is_none() {
            if let (Some(employer), Some(employee), Some(etype)) = (
                &claim.employer_name,
                &claim.employee_name,
                &claim.employment_type,
            ) {
                claim.employment_relationship = Some(EmploymentRelationship {
                    employer_name: Box::leak(employer.clone().into_boxed_str()),
                    employee_name: Box::leak(employee.clone().into_boxed_str()),
                    employment_type: etype.clone(),
                    relationship_duration: None,
                });
            } else {
                return Err(TortClaimError::MissingField(
                    "employment_relationship".to_string(),
                ));
            }
        }

        Ok(claim)
    }

    /// Validate the Article 715 claim
    pub fn validate(&self) -> Result<(), ValidationError> {
        crate::tort::validator::validate_article_715(self).map(|_| ())
    }

    /// Check if employer liability is established (simplified check)
    pub fn is_liability_established(&self) -> bool {
        self.employment_relationship.is_some()
            && self.employee_tort.is_some()
            && self.during_business_execution == Some(true)
            && (self.reasonable_care_appointment != Some(true)
                || self.reasonable_care_supervision != Some(true))
    }
}

impl<'a> Default for Article715<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of Article 715 liability validation (715条検証結果)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article715Liability {
    /// Reference to Article 715
    pub article: ArticleReference,

    /// Whether employer is liable
    pub employer_liable: bool,

    /// Liability status
    pub status: VicariousLiabilityStatus,

    /// Reasoning for determination (判断理由)
    pub reasoning: Vec<String>,

    /// Defenses that were evaluated (検討された抗弁)
    pub applicable_defenses: Vec<String>,

    /// Compensation basis (賠償の基礎)
    pub compensation_basis: Option<String>,

    /// Detailed validation results
    pub validation_details: Vec<String>,
}

impl Article715Liability {
    /// Check if employer liability is established
    pub fn is_employer_liable(&self) -> bool {
        matches!(self.status, VicariousLiabilityStatus::Liable)
    }
}

/// Vicarious liability status (使用者責任の状態)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VicariousLiabilityStatus {
    /// Employer is vicariously liable (使用者責任あり)
    Liable,

    /// Employer not liable due to defense (免責)
    NotLiable { defense_reason: String },

    /// Requires judicial determination (司法判断を要する)
    RequiresJudicialDetermination { factors: Vec<String> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tort::types::*;

    #[test]
    fn test_article715_builder() {
        let employee_tort = Article709::new()
            .with_act("配達中の事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::BodyAndHealth)
            .with_damage(Damage::new(500_000, "治療費"))
            .with_causal_link(CausalLink::Direct);

        let claim = Article715::new()
            .employee_tort(employee_tort)
            .employer("ABC配送")
            .employee("従業員X")
            .employment_type(EmploymentType::FullTime)
            .during_business_execution(true);

        assert!(claim.employee_tort.is_some());
        assert!(claim.during_business_execution.is_some());
    }

    #[test]
    fn test_build_with_missing_employee_tort() {
        let result = Article715::new()
            .employer("ABC配送")
            .during_business_execution(true)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_liability_check() {
        let employee_tort = Article709::new()
            .with_act("業務中の事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(300_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let claim = Article715::new()
            .employee_tort(employee_tort)
            .employer("運送会社")
            .employee("ドライバー")
            .employment_type(EmploymentType::FullTime)
            .during_business_execution(true)
            .build()
            .unwrap();

        assert!(claim.is_liability_established());
    }

    #[test]
    fn test_defense_blocks_liability() {
        let employee_tort = Article709::new()
            .with_act("業務中の事故")
            .with_intent(Intent::Negligence)
            .with_victim_interest(ProtectedInterest::Property("車両"))
            .with_damage(Damage::new(300_000, "修理費"))
            .with_causal_link(CausalLink::Direct);

        let claim = Article715::new()
            .employee_tort(employee_tort)
            .employer("運送会社")
            .employee("ドライバー")
            .employment_type(EmploymentType::FullTime)
            .during_business_execution(true)
            .reasonable_care_appointment(true)
            .reasonable_care_supervision(true);

        // Both defenses present -> liability not automatically established
        assert!(!claim.is_liability_established());
    }
}
