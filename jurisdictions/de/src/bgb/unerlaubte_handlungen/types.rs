//! BGB Tort Law Types (Unerlaubte Handlungen)
//!
//! Type-safe representations of German tort law under the BGB
//! (Bürgerliches Gesetzbuch - German Civil Code, §§823-853).
//!
//! This module provides builder patterns for constructing tort claims with
//! comprehensive validation, similar to the Japanese minpo Article 709 implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

/// Protected interests under §823 Abs. 1 BGB
///
/// BGB §823 Abs. 1 enumerates specific protected interests (numerus clausus approach):
/// - Life (Leben)
/// - Body (Körper)
/// - Health (Gesundheit)
/// - Freedom (Freiheit)
/// - Property (Eigentum)
/// - Other rights (sonstiges Recht)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectedInterest {
    /// Life (Leben) - §823 Abs. 1 BGB
    Life,
    /// Body (Körper) - §823 Abs. 1 BGB
    Body,
    /// Health (Gesundheit) - §823 Abs. 1 BGB
    Health,
    /// Freedom (Freiheit) - §823 Abs. 1 BGB
    Freedom,
    /// Property (Eigentum) - §823 Abs. 1 BGB
    Property,
    /// Other rights (sonstiges Recht) - §823 Abs. 1 BGB
    /// Includes: personality rights, intellectual property, established business operations
    OtherRight,
}

/// Fault level (Verschulden) per §276 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verschulden {
    /// Intent (Vorsatz) - intentional wrongdoing
    Vorsatz,
    /// Gross negligence (grobe Fahrlässigkeit) - serious breach of duty of care
    GrobeFahrlassigkeit,
    /// Ordinary negligence (einfache Fahrlässigkeit) - failure to exercise reasonable care
    EinfacheFahrlassigkeit,
    /// No fault (kein Verschulden)
    KeinVerschulden,
}

/// Type of violation for §823 Abs. 1 claims
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// Direct physical injury (e.g., traffic accident, assault)
    DirectInjury {
        /// Type of injury
        injury_type: String,
        /// Severity description
        severity: String,
    },
    /// Property damage (e.g., destruction, theft)
    PropertyDamage {
        /// Description of property damaged
        property_description: String,
        /// Estimated value before damage
        original_value: Capital,
    },
    /// Violation of personality rights (Persönlichkeitsrecht)
    PersonalityRightsViolation {
        /// Type of violation (defamation, privacy, etc.)
        violation_type: String,
    },
    /// Violation of other rights (e.g., intellectual property)
    OtherRightsViolation {
        /// Description of right violated
        right_description: String,
    },
}

/// Party involved in a tort claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TortParty {
    /// Name of the party
    pub name: String,
    /// Address
    pub address: Option<String>,
    /// Whether party is natural person or legal entity
    pub is_natural_person: bool,
}

/// Tort claim under §823 Abs. 1 BGB with builder pattern
///
/// This structure allows fluent construction of tort claims with comprehensive
/// validation at each step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TortClaim823_1 {
    /// Tortfeasor (Schädiger)
    pub tortfeasor: TortParty,
    /// Injured party (Geschädigter)
    pub injured_party: TortParty,
    /// Protected interest violated
    pub protected_interest: ProtectedInterest,
    /// Type of violation
    pub violation: ViolationType,
    /// Fault level (Verschulden)
    pub verschulden: Verschulden,
    /// Whether unlawfulness exists (Widerrechtlichkeit)
    pub widerrechtlich: bool,
    /// Justification grounds (Rechtfertigungsgründe) if any
    pub justification: Option<Justification>,
    /// Date of incident
    pub incident_date: DateTime<Utc>,
    /// Damages claimed
    pub damages: DamageClaim,
    /// Causation established
    pub causation_established: bool,
    /// Additional notes
    pub notes: Option<String>,
}

