# Professional Licensing Guide

**Attorney, Physician, and Architect licensing across 51 US jurisdictions**

## Overview

Professional licensing in the US is **decentralized** - each state independently licenses attorneys, physicians, and architects. This creates barriers to interstate practice and mobility.

Legalis-US tracks three major licensing systems designed to improve portability:

1. **UBE (Uniform Bar Examination)** - Attorney licensing (40+ jurisdictions)
2. **IMLC (Interstate Medical Licensure Compact)** - Physician licensing (35+ states)
3. **NCARB (National Council of Architectural Registration Boards)** - Architect licensing (54 jurisdictions)

---

## Part 1: Attorney Licensing (UBE)

### What is the UBE?

The **Uniform Bar Examination** is a standardized bar exam administered by the National Conference of Bar Examiners (NCBE). Key features:

- **Introduced**: 2011
- **Adoption**: 40 jurisdictions (as of 2024)
- **Score range**: 0-400 points
- **Portability**: Scores transferable between UBE jurisdictions

### UBE Structure

**Three components administered over 2 days:**

```
Day 1 (Tuesday):
├── MEE (Multistate Essay Examination)
│   ├── 6 essays
│   ├── 30% of total score
│   └── 13 subjects tested
└── MPT (Multistate Performance Test)
    ├── 2 performance tasks
    ├── 20% of total score
    └── Practical lawyering skills

Day 2 (Wednesday):
└── MBE (Multistate Bar Examination)
    ├── 200 multiple-choice questions
    ├── 50% of total score
    └── 7 subjects tested
```

### Check UBE Adoption Status

```rust
use legalis_us::professional_licensing::bar_admission::ube_status;

// Check if state uses UBE
let ny_status = ube_status("NY");

match ny_status {
    UBEStatus::Adopted {
        adoption_year,
        minimum_score,
        additional_requirements
    } => {
        println!("New York adopted UBE in {}", adoption_year); // 2016
        println!("Minimum score: {}", minimum_score); // 266
        println!("Additional: {:?}", additional_requirements); // NYLE
    }
    UBEStatus::NotAdopted { exam_name, .. } => {
        println!("Uses state-specific exam: {}", exam_name);
    }
}
```

### UBE Score Transfer

**Key feature**: Take exam once, transfer score to multiple jurisdictions.

```rust
use legalis_us::professional_licensing::bar_admission::can_transfer_ube_score;

// Attorney scored 280 on UBE in New York
// Can they transfer to Colorado?
let score = 280;
let can_transfer = can_transfer_ube_score("NY", "CO", score);

if can_transfer {
    println!("✅ Score of {} meets Colorado's requirement (276)", score);
} else {
    println!("❌ Score too low for Colorado");
}
```

### Minimum Score Requirements

UBE jurisdictions set their own minimum passing scores:

```rust
// Get minimum scores for multiple states
let states = vec!["AL", "AK", "CO", "NY", "TX"];

for state in states {
    if let UBEStatus::Adopted { minimum_score, .. } = ube_status(state) {
        println!("{}: {} minimum", state, minimum_score);
    }
}

// Output:
// AL: 260 minimum (lowest)
// AK: 280 minimum (highest)
// CO: 276 minimum
// NY: 266 minimum
// TX: 270 minimum
```

**Typical Ranges:**
- **Lowest**: 260 (AL, MN, MO, NM, ND, SD)
- **Most common**: 266-270 (33 jurisdictions)
- **Highest**: 280 (AK)

### Strategic Score Transfer

**Scenario**: Attorney takes UBE in Alabama (260 minimum), scores 275.

```rust
// Check which states accept this score
let score = 275;
let target_states = vec!["AL", "AK", "CO", "NY", "TX", "IL", "MA"];

for state in target_states {
    let can_transfer = can_transfer_ube_score("AL", state, score);
    if can_transfer {
        println!("✅ {}", state);
    } else {
        println!("❌ {} (score too low)", state);
    }
}

// ✅ AL (260)
// ❌ AK (280) - score too low
// ❌ CO (276) - score too low
// ✅ NY (266)
// ✅ TX (270)
// ✅ IL (266)
// ✅ MA (270)
```

### Non-UBE States (11 remaining)

**Notable states that maintain their own exams:**

```rust
// California - largest legal market
let ca = ube_status("CA");
assert!(matches!(ca, UBEStatus::NotAdopted { .. }));
// CA maintains its own exam (historically ~40-50% pass rate)

// Louisiana - only Civil Law state
let la = ube_status("LA");
if let UBEStatus::NotAdopted { exam_name, .. } = la {
    assert!(exam_name.contains("Civil Law"));
    // Tests both Common Law and Louisiana Civil Code
}

// Florida - second-largest non-UBE market
let fl = ube_status("FL");
// Includes unique public policy questions
```

