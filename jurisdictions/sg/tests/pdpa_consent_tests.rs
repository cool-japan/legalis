//! Integration tests for the PDPA consent model (Personal Data Protection Act
//! 2012): express vs deemed consent (s. 14/15/15A), purpose limitation (s. 18),
//! withdrawal of consent (s. 16), the business contact information exemption
//! (s. 4(5)) and the DNC check-before-marketing rule (Part 9).

use chrono::{Duration, Utc};
use legalis_sg::pdpa::*;

// ---------------------------------------------------------------------------
// Express vs deemed consent (s. 14 / s. 15 / s. 15A)
// ---------------------------------------------------------------------------

#[test]
fn express_consent_is_valid_with_a_category() {
    let consent = ConsentRecordBuilder::express(
        "c-express",
        "subj",
        PurposeOfCollection::Marketing,
        ConsentMethod::ExpressWritten,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("express consent should build");
    assert!(consent.consent_method.is_express());
    assert_eq!(consent.consent_method.statute_section(), "PDPA s. 14");
    assert!(validate_consent(&consent).is_ok());
}

#[test]
fn consent_without_category_is_invalid() {
    let result = ConsentRecordBuilder::express(
        "c-empty",
        "subj",
        PurposeOfCollection::ServiceDelivery,
        ConsentMethod::ExpressElectronic,
    )
    .build();
    assert!(
        result.is_err(),
        "consent with no data categories must be rejected"
    );
}

#[test]
fn deemed_consent_by_conduct_does_not_need_assessment() {
    // s. 15(1): deemed consent by conduct — no s. 15A assessment required.
    let consent = ConsentRecordBuilder::deemed(
        "c-conduct",
        "subj",
        PurposeOfCollection::ServiceDelivery,
        DeemedConsentBasis::ByConduct,
    )
    .data_category(PersonalDataCategory::Phone)
    .build()
    .expect("deemed-by-conduct consent should build");
    assert_eq!(
        consent.deemed_basis.expect("basis").statute_section(),
        "PDPA s. 15(1)"
    );
    assert!(validate_consent(&consent).is_ok());
}

#[test]
fn deemed_consent_by_contractual_necessity_section() {
    let consent = ConsentRecordBuilder::deemed(
        "c-contract",
        "subj",
        PurposeOfCollection::OrderProcessing,
        DeemedConsentBasis::ByContractualNecessity,
    )
    .data_category(PersonalDataCategory::Name)
    .data_category(PersonalDataCategory::Address)
    .build()
    .expect("deemed-by-contract consent should build");
    assert_eq!(
        consent.deemed_basis.expect("basis").statute_section(),
        "PDPA s. 15(3)-(8)"
    );
}

#[test]
fn deemed_consent_by_notification_requires_assessment_and_opt_out() {
    // Missing assessment + opt-out -> rejected (s. 15A(4)).
    let bad = ConsentRecordBuilder::deemed(
        "c-15a-bad",
        "subj",
        PurposeOfCollection::Analytics,
        DeemedConsentBasis::ByNotification,
    )
    .data_category(PersonalDataCategory::Email)
    .build();
    assert!(matches!(bad, Err(PdpaError::InvalidDeemedConsent { .. })));

    // With assessment + opt-out window -> valid.
    let good = ConsentRecordBuilder::deemed(
        "c-15a-good",
        "subj",
        PurposeOfCollection::Analytics,
        DeemedConsentBasis::ByNotification,
    )
    .data_category(PersonalDataCategory::Email)
    .notification_assessment(Duration::days(30))
    .build()
    .expect("complete s. 15A consent should build");
    assert!(good.adverse_effect_assessment_done);
    assert!(good.opt_out_window.is_some());
    assert!(validate_consent(&good).is_ok());
}

// ---------------------------------------------------------------------------
// Purpose limitation (s. 18)
// ---------------------------------------------------------------------------

#[test]
fn purpose_limitation_permits_compatible_operational_use() {
    let consent = ConsentRecordBuilder::express(
        "c-purpose",
        "subj",
        PurposeOfCollection::ServiceDelivery,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("valid");
    // Order processing and customer support are compatible with service delivery.
    assert!(validate_purpose_limitation(&consent, PurposeOfCollection::OrderProcessing).is_ok());
    assert!(validate_purpose_limitation(&consent, PurposeOfCollection::CustomerSupport).is_ok());
}

#[test]
fn purpose_limitation_blocks_repurposing_for_marketing() {
    let consent = ConsentRecordBuilder::express(
        "c-purpose-mkt",
        "subj",
        PurposeOfCollection::ServiceDelivery,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("valid");
    // Re-purposing service data for marketing needs fresh consent (s. 18).
    assert!(matches!(
        validate_purpose_limitation(&consent, PurposeOfCollection::Marketing),
        Err(PdpaError::PurposeLimitationViolation)
    ));
}

#[test]
fn legal_compliance_is_compatible_with_any_operational_purpose() {
    let consent = ConsentRecordBuilder::express(
        "c-purpose-legal",
        "subj",
        PurposeOfCollection::OrderProcessing,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Financial)
    .build()
    .expect("valid");
    assert!(validate_purpose_limitation(&consent, PurposeOfCollection::LegalCompliance).is_ok());
    assert!(validate_purpose_limitation(&consent, PurposeOfCollection::FraudPrevention).is_ok());
}

// ---------------------------------------------------------------------------
// Withdrawal of consent (s. 16)
// ---------------------------------------------------------------------------

#[test]
fn withdrawal_requires_consequences_explained() {
    let mut consent = ConsentRecordBuilder::express(
        "c-withdraw",
        "subj",
        PurposeOfCollection::Marketing,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("valid");

    consent.withdraw(Some("opt out".to_string()), false);
    assert!(consent.is_withdrawn());
    assert!(matches!(
        validate_withdrawal(&consent),
        Err(PdpaError::WithdrawalConsequencesNotExplained)
    ));

    consent.consequences_of_withdrawal_explained = true;
    assert!(validate_withdrawal(&consent).is_ok());
}

#[test]
fn withdrawn_consent_authorises_nothing_and_blocks_validation() {
    let mut consent = ConsentRecordBuilder::express(
        "c-withdraw-2",
        "subj",
        PurposeOfCollection::ServiceDelivery,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Email)
    .build()
    .expect("valid");
    consent.withdraw(None, true);
    // s. 16(4): processing must cease.
    assert!(!consent.authorises_purpose(PurposeOfCollection::ServiceDelivery));
    assert!(matches!(
        validate_consent(&consent),
        Err(PdpaError::ConsentWithdrawn)
    ));
    assert!(matches!(
        validate_purpose_limitation(&consent, PurposeOfCollection::ServiceDelivery),
        Err(PdpaError::ConsentWithdrawn)
    ));
}

// ---------------------------------------------------------------------------
// Business contact information exemption (s. 4(5) / s. 2(1))
// ---------------------------------------------------------------------------

#[test]
fn business_contact_information_is_exempt_only_in_business_capacity() {
    // Name card details given in a business capacity are exempt BCI.
    assert!(is_business_contact_information(
        PersonalDataCategory::Email,
        DataContext::BusinessCapacity
    ));
    assert!(is_business_contact_information(
        PersonalDataCategory::Phone,
        DataContext::BusinessCapacity
    ));
    // The same email provided for personal purposes is fully protected.
    assert!(!is_business_contact_information(
        PersonalDataCategory::Email,
        DataContext::PersonalCapacity
    ));
}

#[test]
fn substantive_personal_data_is_never_business_contact_information() {
    for category in [
        PersonalDataCategory::IdentificationNumber,
        PersonalDataCategory::Financial,
        PersonalDataCategory::Health,
        PersonalDataCategory::DateOfBirth,
        PersonalDataCategory::AccountCredentials,
    ] {
        assert!(
            !is_business_contact_information(category, DataContext::BusinessCapacity),
            "{category:?} must not be treated as business contact information"
        );
    }
}

// ---------------------------------------------------------------------------
// DNC check before marketing (Part 9, s. 43)
// ---------------------------------------------------------------------------

#[test]
fn dnc_three_registers_are_independent() {
    let mut reg = DncRegistration::new("+6591234567");
    reg.register(DncRegisterKind::VoiceCall);
    let now = Utc::now();
    let conf_voice = DncCheckConfirmation::at("+6591234567", DncRegisterKind::VoiceCall, now);
    let conf_sms = DncCheckConfirmation::at("+6591234567", DncRegisterKind::TextMessage, now);

    // Listed on voice -> blocked for calls.
    assert!(matches!(
        validate_dnc_before_marketing(
            "+6591234567",
            DncRegisterKind::VoiceCall,
            &reg,
            Some(&conf_voice),
            now
        ),
        Err(PdpaError::DncViolation { .. })
    ));
    // Not listed on text -> may SMS with a valid check.
    assert!(
        validate_dnc_before_marketing(
            "+6591234567",
            DncRegisterKind::TextMessage,
            &reg,
            Some(&conf_sms),
            now
        )
        .is_ok()
    );
}

#[test]
fn dnc_confirmation_must_be_within_21_days() {
    let unlisted = DncRegistration::new("+6599998888");
    let now = Utc::now();

    // 22-day-old confirmation is stale -> MissingDncCheck.
    let stale = DncCheckConfirmation::at(
        "+6599998888",
        DncRegisterKind::VoiceCall,
        now - Duration::days(22),
    );
    assert!(matches!(
        validate_dnc_before_marketing(
            "+6599998888",
            DncRegisterKind::VoiceCall,
            &unlisted,
            Some(&stale),
            now
        ),
        Err(PdpaError::MissingDncCheck { .. })
    ));

    // 21-day-old confirmation is still valid.
    let fresh = DncCheckConfirmation::at(
        "+6599998888",
        DncRegisterKind::VoiceCall,
        now - Duration::days(21),
    );
    assert!(
        validate_dnc_before_marketing(
            "+6599998888",
            DncRegisterKind::VoiceCall,
            &unlisted,
            Some(&fresh),
            now
        )
        .is_ok()
    );
}
