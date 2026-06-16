//! BGB §§ 836-838 - Liability for Buildings and Structures (Gebäudehaftung)
//!
//! ## § 836 BGB - Liability of the Land Possessor (Haftung des Grundstücksbesitzers)
//!
//! > (1) Wird durch den Einsturz eines Gebäudes oder eines anderen mit einem
//! > Grundstück verbundenen Werkes oder durch die Ablösung von Teilen des Gebäudes
//! > oder des Werkes ein Mensch getötet, der Körper oder die Gesundheit eines
//! > Menschen verletzt oder eine Sache beschädigt, so ist der Besitzer des
//! > Grundstücks, sofern der Einsturz oder die Ablösung die Folge fehlerhafter
//! > Errichtung oder mangelhafter Unterhaltung ist, verpflichtet, dem Verletzten
//! > den daraus entstehenden Schaden zu ersetzen. Die Ersatzpflicht tritt nicht
//! > ein, wenn der Besitzer zum Zwecke der Abwendung der Gefahr die im Verkehr
//! > erforderliche Sorgfalt beobachtet hat.
//!
//! **English**: If the collapse of a building or other structure attached to land,
//! or the detachment of parts of such a building or structure, kills a person,
//! injures the body or health of a person, or damages a thing, the **possessor of
//! the land** is liable for the resulting damage **insofar as the collapse or
//! detachment results from faulty construction or defective maintenance**.
//! Liability does not arise if the possessor observed the care required in dealings
//! for the purpose of averting the danger.
//!
//! ## § 837 BGB - Liability of the Building Possessor (Gebäudebesitzer)
//!
//! Where a person possesses a building or structure on another's land in the
//! exercise of a right, that person (instead of the land possessor) bears the
//! § 836 liability.
//!
//! ## § 838 BGB - Liability of the Maintenance Obligor (Gebäudeunterhaltungspflichtiger)
//!
//! A person who assumes the maintenance of a building for the possessor, or who has
//! to maintain it by virtue of a right of use, is liable like the possessor under
//! § 836.
//!
//! ## Legal Structure
//!
//! All three provisions establish **presumed-fault liability** (Haftung für
//! vermutetes Verschulden / Beweislastumkehr): once collapse/detachment due to
//! faulty construction or defective maintenance is shown, the defendant is liable
//! unless able to **exculpate** by proving the required care was observed.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, PhysicalHarmType, TortParty};
use crate::gmbhg::Capital;

/// Which provision (§§ 836-838) the liable party is being held under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingLiableParty {
    /// Possessor of the land (Grundstücksbesitzer, § 836 BGB).
    LandPossessor,
    /// Possessor of a building on another's land (Gebäudebesitzer, § 837 BGB).
    BuildingPossessor,
    /// Person obliged to maintain the building (Unterhaltungspflichtiger, § 838 BGB).
    MaintenanceObligor,
}

/// The structural event that caused the harm (§ 836 Abs. 1 S. 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralFailureType {
    /// Collapse of the building or structure (Einsturz).
    Collapse,
    /// Detachment of parts of the building or structure (Ablösung von Teilen).
    DetachmentOfParts,
}

/// The cause of the structural failure (§ 836 Abs. 1 S. 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralDefectCause {
    /// Faulty construction (fehlerhafte Errichtung).
    FaultyConstruction,
    /// Defective maintenance (mangelhafte Unterhaltung).
    DefectiveMaintenance,
    /// Neither - e.g. force majeure, third-party act (no § 836 liability).
    Other,
}

/// Claim under §§ 836-838 BGB for damage from a building or structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingLiabilityClaim {
    /// The liable party (Besitzer / Unterhaltungspflichtiger).
    pub liable_party: TortParty,
    /// The role under which liability is asserted (§ 836 / § 837 / § 838).
    pub liable_party_role: BuildingLiableParty,
    /// The injured party (Verletzter).
    pub injured_party: TortParty,
    /// Description of the building or structure.
    pub structure_description: String,
    /// The structural failure (collapse / detachment).
    pub failure_type: StructuralFailureType,
    /// The cause of the failure.
    pub defect_cause: StructuralDefectCause,
    /// The type of harm caused.
    pub harm_type: PhysicalHarmType,
    /// Exculpation: the required care to avert the danger was observed.
    pub required_care_observed: bool,
    /// Date of the incident.
    pub incident_date: DateTime<Utc>,
    /// Damages claimed.
    pub damages: DamageClaim,
    /// Causation established.
    pub causation_established: bool,
    /// Additional notes.
    pub notes: Option<String>,
}

