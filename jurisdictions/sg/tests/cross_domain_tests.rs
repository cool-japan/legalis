//! Cross-domain scenario tests exercising several Singapore legal domains in a
//! single realistic workflow:
//!
//! 1. **Company + Employment** — a newly incorporated Pte Ltd hires its first
//!    employee; both the Companies Act formation rules and the Employment Act
//!    contract/CPF rules must hold.
//! 2. **Company + PDPA** — the company acts as a data controller and must
//!    discharge the PDPA Accountability Obligation (mandatory DPO, s. 11).
//! 3. **Employment + PDPA** — the employer collects employee personal data; the
//!    PDPA employment exceptions and breach regime interact with the employment
//!    relationship.
//! 4. **Consumer contract + PDPA** — an e-commerce sale combines a consumer
//!    contract (SOGA/CPFTA) with PDPA consent for the customer's data and a DNC
//!    check before marketing.

use chrono::{Duration, Utc};
use legalis_sg::companies::types::Address;
use legalis_sg::companies::{Company, CompanyType, Director};
use legalis_sg::consumer::{ConsumerContract, TransactionType, validate_consumer_contract};
use legalis_sg::employment::*;
use legalis_sg::pdpa::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sg_address() -> Address {
    Address::singapore("1 Raffles Place", "048616")
}

/// A compliant private limited company with a resident director (s. 145).
fn incorporated_company() -> Company {
    let mut company = Company::new(
        "202412345A",
        "Tech Innovations Pte Ltd",
        CompanyType::PrivateLimited,
        sg_address(),
    );
    company
        .directors
        .push(Director::new("John Tan", "S1234567A", true));
    company
}

// ---------------------------------------------------------------------------
// 1. Company + Employment
// ---------------------------------------------------------------------------

#[test]
fn company_hires_first_employee_company_and_employment_compliant() {
    let company = incorporated_company();
    // Companies Act: resident director present (s. 145).
    assert!(
        company.has_resident_director(),
        "company must have a resident director (CA s. 145)"
    );

    // Employment Act: the company hires a workman earning SGD 3,500.
    let contract = EmploymentContract {
        employee_name: "Jane Lim".to_string(),
        employer_name: company.name.clone(),
        contract_type: ContractType::Indefinite,
        start_date: Utc::now(),
        end_date: None,
        basic_salary_cents: 350_000, // SGD 3,500
        allowances: vec![Allowance::new("Transport", 20_000, true)],
        working_hours: WorkingHours::standard(),
        leave_entitlement: LeaveEntitlement::new(0),
        cpf_applicable: true,
        covered_by_ea: true,
    };

    let report = validate_employment_contract(&contract).expect("contract validates");
    assert!(report.is_valid, "errors: {:?}", report.errors);

    // A workman at SGD 3,500 is covered by Part IV (s. 35).
    assert!(is_covered_by_part_iv(EmployeeCategory::Workman, 350_000));

    // CPF for the same employer (age 30): employer 17%, employee 20%.
    let cpf = CpfContribution::new(30, 350_000);
    assert_eq!(cpf.employer_rate_bps, 1700);
    assert_eq!(cpf.employee_rate_bps, 2000);
    assert_eq!(cpf.employer_contribution_cents(), 59_500); // SGD 595
}

// ---------------------------------------------------------------------------
// 2. Company + PDPA (corporate data controller)
// ---------------------------------------------------------------------------

#[test]
fn company_as_data_controller_must_designate_dpo() {
    let company = incorporated_company();

    // The same legal entity is a PDPA organisation; s. 11(3) makes DPO
    // designation mandatory.
    let org_without_dpo = PdpaOrganisation::new(company.name.clone(), OrganisationType::Private)
        .with_uen(company.uen.clone());
    let report = validate_organisation_accountability(&org_without_dpo);
    assert!(
        !report.is_compliant,
        "a company without a DPO contravenes PDPA s. 11(3)"
    );
    assert!(report.errors.iter().any(|e| e.contains("s. 11(3)")));

    // Once a DPO is designated and its contact published (s. 11(5)), and a
    // privacy policy is in place (s. 12), the company is compliant.
    let mut dpo = DpoContact::new(
        "Data Protection Officer",
        "dpo@techinnovations.sg",
        "+6561234567",
    );
    dpo.publish();
    let org_with_dpo = PdpaOrganisation::new(company.name.clone(), OrganisationType::Private)
        .with_uen(company.uen.clone())
        .with_dpo(dpo)
        .with_privacy_policy("https://techinnovations.sg/privacy")
        .with_data_profile(2_000, false);
    let report = validate_organisation_accountability(&org_with_dpo);
    assert!(report.is_compliant, "errors: {:?}", report.errors);
    assert!(dpo_appointment_satisfied(&org_with_dpo));
}

// ---------------------------------------------------------------------------
// 3. Employment + PDPA (employee personal data)
// ---------------------------------------------------------------------------

