# legalis-eu API Reference

`legalis-eu` provides comprehensive modeling of EU law across major regulatory areas,
including GDPR, the AI Act, DSA/DMA, MiFID II/PSD2, ePrivacy, Competition Law, and
Treaty Framework.

**Version**: 0.1.7  
**docs.rs**: <https://docs.rs/legalis-eu>  
**Crate on crates.io**: <https://crates.io/crates/legalis-eu>

---

## Installation

```toml
[dependencies]
legalis-eu = "0.1.7"
```

---

## Feature Flags

| Feature   | Description                                                             | Activates          |
|-----------|-------------------------------------------------------------------------|--------------------|
| `serde`   | `Serialize`/`Deserialize` on all public types via `serde`               | `serde` derives    |
| `schema`  | `JsonSchema` on all public types via `schemars` (implies JSON Schema generation) | `schemars` derives |

Enable features in `Cargo.toml`:

```toml
legalis-eu = { version = "0.1.7", features = ["serde", "schema"] }
```

---

## Module Overview

| Module path                             | Legal area                                              |
|-----------------------------------------|---------------------------------------------------------|
| `legalis_eu::gdpr`                      | GDPR (Regulation 2016/679) — core data protection      |
| `legalis_eu::member_states`             | National GDPR implementations (DE/FR/IT)               |
| `legalis_eu::reasoning`                 | Legal reasoning engine (compliance analysis)            |
| `legalis_eu::ai_regulation`             | EU AI Act (Regulation 2024/1689)                        |
| `legalis_eu::digital_services`          | DSA (2022/2065) and DMA (2022/1925)                     |
| `legalis_eu::financial_services`        | MiFID II (2014/65/EU) and PSD2 (2015/2366/EU)           |
| `legalis_eu::eprivacy`                  | ePrivacy Directive (2002/58/EC)                         |
| `legalis_eu::consumer_rights`           | Consumer Rights Directive (2011/83/EU) + UCPD           |
| `legalis_eu::competition`               | Competition Law (Articles 101–102 TFEU)                 |
| `legalis_eu::treaty`                    | TFEU / TEU / Charter — treaty framework                 |
| `legalis_eu::intellectual_property`     | EU trademarks, copyright, designs, trade secrets        |
| `legalis_eu::citation`                  | EUR-Lex / CELEX citation system                         |
| `legalis_eu::i18n`                      | Multilingual legal text (24 EU languages)               |
| `legalis_eu::shared`                    | Shared types (`MemberState`)                            |

All top-level types are re-exported from the crate root so that
`use legalis_eu::*;` gives direct access without traversing module paths.

---

## GDPR Core API

The GDPR API is rooted at `legalis_eu::gdpr` and all types are also re-exported
from `legalis_eu` for convenience.

### Data Processing (Article 6) — `DataProcessing` builder

```rust
use legalis_eu::gdpr::*;

let processing = DataProcessing::new()
    .with_controller("Acme Corp")
    .with_purpose("Marketing emails")
    .add_data_category(PersonalDataCategory::Regular("email".to_string()))
    .with_lawful_basis(LawfulBasis::Consent {
        freely_given: true,
        specific: true,
        informed: true,
        unambiguous: true,
    });

match processing.validate() {
    Ok(v) if v.is_compliant() => println!("Processing is GDPR compliant"),
    Ok(_) => println!("Partially compliant"),
    Err(e) => println!("Validation error: {}", e),
}
```

**Key types**

| Type                   | Description                                          |
|------------------------|------------------------------------------------------|
| `DataProcessing`       | Builder for a single processing activity             |
| `ProcessingValidation` | Outcome of `DataProcessing::validate()`              |
| `LawfulBasis`          | The six bases under Article 6(1)(a)–(f)              |
| `PersonalDataCategory` | `Regular(String)` or `Special(SpecialCategory)`      |
| `ProcessingOperation`  | Collection, Storage, Erasure, CrossBorderTransfer, … |
| `ComplianceStatus`     | `Compliant` / `NonCompliant` / `PartiallyCompliant`  |

**`LawfulBasis` variants (Article 6(1))**

