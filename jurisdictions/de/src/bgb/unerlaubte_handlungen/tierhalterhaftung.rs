//! BGB §§ 833-835 - Animal Liability (Tierhalterhaftung)
//!
//! ## § 833 BGB - Liability of the Animal Keeper (Haftung des Tierhalters)
//!
//! > Wird durch ein Tier ein Mensch getötet oder der Körper oder die Gesundheit
//! > eines Menschen verletzt oder eine Sache beschädigt, so ist derjenige, welcher
//! > das Tier hält, verpflichtet, dem Verletzten den daraus entstehenden Schaden zu
//! > ersetzen. Die Ersatzpflicht tritt nicht ein, wenn der Schaden durch ein
//! > Haustier verursacht wird, das dem Beruf, der Erwerbstätigkeit oder dem
//! > Unterhalt des Tierhalters zu dienen bestimmt ist, und entweder der Tierhalter
//! > bei der Beaufsichtigung des Tieres die im Verkehr erforderliche Sorgfalt
//! > beobachtet oder der Schaden auch bei Anwendung dieser Sorgfalt entstanden sein
//! > würde.
//!
//! **English**: If an animal kills a person, injures the body or health of a
//! person, or damages a thing, the keeper of the animal is liable to compensate
//! the injured party for the resulting damage. Liability does not arise if the
//! damage is caused by a **domestic animal intended to serve the keeper's
//! profession, business or livelihood** (Nutztier) and the keeper either observed
//! the required care in supervising the animal or the damage would have occurred
//! even with such care.
//!
//! ## § 834 BGB - Liability of the Animal Supervisor (Haftung des Tieraufsehers)
//!
//! A person who, by contract, assumes the supervision of an animal for the keeper
//! is liable for damage the animal causes to a third party in the manner of § 833;
//! liability is excluded if the supervisor observed the required care or the
//! damage would have occurred anyway.
//!
//! ## § 835 BGB
//!
//! § 835 BGB (Wildschaden - liability for damage caused by game) was **repealed**
//! ("aufgehoben") with effect from 1 April 2010; game damage is now governed by the
//! Bundesjagdgesetz (BJagdG). It is therefore not modelled here. See
//! [`SECTION_835_REPEALED`].
//!
//! ## Legal Structure
//!
//! - **§ 833 S. 1 (Luxustier)**: strict liability (Gefährdungshaftung) - no
//!   exculpation is possible for hobby/luxury animals.
//! - **§ 833 S. 2 (Nutztier)**: liability with **exculpation** for animals serving
//!   the keeper's occupation/livelihood (presumed-fault liability).
//! - **§ 834 (Tieraufseher)**: presumed-fault liability with exculpation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, PhysicalHarmType, TortParty};
use crate::gmbhg::Capital;

/// Documentation marker: § 835 BGB was repealed effective 1 April 2010.
pub const SECTION_835_REPEALED: &str =
    "§ 835 BGB (Wildschaden) aufgehoben zum 01.04.2010; jetzt Bundesjagdgesetz (BJagdG)";

/// Who is being held liable under §§ 833-834 BGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalLiabilityBasis {
    /// Animal keeper (Tierhalter, § 833 BGB).
    Keeper,
    /// Contractual animal supervisor (Tieraufseher, § 834 BGB).
    Supervisor,
}

/// Category of the animal, decisive for whether exculpation is available (§ 833).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalCategory {
    /// Hobby / luxury animal (Luxustier) - § 833 S. 1: strict liability, no exculpation.
    LuxuryAnimal,
    /// Domestic animal serving the keeper's profession/business/livelihood
    /// (Nutztier) - § 833 S. 2: liability with exculpation.
    DomesticUtilityAnimal,
}

/// Claim under §§ 833-834 BGB for damage caused by an animal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimalLiabilityClaim {
    /// The liable party (keeper or supervisor).
    pub liable_party: TortParty,
    /// The injured party (Verletzter).
    pub injured_party: TortParty,
    /// Description of the animal.
    pub animal_description: String,
    /// Whether liability is asserted against the keeper (§ 833) or supervisor (§ 834).
    pub basis: AnimalLiabilityBasis,
    /// Category of the animal (luxury vs utility) - relevant for § 833 exculpation.
    pub animal_category: AnimalCategory,
    /// The type of harm caused.
    pub harm_type: PhysicalHarmType,
    /// Whether the damage resulted from the specific animal hazard (Tiergefahr).
    pub caused_by_animal: bool,
    /// Exculpation (a): the required care (verkehrserforderliche Sorgfalt) was observed.
    pub required_care_observed: bool,
    /// Exculpation (b): the damage would have occurred even with the required care.
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

