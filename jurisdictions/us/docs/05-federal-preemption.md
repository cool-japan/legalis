# Federal Preemption Guide

**Understanding when federal law overrides state law**

## Overview

The **Supremacy Clause** (U.S. Constitution, Article VI, Clause 2) establishes that federal law is the "supreme Law of the Land" and preempts conflicting state law.

However, preemption analysis is complex because:
- Federal government has **limited enumerated powers**
- States have broad **police powers** (health, safety, welfare)
- Courts **presume against preemption** in traditional state law areas

Legalis-US provides structured preemption analysis for determining when federal law displaces state regulation.

---

## The Three Types of Preemption

### 1. Express Preemption

**Definition**: Congress explicitly states in statute that federal law preempts state law.

**Example - ERISA (Employee Retirement Income Security Act):**

```rust
use legalis_us::federal::preemption::{PreemptionAnalysis, PreemptionType};

// ERISA §514(a): "shall supersede any and all State laws"
let erisa_analysis = PreemptionAnalysis::new(
    "ERISA regulation of employee benefit plans",
    "State insurance regulation of ERISA plans"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "§514(a): shall supersede any and all State laws insofar as they relate to any employee benefit plan".to_string()
})
.with_federal_statute("Employee Retirement Income Security Act (ERISA)")
.with_presumption_against(false); // Express preemption overcomes presumption

let result = erisa_analysis.analyze();

assert!(result.is_preempted);
println!("Confidence: {:.0}%", result.confidence * 100.0); // High confidence
```

**Other Examples:**
- **Federal Arbitration Act**: "valid, irrevocable, and enforceable, save upon such grounds as exist at law"
- **National Bank Act**: Preempts state usury laws for national banks
- **Airline Deregulation Act**: Preempts state regulation of airline routes/services

### 2. Field Preemption (Implied)

**Definition**: Federal regulation is so comprehensive that Congress intended to "occupy the field," leaving no room for state regulation.

**Example - Immigration:**

```rust
// Immigration is exclusively federal domain
let immigration_analysis = PreemptionAnalysis::new(
    "Federal immigration enforcement",
    "State criminal law targeting unauthorized immigrants"
)
.with_preemption_type(PreemptionType::ImpliedField {
    federal_comprehensiveness: 0.95, // Immigration is almost entirely federal
    traditional_state_concern: false, // Immigration historically federal
})
.with_federal_statute("Immigration and Nationality Act")
.with_supreme_court_precedent("Arizona v. United States, 567 U.S. 387 (2012)")
.with_presumption_against(false); // Immigration not traditional state concern

let result = immigration_analysis.analyze();
assert!(result.is_preempted);
```

**Fields Occupied by Federal Law:**
- **Immigration**: Federal government has exclusive authority
- **Foreign affairs**: Exclusively federal domain
- **Bankruptcy**: Comprehensive federal bankruptcy code
- **Nuclear energy**: Atomic Energy Act occupies field
- **Copyright**: Federal law exclusive (Constitution grants Congress power)

**Partial Field Preemption:**
- **Aviation safety**: Federal (FAA), but state tort law not preempted
- **Railroad safety**: Federal safety standards, but state negligence law survives

### 3. Conflict Preemption (Implied)

**Definition**: State law conflicts with federal law, either by:
- Making **compliance with both impossible**, OR
- **Obstructing** federal objectives

#### A. Impossibility Preemption

```rust
use legalis_us::federal::preemption::ConflictPreemptionType;

// Federal law: "No airbags required"
// State law: "Airbags required"
// → Impossible to comply with both

let impossibility = PreemptionAnalysis::new(
    "Federal vehicle safety standards (no airbag mandate)",
    "State law requiring airbags"
)
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Impossibility
})
.with_federal_statute("National Traffic and Motor Vehicle Safety Act")
.with_presumption_against(false); // Clear impossibility

let result = impossibility.analyze();
assert!(result.is_preempted);
```

**Classic Example**: *Florida Lime & Avocado Growers v. Paul*, 373 U.S. 132 (1963)
- Federal standard: Avocados with 8%+ oil content are mature
- California law: Avocados must pass different maturity test
- Held: NOT impossible to comply with both (can meet stricter standard)