```rust
// Article 6(1)(a) - Consent
LawfulBasis::Consent { freely_given, specific, informed, unambiguous }

// Article 6(1)(b) - Contract
LawfulBasis::Contract { necessary_for_performance: true }

// Article 6(1)(c) - Legal obligation
LawfulBasis::LegalObligation { eu_law: Some("GDPR".into()), member_state_law: None }

// Article 6(1)(d) - Vital interests
LawfulBasis::VitalInterests { life_threatening: true }

// Article 6(1)(e) - Public task
LawfulBasis::PublicTask { task_basis: "Electoral roll management".into() }

// Article 6(1)(f) - Legitimate interests (requires balancing test)
LawfulBasis::LegitimateInterests {
    controller_interest: "Fraud prevention".into(),
    balancing_test_passed: true,
}
```

---

### Special Categories (Article 9)

```rust
use legalis_eu::gdpr::*;

let special = Article9Processing::new()
    .with_category(SpecialCategory::HealthData)
    .with_exception(Article9Exception::ExplicitConsent {
        purposes: vec!["medical treatment".into()],
        consent_documented: true,
    });

let result = special.validate();
assert!(result.is_ok());
```

**Key types**

| Type                  | Description                                      |
|-----------------------|--------------------------------------------------|
| `Article9Processing`  | Builder for special-category processing          |
| `Article9Exception`   | 10 exceptions under Article 9(2)(a)–(j)          |
| `Article9Validation`  | Outcome of validation                            |
| `SpecialCategory`     | Health, Genetic, Biometric, RacialEthnicOrigin, … |
| `HealthcarePurpose`   | Purpose sub-type for healthcare processing        |
| `ResearchPurpose`     | Purpose sub-type for scientific research          |

---

### Data Subject Rights (Articles 15–22)

```rust
use legalis_eu::gdpr::*;

let request = DataSubjectRequest::new()
    .with_data_subject("user@example.com")
    .with_right(DataSubjectRight::Erasure)
    .with_controller("Acme Corp")
    .with_grounds("No longer necessary for original purpose");

let validation = request.validate().expect("valid request");
// response_deadline_days() → 30 (one calendar month)
```

**`DataSubjectRight` variants**

- `Access` (Art. 15) — copy of personal data
- `Rectification` (Art. 16) — correct inaccurate data
- `Erasure` (Art. 17) — right to be forgotten
- `RestrictionOfProcessing` (Art. 18) — restrict but not erase
- `DataPortability` (Art. 20) — machine-readable export
- `Objection` (Art. 21) — object to processing
- `AutomatedDecisionMaking` (Art. 22) — human review of automated decisions

---

### Security and Breach Notification (Articles 32–34)

```rust
use legalis_eu::gdpr::*;

let assessment = SecurityAssessment::new()
    .with_entity("Acme Corp")
    .add_technical_measure(TechnicalMeasure::Encryption {
        data_at_rest: true,
        data_in_transit: true,
        algorithm: "AES-256".into(),
    })
    .add_organizational_measure(OrganizationalMeasure::AccessControl {
        role_based: true,
        least_privilege: true,
    })
    .with_risk_level(SecurityRiskLevel::High);

let validation = assessment.validate().expect("valid assessment");
```

**Key types**

| Type                           | Description                                    |
|--------------------------------|------------------------------------------------|
| `SecurityAssessment`           | Builder for Article 32 TOMs assessment         |
| `SecurityValidation`           | Outcome of security assessment                 |
| `TechnicalMeasure`             | Encryption, Pseudonymisation, AccessControl, … |
| `OrganizationalMeasure`        | Policies, training, audit, incident response   |
| `DataBreach`                   | Personal data breach under Articles 33–34      |
| `BreachNotificationRequirements` | 72-hour SA notification + data subject comms |
| `SecurityRiskLevel`            | Low / Medium / High / VeryHigh                 |

---

### Controller Accountability (Article 24)

```rust
use legalis_eu::gdpr::*;

let accountability = ControllerAccountability::new()
    .with_controller_name("Acme Corp")
    .add_measure(AccountabilityMeasure::DataProtectionPolicy {
        policy_name: "Privacy Policy v2.0".into(),
        last_reviewed: chrono::Utc::now(),
    });

let validation = accountability.validate().expect("valid");
```

