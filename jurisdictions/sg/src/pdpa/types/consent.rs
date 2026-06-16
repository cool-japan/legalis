//! PDPA consent model — collection, use and disclosure of personal data.
//!
//! Singapore's PDPA is **consent-centric**: unlike the GDPR's six lawful bases
//! (Art. 6 GDPR), the default position under the PDPA is that an organisation
//! must obtain the individual's **consent** before it collects, uses or discloses
//! personal data ([`s. 13`]), subject to a closed list of exceptions in the First
//! and Second Schedules and the **deemed consent** regime in s. 15 / s. 15A.
//!
//! This module models:
//!
//! * Express consent ([`ConsentMethod::ExpressWritten`] / [`ConsentMethod::ExpressElectronic`] /
//!   [`ConsentMethod::ExpressOral`]).
//! * Deemed consent by conduct (s. 15(1)), by contractual necessity / pass-through
//!   (s. 15(3)-(8)) and by notification (s. 15A) — see [`DeemedConsentBasis`].
//! * Withdrawal of consent (s. 16) — see [`ConsentRecord::withdraw`].
//! * Purpose limitation (s. 18) — see [`PurposeOfCollection`] and the validator.
//!
//! Citations are to the Personal Data Protection Act 2012 (No. 26 of 2012) as
//! amended by the Personal Data Protection (Amendment) Act 2020 (Act 40 of 2020).

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Method by which consent was obtained for the collection, use or disclosure of
/// personal data under the PDPA.
///
/// The PDPA distinguishes **actual consent** given under s. 14 (which may be
/// written, electronic or oral) from **deemed consent** under s. 15 / s. 15A.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentMethod {
    /// Express consent in writing (signed document) — s. 14.
    ExpressWritten,
    /// Express consent given electronically (checkbox, click-through, e-signature) — s. 14.
    ExpressElectronic,
    /// Express consent given orally (e.g. recorded telephone call) — s. 14.
    ExpressOral,
    /// Deemed consent — see [`DeemedConsentBasis`] for the specific limb (s. 15 / s. 15A).
    Deemed,
}

impl ConsentMethod {
    /// Returns `true` if this is a form of **express** (actual) consent under s. 14.
    pub fn is_express(&self) -> bool {
        matches!(
            self,
            ConsentMethod::ExpressWritten
                | ConsentMethod::ExpressElectronic
                | ConsentMethod::ExpressOral
        )
    }

    /// Returns `true` if this is **deemed** consent (s. 15 / s. 15A).
    pub fn is_deemed(&self) -> bool {
        matches!(self, ConsentMethod::Deemed)
    }

    /// Returns the governing PDPA section reference for this consent method.
    pub fn statute_section(&self) -> &'static str {
        match self {
            ConsentMethod::ExpressWritten
            | ConsentMethod::ExpressElectronic
            | ConsentMethod::ExpressOral => "PDPA s. 14",
            ConsentMethod::Deemed => "PDPA s. 15",
        }
    }
}

/// The specific limb of **deemed consent** relied upon (s. 15 / s. 15A).
///
/// Deemed consent is *not* a free-standing lawful basis: each limb has its own
/// statutory pre-conditions, and the most onerous (deemed consent by
/// notification, s. 15A) requires the organisation to have conducted a prior
/// assessment that the activity is not likely to have an adverse effect and to
/// have given the individual a reasonable opt-out period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeemedConsentBasis {
    /// Deemed consent **by conduct** — s. 15(1).
    ///
    /// The individual voluntarily provides the personal data for a purpose, and
    /// it is reasonable that the individual would do so.
    ByConduct,
    /// Deemed consent **by contractual necessity / pass-through** — s. 15(3)-(8).
    ///
    /// Where an individual provides data to organisation A with a view to
    /// entering into (or in the performance of) a contract, the individual is
    /// deemed to consent to disclosure to, and use by, a downstream organisation
    /// B where reasonably necessary for the contract.
    ByContractualNecessity,
    /// Deemed consent **by notification** — s. 15A.
    ///
    /// The organisation has (a) conducted an assessment that the proposed use is
    /// not likely to have an adverse effect on the individual, taking measures to
    /// eliminate or mitigate any adverse effect (s. 15A(4)(a), (5)); and
    /// (b) notified the individual of the purpose and a reasonable opt-out period
    /// (s. 15A(4)(b)); and the individual did not opt out within that period.
    ByNotification,
}

