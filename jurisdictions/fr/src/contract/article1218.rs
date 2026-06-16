//! Code Civil Article 1218 — Force Majeure
//!
//! This module implements the French civil law force majeure doctrine
//! as codified in the 2016 reform of the Code civil.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Returns the Statute encoding Code Civil Art. 1218 (force majeure).
///
/// ## French Text
///
/// > Il y a force majeure en matière contractuelle lorsqu'un événement échappant au contrôle
/// > du débiteur, qui ne pouvait être raisonnablement prévu lors de la conclusion du contrat
/// > et dont les effets ne peuvent être évités par des mesures appropriées, empêche l'exécution
/// > de son obligation par le débiteur.
/// >
/// > Si l'empêchement est temporaire, l'exécution de l'obligation est suspendue à moins que
/// > le retard qui en résulterait ne justifie la résolution du contrat. Si l'empêchement est
/// > définitif, le contrat est résolu de plein droit et les parties sont libérées de leurs
/// > obligations dans les conditions prévues aux articles 1351 et 1351-1.
///
/// ## English Translation
///
/// > There is force majeure in contractual matters when an event beyond the debtor's control,
/// > which could not reasonably have been foreseen at the time of contracting, and whose effects
/// > cannot be avoided by appropriate measures, prevents the debtor from performing the obligation.
/// >
/// > If the impediment is temporary, performance of the obligation is suspended unless the
/// > resulting delay justifies termination of the contract. If the impediment is permanent,
/// > the contract is resolved by operation of law and the parties are released from their
/// > obligations under Articles 1351 and 1351-1.
///
/// ## Three Cumulative Criteria (Critères cumulatifs)
///
/// All three of the following must be established for force majeure to apply:
///
/// ### 1. Externality (Extériorité)
///
/// The event must be beyond the debtor's control (external to the debtor's sphere of activity).
/// Internal events — employee strikes, equipment failure from poor maintenance — are generally
/// not external. COVID-19 pandemic qualified as external for most debtors.
///
/// ### 2. Unforeseeability (Imprévisibilité)
///
/// The event must not have been reasonably foreseeable at the time of contracting.
/// Courts assess foreseeability at contract formation, not at the time of the event.
/// A debtor who contracts during a known hurricane season cannot claim force majeure
/// for a hurricane that materializes.
///
/// ### 3. Irresistibility (Irrésistibilité)
///
/// The effects of the event cannot be avoided by appropriate measures. This is the most
/// demanding criterion. Courts examine whether the debtor took all reasonable steps to
/// mitigate or work around the impediment.
///
/// ## Effects
///
/// - **Temporary impediment**: Suspends the obligation; if delay becomes excessive,
///   either party may terminate (résolution).
/// - **Permanent impediment**: Contract dissolved by operation of law (résolution de plein droit);
///   parties released from future obligations. Past performance may be unaffected.
///
/// ## Comparative Analysis
///
/// | Jurisdiction | Doctrine | Criteria |
/// |---|---|---|
/// | **France** | Force majeure (Art. 1218) | External + Unforeseeable + Irresistible |
/// | **Germany** | Unmöglichkeit (§275 BGB) | Objective/subjective impossibility |
/// | **Japan** | 履行不能 Art. 415 + 541 | Impossibility attributable to neither party |
/// | **CISG** | Art. 79 | Beyond control + not reasonably expected + cannot overcome |
/// | **UK/Common Law** | Frustration | Radical change in obligation; narrow doctrine |
///
/// French force majeure is generally broader than common law frustration, allowing
/// temporary suspension (not just dissolution).
#[must_use]
pub fn article1218() -> Statute {
    Statute::new(
        "code-civil-1218",
        "Code civil Article 1218 — Force majeure (Force Majeure)",
        Effect::new(
            EffectType::StatusChange,
            "La force majeure suspend ou résout l'obligation contractuelle.\n\nForce majeure suspends or extinguishes the contractual obligation.",
        )
        .with_parameter("effect_temporary", "Suspension de l'obligation / Suspension of obligation")
        .with_parameter("effect_permanent", "Résolution de plein droit / Resolution by operation of law")
        .with_parameter("criteria", "Extériorité + Imprévisibilité + Irrésistibilité"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(
        Condition::And(
            Box::new(Condition::HasAttribute {
                key: "is_contract".to_string(),
            }),
            Box::new(Condition::And(
                Box::new(Condition::HasAttribute {
                    key: "event_unforeseeable".to_string(),
                }),
                Box::new(Condition::And(
                    Box::new(Condition::HasAttribute {
                        key: "event_irresistible".to_string(),
                    }),
                    Box::new(Condition::HasAttribute {
                        key: "event_external".to_string(),
                    }),
                )),
            )),
        ),
    )
    .with_discretion(
        "L'article 1218 codifie la force majeure contractuelle en exigeant trois critères cumulatifs: \
        (1) extériorité — l'événement échappe au contrôle du débiteur; \
        (2) imprévisibilité — l'événement n'était pas raisonnablement prévisible lors de la conclusion; \
        (3) irrésistibilité — les effets ne peuvent être évités par des mesures appropriées. \
        L'empêchement temporaire suspend l'obligation; l'empêchement définitif résout le contrat \
        de plein droit. Tous trois critères doivent être réunis simultanément. \
        Les cours apprécient chaque critère in concreto selon les circonstances de l'espèce.\
        \n\n\
        Article 1218 codifies contractual force majeure requiring three cumulative criteria: \
        (1) externality — the event is beyond the debtor's control; \
        (2) unforeseeability — the event was not reasonably foreseeable at contracting; \
        (3) irresistibility — the effects cannot be avoided by appropriate measures. \
        A temporary impediment suspends the obligation; a permanent impediment dissolves \
        the contract by operation of law. All three criteria must be simultaneously met. \
        Courts assess each criterion in concreto based on the specific circumstances.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article1218_has_correct_id() {
        let s = article1218();
        assert_eq!(s.id, "code-civil-1218");
    }

    #[test]
    fn article1218_has_fr_jurisdiction() {
        let s = article1218();
        assert_eq!(s.jurisdiction, Some("FR".to_string()));
    }

    #[test]
    fn article1218_has_preconditions() {
        let s = article1218();
        assert!(!s.preconditions.is_empty());
    }

    #[test]
    fn article1218_has_discretion() {
        let s = article1218();
        assert!(s.discretion_logic.is_some());
        assert!(!s.discretion_logic.as_ref().unwrap().is_empty());
    }

    #[test]
    fn article1218_effect_is_status_change() {
        let s = article1218();
        assert!(matches!(s.effect.effect_type, EffectType::StatusChange));
    }

    #[test]
    fn article1218_has_three_criteria_parameters() {
        let s = article1218();
        assert!(s.effect.parameters.contains_key("criteria"));
        assert!(s.effect.parameters.contains_key("effect_temporary"));
        assert!(s.effect.parameters.contains_key("effect_permanent"));
    }

    #[test]
    fn article1218_is_valid() {
        let s = article1218();
        assert!(s.is_valid());
        assert_eq!(s.validate().len(), 0);
    }
}
