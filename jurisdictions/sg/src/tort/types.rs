//! Tort Law - Type Definitions
//!
//! Type-safe models of the Singapore law of tort. This file covers the tort of
//! **negligence** and the tort of **defamation**; private/public nuisance,
//! occupiers' liability and the general defences are in
//! [`super::nuisance`].
//!
//! ## Negligence
//!
//! The elements are duty of care, breach, causation (factual and legal/
//! remoteness) and actionable damage. Duty in Singapore is governed by the
//! single, universal two-stage test in *Spandeck Engineering (Private) Ltd v
//! Defence Science & Technology Agency* \[2007\] SGCA 37, applied against a
//! threshold of factual foreseeability:
//!
//! 1. **Factual foreseeability** (threshold) — was harm to the claimant
//!    reasonably foreseeable?
//! 2. **Stage 1 — legal proximity** — was there sufficient closeness between the
//!    parties (physical, circumstantial, causal; voluntary assumption of
//!    responsibility and reliance) (*Hedley Byrne v Heller* \[1964\] AC 465)?
//! 3. **Stage 2 — policy** — are there policy considerations negating the duty?
//!
//! ## Defamation
//!
//! Governed by the common law and the Defamation Act 1957. A defamatory
//! statement is one that lowers the claimant in the estimation of right-thinking
//! members of society; it must refer to the claimant and be published to a third
//! party. **Libel** (permanent form) is actionable per se; **slander**
//! (transient form) requires proof of special damage except in the cases in ss.
//! 5–6 of the Act.

use serde::{Deserialize, Serialize};

// ===========================================================================
// Negligence
// ===========================================================================

/// The category of harm complained of, which bears on the proximity analysis
/// (pure economic loss and psychiatric harm attract additional scrutiny).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmCategory {
    /// Physical injury to the person.
    PersonalInjury,
    /// Physical damage to property.
    PropertyDamage,
    /// Pure economic loss not consequent on physical damage (recoverable in
    /// Singapore subject to the *Spandeck* test — *Spandeck* \[2007\] SGCA 37;
    /// *RSP Architects v Ocean Front* \[1995\] SGCA).
    PureEconomicLoss,
    /// Psychiatric harm / nervous shock.
    PsychiatricHarm,
}

/// The duty-of-care analysis under the *Spandeck* two-stage test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DutyOfCareAnalysis {
    /// Whether harm to this claimant was reasonably foreseeable (the threshold).
    pub factual_foreseeability: bool,
    /// Stage 1: whether there was sufficient legal proximity between the parties.
    pub legal_proximity: bool,
    /// Stage 2: whether there are policy considerations that negate the duty.
    pub policy_negates_duty: bool,
    /// The category of harm in issue.
    pub harm_category: HarmCategory,
}

impl DutyOfCareAnalysis {
    /// Creates a duty analysis for personal injury with the threshold and stage 1
    /// satisfied and no negating policy (the typical positive case).
    pub fn established(harm_category: HarmCategory) -> Self {
        Self {
            factual_foreseeability: true,
            legal_proximity: true,
            policy_negates_duty: false,
            harm_category,
        }
    }

    /// Sets whether harm was factually foreseeable.
    pub fn with_foreseeability(mut self, value: bool) -> Self {
        self.factual_foreseeability = value;
        self
    }

    /// Sets whether there was legal proximity.
    pub fn with_proximity(mut self, value: bool) -> Self {
        self.legal_proximity = value;
        self
    }

    /// Records that policy considerations negate the duty.
    pub fn with_policy_negation(mut self) -> Self {
        self.policy_negates_duty = true;
        self
    }

    /// Returns whether a duty of care is owed: the threshold and stage 1 must be
    /// satisfied and stage 2 policy must not negate it.
    pub fn duty_owed(&self) -> bool {
        self.factual_foreseeability && self.legal_proximity && !self.policy_negates_duty
    }
}

