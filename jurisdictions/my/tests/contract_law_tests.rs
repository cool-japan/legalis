//! Integration tests for contract law.

use legalis_my::contract_law::*;

#[test]
fn test_valid_contract() {
    let party1 = Party::new("Ahmad bin Ali", "850123-01-5678", PartyType::Individual);
    let party2 = Party::new("Tech Sdn Bhd", "201601012345", PartyType::Company);
    let consideration = Consideration::new("Software services").with_value_sen(1000000);

    let contract = Contract::builder()
        .contract_type(ContractType::ServiceAgreement)
        .add_party(party1)
        .add_party(party2)
        .consideration(consideration)
        .free_consent(true)
        .lawful_object(true)
        .build()
        .expect("Valid contract");

    let report = contract.validate().expect("Validation succeeds");
    assert!(report.valid);
}

#[test]
fn test_void_contract_lack_of_capacity() {
    let party1 = Party::new("Minor", "050123-01-5678", PartyType::Individual).with_capacity(false);
    let party2 = Party::new("Tech Sdn Bhd", "201601012345", PartyType::Company);
    let consideration = Consideration::new("Services");

    let contract = Contract::builder()
        .contract_type(ContractType::ServiceAgreement)
        .add_party(party1)
        .add_party(party2)
        .consideration(consideration)
        .free_consent(true)
        .lawful_object(true)
        .build()
        .expect("Contract built");

    let report = contract.validate().expect("Validation succeeds");
    assert!(!report.valid);
    assert_eq!(report.status, ContractStatus::Void);
}