impl AnimalLiabilityClaim {
    /// Whether this claim is subject to **strict** liability with no exculpation.
    ///
    /// True only for a luxury animal (Luxustier) held by the keeper under
    /// § 833 S. 1 BGB. The contractual supervisor (§ 834) always has the
    /// exculpation defence, regardless of animal category.
    #[must_use]
    pub fn is_strict_liability(&self) -> bool {
        matches!(self.basis, AnimalLiabilityBasis::Keeper)
            && matches!(self.animal_category, AnimalCategory::LuxuryAnimal)
    }
}

/// Builder for [`AnimalLiabilityClaim`] (§§ 833-834 BGB).
#[derive(Debug, Default)]
pub struct AnimalLiabilityClaimBuilder {
    liable_party: Option<TortParty>,
    injured_party: Option<TortParty>,
    animal_description: Option<String>,
    basis: Option<AnimalLiabilityBasis>,
    animal_category: Option<AnimalCategory>,
    harm_type: Option<PhysicalHarmType>,
    caused_by_animal: bool,
    required_care_observed: bool,
    damage_would_have_occurred_anyway: bool,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    personal_injury: Option<Capital>,
    pain_and_suffering: Option<Capital>,
    medical_expenses: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl AnimalLiabilityClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the liable party.
    pub fn liable_party(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.liable_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the injured party.
    pub fn injured_party(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.injured_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Describe the animal.
    pub fn animal(mut self, description: impl Into<String>) -> Self {
        self.animal_description = Some(description.into());
        self
    }

    /// Set the liability basis (keeper / supervisor).
    pub fn basis(mut self, basis: AnimalLiabilityBasis) -> Self {
        self.basis = Some(basis);
        self
    }

    /// Set the animal category.
    pub fn category(mut self, category: AnimalCategory) -> Self {
        self.animal_category = Some(category);
        self
    }

    /// Set the type of harm.
    pub fn harm_type(mut self, harm: PhysicalHarmType) -> Self {
        self.harm_type = Some(harm);
        self
    }

    /// Set whether the specific animal hazard caused the damage.
    pub fn caused_by_animal(mut self, value: bool) -> Self {
        self.caused_by_animal = value;
        self
    }

    /// Set exculpation (a): required care observed.
    pub fn required_care_observed(mut self, value: bool) -> Self {
        self.required_care_observed = value;
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

    /// Set pain and suffering (Schmerzensgeld).
    pub fn damages_pain_suffering(mut self, amount: Capital) -> Self {
        self.pain_and_suffering = Some(amount);
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
    pub fn build(self) -> std::result::Result<AnimalLiabilityClaim, String> {
        let liable_party = self.liable_party.ok_or("Liable party required")?;
        let injured_party = self.injured_party.ok_or("Injured party required")?;
        let animal_description = self
            .animal_description
            .ok_or("Animal description required")?;
        let basis = self.basis.ok_or("Liability basis required")?;
        let animal_category = self.animal_category.ok_or("Animal category required")?;
        let harm_type = self.harm_type.ok_or("Harm type required")?;
        let incident_date = self.incident_date.ok_or("Incident date required")?;

        let mut damages = DamageClaim {
            property_damage: self.property_damage,
            personal_injury: self.personal_injury,
            pain_and_suffering: self.pain_and_suffering,
            lost_income: None,
            medical_expenses: self.medical_expenses,
            consequential_damages: None,
            total: Capital { amount_cents: 0 },
        };
        damages.calculate_total();

        Ok(AnimalLiabilityClaim {
            liable_party,
            injured_party,
            animal_description,
            basis,
            animal_category,
            harm_type,
            caused_by_animal: self.caused_by_animal,
            required_care_observed: self.required_care_observed,
            damage_would_have_occurred_anyway: self.damage_would_have_occurred_anyway,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate an animal-liability claim under §§ 833-834 BGB.
///
/// For a luxury animal held by the keeper (§ 833 S. 1) liability is strict and no
/// exculpation is possible. For a utility animal (§ 833 S. 2) or a contractual
/// supervisor (§ 834) the defendant escapes liability by exculpation.
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// // Strict liability: a privately kept dog bites a passer-by.
/// let claim = AnimalLiabilityClaimBuilder::new()
///     .liable_party("Dog owner", "Hamburg")
///     .injured_party("Passer-by", "Hamburg")
///     .animal("Privately kept dog")
///     .basis(AnimalLiabilityBasis::Keeper)
///     .category(AnimalCategory::LuxuryAnimal)
///     .harm_type(PhysicalHarmType::HealthInjury)
///     .caused_by_animal(true)
///     .required_care_observed(true) // irrelevant for a luxury animal
///     .incident_date(Utc::now())
///     .damages_medical(Capital::from_euros(3_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(claim.is_strict_liability());
/// assert!(validate_animal_liability(&claim).is_ok());
/// ```
pub fn validate_animal_liability(claim: &AnimalLiabilityClaim) -> Result<()> {
    if claim.liable_party.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.injured_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement: the specific animal hazard (Tiergefahr) must have caused the damage.
    if !claim.caused_by_animal {
        return Err(TortError::NotCausedByAnimal);
    }

    // Exculpation is available for utility animals (§ 833 S. 2) and for the
    // contractual supervisor (§ 834). It is NOT available for a luxury animal
    // held by the keeper (§ 833 S. 1 - strict liability).
    if !claim.is_strict_liability() {
        if claim.required_care_observed {
            return Err(TortError::AnimalKeeperExculpated {
                ground: "im Verkehr erforderliche Sorgfalt beobachtet / required care observed"
                    .to_string(),
            });
        }
        if claim.damage_would_have_occurred_anyway {
            return Err(TortError::AnimalKeeperExculpated {
                ground: "Schaden auch bei erforderlicher Sorgfalt entstanden / damage would have \
                         occurred anyway"
                    .to_string(),
            });
        }
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

    fn luxury_keeper_claim() -> AnimalLiabilityClaim {
        AnimalLiabilityClaimBuilder::new()
            .liable_party("Dog owner", "Hamburg")
            .injured_party("Passer-by", "Hamburg")
            .animal("Privately kept dog")
            .basis(AnimalLiabilityBasis::Keeper)
            .category(AnimalCategory::LuxuryAnimal)
            .harm_type(PhysicalHarmType::HealthInjury)
            .caused_by_animal(true)
            .incident_date(Utc::now())
            .damages_medical(Capital::from_euros(3_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    fn utility_keeper_claim() -> AnimalLiabilityClaim {
        AnimalLiabilityClaimBuilder::new()
            .liable_party("Farmer", "Bavaria")
            .injured_party("Visitor", "Bavaria")
            .animal("Farm guard dog")
            .basis(AnimalLiabilityBasis::Keeper)
            .category(AnimalCategory::DomesticUtilityAnimal)
            .harm_type(PhysicalHarmType::BodilyInjury)
            .caused_by_animal(true)
            .incident_date(Utc::now())
            .damages_medical(Capital::from_euros(4_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_strict_liability_luxury_animal() {
        let claim = luxury_keeper_claim();
        assert!(claim.is_strict_liability());
        assert!(validate_animal_liability(&claim).is_ok());
    }

    #[test]
    fn test_luxury_animal_no_exculpation() {
        let mut claim = luxury_keeper_claim();
        // Even if care was observed, a luxury animal keeper remains strictly liable.
        claim.required_care_observed = true;
        claim.damage_would_have_occurred_anyway = true;
        assert!(validate_animal_liability(&claim).is_ok());
    }

    #[test]
    fn test_utility_animal_exculpation_care() {
        let mut claim = utility_keeper_claim();
        assert!(!claim.is_strict_liability());
        claim.required_care_observed = true;
        assert!(matches!(
            validate_animal_liability(&claim),
            Err(TortError::AnimalKeeperExculpated { .. })
        ));
    }

    #[test]
    fn test_utility_animal_no_exculpation_liable() {
        let claim = utility_keeper_claim();
        // No exculpation pleaded -> utility-animal keeper is liable.
        assert!(validate_animal_liability(&claim).is_ok());
    }

    #[test]
    fn test_supervisor_always_exculpable() {
        let mut claim = luxury_keeper_claim();
        claim.basis = AnimalLiabilityBasis::Supervisor;
        // Supervisor (§ 834) has the exculpation defence even for a luxury animal.
        assert!(!claim.is_strict_liability());
        claim.damage_would_have_occurred_anyway = true;
        assert!(matches!(
            validate_animal_liability(&claim),
            Err(TortError::AnimalKeeperExculpated { .. })
        ));
    }

    #[test]
    fn test_not_caused_by_animal() {
        let mut claim = luxury_keeper_claim();
        claim.caused_by_animal = false;
        assert!(matches!(
            validate_animal_liability(&claim),
            Err(TortError::NotCausedByAnimal)
        ));
    }

    #[test]
    fn test_no_causation() {
        let mut claim = luxury_keeper_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_animal_liability(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = luxury_keeper_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_animal_liability(&claim),
            Err(TortError::ZeroDamage)
        ));
    }

    #[test]
    fn test_section_835_repealed_marker() {
        assert!(SECTION_835_REPEALED.contains("aufgehoben"));
        assert!(SECTION_835_REPEALED.contains("BJagdG"));
    }
}