impl DeemedConsentBasis {
    /// Returns the governing PDPA section reference for this deemed-consent limb.
    pub fn statute_section(&self) -> &'static str {
        match self {
            DeemedConsentBasis::ByConduct => "PDPA s. 15(1)",
            DeemedConsentBasis::ByContractualNecessity => "PDPA s. 15(3)-(8)",
            DeemedConsentBasis::ByNotification => "PDPA s. 15A",
        }
    }

    /// Returns `true` if this limb requires the organisation to have conducted a
    /// prior **risk / adverse-effect assessment** (only deemed consent by
    /// notification, s. 15A(4)(a)/(5)).
    pub fn requires_prior_assessment(&self) -> bool {
        matches!(self, DeemedConsentBasis::ByNotification)
    }

    /// Returns `true` if this limb requires the organisation to have given the
    /// individual a reasonable **opt-out window** (only deemed consent by
    /// notification, s. 15A(4)(b)).
    pub fn requires_opt_out_period(&self) -> bool {
        matches!(self, DeemedConsentBasis::ByNotification)
    }
}

/// Purpose for which personal data is collected, used or disclosed.
///
/// Under the **Purpose Limitation Obligation** (s. 18) an organisation may
/// collect, use or disclose personal data only for purposes (a) that a
/// reasonable person would consider appropriate in the circumstances; and
/// (b) that the individual has been informed of under s. 20 (the Notification
/// Obligation), where applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurposeOfCollection {
    /// Direct marketing and promotional communications.
    Marketing,
    /// Provision/fulfilment of the requested product or service.
    ServiceDelivery,
    /// Customer support and after-sales service.
    CustomerSupport,
    /// Order processing, billing and payment.
    OrderProcessing,
    /// Compliance with legal or regulatory obligations.
    LegalCompliance,
    /// Internal analytics, statistics and research.
    Analytics,
    /// Recruitment / employment screening of a job applicant.
    EmploymentScreening,
    /// Administration of an existing employment relationship (HR, payroll, CPF).
    EmploymentManagement,
    /// Fraud detection and prevention.
    FraudPrevention,
}

impl PurposeOfCollection {
    /// Returns whether the given *new* purpose is reasonably compatible with the
    /// *original* purpose for which consent was obtained, for the limited
    /// purpose-limitation check in s. 18.
    ///
    /// This encodes a conservative compatibility matrix: a purpose is always
    /// compatible with itself; service delivery reasonably extends to order
    /// processing, customer support, billing and fraud prevention; and legal
    /// compliance / fraud prevention are treated as compatible with any
    /// operational purpose because they are required or authorised under other
    /// written law (s. 18 read with the exceptions in the First Schedule).
    ///
    /// Crucially, **marketing is never deemed compatible** with a non-marketing
    /// purpose: re-purposing operational data for direct marketing requires
    /// fresh consent (and, for telephone/SMS/fax marketing, a DNC check under
    /// Part 9). See [`crate::pdpa::types::dnc`].
    pub fn is_compatible_with(&self, original: PurposeOfCollection) -> bool {
        use PurposeOfCollection::*;
        if *self == original {
            return true;
        }
        // Legal compliance and fraud prevention are required/authorised under
        // other written law and are treated as compatible operational purposes.
        if matches!(self, LegalCompliance | FraudPrevention) {
            return true;
        }
        match original {
            ServiceDelivery => matches!(
                self,
                OrderProcessing | CustomerSupport | FraudPrevention | LegalCompliance
            ),
            OrderProcessing => matches!(self, ServiceDelivery | CustomerSupport | FraudPrevention),
            CustomerSupport => matches!(self, ServiceDelivery | OrderProcessing),
            EmploymentScreening => matches!(self, EmploymentManagement),
            EmploymentManagement => {
                matches!(self, LegalCompliance | FraudPrevention)
            }
            // Marketing and analytics never silently expand to other purposes.
            _ => false,
        }
    }