#### B. Obstacle Preemption

**Definition**: State law "stands as an obstacle" to accomplishing federal objectives.

```rust
// Federal law: Generic drugs must have same label as brand-name
// State tort law: Failure-to-warn liability for not updating label
// → State law would require impossibility (can't change label)

let obstacle = PreemptionAnalysis::new(
    "FDA generic drug labeling requirements (sameness requirement)",
    "State failure-to-warn tort law"
)
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Obstacle
})
.with_federal_statute("Federal Food, Drug, and Cosmetic Act")
.with_supreme_court_precedent("PLIVA, Inc. v. Mensing, 564 U.S. 604 (2011)")
.with_presumption_against(true); // Tort law is traditional state concern

let result = obstacle.analyze();
assert!(result.is_preempted);
println!("Reasoning: {}", result.reasoning);
```

**Controversial Example**: *Geier v. American Honda Motor Co.*, 529 U.S. 861 (2000)
- Federal regulation: Manufacturers have choice among passive restraint options
- State tort law: Failure to install airbags = negligence
- Held: State law preempted (would eliminate manufacturer choice)

---

## Presumption Against Preemption

### When Presumption Applies

Courts **presume against preemption** in fields traditionally regulated by states:

```rust
// Traditional state concerns (strong presumption against preemption):
let traditional_areas = vec![
    "Tort law",
    "Contract law",
    "Property law",
    "Family law",
    "Criminal law",
    "Health and safety regulation",
    "Professional licensing",
    "Land use and zoning",
    "Insurance regulation",
];

for area in traditional_areas {
    let analysis = PreemptionAnalysis::new(
        "Federal regulation",
        format!("State {} law", area).as_str()
    )
    .with_presumption_against(true); // Strong presumption

    // Requires clear Congressional intent to preempt
}
```

### When Presumption Does NOT Apply

```rust
// Federal domains (no presumption against preemption):
let federal_areas = vec![
    "Immigration",
    "Foreign affairs",
    "Interstate commerce regulation",
    "Bankruptcy",
    "Copyright and patent",
    "Currency",
];

for area in federal_areas {
    let analysis = PreemptionAnalysis::new(
        format!("Federal {} regulation", area).as_str(),
        "State regulation"
    )
    .with_presumption_against(false); // No presumption
}
```

---

## Practical Examples

### Example 1: Pharmaceutical Labeling

**Issue**: Can patient sue drug manufacturer for failure-to-warn under state tort law when FDA approved the drug label?

**Analysis depends on brand-name vs. generic:**

```rust
// Brand-name drugs: NOT preempted
let brand_name = PreemptionAnalysis::new(
    "FDA approval of brand-name drug label",
    "State failure-to-warn tort claim"
)
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Obstacle
})
.with_federal_statute("Federal Food, Drug, and Cosmetic Act")
.with_supreme_court_precedent("Wyeth v. Levine, 555 U.S. 555 (2009)")
.with_presumption_against(true) // Tort law = traditional state concern
.with_additional_context("Brand-name manufacturers can update labels via CBE process");

let result = brand_name.analyze();
assert!(!result.is_preempted); // NOT preempted - can sue

// Generic drugs: PREEMPTED
let generic = PreemptionAnalysis::new(
    "FDA generic drug labeling (sameness requirement)",
    "State failure-to-warn tort claim"
)
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Impossibility
})
.with_supreme_court_precedent("PLIVA, Inc. v. Mensing, 564 U.S. 604 (2011)")
.with_presumption_against(true)
.with_additional_context("Generic manufacturers CANNOT change labels (must match brand)");

let result = generic.analyze();
assert!(result.is_preempted); // PREEMPTED - cannot sue
```

**Key Difference**: Brand-name manufacturers can unilaterally update labels; generic manufacturers cannot.

### Example 2: Automobile Safety Standards