**Key types**: `ControllerAccountability`, `Article24Validation`, `AccountabilityMeasure`,
`DataSensitivity`, `DataVolume`, `ComplianceCertification`.

---

### Data Protection by Design/Default (Article 25)

```rust
use legalis_eu::gdpr::*;

let design = DataProtectionByDesign::new()
    .add_principle(DesignPrinciple::DataMinimisation)
    .add_privacy_technology(PrivacyEnhancingTechnology::Pseudonymisation)
    .with_default_setting(DefaultSetting::MinimumDataCollection);

let validation = design.validate().expect("valid");
```

**Key types**: `DataProtectionByDesign`, `Article25Validation`, `DesignPrinciple`,
`PrivacyEnhancingTechnology`, `DefaultSetting`.

---

### Joint Controllers (Article 26)

**Key types**: `JointControllerArrangement`, `JointController`, `JointControllerArrangement`,
`JointControllershipBasis`, `Responsibility`, `Article26Validation`.

---

### Processor Contracts (Article 28)

```rust,no_run
use legalis_eu::gdpr::processor_contract::*;

let contract = ProcessorContract::new()
    .with_controller("Acme Corp", "dpo@acme.com")
    .with_processor("CloudService GmbH", "processor@cloudservice.de")
    .with_subject_matter("Customer data processing and storage")
    .with_duration_months(24)
    .with_clause(Article28Clause::ProcessOnlyOnInstructions)
    .with_clause(Article28Clause::ConfidentialityObligation)
    .with_clause(Article28Clause::SecurityMeasures)
    .with_clause(Article28Clause::DeletionOrReturn)
    .with_clause(Article28Clause::AuditsAndInspections);

match contract.validate() {
    Ok(v) if v.compliant => println!("Contract complies with Article 28"),
    Ok(_) => println!("Contract needs amendments"),
    Err(e) => println!("Error: {}", e),
}
```

**Key types**: `ProcessorContract`, `ProcessorContractValidation`, `Article28Clause`,
`SubProcessor`, `SubProcessorAuthorization`, `ContractParty`, `ContractDuration`.

---

### Records of Processing Activities (Article 30)

```rust
use legalis_eu::gdpr::ropa::*;
use legalis_eu::gdpr::types::{PersonalDataCategory, ProcessingOperation};

let record = ProcessingRecord::new()
    .with_name("CRM")
    .with_controller("Acme Corp", "privacy@acme.com")
    .with_purpose("Customer service")
    .add_data_category(PersonalDataCategory::Regular("email".into()))
    .with_retention_period("7 years after last contact");

let ropa = RecordsOfProcessingActivities::new()
    .add_record(record.validate().expect("valid record"));
```

**Key types**: `RecordsOfProcessingActivities`, `ProcessingRecord`, `RecordValidation`,
`RopaValidation`, `RopaExemption`, `ThirdCountryTransfer`, `ContactDetails`.

---

### Data Protection Impact Assessment (Articles 35–36)

```rust
use legalis_eu::gdpr::dpia::*;

let dpia = DataProtectionImpactAssessment::new()
    .with_processing_description("AI-powered recruitment screening")
    .with_purpose("Automated candidate evaluation")
    .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
        produces_legal_effects: true,
        systematic: true,
        extensive: true,
    })
    .add_risk(RiskAssessment {
        risk_type: RiskType::Discrimination,
        likelihood: Likelihood::High,
        severity: Severity::High,
        description: "AI may exhibit bias against protected groups".into(),
    })
    .add_mitigation(Mitigation {
        risk_addressed: RiskType::Discrimination,
        measure: "Regular algorithmic fairness audits".into(),
        effectiveness: Effectiveness::High,
    });

match dpia.validate() {
    Ok(result) if result.prior_consultation_required =>
        println!("Must consult supervisory authority before processing"),
    Ok(_) => println!("DPIA complete"),
    Err(e) => println!("DPIA incomplete: {}", e),
}
```

