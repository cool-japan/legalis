//! Code Civil Article 1104 — Good Faith (Bonne foi)
//!
//! This module implements the French civil law principle of good faith
//! as codified in the 2016 reform of the Code civil.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Returns the Statute encoding Code Civil Art. 1104 (obligation of good faith).
///
/// ## French Text
///
/// > Les contrats doivent être négociés, formés et exécutés de bonne foi.
/// > Cette disposition est d'ordre public.
///
/// ## English Translation
///
/// > Contracts must be negotiated, formed, and performed in good faith.
/// > This provision is of public order (mandatory).
///
/// ## Legal Significance
///
/// Art. 1104 codified the obligation of good faith across all phases of the
/// contractual lifecycle: pre-contractual negotiations (pourparlers), formation,
/// and performance. The 2016 reform elevated this from a general principle to
/// an explicit, mandatory statutory rule of public order.
///
/// ### Three Phases of Bonne Foi
///
/// 1. **Negotiation (Négociation)**: Parties must negotiate honestly, disclose
///    material information, and not break off negotiations abruptly without cause
///    (rupture abusive des pourparlers — civil liability under Art. 1240).
///
/// 2. **Formation (Formation)**: Consent must be free and informed; no fraudulent
///    concealment of material facts (réticence dolosive — Art. 1137).
///
/// 3. **Performance (Exécution)**: Obligations must be performed cooperatively;
///    debtors must not deliberately undermine the creditor's legitimate expectations.
///
/// ### Ordre Public Status
///
/// The second sentence — "cette disposition est d'ordre public" — means parties
/// cannot contractually exclude the good faith obligation. Any clause purporting
/// to permit bad faith performance is void.
///
/// ## Comparative Analysis
///
/// | Jurisdiction | Good Faith Codification | Scope |
/// |---|---|---|
/// | **France** | Art. 1104 (2016) | All contracts; mandatory |
/// | **Germany** | §242 BGB (1900) | All obligations; foundational principle |
/// | **Japan** | Art. 1(2) Civil Code (1896/2017) | All private law; general clause |
/// | **USA** | UCC §1-304 + Restatement 2d | Commercial contracts; implied covenant |
/// | **UK** | No general duty (common law) | Case-by-case; limited |
///
/// France's codification aligns with continental civil law tradition (BGB §242,
/// Swiss OR Art. 2), moving away from the more limited pre-2016 position that
/// good faith applied only to performance (old Art. 1134 al. 3).
#[must_use]
pub fn article1104() -> Statute {
    Statute::new(
        "code-civil-1104",
        "Code civil Article 1104 — Obligation de bonne foi (Principle of Good Faith)",
        Effect::new(
            EffectType::Obligation,
            "La bonne foi est requise dans la négociation, la formation et l'exécution du contrat.\n\nGood faith is required in contract negotiation, formation, and performance.",
        )
        .with_parameter("scope", "Negotiation, formation, and performance (négociation, formation, exécution)")
        .with_parameter("mandatory", "Ordre public — cannot be contractually excluded"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "is_contract".to_string(),
    })
    .with_discretion(
        "L'article 1104, issu de la réforme de 2016, consacre l'obligation de bonne foi \
        à tous les stades du contrat: négociation, formation et exécution. Son caractère \
        d'ordre public interdit toute clause d'exclusion. La bonne foi est une norme \
        objective appréciée in abstracto (comportement d'un contractant raisonnable). \
        Elle couvre notamment la culpa in contrahendo (rupture abusive des pourparlers), \
        la réticence dolosive, et les obligations de coopération en cours d'exécution.\
        \n\n\
        Article 1104, introduced by the 2016 reform, establishes the good faith obligation \
        at all stages of the contract: negotiation, formation, and performance. Its mandatory \
        public order nature prohibits any exclusion clause. Good faith is an objective standard \
        assessed in abstracto (conduct of a reasonable contracting party). It covers pre-contractual \
        liability (culpa in contrahendo), fraudulent concealment, and cooperation duties during performance.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article1104_has_correct_id() {
        let s = article1104();
        assert_eq!(s.id, "code-civil-1104");
    }

    #[test]
    fn article1104_has_fr_jurisdiction() {
        let s = article1104();
        assert_eq!(s.jurisdiction, Some("FR".to_string()));
    }

    #[test]
    fn article1104_has_preconditions() {
        let s = article1104();
        assert!(!s.preconditions.is_empty());
    }

    #[test]
    fn article1104_has_discretion() {
        let s = article1104();
        assert!(s.discretion_logic.is_some());
        assert!(!s.discretion_logic.as_ref().unwrap().is_empty());
    }

    #[test]
    fn article1104_effect_is_obligation() {
        let s = article1104();
        assert!(matches!(s.effect.effect_type, EffectType::Obligation));
    }

    #[test]
    fn article1104_is_valid() {
        let s = article1104();
        assert!(s.is_valid());
        assert_eq!(s.validate().len(), 0);
    }
}