/// Justification grounds (Rechtfertigungsgründe) that negate unlawfulness
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justification {
    /// Self-defense (Notwehr §32 StGB)
    SelfDefense,
    /// Necessity (Notstand §34 StGB, §228, §904 BGB)
    Necessity,
    /// Consent (Einwilligung)
    Consent,
    /// Authorized by law (Gesetzliche Befugnis)
    LegalAuthorization,
    /// Exercise of rights (Rechtausübung)
    ExerciseOfRights,
}

/// Damage claim with detailed breakdown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageClaim {
    /// Property damage (Sachschaden)
    pub property_damage: Option<Capital>,
    /// Personal injury compensation (Personenschaden)
    pub personal_injury: Option<Capital>,
    /// Pain and suffering (Schmerzensgeld §253 BGB)
    pub pain_and_suffering: Option<Capital>,
    /// Lost income (entgangener Gewinn §252 BGB)
    pub lost_income: Option<Capital>,
    /// Medical expenses (Heilungskosten)
    pub medical_expenses: Option<Capital>,
    /// Other consequential damages (Folgeschäden)
    pub consequential_damages: Option<Capital>,
    /// Total amount claimed
    pub total: Capital,
}

impl DamageClaim {
    /// Calculate total from components
    pub fn calculate_total(&mut self) {
        let mut total_cents = 0u64;

        if let Some(pd) = &self.property_damage {
            total_cents += pd.amount_cents;
        }
        if let Some(pi) = &self.personal_injury {
            total_cents += pi.amount_cents;
        }
        if let Some(pas) = &self.pain_and_suffering {
            total_cents += pas.amount_cents;
        }
        if let Some(li) = &self.lost_income {
            total_cents += li.amount_cents;
        }
        if let Some(me) = &self.medical_expenses {
            total_cents += me.amount_cents;
        }
        if let Some(cd) = &self.consequential_damages {
            total_cents += cd.amount_cents;
        }

        self.total = Capital {
            amount_cents: total_cents,
        };
    }
}

/// Builder for §823 Abs. 1 tort claims
///
/// Provides fluent API for constructing tort claims with validation.
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = TortClaim823_1Builder::new()
///     .tortfeasor("Max Mustermann", "Berlin")
///     .injured_party("Erika Schmidt", "Munich")
///     .protected_interest(ProtectedInterest::Body)
///     .violation_direct_injury("Broken arm", "Moderate")
///     .verschulden(Verschulden::EinfacheFahrlassigkeit)
///     .widerrechtlich(true)
///     .incident_date(Utc::now())
///     .damages_property(Capital::from_euros(5_000))
///     .damages_medical(Capital::from_euros(3_000))
///     .causation_established(true)
///     .build();
///
/// assert!(claim.is_ok());
/// ```
#[derive(Debug, Default)]
pub struct TortClaim823_1Builder {
    tortfeasor: Option<TortParty>,
    injured_party: Option<TortParty>,
    protected_interest: Option<ProtectedInterest>,
    violation: Option<ViolationType>,
    verschulden: Option<Verschulden>,
    widerrechtlich: Option<bool>,
    justification: Option<Justification>,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    personal_injury: Option<Capital>,
    pain_and_suffering: Option<Capital>,
    lost_income: Option<Capital>,
    medical_expenses: Option<Capital>,
    consequential_damages: Option<Capital>,
    causation_established: Option<bool>,
    notes: Option<String>,
}

