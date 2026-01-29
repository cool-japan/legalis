//! Integration tests for PDPA compliance.

use legalis_my::data_protection::*;

#[test]
fn test_valid_consent() {
    let consent = ConsentRecord::builder()
        .data_subject_id("customer@example.com")
        .purpose(PurposeOfCollection::Marketing)
        .consent_method(ConsentMethod::Written)
        .add_data_category(PersonalDataCategory::Name)
        .add_data_category(PersonalDataCategory::ContactInfo)
        .notice_given(true)
        .build()
        .expect("Valid consent");

    assert!(consent.validate().is_ok());
}

#[test]
fn test_invalid_consent_no_notice() {
    let consent = ConsentRecord::builder()
        .data_subject_id("customer@example.com")
        .purpose(PurposeOfCollection::Marketing)
        .consent_method(ConsentMethod::Written)
        .add_data_category(PersonalDataCategory::Name)
        .notice_given(false)
        .build()
        .expect("Consent built");

    assert!(consent.validate().is_err());
}
