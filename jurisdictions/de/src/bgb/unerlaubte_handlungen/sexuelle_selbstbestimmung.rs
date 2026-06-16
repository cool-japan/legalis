//! BGB § 825 - Inducement to Sexual Acts (Bestimmung zu sexuellen Handlungen)
//!
//! ## German Text (§ 825 BGB)
//!
//! > Wer einen anderen durch Hinterlist, Drohung oder Missbrauch eines
//! > Abhängigkeitsverhältnisses zur Vornahme oder Duldung sexueller Handlungen
//! > bestimmt, ist ihm zum Ersatz des daraus entstehenden Schadens verpflichtet.
//!
//! ## English Translation
//!
//! > A person who induces another to undertake or tolerate sexual acts by
//! > deception, threat or abuse of a relationship of dependence is liable to that
//! > other person for the resulting damage.
//!
//! ## Legal Structure
//!
//! § 825 protects sexual self-determination (sexuelle Selbstbestimmung). It was
//! modernised in 2002 to a gender-neutral, technology-neutral wording. It is a
//! tort of its own that frequently overlaps with § 823 Abs. 1 (Körper/Gesundheit)
//! and § 823 Abs. 2 in conjunction with §§ 174 ff. StGB.
//!
//! **Requirements**:
//! 1. Inducement (Bestimmen) of another to **undertake** (Vornahme) or **tolerate**
//!    (Duldung) sexual acts (sexuelle Handlungen).
//! 2. By a **qualifying means**:
//!    - Deception (Hinterlist), or
//!    - Threat (Drohung), or
//!    - Abuse of a relationship of dependence (Missbrauch eines Abhängigkeitsverhältnisses).
//! 3. Causation and damage.
//!
//! Because § 825 protects a personal (non-pecuniary) interest, compensation for
//! pain and suffering (Schmerzensgeld, § 253 Abs. 2 BGB) is available.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, TortParty};
use crate::gmbhg::Capital;

/// Qualifying means of inducement under § 825 BGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InducementMeans {
    /// Deception / cunning (Hinterlist).
    Deception,
    /// Threat (Drohung).
    Threat,
    /// Abuse of a relationship of dependence (Missbrauch eines Abhängigkeitsverhältnisses).
    AbuseOfDependence,
}

/// Whether the victim was induced to undertake or to tolerate the act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SexualActInvolvement {
    /// Undertaking the act (Vornahme).
    Undertaking,
    /// Tolerating the act (Duldung).
    Toleration,
}

/// Claim under § 825 BGB for inducement to sexual acts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SexualSelfDeterminationClaim {
    /// Tortfeasor (Schädiger).
    pub tortfeasor: TortParty,
    /// Injured party (Geschädigter).
    pub injured_party: TortParty,
    /// Whether the inducement (Bestimmen) is established.
    pub inducement_established: bool,
    /// The qualifying means used, if any. `None` means no qualifying means.
    pub means: Option<InducementMeans>,
    /// Undertaking vs toleration of the act.
    pub act_involvement: SexualActInvolvement,
    /// Date of the incident.
    pub incident_date: DateTime<Utc>,
    /// Damages claimed (may include Schmerzensgeld, § 253 Abs. 2 BGB).
    pub damages: DamageClaim,
    /// Causation established.
    pub causation_established: bool,
    /// Additional notes.
    pub notes: Option<String>,
}

