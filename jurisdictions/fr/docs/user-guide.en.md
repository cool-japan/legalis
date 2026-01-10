# Legalis-FR User Guide

Comprehensive guide with practical examples for all 11 legal domains in legalis-fr.

## Table of Contents

1. [Contract Law](#1-contract-law)
2. [Labor Law](#2-labor-law)
3. [Family Law](#3-family-law)
4. [Inheritance Law](#4-inheritance-law)
5. [Property Law](#5-property-law)
6. [Intellectual Property Law](#6-intellectual-property-law)
7. [Evidence Law](#7-evidence-law)
8. [Company Law](#8-company-law)
9. [Constitutional Law](#9-constitutional-law)
10. [Administrative Law](#10-administrative-law)
11. [Tort Law](#11-tort-law)
12. [Legal Reasoning Engine](#12-legal-reasoning-engine)

---

## 1. Contract Law

**Code civil, Book III - Articles 1101-1231**

### 1.1 Creating and Validating Contracts

```rust
use legalis_fr::contract::{Contract, ContractType, validate_contract};
use chrono::NaiveDate;

// Example: Real estate sale contract
let sale_contract = Contract::builder()
    .contract_type(ContractType::Sale)
    .parties(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .object("Apartment at 45 Rue de Rivoli, Paris 75001")
    .price(450_000)  // €450,000
    .formation_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .build()?;

// Validate under Article 1128 (essential elements: consent, capacity, content)
match validate_contract(&sale_contract) {
    Ok(_) => println!("✅ Contract is valid"),
    Err(e) => println!("❌ Invalid contract: {}", e),
}
```

### 1.2 Contract Performance (Article 1217)

```rust
use legalis_fr::contract::{assess_contract_performance, PerformanceStatus};

let performance = assess_contract_performance(
    &sale_contract,
    PerformanceStatus::Performed,
);

match performance {
    Ok(result) => println!("Contract performed: {:?}", result),
    Err(e) => println!("Performance issue: {}", e),
}
```

### 1.3 Contract Termination

```rust
use legalis_fr::contract::{TerminationReason, validate_termination};

let termination = validate_termination(
    &sale_contract,
    TerminationReason::MutualConsent,
    NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
)?;

println!("Termination valid: {}", termination.is_valid);
```

### 1.4 Real-World Use Case: E-Commerce Platform

```rust
use legalis_fr::contract::{Contract, ContractType, validate_contract};

fn validate_online_purchase(
    buyer_id: &str,
    seller_id: &str,
    product: &str,
    price: u64,
) -> Result<Contract, Box<dyn std::error::Error>> {
    let contract = Contract::builder()
        .contract_type(ContractType::Sale)
        .parties(vec![buyer_id.to_string(), seller_id.to_string()])
        .object(product)
        .price(price)
        .formation_date(chrono::Utc::now().naive_utc().date())
        .build()?;

    validate_contract(&contract)?;

    Ok(contract)
}

// Usage
let purchase = validate_online_purchase(
    "buyer@example.com",
    "seller@example.com",
    "MacBook Pro 14-inch",
    2_299_00,  // €2,299.00 in cents
)?;
```

---

## 2. Labor Law

**Code du travail - Articles L1221-1 to L5422-3**

### 2.1 Employment Contracts (CDI/CDD)

```rust
use legalis_fr::labor::{Employment, EmploymentType, validate_employment};
use chrono::NaiveDate;

// Example 1: Indefinite contract (CDI)
let cdi = Employment::builder()
    .employee_name("Marie Dupont")
    .employer_name("TechCorp SARL")
    .employment_type(EmploymentType::Indefinite)
    .start_date(NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
    .position("Software Engineer")
    .monthly_salary(3_500)
    .weekly_hours(35.0)  // Legal 35-hour workweek
    .probation_period_months(Some(3))
    .build()?;

validate_employment(&cdi)?;

// Example 2: Fixed-term contract (CDD)
let cdd = Employment::builder()
    .employee_name("Pierre Martin")
    .employer_name("SeasonalCo SAS")
    .employment_type(EmploymentType::FixedTerm {
        reason: "Seasonal increase in activity".to_string(),
    })
    .start_date(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
    .end_date(Some(NaiveDate::from_ymd_opt(2024, 9, 30).unwrap()))
    .position("Sales Associate")
    .monthly_salary(2_000)
    .weekly_hours(35.0)
    .build()?;

validate_employment(&cdd)?;
```

### 2.2 Minimum Wage Validation (Article L3231-2)

```rust
use legalis_fr::labor::{validate_minimum_wage, SMIC_2024};

let salary = 1_800;  // €1,800/month
let hours = 35.0;

match validate_minimum_wage(salary, hours) {
    Ok(_) => println!("✅ Salary complies with SMIC (€{})", SMIC_2024),
    Err(e) => println!("❌ Below minimum wage: {}", e),
}
```

### 2.3 Working Hours Limits (Article L3121-27)

```rust
use legalis_fr::labor::validate_working_hours;

let weekly_hours = 40.0;

match validate_working_hours(weekly_hours) {
    Ok(_) => println!("✅ Working hours valid"),
    Err(e) => {
        println!("❌ Exceeds legal limit: {}", e);
        println!("French: {}", e.message_fr());
    }
}
```

### 2.4 Termination (Article L1234-1)

```rust
use legalis_fr::labor::{TerminationReason, validate_termination};

let termination = validate_termination(
    &cdi,
    TerminationReason::EconomicDismissal,
    NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
)?;

println!("Notice period: {} months", termination.notice_period_months);
println!("Severance pay: €{}", termination.severance_pay);
```

### 2.5 Real-World Use Case: HR Management System

```rust
use legalis_fr::labor::{Employment, validate_employment, validate_termination};

fn onboard_employee(
    name: &str,
    position: &str,
    salary: u64,
) -> Result<Employment, Box<dyn std::error::Error>> {
    let employment = Employment::builder()
        .employee_name(name)
        .employer_name("MyCompany SAS")
        .start_date(chrono::Utc::now().naive_utc().date())
        .position(position)
        .monthly_salary(salary)
        .weekly_hours(35.0)
        .build()?;

    // Validate under French labor law
    validate_employment(&employment)?;

    // Store in database...
    Ok(employment)
}
```

---

## 3. Family Law

**Code civil, Book I - Articles 143-515-13**

### 3.1 Marriage (Articles 143-227)

```rust
use legalis_fr::family::{Marriage, MarriageRegime, validate_marriage};
use chrono::NaiveDate;

let marriage = Marriage::builder()
    .spouse1("Jean Martin", 28)
    .spouse2("Sophie Dubois", 26)
    .marriage_date(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap())
    .regime(MarriageRegime::CommunityOfProperty)
    .build()?;

validate_marriage(&marriage)?;
```

### 3.2 Divorce (Articles 229-232)

```rust
use legalis_fr::family::{Divorce, DivorceType, validate_divorce};

let divorce = Divorce::builder()
    .marriage(marriage.clone())
    .divorce_type(DivorceType::MutualConsent)
    .filing_date(NaiveDate::from_ymd_opt(2025, 3, 10).unwrap())
    .build()?;

validate_divorce(&divorce)?;
```

### 3.3 Parental Authority (Article 371-1)

```rust
use legalis_fr::family::{ParentalAuthority, assess_parental_authority};

let authority = ParentalAuthority::builder()
    .child_name("Emma Martin")
    .child_birthdate(NaiveDate::from_ymd_opt(2015, 4, 20).unwrap())
    .parents(vec!["Jean Martin".to_string(), "Sophie Dubois".to_string()])
    .joint_authority(true)
    .build()?;

assess_parental_authority(&authority)?;
```

### 3.4 PACS (Articles 515-1 to 515-7)

```rust
use legalis_fr::family::{PACS, validate_pacs};

let pacs = PACS::builder()
    .partner1("Alice Moreau", 30)
    .partner2("Bob Lefebvre", 32)
    .registration_date(NaiveDate::from_ymd_opt(2023, 9, 1).unwrap())
    .build()?;

validate_pacs(&pacs)?;
```

---

## 4. Inheritance Law

**Code civil, Book III, Title I - Articles 720-892**

### 4.1 Succession (Article 720)

```rust
use legalis_fr::inheritance::{Succession, Heir, Relationship, calculate_succession};
use chrono::NaiveDate;

let succession = Succession::builder()
    .deceased("Jean Martin")
    .death_date(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
    .heirs(vec![
        Heir::new("Marie Martin", Relationship::Child, None),
        Heir::new("Pierre Martin", Relationship::Child, None),
        Heir::new("Sophie Martin", Relationship::Spouse, None),
    ])
    .estate_value(500_000)  // €500,000
    .build()?;

let result = calculate_succession(&succession)?;
println!("Distribution: {:?}", result.distribution);
```

### 4.2 Reserved Portion (Articles 912-913)

```rust
use legalis_fr::inheritance::{calculate_reserved_portions, ReservedPortion};

// Two children: reserved portion = 2/3, available = 1/3
let portions = calculate_reserved_portions(2)?;

assert!((portions.reserved_portion - 2.0/3.0).abs() < 1e-10);
assert!((portions.available_portion - 1.0/3.0).abs() < 1e-10);

println!("With 2 children:");
println!("  Reserved portion: {:.2}%", portions.reserved_portion * 100.0);
println!("  Available portion: {:.2}%", portions.available_portion * 100.0);
```

### 4.3 Wills (Articles 774-792)

```rust
use legalis_fr::inheritance::{Will, WillType, validate_will};

let will = Will::builder()
    .testator("Jean Martin")
    .will_type(WillType::Holographic {
        handwritten: true,
        dated: true,
        signed: true,
    })
    .date(NaiveDate::from_ymd_opt(2023, 1, 10).unwrap())
    .dispositions(vec![
        "Leave apartment to Marie Martin".to_string(),
        "Leave car to Pierre Martin".to_string(),
    ])
    .build()?;

validate_will(&will)?;
```

### 4.4 Real-World Use Case: Estate Planning Tool

```rust
use legalis_fr::inheritance::{Succession, calculate_succession};

fn calculate_estate_distribution(
    deceased: &str,
    heirs: Vec<(&str, Relationship)>,
    estate_value: u64,
) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    let succession = Succession::builder()
        .deceased(deceased)
        .death_date(chrono::Utc::now().naive_utc().date())
        .heirs(heirs.into_iter()
            .map(|(name, rel)| Heir::new(name, rel, None))
            .collect())
        .estate_value(estate_value)
        .build()?;

    let result = calculate_succession(&succession)?;

    Ok(result.distribution.into_iter()
        .map(|(name, amount)| (name, amount))
        .collect())
}
```

---

## 5. Property Law

**Code civil, Book II-III - Articles 490-734**

### 5.1 Ownership Rights (Article 544)

```rust
use legalis_fr::property::{Property, PropertyType, validate_ownership};

let property = Property::builder()
    .property_type(PropertyType::Immovable {
        land_area: 500.0,  // 500 m²
        building_area: Some(150.0),  // 150 m²
    })
    .owner("Marie Dupont")
    .location("12 Rue de la Paix, Paris 75002")
    .value(750_000)  // €750,000
    .build()?;

validate_ownership(&property)?;
```

### 5.2 Easements (Articles 637-710)

```rust
use legalis_fr::property::{Easement, EasementType, validate_easement};

// Right of way (passage) easement
let easement = Easement::builder()
    .easement_type(EasementType::RightOfWay)
    .dominant_estate(Some("Parcel A"))
    .servient_estate("Parcel B")
    .description("3-meter wide path for vehicle access")
    .build()?;

validate_easement(&easement)?;
```

### 5.3 Water Rights (Article 555)

```rust
use legalis_fr::property::{EasementType, validate_water_rights};

let water_easement = Easement::builder()
    .easement_type(EasementType::WaterRights)
    .dominant_estate(Some("Farm property"))
    .servient_estate("Property with stream")
    .description("Cattle watering rights")
    .build()?;

validate_water_rights(&water_easement)?;
```

### 5.4 Real-World Use Case: Real Estate Platform

```rust
use legalis_fr::property::{Property, PropertyType, Easement};

fn validate_property_transaction(
    property: &Property,
    easements: Vec<Easement>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate ownership
    validate_ownership(property)?;

    // Validate all easements
    for easement in easements {
        validate_easement(&easement)?;
    }

    Ok(true)
}
```

---

## 6. Intellectual Property Law

**Code de la propriété intellectuelle**

### 6.1 Patents (Articles L611-10, L611-11)

```rust
use legalis_fr::intellectual_property::{Patent, validate_patent};
use chrono::NaiveDate;

let patent = Patent::builder()
    .title("Novel Solar Panel Design")
    .inventor("Dr. Marie Curie")
    .filing_date(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap())
    .novelty(true)
    .inventive_step(true)
    .industrial_applicability(true)
    .build()?;

validate_patent(&patent)?;

// Protection period: 20 years from filing
println!("Protection until: {}", patent.expiry_date());
```

### 6.2 Copyright (Articles L122-1, L123-1)

```rust
use legalis_fr::intellectual_property::{Copyright, WorkType, validate_copyright};

let copyright = Copyright::builder()
    .work_title("Le Petit Prince")
    .author("Antoine de Saint-Exupéry")
    .creation_date(NaiveDate::from_ymd_opt(1943, 4, 6).unwrap())
    .work_type(WorkType::Literary)
    .build()?;

validate_copyright(&copyright)?;

// Protection: 70 years post-mortem
println!("Protected until: {}", copyright.expiry_date());
```

### 6.3 Trademarks (Articles L711-1, L712-1)

```rust
use legalis_fr::intellectual_property::{Trademark, validate_trademark};

let trademark = Trademark::builder()
    .mark("LEGALIS™")
    .owner("Legalis SAS")
    .registration_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
    .distinctiveness(true)
    .build()?;

validate_trademark(&trademark)?;

// Protection: 10 years, renewable indefinitely
println!("Renewal due: {}", trademark.next_renewal_date());
```

### 6.4 Real-World Use Case: IP Management System

```rust
use legalis_fr::intellectual_property::{Patent, Copyright, Trademark};

fn check_ip_portfolio() -> Result<(), Box<dyn std::error::Error>> {
    let patents = load_patents()?;
    let copyrights = load_copyrights()?;
    let trademarks = load_trademarks()?;

    // Check expiring patents
    let today = chrono::Utc::now().naive_utc().date();
    for patent in patents {
        if patent.is_expiring_within_days(today, 365) {
            println!("⚠️ Patent '{}' expires in less than 1 year", patent.title);
        }
    }

    Ok(())
}
```

---

## 7. Evidence Law

**Code civil, Book III, Title XX - Articles 1353-1378**

### 7.1 Burden of Proof (Article 1353)

```rust
use legalis_fr::evidence::{assess_burden_of_proof, BurdenOfProof};

let burden = BurdenOfProof::builder()
    .claimant_must_prove(vec![
        "Contract was signed".to_string(),
        "Payment was made".to_string(),
    ])
    .defendant_must_prove(vec![
        "Goods were delivered".to_string(),
    ])
    .build()?;

let assessment = assess_burden_of_proof(&burden)?;
println!("Burden of proof: {:?}", assessment);
```

### 7.2 Electronic Evidence (Articles 1366-1378)

```rust
use legalis_fr::evidence::{Evidence, EvidenceType, validate_electronic_evidence};

let evidence = Evidence::builder()
    .evidence_type(EvidenceType::WrittenDocument {
        electronic: true,
        signed: true,  // Electronic signature
    })
    .description("Electronically signed contract")
    .date_obtained(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
    .authenticity_verified(true)
    .build()?;

validate_electronic_evidence(&evidence)?;
```

### 7.3 Presumptions (Article 1354)

```rust
use legalis_fr::evidence::{PresumptionType, assess_presumption};

// Simple presumption (rebuttable)
let simple = assess_presumption(PresumptionType::Simple)?;
println!("Rebuttable: {}", simple.rebuttable);

// Irrebuttable presumption
let irrebuttable = assess_presumption(PresumptionType::Irrebuttable)?;
println!("Rebuttable: {}", irrebuttable.rebuttable);  // false
```

---

## 8. Company Law

**Code de commerce - Articles L210-1 to L247-1**

### 8.1 Company Formation (Article L210-2)

```rust
use legalis_fr::company::{Company, CompanyType, validate_company_formation};

let sarl = Company::builder()
    .name("TechCorp SARL")
    .company_type(CompanyType::SARL)
    .capital(10_000)  // Minimum €1 for SARL
    .shareholders(vec![
        "Jean Martin (60%)".to_string(),
        "Sophie Dubois (40%)".to_string(),
    ])
    .registered_office("45 Rue de Rivoli, Paris 75001")
    .build()?;

validate_company_formation(&sarl)?;
```

### 8.2 SAS Formation

```rust
let sas = Company::builder()
    .name("InnovateSAS")
    .company_type(CompanyType::SAS)
    .capital(50_000)
    .shareholders(vec![
        "Founder 1 (70%)".to_string(),
        "Founder 2 (30%)".to_string(),
    ])
    .registered_office("10 Avenue des Champs-Élysées, Paris 75008")
    .build()?;

validate_company_formation(&sas)?;
```

---

## 9. Constitutional Law

**Constitution of the Fifth Republic (1958)**

### 9.1 Constitutionality Assessment (Article 61)

```rust
use legalis_fr::constitution::{assess_constitutionality, ConstitutionalIssue};

let issue = ConstitutionalIssue::builder()
    .law_text("New data retention law")
    .challenged_provisions(vec![
        "Mandatory 5-year data retention".to_string(),
        "No judicial oversight".to_string(),
    ])
    .constitutional_rights_at_stake(vec![
        "Right to privacy (Article 2)".to_string(),
        "Freedom of communication (Article 11)".to_string(),
    ])
    .build()?;

let assessment = assess_constitutionality(&issue)?;
println!("Constitutional: {}", assessment.is_constitutional);
```

---

## 10. Administrative Law

**Code de justice administrative**

### 10.1 Administrative Acts

```rust
use legalis_fr::administrative::{AdministrativeAct, validate_act};

let act = AdministrativeAct::builder()
    .authority("Mayor of Paris")
    .subject("Building permit for new construction")
    .date(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap())
    .legal_basis("Article L421-1 of Urban Planning Code")
    .build()?;

validate_act(&act)?;
```

---

## 11. Tort Law

**Code civil, Articles 1240-1244**

### 11.1 Tort Liability (Article 1240)

```rust
use legalis_fr::code_civil::{assess_tort_liability, TortClaim};

let claim = TortClaim::builder()
    .wrongful_act("Negligent driving causing collision")
    .damage("Vehicle damage: €5,000; Medical expenses: €2,000")
    .causation("Direct and certain causal link")
    .build()?;

let liability = assess_tort_liability(&claim)?;
println!("Liable: {}", liability.is_liable);
println!("Damages: €{}", liability.damages_awarded);
```

---

## 12. Legal Reasoning Engine

Advanced legal analysis and case evaluation.

### 12.1 Case Analysis

```rust
use legalis_fr::reasoning::{LegalCase, apply_legal_reasoning, Domain};

let case = LegalCase::builder()
    .facts(vec![
        "Contract signed on January 15, 2023".to_string(),
        "Seller failed to deliver goods by agreed date".to_string(),
        "Buyer suffered financial loss of €10,000".to_string(),
    ])
    .legal_question("Can buyer claim damages for non-performance?")
    .relevant_domain(Domain::ContractLaw)
    .build()?;

let result = apply_legal_reasoning(case)?;

println!("Applicable articles: {:?}", result.applicable_articles);
println!("Conclusion: {}", result.conclusion);
println!("Confidence: {:.0}%", result.confidence * 100.0);
```

### 12.2 Multi-Domain Analysis

```rust
let complex_case = LegalCase::builder()
    .facts(vec![
        "Employee dismissed during pregnancy".to_string(),
        "No economic justification provided".to_string(),
    ])
    .legal_question("Is the dismissal valid?")
    .relevant_domain(Domain::LaborLaw)
    .constitutional_rights_implicated(vec![
        "Right to equality (Constitution)".to_string(),
        "Protection of family (Article 8 ECHR)".to_string(),
    ])
    .build()?;

let result = apply_legal_reasoning(complex_case)?;
```

---

## Best Practices

### 1. Error Handling

Always handle errors gracefully:

```rust
match validate_employment(&employment) {
    Ok(_) => {
        // Success path
    }
    Err(e) => {
        // Log error
        eprintln!("Validation failed: {}", e);
        // Show user-friendly message
        println!("French: {}", e.message_fr());
        // Return error to caller
        return Err(e.into());
    }
}
```

### 2. Date Handling

Use `chrono` for all date operations:

```rust
use chrono::NaiveDate;

// Correct
let date = NaiveDate::from_ymd_opt(2023, 6, 15)
    .ok_or("Invalid date")?;

// Date arithmetic
let future = date + chrono::Duration::days(30);
```

### 3. Builder Pattern

Always use builders for type construction:

```rust
// Good
let employment = Employment::builder()
    .employee_name("Marie")
    .build()?;

// Avoid direct construction
let employment = Employment {
    employee_name: "Marie".to_string(),
    // ... missing required fields
};
```

### 4. Validation

Validate data as early as possible:

```rust
// Validate immediately after creation
let contract = Contract::builder().build()?;
validate_contract(&contract)?;

// Or use validation in business logic
fn process_contract(contract: Contract) -> Result<(), ContractLawError> {
    validate_contract(&contract)?;
    // ... rest of logic
    Ok(())
}
```

---

## Next Steps

- **[API Patterns](./api-patterns.md)** - Learn advanced patterns and best practices
- **[Legal Domains](./legal-domains.md)** - Deep dive into each legal domain
- **[API Documentation](https://docs.rs/legalis-fr)** - Full API reference

---

**Need help?** Check the [troubleshooting section](./getting-started.md#troubleshooting) or open an issue on GitHub.
