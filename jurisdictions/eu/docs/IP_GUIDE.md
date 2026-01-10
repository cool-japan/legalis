# Intellectual Property Law Guide

Complete guide to EU IP protection validation using legalis-eu.

## Table of Contents

- [Overview](#overview)
- [EU Trademarks](#eu-trademarks)
- [Community Designs](#community-designs)
- [Copyright](#copyright)
- [Trade Secrets](#trade-secrets)
- [Layered IP Strategy](#layered-ip-strategy)

## Overview

The EU provides harmonized intellectual property protection across all member states through unitary EU-wide rights and directives.

### Four IP Protection Types

| Type | Regulation/Directive | Protection | Duration |
|------|---------------------|------------|----------|
| **Trademark** | Regulation (EU) 2017/1001 | Brand identity | 10 years (renewable indefinitely) |
| **Design** | Regulation (EC) No 6/2002 | Visual appearance | 25 years (RCD) / 3 years (UCD) |
| **Copyright** | InfoSoc 2001/29/EC + others | Creative works | Life + 70 years |
| **Trade Secret** | Directive (EU) 2016/943 | Confidential know-how | Indefinite (while secret) |

## EU Trademarks

### Registration Requirements

EU Trademarks (EUTM) provide pan-EU protection through a single registration at EUIPO (Alicante).

#### Basic Validation

```rust
use legalis_eu::intellectual_property::*;

let trademark = EuTrademark::new()
    .with_mark_text("INNOVATECH")
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Tech Company GmbH")
    .add_nice_class(9)   // Software
    .unwrap()
    .add_nice_class(42)  // IT services
    .unwrap()
    .add_goods_services("Computer software for data analysis");

match trademark.validate() {
    Ok(validation) => {
        if validation.is_registrable {
            println!("✅ Trademark is registrable");
            println!("   Distinctiveness: {}", validation.distinctiveness_established);
        }
    }
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

### Nice Classification

Trademarks must specify goods/services in one or more of 45 Nice Classes:

**Classes 1-34**: Goods
- Class 9: Computer software, electronics
- Class 25: Clothing, footwear
- Class 30: Coffee, tea, confectionery

**Classes 35-45**: Services
- Class 35: Advertising, business management
- Class 42: Scientific and technological services, software development
- Class 45: Legal services

```rust
// Multiple classes
let trademark = EuTrademark::new()
    .with_mark_text("TECHSTYLE")
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Fashion Tech Ltd")
    .add_nice_class(9)?   // Smart watches
    .add_nice_class(25)?  // Clothing
    .add_nice_class(35)?; // Retail services
```

### Mark Types

```rust
pub enum MarkType {
    WordMark,              // Text only: "NIKE"
    FigurativeMark,        // Logo/image
    CombinedMark,          // Text + logo: "McDonald's" with arches
    ThreeDimensionalMark,  // Shape: Coca-Cola bottle
    ColorMark,             // Color: Tiffany blue
    SoundMark,             // Sound: Intel chime
    MotionMark,            // Animation
    MultimediaMark,        // Audio + visual
    PositionMark,          // Position on product
    PatternMark,           // Repeated pattern
}
```

### Distinctiveness (Article 7)

Marks must be distinctive to be registrable:

#### ✅ Distinctive Marks (Registrable)

```rust
let distinctive = EuTrademark::new()
    .with_mark_text("APPLE")      // Arbitrary for computers
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Apple Inc")
    .add_nice_class(9)?;

// "APPLE" is distinctive for computers (arbitrary mark)
```

#### ❌ Descriptive Marks (Not Registrable)

```rust
let descriptive = EuTrademark::new()
    .with_mark_text("FAST")       // Descriptive for computers
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Computer Co")
    .add_nice_class(9)?
    .with_descriptive(true);      // Marks it as descriptive

match descriptive.validate() {
    Err(IpError::LackOfDistinctiveness { reason }) => {
        println!("❌ Not registrable: {}", reason);
    }
    _ => {}
}
```

#### ✅ Descriptive Marks with Secondary Meaning

If a descriptive mark has acquired distinctiveness through use:

```rust
let windows = EuTrademark::new()
    .with_mark_text("WINDOWS")
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Microsoft")
    .add_nice_class(9)?
    .with_descriptive(true)
    .with_secondary_meaning(true);  // Acquired distinctiveness

match windows.validate() {
    Ok(validation) => {
        println!("✅ Registrable due to secondary meaning");
    }
    _ => {}
}
```

#### ❌ Generic Marks (Never Registrable)

```rust
let generic = EuTrademark::new()
    .with_mark_text("COMPUTER")
    .with_mark_type(MarkType::WordMark)
    .with_applicant("Company")
    .add_nice_class(9)?
    .with_generic(true);

// Will always fail validation - generic terms cannot be registered
```

## Community Designs

Protects the visual appearance of products (not their function).

### Registered Community Design (RCD)

25 years maximum protection (5 renewal periods of 5 years each):

```rust
use legalis_eu::intellectual_property::*;

let design = CommunityDesign::new()
    .with_design_type(DesignType::Registered)
    .with_appearance(DesignAppearance {
        features: vec![
            "Curved edges with 2mm radius".into(),
            "Matte black finish".into(),
            "Centered circular button".into(),
        ],
        product_indication: "Smartphone case".into(),
    })
    .with_creator("Jane Designer")
    .with_owner("Design Company Ltd")
    .with_novelty(true)
    .with_individual_character(true);

match design.validate() {
    Ok(validation) => {
        println!("✅ Design protectable as RCD");
        println!("   Max protection: {} years", validation.max_protection_years);
    }
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

### Unregistered Community Design (UCD)

3 years automatic protection from first disclosure:

```rust
let ucd = CommunityDesign::new()
    .with_design_type(DesignType::Unregistered)
    .with_appearance(DesignAppearance {
        features: vec!["Unique textile pattern".into()],
        product_indication: "Fashion garment".into(),
    })
    .with_novelty(true)
    .with_individual_character(true);

match ucd.validate() {
    Ok(validation) => {
        println!("✅ UCD protection: {} years", validation.max_protection_years);
        // Will print: 3 years
    }
    _ => {}
}
```

### Requirements

**Article 5 - Novelty**: No identical design available to the public before filing

**Article 6 - Individual Character**: Overall impression must differ from prior designs

```rust
// Lacking novelty
let not_novel = CommunityDesign::new()
    .with_design_type(DesignType::Registered)
    .with_appearance(DesignAppearance {
        features: vec!["Standard rectangular shape".into()],
        product_indication: "Table".into(),
    })
    .with_novelty(false)  // Identical design already public
    .with_individual_character(true);

match not_novel.validate() {
    Err(IpError::InvalidDesign { reason }) => {
        println!("❌ Not protectable: {}", reason);
        // "Design must be novel (Art. 5)"
    }
    _ => {}
}
```

## Copyright

Automatic protection for original creative works (no registration required).

### Work Types

```rust
pub enum WorkType {
    Literary,        // Books, articles, software
    Musical,         // Songs, compositions
    Artistic,        // Paintings, sculptures
    Photographic,    // Photographs
    Audiovisual,     // Films, videos
    Software,        // Protected as literary works
    Database,        // Sui generis protection
    Architectural,   // Building designs
    Choreographic,   // Dance routines
    AppliedArt,      // Artistic applied to useful objects
}
```

### Basic Validation

```rust
use legalis_eu::intellectual_property::*;

let work = CopyrightWork::new()
    .with_title("My Software Application")
    .with_author("Developer Name")
    .with_work_type(WorkType::Software)
    .with_creation_date(chrono::Utc::now())
    .with_originality(true)
    .with_fixation(true);

match work.validate() {
    Ok(validation) => {
        if validation.is_protectable {
            println!("✅ Work is protectable by copyright");
            println!("   Duration: Life + 70 years");
            println!("   Currently protected: {}", validation.is_protected);
        }
    }
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

### Originality Requirement

Works must be "author's own intellectual creation" (InfoSoc Directive):

```rust
// ✅ Original work
let original = CopyrightWork::new()
    .with_title("Novel")
    .with_author("Author")
    .with_work_type(WorkType::Literary)
    .with_originality(true);  // Author's creative choices

// ❌ Not original (e.g., phone directory)
let not_original = CopyrightWork::new()
    .with_title("Phone Directory")
    .with_author("Company")
    .with_work_type(WorkType::Database)
    .with_originality(false);  // Mere alphabetical listing

match not_original.validate() {
    Err(IpError::CopyrightIssue { reason }) => {
        println!("❌ {}", reason);
    }
    _ => {}
}
```

### Protection Duration

**Life + 70 years** (Term Directive 2006/116/EC):

```rust
use chrono::Utc;

let death_date = Utc::now() - chrono::Duration::days(60 * 365);

let work = CopyrightWork::new()
    .with_title("Classic Novel")
    .with_author("Historic Author")
    .with_work_type(WorkType::Literary)
    .with_death_date_of_author(death_date)
    .with_originality(true);

match work.validate() {
    Ok(validation) => {
        if validation.is_protected {
            println!("✅ Still protected (60 years < 70 years)");
        } else {
            println!("📖 In public domain");
        }
    }
    _ => {}
}
```

### Copyright Exceptions

The law provides exceptions for certain uses:

```rust
pub enum CopyrightException {
    PrivateCopying,             // Article 5(2)(b)
    Quotation,                  // Article 5(3)(d)
    Parody,                     // Article 5(3)(k)
    EducationalUse,             // Article 5(3)(a)
    NewsReporting,              // Article 5(3)(c)
    TextDataMining,             // DSM Directive Art. 3-4
    AccessibilityForDisabled,   // DSM Directive Art. 6
}
```

The crate automatically identifies applicable exceptions:

```rust
let software = CopyrightWork::new()
    .with_title("Research Tool")
    .with_author("Developer")
    .with_work_type(WorkType::Software)
    .with_originality(true);

match software.validate() {
    Ok(validation) => {
        println!("Applicable exceptions:");
        for exception in &validation.applicable_exceptions {
            match exception {
                CopyrightException::TextDataMining => {
                    println!("  - Text/data mining for research");
                }
                CopyrightException::Quotation => {
                    println!("  - Quotation for criticism/review");
                }
                _ => {}
            }
        }
    }
    _ => {}
}
```

### Software Copyright

Software is protected as literary works (Software Directive 2009/24/EC):

```rust
let software = CopyrightWork::new()
    .with_title("DataViz Pro v2.0")
    .with_author("Software Company")
    .with_work_type(WorkType::Software)
    .with_originality(true)
    .with_fixation(true)  // Required for software
    .with_country_of_origin("Germany");

// Protects: Source code, object code, preparatory design material
// Does NOT protect: Ideas, algorithms, interfaces (need patents/trade secrets)
```

## Trade Secrets

Protects confidential business information.

### Three-Part Test (Article 2(1))

Information qualifies as a trade secret if ALL three conditions are met:

```rust
use legalis_eu::intellectual_property::*;

let secret = TradeSecret::new()
    .with_description("Proprietary ML algorithm for fraud detection")
    .with_holder("FinTech Company Ltd")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,                // (a) Not generally known
        has_commercial_value: true,     // (b) Has value because secret
        reasonable_steps_taken: true,   // (c) Protected by measures
    })
    .add_protective_measure("NDA with all employees")
    .add_protective_measure("Access control to source code")
    .add_protective_measure("Encryption at rest");

match secret.validate() {
    Ok(validation) => {
        if validation.three_part_test_passed {
            println!("✅ Protected as trade secret");
            println!("   Duration: Indefinite (while secret maintained)");
        }
    }
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

### Protective Measures

The law requires "reasonable steps" to keep information secret:

**Technical Measures:**
- Access controls (passwords, biometrics)
- Encryption
- Network segmentation
- Audit logs

**Organizational Measures:**
- Non-disclosure agreements (NDAs)
- Confidentiality clauses in employment contracts
- Need-to-know access policies
- Physical access restrictions
- Exit interviews
- Marking documents as "Confidential"

```rust
let well_protected = TradeSecret::new()
    .with_description("Secret formula")
    .with_holder("Company")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,
        has_commercial_value: true,
        reasonable_steps_taken: true,
    })
    .add_protective_measure("NDA with employees")
    .add_protective_measure("NDA with contractors")
    .add_protective_measure("Access control system")
    .add_protective_measure("Encryption")
    .add_protective_measure("Exit interviews");

// 5+ measures = adequate protection
```

### Misappropriation Analysis

The directive protects against unlawful acquisition:

```rust
pub enum AcquisitionMethod {
    UnauthorizedAccess,          // ❌ Unlawful
    Breach,                      // ❌ Unlawful (NDA breach)
    InducingBreach,              // ❌ Unlawful
    IndependentDiscovery,        // ✅ Lawful
    ReverseEngineering,          // ✅ Lawful
    ObservationOfPublicProduct,  // ✅ Lawful
}
```

#### Unlawful Acquisition

```rust
let secret = TradeSecret::new()
    .with_description("Manufacturing process")
    .with_holder("Manufacturer")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,
        has_commercial_value: true,
        reasonable_steps_taken: true,
    });

let analysis = secret.analyze_misappropriation(
    AcquisitionMethod::UnauthorizedAccess
);

if analysis.is_unlawful {
    println!("❌ Unlawful acquisition");
    println!("Available remedies:");
    for remedy in &analysis.remedies_available {
        println!("  - {}", remedy);
    }
    // Output:
    // - Injunction (Art. 12)
    // - Damages (Art. 13)
    // - Product recall (Art. 12(1)(a))
}
```

#### Lawful Acquisition

```rust
let analysis = secret.analyze_misappropriation(
    AcquisitionMethod::ReverseEngineering
);

if !analysis.is_unlawful {
    println!("✅ Lawful acquisition");
    println!("Reverse engineering is permitted under EU law");
    // Trade secret holder cannot prevent this
}
```

## Layered IP Strategy

Smart companies use multiple IP rights for comprehensive protection.

### Example: Software Product

```rust
// 1. Trademark for brand name
let trademark = EuTrademark::new()
    .with_mark_text("DATAVIZ PRO")
    .with_mark_type(MarkType::WordMark)
    .add_nice_class(9)?
    .add_nice_class(42)?;

// 2. Copyright for source code
let copyright = CopyrightWork::new()
    .with_title("DataViz Pro v2.0")
    .with_work_type(WorkType::Software)
    .with_originality(true)
    .with_fixation(true);

// 3. Design for UI appearance
let design = CommunityDesign::new()
    .with_design_type(DesignType::Registered)
    .with_appearance(DesignAppearance {
        features: vec!["Unique dashboard layout".into()],
        product_indication: "Software user interface".into(),
    })
    .with_novelty(true)
    .with_individual_character(true);

// 4. Trade secret for algorithms
let secret = TradeSecret::new()
    .with_description("Proprietary ML algorithm")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: true,
        has_commercial_value: true,
        reasonable_steps_taken: true,
    })
    .add_protective_measure("Code obfuscation")
    .add_protective_measure("NDA with developers");
```

### What Each Protects

| IP Type | Protects | Against | Duration |
|---------|----------|---------|----------|
| **Trademark** | Brand "DATAVIZ PRO" | Confusingly similar marks | Indefinite |
| **Copyright** | Source code expression | Copying code | Life + 70 years |
| **Design** | UI visual appearance | Look-alike interfaces | 25 years |
| **Trade Secret** | Algorithm logic | Disclosure | While secret |

### When to Use What

**Trademark**: Always, for any brand you want to protect

**Copyright**: Automatic, but document creation dates

**Design**: If visual appearance is important and novel

**Trade Secret**:
- ✅ For competitive advantages you can keep secret
- ❌ Not for anything reverse-engineerable
- ❌ Not for anything you'll patent (patents require disclosure)

## Best Practices

### 1. Document Everything

```rust
// Good: Clear documentation
let work = CopyrightWork::new()
    .with_title("Software v1.0")
    .with_author("Company Ltd")
    .with_creation_date(Utc::now())  // Document creation date!
    .with_country_of_origin("Germany");
```

### 2. File Early

- **Trademarks**: File before product launch
- **Designs**: File within 12 months of first disclosure
- **Copyright**: Automatic, but keep creation records
- **Trade Secrets**: Implement protection measures immediately

### 3. Monitor and Enforce

```rust
// Regularly check trademark status
let status = trademark.status;
match status {
    Some(TrademarkStatus::Registered { renewal_due, .. }) => {
        if renewal_due < Utc::now() + chrono::Duration::days(90) {
            println!("⚠️ Renewal due soon!");
        }
    }
    _ => {}
}
```

### 4. Layered Protection

Don't rely on one IP type - combine them:
- Brand: Trademark
- Code: Copyright + Trade Secrets
- UI: Design + Copyright
- Algorithms: Trade Secrets (or patents)

## Common Mistakes

### ❌ Using Generic Terms as Trademarks

```rust
// Won't work - "COMPUTER" is generic for Class 9
let bad = EuTrademark::new()
    .with_mark_text("COMPUTER")
    .add_nice_class(9)?;  // Will fail validation
```

### ❌ Publishing Trade Secrets

```rust
// If you publish in a patent, it's no longer secret!
let published = TradeSecret::new()
    .with_description("Algorithm from patent EP12345")
    .with_characteristics(TradeSecretCharacteristics {
        is_secret: false,  // ❌ Not secret anymore
        has_commercial_value: true,
        reasonable_steps_taken: true,
    });
```

### ❌ Assuming Copyright Protects Ideas

```rust
// Copyright only protects expression, not ideas
// Two people can independently create similar software
```

## Further Reading

- **EUIPO**: [European Union Intellectual Property Office](https://euipo.europa.eu/)
- **EUR-Lex**: Official EU legal database
- **Examples**: See `examples/ip_*.rs` in this crate
- **API Docs**: Run `cargo doc --open`

## Need Help?

- Check the [FAQ](FAQ.md)
- Review the [examples](../examples/)
- Open an [issue](https://github.com/cool-japan/legalis/issues)