**Key types**: `DataProtectionImpactAssessment`, `DpiaTrigger`, `DpiaValidation`,
`RiskAssessment`, `RiskType`, `Likelihood`, `Severity`, `Mitigation`, `Effectiveness`.

---

### Data Protection Officer (Articles 37–39)

**Key types**: `DpoDesignation`, `DpoValidation`, `DpoTask`, `DpoEntityType`,
`DpoQualification`, `DpoRequirementResult`, `DpoContactDetails`, `CoreActivity`,
`ProcessingScale`, `MonitoringType`.

---

### Cross-Border Transfers (Chapter V, Articles 44–49)

```rust
use legalis_eu::gdpr::cross_border::*;

let transfer = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses {
        version: "2021".into(),
        clauses_signed: true,
    });

match transfer.validate() {
    Ok(v) => println!("Transfer validation: {:?}", v),
    Err(e) => println!("Transfer not allowed: {}", e),
}
```

**Key types**: `CrossBorderTransfer`, `CrossBorderTransferValidation`, `AdequateCountry`,
`TransferSafeguard`, `TransferDerogation`, `TransferLegalBasis`, `Article49Derogation`.

---

### Administrative Fines (Article 83)

```rust
use legalis_eu::gdpr::*;

let factors = Article83Factors {
    duration_months: Some(6),
    data_subjects_affected: Some(50_000),
    damage_suffered: Some(100_000.0),
    intentional: true,
    previous_violations: false,
    cooperation_with_sa: true,
    notified_breach: true,
    ..Default::default()
};

let fine = AdministrativeFine::new()
    .with_violation(ViolatedArticle::Article6LawfulBasis)
    .with_annual_turnover(10_000_000.0)
    .with_factors(factors);

let calc: FineCalculation = fine.calculate();
// calc.tier → FineTier::UpperTier  (up to €20M or 4% global turnover)
```

**Key types**: `AdministrativeFine`, `FineCalculation`, `FineTier`, `ViolatedArticle`,
`Article83Factors`.

---

### Audit and Accountability

The `legalis_eu::gdpr::audit` module integrates with `legalis-audit` to provide
GDPR-compliant audit trails. The example below is for illustration (requires runtime
context and storage backend — use `no_run` in doctests):

```rust,no_run
use legalis_eu::gdpr::audit::{GdprAuditTrail, GdprDecisionRecord};
use legalis_eu::gdpr::{LawfulBasis, PersonalDataCategory};

let mut audit = GdprAuditTrail::new();
// record_decision(), handle_dsar(), explain_decision() operate on the trail.
```

**Key types**: `GdprAuditTrail`, `GdprDecisionRecord`, `LawfulBasisMetadata`,
`DsarResponse`, `Article22Explanation`, `GdprActor`, `ProcessingActivitySummary`,
`RetentionCandidate`, `DecisionSignificance`.

---

## Key Types and Enums

| Type                   | Module                      | Description                                   |
|------------------------|-----------------------------|-----------------------------------------------|
| `LawfulBasis`          | `gdpr::types`               | Article 6(1)(a)–(f) lawful bases              |
| `ComplianceStatus`     | `gdpr::types`               | Compliant / NonCompliant / PartiallyCompliant |
| `PersonalDataCategory` | `gdpr::types`               | Regular or Special                            |
| `SpecialCategory`      | `gdpr::types`               | Health, Genetic, Biometric, etc.              |
| `DataSubjectRight`     | `gdpr::types`               | Access, Erasure, Portability, etc.            |
| `FineTier`             | `gdpr::article83`           | LowerTier (2%) / UpperTier (4%)               |
| `CrossBorderMechanism` | `gdpr::types`               | AdequacyDecision, SCCs, BCRs, Derogation      |
| `MemberState`          | `shared`                    | All 27 EU member states + EEA                 |
| `GdprError`            | `gdpr::error`               | All GDPR validation errors                    |

---

## Legal Reasoning Engine