**Full list of non-UBE states:**
- California (CA)
- Delaware (DE)
- Florida (FL)
- Georgia (GA)
- Hawaii (HI)
- Indiana (IN)
- Kentucky (KY)
- Louisiana (LA)
- Michigan (MI)
- Mississippi (MS)
- Nevada (NV)
- Oklahoma (OK)

### Complete Bar Requirements

```rust
use legalis_us::professional_licensing::bar_admission::bar_requirements;

// Get all requirements for New York
let ny_reqs = bar_requirements("NY");

println!("UBE Status: {:?}", ny_reqs.ube_status);
println!("MPRE Required: {:?}", ny_reqs.mpre_minimum_score); // Some(85)
println!("Character & Fitness: {}", ny_reqs.character_and_fitness); // true
println!("Pro Hac Vice Available: {}", ny_reqs.pro_hac_vice_available); // true

// Law school requirements
let law_school = &ny_reqs.law_school_requirements;
println!("ABA Accreditation Required: {}", law_school.aba_accredited_required);
```

**MPRE (Ethics Exam):**
- **Standard**: 85/120 (most states)
- **Higher**: 86/120 (CA, UT)
- **Lower**: 75/120 (GA, AZ)

---

## Part 2: Physician Licensing (IMLC)

### What is IMLC?

The **Interstate Medical Licensure Compact** expedites licensing for physicians practicing in multiple states.

- **Launched**: 2017
- **Member states**: 35+ (as of 2024)
- **Process**: Apply once, get licensed in multiple states
- **Typical timeline**: 30-60 days (vs. 6+ months per state)

### Check IMLC Membership

```rust
use legalis_us::professional_licensing::medical::is_imlc_member;

// Is Texas an IMLC member?
if is_imlc_member("TX") {
    println!("✅ Texas is an IMLC member");
    println!("Physicians can use expedited multi-state licensing");
} else {
    println!("❌ Texas is not an IMLC member");
    println!("Must apply for license separately");
}
```

### IMLC Requirements

**To use IMLC, physician must:**
1. Hold full medical license in state of principal licensure
2. Not be under investigation or discipline
3. Pass FBI background check
4. Meet state-specific requirements
5. Pay fees to each state

```rust
use legalis_us::professional_licensing::medical::{IMLCStatus, telemedicine_requirements};

// Check state's IMLC status and telemedicine rules
let tx_status = IMLCStatus::Member {
    join_date: NaiveDate::from_ymd_opt(2018, 5, 1).unwrap(),
    requirements: vec!["FBI background check".into()],
};

// Get telemedicine requirements
let tele_reqs = telemedicine_requirements("TX");
println!("Telemedicine allowed: {}", tele_reqs.allowed);
println!("License required: {}", tele_reqs.license_required);
```

### Telemedicine Regulations

**Key Issue**: Does physician need license in patient's state or physician's state?

**General Rule**: License required in **patient's location** (not physician's location).

```rust
// Physician in California, patient in Texas
// Physician needs TEXAS license (where patient is located)

if is_imlc_member("TX") && is_imlc_member("CA") {
    println!("✅ Can use IMLC for expedited TX license");
} else {
    println!("⚠️ Must apply for full TX license (6+ months)");
}
```

**IMLC Member States** (35+ as of 2024):
Alabama, Arizona, Colorado, Florida, Georgia, Idaho, Illinois, Indiana, Iowa, Kansas, Kentucky, Louisiana, Maryland, Michigan, Minnesota, Mississippi, Montana, Nebraska, Nevada, New Hampshire, North Carolina, North Dakota, Ohio, Oklahoma, Pennsylvania, South Carolina, South Dakota, Tennessee, Texas, Utah, Vermont, Virginia, Washington, West Virginia, Wisconsin, Wyoming

---

## Part 3: Architect Licensing (NCARB)

### What is NCARB?

The **National Council of Architectural Registration Boards** provides reciprocal licensing for architects.

- **Coverage**: All 54 US jurisdictions (50 states + DC + 3 territories)
- **Certificate**: NCARB certification enables practice in multiple states
- **Exam**: ARE (Architect Registration Examination) - standardized across states

### Check NCARB Reciprocity

```rust
use legalis_us::professional_licensing::architect::can_use_ncarb_certificate;

// Architect has NCARB certificate from New York
// Can they practice in California?
let can_practice = can_use_ncarb_certificate("NY", "CA");

if can_practice {
    println!("✅ NCARB certificate enables CA practice");
} else {
    println!("❌ Additional requirements needed");
}
```

### State-Specific Requirements

Some states require additional exams beyond NCARB:

