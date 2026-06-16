//! BGB § 832 - Liability of a Person with a Duty of Supervision (Aufsichtspflicht)
//!
//! ## German Text (§ 832 BGB)
//!
//! > (1) Wer kraft Gesetzes zur Führung der Aufsicht über eine Person verpflichtet
//! > ist, die wegen Minderjährigkeit oder wegen ihres geistigen oder körperlichen
//! > Zustands der Beaufsichtigung bedarf, ist zum Ersatz des Schadens verpflichtet,
//! > den diese Person einem Dritten widerrechtlich zufügt. Die Ersatzpflicht tritt
//! > nicht ein, wenn er seiner Aufsichtspflicht genügt oder wenn der Schaden auch
//! > bei gehöriger Aufsichtsführung entstanden sein würde.
//! >
//! > (2) Die gleiche Verantwortlichkeit trifft denjenigen, welcher die Führung der
//! > Aufsicht durch Vertrag übernimmt.
//!
//! ## English Translation
//!
//! > (1) A person who is obliged by operation of law to supervise a person who
//! > requires supervision because of minority or because of their mental or
//! > physical condition is liable to make compensation for the damage that this
//! > person unlawfully causes to a third party. Liability does not arise if the
//! > supervisor fulfils the duty of supervision or if the damage would also have
//! > occurred even if the supervision had been properly exercised.
//! >
//! > (2) The same responsibility applies to a person who assumes the supervision
//! > by contract.
//!
//! ## Legal Structure
//!
//! § 832 is a case of **presumed fault** (vermutetes Verschulden / Verschulden mit
//! Beweislastumkehr): once the requirements are met, the supervisor's breach of
//! the duty of supervision is presumed and the supervisor bears the burden of
//! **exculpation** (Entlastungsbeweis).
//!
//! **Requirements**:
//! 1. A duty of supervision exists, either **by law** (Abs. 1, e.g. parents,
//!    § 1631 BGB) or **by contract** (Abs. 2, e.g. childminders, care homes).
//! 2. The supervised person **requires** supervision because of minority
//!    (Minderjährigkeit) or a mental/physical condition.
//! 3. That person **unlawfully** caused damage to a third party.
//! 4. **No exculpation**: the supervisor neither (a) fulfilled the duty of
//!    supervision, nor (b) can show that the damage would have occurred anyway.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, TortParty};
use crate::gmbhg::Capital;

/// Basis of the duty of supervision under § 832 BGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisionBasis {
    /// Statutory duty (kraft Gesetzes, § 832 Abs. 1) - e.g. parents.
    Statutory,
    /// Contractual duty (durch Vertrag, § 832 Abs. 2) - e.g. childminder, care home.
    Contractual,
}

/// Reason the supervised person requires supervision (§ 832 Abs. 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisionReason {
    /// Minority (Minderjährigkeit).
    Minority,
    /// Mental or physical condition (geistiger oder körperlicher Zustand).
    MentalOrPhysicalCondition,
}

/// Claim under § 832 BGB for liability of a person with a duty of supervision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupervisionLiabilityClaim {
    /// The supervisor (Aufsichtspflichtiger).
    pub supervisor: TortParty,
    /// The supervised person (Beaufsichtigter).
    pub supervised_person: TortParty,
    /// The injured third party (geschädigter Dritter).
    pub injured_third_party: TortParty,
    /// Basis of the duty (statutory / contractual).
    pub supervision_basis: SupervisionBasis,
    /// Why the supervised person needs supervision.
    pub supervision_reason: SupervisionReason,
    /// Whether the supervised person unlawfully caused damage (widerrechtlich zugefügt).
    pub unlawful_damage_caused: bool,
    /// Exculpation (a): the duty of supervision was actually fulfilled.
    pub supervision_duty_fulfilled: bool,
    /// Exculpation (b): the damage would have occurred even with proper supervision.
    pub damage_would_have_occurred_anyway: bool,
    /// Date of the incident.
    pub incident_date: DateTime<Utc>,
    /// Damages claimed.
    pub damages: DamageClaim,
    /// Causation established.
    pub causation_established: bool,
    /// Additional notes.
    pub notes: Option<String>,
}

