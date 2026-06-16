//! BGB § 839 - Liability of Public Officials (Amtshaftung)
//!
//! ## German Text (§ 839 BGB)
//!
//! > (1) Verletzt ein Beamter vorsätzlich oder fahrlässig die ihm einem Dritten
//! > gegenüber obliegende Amtspflicht, so hat er dem Dritten den daraus entstehenden
//! > Schaden zu ersetzen. Fällt dem Beamten nur Fahrlässigkeit zur Last, so kann er
//! > nur dann in Anspruch genommen werden, wenn der Verletzte nicht auf andere Weise
//! > Ersatz zu erlangen vermag.
//! >
//! > (2) Verletzt ein Beamter bei dem Urteil in einer Rechtssache seine Amtspflicht,
//! > so ist er für den daraus entstehenden Schaden nur dann verantwortlich, wenn die
//! > Pflichtverletzung in einer Straftat besteht. Auf eine pflichtwidrige Verweigerung
//! > oder Verzögerung der Ausübung des Amts findet diese Vorschrift keine Anwendung.
//! >
//! > (3) Die Ersatzpflicht tritt nicht ein, wenn der Verletzte vorsätzlich oder
//! > fahrlässig unterlassen hat, den Schaden durch Gebrauch eines Rechtsmittels
//! > abzuwenden.
//!
//! ## English Translation
//!
//! > (1) If an official intentionally or negligently breaches the official duty
//! > incumbent on them in relation to a third party, the official must compensate the
//! > third party for the resulting damage. If only negligence is attributable to the
//! > official, the official may be held liable only if the injured party is unable to
//! > obtain compensation in another way.
//! >
//! > (2) If an official breaches official duty in delivering a judgment in a legal
//! > matter, the official is responsible for the resulting damage only if the breach
//! > of duty constitutes a criminal offence. This provision does not apply to a
//! > breach of duty consisting of a wrongful refusal or delay in exercising the
//! > office.
//! >
//! > (3) Liability does not arise if the injured party intentionally or negligently
//! > failed to avert the damage by use of a legal remedy.
//!
//! ## Legal Structure
//!
//! § 839 is the central provision of German State-liability law
//! (Staatshaftungsrecht). Under **Art. 34 GG** liability is, as a rule, transferred
//! from the individual official to the State (Haftungsüberleitung), but the
//! substantive requirements are those of § 839 BGB.
//!
//! **Requirements (Abs. 1 S. 1)**:
//! 1. **Official** in the liability sense (haftungsrechtlicher Beamtenbegriff,
//!    broader than civil-service status).
//! 2. Breach of an **official duty owed to a third party** (drittbezogene
//!    Amtspflicht).
//! 3. **Fault**: intent (Vorsatz) or negligence (Fahrlässigkeit).
//! 4. Causation and damage.
//!
//! **Limitations**:
//! - **Subsidiarity (Abs. 1 S. 2 - Verweisungsprivileg)**: for mere negligence,
//!   liability only if no other source of compensation is available.
//! - **Judicial privilege (Abs. 2 - Spruchrichterprivileg)**: for a breach in a
//!   judgment, liability only if the breach is a criminal offence; this does **not**
//!   apply to wrongful refusal/delay of office.
//! - **Failure to use a legal remedy (Abs. 3)**: no liability if the injured party
//!   culpably failed to avert the damage by a legal remedy.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, TortParty};
use crate::gmbhg::Capital;

/// Fault of the official (§ 839 Abs. 1 BGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfficialFault {
    /// Intent (Vorsatz).
    Intent,
    /// Negligence (Fahrlässigkeit) - triggers the subsidiarity rule of Abs. 1 S. 2.
    Negligence,
    /// No fault.
    None,
}

/// Claim under § 839 BGB for liability of a public official.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfficialLiabilityClaim {
    /// The official (Beamter im haftungsrechtlichen Sinne).
    pub official: TortParty,
    /// The injured third party (geschädigter Dritter).
    pub injured_party: TortParty,
    /// Whether the wrongdoer is an official in the liability sense.
    pub is_official_in_liability_sense: bool,
    /// Whether an official duty was breached (Amtspflichtverletzung).
    pub official_duty_breached: bool,
    /// Whether the duty was owed to the injured party (Drittbezogenheit).
    pub duty_owed_to_third_party: bool,
    /// Fault of the official.
    pub fault: OfficialFault,
    /// Whether the injured party can obtain compensation in another way (Abs. 1 S. 2).
    pub alternative_compensation_available: bool,
    /// Whether the breach occurred in a judicial decision (Urteil in einer Rechtssache, Abs. 2).
    pub judicial_decision_context: bool,
    /// Whether the breach of duty constitutes a criminal offence (Abs. 2).
    pub breach_constitutes_crime: bool,
    /// Whether the act was a wrongful refusal/delay of office (Abs. 2 S. 2 - privilege inapplicable).
    pub refusal_or_delay_of_office: bool,
    /// Whether the injured party culpably failed to use a legal remedy (Abs. 3).
    pub failed_to_use_legal_remedy: bool,
    /// Date of the incident.
    pub incident_date: DateTime<Utc>,
    /// Damages claimed.
    pub damages: DamageClaim,
    /// Causation established.
    pub causation_established: bool,
    /// Additional notes.
    pub notes: Option<String>,
}

