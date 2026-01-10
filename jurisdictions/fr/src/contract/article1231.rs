//! Article 1231 - Damages (Dommages-intérêts)
//!
//! Implementation of Code civil Article 1231 (2016 reform).

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1231 - Damages for Breach (Dommages-intérêts)
///
/// ## French Text (2016 version)
///
/// > Le débiteur est condamné, s'il y a lieu, au paiement de dommages et intérêts
/// > soit à raison de l'inexécution de l'obligation, soit à raison du retard dans
/// > l'exécution, s'il ne justifie pas que l'exécution a été empêchée par la force majeure.
///
/// ## English Translation
///
/// > The debtor is condemned, if there is cause, to the payment of damages
/// > either by reason of non-performance of the obligation, or by reason of delay
/// > in performance, unless he proves that performance was prevented by force majeure.
///
/// ## Historical Context
///
/// This article consolidates former Articles 1146-1147 (1804-2016) on contractual damages.
/// The 2016 reform modernized the language and clarified the force majeure exception.
///
/// ## Legal Principles
///
/// ### 1. Contractual vs. Tort Damages
///
/// **Key Difference**:
/// - **Tort damages** (Article 1240): Require proof of **fault** (faute)
/// - **Contract damages** (Article 1231): No fault required, only **breach** (inexécution)
///
/// Contractual liability is more favorable to creditors: breach alone suffices.
///
/// ### 2. Types of Damages
///
/// French law recognizes:
/// - **Compensatory damages** (dommages-intérêts compensatoires): Repair the harm
/// - **Consequential damages** (dommages-intérêts moratoires): For delay
///
/// Calculation includes:
/// - **Damnum emergens**: Actual loss suffered (perte éprouvée)
/// - **Lucrum cessans**: Lost profits (gain manqué) - Article 1231-2
///
/// ### 3. Force Majeure Exception (Article 1218)
///
/// The debtor is exempt from liability if performance was prevented by force majeure:
/// - Event beyond debtor's control (échappant au contrôle du débiteur)
/// - Could not reasonably be foreseen (imprévisible)
/// - Effects could not be avoided (irrésistible)
///
/// Examples: war, natural disaster, government prohibition
///
/// ### 4. Foreseeability Limit (Article 1231-3)
///
/// Damages are limited to foreseeable harm at contract formation, **except**:
/// - Intentional or gross fault (dol or faute lourde)
/// - Debtor committed not to assign (clause de non-concurrence)
///
/// ### 5. Penalty Clauses (Article 1231-5)
///
/// Parties may agree on liquidated damages (clause pénale):
/// - Pre-estimates damages
/// - Binding on parties
/// - **BUT**: Courts may reduce excessive penalties (modération judiciaire)
///
/// ## Comparison with Other Systems
///
/// | Aspect | France | Japan | Common Law |
/// |--------|--------|-------|------------|
/// | Fault required? | No (breach sufficient) | No (415条) | No |
/// | Foreseeability limit | Yes (Art. 1231-3) | Yes (416条) | Yes (Hadley rule) |
/// | Penalty clauses | Valid (Art. 1231-5) | Valid (420条) | Liquidated damages |
/// | Force majeure | Exempts (Art. 1218) | Exempts (419条3号) | Impossibility |
///
/// ## Example
///
/// ```rust
/// use legalis_fr::contract::article1231;
///
/// let statute = article1231();
/// assert_eq!(statute.id, "code-civil-1231");
/// ```
#[must_use]
pub fn article1231() -> Statute {
    Statute::new(
        "code-civil-1231",
        "Code civil Article 1231 - Dommages-intérêts / Damages for Breach",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Paiement de dommages-intérêts / Payment of damages",
        )
        .with_parameter("payor", "débiteur (debtor)")
        .with_parameter("payee", "créancier (creditor)")
        .with_parameter("basis", "inexécution ou retard (non-performance or delay)")
        .with_parameter("calculation", "perte éprouvée + gain manqué (actual loss + lost profits)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Precondition 1: Non-performance OR delayed performance
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "non_performance".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "delayed_performance".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Precondition 2: No force majeure (Article 1218)
    // Force majeure exempts from liability
    .with_precondition(Condition::AttributeEquals {
        key: "force_majeure".to_string(),
        value: "false".to_string(),
    })
    // Precondition 3: Harm suffered (dommage)
    .with_precondition(Condition::AttributeEquals {
        key: "harm_suffered".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article 1231 consacre le principe de la responsabilité contractuelle pour inexécution. \
        À la différence de la responsabilité délictuelle (Article 1240), la responsabilité \
        contractuelle ne requiert pas la preuve d'une faute : la simple inexécution suffit. \
        \n\nLe débiteur ne peut s'exonérer qu'en prouvant la force majeure (Article 1218) : \
        un événement imprévisible, irrésistible, et extérieur. \
        \n\nLes dommages-intérêts comprennent : \
        • La perte éprouvée (damnum emergens) : perte effective \
        • Le gain manqué (lucrum cessans) : profit dont le créancier a été privé (Article 1231-2) \
        \n\nLimites : \
        • Prévisibilité : seuls les dommages prévisibles lors de la conclusion du contrat \
          sont réparables (Article 1231-3), sauf dol ou faute lourde \
        • Clause pénale : les parties peuvent fixer à l'avance le montant des dommages-intérêts, \
          mais le juge peut modérer les clauses pénales manifestement excessives (Article 1231-5) \
        \n\nArticle 1231 establishes contractual liability for breach. Unlike tort liability \
        (Article 1240), contractual liability does not require proof of fault: mere breach suffices. \
        \n\nThe debtor can only be exempted by proving force majeure (Article 1218): \
        an unforeseeable, irresistible, and external event. \
        \n\nDamages include: \
        • Actual loss (damnum emergens): effective loss \
        • Lost profits (lucrum cessans): profit of which creditor was deprived (Article 1231-2) \
        \n\nLimits: \
        • Foreseeability: only foreseeable damages are recoverable (Article 1231-3), except for fraud or gross fault \
        • Penalty clauses: parties may pre-fix damages, but courts may reduce manifestly excessive penalties (Article 1231-5) \
        \n\n【比較法的考察】\n\
        契約上の損害賠償については、仏日両国ともに「債務不履行」のみで足り、「過失」の証明は不要である。\
        これは不法行為（仏1240条、日709条）との重要な相違点である。\
        予見可能性の制限（仏1231-3条、日416条）も共通する。\
        ただし、フランス法の特徴は、違約金条項(clause pénale)の裁判所による減額権限を明文で認めている点にある。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1231_creation() {
        let statute = article1231();
        assert_eq!(statute.id, "code-civil-1231");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::MonetaryTransfer);
    }

    #[test]
    fn test_article1231_preconditions() {
        let statute = article1231();
        // Should have 3 preconditions
        assert_eq!(statute.preconditions.len(), 3);

        // 1. Non-performance OR delayed performance
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));

        // 2. No force majeure
        if let Condition::AttributeEquals { key, value } = &statute.preconditions[1] {
            assert_eq!(key, "force_majeure");
            assert_eq!(value, "false");
        } else {
            panic!("Expected AttributeEquals for force_majeure");
        }

        // 3. Harm suffered
        assert!(matches!(
            statute.preconditions[2],
            Condition::AttributeEquals { .. }
        ));
    }

    #[test]
    fn test_article1231_effect_parameters() {
        let statute = article1231();

        let params = &statute.effect.parameters;
        assert!(params.contains_key("payor"));
        assert!(params.contains_key("payee"));
        assert!(params.contains_key("basis"));
        assert!(params.contains_key("calculation"));

        assert_eq!(params.get("payor").unwrap(), "débiteur (debtor)");
    }

    #[test]
    fn test_article1231_validation() {
        let statute = article1231();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article1231_has_discretion() {
        let statute = article1231();
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("dommages-intérêts"));
        assert!(discretion.contains("damages"));
        assert!(discretion.contains("force majeure"));
    }

    #[test]
    fn test_article1231_display() {
        let statute = article1231();
        let display = format!("{}", statute);

        assert!(display.contains("code-civil-1231"));
        assert!(display.contains("Article 1231"));
        assert!(display.contains("FR"));
        assert!(display.contains("MONETARY_TRANSFER"));
    }
}
