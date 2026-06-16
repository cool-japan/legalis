//! BGB § 824 - Credit Endangerment (Kreditgefährdung)
//!
//! ## German Text (§ 824 BGB)
//!
//! > (1) Wer der Wahrheit zuwider eine Tatsache behauptet oder verbreitet, die
//! > geeignet ist, den Kredit eines anderen zu gefährden oder sonstige Nachteile
//! > für dessen Erwerb oder Fortkommen herbeizuführen, hat dem anderen den daraus
//! > entstehenden Schaden auch dann zu ersetzen, wenn er die Unwahrheit zwar nicht
//! > kennt, aber kennen muss.
//! >
//! > (2) Durch eine Mitteilung, deren Unwahrheit dem Mitteilenden unbekannt ist,
//! > wird dieser nicht zum Schadensersatz verpflichtet, wenn er oder der Empfänger
//! > der Mitteilung an ihr ein berechtigtes Interesse hat.
//!
//! ## English Translation
//!
//! > (1) A person who, contrary to the truth, asserts or disseminates a fact that
//! > is suitable to endanger the credit of another or to cause other disadvantages
//! > for that person's earnings or advancement is liable to the other for the
//! > resulting damage even if the person does not know that the fact is untrue but
//! > should know.
//! >
//! > (2) A communication whose untruth is unknown to the communicating party does
//! > not oblige that party to pay damages if the party or the recipient of the
//! > communication has a legitimate interest in it.
//!
//! ## Legal Structure
//!
//! § 824 protects economic reputation (Kredit/Erwerb/Fortkommen) against the
//! assertion or dissemination of **untrue facts**. Unlike § 823 Abs. 1, the
//! protected interest is purely economic; unlike § 826, mere negligence
//! ("kennen muss") suffices.
//!
//! **Requirements (Abs. 1)**:
//! 1. Assertion (Behaupten) or dissemination (Verbreiten) of a **fact**
//!    (Tatsachenbehauptung) - value judgments (Werturteile, Art. 5 GG) are not covered.
//! 2. The fact is **untrue** ("der Wahrheit zuwider").
//! 3. The fact is **suitable** to endanger credit or cause disadvantages for the
//!    affected party's earnings or advancement.
//! 4. At least **negligent ignorance** of the untruth ("kennt oder kennen muss").
//! 5. Causation and damage.
//!
//! **Privilege (Abs. 2 - Wahrnehmung berechtigter Interessen)**:
//! If the communicating party did **not positively know** of the untruth and the
//! party or the recipient has a **legitimate interest** in the communication, no
//! liability arises (e.g. credit reference agencies, employment references).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::{Result, TortError};
use super::types::{DamageClaim, TortParty};
use crate::gmbhg::Capital;

/// Form of the statement under § 824 Abs. 1 BGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementForm {
    /// Asserting a fact as one's own (Behaupten).
    Assertion,
    /// Disseminating a fact stated by another (Verbreiten).
    Dissemination,
}

/// Nature of the statement: only assertions of fact are actionable under § 824.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementNature {
    /// Assertion of a verifiable fact (Tatsachenbehauptung) - covered by § 824.
    FactualAssertion,
    /// Value judgment / opinion (Werturteil) - not covered by § 824, see Art. 5 GG.
    ValueJudgment,
}

/// Claim for credit endangerment under § 824 BGB.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditEndangermentClaim {
    /// Party making/disseminating the statement (Äußernder).
    pub asserting_party: TortParty,
    /// Party affected by the statement (Betroffener).
    pub affected_party: TortParty,
    /// The statement at issue.
    pub statement: String,
    /// Whether it was asserted or disseminated (§ 824 Abs. 1).
    pub statement_form: StatementForm,
    /// Fact vs value judgment - only facts are actionable.
    pub statement_nature: StatementNature,
    /// Whether the fact is untrue ("der Wahrheit zuwider").
    pub is_untrue: bool,
    /// Suitable to endanger credit, earnings or advancement.
    pub suitable_to_endanger_credit: bool,
    /// Knowledge or negligent ignorance of the untruth ("kennt oder kennen muss", Abs. 1).
    pub knew_or_should_have_known_untruth: bool,
    /// Communicating party had no positive knowledge of the untruth (Abs. 2 precondition).
    pub communicator_unaware_of_untruth: bool,
    /// The communicator or recipient had a legitimate interest (Abs. 2 privilege).
    pub legitimate_interest: bool,
    /// Date of the statement.
    pub incident_date: DateTime<Utc>,
    /// Damages claimed.
    pub damages: DamageClaim,
    /// Causation established.
    pub causation_established: bool,
    /// Additional notes.
    pub notes: Option<String>,
}