/// The standard of care applicable to the defendant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardOfCare {
    /// The ordinary reasonable person (*Blyth v Birmingham Waterworks* (1856)
    /// 11 Ex 781).
    ReasonablePerson,
    /// A professional exercising the skill of the ordinary competent member of
    /// that profession (the *Bolam* test, with the *Bolitho* gloss).
    Professional,
    /// The standard appropriate to a child of the defendant's age (*Mullin v
    /// Richards* \[1998\] 1 WLR 1304).
    Child,
    /// A learner held to the standard of the reasonably competent practitioner
    /// (*Nettleship v Weston* \[1971\] 2 QB 691).
    Learner,
}

impl StandardOfCare {
    /// Returns the leading authority for this standard.
    pub fn authority(&self) -> &'static str {
        match self {
            StandardOfCare::ReasonablePerson => "Blyth v Birmingham Waterworks (1856) 11 Ex 781",
            StandardOfCare::Professional => "Bolam v Friern Hospital [1957] 1 WLR 582",
            StandardOfCare::Child => "Mullin v Richards [1998] 1 WLR 1304",
            StandardOfCare::Learner => "Nettleship v Weston [1971] 2 QB 691",
        }
    }
}

/// The breach analysis: whether the defendant fell below the applicable standard,
/// having regard to the risk-calculus factors.
///
/// The factors are the likelihood of harm, the seriousness of potential harm,
/// the cost/practicability of precautions and the social utility of the conduct
/// (*Bolton v Stone* \[1951\] AC 850; *Wagon Mound (No 2)* \[1967\] 1 AC 617).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachAnalysis {
    /// The applicable standard of care.
    pub standard: StandardOfCare,
    /// Whether, on the facts, the defendant fell below that standard.
    pub fell_below_standard: bool,
    /// Likelihood of harm materialising (0–100).
    pub likelihood_of_harm: u8,
    /// Seriousness of the potential harm (0–100).
    pub seriousness_of_harm: u8,
    /// Cost / practicability of taking precautions (0–100; higher = costlier).
    pub cost_of_precautions: u8,
}

impl BreachAnalysis {
    /// Creates a breach analysis recording whether the defendant fell below the
    /// standard.
    pub fn new(standard: StandardOfCare, fell_below_standard: bool) -> Self {
        Self {
            standard,
            fell_below_standard,
            likelihood_of_harm: 0,
            seriousness_of_harm: 0,
            cost_of_precautions: 0,
        }
    }

    /// Records the risk-calculus factors.
    pub fn with_risk_factors(
        mut self,
        likelihood: u8,
        seriousness: u8,
        cost_of_precautions: u8,
    ) -> Self {
        self.likelihood_of_harm = likelihood.min(100);
        self.seriousness_of_harm = seriousness.min(100);
        self.cost_of_precautions = cost_of_precautions.min(100);
        self
    }

    /// Returns whether there was a breach of the standard of care.
    pub fn is_breach(&self) -> bool {
        self.fell_below_standard
    }
}

/// The causation analysis: factual ("but for") causation, intervening acts and
/// legal remoteness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CausationAnalysis {
    /// Whether the breach satisfies the "but for" test (*Barnett v Chelsea &
    /// Kensington Hospital* \[1969\] 1 QB 428).
    pub but_for_satisfied: bool,
    /// Whether an intervening act broke the chain of causation (novus actus
    /// interveniens).
    pub novus_actus: bool,
    /// Whether the kind of damage was reasonably foreseeable (*The Wagon Mound
    /// (No 1)* \[1961\] AC 388).
    pub damage_kind_foreseeable: bool,
}

impl CausationAnalysis {
    /// Creates a causation analysis with all elements satisfied (the typical
    /// positive case).
    pub fn established() -> Self {
        Self {
            but_for_satisfied: true,
            novus_actus: false,
            damage_kind_foreseeable: true,
        }
    }

