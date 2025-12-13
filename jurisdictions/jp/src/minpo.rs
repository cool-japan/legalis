//! Japanese Civil Code (民法 - Minpo) support.
//!
//! This module provides structured representations of the Japanese Civil Code
//! (Act No. 89 of 1896), including tort law, contract law, and property rights.
//!
//! ## Structure
//!
//! The Civil Code consists of 5 books (編):
//! - Book 1: General Provisions (総則) - Articles 1-174
//! - Book 2: Real Rights (物権) - Articles 175-398
//! - Book 3: Claims (債権) - Articles 399-724
//! - Book 4: Family (親族) - Articles 725-881
//! - Book 5: Succession (相続) - Articles 882-1050
//!
//! ## Key Articles Implemented
//!
//! - Article 709: Tort Liability (不法行為による損害賠償)
//! - Article 710: Compensation for Non-Pecuniary Damage
//! - Article 715: Employer's Liability

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Creates the statute for Article 709: Tort Liability.
///
/// ## Article 709 (不法行為による損害賠償)
///
/// > 故意又は過失によって他人の権利又は法律上保護される利益を侵害した者は、
/// > これによって生じた損害を賠償する責任を負う。
///
/// English translation:
/// > A person who has intentionally or negligently infringed the rights of another,
/// > or legally protected interests of another, is liable to compensate for damages
/// > arising therefrom.
///
/// ## Legal Structure
///
/// **Requirements (要件)**:
/// 1. Intent (故意) OR Negligence (過失) - *Requires judicial discretion*
/// 2. Infringement of rights or legally protected interests (権利侵害) - *Factual determination*
/// 3. Causation (因果関係) - *Requires judicial discretion*
/// 4. Damages (損害) - *Factual/judicial determination*
///
/// **Effect (効果)**:
/// - Obligation to compensate for damages (損害賠償責任)
///
/// ## Example
///
/// ```rust
/// use legalis_jp::minpo::article_709;
/// use legalis_core::LegalEntity;
///
/// let statute = article_709();
/// println!("{}", statute);
/// // => STATUTE minpo-709: "民法第709条 不法行為による損害賠償"
/// ```
#[must_use]
pub fn article_709() -> Statute {
    Statute::new(
        "minpo-709",
        "民法第709条 不法行為による損害賠償 / Article 709: Tort Liability",
        Effect::new(EffectType::Obligation, "損害賠償責任 (Liability for Damages)")
            .with_parameter("subject", "加害者 (tortfeasor)")
            .with_parameter("beneficiary", "被害者 (victim)")
            .with_parameter("content", "生じた損害の賠償 (compensation for damages arising)"),
    )
    .with_jurisdiction("JP")
    .with_version(1)
    // Requirement 1: Intent OR Negligence (both require judicial assessment)
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "intent".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "negligence".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 2: Infringement of rights/interests
    .with_precondition(Condition::AttributeEquals {
        key: "infringement".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Causation between act and damage
    .with_precondition(Condition::AttributeEquals {
        key: "causation".to_string(),
        value: "true".to_string(),
    })
    // Requirement 4: Existence of damages
    .with_precondition(Condition::AttributeEquals {
        key: "damages_exist".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "故意・過失の認定、違法性の判断、因果関係の立証、損害額の算定は司法判断を要する。\
        特に過失の有無は、行為時の注意義務違反の程度、予見可能性、結果回避可能性などを総合的に考慮する。\
        \n\nDetermination of intent/negligence, assessment of unlawfulness, proof of causation, \
        and calculation of damages require judicial discretion. \
        Negligence assessment involves comprehensive consideration of duty of care breach, \
        foreseeability, and avoidability of consequences.",
    )
}

/// Creates the statute for Article 710: Compensation for Non-Pecuniary Damage.
///
/// ## Article 710 (財産以外の損害の賠償)
///
/// > 他人の身体、自由若しくは名誉を侵害した場合又は他人の財産権を侵害した場合のいずれであるかを問わず、
/// > 前条の規定により損害賠償の責任を負う者は、財産以外の損害に対しても、その賠償をしなければならない。
///
/// English translation:
/// > A person who is liable to compensate for damage under the provisions of the preceding Article
/// > shall compensate for non-pecuniary damage as well, regardless of whether the victim's
/// > body, liberty, reputation, or property rights were infringed.
#[must_use]
pub fn article_710() -> Statute {
    Statute::new(
        "minpo-710",
        "民法第710条 財産以外の損害の賠償 / Article 710: Non-Pecuniary Damages",
        Effect::new(
            EffectType::Obligation,
            "財産以外の損害の賠償責任 (Liability for non-pecuniary damages)",
        )
        .with_parameter("damage_type", "non-pecuniary (精神的損害等)")
        .with_parameter("includes", "pain and suffering, emotional distress"),
    )
    .with_jurisdiction("JP")
    .with_version(1)
    // Requires that Article 709 liability is established
    .with_precondition(Condition::AttributeEquals {
        key: "article_709_liable".to_string(),
        value: "true".to_string(),
    })
    // Any of: bodily harm, liberty infringement, reputation damage, property rights violation
    .with_precondition(Condition::Or(
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "bodily_harm".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "liberty_infringement".to_string(),
                value: "true".to_string(),
            }),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "reputation_damage".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "property_rights_violation".to_string(),
                value: "true".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "慰謝料額の算定は、被害の内容・程度、加害者の悪質性、被害者の社会的地位などを総合考慮して判断される。\
        \n\nCalculation of consolation money (慰謝料) requires comprehensive judicial assessment \
        of the nature and degree of harm, maliciousness of the tortfeasor, social status of the victim, etc.",
    )
}

/// Creates the statute for Article 715: Employer's Vicarious Liability.
///
/// ## Article 715(1) (使用者等の責任)
///
/// > ある事業のために他人を使用する者は、被用者がその事業の執行について第三者に加えた損害を賠償する責任を負う。
/// > ただし、使用者が被用者の選任及びその事業の監督について相当の注意をしたとき、
/// > 又は相当の注意をしても損害が生ずべきであったときは、この限りでない。
///
/// English translation:
/// > A person who employs another to engage in an undertaking is liable for damage
/// > inflicted on a third party by the employee in the course of execution of that undertaking;
/// > provided, however, that this does not apply if the employer exercised reasonable care
/// > in appointing the employee and in supervising the undertaking, or if the damage
/// > would have occurred even if the employer had exercised reasonable care.
#[must_use]
pub fn article_715_1() -> Statute {
    Statute::new(
        "minpo-715-1",
        "民法第715条第1項 使用者責任 / Article 715(1): Employer's Vicarious Liability",
        Effect::new(
            EffectType::Obligation,
            "使用者の損害賠償責任 (Employer's liability for employee's torts)",
        )
        .with_parameter("liable_party", "使用者 (employer)")
        .with_parameter("primary_tortfeasor", "被用者 (employee)")
        .with_parameter("liability_type", "vicarious (使用者責任)"),
    )
    .with_jurisdiction("JP")
    .with_version(1)
    // Requirement 1: Employer-employee relationship
    .with_precondition(Condition::EntityRelationship {
        relationship_type: legalis_core::RelationshipType::Employment,
        target_entity_id: None,
    })
    // Requirement 2: Employee committed a tort under Article 709
    .with_precondition(Condition::AttributeEquals {
        key: "employee_article_709_liable".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Tort occurred during execution of business
    .with_precondition(Condition::AttributeEquals {
        key: "during_business_execution".to_string(),
        value: "true".to_string(),
    })
    // Defense 1: Employer exercised reasonable care in appointment/supervision
    .with_precondition(Condition::Not(Box::new(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "reasonable_care_appointment".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "reasonable_care_supervision".to_string(),
            value: "true".to_string(),
        }),
    ))))
    .with_discretion(
        "「事業の執行について」の判断では、外形理論（被害者から見て事業執行中と見える行為）が採用される。\
        使用者の免責事由（相当な注意）の立証責任は使用者側が負う。\
        \n\nThe 'during execution of business' requirement applies the external appearance doctrine \
        (acts that appear to be business execution from the victim's perspective). \
        The employer bears the burden of proving the defense of reasonable care.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_709_creation() {
        let statute = article_709();
        assert_eq!(statute.id, "minpo-709");
        assert_eq!(statute.jurisdiction, Some("JP".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::Obligation);

        // Should have 4 preconditions: (intent OR negligence), infringement, causation, damages
        assert_eq!(statute.preconditions.len(), 4);

        // First precondition should be an OR
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));

        // Should have discretion logic
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_article_709_display() {
        let statute = article_709();
        let display = format!("{}", statute);

        assert!(display.contains("minpo-709"));
        assert!(display.contains("民法第709条"));
        assert!(display.contains("JP"));
        assert!(display.contains("OBLIGATION"));
    }

    #[test]
    fn test_article_710_creation() {
        let statute = article_710();
        assert_eq!(statute.id, "minpo-710");
        assert_eq!(statute.jurisdiction, Some("JP".to_string()));

        // Should require Article 709 liability + one of four harm types
        assert_eq!(statute.preconditions.len(), 2);
    }

    #[test]
    fn test_article_715_creation() {
        let statute = article_715_1();
        assert_eq!(statute.id, "minpo-715-1");

        // Should have employment relationship check
        let has_employment_check = statute.preconditions.iter().any(|c| {
            matches!(
                c,
                Condition::EntityRelationship {
                    relationship_type: legalis_core::RelationshipType::Employment,
                    ..
                }
            )
        });
        assert!(has_employment_check);
    }

    #[test]
    fn test_article_709_validation() {
        let statute = article_709();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article_710_validation() {
        let statute = article_710();
        assert!(statute.is_valid());
    }

    #[test]
    fn test_article_715_validation() {
        let statute = article_715_1();
        assert!(statute.is_valid());
    }
}