```rust
// Federal safety standards preempt state regulations but not tort law

// State safety regulation: PREEMPTED
let regulation = PreemptionAnalysis::new(
    "Federal Motor Vehicle Safety Standards (FMVSS)",
    "State regulation requiring different safety equipment"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "§1392(d): No State shall have any authority to establish safety standards".to_string()
})
.with_federal_statute("National Traffic and Motor Vehicle Safety Act");

assert!(regulation.analyze().is_preempted);

// State tort law: NOT preempted
let tort = PreemptionAnalysis::new(
    "Federal Motor Vehicle Safety Standards (FMVSS)",
    "State negligence/product liability tort claim"
)
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Obstacle
})
.with_supreme_court_precedent("Williamson v. Mazda, 562 U.S. 323 (2011)")
.with_presumption_against(true) // Tort law traditional state concern
.with_additional_context("Federal standards are minimum, not ceiling");

assert!(!tort.analyze().is_preempted); // NOT preempted
```

**Principle**: Federal safety standards set a floor, not a ceiling. State tort law can incentivize exceeding federal minimums.

### Example 3: Medical Devices

**FDA preemption varies by approval pathway:**

```rust
// Class III devices (PMA - Premarket Approval): PREEMPTED
let pma_device = PreemptionAnalysis::new(
    "FDA premarket approval of Class III medical device",
    "State tort claim for design defect"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "§360k(a): No State may establish any requirement different from federal".to_string()
})
.with_federal_statute("Medical Device Amendments of 1976")
.with_supreme_court_precedent("Riegel v. Medtronic, 552 U.S. 312 (2008)")
.with_presumption_against(true);

assert!(pma_device.analyze().is_preempted); // PREEMPTED

// 510(k) devices (substantial equivalence): NOT preempted
let five_ten_k = PreemptionAnalysis::new(
    "FDA 510(k) clearance of medical device",
    "State tort claim for design defect"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "§360k(a) applies only to requirements, not common-law duties".to_string()
})
.with_supreme_court_precedent("Medtronic v. Lohr, 518 U.S. 470 (1996)")
.with_presumption_against(true);

assert!(!five_ten_k.analyze().is_preempted); // NOT preempted
```

**Key Distinction**:
- **PMA (Class III)**: Rigorous FDA review → preempts state tort claims
- **510(k)**: Minimal FDA review (just "substantial equivalence") → does NOT preempt

### Example 4: Arbitration Agreements

```rust
// Federal Arbitration Act (FAA) preempts state anti-arbitration laws

let arbitration = PreemptionAnalysis::new(
    "Federal Arbitration Act (FAA)",
    "State law invalidating arbitration agreements in consumer contracts"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "Arbitration agreements shall be valid, irrevocable, and enforceable".to_string()
})
.with_federal_statute("Federal Arbitration Act, 9 U.S.C. §2")
.with_supreme_court_precedent("AT&T Mobility v. Concepcion, 563 U.S. 333 (2011)")
.with_presumption_against(true); // Contract law = traditional state concern

let result = arbitration.analyze();
assert!(result.is_preempted);
println!("Analysis: FAA has robust preemptive effect even in state law areas");
```

**Controversial Area**: Supreme Court has expanded FAA preemption significantly, overriding state contract law protections for consumers and employees.

---

## Commerce Clause Constraints

Even without preemption, the **dormant Commerce Clause** limits state regulation of interstate commerce.

```rust
use legalis_us::federal::commerce_clause::{CommerceClauseAnalysis, StateRegulationType};

// State regulation that discriminates against out-of-state commerce
let discriminatory = CommerceClauseAnalysis::new()
    .with_regulation_type(StateRegulationType::Discriminatory {
        facially_discriminatory: true,
        protectionist_purpose: true,
    })
    .with_state_interest("Protect in-state businesses")
    .with_alternative_means_available(true);

let result = discriminatory.analyze();
assert!(!result.is_constitutional); // Strict scrutiny → likely unconstitutional

// Non-discriminatory regulation with incidental burden on commerce
let neutral = CommerceClauseAnalysis::new()
    .with_regulation_type(StateRegulationType::Neutral {
        burden_on_interstate_commerce: 0.4, // Moderate burden
    })
    .with_state_interest("Highway safety")
    .with_legitimate_local_concern(true);

let result = neutral.analyze();
// Pike balancing: state interest vs. burden on commerce
```