/// Builder for [`SexualSelfDeterminationClaim`] (§ 825 BGB).
#[derive(Debug, Default)]
pub struct SexualSelfDeterminationClaimBuilder {
    tortfeasor: Option<TortParty>,
    injured_party: Option<TortParty>,
    inducement_established: bool,
    means: Option<InducementMeans>,
    act_involvement: Option<SexualActInvolvement>,
    incident_date: Option<DateTime<Utc>>,
    pain_and_suffering: Option<Capital>,
    medical_expenses: Option<Capital>,
    consequential_damages: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl SexualSelfDeterminationClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tortfeasor.
    pub fn tortfeasor(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.tortfeasor = Some(TortParty {
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

    /// Mark whether inducement (Bestimmen) is established.
    pub fn inducement(mut self, established: bool) -> Self {
        self.inducement_established = established;
        self
    }

    /// Set the qualifying means used.
    pub fn means(mut self, means: InducementMeans) -> Self {
        self.means = Some(means);
        self
    }

    /// Set the form of involvement (undertaking/toleration).
    pub fn act_involvement(mut self, involvement: SexualActInvolvement) -> Self {
        self.act_involvement = Some(involvement);
        self
    }

    /// Set the incident date.
    pub fn incident_date(mut self, date: DateTime<Utc>) -> Self {
        self.incident_date = Some(date);
        self
    }

    /// Set pain and suffering (Schmerzensgeld, § 253 Abs. 2 BGB).
    pub fn damages_pain_suffering(mut self, amount: Capital) -> Self {
        self.pain_and_suffering = Some(amount);
        self
    }

    /// Set medical expenses.
    pub fn damages_medical(mut self, amount: Capital) -> Self {
        self.medical_expenses = Some(amount);
        self
    }

    /// Set consequential damages.
    pub fn damages_consequential(mut self, amount: Capital) -> Self {
        self.consequential_damages = Some(amount);
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
    pub fn build(self) -> std::result::Result<SexualSelfDeterminationClaim, String> {
        let tortfeasor = self.tortfeasor.ok_or("Tortfeasor required")?;
        let injured_party = self.injured_party.ok_or("Injured party required")?;
        let act_involvement = self.act_involvement.ok_or("Act involvement required")?;
        let incident_date = self.incident_date.ok_or("Incident date required")?;

        let mut damages = DamageClaim {
            property_damage: None,
            personal_injury: None,
            pain_and_suffering: self.pain_and_suffering,
            lost_income: None,
            medical_expenses: self.medical_expenses,
            consequential_damages: self.consequential_damages,
            total: Capital { amount_cents: 0 },
        };
        damages.calculate_total();

        Ok(SexualSelfDeterminationClaim {
            tortfeasor,
            injured_party,
            inducement_established: self.inducement_established,
            means: self.means,
            act_involvement,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate a claim under § 825 BGB.
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = SexualSelfDeterminationClaimBuilder::new()
///     .tortfeasor("A", "Berlin")
///     .injured_party("B", "Berlin")
///     .inducement(true)
///     .means(InducementMeans::AbuseOfDependence)
///     .act_involvement(SexualActInvolvement::Toleration)
///     .incident_date(Utc::now())
///     .damages_pain_suffering(Capital::from_euros(15_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(validate_sexual_self_determination_claim(&claim).is_ok());
/// ```
pub fn validate_sexual_self_determination_claim(
    claim: &SexualSelfDeterminationClaim,
) -> Result<()> {
    if claim.tortfeasor.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.injured_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement: inducement (Bestimmen) must be established.
    if !claim.inducement_established {
        return Err(TortError::NoSexualActInducement);
    }

    // Requirement: a qualifying means (Hinterlist, Drohung, Abhängigkeit) is mandatory.
    if claim.means.is_none() {
        return Err(TortError::NoQualifyingInducementMeans);
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

    fn valid_claim() -> SexualSelfDeterminationClaim {
        SexualSelfDeterminationClaimBuilder::new()
            .tortfeasor("A", "Berlin")
            .injured_party("B", "Berlin")
            .inducement(true)
            .means(InducementMeans::Threat)
            .act_involvement(SexualActInvolvement::Toleration)
            .incident_date(Utc::now())
            .damages_pain_suffering(Capital::from_euros(15_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_valid_claim() {
        let claim = valid_claim();
        assert!(validate_sexual_self_determination_claim(&claim).is_ok());
    }

    #[test]
    fn test_no_inducement() {
        let mut claim = valid_claim();
        claim.inducement_established = false;
        assert!(matches!(
            validate_sexual_self_determination_claim(&claim),
            Err(TortError::NoSexualActInducement)
        ));
    }

    #[test]
    fn test_no_qualifying_means() {
        let mut claim = valid_claim();
        claim.means = None;
        assert!(matches!(
            validate_sexual_self_determination_claim(&claim),
            Err(TortError::NoQualifyingInducementMeans)
        ));
    }

    #[test]
    fn test_deception_means_actionable() {
        let mut claim = valid_claim();
        claim.means = Some(InducementMeans::Deception);
        assert!(validate_sexual_self_determination_claim(&claim).is_ok());
    }

    #[test]
    fn test_abuse_of_dependence_undertaking() {
        let mut claim = valid_claim();
        claim.means = Some(InducementMeans::AbuseOfDependence);
        claim.act_involvement = SexualActInvolvement::Undertaking;
        assert!(validate_sexual_self_determination_claim(&claim).is_ok());
    }

    #[test]
    fn test_no_causation() {
        let mut claim = valid_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_sexual_self_determination_claim(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = valid_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_sexual_self_determination_claim(&claim),
            Err(TortError::ZeroDamage)
        ));
    }

    #[test]
    fn test_missing_tortfeasor() {
        let mut claim = valid_claim();
        claim.tortfeasor.name = String::new();
        assert!(matches!(
            validate_sexual_self_determination_claim(&claim),
            Err(TortError::TortfeasorMissing)
        ));
    }
}