#[test]
fn employer_processes_employee_data_under_pdpa() {
    // Collecting an applicant's NRIC/financial data for screening is deemed
    // consent by conduct (s. 15(1)) when the applicant provides it voluntarily.
    let screening_consent = ConsentRecordBuilder::deemed(
        "emp-consent-screen",
        "applicant-001",
        PurposeOfCollection::EmploymentScreening,
        DeemedConsentBasis::ByConduct,
    )
    .data_category(PersonalDataCategory::IdentificationNumber)
    .data_category(PersonalDataCategory::Financial)
    .build()
    .expect("screening consent should build");
    assert!(validate_consent(&screening_consent).is_ok());

    // Screening data may be used to administer the ensuing employment
    // relationship (compatible purpose under s. 18).
    assert!(
        validate_purpose_limitation(
            &screening_consent,
            PurposeOfCollection::EmploymentManagement
        )
        .is_ok()
    );
    // But NOT for marketing to the employee without fresh consent (s. 18).
    assert!(matches!(
        validate_purpose_limitation(&screening_consent, PurposeOfCollection::Marketing),
        Err(PdpaError::PurposeLimitationViolation)
    ));

    // A breach of an HR database holding employee NRIC + bank details is a
    // significant-harm breach (reg. 3(1)(a)) and must be reported within 3
    // calendar days of assessment (s. 26D(1)).
    let assessed = Utc::now();
    let mut breach = DataBreachBuilder::new(
        "emp-breach",
        BreachType::UnauthorizedAccess,
        "HR database with employee NRIC and bank account numbers accessed",
    )
    .affected_individuals(40)
    .affected_category(PersonalDataCategory::IdentificationNumber)
    .affected_category(PersonalDataCategory::Financial)
    .build();
    breach.record_assessment(assessed);
    assert!(breach.is_notifiable());
    assert!(
        breach
            .assess_notifiability()
            .requires_individual_notification()
    );

    // Notify the PDPC the next day and the affected employees -> compliant.
    breach.notify_pdpc(assessed + Duration::days(1));
    breach.notify_individuals(assessed + Duration::days(1));
    assert!(validate_breach_notification(&breach).is_ok());
}

// ---------------------------------------------------------------------------
// 4. Consumer contract + PDPA (e-commerce)
// ---------------------------------------------------------------------------

#[test]
fn ecommerce_sale_combines_consumer_contract_and_pdpa() {
    // Consumer side: an online sale of goods for SGD 1,200.
    let contract = ConsumerContract::new(
        "order-9001",
        "ShopSG Pte Ltd",
        "Alex Wong",
        TransactionType::SaleOfGoods,
        120_000, // SGD 1,200
        "Wireless headphones",
    );
    // No unfair terms, under the SCT limit -> validates clean.
    assert!(
        validate_consumer_contract(&contract).is_ok(),
        "consumer contract should validate"
    );
    // Under the Small Claims Tribunal threshold (SGD 20,000) and within the
    // Lemon Law window for goods.
    assert!(contract.is_sct_eligible());
    assert!(contract.is_lemon_law_applicable());

    // PDPA side: the customer consents to use of their data for fulfilling the
    // order (s. 14), which covers order processing and support (s. 18).
    let consent = ConsentRecordBuilder::express(
        "order-consent-9001",
        "alex.wong@example.com",
        PurposeOfCollection::OrderProcessing,
        ConsentMethod::ExpressElectronic,
    )
    .data_category(PersonalDataCategory::Name)
    .data_category(PersonalDataCategory::Email)
    .data_category(PersonalDataCategory::Address)
    .data_category(PersonalDataCategory::Phone)
    .build()
    .expect("order consent should build");
    assert!(validate_consent(&consent).is_ok());
    assert!(validate_purpose_limitation(&consent, PurposeOfCollection::CustomerSupport).is_ok());

    // Using the order data for marketing requires fresh consent (s. 18) ...
    assert!(matches!(
        validate_purpose_limitation(&consent, PurposeOfCollection::Marketing),
        Err(PdpaError::PurposeLimitationViolation)
    ));

    // ... and any tele-marketing also needs a DNC check (Part 9). The customer's
    // number is on the No Voice Call Register, so a marketing call is blocked.
    let mut dnc = DncRegistration::new("+6598887777");
    dnc.register(DncRegisterKind::VoiceCall);
    let now = Utc::now();
    let conf = DncCheckConfirmation::at("+6598887777", DncRegisterKind::VoiceCall, now);
    assert!(matches!(
        validate_dnc_before_marketing(
            "+6598887777",
            DncRegisterKind::VoiceCall,
            &dnc,
            Some(&conf),
            now
        ),
        Err(PdpaError::DncViolation { .. })
    ));

    // A cross-border transfer of the order data to a US fulfilment provider is
    // valid only with comparable-protection contractual clauses (s. 26).
    let transfer = DataTransfer::new(
        "order-transfer-9001",
        "United States",
        "Order fulfilment",
        TransferMechanism::ContractualClauses,
    )
    .with_comparable_protection()
    .with_affected_individuals(1);
    assert!(validate_cross_border_transfer(&transfer).is_ok());

    let bad_transfer = DataTransfer::new(
        "order-transfer-bad",
        "United States",
        "Order fulfilment",
        TransferMechanism::ContractualClauses,
    ); // no comparable-protection clause
    assert!(matches!(
        validate_cross_border_transfer(&bad_transfer),
        Err(PdpaError::InadequateTransferProtection { .. })
    ));
}