/// Builder for [`CreditEndangermentClaim`] (§ 824 BGB).
#[derive(Debug, Default)]
pub struct CreditEndangermentClaimBuilder {
    asserting_party: Option<TortParty>,
    affected_party: Option<TortParty>,
    statement: Option<String>,
    statement_form: Option<StatementForm>,
    statement_nature: Option<StatementNature>,
    is_untrue: bool,
    suitable_to_endanger_credit: bool,
    knew_or_should_have_known_untruth: bool,
    communicator_unaware_of_untruth: bool,
    legitimate_interest: bool,
    incident_date: Option<DateTime<Utc>>,
    property_damage: Option<Capital>,
    lost_income: Option<Capital>,
    consequential_damages: Option<Capital>,
    causation_established: bool,
    notes: Option<String>,
}

impl CreditEndangermentClaimBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the asserting party (Äußernder).
    pub fn asserting_party(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.asserting_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the affected party (Betroffener).
    pub fn affected_party(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.affected_party = Some(TortParty {
            name: name.into(),
            address: Some(address.into()),
            is_natural_person: true,
        });
        self
    }

    /// Set the statement and its form/nature.
    pub fn statement(
        mut self,
        text: impl Into<String>,
        form: StatementForm,
        nature: StatementNature,
    ) -> Self {
        self.statement = Some(text.into());
        self.statement_form = Some(form);
        self.statement_nature = Some(nature);
        self
    }

    /// Mark the fact as untrue.
    pub fn untrue(mut self, untrue: bool) -> Self {
        self.is_untrue = untrue;
        self
    }

    /// Mark the fact as suitable to endanger credit/earnings/advancement.
    pub fn suitable_to_endanger_credit(mut self, suitable: bool) -> Self {
        self.suitable_to_endanger_credit = suitable;
        self
    }

    /// Set whether the party knew or should have known of the untruth (Abs. 1).
    pub fn knew_or_should_have_known(mut self, value: bool) -> Self {
        self.knew_or_should_have_known_untruth = value;
        self
    }

    /// Configure the § 824 Abs. 2 privilege (no positive knowledge + legitimate interest).
    pub fn legitimate_interest_privilege(
        mut self,
        unaware: bool,
        legitimate_interest: bool,
    ) -> Self {
        self.communicator_unaware_of_untruth = unaware;
        self.legitimate_interest = legitimate_interest;
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

    /// Set lost income (entgangener Gewinn).
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

    /// Build the claim, returning an error if required fields are missing.
    pub fn build(self) -> std::result::Result<CreditEndangermentClaim, String> {
        let asserting_party = self.asserting_party.ok_or("Asserting party required")?;
        let affected_party = self.affected_party.ok_or("Affected party required")?;
        let statement = self.statement.ok_or("Statement text required")?;
        let statement_form = self.statement_form.ok_or("Statement form required")?;
        let statement_nature = self.statement_nature.ok_or("Statement nature required")?;
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

        Ok(CreditEndangermentClaim {
            asserting_party,
            affected_party,
            statement,
            statement_form,
            statement_nature,
            is_untrue: self.is_untrue,
            suitable_to_endanger_credit: self.suitable_to_endanger_credit,
            knew_or_should_have_known_untruth: self.knew_or_should_have_known_untruth,
            communicator_unaware_of_untruth: self.communicator_unaware_of_untruth,
            legitimate_interest: self.legitimate_interest,
            incident_date,
            damages,
            causation_established: self.causation_established,
            notes: self.notes,
        })
    }
}

/// Validate a credit-endangerment claim under § 824 BGB.
///
/// Requirements (Abs. 1): untrue assertion of fact, suitable to endanger credit,
/// at least negligent ignorance of the untruth, causation and damage. The
/// § 824 Abs. 2 privilege (legitimate interest without positive knowledge of the
/// untruth) bars the claim.
///
/// # Example
///
/// ```
/// use legalis_de::bgb::unerlaubte_handlungen::*;
/// use legalis_de::gmbhg::Capital;
/// use chrono::Utc;
///
/// let claim = CreditEndangermentClaimBuilder::new()
///     .asserting_party("Competitor GmbH", "Berlin")
///     .affected_party("Mustermann AG", "Munich")
///     .statement("Firma ist zahlungsunfähig", StatementForm::Assertion, StatementNature::FactualAssertion)
///     .untrue(true)
///     .suitable_to_endanger_credit(true)
///     .knew_or_should_have_known(true)
///     .incident_date(Utc::now())
///     .damages_lost_income(Capital::from_euros(40_000))
///     .causation_established(true)
///     .build()
///     .expect("valid claim");
///
/// assert!(validate_credit_endangerment_claim(&claim).is_ok());
/// ```
pub fn validate_credit_endangerment_claim(claim: &CreditEndangermentClaim) -> Result<()> {
    if claim.asserting_party.name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }
    if claim.affected_party.name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    // Requirement: assertion of a fact, not a value judgment.
    if claim.statement_nature != StatementNature::FactualAssertion {
        return Err(TortError::NotFactualAssertion);
    }

    // Requirement: the fact must be untrue.
    if !claim.is_untrue {
        return Err(TortError::StatementNotUntrue);
    }

    // Requirement: suitable to endanger credit/earnings/advancement.
    if !claim.suitable_to_endanger_credit {
        return Err(TortError::NotSuitableToEndangerCredit);
    }

    // § 824 Abs. 2 privilege: no positive knowledge of untruth + legitimate interest.
    // Checked before the Abs. 1 fault element because the privilege presupposes the
    // absence of positive knowledge and protects even negligent communicators.
    if claim.communicator_unaware_of_untruth && claim.legitimate_interest {
        return Err(TortError::LegitimateInterestPrivilege);
    }

    // § 824 Abs. 1 fault: knowledge or negligent ignorance of the untruth.
    if !claim.knew_or_should_have_known_untruth {
        return Err(TortError::NoKnowledgeOrNegligenceOfUntruth);
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

    fn valid_claim() -> CreditEndangermentClaim {
        CreditEndangermentClaimBuilder::new()
            .asserting_party("Competitor GmbH", "Berlin")
            .affected_party("Mustermann AG", "Munich")
            .statement(
                "Firma ist zahlungsunfähig",
                StatementForm::Assertion,
                StatementNature::FactualAssertion,
            )
            .untrue(true)
            .suitable_to_endanger_credit(true)
            .knew_or_should_have_known(true)
            .incident_date(Utc::now())
            .damages_lost_income(Capital::from_euros(40_000))
            .causation_established(true)
            .build()
            .expect("valid claim builds")
    }

    #[test]
    fn test_valid_credit_endangerment_claim() {
        let claim = valid_claim();
        assert!(validate_credit_endangerment_claim(&claim).is_ok());
        assert_eq!(claim.damages.total.to_euros(), 40_000.0);
    }

    #[test]
    fn test_value_judgment_not_actionable() {
        let mut claim = valid_claim();
        claim.statement_nature = StatementNature::ValueJudgment;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::NotFactualAssertion)
        ));
    }

    #[test]
    fn test_true_statement_not_actionable() {
        let mut claim = valid_claim();
        claim.is_untrue = false;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::StatementNotUntrue)
        ));
    }

    #[test]
    fn test_not_suitable_to_endanger_credit() {
        let mut claim = valid_claim();
        claim.suitable_to_endanger_credit = false;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::NotSuitableToEndangerCredit)
        ));
    }

    #[test]
    fn test_no_fault_bars_claim() {
        let mut claim = valid_claim();
        claim.knew_or_should_have_known_untruth = false;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::NoKnowledgeOrNegligenceOfUntruth)
        ));
    }

    #[test]
    fn test_legitimate_interest_privilege_abs_2() {
        let mut claim = valid_claim();
        // Communicator unaware of untruth and a legitimate interest exists (e.g. credit agency).
        claim.communicator_unaware_of_untruth = true;
        claim.legitimate_interest = true;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::LegitimateInterestPrivilege)
        ));
    }

    #[test]
    fn test_privilege_requires_no_positive_knowledge() {
        let mut claim = valid_claim();
        // Legitimate interest but the communicator positively knew the untruth -> no privilege.
        claim.communicator_unaware_of_untruth = false;
        claim.legitimate_interest = true;
        assert!(validate_credit_endangerment_claim(&claim).is_ok());
    }

    #[test]
    fn test_dissemination_form_actionable() {
        let mut claim = valid_claim();
        claim.statement_form = StatementForm::Dissemination;
        assert!(validate_credit_endangerment_claim(&claim).is_ok());
    }

    #[test]
    fn test_no_causation() {
        let mut claim = valid_claim();
        claim.causation_established = false;
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::CausationNotProven)
        ));
    }

    #[test]
    fn test_zero_damage() {
        let mut claim = valid_claim();
        claim.damages.total = Capital { amount_cents: 0 };
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::ZeroDamage)
        ));
    }

    #[test]
    fn test_missing_affected_party() {
        let mut claim = valid_claim();
        claim.affected_party.name = "  ".to_string();
        assert!(matches!(
            validate_credit_endangerment_claim(&claim),
            Err(TortError::InjuredPartyMissing)
        ));
    }

    #[test]
    fn test_builder_missing_field() {
        let result = CreditEndangermentClaimBuilder::new()
            .asserting_party("X", "Berlin")
            .build();
        assert!(result.is_err());
    }
}