    /// Returns `true` if this purpose is **direct marketing**, which engages the
    /// DNC obligations in Part 9 of the PDPA when conducted by telephone call,
    /// text message or fax.
    pub fn is_marketing(&self) -> bool {
        matches!(self, PurposeOfCollection::Marketing)
    }
}

/// Category of personal data, used both for consent records and for the
/// significant-harm assessment in a data breach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonalDataCategory {
    /// Full name.
    Name,
    /// NRIC / FIN / passport or any government-issued identification number
    /// (PDP (Notification of Data Breaches) Regulations 2021, reg. 2).
    IdentificationNumber,
    /// Residential or postal address.
    Address,
    /// E-mail address.
    Email,
    /// Telephone number.
    Phone,
    /// Date of birth.
    DateOfBirth,
    /// Financial information — wages, account numbers, credit/charge/debit card
    /// numbers, creditworthiness (Schedule Part 1, S 64/2021).
    Financial,
    /// Health or medical information (Schedule Part 1, items 17-21, S 64/2021).
    Health,
    /// Account credentials — password, security code, access code, biometric or
    /// security-question answer (reg. 3(1)(b), S 64/2021).
    AccountCredentials,
    /// Biometric data.
    Biometric,
}

impl PersonalDataCategory {
    /// Returns `true` if this category is a *prescribed* category that, when
    /// combined with a name or identification number, deems a data breach to
    /// cause **significant harm** under reg. 3(1)(a) and the Schedule to the
    /// PDP (Notification of Data Breaches) Regulations 2021.
    pub fn is_significant_harm_category(&self) -> bool {
        matches!(
            self,
            PersonalDataCategory::Financial | PersonalDataCategory::Health
        )
    }
}

/// A record of consent obtained for the collection, use or disclosure of
/// personal data under the PDPA (s. 13-16, s. 18).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Stable identifier for this consent record.
    pub consent_id: String,
    /// Identifier for the data subject (e.g. hashed e-mail, masked NRIC).
    pub data_subject_id: String,
    /// The purpose for which consent was obtained (s. 18 / s. 20).
    pub purpose: PurposeOfCollection,
    /// Categories of personal data covered by this consent.
    pub data_categories: Vec<PersonalDataCategory>,
    /// How consent was obtained.
    pub consent_method: ConsentMethod,
    /// When [`ConsentMethod::Deemed`], which deemed-consent limb is relied upon.
    pub deemed_basis: Option<DeemedConsentBasis>,
    /// For deemed consent by notification (s. 15A): whether the prior
    /// not-likely-to-have-an-adverse-effect assessment was conducted.
    pub adverse_effect_assessment_done: bool,
    /// For deemed consent by notification (s. 15A): the opt-out window the
    /// individual was given. `None` means no window was offered (invalid s. 15A).
    pub opt_out_window: Option<Duration>,
    /// Timestamp at which consent took effect.
    pub consent_timestamp: DateTime<Utc>,
    /// Whether the consent is currently valid (i.e. not withdrawn).
    pub is_valid: bool,
    /// When consent was withdrawn (s. 16), if applicable.
    pub withdrawal_timestamp: Option<DateTime<Utc>>,
    /// Free-text reason given on withdrawal (s. 16), if any.
    pub withdrawal_reason: Option<String>,
    /// Whether the organisation informed the individual of the likely
    /// consequences of withdrawal, as required by s. 16(2).
    pub consequences_of_withdrawal_explained: bool,
}