```rust
use legalis_eu::reasoning::*;
use legalis_eu::gdpr::types::DataController;

let engine = LegalReasoningEngine::new();

let controller = DataController {
    id: "acme-001".into(),
    name: "Acme Corp".into(),
    established_in_eu: true,
    dpo_appointed: true,
};

let analysis: LegalAnalysis = engine
    .analyze_gdpr_compliance(
        &controller,
        true,  // has_lawful_basis
        true,  // has_consent_mechanism
        true,  // has_security_measures
    )
    .expect("analysis succeeds");

for step in &analysis.reasoning_steps {
    println!("{}: {:?}", step.statute_id, step.result);
}
```

**Key types**

| Type                         | Description                                           |
|------------------------------|-------------------------------------------------------|
| `LegalReasoningEngine`       | Main analysis engine; wraps a `StatuteRegistry`       |
| `LegalAnalysis`              | Full analysis result with steps, violations, status   |
| `ReasoningStep`              | Single statute evaluation step                        |
| `Violation`                  | A specific legal violation with remediation guidance  |
| `ViolationSeverity`          | Advisory / Minor / Major / Critical                   |
| `ComplianceStatus` (reasoning) | Compliant / NonCompliant / PartiallyCompliant / Indeterminate |
| `RiskLevel` (reasoning)      | None / Low / Medium / High / Critical                 |

**Statute adapters** (return `Vec<Statute>` for the registry):
- `all_eu_statutes()` — all modelled EU statutes
- `gdpr_statutes()` — GDPR statutes only
- `competition_statutes()` — TFEU Articles 101–102
- `consumer_rights_statutes()` — CRD statutes

---

## Member-State Layer

```rust
use legalis_eu::member_states::{self, NationalGdprQuery};
use legalis_eu::shared::MemberState;

// Resolve the effective age of digital consent (Article 8(1) GDPR)
assert_eq!(member_states::effective_age_of_digital_consent(MemberState::France), 15);
assert_eq!(member_states::effective_age_of_digital_consent(MemberState::Italy), 14);
assert_eq!(member_states::effective_age_of_digital_consent(MemberState::Germany), 16);

// Use the query facade for a specific state
let query = NationalGdprQuery::new(MemberState::Germany).expect("implemented");
assert_eq!(query.supervisory_authority().abbreviation, "BfDI");
assert!(!query.child_can_consent(15));  // Germany's threshold is 16

// Assess child consent combined with national specifics
let assessment = member_states::combined_consent_assessment(MemberState::Italy, 14);
assert!(assessment.child_can_consent);
assert!(assessment.national_implementation_applied);
```

**Key types**

| Type                        | Description                                                    |
|-----------------------------|----------------------------------------------------------------|
| `MemberStateGdpr`           | National GDPR implementation (SA, age of consent, derogations)|
| `MemberStateGdprBuilder`    | Builder for `MemberStateGdpr`                                  |
| `SupervisoryAuthority`      | Name, abbreviation, country, website                           |
| `NationalDerogation`        | Specific national law invoking a GDPR opening clause           |
| `OpeningClause`             | GDPR opening clause (Article 8, 9, 17, 88, etc.)               |
| `NationalActCitation`       | National act citation (e.g. `§ 26 BDSG`)                      |
| `NationalGdprQuery`         | Query facade combining core GDPR with national specifics       |
| `CombinedConsentAssessment` | Result of `combined_consent_assessment()`                      |
| `TranspositionTracker`      | Tracks directive-to-national-act transposition progress        |
| `TranspositionRecord`       | A single directive's transposition record                      |
| `TranspositionStatus`       | Implemented / Partial / NotImplemented / Delayed               |
| `DirectiveReference`        | EUR-Lex reference for a directive                              |

**Free functions** (also re-exported at crate root):
- `member_state_implementation(state)` — `for_state()` alias
- `effective_age_of_digital_consent(state)` — Article 8(1) age
- `combined_consent_assessment(state, age)` — combined assessment

Currently modelled states: **Germany** (BDSG), **France** (Loi Informatique et Libertés),
**Italy** (Codice Privacy). Other states fall back to GDPR defaults.

---

## Other Regulatory Modules

### EU AI Act (`legalis_eu::ai_regulation`)