/// Builder for [`BuildingLiabilityClaim`] (§§ 836-838 BGB).
#[derive(Debug, Default)]
pub struct BuildingLiabilityClaimBuilder {
    liable_party: Option<TortParty>,
    liable_party_role: Option<BuildingLiableParty>,
    injured_party: Option<TortParty>,
    structure_description: Option<String>,
    failure_type: Option<StructuralFailureType>,
    defect_cause: Option<StructuralDefectCause>,
    harm_type: Option<PhysicalHarmType>,
    required_care_observed: bool,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    personal_injury: Option<Capital>,
    medical_expenses: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl BuildingLiabilityClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the liable party and the provision under which liability is asserted.
    pub fn liable_party(
        mut self,
        name: impl Into<String>,
        address: impl Into<String>,
        role: BuildingLiableParty,
    ) -> Self {
        self.liable_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self.liable_party_role = Some(role);
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

    /// Describe the building or structure.
    pub fn structure(mut self, description: impl Into<String>) -> Self {
        self.structure_description = Some(description.into());
        self
    }

    /// Set the structural failure type.
    pub fn failure_type(mut self, failure: StructuralFailureType) -> Self {
        self.failure_type = Some(failure);
        self
    }

    /// Set the cause of the failure.
    pub fn defect_cause(mut self, cause: StructuralDefectCause) -> Self {
        self.defect_cause = Some(cause);
        self
    }

    /// Set the type of harm.
    pub fn harm_type(mut self, harm: PhysicalHarmType) -> Self {
        self.harm_type = Some(harm);
        self
    }

    /// Set whether the required care to avert the danger was observed (exculpation).
    pub fn required_care_observed(mut self, value: bool) -> Self {
        self.required_care_observed = value;
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
    pub fn build(self) -> std::result::Result<BuildingLiabilityClaim, String> {
        let liable_party = self.liable_party.ok_or("Liable party required")?;
        let liable_party_role = self.liable_party_role.ok_or("Liable party role required")?;
        let injured_party = self.injured_party.ok_or("Injured party required")?;
        let structure_description = self
            .structure_description
            .ok_or("Structure description required")?;
        let failure_type = self.failure_type.ok_or("Failure type required")?;
        let defect_cause = self.defect_cause.ok_or("Defect cause required")?;
        let harm_type = self.harm_type.ok_or("Harm type required")?;
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

        Ok(BuildingLiabilityClaim {
            liable_party,
            liable_party_role,
            injured_party,
            structure_description,
            failure_type,
            defect_cause,
            harm_type,
            required_care_observed: self.required_care_observed,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate a building-liability claim under §§ 836-838 BGB.
///
/// Liability requires that the collapse or detachment resulted from faulty
/// construction or defective maintenance. The defendant escapes liability by
/// exculpation (required care to avert the danger was observed).
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = BuildingLiabilityClaimBuilder::new()
///     .liable_party("Land owner", "Cologne", BuildingLiableParty::LandPossessor)
///     .injured_party("Pedestrian", "Cologne")
///     .structure("Old facade")
///     .failure_type(StructuralFailureType::DetachmentOfParts)
///     .defect_cause(StructuralDefectCause::DefectiveMaintenance)
///     .harm_type(PhysicalHarmType::BodilyInjury)
///     .incident_date(Utc::now())
///     .damages_medical(Capital::from_euros(6_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(validate_building_liability(&claim).is_ok());
/// ```
pub fn validate_building_liability(claim: &BuildingLiabilityClaim) -> Result<()> {
    if claim.liable_party.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.injured_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement: collapse/detachment must result from faulty construction or
    // defective maintenance (§ 836 Abs. 1 S. 1).
    if matches!(claim.defect_cause, StructuralDefectCause::Other) {
        return Err(TortError::NoStructuralDefectCausation);
    }

    // Exculpation (§ 836 Abs. 1 S. 2, applicable via §§ 837, 838).
    if claim.required_care_observed {
        return Err(TortError::BuildingPossessorExculpated {
            ground: "im Verkehr erforderliche Sorgfalt zur Gefahrabwendung beobachtet / required \
                     care to avert the danger observed"
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

    fn valid_claim() -> BuildingLiabilityClaim {
        BuildingLiabilityClaimBuilder::new()
            .liable_party("Land owner", "Cologne", BuildingLiableParty::LandPossessor)
            .injured_party("Pedestrian", "Cologne")
            .structure("Old facade")
            .failure_type(StructuralFailureType::DetachmentOfParts)
            .defect_cause(StructuralDefectCause::DefectiveMaintenance)
            .harm_type(PhysicalHarmType::BodilyInjury)
            .incident_date(Utc::now())
            .damages_medical(Capital::from_euros(6_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_valid_land_possessor_836() {
        let claim = valid_claim();
        assert!(validate_building_liability(&claim).is_ok());
    }

    #[test]
    fn test_building_possessor_837() {
        let mut claim = valid_claim();
        claim.liable_party_role = BuildingLiableParty::BuildingPossessor;
        claim.failure_type = StructuralFailureType::Collapse;
        claim.defect_cause = StructuralDefectCause::FaultyConstruction;
        assert!(validate_building_liability(&claim).is_ok());
    }

    #[test]
    fn test_maintenance_obligor_838() {
        let mut claim = valid_claim();
        claim.liable_party_role = BuildingLiableParty::MaintenanceObligor;
        assert!(validate_building_liability(&claim).is_ok());
    }

    #[test]
    fn test_other_cause_no_liability() {
        let mut claim = valid_claim();
        claim.defect_cause = StructuralDefectCause::Other;
        assert!(matches!(
            validate_building_liability(&claim),
            Err(TortError::NoStructuralDefectCausation)
        ));
    }

    #[test]
    fn test_exculpation_required_care() {
        let mut claim = valid_claim();
        claim.required_care_observed = true;
        assert!(matches!(
            validate_building_liability(&claim),
            Err(TortError::BuildingPossessorExculpated { .. })
        ));
    }

    #[test]
    fn test_no_causation() {
        let mut claim = valid_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_building_liability(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = valid_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_building_liability(&claim),
            Err(TortError::ZeroDamage)
        ));
    }

    #[test]
    fn test_missing_liable_party() {
        let mut claim = valid_claim();
        claim.liable_party.name = String::new();
        assert!(matches!(
            validate_building_liability(&claim),
            Err(TortError::TortfeasorMissing)
        ));
    }
}