impl TortClaim823_1Builder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set tortfeasor (Schädiger)
    pub fn tortfeasor(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.tortfeasor = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set tortfeasor as legal entity
    pub fn tortfeasor_legal_entity(
        mut self,
        name: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        self.tortfeasor = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: false,
        });
        self
    }

    /// Set injured party (Geschädigter)
    pub fn injured_party(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.injured_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set injured party as legal entity
    pub fn injured_party_legal_entity(
        mut self,
        name: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        self.injured_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: false,
        });
        self
    }

    /// Set protected interest violated
    pub fn protected_interest(mut self, interest: ProtectedInterest) -> Self {
        self.protected_interest = Some(interest);
        self
    }

    /// Set violation as direct injury
    pub fn violation_direct_injury(
        mut self,
        injury_type: impl Into<String>,
        severity: impl Into<String>,
    ) -> Self {
        self.violation = Some(ViolationType::DirectInjury {
            injury_type: injury_type.into(),
            severity: severity.into(),
        });
        self
    }

    /// Set violation as property damage
    pub fn violation_property_damage(
        mut self,
        description: impl Into<String>,
        original_value: Capital,
    ) -> Self {
        self.violation = Some(ViolationType::PropertyDamage {
            property_description: description.into(),
            original_value,
        });
        self
    }

    /// Set violation as personality rights violation
    pub fn violation_personality_rights(mut self, violation_type: impl Into<String>) -> Self {
        self.violation = Some(ViolationType::PersonalityRightsViolation {
            violation_type: violation_type.into(),
        });
        self
    }

    /// Set violation as other rights violation
    pub fn violation_other_rights(mut self, right_description: impl Into<String>) -> Self {
        self.violation = Some(ViolationType::OtherRightsViolation {
            right_description: right_description.into(),
        });
        self
    }

    /// Set fault level (Verschulden)
    pub fn verschulden(mut self, level: Verschulden) -> Self {
        self.verschulden = Some(level);
        self
    }

    /// Set unlawfulness (Widerrechtlichkeit)
    pub fn widerrechtlich(mut self, is_unlawful: bool) -> Self {
        self.widerrechtlich = Some(is_unlawful);
        self
    }

    /// Set justification grounds
    pub fn justification(mut self, grounds: Justification) -> Self {
        self.justification = Some(grounds);
        self
    }

    /// Set incident date
    pub fn incident_date(mut self, date: DateTime<Utc>) -> Self {
        self.incident_date = Some(date);
        self
    }

    /// Set property damage amount
    pub fn damages_property(mut self, amount: Capital) -> Self {
        self.property_damage = Some(amount);
        self
    }

    /// Set personal injury compensation
    pub fn damages_personal_injury(mut self, amount: Capital) -> Self {
        self.personal_injury = Some(amount);
        self
    }

    /// Set pain and suffering (Schmerzensgeld)
    pub fn damages_pain_suffering(mut self, amount: Capital) -> Self {
        self.pain_and_suffering = Some(amount);
        self
    }

    /// Set lost income (entgangener Gewinn)
    pub fn damages_lost_income(mut self, amount: Capital) -> Self {
        self.lost_income = Some(amount);
        self
    }

    /// Set medical expenses
    pub fn damages_medical(mut self, amount: Capital) -> Self {
        self.medical_expenses = Some(amount);
        self
    }

    /// Set consequential damages
    pub fn damages_consequential(mut self, amount: Capital) -> Self {
        self.consequential_damages = Some(amount);
        self
    }

    /// Set whether causation is established
    pub fn causation_established(mut self, established: bool) -> Self {
        self.causation_established = Some(established);
        self
    }

    /// Add notes to the claim
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Build the tort claim
    ///
    /// Returns error if required fields are missing.
    pub fn build(self) -> Result<TortClaim823_1, String> {
        let tortfeasor = self.tortfeasor.ok_or("Tortfeasor (Schädiger) required")?;
        let injured_party = self
            .injured_party
            .ok_or("Injured party (Geschädigter) required")?;
        let protected_interest = self
            .protected_interest
            .ok_or("Protected interest required")?;
        let violation = self.violation.ok_or("Violation type required")?;
        let verschulden = self.verschulden.ok_or("Verschulden (fault) required")?;
        let widerrechtlich = self
            .widerrechtlich
            .ok_or("Widerrechtlichkeit (unlawfulness) required")?;
        let incident_date = self.incident_date.ok_or("Incident date required")?;
        let causation_established = self.causation_established.unwrap_or(false);

        let mut damages = DamageClaim {
            property_damage: self.property_damage,
            personal_injury: self.personal_injury,
            pain_and_suffering: self.pain_and_suffering,
            lost_income: self.lost_income,
            medical_expenses: self.medical_expenses,
            consequential_damages: self.consequential_damages,
            total: Capital { amount_cents: 0 },
        };

        damages.calculate_total();

        Ok(TortClaim823_1 {
            tortfeasor,
            injured_party,
            protected_interest,
            violation,
            verschulden,
            widerrechtlich,
            justification: self.justification,
            incident_date,
            damages,
            causation_established,
            notes: self.notes,
        })
    }
}