```rust,no_run
use legalis_eu::ai_regulation::*;

let system = AiSystem {
    system_id: "HR-AI-001".to_string(),
    name: "Resume Screening AI".to_string(),
    description: "AI system for automated resume screening".to_string(),
    provider: "HRTech Inc".to_string(),
    deployer: Some("BigCorp".to_string()),
    intended_purpose: "Screen and rank job applicants".to_string(),
    risk_level: RiskLevel::HighRisk {
        category: HighRiskCategory::Employment {
            use_case: "recruitment".to_string(),
        },
    },
    adaptive: true,
    market_placement_date: None,
    conformity_status: ConformityStatus::NotAssessed,
};

let validation = validate_ai_system(&system).expect("validation succeeds");
// validation.applicable_requirements lists Articles 9-15 obligations
assert!(!validation.is_compliant()); // Requires conformity assessment
```

Key types: `AiSystem`, `AiRiskLevel`, `ProhibitedPractice`, `HighRiskCategory`,
`HighRiskRequirements`, `GeneralPurposeAiModel`, `ConformityStatus`,
`TransparencyObligation`, `LimitedRiskType`, `HumanOversight`, `AiActValidationResult`.

---

### Digital Services Act / Digital Markets Act (`legalis_eu::digital_services`)

Key types: `PlatformType`, `QuantitativeThresholds`, `DsaValidationResult`,
`SystemicRisk`, `IllegalContent`, `IllegalContentNotice`, `NoticeDecision`,
`NoticeResponse`, `StatementOfReasons`, `ModerationDecision`, `TransparencyReport`,
`AlgorithmicTransparency`, `GatekeeperDesignation`, `CorePlatformService`,
`GatekeeperObligation`, `InteroperabilityRequirement`, `DmaComplianceReport`.

---

### Financial Services — MiFID II and PSD2 (`legalis_eu::financial_services`)

Key types: `ClientCategory`, `InvestmentService`, `BestExecutionPolicy`,
`ConductOfBusiness`, `StrongCustomerAuthentication`, `AuthenticationElement`,
`ScaExemption`, `PaymentService`, `OpenBankingApi`, `ThirdPartyProvider`,
`PaymentInitiationProvider`, `AccountInformationProvider`, `Passport`.

---

### ePrivacy Directive (`legalis_eu::eprivacy`)

```rust
use legalis_eu::eprivacy::*;
use legalis_eu::eprivacy::types::CookieInformation;

// Strictly necessary cookie (exempt from consent)
let session_cookie = CookieConsent {
    category: CookieCategory::StrictlyNecessary,
    purpose: "Maintain user session".to_string(),
    duration: CookieDuration::Session,
    consent_obtained: false,
    consent_timestamp: None,
    exempt: true,
    exemption_reason: Some(CookieExemption::StrictlyNecessaryForService),
};

// Analytics cookie (requires prior consent)
let analytics_cookie = CookieConsent {
    category: CookieCategory::Performance,
    purpose: "Measure website performance".to_string(),
    duration: CookieDuration::Persistent { days: 365 },
    consent_obtained: true,
    consent_timestamp: None,
    exempt: false,
    exemption_reason: None,
};

// GDPR-compliant cookie banner configuration
let banner = CookieBanner {
    shown_before_cookies: true,
    granular_control: true,
    accept_reject_all: true,
    cookie_wall: false,
    information_provided: CookieInformation {
        purpose_explained: true,
        duration_disclosed: true,
        third_parties_identified: true,
        cookie_policy_link: true,
    },
};
```

Key types: `CookieConsent`, `CookieBanner`, `CookieCategory`, `CookieDuration`,
`CookieExemption`, `DirectMarketing`, `MarketingChannel`, `LocationDataProcessing`.

---

### Consumer Rights and UCPD (`legalis_eu::consumer_rights`)

Key types: `DistanceContract`, `OffPremisesContract`, `WithdrawalRight`,
`WithdrawalPeriod`, `WithdrawalException`, `ContractType`, `MisleadingAction`,
`MisleadingOmission`, `AggressivePractice`, `ConsumerProhibitedPractice`,
`UnfairCommercialPractice`.

---

### Competition Law (`legalis_eu::competition`)