    /// Sets whether the "but for" test is satisfied.
    pub fn with_but_for(mut self, value: bool) -> Self {
        self.but_for_satisfied = value;
        self
    }

    /// Records an intervening act that breaks the chain.
    pub fn with_novus_actus(mut self) -> Self {
        self.novus_actus = true;
        self
    }

    /// Sets whether the kind of damage was foreseeable.
    pub fn with_foreseeable_damage(mut self, value: bool) -> Self {
        self.damage_kind_foreseeable = value;
        self
    }

    /// Returns whether causation (factual and legal) is established.
    pub fn causation_established(&self) -> bool {
        self.but_for_satisfied && !self.novus_actus && self.damage_kind_foreseeable
    }
}

/// A complete negligence claim assembled from its constituent analyses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegligenceClaim {
    /// Identifier of the claim.
    pub id: String,
    /// The claimant.
    pub claimant: String,
    /// The defendant.
    pub defendant: String,
    /// Duty-of-care analysis.
    pub duty: DutyOfCareAnalysis,
    /// Breach analysis.
    pub breach: BreachAnalysis,
    /// Causation analysis.
    pub causation: CausationAnalysis,
    /// Actionable damage suffered, in SGD cents.
    pub damage_cents: u64,
}

impl NegligenceClaim {
    /// Assembles a negligence claim from its elements.
    pub fn new(
        id: impl Into<String>,
        claimant: impl Into<String>,
        defendant: impl Into<String>,
        duty: DutyOfCareAnalysis,
        breach: BreachAnalysis,
        causation: CausationAnalysis,
        damage_cents: u64,
    ) -> Self {
        Self {
            id: id.into(),
            claimant: claimant.into(),
            defendant: defendant.into(),
            duty,
            breach,
            causation,
            damage_cents,
        }
    }

    /// Returns whether all four elements (duty, breach, causation, damage) are
    /// established.
    pub fn is_established(&self) -> bool {
        self.duty.duty_owed()
            && self.breach.is_breach()
            && self.causation.causation_established()
            && self.damage_cents > 0
    }
}

// ===========================================================================
// Defamation
// ===========================================================================

/// The form of a defamatory statement, which fixes whether it is actionable per
/// se.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationForm {
    /// Libel: a statement in permanent form (writing, print, broadcast). Under
    /// the Defamation Act 1957 broadcast words are treated as publication in
    /// permanent form (s. 4). Actionable without proof of special damage.
    Libel,
    /// Slander: a statement in transient form (spoken words, gestures).
    /// Generally requires proof of special damage.
    Slander,
}

/// A statutory exception under which slander is actionable per se (without proof
/// of special damage).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlanderPerSeException {
    /// Section 5 — words imputing a criminal offence punishable with
    /// imprisonment.
    CriminalOffence,
    /// Section 6 — words calculated to disparage the claimant in any office,
    /// profession, calling, trade or business.
    DisparagementInOffice,
    /// Common-law exception — imputation of a contagious/loathsome disease.
    ContagiousDisease,
    /// Common-law exception — imputation of unchastity to a woman/girl
    /// (preserved in some jurisdictions).
    Unchastity,
}

impl SlanderPerSeException {
    /// Returns the statutory or common-law basis for the exception.
    pub fn basis(&self) -> &'static str {
        match self {
            SlanderPerSeException::CriminalOffence => "Defamation Act 1957 s. 5",
            SlanderPerSeException::DisparagementInOffice => "Defamation Act 1957 s. 6",
            SlanderPerSeException::ContagiousDisease => "common law (loathsome disease)",
            SlanderPerSeException::Unchastity => "common law (imputation of unchastity)",
        }
    }
}