/// Builder for [`SupervisionLiabilityClaim`] (§ 832 BGB).
#[derive(Debug, Default)]
pub struct SupervisionLiabilityClaimBuilder {
    supervisor: Option<TortParty>,
    supervised_person: Option<TortParty>,
    injured_third_party: Option<TortParty>,
    supervision_basis: Option<SupervisionBasis>,
    supervision_reason: Option<SupervisionReason>,
    unlawful_damage_caused: bool,
    supervision_duty_fulfilled: bool,
    damage_would_have_occurred_anyway: bool,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    personal_injury: Option<Capital>,
    medical_expenses: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl SupervisionLiabilityClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the supervisor.
    pub fn supervisor(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.supervisor = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the supervised person.
    pub fn supervised_person(
        mut self,
        name: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        self.supervised_person = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the injured third party.
    pub fn injured_third_party(
        mut self,
        name: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        self.injured_third_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the basis of the supervision duty.
    pub fn supervision_basis(mut self, basis: SupervisionBasis) -> Self {
        self.supervision_basis = Some(basis);
        self
    }

    /// Set the reason supervision is required.
    pub fn supervision_reason(mut self, reason: SupervisionReason) -> Self {
        self.supervision_reason = Some(reason);
        self
    }

    /// Set whether the supervised person unlawfully caused damage.
    pub fn unlawful_damage_caused(mut self, value: bool) -> Self {
        self.unlawful_damage_caused = value;
        self
    }

    /// Set exculpation (a): duty of supervision fulfilled.
    pub fn supervision_duty_fulfilled(mut self, value: bool) -> Self {
        self.supervision_duty_fulfilled = value;
        self
    }

    /// Set exculpation (b): damage would have occurred anyway.
    pub fn damage_would_have_occurred_anyway(mut self, value: bool) -> Self {
        self.damage_would_have_occurred_anyway = value;
        self
    }

    /// Set the incident date.
    pub fn incident_date(mut self, date: DateTime<Utc>) -> Self {
        self.incident_date = Some(date);
        self
    }

    /// Set property damage.
    pub fn damages_property(mut self, amount: Capital) -> Self {
        self.property_damage = Some(amount);
        self
    }

    /// Set personal injury compensation.
    pub fn damages_personal_injury(mut self, amount: Capital) -> Self {
        self.personal_injury = Some(amount);
        self
    }

    /// Set medical expenses.
    pub fn damages_medical(mut self, amount: Capital) -> Self {
        self.medical_expenses = Some(amount);
        self
    }

    /// Set whether causation is established.
    pub fn causation_established(mut self, established: bool) -> Self {
        self.causation_established = established;
        self
    }

    /// Add notes.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Build the claim.
    pub fn build(self) -> std::result::Result<SupervisionLiabilityClaim, String> {
        let supervisor = self.supervisor.ok_or("Supervisor required")?;
        let supervised_person = self.supervised_person.ok_or("Supervised person required")?;
        let injured_third_party = self
            .injured_third_party
            .ok_or("Injured third party required")?;
        let supervision_basis = self.supervision_basis.ok_or("Supervision basis required")?;
        let supervision_reason = self
            .supervision_reason
            .ok_or("Supervision reason required")?;
        let incident_date = self.incident_date.ok_or("Incident date required")?;

        let mut damages = DamageClaim {
            property_damage: self.property_damage,
            personal_injury: self.personal_injury,
            pain_and_suffering: None,
            lost_income: None,
            medical_expenses: self.medical_expenses,
            consequential_damages: None,
            total: Capital { amount_cents: 0 },
        };
        damages.calculate_total();

        Ok(SupervisionLiabilityClaim {
            supervisor,
            supervised_person,
            injured_third_party,
            supervision_basis,
            supervision_reason,
            unlawful_damage_caused: self.unlawful_damage_caused,
            supervision_duty_fulfilled: self.supervision_duty_fulfilled,
            damage_would_have_occurred_anyway: self.damage_would_have_occurred_anyway,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate a claim under § 832 BGB.
///
/// Liability is presumed once the requirements are met; the supervisor escapes
/// liability only by exculpation (duty fulfilled, or damage would have occurred
/// anyway).
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = SupervisionLiabilityClaimBuilder::new()
///     .supervisor("Parent", "Berlin")
///     .supervised_person("Child (8)", "Berlin")
///     .injured_third_party("Neighbour", "Berlin")
///     .supervision_basis(SupervisionBasis::Statutory)
///     .supervision_reason(SupervisionReason::Minority)
///     .unlawful_damage_caused(true)
///     .incident_date(Utc::now())
///     .damages_property(Capital::from_euros(2_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(validate_supervision_liability(&claim).is_ok());
/// ```
pub fn validate_supervision_liability(claim: &SupervisionLiabilityClaim) -> Result<()> {
    if claim.supervisor.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.injured_third_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement: the supervised person must have unlawfully caused damage.
    if !claim.unlawful_damage_caused {
        return Err(TortError::NoUnlawfulDamageBySupervised);
    }

    // Exculpation (Entlastungsbeweis), § 832 Abs. 1 S. 2.
    if claim.supervision_duty_fulfilled {
        return Err(TortError::SupervisorExculpated {
            ground: "Aufsichtspflicht genügt / duty of supervision fulfilled".to_string(),
        });
    }
    if claim.damage_would_have_occurred_anyway {
        return Err(TortError::SupervisorExculpated {
            ground: "Schaden wäre auch bei gehöriger Aufsicht entstanden / damage would have \
                     occurred anyway"
                .to_string(),
        });
    }

    if !claim.causation_established {
        return Err(TortError::CausationNotProven);
    }
    if claim.damages.total.amount_cents == 0 {
        return Err(TortError::ZeroDamage);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_claim() -> SupervisionLiabilityClaim {
        SupervisionLiabilityClaimBuilder::new()
            .supervisor("Parent", "Berlin")
            .supervised_person("Child (8)", "Berlin")
            .injured_third_party("Neighbour", "Berlin")
            .supervision_basis(SupervisionBasis::Statutory)
            .supervision_reason(SupervisionReason::Minority)
            .unlawful_damage_caused(true)
            .incident_date(Utc::now())
            .damages_property(Capital::from_euros(2_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_valid_statutory_supervision() {
        let claim = valid_claim();
        assert!(validate_supervision_liability(&claim).is_ok());
    }

    #[test]
    fn test_valid_contractual_supervision() {
        let mut claim = valid_claim();
        claim.supervision_basis = SupervisionBasis::Contractual;
        claim.supervision_reason = SupervisionReason::MentalOrPhysicalCondition;
        assert!(validate_supervision_liability(&claim).is_ok());
    }

    #[test]
    fn test_no_unlawful_damage() {
        let mut claim = valid_claim();
        claim.unlawful_damage_caused = false;
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::NoUnlawfulDamageBySupervised)
        ));
    }

    #[test]
    fn test_exculpation_duty_fulfilled() {
        let mut claim = valid_claim();
        claim.supervision_duty_fulfilled = true;
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::SupervisorExculpated { .. })
        ));
    }

    #[test]
    fn test_exculpation_damage_anyway() {
        let mut claim = valid_claim();
        claim.damage_would_have_occurred_anyway = true;
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::SupervisorExculpated { .. })
        ));
    }

    #[test]
    fn test_no_causation() {
        let mut claim = valid_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = valid_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::ZeroDamage)
        ));
    }

    #[test]
    fn test_missing_injured_party() {
        let mut claim = valid_claim();
        claim.injured_third_party.name = String::new();
        assert!(matches!(
            validate_supervision_liability(&claim),
            Err(TortError::InjuredPartyMissing)
        ));
    }
}
