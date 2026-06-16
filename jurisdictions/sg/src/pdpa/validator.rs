//! Personal Data Protection Act 2012 — validation logic and builders.
//!
//! This module provides the validators that apply the statutory rules encoded in
//! [`crate::pdpa::types`], plus ergonomic builders for the more complex records.
//! Every validator returns a [`Result`] carrying a [`PdpaError`] with an accurate
//! section reference, or a [`PdpaValidationReport`] aggregating multiple findings.

use super::error::{PdpaError, Result};
use super::types::*;
use chrono::{DateTime, Duration, Utc};

/// Aggregated outcome of validating an organisation or a workflow against
/// multiple PDPA obligations.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PdpaValidationReport {
    /// Whether the subject is compliant (no errors recorded).
    pub is_compliant: bool,
    /// Fatal compliance failures (each maps to a PDPA contravention).
    pub errors: Vec<String>,
    /// Non-fatal advisory findings (good-practice gaps, recommendations).
    pub warnings: Vec<String>,
}

impl PdpaValidationReport {
    /// Creates an empty, compliant report.
    pub fn new() -> Self {
        Self {
            is_compliant: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Records a fatal compliance failure (marks the report non-compliant).
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.is_compliant = false;
        self.errors.push(error.into());
    }

    /// Records a non-fatal advisory finding.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Folds a [`Result`] into the report, recording any error.
    pub fn absorb(&mut self, outcome: Result<()>) {
        if let Err(e) = outcome {
            self.add_error(e.to_string());
        }
    }
}

// ---------------------------------------------------------------------------
// Consent (ss. 13-16, 18)
// ---------------------------------------------------------------------------

/// Validates a consent record under the Consent Obligation (s. 13) and the
/// deemed-consent requirements (s. 15 / s. 15A).
///
/// Checks performed:
/// * the consent has not been withdrawn (s. 16);
/// * at least one personal-data category is specified;
/// * for deemed consent by notification (s. 15A), the prior adverse-effect
///   assessment was carried out (s. 15A(4)(a)/(5)) and an opt-out window was
///   given (s. 15A(4)(b)).
pub fn validate_consent(record: &ConsentRecord) -> Result<()> {
    if !record.is_valid {
        return Err(PdpaError::ConsentWithdrawn);
    }

    if record.data_categories.is_empty() {
        return Err(PdpaError::ValidationError {
            message: "No personal data categories specified in the consent record".to_string(),
        });
    }

    if record.consent_method.is_deemed() {
        let basis = record
            .deemed_basis
            .ok_or_else(|| PdpaError::ValidationError {
                message: "Deemed consent record does not specify a deemed-consent basis"
                    .to_string(),
            })?;
        if basis.requires_prior_assessment() && !record.adverse_effect_assessment_done {
            return Err(PdpaError::InvalidDeemedConsent {
                reason: "no prior assessment that the use is not likely to have an adverse effect"
                    .to_string(),
            });
        }
        if basis.requires_opt_out_period() && record.opt_out_window.is_none() {
            return Err(PdpaError::InvalidDeemedConsent {
                reason: "no reasonable opt-out period was offered to the individual".to_string(),
            });
        }
    }

    Ok(())
}

/// Validates a withdrawal of consent under s. 16.
///
/// A compliant withdrawal requires that the organisation informed the individual
/// of the likely consequences of withdrawal (s. 16(2)). Returns an error if the
/// record has not actually been withdrawn, or if the consequences were not
/// explained.
pub fn validate_withdrawal(record: &ConsentRecord) -> Result<()> {
    if !record.is_withdrawn() {
        return Err(PdpaError::ValidationError {
            message: "Consent record has not been withdrawn".to_string(),
        });
    }
    if !record.consequences_of_withdrawal_explained {
        return Err(PdpaError::WithdrawalConsequencesNotExplained);
    }
    Ok(())
}

/// Validates the Purpose Limitation Obligation (s. 18): whether personal data
/// collected under `consent` may be used or disclosed for `intended_purpose`.
///
/// Returns [`PdpaError::PurposeLimitationViolation`] if the intended purpose is
/// not reasonably compatible with the original purpose (or if consent has been
/// withdrawn).
pub fn validate_purpose_limitation(
    consent: &ConsentRecord,
    intended_purpose: PurposeOfCollection,
) -> Result<()> {
    if !consent.is_valid {
        return Err(PdpaError::ConsentWithdrawn);
    }
    if !consent.authorises_purpose(intended_purpose) {
        return Err(PdpaError::PurposeLimitationViolation);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Data breach (ss. 26B-26D)
// ---------------------------------------------------------------------------

/// Validates the data breach notification lifecycle (ss. 26B-26D).
///
/// * If the breach is notifiable (s. 26B) but the PDPC has not been notified,
///   returns [`PdpaError::LateBreachNotification`].
/// * If the PDPC was notified but more than 3 calendar days after the
///   assessment (s. 26D(1)), returns [`PdpaError::LateBreachNotification`].
/// * If the significant-harm limb applies and affected individuals have neither
///   been notified nor exempted (s. 26D(2)/(5)-(7)), returns
///   [`PdpaError::IndividualsNotNotified`].
pub fn validate_breach_notification(breach: &DataBreachNotification) -> Result<()> {
    let assessment = breach.assess_notifiability();
    if !assessment.is_notifiable() {
        return Ok(());
    }

    match breach.pdpc_notification_date {
        None => return Err(PdpaError::LateBreachNotification),
        Some(_) => {
            if !breach.is_pdpc_notification_timely() {
                return Err(PdpaError::LateBreachNotification);
            }
        }
    }

    if breach.individual_notification_outstanding() {
        return Err(PdpaError::IndividualsNotNotified);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// DNC Registry (Part 9)
// ---------------------------------------------------------------------------

/// Validates that a marketing message of `kind` may be sent to `phone` at time
/// `now`, given the number's DNC registration and any prior non-registration
/// confirmation (Part 9, s. 43).
///
/// The message may be sent only if:
/// * the number is not listed on the relevant register (otherwise
///   [`PdpaError::DncViolation`]); and
/// * there is a valid, in-date (within 21 days) confirmation of non-registration
///   covering that number and register (otherwise [`PdpaError::MissingDncCheck`]).
pub fn validate_dnc_before_marketing(
    phone: &str,
    kind: DncRegisterKind,
    registration: &DncRegistration,
    confirmation: Option<&DncCheckConfirmation>,
    now: DateTime<Utc>,
) -> Result<()> {
    if registration.is_listed_on(kind) {
        return Err(PdpaError::DncViolation {
            phone: phone.to_string(),
            register: kind.register_name().to_string(),
        });
    }
    match confirmation {
        Some(conf) if conf.is_valid_for(phone, kind, now) => Ok(()),
        _ => Err(PdpaError::MissingDncCheck {
            phone: phone.to_string(),
        }),
    }
}

// ---------------------------------------------------------------------------
// Cross-border transfer (s. 26)
// ---------------------------------------------------------------------------

/// Validates a cross-border transfer against the Transfer Limitation Obligation
/// (s. 26 read with PDP Regulations 2021, regs. 10-12).
///
/// * For the consent mechanism, the individual must have been informed of the
///   transfer (reg. 10(2)(b)).
/// * For the contractual-clauses and BCR mechanisms, the recipient must in fact
///   be bound by legally enforceable obligations to provide comparable
///   protection (regs. 10/11).
/// * The certification, contractual-necessity and deemed-satisfaction
///   mechanisms are accepted on their own terms.
pub fn validate_cross_border_transfer(transfer: &DataTransfer) -> Result<()> {
    match transfer.mechanism {
        TransferMechanism::Consent => {
            if !transfer.individual_informed_of_transfer {
                return Err(PdpaError::InadequateTransferProtection {
                    country: transfer.destination_country.clone(),
                });
            }
        }
        TransferMechanism::ContractualClauses | TransferMechanism::BindingCorporateRules => {
            if !transfer.recipient_bound_comparable_protection {
                return Err(PdpaError::InadequateTransferProtection {
                    country: transfer.destination_country.clone(),
                });
            }
        }
        TransferMechanism::ContractualNecessity
        | TransferMechanism::SpecifiedCertification
        | TransferMechanism::DeemedSatisfaction => {}
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Access / correction (ss. 21-22)
// ---------------------------------------------------------------------------

/// Validates a data-subject access or correction request against the response
/// deadlines (s. 21 / reg. 5 for access; s. 22 for correction), evaluated at
/// time `now`.
pub fn validate_data_subject_request(
    request: &DataSubjectRequest,
    now: DateTime<Utc>,
) -> Result<()> {
    if request.is_within_deadline(now) {
        return Ok(());
    }
    match request.kind {
        DataSubjectRequestKind::Access => Err(PdpaError::AccessRequestOverdue),
        DataSubjectRequestKind::Correction => Err(PdpaError::CorrectionRequestOverdue),
    }
}

// ---------------------------------------------------------------------------
// Organisation accountability (s. 11)
// ---------------------------------------------------------------------------

/// Validates an organisation's accountability posture (s. 11).
///
/// Records:
/// * an **error** if no DPO has been designated (mandatory under s. 11(3));
/// * an **error** if a DPO is designated but its business contact information
///   has not been made public (s. 11(5));
/// * a **warning** if no data protection / privacy policy URL is published
///   (s. 12 good practice);
/// * a **warning** with the recommended DPO staffing level (advisory only).
pub fn validate_organisation_accountability(org: &PdpaOrganisation) -> PdpaValidationReport {
    let mut report = PdpaValidationReport::new();

    match &org.dpo_contact {
        None => report.add_error(PdpaError::NoDataProtectionOfficer.to_string()),
        Some(dpo) => {
            if !dpo.published_to_public {
                report.add_error(PdpaError::DpoContactNotPublished.to_string());
            }
        }
    }

    if org.privacy_policy_url.is_none() {
        report.add_warning(
            "No published data protection policy URL (PDPA s. 12 — develop and implement policies)"
                .to_string(),
        );
    }

    report.add_warning(format!(
        "Recommended DPO staffing level (advisory, s. 11(3) designation is mandatory regardless): {:?}",
        org.dpo_staffing_recommendation()
    ));

    report
}

/// Convenience: returns whether the organisation satisfies the *mandatory*
/// s. 11(3) duty to designate at least one DPO. Note that the appointment is
/// always required; this is **not** an advisory threshold.
pub fn dpo_appointment_satisfied(org: &PdpaOrganisation) -> bool {
    org.has_designated_dpo()
}

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

/// Ergonomic builder for [`ConsentRecord`], supporting both express (s. 14) and
/// deemed (s. 15 / s. 15A) consent.
#[derive(Debug, Clone)]
pub struct ConsentRecordBuilder {
    consent_id: String,
    data_subject_id: String,
    purpose: PurposeOfCollection,
    method: ConsentMethod,
    deemed_basis: Option<DeemedConsentBasis>,
    categories: Vec<PersonalDataCategory>,
    assessment_done: bool,
    opt_out_window: Option<Duration>,
}

impl ConsentRecordBuilder {
    /// Begins building an **express** consent record (s. 14).
    pub fn express(
        consent_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        purpose: PurposeOfCollection,
        method: ConsentMethod,
    ) -> Self {
        Self {
            consent_id: consent_id.into(),
            data_subject_id: data_subject_id.into(),
            purpose,
            method,
            deemed_basis: None,
            categories: Vec::new(),
            assessment_done: false,
            opt_out_window: None,
        }
    }

    /// Begins building a **deemed** consent record (s. 15 / s. 15A).
    pub fn deemed(
        consent_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        purpose: PurposeOfCollection,
        basis: DeemedConsentBasis,
    ) -> Self {
        Self {
            consent_id: consent_id.into(),
            data_subject_id: data_subject_id.into(),
            purpose,
            method: ConsentMethod::Deemed,
            deemed_basis: Some(basis),
            categories: Vec::new(),
            assessment_done: false,
            opt_out_window: None,
        }
    }

    /// Adds a personal-data category.
    pub fn data_category(mut self, category: PersonalDataCategory) -> Self {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }
        self
    }

    /// Records the s. 15A(4)(a)/(5) adverse-effect assessment and the opt-out
    /// window offered to the individual.
    pub fn notification_assessment(mut self, opt_out_window: Duration) -> Self {
        self.assessment_done = true;
        self.opt_out_window = Some(opt_out_window);
        self
    }

    /// Builds and validates the consent record. Returns an error if the record
    /// is not a valid consent under [`validate_consent`].
    pub fn build(self) -> Result<ConsentRecord> {
        let mut record = match self.deemed_basis {
            Some(basis) => {
                ConsentRecord::deemed(self.consent_id, self.data_subject_id, self.purpose, basis)
            }
            None => ConsentRecord::express(
                self.consent_id,
                self.data_subject_id,
                self.purpose,
                self.method,
            ),
        };
        for category in self.categories {
            record.add_data_category(category);
        }
        record.adverse_effect_assessment_done = self.assessment_done;
        record.opt_out_window = self.opt_out_window;
        validate_consent(&record)?;
        Ok(record)
    }
}

/// Ergonomic builder for [`DataBreachNotification`].
#[derive(Debug, Clone)]
pub struct DataBreachBuilder {
    breach_id: String,
    breach_type: BreachType,
    scope: BreachScope,
    affected_individuals: u32,
    description: String,
    categories: Vec<PersonalDataCategory>,
}

impl DataBreachBuilder {
    /// Begins building an external data breach record.
    pub fn new(
        breach_id: impl Into<String>,
        breach_type: BreachType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            breach_id: breach_id.into(),
            breach_type,
            scope: BreachScope::External,
            affected_individuals: 0,
            description: description.into(),
            categories: Vec::new(),
        }
    }

    /// Marks the breach as internal-only (excluded from notification, s. 26B(4)).
    pub fn internal_only(mut self) -> Self {
        self.scope = BreachScope::InternalOnly;
        self
    }

    /// Sets the number of affected individuals.
    pub fn affected_individuals(mut self, n: u32) -> Self {
        self.affected_individuals = n;
        self
    }

    /// Adds an affected personal-data category.
    pub fn affected_category(mut self, category: PersonalDataCategory) -> Self {
        self.categories.push(category);
        self
    }

    /// Builds the breach record.
    pub fn build(self) -> DataBreachNotification {
        let mut breach = DataBreachNotification::new(
            self.breach_id,
            self.breach_type,
            self.affected_individuals,
            self.description,
        );
        breach.scope = self.scope;
        for category in self.categories {
            breach.add_affected_category(category);
        }
        breach
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_express_consent() {
        let consent = ConsentRecordBuilder::express(
            "c1",
            "subj",
            PurposeOfCollection::Marketing,
            ConsentMethod::ExpressElectronic,
        )
        .data_category(PersonalDataCategory::Email)
        .build()
        .expect("valid express consent");
        assert!(validate_consent(&consent).is_ok());
    }

    #[test]
    fn deemed_by_notification_requires_assessment() {
        // Missing assessment + opt-out window -> build fails.
        let result = ConsentRecordBuilder::deemed(
            "c2",
            "subj",
            PurposeOfCollection::Analytics,
            DeemedConsentBasis::ByNotification,
        )
        .data_category(PersonalDataCategory::Email)
        .build();
        assert!(matches!(
            result,
            Err(PdpaError::InvalidDeemedConsent { .. })
        ));

        // With assessment + opt-out window -> ok.
        let ok = ConsentRecordBuilder::deemed(
            "c3",
            "subj",
            PurposeOfCollection::Analytics,
            DeemedConsentBasis::ByNotification,
        )
        .data_category(PersonalDataCategory::Email)
        .notification_assessment(Duration::days(30))
        .build();
        assert!(ok.is_ok());
    }

    #[test]
    fn withdrawal_requires_consequences_explained() {
        let mut consent = ConsentRecordBuilder::express(
            "c4",
            "subj",
            PurposeOfCollection::Marketing,
            ConsentMethod::ExpressElectronic,
        )
        .data_category(PersonalDataCategory::Email)
        .build()
        .expect("valid");

        consent.withdraw(Some("stop".to_string()), false);
        assert!(matches!(
            validate_withdrawal(&consent),
            Err(PdpaError::WithdrawalConsequencesNotExplained)
        ));

        consent.consequences_of_withdrawal_explained = true;
        assert!(validate_withdrawal(&consent).is_ok());
    }

    #[test]
    fn purpose_limitation_blocks_marketing() {
        let consent = ConsentRecordBuilder::express(
            "c5",
            "subj",
            PurposeOfCollection::ServiceDelivery,
            ConsentMethod::ExpressElectronic,
        )
        .data_category(PersonalDataCategory::Email)
        .build()
        .expect("valid");
        assert!(
            validate_purpose_limitation(&consent, PurposeOfCollection::OrderProcessing).is_ok()
        );
        assert!(matches!(
            validate_purpose_limitation(&consent, PurposeOfCollection::Marketing),
            Err(PdpaError::PurposeLimitationViolation)
        ));
    }

    #[test]
    fn dnc_blocks_listed_number() {
        let mut reg = DncRegistration::new("+6591234567");
        reg.register(DncRegisterKind::VoiceCall);
        let result = validate_dnc_before_marketing(
            "+6591234567",
            DncRegisterKind::VoiceCall,
            &reg,
            None,
            Utc::now(),
        );
        assert!(matches!(result, Err(PdpaError::DncViolation { .. })));
    }

    #[test]
    fn dnc_requires_valid_confirmation() {
        let reg = DncRegistration::new("+6591234567");
        // Unlisted but no confirmation -> MissingDncCheck.
        let result = validate_dnc_before_marketing(
            "+6591234567",
            DncRegisterKind::VoiceCall,
            &reg,
            None,
            Utc::now(),
        );
        assert!(matches!(result, Err(PdpaError::MissingDncCheck { .. })));

        // Unlisted with valid confirmation -> ok.
        let now = Utc::now();
        let conf = DncCheckConfirmation::at("+6591234567", DncRegisterKind::VoiceCall, now);
        assert!(
            validate_dnc_before_marketing(
                "+6591234567",
                DncRegisterKind::VoiceCall,
                &reg,
                Some(&conf),
                now,
            )
            .is_ok()
        );
    }

    #[test]
    fn organisation_without_dpo_is_non_compliant() {
        let org = PdpaOrganisation::new("Acme", OrganisationType::Private);
        let report = validate_organisation_accountability(&org);
        assert!(!report.is_compliant);
        assert!(report.errors.iter().any(|e| e.contains("s. 11(3)")));
    }

    #[test]
    fn organisation_with_published_dpo_is_compliant() {
        let mut dpo = DpoContact::new("DPO", "dpo@acme.sg", "+6561234567");
        dpo.publish();
        let org = PdpaOrganisation::new("Acme", OrganisationType::Private)
            .with_dpo(dpo)
            .with_privacy_policy("https://acme.sg/privacy");
        let report = validate_organisation_accountability(&org);
        assert!(report.is_compliant, "errors: {:?}", report.errors);
    }
}
