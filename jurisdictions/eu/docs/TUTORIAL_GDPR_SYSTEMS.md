# Tutorial: Building GDPR-Compliant Systems with legalis-eu

A step-by-step guide to building production-ready, GDPR-compliant Rust applications
using the `legalis-eu` crate.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites & Installation](#2-prerequisites--installation)
3. [Understanding the Data Processing Model (Art. 6)](#3-understanding-the-data-processing-model-art-6)
4. [Managing Consent (Art. 7)](#4-managing-consent-art-7)
5. [Handling Data Subject Rights (Arts. 15–22)](#5-handling-data-subject-rights-arts-1522)
6. [Security & Breach Notification (Arts. 32–34)](#6-security--breach-notification-arts-3234)
7. [Data Protection Impact Assessments (Arts. 35–36)](#7-data-protection-impact-assessments-arts-3536)
8. [Cross-Border Transfers (Chapter V)](#8-cross-border-transfers-chapter-v)
9. [Administrative Fines Calculator (Art. 83)](#9-administrative-fines-calculator-art-83)
10. [Building a Complete Compliance Workflow](#10-building-a-complete-compliance-workflow)
11. [Next Steps](#11-next-steps)

---

## Status Checklist

- [x] ✅ Introduction
- [x] ✅ Prerequisites & Installation
- [x] ✅ Understanding the Data Processing Model (Art. 6)
- [x] ✅ Managing Consent (Art. 7)
- [x] ✅ Handling Data Subject Rights (Arts. 15–22)
- [x] ✅ Security & Breach Notification (Arts. 32–34)
- [x] ✅ Data Protection Impact Assessments (Arts. 35–36)
- [x] ✅ Cross-Border Transfers (Chapter V)
- [x] ✅ Administrative Fines Calculator (Art. 83)
- [x] ✅ Complete Compliance Workflow
- [x] ✅ Next Steps

---

## 1. Introduction

The General Data Protection Regulation (GDPR, Regulation 2016/679) is the EU's
comprehensive data protection framework. Fines for non-compliance can reach **€20 million
or 4 % of global annual turnover** — whichever is higher. Getting compliance right in
code, at the type level, is therefore not just good engineering practice; it is essential
risk management.

`legalis-eu` encodes GDPR compliance logic as Rust types. Validation errors surface at
**runtime** (not at compile time, because compliance judgments often depend on runtime
facts), but the builder API makes it impossible to accidentally omit a required field
without the compiler raising at least a warning.

### What you will build

By the end of this tutorial you will have implemented a compliance workflow for a
fictional e-commerce company — **TechShop Europe GmbH** — that:

- Establishes a lawful basis before processing any personal data
- Validates consent to Art. 7 requirements
- Handles Data Subject Access Requests (DSARs) for all seven rights
- Assesses breach severity and determines notification obligations
- Conducts a Data Protection Impact Assessment (DPIA) for a new AI feature
- Validates cross-border transfers to third countries
- Calculates the indicative fine exposure for a hypothetical infringement

The running example mirrors the
`examples/gdpr_complete_compliance_workflow.rs` file in the crate repository.

---

## 2. Prerequisites & Installation

### GDPR knowledge prerequisites

You should have a working familiarity with:

- The six lawful bases (Art. 6(1)(a)–(f))
- Data subject rights (Chapter III)
- The accountability principle (Art. 5(2))

You do **not** need to be a lawyer; `legalis-eu` surfaces the legal requirements as
code comments and error messages.

### Add the dependency

```toml
[dependencies]
legalis-eu = "0.1"
chrono = { version = "0.4", features = ["serde"] }
```

> **Feature flags**
>
> | Flag | Enables |
> |------|---------|
> | `serde` | `Serialize` / `Deserialize` for all types |
> | `schema` | `JsonSchema` derivations (requires `schemars`) |
>
> The examples in this tutorial require no extra features beyond the defaults.

### Verify the installation

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processing = DataProcessing::new()
        .with_controller("Test Corp")
        .with_purpose("Smoke test")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    processing.validate()?;
    println!("legalis-eu is working correctly.");
    Ok(())
}
```

Run `cargo run` and you should see the confirmation message.

---

## 3. Understanding the Data Processing Model (Art. 6)

**Legal context.** Article 6 GDPR requires that *every* processing operation has a
documented lawful basis. Processing without one is unlawful — the first question any
supervisory authority will ask.

### The `DataProcessing` builder

`DataProcessing` is the foundational type in `legalis-eu`. It models a single
processing activity and validates it against Article 6 requirements.

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a processing activity for e-commerce order fulfillment
    let processing = DataProcessing::new()
        .with_controller("TechShop Europe GmbH")
        .with_purpose("Process customer orders and fulfill contracts")
        .add_data_category(PersonalDataCategory::Regular(
            "Name, email, shipping address, payment details".to_string(),
        ))
        .with_operations(vec![
            ProcessingOperation::Collection,
            ProcessingOperation::Storage,
            ProcessingOperation::Use,
            ProcessingOperation::Disclosure,
        ])
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    let validation = processing.validate()?;

    if validation.is_compliant() {
        println!("✅ Article 6 satisfied — lawful basis: Contract performance (Art. 6(1)(b))");
    }

    Ok(())
}
```

**What the code does.** `DataProcessing::new()` starts a builder. Each
`.with_*()` / `.add_*()` call fills in a field. `.validate()` runs the Article 6
check; it returns `Err(GdprError)` if a required field is missing or if the
chosen lawful basis is internally inconsistent (e.g. consent with `freely_given: false`).

### The six lawful bases

| Enum variant | GDPR article | Typical use case |
|---|---|---|
| `LawfulBasis::Consent { .. }` | 6(1)(a) | Marketing emails, optional features |
| `LawfulBasis::Contract { .. }` | 6(1)(b) | Order fulfilment, delivery |
| `LawfulBasis::LegalObligation` | 6(1)(c) | Tax records, anti-money-laundering |
| `LawfulBasis::VitalInterests` | 6(1)(d) | Emergency healthcare |
| `LawfulBasis::PublicTask` | 6(1)(e) | Government data processing |
| `LawfulBasis::LegitimateInterests { .. }` | 6(1)(f) | Fraud detection, direct mail |

### Special categories (Art. 9)

If `add_data_category(PersonalDataCategory::Special(_))` is called, `validate()`
sets `validation.requires_article9_exception = true` and returns
`ComplianceStatus::RequiresAdditionalReview`. You must apply one of the ten
exceptions listed in Art. 9(2) before processing may begin.

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let health_processing = DataProcessing::new()
        .with_controller("Hospital Management System")
        .with_purpose("Patient medical records management")
        .add_data_category(PersonalDataCategory::Regular("patient name".to_string()))
        .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    let validation = health_processing.validate()?;

    if validation.requires_article9_exception {
        println!("⚠️  Special category data — an Art. 9(2) exception is required.");
        println!("    Possible exceptions:");
        println!("    • Art. 9(2)(a): Explicit consent");
        println!("    • Art. 9(2)(h): Healthcare / medical diagnosis");
        println!("    • Art. 9(2)(i): Public health");
    }

    Ok(())
}
```

> **Caveat.** `legalis-eu` validates that you *recognise* the need for an Art. 9
> exception; it does not automatically grant one. Recording which exception applies is
> part of your Records of Processing Activities (Art. 30).

---

## 4. Managing Consent (Art. 7)

**Legal context.** When consent is your lawful basis under Art. 6(1)(a), Art. 7 imposes
four cumulative conditions: consent must be *freely given*, *specific*, *informed*, and
given through an *unambiguous* indication. Failure on any one condition makes the
consent invalid.

### Valid consent

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processing = DataProcessing::new()
        .with_controller("Acme Corporation")
        .with_purpose("Email marketing for new product announcements")
        .add_data_category(PersonalDataCategory::Regular("email address".to_string()))
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .with_operation(ProcessingOperation::Collection)
        .with_operation(ProcessingOperation::Storage)
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,   // Not bundled; user can decline without penalty
            specific: true,       // Clearly scoped to product announcements
            informed: true,       // Privacy notice served before opt-in
            unambiguous: true,    // Explicit tick-box; no pre-ticked boxes
        });

    let validation = processing.validate()?;

    if validation.is_compliant() {
        println!("✅ Consent valid — Art. 6(1)(a) + Art. 7 satisfied");
    }

    Ok(())
}
```

### Invalid consent (coerced)

Setting `freely_given: false` makes the consent invalid. `validate()` returns an
error:

```rust
use legalis_eu::gdpr::*;

fn main() {
    let invalid = DataProcessing::new()
        .with_controller("Acme Corporation")
        .with_purpose("Marketing")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: false, // Consent bundled with terms of service
            specific: true,
            informed: true,
            unambiguous: true,
        });

    match invalid.validate() {
        Ok(_) => println!("Validated"),
        Err(e) => println!("❌ Validation failed: {e}"),
        // Output: ❌ Validation failed: Consent is not freely given
    }
}
```

### Legitimate interests and the balancing test

`LawfulBasis::LegitimateInterests` requires a balancing test (Recital 47). When
`balancing_test_passed: false`, the library returns a `LegalResult::JudicialDiscretion`
variant — a signal that human review is required before processing begins:

```rust
use legalis_eu::gdpr::*;
use legalis_core::LegalResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let li_processing = DataProcessing::new()
        .with_controller("Security Company")
        .with_purpose("Fraud detection and prevention")
        .add_data_category(PersonalDataCategory::Regular("IP address".to_string()))
        .add_data_category(PersonalDataCategory::Regular(
            "transaction history".to_string(),
        ))
        .with_lawful_basis(LawfulBasis::LegitimateInterests {
            controller_interest: "Preventing fraudulent transactions to protect customers"
                .to_string(),
            balancing_test_passed: false, // Assessment still pending
        });

    let validation = li_processing.validate()?;

    match &validation.lawful_basis_valid {
        LegalResult::JudicialDiscretion {
            issue,
            narrative_hint,
            ..
        } => {
            println!("⚖️  Human judgment required");
            println!("   Issue: {issue}");
            if let Some(hint) = narrative_hint {
                println!("   Guidance: {hint}");
            }
        }
        _ => println!("Result: {:?}", validation),
    }

    Ok(())
}
```

**What to do next.** Conduct a Legitimate Interests Assessment (LIA), document the
outcome, then re-run the check with `balancing_test_passed: true` once the LIA
confirms the interests are not overridden by data subjects' rights.

---

## 5. Handling Data Subject Rights (Arts. 15–22)

**Legal context.** Chapter III of the GDPR grants individuals seven rights over their
personal data. Controllers must respond within **30 days** (extendable by 60 days for
complex requests). `DataSubjectRequest` models any one of these rights and computes the
deadline and any applicable exceptions.

### The `DataSubjectRequest` builder

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Art. 15 — Right of Access
    let access_request = DataSubjectRequest::new()
        .with_data_subject("john.doe@example.com")
        .with_right(DataSubjectRight::Access)
        .with_controller("E-commerce Platform Inc");

    let validation = access_request.validate()?;

    println!("Right: {:?}", validation.right);
    println!("Response deadline: {} days", validation.deadline_days);
    Ok(())
}
```

### All seven rights at a glance

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Art. 16 — Rectification
    let rect = DataSubjectRequest::new()
        .with_data_subject("correct.me@example.com")
        .with_right(DataSubjectRight::Rectification)
        .with_controller("Customer Database System");
    rect.validate()?;

    // Art. 17 — Erasure ("right to be forgotten")
    let erasure = DataSubjectRequest::new()
        .with_data_subject("jane.smith@example.com")
        .with_right(DataSubjectRight::Erasure)
        .with_controller("Social Media Platform")
        .with_grounds("Personal data no longer necessary for the purposes collected");

    let erasure_val = erasure.validate()?;
    println!("Must comply with erasure: {}", erasure_val.must_comply);

    // Art. 20 — Data portability
    let portability = DataSubjectRequest::new()
        .with_data_subject("user123@example.com")
        .with_right(DataSubjectRight::DataPortability)
        .with_controller("Cloud Storage Provider");
    let port_val = portability.validate()?;

    if !port_val.exceptions.is_empty() {
        println!("⚠️ Portability restrictions:");
        for exc in &port_val.exceptions {
            println!("   - {exc}");
        }
    }

    // Art. 21 — Right to object
    let objection = DataSubjectRequest::new()
        .with_data_subject("privacy.advocate@example.com")
        .with_right(DataSubjectRight::Object)
        .with_controller("Marketing Analytics Company")
        .with_grounds("Object to processing for direct marketing purposes");
    objection.validate()?;

    Ok(())
}
```

### Erasure requires grounds

Omitting `.with_grounds()` on an erasure request causes `validate()` to return an
error:

```rust
use legalis_eu::gdpr::*;

fn main() {
    let invalid = DataSubjectRequest::new()
        .with_data_subject("user@example.com")
        .with_right(DataSubjectRight::Erasure)
        .with_controller("Service Provider");
    // No .with_grounds() — erasure requests require grounds under Art. 17

    match invalid.validate() {
        Ok(_) => {}
        Err(e) => {
            println!("❌ {e}");
            // ❌ Erasure requests require grounds under Article 17
        }
    }
}
```

### DSAR best practices

The `deadline_days` field and `exceptions` vec tell you everything you need to
generate an automated acknowledgement letter:

1. Respond within `validation.deadline_days` days (always 30 for initial response).
2. Verify the data subject's identity before disclosing data.
3. First request is free; subsequent repetitive requests may incur a reasonable fee.
4. If refusing, explain the reason and the right to complain to a supervisory authority.
5. Document every DSAR received and every response sent (Art. 5(2) accountability).

---

## 6. Security & Breach Notification (Arts. 32–34)

### Security measures (Art. 32)

**Legal context.** Article 32 requires "appropriate technical and organisational
measures" proportionate to the risk. The `SecurityAssessment` builder lets you document
and validate those measures.

```rust
use legalis_eu::gdpr::*;
use legalis_eu::gdpr::security::RiskLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let security = SecurityAssessment::new()
        .with_entity("TechShop Europe GmbH")
        .with_risk_level(RiskLevel::High) // Payment data = high risk
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256, TLS 1.3".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Customer IDs in analytics".to_string(),
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
            frequency: "Quarterly GDPR training".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true);

    let validation = security.validate()?;

    println!("Technical measures:      {}", validation.technical_measures_count);
    println!("Organisational measures: {}", validation.organizational_measures_count);
    println!("Compliant:               {}", validation.compliant);

    for warning in &validation.warnings {
        println!("⚠️  {warning}");
    }

    Ok(())
}
```

**Interpretation.** A `compliant: true` result means the declared measures satisfy the
minimum threshold for the stated risk level. `warnings` surface recommendations for
improvement even when the baseline is met.

### Breach notification (Arts. 33–34)

**Legal context.** Art. 33 requires notifying the supervisory authority within **72 hours**
of discovering a personal data breach (unless the breach is unlikely to result in risk).
Art. 34 requires notifying affected individuals directly when the breach is *high risk*
to their rights and freedoms.

```rust
use chrono::{Duration, Utc};
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Breach discovered 48 hours ago — still within the 72-hour window
    let breach = DataBreach::new()
        .with_controller("Online Retailer Ltd")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(48))
        .with_affected_data_subjects(500)
        .with_severity(BreachSeverity::Medium)
        .with_description(
            "Unauthorized access to customer database via SQL injection",
        )
        .add_mitigation_measure("Patched SQL injection vulnerability")
        .add_mitigation_measure("Reset affected user passwords")
        .add_mitigation_measure("Enhanced database access logging");

    let req = breach.validate_notification_requirements()?;

    println!("Hours since discovery: {}", req.hours_since_discovery);
    println!("SA deadline:           {}", req.supervisory_authority_deadline);

    if req.supervisory_authority_notification_required {
        if req.supervisory_authority_deadline_passed {
            println!("❌ 72-hour deadline EXCEEDED — non-compliant!");
        } else {
            let remaining = 72 - req.hours_since_discovery;
            println!("✅ {remaining} hours remaining to notify supervisory authority");
        }
    }

    if req.data_subject_notification_required {
        println!("⚠️  Must notify data subjects — breach is high risk");
    } else {
        println!("ℹ️  Data subject notification not required (medium risk)");
    }

    println!("Compliance status: {:?}", req.compliance_status);
    Ok(())
}
```

### Critical breach — immediate action required

When `BreachSeverity::Critical` is combined with special-category data, both the
supervisory authority and affected individuals must be notified without delay:

```rust
use chrono::{Duration, Utc};
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let critical = DataBreach::new()
        .with_controller("Healthcare Provider")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(2))
        .with_affected_data_subjects(50_000)
        .with_severity(BreachSeverity::Critical)
        .with_description("Patient medical records exposed on dark web")
        .with_affected_data_categories(vec![
            "Names".to_string(),
            "Addresses".to_string(),
            "Medical diagnoses".to_string(),
            "Treatment records".to_string(),
        ]);

    let req = critical.validate_notification_requirements()?;

    if req.data_subject_notification_required {
        println!("🚨 IMMEDIATE ACTIONS:");
        println!("   1. Notify supervisory authority (< 72 h, {} h left)", 72 - req.hours_since_discovery);
        println!("   2. Notify all affected data subjects WITHOUT DELAY");
        println!("   3. Record breach in Art. 33(5) internal register");
        println!("   4. Implement containment measures immediately");
    }

    Ok(())
}
```

### Handling a non-compliant late notification

```rust
use chrono::{Duration, Utc};
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let late = DataBreach::new()
        .with_controller("Financial Services Inc")
        .with_breach_category(BreachCategory::ConfidentialityBreach)
        .with_discovered_at(Utc::now() - Duration::hours(80)) // 80 h > 72 h limit
        .with_affected_data_subjects(10_000)
        .with_severity(BreachSeverity::High)
        .with_description("Ransomware attack encrypting customer financial records");

    let req = late.validate_notification_requirements()?;

    match req.compliance_status {
        BreachComplianceStatus::NonCompliant { ref violation } => {
            println!("❌ NON-COMPLIANT: {violation}");
            println!("   Notify immediately + include explanation of delay in report");
        }
        BreachComplianceStatus::Compliant => println!("✅ Compliant"),
    }

    Ok(())
}
```

> **Internal register (Art. 33(5)).** Every breach — regardless of severity — must be
> recorded internally. Use `legalis-eu`'s validation result together with your own audit
> log to satisfy this obligation.

---

## 7. Data Protection Impact Assessments (Arts. 35–36)

**Legal context.** Article 35 requires a DPIA *before* beginning processing that is
"likely to result in a high risk" to individuals. Three categories in Art. 35(3) always
trigger a mandatory DPIA: (a) systematic automated decision-making, (b) large-scale
processing of special categories, and (c) systematic monitoring of publicly accessible
areas.

### When is a DPIA required?

| Processing type | Art. 35(3) trigger | `DpiaTrigger` variant |
|---|---|---|
| AI recruitment scoring | Systematic automated decision-making | `AutomatedDecisionMaking` |
| Hospital records (50 000 patients) | Large-scale special categories | `LargeScaleSpecialCategories` |
| CCTV facial recognition | Systematic monitoring | `SystematicMonitoring` |
| Email newsletter (5 000 subscribers) | *None* — low risk | No DPIA needed |

### Scenario: AI recruitment screening

```rust
use chrono::Utc;
use legalis_eu::gdpr::dpia::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{PersonalDataCategory, ProcessingOperation};

fn main() -> Result<(), GdprError> {
    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("TechCorp Inc")
        .with_conducted_date(Utc::now())
        .with_processing_description(
            "AI-powered automated screening of job applications using machine learning \
             to evaluate candidates based on CV content and predictive performance modelling",
        )
        .with_purpose("Automated candidate evaluation and ranking for recruitment efficiency")
        .add_data_category(PersonalDataCategory::Regular("name".to_string()))
        .add_data_category(PersonalDataCategory::Regular("employment history".to_string()))
        .add_data_category(PersonalDataCategory::Regular("education".to_string()))
        .add_operation(ProcessingOperation::Collection)
        .add_operation(ProcessingOperation::Use)
        .add_operation(ProcessingOperation::Disclosure)
        .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true, // Affects job prospects
            systematic: true,
            extensive: true,
        })
        .add_trigger(DpiaTrigger::ProfilingOrScoring {
            profiling_type: "Candidate scoring algorithm".to_string(),
            significant_effects: true,
        })
        .with_necessity_assessment(
            "Processing 10 000+ applications per month makes manual review impractical; \
             automated screening is necessary to handle volume while maintaining quality.",
        )
        .with_proportionality_assessment(
            "Proportionate: only relevant data used, human review before final decision, \
             right to object provided, limited retention period.",
        )
        // Identify risks
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "AI algorithms may exhibit bias against protected characteristics \
                         if training data contains historical biases"
                .to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::RightsViolation,
            likelihood: Likelihood::Medium,
            severity: Severity::High,
            description: "Candidates may not understand how decisions are made, \
                         limiting their ability to challenge unfair outcomes"
                .to_string(),
        })
        // Mitigations
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Quarterly algorithmic fairness audits with retraining if bias detected"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::RightsViolation,
            measure: "Provide meaningful explanation of decision logic; allow candidates to \
                     request human review (Art. 22 compliance)"
                .to_string(),
            effectiveness: Effectiveness::High,
        })
        .with_dpo_consulted(true)
        .with_dpo_opinion("DPO recommends implementing bias audits and human oversight")
        .with_data_subjects_consulted(false);

    let validation = dpia.validate()?;

    println!("DPIA complete:                {:?}", validation.dpia_complete);
    println!("Residual risk level:          {:?}", validation.residual_risk_level);
    println!("Prior consultation required:  {}", validation.prior_consultation_required);
    println!("Processing may proceed:       {:?}", validation.processing_may_proceed);

    for rec in &validation.recommendations {
        println!("⚠️  {rec}");
    }

    Ok(())
}
```

**Interpreting the result.**

- `dpia_complete: true` — all mandatory sections are filled in.
- `residual_risk_level` — computed from risks minus the effectiveness of each mitigation. Aim for `Low` or `Medium`.
- `prior_consultation_required: true` — residual risk is still `High` after mitigations; you must consult your supervisory authority under Art. 36 before processing begins.
- `processing_may_proceed` — `None` when prior consultation is required (supervisory authority decides).

### Scenario: high residual risk (Art. 36 prior consultation)

When mitigations are insufficient, the DPIA signals that the supervisory authority must
be consulted:

```rust
use legalis_eu::gdpr::dpia::*;
use legalis_eu::gdpr::error::GdprError;

fn main() -> Result<(), GdprError> {
    let dpia = DataProtectionImpactAssessment::new()
        .with_controller("FinTech Startup")
        .with_processing_description("Credit scoring using alternative data sources")
        .with_purpose("Loan approval automation")
        .with_necessity_assessment("Required for business model")
        .with_proportionality_assessment("Proportionate to risk")
        .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
            produces_legal_effects: true,
            systematic: true,
            extensive: true,
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::Discrimination,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Alternative data may introduce prohibited discrimination".to_string(),
        })
        .add_risk(RiskAssessment {
            risk_type: RiskType::FinancialLoss,
            likelihood: Likelihood::High,
            severity: Severity::High,
            description: "Incorrect credit decisions cause financial harm".to_string(),
        })
        // Weak mitigations — LOW effectiveness
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::Discrimination,
            measure: "Annual fairness review".to_string(),
            effectiveness: Effectiveness::Low,
        })
        .add_mitigation(Mitigation {
            risk_addressed: RiskType::FinancialLoss,
            measure: "Manual review of 10 % of decisions".to_string(),
            effectiveness: Effectiveness::Low,
        });

    let validation = dpia.validate()?;

    println!("Residual risk:               {:?}", validation.residual_risk_level);
    println!("Prior consultation required: {}", validation.prior_consultation_required);

    if validation.prior_consultation_required {
        println!("⚠️  Art. 36(1) — consult supervisory authority BEFORE processing begins");
        for rec in &validation.recommendations {
            println!("   Recommendation: {rec}");
        }
    }

    Ok(())
}
```

> **Tip.** Upgrade `Effectiveness::Low` to `Effectiveness::High` (e.g. continuous
> monitoring instead of annual reviews) and re-run the DPIA. If residual risk drops to
> `Medium` or `Low`, prior consultation may no longer be required.

---

## 8. Cross-Border Transfers (Chapter V)

**Legal context.** Transferring personal data outside the EEA is prohibited unless one
of three mechanisms is in place: (1) an adequacy decision (Art. 45), (2) appropriate
safeguards such as SCCs or BCRs (Art. 46), or (3) a specific derogation (Art. 49).
Post-*Schrems II* (CJEU C-311/18), transfers to the US under SCCs also require a
Transfer Impact Assessment.

### Adequacy decision — Switzerland

Switzerland holds an adequacy decision; no additional safeguards are needed:

```rust
use legalis_eu::gdpr::cross_border::*;
use legalis_eu::gdpr::error::GdprError;

fn main() -> Result<(), GdprError> {
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Switzerland")
        .with_adequate_destination(AdequateCountry::Switzerland)
        .add_data_category("customer names")
        .add_data_category("email addresses")
        .with_purpose("Cloud storage");

    let validation = transfer.validate()?;

    println!(
        "Adequacy decision granted: {}",
        AdequateCountry::Switzerland.adequacy_year()
    );
    println!("Transfer permitted:            {:?}", validation.transfer_permitted);
    println!("Additional measures required:  {}", validation.additional_measures_required);
    println!("Risk assessment required:      {}", validation.risk_assessment_required);

    Ok(())
}
```

### Standard Contractual Clauses (SCCs) — US transfer

Use the **2021** version of the SCCs (Commission Implementing Decision 2021/914). The
old 2001/2004/2010 versions expired on 27 June 2022.

```rust
use legalis_eu::gdpr::cross_border::*;
use legalis_eu::gdpr::error::GdprError;

fn main() -> Result<(), GdprError> {
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("US")
        .with_safeguard(TransferSafeguard::StandardContractualClauses {
            version: "2021".to_string(), // Must be "2021" — older versions are rejected
            clauses_signed: true,
        })
        .add_data_category("user profiles")
        .with_purpose("Data analytics");

    let validation = transfer.validate()?;

    println!("Transfer permitted:           {:?}", validation.transfer_permitted);
    println!("Additional measures required: {}", validation.additional_measures_required);

    // Post-Schrems II: a Transfer Impact Assessment is still required
    if validation.risk_assessment_required {
        println!("⚠️  Perform a Transfer Impact Assessment:");
        println!("   1. Assess US surveillance laws (FISA 702, EO 12333)");
        println!("   2. Evaluate whether recipient is subject to government access");
        println!("   3. Consider supplementary measures (E2E encryption, pseudonymisation)");
        println!("   4. Document assessment and decision");
    }

    Ok(())
}
```

### Binding Corporate Rules (BCRs) — intragroup transfers

BCRs allow a multinational group to transfer data within its own corporate family once
approved by a lead supervisory authority:

```rust
use chrono::Utc;
use legalis_eu::gdpr::cross_border::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::shared::member_states::MemberState;

fn main() -> Result<(), GdprError> {
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Singapore")
        .with_safeguard(TransferSafeguard::BindingCorporateRules {
            approved_by: MemberState::Ireland,
            approval_date: Utc::now() - chrono::Duration::days(365),
        })
        .add_data_category("employee data")
        .with_purpose("HR management");

    let validation = transfer.validate()?;
    println!("Transfer permitted: {:?}", validation.transfer_permitted);

    Ok(())
}
```

### Derogations (Art. 49) — last resort

Derogations are for *specific, non-repetitive* situations. They cannot substitute for
proper safeguards in regular transfers:

```rust
use legalis_eu::gdpr::cross_border::*;
use legalis_eu::gdpr::error::GdprError;

fn main() -> Result<(), GdprError> {
    // ✅ Valid: one-off, 5 data subjects, non-repetitive
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("India")
        .with_derogation(TransferDerogation::CompellingLegitimateInterests {
            affected_data_subjects: 5,
            is_repetitive: false,
        })
        .add_data_category("contract details")
        .with_purpose("One-time legal claim");

    transfer.validate()?;

    // ❌ Invalid: too many data subjects for a derogation
    let bulk = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("India")
        .with_derogation(TransferDerogation::CompellingLegitimateInterests {
            affected_data_subjects: 100, // Exceeds the threshold
            is_repetitive: false,
        });

    match bulk.validate() {
        Ok(_) => {}
        Err(e) => println!("❌ Bulk derogation rejected: {e}"),
    }

    Ok(())
}
```

### Validation always fails without a basis

Attempting to transfer to an unknown country without any mechanism produces a clear
error at runtime:

```rust
use legalis_eu::gdpr::cross_border::*;

fn main() {
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Unknown Country")
        .add_data_category("personal data");
    // No safeguard, derogation, or adequacy decision

    match transfer.validate() {
        Ok(_) => {}
        Err(e) => println!("❌ Transfer rejected: {e}"),
        // ❌ No adequacy decision, appropriate safeguards, or derogation provided
    }
}
```

---

## 9. Administrative Fines Calculator (Art. 83)

**Legal context.** Article 83 GDPR provides two fine tiers:

| Tier | Articles | Maximum |
|---|---|---|
| Lower (Art. 83(4)) | Arts. 8, 11, 25–39, 42, 43 | €10 M or 2 % global turnover |
| Upper (Art. 83(5)) | Arts. 5, 6, 7, 9, 12–22, 44–49, 58(2) | €20 M or 4 % global turnover |

The `AdministrativeFine` builder models the Art. 83(2) assessment factors and computes
an indicative fine range. This is a **planning and risk-assessment tool** — actual fines
are set by supervisory authorities on a case-by-case basis.

### Upper-tier violation — large corporation

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factors = Article83Factors {
        duration_months: Some(18),
        data_subjects_affected: Some(500_000),
        damage_suffered: Some(2_000_000.0),
        intentional: false, // Negligent, not intentional
        mitigation_actions_taken: vec![
            "Immediately ceased processing".to_string(),
            "Notified all affected data subjects".to_string(),
        ],
        technical_organizational_measures: vec![
            "Implemented new data governance framework".to_string(),
        ],
        previous_violations: vec![],
        cooperated_with_authority: true,
        special_categories_involved: false,
        breach_notification_timely: None,
        certifications: vec![],
        other_aggravating: vec![],
        other_mitigating: vec!["First-time violation".to_string()],
        financial_benefit_gained: None,
    };

    let fine = AdministrativeFine::new()
        .with_controller("Tech Giant Corp")
        .with_violation(ViolatedArticle::Article6LawfulBasis)
        .with_turnover_eur(50_000_000_000.0) // €50 billion
        .with_factors(factors);

    let calc = fine.calculate_maximum()?;

    println!("Tier:                       {:?}", calc.tier);
    println!("Statutory maximum:          €{:.0}M", calc.statutory_maximum_eur / 1_000_000.0);
    println!(
        "Turnover-based max (4 %):   €{:.0}M",
        calc.turnover_based_maximum_eur.unwrap_or(0.0) / 1_000_000.0
    );
    println!("Applicable maximum:         €{:.0}M", calc.maximum_fine_eur / 1_000_000.0);
    println!("Severity score:             {:.1} %", calc.severity_score * 100.0);
    println!("Suggested fine:             {}", calc.format_amount());

    for factor in &calc.factors_summary {
        println!("  {factor}");
    }

    Ok(())
}
```

### Worst-case scenario — intentional violation with aggravating factors

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factors = Article83Factors {
        duration_months: Some(36),
        data_subjects_affected: Some(2_000_000),
        damage_suffered: Some(10_000_000.0),
        intentional: true,
        mitigation_actions_taken: vec![],
        technical_organizational_measures: vec![],
        previous_violations: vec![
            "2021 Article 6 violation (€5M fine)".to_string(),
            "2022 Article 32 security violation".to_string(),
        ],
        cooperated_with_authority: false,
        special_categories_involved: true, // Health data
        breach_notification_timely: Some(false),
        certifications: vec![],
        other_aggravating: vec![
            "Attempted to conceal violation from authority".to_string(),
        ],
        other_mitigating: vec![],
        financial_benefit_gained: Some(50_000_000.0),
    };

    let fine = AdministrativeFine::new()
        .with_controller("Bad Actor Inc")
        .with_violation(ViolatedArticle::Article9SpecialCategories)
        .with_turnover_eur(5_000_000_000.0) // €5 billion
        .with_factors(factors);

    let calc = fine.calculate_maximum()?;

    println!("Maximum fine:   €{:.0}M", calc.maximum_fine_eur / 1_000_000.0);
    println!("Severity score: {:.1} %", calc.severity_score * 100.0);
    println!(
        "Suggested fine: {} ({:.1} % of maximum)",
        calc.format_amount(),
        (calc.suggested_fine_eur / calc.maximum_fine_eur) * 100.0
    );

    Ok(())
}
```

### Lower-tier violation — child consent (Art. 8)

```rust
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factors = Article83Factors {
        duration_months: Some(6),
        data_subjects_affected: Some(1_500),
        intentional: false,
        mitigation_actions_taken: vec![
            "Immediately implemented age verification".to_string(),
            "Deleted data of affected minors".to_string(),
            "Notified parents / guardians".to_string(),
        ],
        cooperated_with_authority: true,
        ..Default::default()
    };

    let fine = AdministrativeFine::new()
        .with_controller("Social Media Startup")
        .with_violation(ViolatedArticle::Article8ChildConsent)
        .with_turnover_eur(50_000_000.0)
        .with_factors(factors);

    let calc = fine.calculate_maximum()?;

    println!("Tier:           {:?} (up to €10M or 2 %)", calc.tier);
    println!("Severity score: {:.1} %", calc.severity_score * 100.0);
    println!("Suggested fine: {}", calc.format_amount());

    Ok(())
}
```

> **Important disclaimer.** `calculate_maximum()` returns an indicative estimate based
> on the Art. 83(2) factors you supply. Real fines are determined by supervisory
> authorities and may differ substantially. Use this tool to understand your risk
> exposure and drive investment in compliance, not to predict an exact outcome.

---

## 10. Building a Complete Compliance Workflow

This section assembles everything from the previous sections into a single coherent
compliance workflow for **TechShop Europe GmbH** — the same scenario used in
`examples/gdpr_complete_compliance_workflow.rs`.

The four pillars of the workflow are:

| Step | Article | Question answered |
|---|---|---|
| 1 | Art. 6 | *Why* are we processing? (lawful basis) |
| 2 | Art. 32 | *How* do we protect the data? (security) |
| 3 | Art. 28 | *Who* helps us? (processor contract) |
| 4 | Art. 24 | Can we *prove* we're accountable? (documentation) |

```rust
use chrono::Utc;
use legalis_eu::gdpr::security::RiskLevel;
use legalis_eu::gdpr::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ─── Step 1: Establish Lawful Basis (Article 6) ───────────────────────────

    let processing = DataProcessing::new()
        .with_controller("TechShop Europe GmbH")
        .with_purpose("Process customer orders and fulfill contracts")
        .add_data_category(PersonalDataCategory::Regular(
            "Name, email, shipping address, payment details".to_string(),
        ))
        .with_operations(vec![
            ProcessingOperation::Collection,
            ProcessingOperation::Storage,
            ProcessingOperation::Use,
            ProcessingOperation::Disclosure,
        ])
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    processing.validate()?;
    println!("✅ Step 1 — Lawful basis: Contract performance (Art. 6(1)(b))");

    // ─── Step 2: Security Measures (Article 32) ───────────────────────────────

    let security = SecurityAssessment::new()
        .with_entity("TechShop Europe GmbH")
        .with_risk_level(RiskLevel::High)
        .add_technical_measure(TechnicalMeasure::Encryption {
            data_at_rest: true,
            data_in_transit: true,
            algorithm: "AES-256, TLS 1.3".to_string(),
        })
        .add_technical_measure(TechnicalMeasure::Pseudonymisation {
            method: "Customer IDs in analytics".to_string(),
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
            frequency: "Quarterly GDPR training".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_implementation_costs_considered(true)
        .with_processing_context_considered(true);

    let sec_val = security.validate()?;
    println!(
        "✅ Step 2 — Security: {} technical + {} organisational measures",
        sec_val.technical_measures_count, sec_val.organizational_measures_count
    );

    // ─── Step 3: Processor Contract (Article 28) ──────────────────────────────

    let processor_contract = ProcessorContract::new()
        .with_controller("TechShop Europe GmbH", "dpo@techshop.eu")
        .with_processor("Amazon Web Services EMEA", "aws-privacy@amazon.com")
        .with_subject_matter("Cloud hosting of e-commerce platform")
        .with_processing_purpose("Website hosting and database storage")
        .add_data_category("Customer orders, names, addresses")
        .add_data_subject_category("TechShop customers")
        .with_all_mandatory_clauses()
        .with_notes(
            "ISO 27001, SOC 2 Type II certified. \
             International transfers to US with SCCs.",
        );

    let contract_val = processor_contract.validate()?;
    println!(
        "✅ Step 3 — Processor contract: Art. 28(3) clauses present = {}",
        contract_val.compliant
    );

    // ─── Step 4: Accountability Framework (Article 24) ────────────────────────

    let accountability = ControllerAccountability::new()
        .with_controller_name("TechShop Europe GmbH")
        .with_data_volume(DataVolume::Medium)
        .with_data_sensitivity(DataSensitivity::High)
        .with_risk_level_assessed(RiskLevel::High)
        .add_technical_measure(AccountabilityMeasure::SecurityMeasures {
            article32_compliant: true,
            documented: true,
            notes: Some("Art. 32 measures demonstrated in Step 2".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::ProcessorContracts {
            processors_identified: true,
            article28_contracts_in_place: true,
            notes: Some("AWS contract demonstrated in Step 3".to_string()),
        })
        .add_organizational_measure(AccountabilityMeasure::StaffTraining {
            training_program_established: true,
            frequency: Some("Quarterly".to_string()),
            notes: None,
        })
        .add_organizational_measure(AccountabilityMeasure::DataSubjectRightsProcedures {
            procedures_documented: true,
            response_process_established: true,
            notes: Some("30-day response SLA established".to_string()),
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

    let acc_val = accountability.validate()?;
    println!(
        "✅ Step 4 — Accountability score: {}/100 — {} recommendations",
        acc_val.compliance_score,
        acc_val.recommendations.len()
    );

    for rec in &acc_val.recommendations {
        println!("   💡 {rec}");
    }

    println!("\n🎉 GDPR compliance workflow complete for TechShop Europe GmbH");
    Ok(())
}
```

### Integration flow summary

```
Article 6  →  WHY we process       (lawful basis)
     ↓
Article 32  →  HOW we protect       (encryption, access control, backups)
     ↓
Article 28  →  WHO helps us         (processor contracts)
     ↓
Article 24  →  PROOF of compliance  (documentation, score, certifications)
```

Each step feeds into the next: the lawful basis (Step 1) determines which security
measures are proportionate (Step 2); the processor contract (Step 3) must reference
those security measures; the accountability framework (Step 4) bundles all of the above
into a package you can present to a supervisory authority during an audit.

---

## 11. Next Steps

### Extend the workflow

| Capability | Module | Starting point |
|---|---|---|
| DSAR response pipeline | `legalis_eu::gdpr::*` | Section 5 of this tutorial |
| Breach incident register | `legalis_eu::gdpr::*` | Section 6 of this tutorial |
| DPIA library for new features | `legalis_eu::gdpr::dpia::*` | Section 7 of this tutorial |
| Cross-border transfer register | `legalis_eu::gdpr::cross_border::*` | Section 8 |
| Fine exposure dashboard | `legalis_eu::gdpr::*` | Section 9 |

### Further reading

- **`GDPR_GUIDE.md`** in this `docs/` folder — comprehensive API reference for every
  GDPR module.
- **`QUICKSTART.md`** — five-minute setup guide.
- **`FAQ.md`** — frequently asked legal and technical questions.
- **`examples/`** directory — runnable examples for every scenario in this tutorial.

### Running the examples

```bash
# Complete compliance workflow (the backbone of this tutorial)
cargo run --example gdpr_complete_compliance_workflow

# Individual topic examples
cargo run --example gdpr_consent_validation
cargo run --example gdpr_dsar_handling
cargo run --example gdpr_breach_notification
cargo run --example gdpr_dpia
cargo run --example gdpr_cross_border_transfers
cargo run --example gdpr_article83_fines
```

### Keep up to date

GDPR law evolves through supervisory authority guidance, EDPB opinions, and CJEU
rulings. Watch the `CHANGELOG` for updates to the `legalis-eu` crate that reflect new
legal developments (e.g. updated adequacy decisions, new SCC modules, or revised EDPB
guidance on legitimate interests).

---

*Tutorial version: legalis-eu 0.1.7 · Last updated: 2026-06-22*
*Copyright © COOLJAPAN OU (Team Kitasan). All rights reserved.*