impl ConsentRecord {
    /// Creates a record of **express** consent (s. 14).
    ///
    /// The `consent_method` must be one of the express variants; pass
    /// [`ConsentMethod::Deemed`] only via [`ConsentRecord::deemed`].
    pub fn express(
        consent_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        purpose: PurposeOfCollection,
        consent_method: ConsentMethod,
    ) -> Self {
        Self {
            consent_id: consent_id.into(),
            data_subject_id: data_subject_id.into(),
            purpose,
            data_categories: Vec::new(),
            consent_method,
            deemed_basis: None,
            adverse_effect_assessment_done: false,
            opt_out_window: None,
            consent_timestamp: Utc::now(),
            is_valid: true,
            withdrawal_timestamp: None,
            withdrawal_reason: None,
            consequences_of_withdrawal_explained: false,
        }
    }

    /// Creates a record of **deemed** consent under the given limb (s. 15 / s. 15A).
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
            data_categories: Vec::new(),
            consent_method: ConsentMethod::Deemed,
            deemed_basis: Some(basis),
            adverse_effect_assessment_done: false,
            opt_out_window: None,
            consent_timestamp: Utc::now(),
            is_valid: true,
            withdrawal_timestamp: None,
            withdrawal_reason: None,
            consequences_of_withdrawal_explained: false,
        }
    }

    /// Adds a personal-data category to this consent (idempotent).
    pub fn add_data_category(&mut self, category: PersonalDataCategory) -> &mut Self {
        if !self.data_categories.contains(&category) {
            self.data_categories.push(category);
        }
        self
    }

    /// Records that the s. 15A(4)(a)/(5) adverse-effect assessment was carried
    /// out and the opt-out window of `window` was offered to the individual.
    pub fn with_notification_assessment(&mut self, window: Duration) -> &mut Self {
        self.adverse_effect_assessment_done = true;
        self.opt_out_window = Some(window);
        self
    }

    /// Withdraws consent under s. 16.
    ///
    /// Per s. 16(2) an organisation must inform the individual of the likely
    /// consequences of withdrawal; set `consequences_explained` accordingly so
    /// the validator can flag a non-compliant withdrawal flow.
    pub fn withdraw(&mut self, reason: Option<String>, consequences_explained: bool) {
        self.is_valid = false;
        self.withdrawal_timestamp = Some(Utc::now());
        self.withdrawal_reason = reason;
        self.consequences_of_withdrawal_explained = consequences_explained;
    }

    /// Returns `true` if consent has been withdrawn (s. 16).
    pub fn is_withdrawn(&self) -> bool {
        self.withdrawal_timestamp.is_some()
    }

    /// Returns `true` if this consent record authorises the given `purpose`,
    /// applying the s. 18 purpose-limitation compatibility test. Withdrawn
    /// consent never authorises any purpose.
    pub fn authorises_purpose(&self, purpose: PurposeOfCollection) -> bool {
        self.is_valid && purpose.is_compatible_with(self.purpose)
    }
}

/// Whether a piece of information qualifies as **business contact information**
/// (s. 2(1)), to which the Data Protection Provisions in Parts 3-6A do not
/// apply (s. 4(5)).
///
/// Business contact information means an individual's name, position name or
/// title, business telephone number, business address, business e-mail address
/// or business fax number, **not provided by the individual solely for personal
/// purposes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataContext {
    /// Provided in a business capacity (e.g. a name card, a corporate directory).
    /// Falls within the s. 4(5) business contact information exemption.
    BusinessCapacity,
    /// Provided solely for the individual's personal purposes. The full Data
    /// Protection Provisions apply.
    PersonalCapacity,
}

