# Best Practices for legalis-eu

> Production patterns for GDPR compliance validation using the `legalis-eu` crate.
> Each section follows the pattern → rationale → code example → pitfall structure.

## Table of Contents

1. [Lawful Basis Selection](#1-lawful-basis-selection)
2. [Consent Quality & Validity](#2-consent-quality--validity)
3. [ROPA Hygiene (Records of Processing)](#3-ropa-hygiene-records-of-processing)
4. [DPIA Trigger Criteria](#4-dpia-trigger-criteria)
5. [Cross-Border Transfer Mechanism Selection](#5-cross-border-transfer-mechanism-selection)
6. [Member-State Override Handling](#6-member-state-override-handling)
7. [Error Handling with GdprError](#7-error-handling-with-gdprerror)
8. [Multilingual / i18n Error Messages](#8-multilingual--i18n-error-messages)
9. [Testing Compliance Logic](#9-testing-compliance-logic)
10. [Performance Patterns](#10-performance-patterns)

[Quick Reference Card](#quick-reference-card)

---

## 1. Lawful Basis Selection

**Why:** Article 6(1) GDPR requires processing to rest on exactly one of six enumerated
lawful bases. Choosing the wrong basis is one of the most common GDPR violations and
typically triggers upper-tier Article 83(5) fines (up to €20M or 4% global turnover).
The selection is permanent at the time of collection — a controller cannot switch bases
retroactively. `LawfulBasis` encodes all six options as a well-typed enum so the
compiler catches missing or malformed bases before they reach production.

**How:**

```rust
use legalis_eu::gdpr::{
    DataProcessing, LawfulBasis, PersonalDataCategory, ProcessingOperation,
};

// ✅ Contract performance (Article 6(1)(b))
// Use for processing that is strictly necessary to fulfill a contract.
// Do NOT use for data collected beyond what the contract requires.
let order_processing = DataProcessing::new()
    .with_controller("Online Retailer Ltd")
    .with_purpose("Order fulfilment and shipping")
    .add_data_category(PersonalDataCategory::Regular("shipping address".into()))
    .add_data_category(PersonalDataCategory::Regular("phone number".into()))
    .with_operation(ProcessingOperation::Collection)
    .with_operation(ProcessingOperation::Use)
    .with_lawful_basis(LawfulBasis::Contract {
        necessary_for_performance: true,
    });

match order_processing.validate() {
    Ok(v) if v.is_compliant() => println!("✅ Lawful basis valid"),
    Ok(v) => println!("⚠️ Requires review: {:?}", v.compliance_status),
    Err(e) => eprintln!("❌ {}", e),
}

// ✅ Legitimate interests (Article 6(1)(f))
// Requires a completed balancing test (Recital 47).
// Set balancing_test_passed: false only when the test has not yet been performed —
// the crate returns LegalResult::JudicialDiscretion to signal human review is needed.
let fraud_detection = DataProcessing::new()
    .with_controller("FinServ GmbH")
    .with_purpose("Fraud detection and prevention")
    .add_data_category(PersonalDataCategory::Regular("transaction history".into()))
    .with_operation(ProcessingOperation::Use)
    .with_lawful_basis(LawfulBasis::LegitimateInterests {
        controller_interest: "Preventing fraudulent transactions to protect customers"
            .into(),
        balancing_test_passed: true, // Document this assessment in your ROPA
    });

// ✅ Legal obligation (Article 6(1)(c))
// Cite the specific EU or Member State law that compels the processing.
let tax_records = DataProcessing::new()
    .with_controller("Accounting Firm SA")
    .with_purpose("Tax record retention")
    .add_data_category(PersonalDataCategory::Regular("invoice data".into()))
    .with_operation(ProcessingOperation::Storage)
    .with_lawful_basis(LawfulBasis::LegalObligation {
        eu_law: None,
        member_state_law: Some("§ 147 AO (German Fiscal Code)".into()),
    });
```

**Pitfall:** Using `Consent` as the default lawful basis for all processing. Consent is
the weakest basis — it can be withdrawn at any time and imposes the heaviest ongoing
obligations (withdrawal mechanism, re-consent after purpose change). Prefer `Contract`
or `LegitimateInterests` where appropriate and reserve `Consent` for genuinely optional
processing (marketing, cookies, etc.).

---

## 2. Consent Quality & Validity

**Why:** Article 7 GDPR and Recital 32 set strict validity criteria for consent: it must
be freely given, specific, informed, and indicated through an unambiguous affirmative
act. A pre-ticked checkbox, bundled consent, or consent conditioned on service access is
invalid. The `ConsentQuality` struct mirrors these five requirements and its `is_valid()`
method performs the Article 7 check. Using this type as the canonical validation point
prevents subtle bugs where a system accepts consent that no DPA would uphold.

**How:**

```rust
use legalis_eu::gdpr::{
    ConsentQuality, ConsentRecord, DataProcessing, LawfulBasis, PersonalDataCategory,
    ProcessingOperation,
};
use chrono::Utc;

// Check consent quality before recording it.
fn record_marketing_consent(
    freely_given: bool,
    specific: bool,
    informed: bool,
    unambiguous: bool,
    easily_withdrawable: bool,
) -> Result<ConsentRecord, String> {
    let quality = ConsentQuality {
        freely_given,
        specific,
        informed,
        unambiguous,
        easily_withdrawable,
    };

    if !quality.is_valid() {
        return Err(
            "Consent does not meet Article 7 requirements — do not record".into()
        );
    }

    Ok(ConsentRecord {
        purpose: "Email marketing".into(),
        timestamp: Utc::now(),
        freely_given: quality.freely_given,
        specific: quality.specific,
        informed: quality.informed,
        unambiguous: quality.unambiguous,
    })
}

// Wire ConsentQuality flags into the DataProcessing builder.
let processing = DataProcessing::new()
    .with_controller("Acme Corporation")
    .with_purpose("Email marketing for product announcements")
    .add_data_category(PersonalDataCategory::Regular("email address".into()))
    .add_data_category(PersonalDataCategory::Regular("full name".into()))
    .with_operation(ProcessingOperation::Collection)
    .with_operation(ProcessingOperation::Storage)
    .with_operation(ProcessingOperation::Use)
    .with_lawful_basis(LawfulBasis::Consent {
        freely_given: true,
        specific: true,     // One checkbox per purpose — never bundle
        informed: true,     // Privacy notice shown before submission
        unambiguous: true,  // Active opt-in, not pre-ticked
    });

match processing.validate() {
    Ok(v) if v.is_compliant() => println!("✅ Consent-based processing valid"),
    Ok(_) => println!("⚠️ Requires additional review"),
    Err(e) => eprintln!("❌ {}", e),
}
```

**Pitfall:** Passing all boolean flags as `true` without verifying the actual UX
implementation. Consent validity depends on how the form is presented, not on what the
back-end records. Pair `ConsentQuality::is_valid()` with a documented UX review that
confirms: no service conditioning, separate granular checkboxes per purpose, and a
withdrawal link as prominent as the sign-up flow.

---

## 3. ROPA Hygiene (Records of Processing)

**Why:** Article 30 GDPR requires every controller and processor to maintain a written
record of processing activities (ROPA). The exemption for organisations with fewer than
250 employees is narrow — it disappears the moment any processing involves special
categories (Article 9), criminal conviction data (Article 10), is non-occasional, or is
likely to result in a risk. Supervisory authorities routinely request ROPAs during
investigations; an absent or incomplete record is itself an Article 30 infringement.
`ProcessingRecord` with its builder API enforces completeness at compile time.

**How:**

```rust
use legalis_eu::gdpr::{
    ropa::{EntityType, ProcessingRecord},
    types::{LawfulBasis, PersonalDataCategory, ProcessingOperation},
};

let crm_record = ProcessingRecord::new()
    .with_name("Customer Relationship Management")
    .with_controller("Acme Corp", "privacy@acme.example")
    .with_purpose("Customer service and post-sale support")
    .add_data_subject_category("customers")
    .add_data_subject_category("prospective customers")
    .add_data_category(PersonalDataCategory::Regular("name".into()))
    .add_data_category(PersonalDataCategory::Regular("email address".into()))
    .add_data_category(PersonalDataCategory::Regular("purchase history".into()))
    .add_recipient("Customer support team (internal)")
    .add_recipient("CRM software provider (processor, Article 28 DPA in place)")
    .with_retention_period("7 years after last contact (tax law obligation)")
    .add_security_measure("TLS 1.3 in transit")
    .add_security_measure("AES-256 at rest")
    .add_security_measure("Role-based access control — support staff only");

match crm_record.validate() {
    Ok(_) => println!("✅ ROPA record complete"),
    Err(e) => eprintln!("❌ Incomplete record: {}", e),
}

// Check exemption status before deciding whether to maintain a record.
// Only organisations with <250 employees and purely occasional, low-risk
// processing may skip the ROPA — and the exemption is rarely available.
let exemption = crm_record.is_exempt(/* employee_count: */ 180);
// Log the exemption decision and keep it on file.
println!("Exemption status: {:?}", exemption);
```

**Pitfall:** Treating the ROPA as a one-time document. Article 30(1)(f) requires records
to reflect *current* retention periods and *current* recipients. When you add a new
processor or change a retention schedule, update the corresponding `ProcessingRecord` in
the same pull request that deploys the change — link the record ID in your commit
message so auditors can trace the change history.

---

## 4. DPIA Trigger Criteria

**Why:** Article 35(1) GDPR mandates a Data Protection Impact Assessment whenever
processing is "likely to result in a high risk to the rights and freedoms of natural
persons." The WP29/EDPB guidelines identify nine criteria; meeting two or more
independently triggers the obligation. Missing a required DPIA is an Article 83(4)
violation (up to €10M or 2% global turnover). The `DpiaTrigger` enum encodes each
statutory trigger, and `DataProtectionImpactAssessment::validate()` signals when prior
consultation with the supervisory authority (Article 36) is also required.

**How:**

```rust
use legalis_eu::gdpr::dpia::{
    DataProtectionImpactAssessment, DpiaTrigger, Effectiveness, Likelihood,
    Mitigation, RiskAssessment, RiskType, Severity,
};

// Example: AI-driven recruitment screening — two triggers present.
let dpia = DataProtectionImpactAssessment::new()
    .with_processing_description("AI-powered CV screening and candidate ranking")
    .with_purpose("Automated shortlisting of job applicants")
    // Trigger 1: Article 35(3)(a) — automated decision-making with legal effects
    .add_trigger(DpiaTrigger::AutomatedDecisionMaking {
        produces_legal_effects: true,  // Affects whether candidate is considered
        systematic: true,
        extensive: true,
    })
    .with_necessity_assessment(
        "Required to process 50,000+ applications per year within statutory deadlines",
    )
    .add_risk(RiskAssessment {
        risk_type: RiskType::Discrimination,
        likelihood: Likelihood::High,
        severity: Severity::High,
        description: "Model may propagate historical hiring bias against protected groups"
            .into(),
    })
    .add_risk(RiskAssessment {
        risk_type: RiskType::LackOfTransparency,
        likelihood: Likelihood::Medium,
        severity: Severity::High,
        description: "Candidates cannot understand or contest algorithmic ranking"
            .into(),
    })
    .add_mitigation(Mitigation {
        risk_addressed: RiskType::Discrimination,
        measure: "Quarterly third-party algorithmic fairness audit".into(),
        effectiveness: Effectiveness::High,
    })
    .add_mitigation(Mitigation {
        risk_addressed: RiskType::LackOfTransparency,
        measure: "Human review of every AI rejection before final decision".into(),
        effectiveness: Effectiveness::High,
    });

match dpia.validate() {
    Ok(result) => {
        if result.prior_consultation_required {
            println!(
                "⚠️ Prior consultation with supervisory authority required (Article 36)"
            );
        } else {
            println!("✅ DPIA complete — processing may proceed");
        }
    }
    Err(e) => eprintln!("❌ DPIA incomplete: {}", e),
}
```

**Pitfall:** Performing the DPIA *after* deployment. Article 35(10) requires the DPIA
before processing begins. Build DPIA validation into your feature gate or compliance
checklist, and require `dpia.validate()` to return `Ok` before the feature flag is
enabled in production.

---

## 5. Cross-Border Transfer Mechanism Selection

**Why:** Chapter V GDPR prohibits transferring personal data to a third country unless
an adequate level of protection is ensured. The *Schrems II* judgment (C-311/18, 2020)
invalidated Privacy Shield and placed a strict Transfer Impact Assessment (TIA)
obligation on every controller relying on Standard Contractual Clauses. The hierarchy
is: adequacy decision (Article 45) → SCCs or BCRs (Article 46) → Article 49 derogations
(exceptional use only). Choosing the correct mechanism avoids the enforcement pattern
seen in post-Schrems II DPA decisions across the EU. The `CrossBorderTransfer` builder
makes the mechanism explicit and rejects known-invalid configurations at validation time.

**How:**

```rust
use legalis_eu::gdpr::cross_border::{
    AdequateCountry, CrossBorderTransfer, TransferDerogation, TransferSafeguard,
};
use legalis_eu::shared::MemberState;
use chrono::Utc;

// ✅ Tier 1: Adequacy decision (simplest, no additional safeguards needed)
let to_switzerland = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("Switzerland")
    .with_adequate_destination(AdequateCountry::Switzerland)
    .add_data_category("customer names")
    .add_data_category("email addresses")
    .with_purpose("Cloud storage backup");

match to_switzerland.validate() {
    Ok(v) => println!("Transfer permitted: {:?}", v.transfer_permitted),
    Err(e) => eprintln!("❌ {}", e),
}

// ✅ Tier 2: Standard Contractual Clauses — must use 2021 version
// (old 2010 SCCs expired 27 June 2022; the crate rejects them)
let to_us_analytics = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses {
        version: "2021".into(), // Commission Implementing Decision 2021/914
        clauses_signed: true,
    })
    .add_data_category("user profiles")
    .with_purpose("Aggregate analytics");

match to_us_analytics.validate() {
    Ok(v) => {
        if v.risk_assessment_required {
            // Schrems II: conduct a TIA before transferring
            println!("⚠️ Transfer Impact Assessment required for US destination");
        }
    }
    Err(e) => eprintln!("❌ {}", e),
}

// ✅ Tier 2: Binding Corporate Rules (intra-group transfers)
let to_singapore_hr = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("Singapore")
    .with_safeguard(TransferSafeguard::BindingCorporateRules {
        approved_by: MemberState::Ireland,
        approval_date: Utc::now() - chrono::Duration::days(365),
    })
    .add_data_category("employee data")
    .with_purpose("HR management");

// ⚠️ Tier 3: Derogations — for occasional, specific situations only
// Article 49(1)(g) compelling legitimate interests: strict limits apply
let to_india_legal = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("India")
    .with_derogation(TransferDerogation::CompellingLegitimateInterests {
        affected_data_subjects: 3, // Must be a small number
        is_repetitive: false,       // Must be non-repetitive
    })
    .add_data_category("contract details")
    .with_purpose("One-off international arbitration");
```

**Pitfall:** Using Article 49 derogations as a routine transfer mechanism. The EDPB
Opinion 3/2018 and Recital 113 make clear that derogations are a last resort for
specific situations, not a substitute for SCCs or BCRs in ongoing business operations.
The `CompellingLegitimateInterests` variant explicitly carries `is_repetitive: bool` and
an `affected_data_subjects: u32` limit to enforce this constraint — transfers that exceed
the threshold are rejected at validation time.

---

## 6. Member-State Override Handling

**Why:** Despite being directly applicable in all 27 member states, the GDPR contains
over 50 opening clauses (*Öffnungsklauseln*) that permit — or require — member states
to specify the regulation in national law. Key examples include the age of digital
consent (Article 8(1): default 16, but France set 15, Italy 14), mandatory DPO
designation beyond Article 37(1) (Germany's BDSG requires a DPO from 20+ persons
processing), and employment data protections (Article 88). A controller operating
across multiple member states must apply the nationally-effective rule, not just the
GDPR core. `MemberStateGdpr` and `NationalGdprQuery` provide the national-law layer
on top of the GDPR core API.

**How:**

```rust
use legalis_eu::{
    member_states::{
        combined_consent_assessment, effective_age_of_digital_consent,
        NationalGdprQuery, OpeningClause,
    },
    shared::MemberState,
};

// Query #1: Resolve the effective age of digital consent for a user's country.
// Use this BEFORE accepting consent from a user who may be a child.
fn verify_child_consent(member_state: MemberState, user_age: u8) -> bool {
    let assessment = combined_consent_assessment(member_state, user_age);

    if assessment.parental_consent_required {
        // Log: parental consent needed — do not proceed with self-consent
        println!(
            "Child aged {} in {:?} requires parental consent (applicable age: {})",
            user_age, member_state, assessment.applicable_age_of_consent
        );
        return false;
    }

    // The child can consent independently under national law
    true
}

// A 14-year-old in Italy can consent (national age: 14);
// the same child in France cannot (national age: 15).
assert!(verify_child_consent(MemberState::Italy, 14));
assert!(!verify_child_consent(MemberState::France, 14));

// For states without a national implementation the GDPR default of 16 applies.
let effective_age = effective_age_of_digital_consent(MemberState::Spain);
assert_eq!(effective_age, 16); // Spain -> GDPR default

// Query #2: Enumerate national derogations for a specific opening clause.
// Use NationalGdprQuery when you need to check *what* national law says,
// not just whether a child can consent.
match NationalGdprQuery::new(MemberState::Germany) {
    Ok(query) => {
        println!(
            "Lead DPA: {} ({})",
            query.supervisory_authority().name_en,
            query.supervisory_authority().abbreviation, // "BfDI"
        );

        let employment_derogations = query.derogations_for(OpeningClause::Article88Employment);
        println!(
            "Germany has {} employment-context derogation(s)",
            employment_derogations.len()
        );

        // Produce a combined child-consent assessment via the query facade.
        let assessment = query.assess_child_consent(15);
        println!(
            "15-year-old in Germany can consent: {}",
            assessment.child_can_consent // false — German age is 16
        );
    }
    // NationalGdprQuery::new returns Err for unmodelled member states.
    // Fall back to GDPR core defaults in that case.
    Err(_) => println!("No national implementation — applying GDPR defaults"),
}
```

**Pitfall:** Assuming the GDPR default of 16 applies everywhere. France (15) and Italy
(14) have lower thresholds under Article 8(1). A service that displays a single
age-gate of "16+" will incorrectly reject valid consents in Italy and France. Always
resolve the effective age via `effective_age_of_digital_consent(state)` or
`combined_consent_assessment(state, age)` rather than hard-coding 16.

---

## 7. Error Handling with GdprError

**Why:** `GdprError` is the single error type returned by all GDPR validation
operations. Each variant maps to a distinct compliance issue with a different remediation
path. Collapsing all variants into a generic `eprintln!("{}", e)` loses the structured
information needed to route errors to the right team (legal, product, engineering) and
to populate user-facing messages in the correct language. Exhaustive pattern matching
ensures that newly added variants (added in minor releases) are caught at compile time.

**How:**

```rust
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::{DataProcessing, PersonalDataCategory, ProcessingOperation};

fn handle_validation_error(err: GdprError) {
    match err {
        // Structural issues — fix in code, not at runtime
        GdprError::MissingField(ref field) => {
            eprintln!("BUG: required field '{}' not set on DataProcessing", field);
        }

        // Missing lawful basis — the most common Article 6 violation
        GdprError::MissingLawfulBasis => {
            eprintln!("No lawful basis provided. Add .with_lawful_basis() to the builder.");
        }

        // Invalid lawful basis — e.g. Consent with freely_given: false
        GdprError::InvalidLawfulBasis { ref reason } => {
            eprintln!("Lawful basis invalid: {}. Review Article 6(1) requirements.", reason);
        }

        // Consent-specific validation failure (Article 7)
        GdprError::InvalidConsent { ref reason } => {
            eprintln!(
                "Consent invalid: {}. Ensure freely_given, specific, informed, unambiguous.",
                reason
            );
        }

        // No data categories declared — always required
        GdprError::NoDataCategories => {
            eprintln!("No PersonalDataCategory declared. Use add_data_category().");
        }

        // Special category data without an Article 9 exception
        GdprError::SpecialCategoryWithoutException => {
            eprintln!(
                "Special category data detected. Declare an Article 9(2) exception \
                 (e.g. explicit consent, health care necessity)."
            );
        }

        // Chapter V transfer violation
        GdprError::InvalidTransfer { ref reason } => {
            eprintln!("Cross-border transfer invalid: {}. Check Chapter V mechanism.", reason);
        }

        // Data subject rights request handling failure (Articles 15-22)
        GdprError::InvalidRequest { ref reason } => {
            eprintln!("Data subject request error: {}. Review DSAR handling procedure.", reason);
        }

        // Article 33: 72-hour notification window exceeded
        GdprError::BreachNotificationLate { hours } => {
            eprintln!(
                "Breach notification {} hour(s) late. Contact DPA immediately \
                 and document delay reasons.",
                hours
            );
        }

        // Batch of violations — iterate for full picture
        GdprError::MultipleViolations(ref violations) => {
            eprintln!("{} GDPR violations detected:", violations.len());
            for v in violations {
                eprintln!("  • {}", v);
            }
        }

        // Processing operation not permitted under the declared basis
        GdprError::OperationNotPermitted { ref operation } => {
            eprintln!(
                "Operation '{}' not permitted for this lawful basis. \
                 Review purpose limitation (Article 5(1)(b)).",
                operation
            );
        }

        // Article 8: child needs verified parental/guardian consent
        GdprError::ChildConsentRequired => {
            eprintln!(
                "Data subject is a child. Obtain and verify parental consent \
                 before processing."
            );
        }

        // Generic field-value validation failure
        GdprError::InvalidValue { ref field, ref reason } => {
            eprintln!("Field '{}' has invalid value: {}", field, reason);
        }
    }
}
```

**Pitfall:** Using `if let Err(GdprError::MissingLawfulBasis) = result` and ignoring
the `_` arm. New `GdprError` variants are added in minor releases; a non-exhaustive
match silently swallows them. Always use exhaustive `match` as shown above. The compiler
will report any unhandled variants immediately after a crate upgrade.

---

## 8. Multilingual / i18n Error Messages

**Why:** GDPR Article 12(1) requires privacy information — and by extension compliance
error messages displayed to data subjects — to be provided "in a concise, transparent,
intelligible and easily accessible form, using clear and plain language." The EU has 24
official languages. `MultilingualText` stores a message in up to 11 EU languages
(English, German, French, Spanish, Italian, Polish, Dutch, Portuguese, Swedish, Czech,
and Greek) with automatic fallback to English for unsupported locales. `GdprError`
exposes `.message(lang)` which internally calls `to_multilingual().in_language(lang)`,
making localized error surfacing a one-liner.

**How:**

```rust
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::MultilingualText;

// Pattern 1: Localize a GdprError directly via .message(lang)
fn user_facing_error(err: &GdprError, user_locale: &str) -> String {
    // Returns the error in the user's language with English fallback.
    err.message(user_locale)
}

let err = GdprError::MissingLawfulBasis;
assert_eq!(
    user_facing_error(&err, "de"),
    "Keine Rechtsgrundlage für die Verarbeitung gemäß Artikel 6"
);
assert_eq!(
    user_facing_error(&err, "fr"),
    "Aucune base juridique pour le traitement en vertu de l'article 6"
);
// Unsupported locale falls back to English
assert_eq!(
    user_facing_error(&err, "ja"),
    "No lawful basis for processing under Article 6"
);

// Pattern 2: Build application-level MultilingualText for custom messages
// (e.g., privacy notice paragraphs, consent prompts)
let consent_prompt = MultilingualText::new(
    "We use your email address to send you product updates. You may withdraw \
     consent at any time.",
)
.with_de(
    "Wir verwenden Ihre E-Mail-Adresse, um Ihnen Produktaktualisierungen zu senden. \
     Sie können Ihre Einwilligung jederzeit widerrufen.",
)
.with_fr(
    "Nous utilisons votre adresse e-mail pour vous envoyer des mises à jour de produits. \
     Vous pouvez retirer votre consentement à tout moment.",
)
.with_nl(
    "Wij gebruiken uw e-mailadres om u productupdates te sturen. U kunt uw toestemming \
     te allen tijde intrekken.",
);

// Render in the user's preferred language.
fn render_for_locale<'a>(text: &'a MultilingualText, locale: &str) -> &'a str {
    text.in_language(locale)
}

println!("{}", render_for_locale(&consent_prompt, "de"));
// "Wir verwenden Ihre E-Mail-Adresse …"

println!("{}", render_for_locale(&consent_prompt, "it"));
// Falls back to English — Italian translation not added yet

// Pattern 3: Parametric error messages preserve interpolated values in all languages
let breach_err = GdprError::BreachNotificationLate { hours: 96 };
println!("{}", breach_err.message("pl"));
// "Przekroczono termin zgłoszenia naruszenia: 96 godzin opóźnienia"
```

**Pitfall:** Hard-coding English error strings in REST API responses. When a non-English
DPA auditor inspects your system logs or a data subject in Poland sees an error, an
English message both harms usability and may violate Article 12(1). Always pass the
`Accept-Language` header (or the user's stored locale preference) into `.message(lang)`
when surfacing `GdprError` variants to end users.

---

## 9. Testing Compliance Logic

**Why:** Compliance logic is subject to legal change (Commission decisions, CJEU rulings,
national transpositions) and must be regression-tested with the same rigour as business
logic. Because GDPR validation is pure (no I/O, no side effects), tests are fast and
hermetic. A well-structured test suite documents the legal requirements your code
enforces and immediately catches regressions when a dependency version changes a
validation rule.

**How:**

```rust
#[cfg(test)]
mod compliance_tests {
    use legalis_eu::gdpr::{
        error::GdprError,
        types::{
            ComplianceStatus, ConsentQuality, LawfulBasis, PersonalDataCategory,
            ProcessingOperation, SpecialCategory,
        },
        DataProcessing,
    };
    use legalis_eu::member_states::{
        combined_consent_assessment, effective_age_of_digital_consent,
    };
    use legalis_eu::shared::MemberState;

    // Document legal requirements as test names (cite the article).
    #[test]
    fn article6_requires_lawful_basis() {
        let processing = DataProcessing::new()
            .with_controller("Test Corp")
            .with_purpose("Testing")
            .add_data_category(PersonalDataCategory::Regular("name".into()));
        // No .with_lawful_basis() call
        assert_eq!(processing.validate(), Err(GdprError::MissingLawfulBasis));
    }

    #[test]
    fn article7_coerced_consent_is_invalid() {
        let processing = DataProcessing::new()
            .with_controller("Test Corp")
            .with_purpose("Marketing")
            .add_data_category(PersonalDataCategory::Regular("email".into()))
            .with_operation(ProcessingOperation::Use)
            .with_lawful_basis(LawfulBasis::Consent {
                freely_given: false, // Bundled or conditioned consent
                specific: true,
                informed: true,
                unambiguous: true,
            });
        let result = processing.validate();
        assert!(
            result.is_err(),
            "Coerced consent must be rejected"
        );
    }

    #[test]
    fn article9_special_category_flags_review() {
        let processing = DataProcessing::new()
            .with_controller("Hospital")
            .with_purpose("Patient records")
            .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData))
            .with_lawful_basis(LawfulBasis::Contract {
                necessary_for_performance: true,
            });
        let validation = processing.validate().expect("should not error");
        assert!(
            validation.requires_article9_exception,
            "Health data must trigger Article 9 exception requirement"
        );
    }

    #[test]
    fn article7_all_flags_required_for_valid_consent() {
        let consent = ConsentQuality {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: false, // Missing!
        };
        assert!(!consent.is_valid(), "Withdrawal must be easy under Article 7(3)");
    }

    // Member-state variations should be tested per jurisdiction.
    #[test]
    fn article8_age_of_consent_varies_by_member_state() {
        assert_eq!(effective_age_of_digital_consent(MemberState::Germany), 16);
        assert_eq!(effective_age_of_digital_consent(MemberState::France), 15);
        assert_eq!(effective_age_of_digital_consent(MemberState::Italy), 14);
        // Unmodelled states fall back to GDPR default
        assert_eq!(effective_age_of_digital_consent(MemberState::Spain), 16);
    }

    #[test]
    fn combined_assessment_reflects_national_law() {
        // 14-year-old: can consent in Italy, cannot in Germany or France
        let it = combined_consent_assessment(MemberState::Italy, 14);
        assert!(it.child_can_consent);
        assert!(it.national_implementation_applied);

        let de = combined_consent_assessment(MemberState::Germany, 14);
        assert!(!de.child_can_consent);
        assert!(de.parental_consent_required);

        let fr = combined_consent_assessment(MemberState::France, 14);
        assert!(!fr.child_can_consent);
        assert!(fr.parental_consent_required);
    }
}
```

**Pitfall:** Testing only the "happy path" (valid processing validates successfully).
GDPR compliance tests must be primarily negative — confirm that invalid configurations
are *rejected*. An incomplete test suite that only verifies `is_compliant() == true`
gives false confidence without catching regressions in the validation logic.

---

## 10. Performance Patterns

**Why:** GDPR validation is stateless and alloc-minimal; `DataProcessing::validate()`
makes no system calls and is suitable for the hot path of a high-throughput API gateway.
However, repeated construction of identical `DataProcessing` objects for the same
processing activity adds unnecessary allocation pressure. Two patterns eliminate this:
(a) cache the validated result for activities that change only when configuration
changes; (b) batch-validate across all processing activities and propagate
`MultipleViolations` as a single structured error rather than short-circuiting on the
first failure.

**How:**

```rust
use legalis_eu::gdpr::{
    error::GdprError,
    types::{
        ComplianceStatus, LawfulBasis, PersonalDataCategory, ProcessingOperation,
    },
    DataProcessing,
};

// Pattern 1: Build once, validate once, cache the result.
// If processing activities are known at startup (config-driven), build them
// during initialisation and store the `ProcessingValidation` results.

struct ProcessingRegistry {
    /// Pre-validated processing activities.
    validated: Vec<(String, Result<legalis_eu::gdpr::article6::ProcessingValidation, GdprError>)>,
}

impl ProcessingRegistry {
    fn from_definitions(definitions: Vec<DataProcessing>) -> Self {
        let validated = definitions
            .into_iter()
            .enumerate()
            .map(|(i, dp)| (format!("activity-{}", i), dp.validate()))
            .collect();
        Self { validated }
    }

    fn all_compliant(&self) -> bool {
        self.validated
            .iter()
            .all(|(_, r)| r.as_ref().map(|v| v.is_compliant()).unwrap_or(false))
    }

    fn violations(&self) -> Vec<String> {
        self.validated
            .iter()
            .filter_map(|(name, r)| match r {
                Err(e) => Some(format!("{}: {}", name, e)),
                Ok(v) if !v.is_compliant() => {
                    Some(format!("{}: non-compliant", name))
                }
                Ok(_) => None,
            })
            .collect()
    }
}

// Pattern 2: Batch validation — collect all violations instead of short-circuiting.
fn validate_all(
    activities: &[DataProcessing],
) -> Result<(), GdprError> {
    let mut violation_messages: Vec<String> = Vec::new();

    for activity in activities {
        match activity.validate() {
            Err(e) => violation_messages.push(e.to_string()),
            Ok(v) => {
                if let ComplianceStatus::NonCompliant { violations } = v.compliance_status {
                    violation_messages.extend(violations);
                }
            }
        }
    }

    if violation_messages.is_empty() {
        Ok(())
    } else {
        Err(GdprError::MultipleViolations(violation_messages))
    }
}

// Example usage
let activities = vec![
    DataProcessing::new()
        .with_controller("Acme Corp")
        .with_purpose("CRM")
        .add_data_category(PersonalDataCategory::Regular("email".into()))
        .with_operation(ProcessingOperation::Storage)
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        }),
    DataProcessing::new()
        .with_controller("Acme Corp")
        .with_purpose("Analytics")
        .add_data_category(PersonalDataCategory::Regular("page views".into()))
        .with_operation(ProcessingOperation::Collection)
        .with_lawful_basis(LawfulBasis::LegitimateInterests {
            controller_interest: "Product improvement".into(),
            balancing_test_passed: true,
        }),
];

match validate_all(&activities) {
    Ok(()) => println!("✅ All activities compliant"),
    Err(GdprError::MultipleViolations(ref v)) => {
        eprintln!("{} violation(s):", v.len());
        for msg in v {
            eprintln!("  • {}", msg);
        }
    }
    Err(e) => eprintln!("❌ {}", e),
}
```

**Pitfall:** Constructing a fresh `DataProcessing` on every incoming HTTP request to
validate a processing activity whose definition has not changed. Validation is
deterministic — the same definition always produces the same result. Cache
`ProcessingValidation` results at service startup (or invalidate on config change) and
serve the cached result on the hot path to avoid redundant heap allocations.

---

## Quick Reference Card

| Goal | API | Notes |
|------|-----|-------|
| Validate a processing activity | `DataProcessing::new().…validate()` | Returns `Result<ProcessingValidation, GdprError>` |
| Check compliant | `validation.is_compliant()` | `ComplianceStatus::Compliant` |
| Detect special-category data | `validation.requires_article9_exception` | Must add Article 9(2) exception |
| Validate consent quality | `ConsentQuality { … }.is_valid()` | All 5 fields must be `true` |
| Effective age of digital consent | `effective_age_of_digital_consent(MemberState::X)` | Applies national override or GDPR default (16) |
| Check if child can consent | `combined_consent_assessment(state, age).child_can_consent` | Returns `CombinedConsentAssessment` |
| Query national GDPR implementation | `NationalGdprQuery::new(MemberState::X)?` | Err for unmodelled states |
| Lead supervisory authority | `query.supervisory_authority().abbreviation` | e.g. `"BfDI"`, `"CNIL"`, `"Garante"` |
| National derogations by clause | `query.derogations_for(OpeningClause::Article88Employment)` | Returns `Vec<&NationalDerogation>` |
| Validate cross-border transfer | `CrossBorderTransfer::new().…validate()` | Use 2021 SCCs; TIA required for US |
| Localize error for user | `gdpr_error.message("de")` | Falls back to English for unsupported locales |
| Custom multilingual text | `MultilingualText::new("en").with_de("…").with_fr("…")` | 11 EU languages supported |
| Render in user's language | `multilingual_text.in_language("fr")` | `&str`, always returns something |
| Estimate Article 83 fine | `AdministrativeFine::new().…calculate_maximum()` | Two-tier: 2%/€10M or 4%/€20M |
| Validate ROPA record | `ProcessingRecord::new().…validate()` | Returns `Result<(), GdprError>` |
| Check ROPA exemption | `record.is_exempt(employee_count)` | Returns `RopaExemption` |
| Validate DPIA | `DataProtectionImpactAssessment::new().…validate()` | Signals `prior_consultation_required` |
| Batch violations | `GdprError::MultipleViolations(Vec<String>)` | Collect before returning |

---

> 💡 **EU house note:** Article references in parentheses throughout this guide refer to
> Regulation (EU) 2016/679 (GDPR) unless stated otherwise. Recital references are
> non-binding but authoritative for interpretation. All code examples are compilable
> against the current `legalis-eu` API with no `unwrap()` calls.
