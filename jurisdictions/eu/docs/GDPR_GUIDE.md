# GDPR Implementation Guide

Complete guide to using legalis-eu for GDPR compliance validation.

## Table of Contents

- [Introduction](#introduction)
- [Core Concepts](#core-concepts)
- [Lawful Bases (Article 6)](#lawful-bases-article-6)
- [Special Categories (Article 9)](#special-categories-article-9)
- [Data Subject Rights](#data-subject-rights)
- [Security and Breaches](#security-and-breaches)
- [Cross-Border Transfers](#cross-border-transfers)
- [Accountability](#accountability)
- [Complete Examples](#complete-examples)

## Introduction

The GDPR (General Data Protection Regulation 2016/679) is the EU's comprehensive data protection framework. This crate provides type-safe validation of GDPR compliance requirements.

### What's Implemented

✅ **All 6 lawful bases** (Article 6)
✅ **Special categories** with 10 exceptions (Article 9)
✅ **All 7 data subject rights** (Articles 15-22)
✅ **Security & breach notification** (Articles 32-34)
✅ **Cross-border transfers** (Chapter V, Articles 44-49)
✅ **Accountability measures** (Articles 24-26, 28, 30)
✅ **DPIA** (Articles 35-36)
✅ **DPO requirements** (Articles 37-39)
✅ **Administrative fines** (Article 83)

## Core Concepts

### Data Processing

The fundamental unit is `DataProcessing`, representing any GDPR-regulated processing operation:

```rust
use legalis_eu::gdpr::*;

let processing = DataProcessing::new()
    .with_controller("Acme Corporation")
    .with_purpose("Customer relationship management")
    .add_data_category(PersonalDataCategory::Regular("name".into()))
    .add_data_category(PersonalDataCategory::Regular("email".into()))
    .with_lawful_basis(LawfulBasis::Contract {
        necessary_for_performance: true,
    });
```

### Validation

All operations validate against GDPR requirements:

```rust
match processing.validate() {
    Ok(validation) => {
        if validation.is_compliant() {
            println!("✅ GDPR compliant");
        } else {
            for warning in &validation.warnings {
                println!("⚠️ {}", warning);
            }
        }
    }
    Err(e) => {
        eprintln!("❌ Compliance error: {}", e);
    }
}
```

## Lawful Bases (Article 6)

Article 6 requires ONE of six lawful bases for processing. Choose carefully!

### 1. Consent (Article 6(1)(a))

**When to use**: Marketing, optional features, non-essential processing

**Requirements**: Freely given, specific, informed, unambiguous, and easily withdrawable

```rust
let processing = DataProcessing::new()
    .with_controller("Marketing Co")
    .with_purpose("Send promotional emails")
    .with_lawful_basis(LawfulBasis::Consent {
        freely_given: true,    // Not bundled with other consent
        specific: true,         // Clear what user consents to
        informed: true,         // User knows what will happen
        unambiguous: true,      // Clear affirmative action
    });
```

**⚠️ Common Mistakes:**
- ❌ Pre-ticked boxes (not unambiguous)
- ❌ Bundling consent (not freely given)
- ❌ Vague consent (not specific)

### 2. Contract (Article 6(1)(b))

**When to use**: Processing necessary to perform a contract

**Example**: Order fulfillment, account creation for service delivery

```rust
let processing = DataProcessing::new()
    .with_controller("E-commerce Shop")
    .with_purpose("Process and deliver customer orders")
    .add_data_category(PersonalDataCategory::Regular("shipping_address".into()))
    .with_lawful_basis(LawfulBasis::Contract {
        necessary_for_performance: true,
    });
```

**Key Point**: Must be **necessary** for the contract. You can't use this basis for optional processing.

### 3. Legal Obligation (Article 6(1)(c))

**When to use**: Required by EU or member state law

**Example**: Tax records, employment law requirements

```rust
let processing = DataProcessing::new()
    .with_controller("Employer Ltd")
    .with_purpose("Comply with tax reporting obligations")
    .with_lawful_basis(LawfulBasis::LegalObligation {
        eu_law: Some("Council Directive 2006/112/EC".into()),
        member_state_law: None,
    });
```

### 4. Vital Interests (Article 6(1)(d))

**When to use**: Life or death situations only

**Example**: Medical emergencies when consent cannot be obtained

```rust
let processing = DataProcessing::new()
    .with_controller("Emergency Services")
    .with_purpose("Emergency medical treatment")
    .with_lawful_basis(LawfulBasis::VitalInterests {
        life_threatening: true,
    });
```

**Note**: Rarely used. Only when absolutely necessary to save someone's life.

### 5. Public Task (Article 6(1)(e))

**When to use**: Public authorities performing official functions

```rust
let processing = DataProcessing::new()
    .with_controller("Tax Authority")
    .with_purpose("Tax assessment and collection")
    .with_lawful_basis(LawfulBasis::PublicTask {
        task_basis: "Article 5 National Tax Code".into(),
    });
```

### 6. Legitimate Interests (Article 6(1)(f))

**When to use**: Balancing your interests against data subject rights

**Requires**: Three-part test (legitimate interest + necessity + balancing)

```rust
let processing = DataProcessing::new()
    .with_controller("Security Co")
    .with_purpose("Fraud prevention and detection")
    .with_lawful_basis(LawfulBasis::LegitimateInterests {
        controller_interest: "Prevent fraudulent transactions".into(),
        balancing_test_passed: true, // You must perform the test!
    });
```

**Balancing Test Checklist:**
1. ✓ Is the interest legitimate?
2. ✓ Is processing necessary?
3. ✓ Do benefits outweigh data subject rights?
4. ✓ Would a reasonable person expect this processing?

## Special Categories (Article 9)

Special categories (sensitive data) require BOTH:
1. A lawful basis from Article 6, AND
2. An exception from Article 9

### Special Category Types

```rust
pub enum SpecialCategory {
    RacialEthnicOrigin,
    PoliticalOpinions,
    ReligiousBeliefs,
    TradeUnionMembership,
    GeneticData,
    BiometricData,      // For unique identification
    HealthData,
    SexLifeOrOrientation,
}
```

### Example: Healthcare

```rust
use legalis_eu::gdpr::*;

let processing = Article9Processing::new()
    .with_controller("City Hospital")
    .with_purpose("Patient treatment and care")
    .add_special_category(SpecialCategory::HealthData)
    .with_exception(Article9Exception::Healthcare {
        purpose: HealthcarePurpose::Treatment,
        healthcare_professional: true,
        subject_of_care: true,
    });

match processing.validate() {
    Ok(validation) => {
        if validation.exception_applies {
            println!("✅ Exception valid for health data processing");
        }
    }
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

### Available Exceptions

1. **Explicit Consent** (Article 9(2)(a))
2. **Employment/Social Security** (Article 9(2)(b))
3. **Vital Interests** (Article 9(2)(c))
4. **Non-profit Organizations** (Article 9(2)(d))
5. **Public Data** (Article 9(2)(e))
6. **Legal Claims** (Article 9(2)(f))
7. **Public Interest** (Article 9(2)(g))
8. **Healthcare** (Article 9(2)(h))
9. **Public Health** (Article 9(2)(i))
10. **Research** (Article 9(2)(j))

## Data Subject Rights

GDPR grants individuals powerful rights over their data.

### Right of Access (Article 15)

Individuals can request a copy of their data:

```rust
let request = DataSubjectRequest::new()
    .with_data_subject("user@example.com")
    .with_right(DataSubjectRight::Access)
    .with_controller("My Company");

match request.validate() {
    Ok(validation) => {
        println!("Response deadline: {} days", validation.deadline_days);
        // Must respond within 30 days (can extend by 60 more)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Right to Erasure (Article 17) - "Right to be Forgotten"

```rust
let erasure_request = DataSubjectRequest::new()
    .with_data_subject("user@example.com")
    .with_right(DataSubjectRight::Erasure)
    .with_controller("My Company")
    .with_grounds("Data no longer necessary for original purpose");

match erasure_request.validate() {
    Ok(validation) => {
        if !validation.exceptions.is_empty() {
            println!("⚠️ Erasure may be refused due to:");
            for exception in &validation.exceptions {
                println!("  - {}", exception);
            }
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Right to Data Portability (Article 20)

```rust
let portability = DataSubjectRequest::new()
    .with_data_subject("user@example.com")
    .with_right(DataSubjectRight::DataPortability)
    .with_controller("Social Network Inc");

// Must provide data in structured, commonly used, machine-readable format
```

## Security and Breaches

### Security Measures (Article 32)

```rust
use legalis_eu::gdpr::*;

let security = SecurityAssessment::new()
    .with_controller("Healthcare Provider")
    .with_risk_level(RiskLevel::High)  // High risk = stricter requirements
    .add_technical_measure(TechnicalMeasure::Encryption)
    .add_technical_measure(TechnicalMeasure::AccessControl)
    .add_organizational_measure(OrganizationalMeasure::StaffTraining)
    .add_organizational_measure(OrganizationalMeasure::IncidentResponse);

match security.validate() {
    Ok(validation) => {
        if validation.meets_article_32 {
            println!("✅ Security measures adequate for risk level");
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Data Breach Notification (Articles 33-34)

**72-hour rule**: Notify supervisory authority within 72 hours of discovery

```rust
use chrono::Utc;

let breach = DataBreach::new()
    .with_controller("Online Retailer")
    .with_breach_category(BreachCategory::ConfidentialityBreach)
    .with_discovered_at(Utc::now() - chrono::Duration::hours(60))
    .with_affected_count(5000)
    .with_severity(BreachSeverity::High);

match breach.validate() {
    Ok(validation) => {
        if validation.supervisory_authority_notification_required {
            let remaining = 72 - validation.hours_since_discovery;
            if remaining > 0 {
                println!("⏰ {} hours to notify authority", remaining);
            } else {
                println!("❌ 72-hour deadline EXCEEDED!");
            }
        }

        if validation.data_subject_notification_required {
            println!("⚠️ Must also notify affected individuals (high risk)");
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Cross-Border Transfers

Transferring data outside the EU/EEA requires special safeguards.

### Adequacy Decision (Article 45)

14 countries have adequacy decisions (as of 2026):

```rust
use legalis_eu::gdpr::cross_border::*;

let transfer = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("Switzerland")
    .with_adequate_destination(AdequateCountry::Switzerland);

match transfer.validate() {
    Ok(validation) => {
        if validation.permitted {
            println!("✅ Transfer permitted (adequacy decision)");
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Standard Contractual Clauses (Article 46)

For countries without adequacy:

```rust
let transfer = CrossBorderTransfer::new()
    .with_origin("EU")
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses {
        version: "2021".into(),
        clauses_signed: true,
    });

match transfer.validate() {
    Ok(validation) => {
        if validation.risk_assessment_required {
            println!("⚠️ Transfer Impact Assessment required (Schrems II)");
            println!("   Assess: Government surveillance, legal remedies");
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Accountability

### Data Protection Impact Assessment (DPIA) - Article 35

Required for high-risk processing:

```rust
use legalis_eu::gdpr::*;

let dpia = DataProtectionImpactAssessment::new()
    .with_controller("City Council")
    .with_description("Automated facial recognition in public spaces")
    .add_trigger(DpiaTrigger::SystematicMonitoring {
        scale: 100_000,  // Large-scale
        public_area: true,
    })
    .add_trigger(DpiaTrigger::NewTechnology {
        description: "AI-powered facial recognition".into(),
    });

match dpia.validate() {
    Ok(validation) => {
        if validation.dpia_required {
            println!("✅ DPIA is REQUIRED");
            println!("   Triggers: {}", validation.triggers.len());
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Data Protection Officer (DPO) - Articles 37-39

When is a DPO required?

```rust
let dpo_assessment = DpoDesignationAssessment::new()
    .with_organization("Hospital")
    .is_public_authority(true)
    .processes_large_scale_special_categories(true);

match dpo_assessment.assess() {
    Ok(assessment) => {
        if assessment.dpo_required {
            println!("✅ DPO designation REQUIRED");
            for reason in &assessment.reasons {
                println!("   - {}", reason);
            }
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Complete Examples

### Example 1: E-commerce Order Processing

```rust
use legalis_eu::gdpr::*;

fn validate_order_processing() -> Result<(), GdprError> {
    let processing = DataProcessing::new()
        .with_controller("Online Shop Ltd")
        .with_purpose("Process customer orders and deliver goods")
        .add_data_category(PersonalDataCategory::Regular("name".into()))
        .add_data_category(PersonalDataCategory::Regular("email".into()))
        .add_data_category(PersonalDataCategory::Regular("address".into()))
        .add_data_category(PersonalDataCategory::Regular("payment_info".into()))
        .with_lawful_basis(LawfulBasis::Contract {
            necessary_for_performance: true,
        });

    let validation = processing.validate()?;

    if validation.is_compliant() {
        println!("✅ Order processing is GDPR compliant");
        println!("   Lawful basis: Contract performance (Art. 6(1)(b))");
    }

    Ok(())
}
```

### Example 2: Marketing with Consent

```rust
fn validate_marketing() -> Result<(), GdprError> {
    let processing = DataProcessing::new()
        .with_controller("Marketing Agency")
        .with_purpose("Send targeted advertisements")
        .add_data_category(PersonalDataCategory::Regular("email".into()))
        .add_data_category(PersonalDataCategory::Regular("preferences".into()))
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    let validation = processing.validate()?;

    println!("✅ Marketing processing validated");
    println!("   Remember: Consent must be easily withdrawable!");

    Ok(())
}
```

### Example 3: Cross-Border SaaS

```rust
use legalis_eu::gdpr::cross_border::*;

fn validate_saas_transfer() -> Result<(), GdprError> {
    // EU company using US cloud provider
    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("US")
        .with_safeguard(TransferSafeguard::StandardContractualClauses {
            version: "2021".into(),
            clauses_signed: true,
        });

    let validation = transfer.validate()?;

    if validation.risk_assessment_required {
        println!("⚠️ Perform Transfer Impact Assessment (TIA):");
        println!("   1. Assess US government surveillance laws");
        println!("   2. Evaluate cloud provider's legal protections");
        println!("   3. Consider supplementary measures (encryption)");
    }

    Ok(())
}
```

## Best Practices

### 1. Choose the Right Lawful Basis

❌ **Don't**: Use consent when contract basis applies
✅ **Do**: Use contract for order fulfillment, consent for marketing

### 2. Document Everything

```rust
// Good: Clear documentation of lawful basis
.with_lawful_basis(LawfulBasis::LegitimateInterests {
    controller_interest: "Fraud prevention to protect customers and business",
    balancing_test_passed: true,  // Documented separately
})
```

### 3. Implement Privacy by Design

- Minimize data collection
- Use pseudonymization where possible
- Implement access controls
- Plan for data subject rights

### 4. Regular Reviews

- Review processing activities annually
- Update ROPAs when processing changes
- Re-assess DPIAs when risks change
- Train staff on GDPR compliance

## Common Pitfalls

### ❌ Mistake 1: Wrong Lawful Basis

```rust
// WRONG: Using consent for essential service functionality
.with_lawful_basis(LawfulBasis::Consent { ... })  // For account creation
```

```rust
// CORRECT: Use contract for essential functionality
.with_lawful_basis(LawfulBasis::Contract {
    necessary_for_performance: true,
})
```

### ❌ Mistake 2: Forgetting Special Category Exceptions

```rust
// WRONG: Processing health data without Article 9 exception
let processing = DataProcessing::new()
    .add_data_category(PersonalDataCategory::Special(SpecialCategory::HealthData));
    // Missing Article9Processing!
```

```rust
// CORRECT: Use Article9Processing
let processing = Article9Processing::new()
    .add_special_category(SpecialCategory::HealthData)
    .with_exception(Article9Exception::ExplicitConsent { ... });
```

### ❌ Mistake 3: Ignoring Transfer Requirements

```rust
// WRONG: Transferring to US without safeguards
// (Just sends data without compliance check)
```

```rust
// CORRECT: Validate transfer with appropriate safeguards
let transfer = CrossBorderTransfer::new()
    .with_destination_country("US")
    .with_safeguard(TransferSafeguard::StandardContractualClauses { ... });
```

## Further Reading

- **Official GDPR Text**: [EUR-Lex 32016R0679](https://eur-lex.europa.eu/eli/reg/2016/679/oj)
- **EDPB Guidelines**: [European Data Protection Board](https://edpb.europa.eu/)
- **Example Code**: See `examples/gdpr_*.rs` in this crate
- **API Docs**: Run `cargo doc --open`

## Need Help?

- Check the [FAQ](FAQ.md)
- Review the [examples](../examples/)
- Open an [issue](https://github.com/cool-japan/legalis/issues)