/// A defence to a defamation claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefamationDefence {
    /// Justification / truth — the statement is substantially true (Defamation
    /// Act 1957 s. 8; a complete defence).
    Justification,
    /// Fair comment — honest comment on a matter of public interest based on
    /// true facts (*Review Publishing v Lee Hsien Loong* \[2009\] SGCA 46).
    FairComment,
    /// Absolute privilege — e.g. statements in Parliament or judicial
    /// proceedings.
    AbsolutePrivilege,
    /// Qualified privilege — a duty/interest occasion, defeated by malice
    /// (*Reynolds v Times Newspapers* \[2001\] 2 AC 127).
    QualifiedPrivilege,
    /// Offer of amends (Defamation Act 1957 s. 7) for unintentional defamation.
    OfferOfAmends,
}

impl DefamationDefence {
    /// Returns the controlling authority / statutory provision.
    pub fn authority(&self) -> &'static str {
        match self {
            DefamationDefence::Justification => "Defamation Act 1957 s. 8",
            DefamationDefence::FairComment => "Review Publishing v Lee Hsien Loong [2009] SGCA 46",
            DefamationDefence::AbsolutePrivilege => "common law / parliamentary privilege",
            DefamationDefence::QualifiedPrivilege => "Reynolds v Times Newspapers [2001] 2 AC 127",
            DefamationDefence::OfferOfAmends => "Defamation Act 1957 s. 7",
        }
    }

    /// Returns whether the defence, if made out, is a complete answer to the
    /// claim.
    pub fn is_complete_defence(&self) -> bool {
        // Each of these is a complete defence when its elements are satisfied;
        // qualified privilege is complete unless defeated by malice (handled in
        // the validator).
        true
    }
}

/// A defamation claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefamationClaim {
    /// Identifier of the claim.
    pub id: String,
    /// The claimant (the person allegedly defamed).
    pub claimant: String,
    /// The defendant (the publisher).
    pub defendant: String,
    /// The statement complained of.
    pub statement: String,
    /// The form (libel or slander).
    pub form: DefamationForm,
    /// Whether the statement is defamatory in meaning (lowers the claimant in
    /// the estimation of right-thinking persons).
    pub defamatory_meaning: bool,
    /// Whether the statement refers to (identifies) the claimant.
    pub refers_to_claimant: bool,
    /// Whether the statement was published to at least one third party.
    pub published_to_third_party: bool,
    /// For slander: any per-se exception that applies.
    pub slander_exception: Option<SlanderPerSeException>,
    /// For slander: whether special (pecuniary) damage is proved.
    pub special_damage_proved: bool,
    /// Defences raised by the defendant.
    pub defences: Vec<DefamationDefence>,
    /// For qualified privilege: whether the claimant proves malice (which
    /// defeats the defence).
    pub malice_proved: bool,
}

impl DefamationClaim {
    /// Creates a defamation claim with the three core ingredients satisfied
    /// (defamatory meaning, reference, publication), which the caller may then
    /// adjust.
    pub fn new(
        id: impl Into<String>,
        claimant: impl Into<String>,
        defendant: impl Into<String>,
        statement: impl Into<String>,
        form: DefamationForm,
    ) -> Self {
        Self {
            id: id.into(),
            claimant: claimant.into(),
            defendant: defendant.into(),
            statement: statement.into(),
            form,
            defamatory_meaning: true,
            refers_to_claimant: true,
            published_to_third_party: true,
            slander_exception: None,
            special_damage_proved: false,
            defences: Vec::new(),
            malice_proved: false,
        }
    }

    /// Sets whether the statement bears a defamatory meaning.
    pub fn with_defamatory_meaning(mut self, value: bool) -> Self {
        self.defamatory_meaning = value;
        self
    }

    /// Sets whether the statement refers to the claimant.
    pub fn with_reference(mut self, value: bool) -> Self {
        self.refers_to_claimant = value;
        self
    }

    /// Sets whether the statement was published to a third party.
    pub fn with_publication(mut self, value: bool) -> Self {
        self.published_to_third_party = value;
        self
    }

    /// Records a slander per-se exception.
    pub fn with_slander_exception(mut self, exception: SlanderPerSeException) -> Self {
        self.slander_exception = Some(exception);
        self
    }