/// Builder for [`OfficialLiabilityClaim`] (§ 839 BGB).
#[derive(Debug, Default)]
pub struct OfficialLiabilityClaimBuilder {
    official: Option<TortParty>,
    injured_party: Option<TortParty>,
    is_official_in_liability_sense: bool,
    official_duty_breached: bool,
    duty_owed_to_third_party: bool,
    fault: Option<OfficialFault>,
    alternative_compensation_available: bool,
    judicial_decision_context: bool,
    breach_constitutes_crime: bool,
    refusal_or_delay_of_office: bool,
    failed_to_use_legal_remedy: bool,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    lost_income: Option<Capital>,
    consequential_damages: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl OfficialLiabilityClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the official.
    pub fn official(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.official = Some(TortParty {
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

    /// Set whether the wrongdoer is an official in the liability sense.
    pub fn is_official(mut self, value: bool) -> Self {
        self.is_official_in_liability_sense = value;
        self
    }

    /// Set whether an official duty was breached.
    pub fn official_duty_breached(mut self, value: bool) -> Self {
        self.official_duty_breached = value;
        self
    }

    /// Set whether the duty was owed to the injured third party.
    pub fn duty_owed_to_third_party(mut self, value: bool) -> Self {
        self.duty_owed_to_third_party = value;
        self
    }

    /// Set the fault of the official.
    pub fn fault(mut self, fault: OfficialFault) -> Self {
        self.fault = Some(fault);
        self
    }

    /// Set whether alternative compensation is available (Abs. 1 S. 2).
    pub fn alternative_compensation_available(mut self, value: bool) -> Self {
        self.alternative_compensation_available = value;
        self
    }

    /// Configure the judicial context (Abs. 2).
    pub fn judicial_context(
        mut self,
        judicial_decision: bool,
        breach_is_crime: bool,
        refusal_or_delay: bool,
    ) -> Self {
        self.judicial_decision_context = judicial_decision;
        self.breach_constitutes_crime = breach_is_crime;
        self.refusal_or_delay_of_office = refusal_or_delay;
        self
    }

    /// Set whether the injured party failed to use a legal remedy (Abs. 3).
    pub fn failed_to_use_legal_remedy(mut self, value: bool) -> Self {
        self.failed_to_use_legal_remedy = value;
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

    /// Set lost income.
    pub fn damages_lost_income(mut self, amount: Capital) -> Self {
        self.lost_income = Some(amount);
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
    pub fn build(self) -> std::result::Result<OfficialLiabilityClaim, String> {
        let official = self.official.ok_or("Official required")?;
        let injured_party = self.injured_party.ok_or("Injured party required")?;
        let fault = self.fault.ok_or("Fault required")?;
        let incident_date = self.incident_date.ok_or("Incident date required")?;

        let mut damages = DamageClaim {
            property_damage: self.property_damage,
            personal_injury: None,
            pain_and_suffering: None,
            lost_income: self.lost_income,
            medical_expenses: None,
            consequential_damages: self.consequential_damages,
            total: Capital { amount_cents: 0 },
        };
        damages.calculate_total();

        Ok(OfficialLiabilityClaim {
            official,
            injured_party,
            is_official_in_liability_sense: self.is_official_in_liability_sense,
            official_duty_breached: self.official_duty_breached,
            duty_owed_to_third_party: self.duty_owed_to_third_party,
            fault,
            alternative_compensation_available: self.alternative_compensation_available,
            judicial_decision_context: self.judicial_decision_context,
            breach_constitutes_crime: self.breach_constitutes_crime,
            refusal_or_delay_of_office: self.refusal_or_delay_of_office,
            failed_to_use_legal_remedy: self.failed_to_use_legal_remedy,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate a claim under § 839 BGB.
///
/// Checks the basic requirements (official, drittbezogene Amtspflichtverletzung,
/// fault), then the limitations: judicial privilege (Abs. 2), subsidiarity for
/// negligence (Abs. 1 S. 2) and the failure-to-use-remedy bar (Abs. 3).
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = OfficialLiabilityClaimBuilder::new()
///     .official("Building inspector", "Dresden")
///     .injured_party("Property owner", "Dresden")
///     .is_official(true)
///     .official_duty_breached(true)
///     .duty_owed_to_third_party(true)
///     .fault(OfficialFault::Intent)
///     .incident_date(Utc::now())
///     .damages_property(Capital::from_euros(25_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(validate_official_liability(&claim).is_ok());
/// ```
pub fn validate_official_liability(claim: &OfficialLiabilityClaim) -> Result<()> {
    if claim.official.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.injured_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement 1: official in the liability sense.
    if !claim.is_official_in_liability_sense {
        return Err(TortError::NotAnOfficial);
    }

    // Requirement 2: breach of an official duty...
    if !claim.official_duty_breached {
        return Err(TortError::NoOfficialDutyBreach);
    }

    // ...owed to the injured third party (Drittbezogenheit).
    if !claim.duty_owed_to_third_party {
        return Err(TortError::NoDutyToThirdParty);
    }

    // Requirement 3: fault (Vorsatz or Fahrlässigkeit).
    if matches!(claim.fault, OfficialFault::None) {
        return Err(TortError::NoOfficialFault);
    }

    // Limitation: judicial privilege (Abs. 2 - Spruchrichterprivileg).
    // For a breach in a judgment, liability only if the breach is a criminal
    // offence; the privilege does not apply to wrongful refusal/delay of office.
    if claim.judicial_decision_context
        && !claim.refusal_or_delay_of_office
        && !claim.breach_constitutes_crime
    {
        return Err(TortError::JudicialPrivilege);
    }

    // Limitation: subsidiarity (Abs. 1 S. 2 - Verweisungsprivileg) for mere negligence.
    if matches!(claim.fault, OfficialFault::Negligence) && claim.alternative_compensation_available
    {
        return Err(TortError::OfficialLiabilitySubsidiary);
    }

    // Limitation: failure to use a legal remedy (Abs. 3).
    if claim.failed_to_use_legal_remedy {
        return Err(TortError::FailureToUseLegalRemedy);
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

    fn valid_intent_claim() -> OfficialLiabilityClaim {
        OfficialLiabilityClaimBuilder::new()
            .official("Building inspector", "Dresden")
            .injured_party("Property owner", "Dresden")
            .is_official(true)
            .official_duty_breached(true)
            .duty_owed_to_third_party(true)
            .fault(OfficialFault::Intent)
            .incident_date(Utc::now())
            .damages_property(Capital::from_euros(25_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_valid_intentional_breach() {
        let claim = valid_intent_claim();
        assert!(validate_official_liability(&claim).is_ok());
    }

    #[test]
    fn test_not_an_official() {
        let mut claim = valid_intent_claim();
        claim.is_official_in_liability_sense = false;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::NotAnOfficial)
        ));
    }

    #[test]
    fn test_no_duty_breach() {
        let mut claim = valid_intent_claim();
        claim.official_duty_breached = false;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::NoOfficialDutyBreach)
        ));
    }

    #[test]
    fn test_no_third_party_relation() {
        let mut claim = valid_intent_claim();
        claim.duty_owed_to_third_party = false;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::NoDutyToThirdParty)
        ));
    }

    #[test]
    fn test_no_fault() {
        let mut claim = valid_intent_claim();
        claim.fault = OfficialFault::None;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::NoOfficialFault)
        ));
    }

    #[test]
    fn test_subsidiarity_negligence_with_alternative() {
        let mut claim = valid_intent_claim();
        claim.fault = OfficialFault::Negligence;
        claim.alternative_compensation_available = true;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::OfficialLiabilitySubsidiary)
        ));
    }

    #[test]
    fn test_subsidiarity_does_not_apply_to_intent() {
        let mut claim = valid_intent_claim();
        // Intent + alternative compensation -> subsidiarity does NOT bar the claim.
        claim.alternative_compensation_available = true;
        assert!(validate_official_liability(&claim).is_ok());
    }

    #[test]
    fn test_negligence_without_alternative_liable() {
        let mut claim = valid_intent_claim();
        claim.fault = OfficialFault::Negligence;
        claim.alternative_compensation_available = false;
        assert!(validate_official_liability(&claim).is_ok());
    }

    #[test]
    fn test_judicial_privilege_no_crime() {
        let mut claim = valid_intent_claim();
        claim.judicial_decision_context = true;
        claim.breach_constitutes_crime = false;
        claim.refusal_or_delay_of_office = false;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::JudicialPrivilege)
        ));
    }

    #[test]
    fn test_judicial_breach_that_is_crime_liable() {
        let mut claim = valid_intent_claim();
        claim.judicial_decision_context = true;
        claim.breach_constitutes_crime = true;
        assert!(validate_official_liability(&claim).is_ok());
    }

    #[test]
    fn test_judicial_refusal_or_delay_not_privileged() {
        let mut claim = valid_intent_claim();
        claim.judicial_decision_context = true;
        claim.breach_constitutes_crime = false;
        claim.refusal_or_delay_of_office = true; // Abs. 2 S. 2 exception
        assert!(validate_official_liability(&claim).is_ok());
    }

    #[test]
    fn test_failure_to_use_legal_remedy() {
        let mut claim = valid_intent_claim();
        claim.failed_to_use_legal_remedy = true;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::FailureToUseLegalRemedy)
        ));
    }

    #[test]
    fn test_no_causation() {
        let mut claim = valid_intent_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = valid_intent_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_official_liability(&claim),
            Err(TortError::ZeroDamage)
        ));
    }
}