/// Tort claim under §826 BGB (Intentional damage contrary to public policy)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TortClaim826 {
    /// Tortfeasor (Schädiger)
    pub tortfeasor: TortParty,
    /// Injured party (Geschädigter)
    pub injured_party: TortParty,
    /// Conduct description
    pub conduct: String,
    /// Whether conduct is contrary to good morals (sittenwidrig)
    pub sittenwidrig: bool,
    /// Intent to cause damage (Schädigungsvorsatz)
    pub schadigungsvorsatz: bool,
    /// Date of incident
    pub incident_date: DateTime<Utc>,
    /// Damages claimed
    pub damages: DamageClaim,
    /// Causation established
    pub causation_established: bool,
    /// Additional notes
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_interest_enum() {
        let life = ProtectedInterest::Life;
        let body = ProtectedInterest::Body;
        let health = ProtectedInterest::Health;

        assert_ne!(life, body);
        assert_ne!(body, health);
    }

    #[test]
    fn test_verschulden_enum() {
        let intent = Verschulden::Vorsatz;
        let gross = Verschulden::GrobeFahrlassigkeit;
        let ordinary = Verschulden::EinfacheFahrlassigkeit;

        assert_ne!(intent, gross);
        assert_ne!(gross, ordinary);
    }

    #[test]
    fn test_damage_claim_calculation() {
        let mut damages = DamageClaim {
            property_damage: Some(Capital::from_euros(5_000)),
            personal_injury: Some(Capital::from_euros(10_000)),
            pain_and_suffering: Some(Capital::from_euros(3_000)),
            lost_income: Some(Capital::from_euros(2_000)),
            medical_expenses: Some(Capital::from_euros(1_500)),
            consequential_damages: None,
            total: Capital { amount_cents: 0 },
        };

        damages.calculate_total();

        assert_eq!(damages.total.to_euros(), 21_500.0);
    }

    #[test]
    fn test_builder_complete_claim() {
        let claim = TortClaim823_1Builder::new()
            .tortfeasor("Max Mustermann", "Berlin")
            .injured_party("Erika Schmidt", "Munich")
            .protected_interest(ProtectedInterest::Body)
            .violation_direct_injury("Broken arm", "Moderate")
            .verschulden(Verschulden::EinfacheFahrlassigkeit)
            .widerrechtlich(true)
            .incident_date(Utc::now())
            .damages_property(Capital::from_euros(1_000))
            .damages_medical(Capital::from_euros(2_000))
            .causation_established(true)
            .build();

        assert!(claim.is_ok());
        let claim = claim.unwrap();
        assert_eq!(claim.tortfeasor.name, "Max Mustermann");
        assert_eq!(claim.injured_party.name, "Erika Schmidt");
        assert_eq!(claim.protected_interest, ProtectedInterest::Body);
        assert!(claim.causation_established);
        assert_eq!(claim.damages.total.to_euros(), 3_000.0);
    }

    #[test]
    fn test_builder_missing_required_field() {
        let claim = TortClaim823_1Builder::new()
            .tortfeasor("Max Mustermann", "Berlin")
            .protected_interest(ProtectedInterest::Property)
            // Missing injured_party
            .build();

        assert!(claim.is_err());
        assert!(claim.unwrap_err().contains("Injured party"));
    }

    #[test]
    fn test_tort_party_natural_person() {
        let party = TortParty {
            name: "Test Person".to_string(),
            address: Some("Test Address".to_string()),
            is_natural_person: true,
        };

        assert!(party.is_natural_person);
        assert_eq!(party.name, "Test Person");
    }

    #[test]
    fn test_tort_party_legal_entity() {
        let party = TortParty {
            name: "Test GmbH".to_string(),
            address: Some("Berlin".to_string()),
            is_natural_person: false,
        };

        assert!(!party.is_natural_person);
    }
}