    /// Records that special damage is proved (relevant to slander).
    pub fn with_special_damage(mut self) -> Self {
        self.special_damage_proved = true;
        self
    }

    /// Adds a defence.
    pub fn add_defence(&mut self, defence: DefamationDefence) {
        self.defences.push(defence);
    }

    /// Records that malice is proved (defeats qualified privilege / fair
    /// comment).
    pub fn with_malice(mut self) -> Self {
        self.malice_proved = true;
        self
    }

    /// Returns whether the three core ingredients of defamation are present.
    pub fn core_ingredients_present(&self) -> bool {
        self.defamatory_meaning && self.refers_to_claimant && self.published_to_third_party
    }

    /// Returns whether the statement is actionable per se (no special damage
    /// required): always for libel; for slander only within an exception.
    pub fn actionable_per_se(&self) -> bool {
        match self.form {
            DefamationForm::Libel => true,
            DefamationForm::Slander => self.slander_exception.is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duty_owed_under_two_stage_test() {
        let duty = DutyOfCareAnalysis::established(HarmCategory::PersonalInjury);
        assert!(duty.duty_owed());
    }

    #[test]
    fn policy_negates_duty() {
        let duty =
            DutyOfCareAnalysis::established(HarmCategory::PureEconomicLoss).with_policy_negation();
        assert!(!duty.duty_owed());
    }

    #[test]
    fn no_proximity_means_no_duty() {
        let duty =
            DutyOfCareAnalysis::established(HarmCategory::PsychiatricHarm).with_proximity(false);
        assert!(!duty.duty_owed());
    }

    #[test]
    fn breach_records_risk_factors() {
        let breach = BreachAnalysis::new(StandardOfCare::ReasonablePerson, true)
            .with_risk_factors(80, 90, 10);
        assert!(breach.is_breach());
        assert_eq!(breach.likelihood_of_harm, 80);
    }

    #[test]
    fn causation_broken_by_novus_actus() {
        let causation = CausationAnalysis::established().with_novus_actus();
        assert!(!causation.causation_established());
    }

    #[test]
    fn unforeseeable_damage_breaks_legal_causation() {
        let causation = CausationAnalysis::established().with_foreseeable_damage(false);
        assert!(!causation.causation_established());
    }

    #[test]
    fn full_negligence_claim_established() {
        let claim = NegligenceClaim::new(
            "n1",
            "Plaintiff",
            "Defendant",
            DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
            BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
            CausationAnalysis::established(),
            500_000,
        );
        assert!(claim.is_established());
    }

    #[test]
    fn negligence_fails_without_damage() {
        let claim = NegligenceClaim::new(
            "n2",
            "P",
            "D",
            DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
            BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
            CausationAnalysis::established(),
            0,
        );
        assert!(!claim.is_established());
    }

    #[test]
    fn libel_is_actionable_per_se() {
        let claim = DefamationClaim::new("d1", "P", "D", "P is a thief", DefamationForm::Libel);
        assert!(claim.actionable_per_se());
        assert!(claim.core_ingredients_present());
    }

    #[test]
    fn slander_actionable_per_se_only_within_exception() {
        let bare = DefamationClaim::new("d2", "P", "D", "P is lazy", DefamationForm::Slander);
        assert!(!bare.actionable_per_se());

        let criminal = bare.with_slander_exception(SlanderPerSeException::CriminalOffence);
        assert!(criminal.actionable_per_se());
        assert_eq!(
            criminal.slander_exception.expect("set").basis(),
            "Defamation Act 1957 s. 5"
        );
    }

    #[test]
    fn serde_roundtrip_defamation() {
        let mut claim =
            DefamationClaim::new("d3", "P", "D", "P took bribes", DefamationForm::Libel);
        claim.add_defence(DefamationDefence::Justification);
        let json = serde_json::to_string(&claim).expect("serialize");
        let back: DefamationClaim = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(claim, back);
    }
}
