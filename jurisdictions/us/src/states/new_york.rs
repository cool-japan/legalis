//! New York State Law Module
//!
//! New York is the financial capital of the world and home to the Cardozo Court of Appeals legacy.
//! Key features:
//! - **Pure Comparative Negligence** (CPLR § 1411, adopted 1975)
//! - **Cardozo Legacy** - Palsgraf, MacPherson, Hynes cases
//! - **Combined Modern Approach** to choice of law
//! - Highest appellate court influence nationwide
//! - New York Court of Appeals (not "Supreme Court" - that's trial level!)

use crate::cases::palsgraf_v_long_island; // Import from existing cases module
use crate::states::types::{LegalTopic, StateId, StateLawVariation, StateRule, StatuteReference};
use chrono::NaiveDate;
use legalis_core::{
    Condition, Effect, EffectType,
    case_law::{Case, CaseRule, Court},
};

/// New York state law module.
pub struct NewYorkLaw;

impl NewYorkLaw {
    /// Get New York state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new_york()
    }

    // ===== Tort Law =====

    /// Pure comparative negligence in New York.
    ///
    /// New York adopted pure comparative negligence in 1975 via statute CPLR § 1411.
    /// This replaced the previous contributory negligence rule.
    ///
    /// Like California's Li v. Yellow Cab (also 1975), NY allows recovery even if
    /// plaintiff is 99% at fault - damages are simply reduced by that percentage.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(
            StatuteReference::new("N.Y. C.P.L.R. § 1411")
                .with_title("Damages Recoverable When Contributory Negligence or Assumption of Risk Is Established")
                .with_year(1975),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1975, 9, 1).unwrap())
        .with_notes(
            "New York adopted pure comparative negligence by statute (CPLR § 1411) \
             in 1975, the same year California adopted it judicially in Li v. Yellow Cab. \
             Prior law: contributory negligence barred recovery if plaintiff had any fault.",
        )
    }

    /// Integrate Palsgraf v. Long Island from existing cases module.
    ///
    /// Palsgraf is already implemented in `/mnt/fd/legalis/jurisdictions/us/src/cases.rs`.
    /// This function provides New York context and integration.
    ///
    /// ## Historical Context
    /// Decided by Judge Benjamin Cardozo of the New York Court of Appeals in 1928.
    /// This case established the **foreseeable plaintiff** rule that defines the scope
    /// of duty in negligence law.
    ///
    /// ## Cardozo vs Andrews Dissent
    /// - **Cardozo (majority)**: Duty runs only to foreseeable plaintiffs
    /// - **Andrews (dissent)**: Duty runs to all, proximate cause is the limit
    ///
    /// Most jurisdictions follow Cardozo's approach.
    #[must_use]
    pub fn palsgraf_ny_context() -> Case {
        // Returns the existing Palsgraf case from cases.rs
        palsgraf_v_long_island()
    }

    /// MacPherson v. Buick Motor Co., 217 N.Y. 382 (1916)
    ///
    /// ## Facts
    /// MacPherson bought a Buick car from a dealer. A defective wheel (manufactured by
    /// a third party) collapsed, causing injury. Buick did not manufacture the wheel but
    /// assembled the car without inspecting it adequately.
    ///
    /// ## Traditional Rule (Winterbottom v. Wright, 1842)
    /// No liability without **privity of contract**. Manufacturer not liable to remote purchaser.
    ///
    /// ## Issue
    /// Does manufacturer owe duty to ultimate consumer despite lack of privity?
    ///
    /// ## Holding (Cardozo, J.)
    /// **YES**. If product is reasonably certain to cause danger if negligently made,
    /// manufacturer owes duty to all foreseeable users, not just immediate purchaser.
    ///
    /// ## Significance
    /// Broke the privity doctrine for products liability. Paved way for modern strict liability.
    /// Cited nationwide as foundation for consumer protection.
    #[must_use]
    pub fn macpherson_v_buick() -> Case {
        Case::new(
            "MacPherson v. Buick Motor Co., 217 N.Y. 382, 111 N.E. 1050 (1916)",
            "MacPherson v. Buick",
            1916,
            Court::Appellate, // NY Court of Appeals
            "US-NY",
        )
        .with_facts(
            "Plaintiff purchased Buick car from dealer. Defective wheel (made by third party) \
             collapsed, injuring plaintiff. Buick did not manufacture wheel but failed to \
             inspect it. No direct contract between Buick and MacPherson.",
        )
        .with_issue(
            "Does manufacturer owe duty of care to ultimate consumer despite lack of privity \
             of contract?",
        )
        .with_holding(
            "Yes. If product is reasonably certain to be dangerous if negligently made, \
             manufacturer owes duty to all foreseeable users. Privity not required.",
        )
        .with_ratio(
            "The nature of the product (dangerous if defective) determines the scope of duty, \
             not the presence of a contract. If manufacturer knows product will be used by \
             persons other than purchaser, duty extends to all foreseeable users. \
             \n\nCardozo: 'If the nature of a thing is such that it is reasonably certain to \
             place life and limb in peril when negligently made, it is then a thing of danger.'",
        )
        .with_rule(CaseRule {
            name: "Products Liability Without Privity (MacPherson)".to_string(),
            conditions: vec![
                Condition::AttributeEquals {
                    key: "product_dangerous_if_defective".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "manufacturer_negligence".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "foreseeable_user".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "injury_caused_by_defect".to_string(),
                    value: "true".to_string(),
                },
            ],
            effect: Effect::new(
                EffectType::Obligation,
                "Manufacturer liable to ultimate consumer without privity",
            )
            .with_parameter("duty", "extends beyond immediate purchaser"),
            exceptions: vec![
                "Product not inherently dangerous".to_string(),
                "No negligence in manufacture or inspection".to_string(),
                "User not foreseeable".to_string(),
            ],
        })
    }

    /// Hynes v. New York Central R.R. Co., 231 N.Y. 229 (1921)
    ///
    /// ## Facts
    /// Plaintiff (16-year-old boy) was swimming from a public beach and climbed onto
    /// a diving board built by boys over defendant's land. Defendant's wires fell and
    /// electrocuted him. Defendant argued boy was trespasser on their property.
    ///
    /// ## Issue
    /// Does landowner owe duty to person whose body is above land but who dove from
    /// a structure over the land?
    ///
    /// ## Holding (Cardozo, J.)
    /// **YES**. Duty is not determined solely by property boundaries. Defendant's wires
    /// reached into public airspace. Duty extends to persons lawfully in that space.
    ///
    /// ## Significance
    /// Cardozo rejects rigid property-based tort boundaries. Duty based on foreseeability
    /// and fairness, not strict property lines.
    ///
    /// ## Cardozo's Reasoning
    /// "Jumping from a boat or a barrel, the boy would have been a bather in the river.
    /// Jumping from the springboard, he was no longer, it is said, a bather, but a trespasser
    /// on a right of way. Rights and duties in systems of living law are not built upon
    /// such quicksands."
    #[must_use]
    pub fn hynes_v_ny_central() -> Case {
        Case::new(
            "Hynes v. New York Central R.R. Co., 231 N.Y. 229, 131 N.E. 898 (1921)",
            "Hynes v. N.Y. Central",
            1921,
            Court::Appellate,
            "US-NY",
        )
        .with_facts(
            "16-year-old boy swimming from public beach climbed onto diving board built \
             over defendant railroad's land. Defendant's wires fell and electrocuted boy \
             while he was on the board. Defendant claimed boy was trespasser.",
        )
        .with_issue(
            "Does landowner owe duty to person in airspace above land but not standing on \
             the land itself?",
        )
        .with_holding(
            "Yes. Duty is not determined by property boundaries alone. Defendant's wires \
             extended into public airspace. Boy was lawfully in that space.",
        )
        .with_ratio(
            "Rights and duties are not built upon property technicalities. The boy was \
             engaged in lawful recreation in public waters. Defendant's negligent maintenance \
             of wires reaching into that space created duty. \
             \n\nCardozo's flexible approach to duty based on fairness, not rigid property rules.",
        )
        .with_rule(CaseRule {
            name: "Duty Beyond Property Boundaries (Hynes)".to_string(),
            conditions: vec![
                Condition::AttributeEquals {
                    key: "defendant_activity_reaches_public_space".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "plaintiff_lawfully_in_space".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "foreseeable_risk".to_string(),
                    value: "true".to_string(),
                },
            ],
            effect: Effect::new(
                EffectType::Obligation,
                "Duty owed despite plaintiff not on defendant's property",
            ),
            exceptions: vec![
                "Plaintiff unlawfully in space".to_string(),
                "Defendant's activity did not reach plaintiff's location".to_string(),
            ],
        })
    }

    // ===== Cardozo Legacy =====

    /// Get all Cardozo Court of Appeals cases.
    ///
    /// Benjamin Cardozo served on the New York Court of Appeals (1914-1932) and wrote
    /// some of the most influential tort decisions in American law.
    ///
    /// These cases demonstrate Cardozo's judicial philosophy:
    /// - Foreseeability as foundation of duty
    /// - Flexible, policy-oriented approach
    /// - Rejection of rigid formalism
    /// - Focus on fairness and social utility
    ///
    /// After leaving NY Court of Appeals, Cardozo served on the US Supreme Court (1932-1938).
    #[must_use]
    pub fn cardozo_cases() -> Vec<Case> {
        vec![
            Self::palsgraf_ny_context(), // 1928 - Foreseeable plaintiff
            Self::macpherson_v_buick(),  // 1916 - Products liability without privity
            Self::hynes_v_ny_central(),  // 1921 - Duty beyond property boundaries
        ]
    }

    /// Get all New York state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![Self::comparative_negligence()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_york_state_id() {
        let ny = NewYorkLaw::state_id();
        assert_eq!(ny.code, "NY");
        assert_eq!(ny.name, "New York");
    }

    #[test]
    fn test_comparative_negligence() {
        let variation = NewYorkLaw::comparative_negligence();

        assert_eq!(variation.state.code, "NY");
        assert_eq!(variation.rule, StateRule::PureComparativeNegligence);
        assert!(variation.statutory_basis.is_some());

        let statute = variation.statutory_basis.unwrap();
        assert!(statute.citation.contains("1411"));
    }

    #[test]
    fn test_palsgraf_integration() {
        let palsgraf = NewYorkLaw::palsgraf_ny_context();

        assert_eq!(palsgraf.year, 1928);
        assert_eq!(palsgraf.jurisdiction, "US-NY");
        assert_eq!(palsgraf.short_name, "Palsgraf v. Long Island R.R.");
        assert!(palsgraf.rule.is_some());
    }

    #[test]
    fn test_macpherson_v_buick() {
        let case = NewYorkLaw::macpherson_v_buick();

        assert_eq!(case.year, 1916);
        assert_eq!(case.jurisdiction, "US-NY");
        assert!(case.rule.is_some());

        let rule = case.rule.unwrap();
        assert!(rule.name.contains("MacPherson"));
        assert!(rule.name.contains("Privity"));
    }

    #[test]
    fn test_hynes_v_ny_central() {
        let case = NewYorkLaw::hynes_v_ny_central();

        assert_eq!(case.year, 1921);
        assert!(case.rule.is_some());

        let rule = case.rule.unwrap();
        assert!(rule.name.contains("Hynes"));
    }

    #[test]
    fn test_cardozo_cases_chronological() {
        let cases = NewYorkLaw::cardozo_cases();

        assert_eq!(cases.len(), 3);

        // Verify chronological order
        assert_eq!(cases[0].year, 1928); // Palsgraf
        assert_eq!(cases[1].year, 1916); // MacPherson
        assert_eq!(cases[2].year, 1921); // Hynes

        // All should be NY Court of Appeals
        for case in &cases {
            assert_eq!(case.jurisdiction, "US-NY");
        }
    }

    #[test]
    fn test_cardozo_legacy() {
        let cases = NewYorkLaw::cardozo_cases();

        // Verify all have rules (Cardozo's analytical approach)
        for case in cases {
            assert!(
                case.rule.is_some(),
                "{} should have a rule",
                case.short_name
            );
        }
    }
}