Key types: `Article101Agreement`, `Article101Exemption`, `Article101Validation`,
`Article102Conduct`, `Article102Validation`, `DominanceAssessment`, `RelevantMarket`,
`GeographicMarket`, `AbuseType`, `ExclusionaryAbuse`, `ExploitativeAbuse`,
`MarketAllocation`, `ConcertedPractice`, `Undertaking`.

---

### Treaty Framework (`legalis_eu::treaty`)

Key types: `TreatyArticle`, `TreatyProvision`, `TreatyType`, `FourFreedom`,
`FreedomType`, `FundamentalRight`, `CharterArticle`, `CjeuCase`, `CjeuPrinciple`,
`LandmarkCase`, `Restriction`, `JustificationGround`.

---

### Intellectual Property (`legalis_eu::intellectual_property`)

Key types: `EuTrademark`, `TrademarkValidation`, `TrademarkStatus`, `MarkType`,
`NiceClass`, `CopyrightWork`, `WorkType`, `CopyrightValidation`, `CopyrightException`,
`CommunityDesign`, `DesignType`, `DesignValidation`, `TradeSecret`,
`TradeSecretCharacteristics`, `TradeSecretValidation`, `MisappropriationAnalysis`,
`AcquisitionMethod`.

---

## Errors

| Error type             | Module                   | Description                                    |
|------------------------|--------------------------|------------------------------------------------|
| `GdprError`            | `gdpr::error`            | GDPR validation errors (missing fields, invalid consent, …) |
| `MemberStateError`     | `member_states::error`   | No national implementation, config errors      |
| `ReasoningError`       | `reasoning`              | Legal reasoning engine errors                  |
| `CompetitionError`     | `competition`            | Competition law validation errors              |
| `ConsumerRightsError`  | `consumer_rights`        | Consumer rights validation errors              |
| `IpError`              | `intellectual_property`  | IP validation errors                           |
| `DigitalServicesError` | `digital_services`       | DSA/DMA validation errors                      |
| `AiRegulationError`    | `ai_regulation`          | AI Act validation errors                       |
| `FinancialServicesError` | `financial_services`   | MiFID II / PSD2 validation errors              |

---

## i18n / Multilingual Legal Text

```rust
use legalis_eu::MultilingualText;

let text = MultilingualText::from_eurlex(
    "Data Controller".into(),
    "Verantwortlicher".into(),
    "CELEX:32016R0679".into(),
);

assert_eq!(text.in_language("en"), "Data Controller");
assert_eq!(text.in_language("de"), "Verantwortlicher");
// Falls back to English for unsupported languages:
assert_eq!(text.in_language("fr"), "Data Controller");
```

The EU has 24 official languages; this crate currently provides English (primary)
and German translations. French, Italian, and other languages are in progress.

---

## Citation System

```rust
use legalis_eu::citation::{EuCitation, EuLegalInstrument};

let cite = EuCitation::gdpr_article(6, Some(vec![1, 'a' as u8]));
// CELEX: "32016R0679"
// Human-readable: "GDPR Art. 6(1)(a)"

let instrument = EuLegalInstrument::Regulation {
    celex: "32016R0679".into(),
    short_name: "GDPR".into(),
};
```

---

## docs.rs Link

Full generated API documentation is at:

**<https://docs.rs/legalis-eu/0.1.7/legalis_eu/>**

Sub-module documentation:
- `legalis_eu::gdpr` — <https://docs.rs/legalis-eu/latest/legalis_eu/gdpr/>
- `legalis_eu::member_states` — <https://docs.rs/legalis-eu/latest/legalis_eu/member_states/>
- `legalis_eu::reasoning` — <https://docs.rs/legalis-eu/latest/legalis_eu/reasoning/>
- `legalis_eu::ai_regulation` — <https://docs.rs/legalis-eu/latest/legalis_eu/ai_regulation/>
- `legalis_eu::digital_services` — <https://docs.rs/legalis-eu/latest/legalis_eu/digital_services/>
- `legalis_eu::financial_services` — <https://docs.rs/legalis-eu/latest/legalis_eu/financial_services/>