```rust
use legalis_us::professional_licensing::architect::{ArchitectLicensing, NCARBStatus};

let ca_licensing = ArchitectLicensing {
    state_id: StateId::from_code("CA"),
    ncarb_status: NCARBStatus::Accepted {
        additional_requirements: vec![
            "California Supplemental Examination (CSE)".to_string(),
            "California laws and regulations".to_string(),
        ],
    },
    reciprocity: ReciprocityType::Conditional {
        requirements: vec!["Pass CSE".into()],
    },
    continuing_education_required: true,
    hours_per_cycle: Some(20.0),
};

println!("Additional requirements for CA: {:?}",
         ca_licensing.ncarb_status);
```

**California Supplemental Exam (CSE):**
- Tests CA-specific building codes
- Required even with NCARB certificate
- 4-hour exam

---

## Comparison: Three Systems

| Feature | UBE (Attorney) | IMLC (Physician) | NCARB (Architect) |
|---------|----------------|------------------|-------------------|
| **Adoption** | 40/51 states | 35+ states | 54/54 jurisdictions |
| **Exam** | Standardized UBE | State boards vary | Standardized ARE |
| **Portability** | Score transfer | Expedited licensing | Reciprocal practice |
| **Timeline** | Immediate | 30-60 days | Varies by state |
| **Additional Requirements** | Rare (NY NYLE) | Common | Common (CA CSE) |
| **Success** | High (78% adoption) | Growing | Universal |

---

## Practical Use Cases

### Use Case 1: Multi-State Law Firm

**Problem**: Law firm expanding from NY to CO, TX, FL.

```rust
// Check which associates can easily transfer
let associates = vec![
    ("Alice", "NY", 275),
    ("Bob", "NY", 285),
    ("Carol", "NY", 260),
];

let target_states = vec!["CO", "TX", "FL"];

for (name, home_state, score) in associates {
    println!("\n{} (score: {}):", name, score);

    for target in &target_states {
        if can_transfer_ube_score(home_state, target, score) {
            println!("  ✅ Can transfer to {}", target);
        } else {
            let required = match ube_status(target) {
                UBEStatus::Adopted { minimum_score, .. } => minimum_score,
                _ => 0,
            };
            println!("  ❌ Cannot transfer to {} (needs {})", target, required);
        }
    }
}
```

### Use Case 2: Telemedicine Platform

**Problem**: Platform connects physicians with patients nationwide.

```rust
// Check which states require physician licenses
let physician_state = "CA";
let patient_states = vec!["TX", "FL", "NY", "AZ"];

for patient_state in patient_states {
    if is_imlc_member(physician_state) && is_imlc_member(patient_state) {
        println!("{}: ✅ Use IMLC for expedited license", patient_state);
    } else {
        println!("{}: ⚠️ Full license application required", patient_state);
    }
}
```

### Use Case 3: Architectural Firm Expansion

**Problem**: Firm wants to bid on projects in multiple states.

```rust
// Check reciprocity for firm's architects
let home_state = "NY";
let project_states = vec!["CA", "TX", "FL", "IL"];

for state in project_states {
    let licensing = architect::licensing_requirements(state);

    match licensing.ncarb_status {
        NCARBStatus::FullReciprocity => {
            println!("{}: ✅ Full reciprocity", state);
        }
        NCARBStatus::Accepted { additional_requirements } => {
            println!("{}: ⚠️ Additional requirements:", state);
            for req in additional_requirements {
                println!("   - {}", req);
            }
        }
        _ => {}
    }
}
```

---

## Best Practices

### For Attorneys

1. **Take UBE in low-minimum state** if planning multi-state practice (AL, MN, MO: 260)
2. **Score high** (280+) to maximize transferability
3. **Check additional requirements** (NY requires NYLE)
4. **Time limits**: Transfer within 3-5 years of taking exam

### For Physicians

1. **Establish principal license** in IMLC member state
2. **Apply early**: Process takes 30-60 days
3. **Maintain clean record**: Discipline blocks IMLC eligibility
4. **Know telemedicine rules**: License follows patient location

### For Architects

1. **Get NCARB certificate** early in career
2. **Check state-specific exams** (CA CSE, etc.)
3. **Maintain continuing education** in all practice states
4. **Update registration** before bidding on projects

---

## Future Developments

### Trends

**UBE Adoption**: Slowing after rapid growth
- 2011-2022: 37 states adopted
- 2023-2024: Only 3 states adopted
- CA, FL, LA unlikely to ever adopt

**IMLC Expansion**: Growing steadily
- Added 5+ states in 2023-2024
- Interstate telemedicine driving adoption
- COVID-19 accelerated acceptance

**NCARB**: Already universal
- All jurisdictions participate
- Focus on streamlining process
- Digital credentials coming

---

**Next**: [Tax Law Guide](04-tax-law.md)