/// Returns `true` if the given personal-data category, when supplied in the
/// given context, is **business contact information** exempt from the Data
/// Protection Provisions (s. 4(5) read with the s. 2(1) definition).
///
/// Only the enumerated contact attributes (name, business e-mail, business
/// phone, business address) qualify, and only when provided in a business
/// capacity. Substantive personal data (NRIC, financial, health, biometric,
/// date of birth, account credentials) is never business contact information.
pub fn is_business_contact_information(
    category: PersonalDataCategory,
    context: DataContext,
) -> bool {
    if context != DataContext::BusinessCapacity {
        return false;
    }
    matches!(
        category,
        PersonalDataCategory::Name
            | PersonalDataCategory::Email
            | PersonalDataCategory::Phone
            | PersonalDataCategory::Address
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn express_consent_is_express() {
        assert!(ConsentMethod::ExpressWritten.is_express());
        assert!(ConsentMethod::ExpressElectronic.is_express());
        assert!(ConsentMethod::ExpressOral.is_express());
        assert!(!ConsentMethod::Deemed.is_express());
        assert!(ConsentMethod::Deemed.is_deemed());
    }

    #[test]
    fn deemed_basis_section_references() {
        assert_eq!(
            DeemedConsentBasis::ByConduct.statute_section(),
            "PDPA s. 15(1)"
        );
        assert_eq!(
            DeemedConsentBasis::ByContractualNecessity.statute_section(),
            "PDPA s. 15(3)-(8)"
        );
        assert_eq!(
            DeemedConsentBasis::ByNotification.statute_section(),
            "PDPA s. 15A"
        );
    }

    #[test]
    fn only_notification_requires_assessment_and_opt_out() {
        assert!(DeemedConsentBasis::ByNotification.requires_prior_assessment());
        assert!(DeemedConsentBasis::ByNotification.requires_opt_out_period());
        assert!(!DeemedConsentBasis::ByConduct.requires_prior_assessment());
        assert!(!DeemedConsentBasis::ByContractualNecessity.requires_opt_out_period());
    }

    #[test]
    fn purpose_compatibility_marketing_never_silent() {
        // Marketing is never compatible with a non-marketing original purpose.
        assert!(
            !PurposeOfCollection::Marketing
                .is_compatible_with(PurposeOfCollection::ServiceDelivery)
        );
        // Service delivery reasonably extends to order processing and support.
        assert!(
            PurposeOfCollection::OrderProcessing
                .is_compatible_with(PurposeOfCollection::ServiceDelivery)
        );
        // Identity holds.
        assert!(PurposeOfCollection::Marketing.is_compatible_with(PurposeOfCollection::Marketing));
    }

    #[test]
    fn consent_authorises_only_compatible_and_valid() {
        let mut c = ConsentRecord::express(
            "c1",
            "subj",
            PurposeOfCollection::ServiceDelivery,
            ConsentMethod::ExpressElectronic,
        );
        c.add_data_category(PersonalDataCategory::Email);
        assert!(c.authorises_purpose(PurposeOfCollection::OrderProcessing));
        assert!(!c.authorises_purpose(PurposeOfCollection::Marketing));

        c.withdraw(None, true);
        assert!(!c.authorises_purpose(PurposeOfCollection::ServiceDelivery));
        assert!(c.is_withdrawn());
    }

    #[test]
    fn business_contact_information_exemption() {
        // Name in business capacity is exempt BCI.
        assert!(is_business_contact_information(
            PersonalDataCategory::Name,
            DataContext::BusinessCapacity
        ));
        // Same name in personal capacity is NOT exempt.
        assert!(!is_business_contact_information(
            PersonalDataCategory::Name,
            DataContext::PersonalCapacity
        ));
        // NRIC is never BCI, even in a business capacity.
        assert!(!is_business_contact_information(
            PersonalDataCategory::IdentificationNumber,
            DataContext::BusinessCapacity
        ));
    }

    #[test]
    fn significant_harm_categories() {
        assert!(PersonalDataCategory::Financial.is_significant_harm_category());
        assert!(PersonalDataCategory::Health.is_significant_harm_category());
        assert!(!PersonalDataCategory::Email.is_significant_harm_category());
    }
}