**Key Cases:**
- *Philadelphia v. New Jersey*, 437 U.S. 617 (1978): State cannot ban out-of-state waste
- *Dean Milk Co. v. Madison*, 340 U.S. 349 (1951): City cannot require local milk processing
- *Pike v. Bruce Church*, 397 U.S. 137 (1970): Balancing test for non-discriminatory laws

---

## Savings Clauses

Some federal statutes contain **savings clauses** explicitly preserving state law:

```rust
// Federal statute with savings clause
let with_savings = PreemptionAnalysis::new(
    "Federal safety regulation",
    "State tort law claim"
)
.with_preemption_type(PreemptionType::Express {
    statutory_text: "Nothing in this Act shall preempt state common law".to_string()
})
.with_savings_clause(true); // Explicitly preserves state law

assert!(!with_savings.analyze().is_preempted);
```

**Examples:**
- **Clean Air Act**: Savings clause preserves state tort remedies
- **Federal Insecticide Act**: Savings clause for state law
- **OSHA**: Savings clause for state worker compensation laws

**But**: Savings clauses don't always save state law if conflict exists. Courts can still find implied preemption.

---

## Best Practices for Preemption Analysis

### Step 1: Identify the Federal and State Laws

```rust
let analysis = PreemptionAnalysis::new(
    "Federal regulation of X",
    "State regulation of Y"
)
.with_federal_statute("Statute name")
.with_state_law_type("Regulation" | "Tort law" | "Criminal law");
```

### Step 2: Check for Express Preemption

```rust
// Look for explicit statutory text
.with_preemption_type(PreemptionType::Express {
    statutory_text: "Quote from statute".to_string()
})
```

### Step 3: Apply Presumption Against Preemption

```rust
// Traditional state law areas → presumption applies
.with_presumption_against(true)

// Federal domains → no presumption
.with_presumption_against(false)
```

### Step 4: Analyze Conflict (if no express preemption)

```rust
// Does state law make federal compliance impossible?
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Impossibility
})

// Or does state law obstruct federal objectives?
.with_preemption_type(PreemptionType::ImpliedConflict {
    conflict_type: ConflictPreemptionType::Obstacle
})
```

### Step 5: Check Confidence Score

```rust
let result = analysis.analyze();

if result.confidence < 0.7 {
    println!("⚠️ Uncertain - requires detailed legal analysis");
    println!("Reasoning: {}", result.reasoning);
}
```

---

## Common Patterns

### Pattern 1: Federal Floor, Not Ceiling

Many federal laws set **minimum standards** that states can exceed:

- Environmental laws (Clean Air Act, Clean Water Act)
- Workplace safety (OSHA)
- Consumer protection (many federal statutes)

**Example**:
```rust
// State can impose stricter emissions standards
let stricter = PreemptionAnalysis::new(
    "Federal minimum emissions standards",
    "California stricter emissions standards"
)
.with_additional_context("Clean Air Act allows California waiver");

assert!(!stricter.analyze().is_preempted);
```

### Pattern 2: Regulations Preempted, Tort Law Not

Common pattern: Federal law preempts state **regulations** but not **tort law**:

- Automobile safety (FMVSS)
- Aviation safety (FAA)
- Railroad safety (FRSA)

### Pattern 3: Complete Field Preemption

Federal law completely occupies field, leaving no room for state law:

- Immigration
- Foreign affairs
- Bankruptcy
- Copyright

---

## Further Reading

**Supreme Court Cases:**
- *Arizona v. United States*, 567 U.S. 387 (2012) - Immigration field preemption
- *Wyeth v. Levine*, 555 U.S. 555 (2009) - Drug labeling (brand-name)
- *PLIVA v. Mensing*, 564 U.S. 604 (2011) - Drug labeling (generic)
- *Geier v. American Honda*, 529 U.S. 861 (2000) - Obstacle preemption
- *Riegel v. Medtronic*, 552 U.S. 312 (2008) - Medical devices (PMA)

**Doctrine:**
- Ernest A. Young, *The Rehnquist Court's Two Federalisms*, 83 Tex. L. Rev. 1 (2004)
- Caleb Nelson, *Preemption*, 86 Va. L. Rev. 225 (2000)

---

**Next**: [Uniform Acts Guide](06-uniform-acts.md)
