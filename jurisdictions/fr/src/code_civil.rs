//! Code civil français (French Civil Code)
//!
//! The French Civil Code, also known as the Napoleonic Code (Code Napoléon),
//! was enacted in 1804 and remains the foundation of French private law.
//!
//! ## Historical Significance
//!
//! The Code civil was revolutionary in its:
//! - Clear, accessible language (vs. complex Roman law)
//! - Systematic organization
//! - Emphasis on individual liberty and property rights
//! - Influence on civil codes worldwide (Belgium, Italy, Spain, Latin America, Japan pre-Meiji)
//!
//! ## Tort Law (Responsabilité civile)
//!
//! French tort law is characterized by:
//! - **Extreme abstraction**: Very short, general principles
//! - **Judicial development**: Courts fill in the details
//! - **Unity**: Single regime for all torts (no distinction like German law)
//!
//! ### Key Articles (2016 Reform)
//!
//! The 2016 reform renumbered tort law articles:
//! - Old Article 1382 → **New Article 1240**: General tort liability
//! - Old Article 1383 → **New Article 1241**: Negligence liability
//! - Old Article 1384 → **New Article 1242**: Liability for others/things

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1240 - General Tort Liability (Responsabilité civile générale)
///
/// ## French Text (2016 version)
///
/// > Tout fait quelconque de l'homme, qui cause à autrui un dommage,
/// > oblige celui par la faute duquel il est arrivé à le réparer.
///
/// ## English Translation
///
/// > Any act whatever of man, which causes damage to another,
/// > obliges the one by whose fault it occurred to repair it.
///
/// ## Historical Version (1804-2016): Article 1382
///
/// The text remained virtually unchanged from 1804 to 2016, a testament to its
/// elegant generality. Only the article number changed.
///
/// ## Legal Philosophy
///
/// This is arguably the **most abstract tort provision in any major civil code**:
/// - **"Tout fait quelconque"** (any act whatever): Unlimited scope
/// - **"de l'homme"** (of man): Only human conduct (but extended to legal persons)
/// - **"dommage"** (damage): Any harm, material or immaterial
/// - **"faute"** (fault): Intent OR negligence (not specified, developed by courts)
///
/// ## Comparison with Other Systems
///
/// | System | Approach | Scope |
/// |--------|----------|-------|
/// | 🇫🇷 France | Ultra-abstract general clause | Unlimited |
/// | 🇯🇵 Japan | Medium abstraction ("rights/interests") | Very broad |
/// | 🇩🇪 Germany | Enumerated protected interests | Limited by list |
///
/// The French approach gives maximum flexibility to courts but minimum legal certainty.
///
/// ## Elements (Conditions)
///
/// French doctrine identifies 3 requirements:
/// 1. **Faute** (fault): Wrongful conduct, intentional or negligent
/// 2. **Dommage** (damage): Actual harm suffered
/// 3. **Lien de causalité** (causal link): Between fault and damage
///
/// ## Example
///
/// ```rust
/// use legalis_fr::article_1240;
///
/// let statute = article_1240();
/// println!("{}", statute);
/// // => STATUTE code-civil-1240: "Code civil Article 1240"
/// ```
#[must_use]
pub fn article_1240() -> Statute {
    Statute::new(
        "code-civil-1240",
        "Code civil Article 1240 - Responsabilité pour faute / General Tort Liability",
        Effect::new(
            EffectType::Obligation,
            "Obligation de réparer le dommage (Obligation to repair the damage)",
        )
        .with_parameter("liable_party", "celui par la faute duquel il est arrivé")
        .with_parameter("beneficiary", "autrui (the other person)")
        .with_parameter("content", "réparation du dommage (reparation of damage)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Requirement 1: Faute (fault) - intentional OR negligent conduct
    // "Faute" is a unitary concept in French law (no distinction needed)
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "faute_intentionnelle".to_string(), // Intentional fault
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "faute_negligence".to_string(), // Negligent fault
            value: "true".to_string(),
        }),
    ))
    // Requirement 2: Dommage (damage) - any harm
    .with_precondition(Condition::AttributeEquals {
        key: "dommage".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Lien de causalité (causal link)
    .with_precondition(Condition::AttributeEquals {
        key: "lien_causalite".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "La notion de 'faute' est d'une extrême souplesse. Elle comprend non seulement la faute intentionnelle \
        mais aussi la simple négligence ou imprudence. Le juge apprécie souverainement l'existence de la faute. \
        Le dommage doit être certain, direct, et personnel. \
        \n\nThe concept of 'faute' (fault) is extremely flexible. It includes not only intentional fault \
        but also simple negligence or imprudence. The judge has sovereign discretion in assessing fault. \
        The damage must be certain, direct, and personal. \
        \n\n【比較法的考察】\n\
        フランス民法1240条は、あらゆる法体系の中で最も抽象的な不法行為規定である。\
        「人のいかなる行為も」という無限定の包括条項により、裁判所に最大限の裁量を与える。\
        これは法的安定性を犠牲にして柔軟性を最大化する選択である。",
    )
}

/// Article 1241 - Negligence Liability (Responsabilité pour négligence)
///
/// ## French Text (2016 version)
///
/// > Chacun est responsable du dommage qu'il a causé non seulement par son fait,
/// > mais encore par sa négligence ou par son imprudence.
///
/// ## English Translation
///
/// > Everyone is responsible for the damage he has caused not only by his act,
/// > but also by his negligence or by his imprudence.
///
/// ## Historical Context
///
/// This was Article 1383 from 1804-2016. It was seen as redundant with Article 1382/1240
/// (since "faute" already includes negligence), but served to emphasize that mere
/// carelessness suffices for liability.
///
/// ## Legal Significance
///
/// - Confirms that **negligence alone** (without intent) creates liability
/// - **"Négligence ou imprudence"**: Clarifies forms of fault
/// - In practice, largely absorbed into Article 1240's general principle
///
/// ## Difference from Article 1240
///
/// Strictly speaking, Article 1241 is redundant since Article 1240's "faute"
/// encompasses both intent and negligence. However, Article 1241:
/// 1. Provides pedagogical clarity
/// 2. Emphasizes the low threshold for liability (mere carelessness)
/// 3. Historically important in judicial development of negligence standards
#[must_use]
pub fn article_1241() -> Statute {
    Statute::new(
        "code-civil-1241",
        "Code civil Article 1241 - Responsabilité pour négligence / Negligence Liability",
        Effect::new(
            EffectType::Obligation,
            "Obligation de réparer le dommage causé par négligence",
        )
        .with_parameter("liable_party", "auteur de la négligence")
        .with_parameter("fault_type", "négligence ou imprudence"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Requirement 1: Negligence or imprudence (no intent required)
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "negligence".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "imprudence".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 2: Damage
    .with_precondition(Condition::AttributeEquals {
        key: "dommage".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Causation
    .with_precondition(Condition::AttributeEquals {
        key: "lien_causalite".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article 1241 précise que la simple négligence ou imprudence suffit pour engager \
        la responsabilité, sans qu'il soit nécessaire de prouver une intention de nuire. \
        La jurisprudence a développé des standards de comportement raisonnable (bonus pater familias). \
        \n\nArticle 1241 specifies that mere negligence or imprudence suffices for liability, \
        without needing to prove intent to harm. Case law has developed standards of reasonable \
        behavior (bonus pater familias - the good family father).",
    )
}

/// Article 1242 (paragraph 1) - Liability for Others (Responsabilité du fait d'autrui)
///
/// ## French Text (2016 version)
///
/// > On est responsable non seulement du dommage que l'on cause par son propre fait,
/// > mais encore de celui qui est causé par le fait des personnes dont on doit répondre,
/// > ou des choses que l'on a sous sa garde.
///
/// ## English Translation
///
/// > One is responsible not only for the damage caused by one's own act,
/// > but also for that which is caused by the acts of persons for whom one is responsible,
/// > or by things under one's custody.
///
/// ## Historical Context
///
/// This was Article 1384 from 1804-2016. It establishes two crucial principles:
///
/// 1. **Responsabilité du fait d'autrui** (liability for others' acts)
///    - Parents for children
///    - Employers for employees
///    - Teachers for students
///
/// 2. **Responsabilité du fait des choses** (liability for things)
///    - Strict liability for objects under one's control
///    - Revolutionary development by courts (not explicit in 1804 text)
///
/// ## Legal Significance
///
/// This article enabled French courts to develop:
/// - **Vicarious liability** (similar to German § 831, Japanese Article 715)
/// - **Strict liability** for dangerous objects (cars, machinery, animals)
/// - **Product liability** before specific legislation
///
/// ## Comparison with Other Systems
///
/// - **Germany**: Separate provisions (§ 831 employer liability, § 833 animal liability)
/// - **Japan**: Article 715 (employer), Article 717 (defective structures)
/// - **France**: Single provision, broadly interpreted by courts
#[must_use]
pub fn article_1242() -> Statute {
    Statute::new(
        "code-civil-1242",
        "Code civil Article 1242 - Responsabilité du fait d'autrui / Liability for Others",
        Effect::new(
            EffectType::Obligation,
            "Responsabilité pour le dommage causé par autrui ou par les choses",
        )
        .with_parameter("liable_party", "gardien ou commettant"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Requirement 1: Relationship requiring responsibility
    .with_precondition(Condition::Or(
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "parent_child_relationship".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "employer_employee_relationship".to_string(),
                value: "true".to_string(),
            }),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "teacher_student_relationship".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "custody_of_thing".to_string(), // Garde de la chose
                value: "true".to_string(),
            }),
        )),
    ))
    // Requirement 2: Damage caused by the other person or thing
    .with_precondition(Condition::AttributeEquals {
        key: "dommage_cause_par_autrui_ou_chose".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Causal link
    .with_precondition(Condition::AttributeEquals {
        key: "lien_causalite".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article 1242 établit une responsabilité de plein droit (sans faute à prouver) \
        pour le commettant du fait de son préposé, et pour le gardien du fait de la chose. \
        La notion de 'garde' (custody) est centrale : elle implique usage, direction, et contrôle. \
        \n\nArticle 1242 establishes strict liability (no fault to prove) for the principal \
        for acts of their agent, and for the custodian for damage by things. \
        The concept of 'garde' (custody) is central: it implies use, direction, and control. \
        \n\n【日独仏比較】\n\
        使用者責任について：\n\
        • フランス: 1242条（推定的無過失責任、免責困難）\n\
        • 日本: 715条（免責可能）\n\
        • ドイツ: § 831（免責可能、選任監督上の注意で免責）\n\
        フランス法が最も被害者保護的である。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_1240_creation() {
        let statute = article_1240();
        assert_eq!(statute.id, "code-civil-1240");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::Obligation);

        // Should have 3 preconditions: faute, dommage, causalité
        assert_eq!(statute.preconditions.len(), 3);

        // First precondition should be OR (intentional OR negligent fault)
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));
    }

    #[test]
    fn test_article_1240_validation() {
        let statute = article_1240();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article_1240_display() {
        let statute = article_1240();
        let display = format!("{}", statute);

        assert!(display.contains("code-civil-1240"));
        assert!(display.contains("Article 1240"));
        assert!(display.contains("FR"));
        assert!(display.contains("OBLIGATION"));
    }

    #[test]
    fn test_article_1241_creation() {
        let statute = article_1241();
        assert_eq!(statute.id, "code-civil-1241");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));

        // Should have 3 preconditions
        assert_eq!(statute.preconditions.len(), 3);
    }

    #[test]
    fn test_article_1241_validation() {
        let statute = article_1241();
        assert!(statute.is_valid());
    }

    #[test]
    fn test_article_1242_creation() {
        let statute = article_1242();
        assert_eq!(statute.id, "code-civil-1242");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));

        // Should have 3 preconditions
        assert_eq!(statute.preconditions.len(), 3);
    }

    #[test]
    fn test_article_1242_validation() {
        let statute = article_1242();
        assert!(statute.is_valid());
    }

    #[test]
    fn test_all_french_statutes_have_discretion() {
        // French law's extreme abstraction means ALL statutes require discretion
        let statutes = vec![article_1240(), article_1241(), article_1242()];

        for statute in statutes {
            assert!(
                statute.discretion_logic.is_some(),
                "{} should have discretion logic",
                statute.id
            );
        }
    }
}
