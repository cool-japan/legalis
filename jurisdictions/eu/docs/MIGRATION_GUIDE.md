# Migration Guide: Manual Compliance → legalis-eu

A practical guide for teams migrating from spreadsheets, checklists, and manual GDPR compliance
workflows to the typed `legalis-eu` API.

## Table of Contents

1. [Overview](#1-overview)
2. [Why Migrate?](#2-why-migrate)
3. [Migration Prerequisites](#3-migration-prerequisites)
4. [Migration Map](#4-migration-map)
5. [Step 1: Lawful Basis Assessment (Art. 6)](#5-step-1-lawful-basis-assessment-art-6)
6. [Step 2: Consent Management (Art. 7)](#6-step-2-consent-management-art-7)
7. [Step 3: Records of Processing Activities (Art. 30)](#7-step-3-records-of-processing-activities-art-30)
8. [Step 4: Data Subject Rights Handling (Arts. 15–22)](#8-step-4-data-subject-rights-handling-arts-1522)
9. [Step 5: Data Protection Impact Assessments (Art. 35)](#9-step-5-data-protection-impact-assessments-art-35)
10. [Step 6: Audit Trail & Accountability](#10-step-6-audit-trail--accountability)
11. [Common Migration Pitfalls](#11-common-migration-pitfalls)
12. [Complete Migration Checklist](#12-complete-migration-checklist)

---

## 1. Overview

Most organizations begin GDPR compliance with spreadsheets: a "lawful basis register," a "DSAR log,"
a "ROPA spreadsheet," and DPIAs in Word documents. These artifacts work well at first, but as the
organization grows they become:

- **Error-prone** — a typo in "Legimate Interests" is invisible in a spreadsheet cell.
- **Stale** — nobody updates the retention column when the retention policy changes.
- **Disconnected** — the lawful basis spreadsheet and the ROPA are maintained by different teams with
  no shared schema.
- **Untestable** — you cannot write a CI pipeline that validates a Google Sheet.

`legalis-eu` replaces those artifacts with Rust types. The GDPR's requirements become compile-time
constraints. Invalid states—consent that is not freely given, a ROPA record missing a retention
period—become compiler errors or explicit `Result::Err` values that must be handled.

---

## 2. Why Migrate?

| Pain point (manual) | How legalis-eu solves it |
|---|---|
| Spreadsheet columns drift from article text | Enum variants map 1:1 to GDPR articles |
| Wrong lawful basis silently accepted | `LawfulBasis` enum makes invalid bases unrepresentable |
| DSAR deadline missed | `RequestValidation.deadline_days` is always present |
| ROPA exemption wrongly claimed | `RecordsOfProcessingActivities::is_exempt` applies all three criteria |
| DPIA not triggered when required | `DpiaTrigger` variants encode Art. 35(3) triggers exactly |
| Audit trail stored in a folder of PDFs | `GdprAuditTrail` provides structured, queryable records |
| Special category data handled the same as regular data | `PersonalDataCategory::Special(SpecialCategory::...)` forces Art. 9 path |

---

## 3. Migration Prerequisites

Add `legalis-eu` to your workspace's `Cargo.toml`:

```toml
[dependencies]
legalis-eu = { workspace = true }
chrono = { workspace = true }
```

Typical imports for a compliance module:

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::ropa::*;
use legalis_eu::gdpr::dpia::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{
    DataSubjectRight,
    LawfulBasis,
    PersonalDataCategory,
    ProcessingOperation,
    SpecialCategory,
};
use chrono::Utc;
```

All public validation functions return `Result<_, GdprError>`. Use `?` in functions that return
`Result`, or `match` at the boundary between your compliance code and the rest of the application.

---

## 4. Migration Map

The table below maps every common manual compliance artifact to the corresponding `legalis-eu`
type or function.

| Manual artifact | legalis-eu equivalent |
|---|---|
| "Lawful basis" spreadsheet column | `LawfulBasis` enum (6 variants, one per Art. 6(1) ground) |
| "Data category" column (free text) | `PersonalDataCategory::Regular(String)` |
| "Special category" flag column | `PersonalDataCategory::Special(SpecialCategory)` |
| "Processing operation" column (e.g. "collect/store") | `ProcessingOperation` enum (14 variants) |
| Consent checkbox with timestamp | `LawfulBasis::Consent { freely_given, specific, informed, unambiguous }` |
| Consent record log | `ConsentRecord { purpose, timestamp, freely_given, specific, informed, unambiguous }` |
| Contract basis note | `LawfulBasis::Contract { necessary_for_performance: true }` |
| Legal obligation citation | `LawfulBasis::LegalObligation { eu_law, member_state_law }` |
| Legitimate interest assessment (LIA) | `LawfulBasis::LegitimateInterests { controller_interest, balancing_test_passed }` |
| ROPA spreadsheet (Art. 30) | `RecordsOfProcessingActivities` + `ProcessingRecord` |
| ROPA exemption analysis | `RecordsOfProcessingActivities::is_exempt(employee_count)` → `RopaExemption` |
| Controller vs. processor column | `EntityType::Controller` / `EntityType::Processor` |
| Third-country transfer row | `ThirdCountryTransfer { country, safeguard, documentation }` |
| DSAR intake form | `DataSubjectRequest` builder |
| DSAR right column | `DataSubjectRight` enum (7 variants, Arts. 15–22) |
| DSAR response deadline tracker | `RequestValidation.deadline_days` (always 30) |
| DPIA Word document | `DataProtectionImpactAssessment` builder |
| DPIA trigger checklist | `DpiaTrigger` enum (Art. 35(3) triggers) |
| DPIA risk register row | `RiskAssessment { risk_type, likelihood, severity, description }` |
| DPIA mitigation row | `Mitigation { risk_addressed, measure, effectiveness }` |
| DPIA outcome (proceed / consult SA) | `DpiaValidation.prior_consultation_required` |
| Security measures checklist (Art. 32) | `SecurityAssessment` + `TechnicalMeasure` + `OrganizationalMeasure` |
| Processor agreement tracker (Art. 28) | `ProcessorContract` builder |
| Accountability framework (Art. 24) | `ControllerAccountability` builder |
| Compliance score / report | `AccountabilityValidation.compliance_score` |
| Audit trail (folder of PDF records) | `GdprAuditTrail` + `GdprDecisionRecord` |

---

## 5. Step 1: Lawful Basis Assessment (Art. 6)

### Before (manual)

Teams typically maintain a register spreadsheet with columns similar to:

```
| Processing Activity | Data Category | Lawful Basis         | Notes                        |
|---------------------|---------------|----------------------|------------------------------|
| Order processing    | Name, address | Contract performance | Necessary for delivery       |
| Email marketing     | Email         | Consent              | Double opt-in, May 2024      |
| Fraud detection     | IP, history   | Legit. interests     | LIA not yet completed        |
| Employee payroll    | Bank account  | Legal obligation     | German tax law § 41b EStG    |
```

Problems:
- "Legit. interests" with "LIA not yet completed" is silently accepted.
- No validation that consent was actually freely given, specific, informed, and unambiguous.
- The "Notes" column carries load-bearing compliance information with no schema.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{LawfulBasis, PersonalDataCategory, ProcessingOperation};

fn assess_lawful_basis() -> Result<(), GdprError> {
    // Contract performance — order processing
    let order_processing = DataProcessing::new()
        .with_controller("Acme GmbH")
        .with_purpose("Process customer orders and arrange delivery")
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .add_data_category(PersonalDataCategory::Regular("shipping address".to_string()))
        .with_operation(ProcessingOperation::Collection)
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    let _validation = order_processing.validate()?;

    // Legitimate interests — requires balancing test to be complete
    let fraud_detection = DataProcessing::new()
        .with_controller("Acme GmbH")
        .with_purpose("Fraud detection and prevention")
        .add_data_category(PersonalDataCategory::Regular("IP address".to_string()))
        .add_data_category(PersonalDataCategory::Regular("transaction history".to_string()))
        .with_lawful_basis(LawfulBasis::LegitimateInterests {
            controller_interest: "Preventing fraudulent transactions to protect customers"
                .to_string(),
            // balancing_test_passed: false  ← surfaces a JudicialDiscretion result,
            // forcing the compliance team to complete the LIA before proceeding
            balancing_test_passed: false,
        });

    match fraud_detection.validate() {
        Ok(v) => {
            use legalis_core::LegalResult;
            if let LegalResult::JudicialDiscretion { issue, narrative_hint, .. } =
                &v.lawful_basis_valid
            {
                eprintln!("LIA incomplete: {}", issue);
                if let Some(hint) = narrative_hint {
                    eprintln!("Guidance: {}", hint);
                }
                // Do NOT proceed until LIA is complete
            }
        }
        Err(e) => return Err(e),
    }

    // Legal obligation — employee payroll
    let payroll = DataProcessing::new()
        .with_controller("Acme GmbH")
        .with_purpose("Employee payroll administration")
        .add_data_category(PersonalDataCategory::Regular("bank account".to_string()))
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::LegalObligation {
            eu_law: None,
            member_state_law: Some("German EStG § 41b (payroll tax filing)".to_string()),
        });

    let _validation = payroll.validate()?;

    Ok(())
}
```

### Benefits

- `LawfulBasis::LegitimateInterests { balancing_test_passed: false }` produces a
  `LegalResult::JudicialDiscretion` value — not a silent spreadsheet cell. The compliance team
  cannot overlook it.
- `LawfulBasis::LegalObligation` requires naming the specific EU or Member State law, eliminating
  vague entries like "Required by law."
- `DataProcessing::validate()` returns `Result<_, GdprError>` — the calling code must handle
  non-compliant states explicitly.

---

## 6. Step 2: Consent Management (Art. 7)

### Before (manual)

Consent is often tracked in a CRM or a separate spreadsheet with columns such as:

```
| User ID | Email              | Marketing Consent | Consent Date | Withdrawal Date |
|---------|--------------------|-------------------|--------------|-----------------|
| 1001    | alice@example.com  | YES               | 2024-03-01   |                 |
| 1002    | bob@example.com    | YES (forced)      | 2024-03-02   |                 |  ← invalid
| 1003    | carol@example.com  | YES               | 2024-03-03   | 2024-06-15      |
```

Problems:
- "YES (forced)" is textually different from "YES" but structurally identical to the system.
- No machine-readable record of *why* consent was given (which purpose, on what legal basis).
- Withdrawal tracking is a separate column with no enforcement.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{ConsentRecord, LawfulBasis, PersonalDataCategory};
use legalis_eu::gdpr::types::ProcessingOperation;
use chrono::Utc;

fn record_valid_consent() -> Result<(), GdprError> {
    // The LawfulBasis::Consent variant enforces Art. 7 requirements structurally.
    // Every field is a bool — you cannot store "YES (forced)" here.
    let marketing = DataProcessing::new()
        .with_controller("Acme GmbH")
        .with_purpose("Email marketing for new product announcements")
        .add_data_category(PersonalDataCategory::Regular("email address".to_string()))
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .with_operation(ProcessingOperation::Collection)
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,     // Not bundled, not coerced
            specific: true,         // Scoped to "marketing" only
            informed: true,         // Privacy notice shown at point of consent
            unambiguous: true,      // Active opt-in (no pre-ticked boxes)
        });

    let validation = marketing.validate()?;

    if validation.is_compliant() {
        // Safe to proceed with marketing emails
    }

    Ok(())
}

fn record_invalid_consent() -> Result<(), GdprError> {
    // Coerced consent — e.g. consent bundled with terms of service
    let coerced = DataProcessing::new()
        .with_controller("Acme GmbH")
        .with_purpose("Marketing")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: false, // Bundled with service registration
            specific: true,
            informed: true,
            unambiguous: true,
        });

    // validate() returns Err — processing must not start
    match coerced.validate() {
        Ok(_) => { /* should not reach here */ }
        Err(e) => {
            eprintln!("Consent invalid: {}", e);
            // Return error upstream; do NOT proceed with processing
            return Err(e);
        }
    }

    Ok(())
}

fn consent_record_for_audit(user_id: &str) -> ConsentRecord {
    // ConsentRecord provides a structured, auditable consent log entry
    ConsentRecord {
        purpose: "Email marketing for new product announcements".to_string(),
        timestamp: Utc::now(),
        freely_given: true,
        specific: true,
        informed: true,
        unambiguous: true,
    }
}

fn check_consent_still_valid(record: &ConsentRecord) -> bool {
    record.is_valid()
}
```

### Benefits

- `LawfulBasis::Consent` has four boolean fields matching Art. 7 requirements exactly. A compliance
  auditor reading the code sees the same requirements as reading the Regulation.
- `ConsentRecord::is_valid()` is a deterministic check — no regex against a spreadsheet cell.
- Withdrawal is modelled by removing or invalidating the `ConsentRecord`; the absence of a valid
  record is the system's natural representation of withdrawal.

---

## 7. Step 3: Records of Processing Activities (Art. 30)

### Before (manual)

A ROPA spreadsheet typically has one row per processing activity with columns:

```
| Activity Name       | Controller | DPO Email       | Purpose          | Legal Basis  | Data Categories     | Recipients        | Retention | Security Measures       | Transfer Countries |
|---------------------|------------|-----------------|------------------|--------------|---------------------|-------------------|-----------|-------------------------|--------------------|
| Customer orders     | Acme GmbH  | dpo@acme.eu     | Order processing | Contract     | name, email, addr   | DHL, Stripe       | 7 years   | TLS, AES-256, RBAC      | US (SCCs)          |
| Email marketing     | Acme GmbH  | dpo@acme.eu     | Promotions       | Consent      | email, name         | Mailchimp         | 2 years   | Encrypted API           |                    |
| HR / payroll        | Acme GmbH  | dpo@acme.eu     | Administration   | Legal oblig. | name, bank, tax ID  | DATEV, tax auth.  | 10 years  | Physical, encrypted DB  |                    |
```

Problems:
- Column "Legal Basis" accepts any string — "Legal oblig." and "legal obligation" are different
  values but mean the same thing.
- Exemption analysis (Art. 30(5)) is a separate Word document written once and never updated.
- No way to validate that all Art. 30(1) fields are present for every record.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::ropa::*;
use legalis_eu::gdpr::types::{LawfulBasis, PersonalDataCategory, ProcessingOperation};
use chrono::Utc;

fn build_ropa() -> Result<(), GdprError> {
    let ropa = RecordsOfProcessingActivities::new("Acme GmbH")
        .add_record(
            ProcessingRecord::new()
                .with_entity_type(EntityType::Controller)
                .with_name("Customer Order Processing")
                .with_controller_details(
                    ContactDetails::new("Acme GmbH", "dpo@acme.eu")
                        .with_address("1 Commerce Street, Berlin, Germany"),
                )
                .with_dpo("Data Protection Officer", "dpo@acme.eu")
                .with_purpose("Processing customer orders and arranging delivery")
                .with_lawful_basis(LawfulBasis::Contract {
                    necessary_for_performance: true,
                })
                .add_data_subject_category("customers")
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_data_category(PersonalDataCategory::Regular("email address".to_string()))
                .add_data_category(PersonalDataCategory::Regular("shipping address".to_string()))
                .add_data_category(PersonalDataCategory::Regular("payment details".to_string()))
                .add_recipient("Stripe Inc (payment processor)")
                .add_recipient("DHL (shipping provider)")
                .add_third_country_transfer(ThirdCountryTransfer {
                    country: "United States".to_string(),
                    safeguard: "Standard Contractual Clauses (2021/914/EU)".to_string(),
                    documentation: Some("SCC-2024-001 signed 2024-01-15".to_string()),
                })
                .with_retention_period("7 years after purchase (German tax law)")
                .add_security_measure("TLS 1.3 in transit")
                .add_security_measure("AES-256 at rest")
                .add_security_measure("Role-based access control")
                .add_operation(ProcessingOperation::Collection)
                .add_operation(ProcessingOperation::Storage)
                .add_operation(ProcessingOperation::Use)
                .with_created_date(Utc::now())
                .with_last_updated(Utc::now()),
        )
        .add_record(
            ProcessingRecord::new()
                .with_entity_type(EntityType::Controller)
                .with_name("Email Marketing")
                .with_controller("Acme GmbH", "dpo@acme.eu")
                .with_dpo("Data Protection Officer", "dpo@acme.eu")
                .with_purpose("Sending promotional emails to subscribers")
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                })
                .add_data_subject_category("newsletter subscribers")
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .add_data_category(PersonalDataCategory::Regular("name".to_string()))
                .add_recipient("Mailchimp (email service provider)")
                .with_retention_period("Until unsubscribe or 2 years of inactivity")
                .add_security_measure("Encrypted API connections")
                .add_security_measure("Access logging"),
        )
        .with_last_reviewed(Utc::now());

    let validation = ropa.validate()?;

    println!("Total records: {}", validation.total_records);
    println!("Complete records: {}", validation.complete_records);
    println!("Records with special categories: {}", validation.records_with_special_categories);
    println!("Records with third-country transfers: {}", validation.records_with_transfers);

    // Art. 30(5) exemption — applied automatically by the API
    match ropa.is_exempt(120) {
        RopaExemption::Exempt => {
            println!("Exempt from ROPA requirement (Art. 30(5))");
        }
        RopaExemption::NotExempt { reason } => {
            println!("ROPA required: {}", reason);
            // Proceed with maintaining the ROPA
        }
    }

    // Inspect per-record completeness
    for (i, record_validation) in validation.record_validations.iter().enumerate() {
        if !record_validation.complete {
            println!("Record {} is incomplete:", i + 1);
            for warning in &record_validation.warnings {
                println!("  - {}", warning);
            }
        }
    }

    Ok(())
}
```

### Benefits

- `ProcessingRecord::validate()` checks all Art. 30(1)/(2) fields and returns structured warnings
  — no manual audit column needed.
- `RecordsOfProcessingActivities::is_exempt(n)` applies all three Art. 30(5) criteria (size,
  systematic processing, special categories) atomically. The exemption analysis is always current.
- `LawfulBasis` is an enum — "legal obligation" and "Legal Oblig." cannot co-exist as different
  strings.

---

## 8. Step 4: Data Subject Rights Handling (Arts. 15–22)

### Before (manual)

Teams maintain a DSAR (Data Subject Access Request) log, usually a shared spreadsheet or Jira board:

```
| Ticket  | Received   | Right        | Identity Verified | Due Date   | Status    | Notes                   |
|---------|------------|--------------|-------------------|------------|-----------|-------------------------|
| DSR-001 | 2024-06-01 | Erasure      | YES               | 2024-07-01 | Complete  |                         |
| DSR-002 | 2024-06-10 | Access       | YES               | 2024-07-10 | Overdue   | Holiday backlog         |
| DSR-003 | 2024-06-15 | Portability  | YES               | 2024-07-15 | Open      | Grounds not collected   |
```

Problems:
- Deadline is computed manually ("received + 30 days"), prone to calendar errors.
- "Grounds not collected" is a free-text note; the system does not prevent responding without grounds.
- The difference between Art. 15 Access and Art. 20 Portability exists only as a label.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::DataSubjectRight;

fn handle_erasure_request() -> Result<(), GdprError> {
    // Art. 17 — erasure requires grounds to be provided
    let request = DataSubjectRequest::new()
        .with_data_subject("alice@example.com")
        .with_right(DataSubjectRight::Erasure)
        .with_controller("Acme GmbH")
        .with_grounds("Data no longer necessary for the original purpose");

    let validation = request.validate()?;

    println!("Deadline: {} days (Art. 12(3))", validation.deadline_days);
    println!("Must comply: {}", validation.must_comply);

    // Art. 17(3) exceptions are surfaced automatically
    if !validation.exceptions.is_empty() {
        println!("Potential exceptions to consider:");
        for exception in &validation.exceptions {
            println!("  - {}", exception);
        }
    }

    Ok(())
}

fn handle_access_request() -> Result<(), GdprError> {
    // Art. 15 — access; no grounds required
    let request = DataSubjectRequest::new()
        .with_data_subject("bob@example.com")
        .with_right(DataSubjectRight::Access)
        .with_controller("Acme GmbH");

    let validation = request.validate()?;
    println!("Deadline: {} days", validation.deadline_days);

    Ok(())
}

fn handle_portability_request() -> Result<(), GdprError> {
    // Art. 20 — portability; only applies to consent/contract basis + automated processing
    let request = DataSubjectRequest::new()
        .with_data_subject("carol@example.com")
        .with_right(DataSubjectRight::DataPortability)
        .with_controller("Acme GmbH");

    let validation = request.validate()?;

    // The API surfaces the Art. 20(3) scope limitation automatically
    for exception in &validation.exceptions {
        println!("Scope note: {}", exception);
    }

    Ok(())
}

fn handle_objection() -> Result<(), GdprError> {
    // Art. 21 — right to object; grounds required
    let request = DataSubjectRequest::new()
        .with_data_subject("dave@example.com")
        .with_right(DataSubjectRight::Object)
        .with_controller("Acme GmbH")
        .with_grounds("Personal situation makes processing disproportionate");

    let _validation = request.validate()?;
    Ok(())
}
```

All `DataSubjectRight` variants available:

```rust
// Art. 15 — access
DataSubjectRight::Access
// Art. 16 — rectification
DataSubjectRight::Rectification
// Art. 17 — erasure ("right to be forgotten")
DataSubjectRight::Erasure
// Art. 18 — restriction of processing
DataSubjectRight::RestrictionOfProcessing
// Art. 20 — data portability
DataSubjectRight::DataPortability
// Art. 21 — right to object
DataSubjectRight::Object
// Art. 22 — rights re automated decision-making
DataSubjectRight::AutomatedDecisionMaking
```

### Benefits

- `RequestValidation.deadline_days` is always 30 — computed from Article 12(3), not a spreadsheet
  formula.
- Erasure and Object requests fail `validate()` if `grounds` is not provided — the system enforces
  the requirement at intake, not at review.
- Portability's Art. 20(3) scope limitation appears in `validation.exceptions` without a legal team
  having to remember it.

---

## 9. Step 5: Data Protection Impact Assessments (Art. 35)

### Before (manual)

DPIAs are produced as Word documents. A typical team uses a template with sections:

```
Section 1: Is a DPIA required?
  [ ] Systematic automated processing (Art. 35(3)(a))?
  [ ] Large-scale special categories (Art. 35(3)(b))?
  [ ] Systematic monitoring of public spaces (Art. 35(3)(c))?

Section 2: Processing description
  ...free text...

Section 3: Necessity and proportionality
  ...free text...

Section 4: Risk register
  | Risk           | Likelihood | Severity | Mitigation          | Residual Risk |
  |----------------|------------|----------|---------------------|---------------|
  | Data breach    | Medium     | High     | Encryption          | Low           |
  | Discrimination | High       | High     | Annual audit        | Medium        |

Section 5: Outcome
  DPO consulted: YES
  Prior consultation with SA required: NO  ← manually assessed
```

Problems:
- "Prior consultation required" is a human judgment on the Word form with no enforcement.
- Risk classification (Low/Medium/High) is informal text; "medium" and "Medium" can coexist.
- The trigger checklist is separate from the processing description — they can contradict each other.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::dpia::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{PersonalDataCategory, ProcessingOperation, SpecialCategory};
use chrono::Utc;

fn conduct_ai_recruitment_dpia() -> Result<(), GdprError> {
    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("Acme GmbH")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "AI-powered automated screening of job applications using machine learning \
             to evaluate and rank candidates based on CV content and skills matching",
        )
        .with_purpose("Automated candidate evaluation to reduce recruitment workload")
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .add_data_category(PersonalDataCategory::Regular("employment history".to_string()))
        .add_data_category(PersonalDataCategory::Regular("education".to_string()))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Use)
        // Triggers determine whether DPIA is required — structured, not a checklist
        .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true,
            systematic: true,
            extensive: true,
        })
        .add_trigger(DpiaTrigger::ProfilingOrScoring {
            profiling_type: "Candidate scoring algorithm".to_string(),
            significant_effects: true,
        })
        .with_necessity_assessment(
            "10,000+ applications per month makes manual review impractical",
        )
        .with_proportionality_assessment(
            "Proportionate: only relevant data used; human review before final decision; \
             right to object provided; limited retention period",
        )
        // Risks use typed enums — no free-text likelihood/severity
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Algorithm may exhibit bias against protected characteristics \
                         if training data reflects historical hiring patterns"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::RightsViolation,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Candidates may be unable to understand or challenge decisions"
                .to_string(),
        })
        // Mitigations reference the risk type they address
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Quarterly algorithmic fairness audits with retraining if bias detected"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::RightsViolation,
            measure: "Provide Art. 22 explanations; human review available on request"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion(
            "DPO recommends implementing bias audits and mandatory human sign-off",
        )
        .with_data_subjects_consulted(false);

    let validation = dpia.validate()?;

    println!("DPIA complete: {:?}", validation.dpia_complete);
    println!("Residual risk level: {:?}", validation.residual_risk_level);
    // prior_consultation_required is computed from residual risk, not a manual checkbox
    println!("Prior consultation required (Art. 36): {}", validation.prior_consultation_required);
    println!("Processing may proceed: {:?}", validation.processing_may_proceed);

    for rec in &validation.recommendations {
        println!("Recommendation: {}", rec);
    }

    Ok(())
}

fn check_hospital_records_dpia() -> Result<(), GdprError> {
    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("Metropolitan Hospital")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "Centralized EHR system for 50,000+ patients storing medical histories and diagnoses",
        )
        .with_purpose("Patient care coordination and medical record management")
        // Special category data — BiometricData, HealthData, etc.
        .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
        .add_data_category(PersonalDataCategory::Regular("contact information".to_string()))
        .add_operation(ProcessingOperation::Storage)
        .add_trigger(DpiaTrigger::LargeScaleSpecialCategories {
            categories: vec![SpecialCategory::HealthData],
            scale: 50_000,
        })
        .with_necessity_assessment(
            "EHR essential for coordinated care across departments and specialists",
        )
        .with_proportionality_assessment(
            "Access controls limit viewing to treating physicians; audit logs track all access",
        )
        .add_risk(RiskAssessment {
            risk_type: RiskType::UnauthorizedAccess,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Unauthorized access to health data causing patient harm".to_string(),
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::UnauthorizedAccess,
            measure: "MFA, RBAC, session timeouts, comprehensive audit logging".to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion("DPO approves with recommendation for annual security audits")
        .with_data_subjects_consulted(true);

    let validation = dpia.validate()?;
    println!("Prior consultation required: {}", validation.prior_consultation_required);

    Ok(())
}
```

### Benefits

- `DpiaTrigger` variants encode Art. 35(3)(a)/(b)/(c) exactly. The trigger inventory is part of
  the type system, not a Word checklist.
- `Likelihood` and `Severity` are enums — `High` and "High" cannot diverge.
- `DpiaValidation.prior_consultation_required` is computed from residual risk using the Art. 36
  threshold, not written manually on the form. An insufficient mitigation (`Effectiveness::Low`)
  leaves residual risk high, which automatically sets the flag.

---

## 10. Step 6: Audit Trail & Accountability

### Before (manual)

Accountability is demonstrated by a folder structure of PDFs and signed documents:

```
compliance/
  2024-Q1-ROPA-review-signed.pdf
  2024-AWS-DPA-signed.pdf
  2024-security-assessment-signed.pdf
  staff-training-Q1-2024.xlsx
  ISO-27001-certificate.pdf
```

Problems:
- Compliance score is a DPO's subjective assessment, not a computed value.
- A new supervisory authority audit requires manually assembling evidence from the folder.
- Accountability measures are siloed: the Art. 32 security assessment and the Art. 28 processor
  contract are separate documents with no linkage.

### After (legalis-eu)

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::security::RiskLevel;
use chrono::Utc;

fn build_accountability_framework() -> Result<(), GdprError> {
    // Article 32 — security measures
    let security = SecurityAssessment::new()
        .with_entity("Acme GmbH")
        .with_risk_level(RiskLevel::High)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256, TLS 1.3".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Customer IDs replaced with UUIDs in analytics".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::BackupRecovery {
            backup_frequency: "Daily automated backups".to_string(),
            recovery_time_objective: "4 hours".to_string(),
            recovery_point_objective: "1 hour".to_string(),
            tested: true,
        })
        .add_organizational_measure(OrganizationalMeasure::AccessControl {
            role_based: true,
            least_privilege: true,
        })
        .add_organizational_measure(OrganizationalMeasure::StaffTraining {
            frequency: "Quarterly GDPR and security training".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true);

    let security_validation = security.validate()?;
    println!("Technical measures: {}", security_validation.technical_measures_count);
    println!("Organizational measures: {}", security_validation.organizational_measures_count);
    println!("Art. 32 compliant: {}", security_validation.compliant);

    // Article 28 — processor contract
    let processor_contract = ProcessorContract::new()
        .with_controller("Acme GmbH", "dpo@acme.eu")
        .with_processor("Amazon Web Services EMEA", "aws-privacy@amazon.com")
        .with_subject_matter("Cloud hosting of e-commerce platform")
        .with_processing_purpose("Website hosting and database storage")
        .add_data_category("Customer orders, names, addresses")
        .add_data_subject_category("Acme GmbH customers")
        .with_all_mandatory_clauses()
        .with_notes("ISO 27001, SOC 2 Type II. International transfers via SCCs.");

    let contract_validation = processor_contract.validate()?;
    println!("Processor contract compliant: {}", contract_validation.compliant);

    // Article 24 — accountability framework (integrates Art. 32 + 28 outcomes)
    let accountability = ControllerAccountability::new()
        .with_controller_name("Acme GmbH")
        .with_data_volume(DataVolume::Medium)
        .with_data_sensitivity(DataSensitivity::High)
        .with_risk_level_assessed(RiskLevel::High)
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: security_validation.compliant,
            documented: true,
            notes: Some("Art. 32 assessment completed Q1 2024".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: contract_validation.compliant,
            notes: Some("AWS DPA signed; DATEV DPA signed".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Quarterly".to_string()),
            notes: None,
        })
        .add_organizational_measure(AccountabilityMeasure::DataSubjectRightsProcedures {
            procedures_documented: true,
            response_process_established: true,
            notes: Some("30-day response SLA; DSAR tracker in use".to_string()),
        })
        .add_certification(ComplianceCertification::InformationSecurity {
            standard: "ISO/IEC 27001:2022".to_string(),
            certified: true,
            valid_until: Some(Utc::now() + chrono::Duration::days(365)),
        })
        .with_compliance_documentation(true)
        .with_nature_considered(true)
        .with_scope_considered(true)
        .with_context_considered(true)
        .with_purposes_considered(true);

    let account_validation = accountability.validate()?;
    println!("Compliance score: {}/100", account_validation.compliance_score);
    println!("Certifications: {}", account_validation.certifications_count);

    for warning in &account_validation.warnings {
        println!("Warning: {}", warning);
    }

    for rec in &account_validation.recommendations {
        println!("Recommendation: {}", rec);
    }

    Ok(())
}
```

### Benefits

- `AccountabilityValidation.compliance_score` is computed deterministically — it does not depend on
  a DPO's subjective assessment.
- The Art. 28 `contract_validation.compliant` result flows directly into `AccountabilityMeasure::
  ProcessorContracts`, creating an explicit linkage between the two articles.
- `ComplianceCertification::InformationSecurity { valid_until }` tracks certificate expiry;
  an expired certificate can surface as a warning in the next validation cycle.

---

## 11. Common Migration Pitfalls

### Pitfall 1: Mapping free-text basis strings to the wrong enum variant

Manual registers often use abbreviations: "LI" (legitimate interests), "Contr." (contract). During
migration, map each free-text value to the exact variant and review the associated fields.

```
"LI — LIA not done" → LawfulBasis::LegitimateInterests { balancing_test_passed: false }
```

Do not default `balancing_test_passed` to `true` to silence a warning. The LIA must be completed.

### Pitfall 2: Treating special category data as regular data

If a spreadsheet column "Data category" contains "health data," it must map to:

```rust
PersonalDataCategory::Special(SpecialCategory::HealthData)
```

not to:

```rust
PersonalDataCategory::Regular("health data".to_string())  // WRONG — Art. 9 bypassed
```

`PersonalDataCategory::Special` triggers Art. 9 exception requirements. Mapping health data to
`Regular` hides an obligation.

### Pitfall 3: Forgetting that ROPA exemption requires ALL three criteria

Art. 30(5) exempts organizations with fewer than 250 employees *only if* all three conditions hold:
the processing is occasional, it is unlikely to risk data subjects' rights, and it does not involve
special categories or criminal data. `is_exempt(n)` checks all three automatically. Do not rely
on headcount alone.

### Pitfall 4: Computing DSAR deadlines manually

Art. 12(3) gives one month from receipt, extendable by two months for complex requests. Do not
recompute this. `RequestValidation.deadline_days` is always 30 (base). Build deadline tracking
on top of `requested_at` and this value, not on ad-hoc date arithmetic.

### Pitfall 5: Treating DpiaValidation.prior_consultation_required as advisory

If `prior_consultation_required` is `true`, Art. 36 requires consulting the supervisory authority
before commencing processing. This is not a recommendation. Do not begin processing on the
assumption that the SA will approve.

### Pitfall 6: Calling validate() and ignoring warnings

```rust
// WRONG — warnings may indicate gaps in the record
let _ = ropa.validate()?;

// RIGHT — inspect warnings per record
let validation = ropa.validate()?;
for record_v in &validation.record_validations {
    for warning in &record_v.warnings {
        eprintln!("ROPA warning: {}", warning);
    }
}
```

### Pitfall 7: Using unwrap() in compliance code

All `legalis-eu` validation functions return `Result`. Use `?` in functions that propagate errors,
or `match` at the application boundary. Never use `unwrap()` in production compliance paths — a
panic in a DSAR handler could mean a missed Art. 12 deadline.

---

## 12. Complete Migration Checklist

Work through this list in order. Each item maps to a section in this guide.

### Phase 1 — Preparation

- [ ] Add `legalis-eu` to `Cargo.toml` workspace dependencies
- [ ] Identify all existing compliance artifacts (spreadsheets, Word docs, Jira boards)
- [ ] Assign each artifact to the corresponding section below

### Phase 2 — Core Processing (Step 1)

- [ ] Enumerate all processing activities in the lawful basis register
- [ ] Map each free-text lawful basis to the correct `LawfulBasis` variant
- [ ] Verify that every `LegitimateInterests` entry has a completed LIA
  (`balancing_test_passed: true`)
- [ ] Identify all special category data; migrate to `PersonalDataCategory::Special`
- [ ] Confirm each `DataProcessing` passes `validate()` before marking as complete

### Phase 3 — Consent (Step 2)

- [ ] Migrate consent database to `ConsentRecord` struct (or equivalent storage schema)
- [ ] Validate existing consent records with `ConsentRecord::is_valid()`
- [ ] Remove or re-obtain consent for records where `freely_given: false`
- [ ] Ensure withdrawal workflow sets record to invalid rather than deleting it (audit trail)

### Phase 4 — ROPA (Step 3)

- [ ] Port each ROPA row to a `ProcessingRecord`
- [ ] Add controller/processor DPO details (`with_controller_details` + `with_dpo`)
- [ ] Add all third-country transfers with `ThirdCountryTransfer { safeguard, documentation }`
- [ ] Run `RecordsOfProcessingActivities::validate()` and resolve all warnings
- [ ] Run `is_exempt(employee_count)` — document result; if `NotExempt`, maintain ROPA

### Phase 5 — Data Subject Rights (Step 4)

- [ ] Port DSAR intake form to `DataSubjectRequest` builder
- [ ] Ensure erasure and objection requests require `with_grounds`
- [ ] Expose `RequestValidation.deadline_days` to the DSAR ticketing system
- [ ] Verify all seven `DataSubjectRight` variants are covered by response procedures

### Phase 6 — DPIA (Step 5)

- [ ] List all processing activities that may be high-risk (Art. 35(1))
- [ ] For each, construct `DataProtectionImpactAssessment` and add all applicable `DpiaTrigger`s
- [ ] Populate the risk register with `RiskAssessment` entries (no free-text likelihood/severity)
- [ ] Add `Mitigation` entries; use `Effectiveness::High` only when genuinely achieved
- [ ] Check `DpiaValidation.prior_consultation_required` — schedule SA consultation if `true`
- [ ] Set `with_dpo_consulted(true)` only after DPO review is complete

### Phase 7 — Accountability & Security (Step 6)

- [ ] Complete `SecurityAssessment` for all high-risk processing (Art. 32)
- [ ] Verify `SecurityValidation.compliant` is `true` before linking to `ControllerAccountability`
- [ ] Build `ProcessorContract` for every sub-processor; verify `contract_validation.compliant`
- [ ] Complete `ControllerAccountability` integrating Art. 28 and Art. 32 outcomes
- [ ] Aim for `AccountabilityValidation.compliance_score` >= 80 before DPA registration

### Phase 8 — Ongoing operations

- [ ] Schedule quarterly ROPA review: rebuild and re-validate
- [ ] On any new processing activity: start from Step 1 (lawful basis) and proceed in order
- [ ] On any new sub-processor: complete `ProcessorContract` before processing begins
- [ ] On a DSAR: use `DataSubjectRequest` at intake to confirm grounds and set deadline
- [ ] On any system change that may trigger Art. 35: re-run DPIA before deployment

---

*This guide covers legalis-eu as shipped with the 0.1.7 release. For API reference, see the
[GDPR Guide](GDPR_GUIDE.md). For getting started quickly, see the [Quickstart](QUICKSTART.md).*
