# legalis-us

United States Jurisdiction Support for Legalis-RS

## Overview

`legalis-us` provides comprehensive Common Law system support for the Legalis-RS framework, including:

- **Restatement of Torts (ALI)** - Synthesized principles from case law
- **Landmark Cases** - Precedential decisions (Palsgraf, Donoghue, etc.)
- **State-Specific Laws** - Variations across 51 jurisdictions (all 50 states + DC)
- **Choice of Law** - Multi-state conflict resolution (5 approaches)
- **Uniform Acts** - UCC and UPA adoption tracking across states
- **Federal-State Boundary** - Preemption and Commerce Clause analysis
- **Professional Licensing** - Attorney (UBE), physician (IMLC), architect (NCARB) licensing
- **Tax Law Variations** - Income, sales, and corporate tax across all jurisdictions
- **Legislative Tracking** - Policy adoption (cannabis, privacy, right to repair) and constitutional provisions

## Current Status

**Phase 1 (Core Infrastructure): 100% Complete ✅**
**Phase 2 (State Expansion): 100% Complete ✅**
**Phase 3 (Professional Licensing): 100% Complete ✅**
**Phase 4 (Tax Law Variations): 100% Complete ✅**
**Phase 5 (Legislative Tracking): 100% Complete ✅**

- ✅ Phase 1A: Foundation (types, registry) - 880 lines
- ✅ Phase 1B: 5 Priority States (CA, NY, TX, LA, FL) - 2,130 lines
- ✅ Phase 1C: State Law Comparator - 420 lines
- ✅ Phase 1D: Choice of Law Enhancement - 1,354 lines
- ✅ Phase 1E: Uniform Acts Tracker - 1,502 lines
- ✅ Phase 1F: Federal-State Boundary - 1,106 lines
- ✅ Phase 2: Remaining 45 States + DC (51 total) - 5,331 lines
- ✅ Phase 3: Professional Licensing (Attorney, Medical, Architect) - 1,667 lines
- ✅ Phase 4: Tax Law Variations (Income, Sales, Corporate) - 1,197 lines
- ✅ Phase 5: Legislative Tracking (Policy Adoption, Constitutional Provisions) - 1,100 lines
- **436 tests passing, 0 warnings**
- **Total: ~18,700+ lines of production code**

## Features

### 1. Restatement of Torts (ALI)

The American Law Institute's Restatement of Torts synthesizes Common Law principles into structured rules:

```rust
use legalis_us::{section_158_battery, section_46_iied, section_402a_products_liability};

// Battery - Harmful or offensive contact
let battery = section_158_battery();
assert!(battery.name.contains("158"));

// IIED - Extreme and outrageous conduct
let iied = section_46_iied();
assert!(iied.name.contains("46"));

// Products liability - Strict liability
let products = section_402a_products_liability();
assert!(products.name.contains("402A"));
```

**Sections Available:**
- **§ 158 - Battery**: Harmful or offensive contact with another's person
- **§ 46 - Intentional Infliction of Emotional Distress (IIED)**: Extreme and outrageous conduct causing severe emotional distress
- **§ 402A - Products Liability**: Strict liability for defective products

### 2. Landmark Tort Cases

Famous cases that established key precedents:

```rust
use legalis_us::{palsgraf_v_long_island, donoghue_v_stevenson, garratt_v_dailey};

// Foreseeability in negligence
let palsgraf = palsgraf_v_long_island();
assert_eq!(palsgraf.year, 1928);
assert!(palsgraf.holding.contains("foreseeable"));

// Neighbor principle and duty of care
let donoghue = donoghue_v_stevenson();
assert_eq!(donoghue.year, 1932);

// Intent in battery
let garratt = garratt_v_dailey();
assert_eq!(garratt.year, 1955);
```

**Cases Available:**
- **Palsgraf v. Long Island Railroad (1928)** - Foreseeability in negligence
- **Donoghue v. Stevenson (1932)** - Neighbor principle and duty of care
- **Garratt v. Dailey (1955)** - Intent in battery
- **Vosburg v. Putney (1891)** - Transferred intent doctrine

### 3. State-Specific Laws (5 Priority States)

Compare legal rules across different US states:

#### California
```rust
use legalis_us::states::california::CaliforniaLaw;

let ca = CaliforniaLaw::default();

// Pure comparative negligence (Li v. Yellow Cab, 1975)
let comp_neg = ca.comparative_negligence_variation();
assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);

// Interest analysis choice-of-law approach
assert_eq!(ca.choice_of_law_approach(), ChoiceOfLawApproach::InterestAnalysis);

// CCPA privacy protection
let ccpa = ca.ccpa_statute();
assert!(ccpa.is_valid());
```

#### New York
```rust
use legalis_us::states::new_york::NewYorkLaw;

let ny = NewYorkLaw::default();

// Pure comparative negligence (CPLR § 1411)
let comp_neg = ny.comparative_negligence_variation();
assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);

// Cardozo legacy integration
let palsgraf_integration = ny.integrate_palsgraf();
assert!(palsgraf_integration.is_some());
```

#### Texas
```rust
use legalis_us::states::texas::TexasLaw;

let tx = TexasLaw::default();

// Modified comparative negligence (51% bar)
let comp_neg = tx.comparative_negligence_variation();
assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);

// Medical malpractice damage caps
let med_mal_cap = tx.medical_malpractice_cap();
assert_eq!(med_mal_cap, 250_000); // $250,000 non-economic
```

#### Louisiana (Special: Only Civil Law State)
```rust
use legalis_us::states::louisiana::LouisianaLaw;

let la = LouisianaLaw::default();

// Louisiana Civil Code Article 2315 (tort)
let article_2315 = la.article_2315_delict();
assert!(article_2315.is_valid());

// Compare with other Civil Law jurisdictions
let comparison = la.compare_with_minpo_709(); // Japan Minpo 709
assert!(comparison.similarity > 0.70);

let comparison = la.compare_with_code_civil_1240(); // France
assert!(comparison.similarity > 0.80);
```

#### Florida
```rust
use legalis_us::states::florida::FloridaLaw;

let fl = FloridaLaw::default();

// Pure comparative negligence
let comp_neg = fl.comparative_negligence_variation();
assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);

// Stand Your Ground law
let stand_your_ground = fl.stand_your_ground_statute();
assert!(stand_your_ground.is_valid());
```

### 4. State Law Comparator

Compare legal rules across multiple states:

```rust
use legalis_us::states::{StateLawComparator, LegalTopic};

let comparator = StateLawComparator::new();

// Compare comparative negligence rules across 5 states
let comparison = comparator.compare_states(
    LegalTopic::ComparativeNegligence,
    &["CA", "NY", "TX", "NC", "FL"],
);

// Get majority rule
assert!(comparison.majority_rule.is_some());
assert_eq!(comparison.majority_rule.unwrap(), StateRule::PureComparativeNegligence);

// Get minority states
let minority = comparison.minority_states();
assert!(minority.contains(&"TX")); // Modified 51%
assert!(minority.contains(&"NC")); // Contributory negligence

// Generate comparison report
let report = comparator.generate_report(&comparison);
println!("{}", report);
```

**Comparison Topics:**
- Comparative Negligence (Pure vs Modified 50% vs 51% vs Contributory)
- Joint and Several Liability
- Dram Shop Liability
- Products Liability variations
- Statute of Limitations
- Damage Caps

### 5. Choice of Law Analysis

Determine which state's law applies to multi-state disputes:

```rust
use legalis_us::choice_of_law::{USChoiceOfLawAnalyzer, ChoiceOfLawApproach};
use legalis_core::LegalFacts;

// Create analyzer with Restatement (Second) approach (majority rule)
let analyzer = USChoiceOfLawAnalyzer::new(ChoiceOfLawApproach::RestatementSecond);

// Analyze tort scenario
let mut facts = LegalFacts::new();
facts.add_location("Injury occurred", "CA");
facts.add_location("Conduct occurred", "NY");
facts.add_location("Plaintiff resides", "CA");
facts.add_location("Defendant resides", "NY");

let result = analyzer.analyze(&facts);

// Result indicates which state's law applies
assert!(result.governing_jurisdiction.is_some());
println!("Governing law: {}", result.governing_jurisdiction.unwrap());
println!("Confidence: {:.1}%", result.confidence * 100.0);
```

**Approaches Supported:**
- **Restatement (First)**: Traditional "place of wrong" rule
- **Restatement (Second)**: Modern "most significant relationship" test (44 states)
- **Interest Analysis**: California approach
- **Better Law**: Minnesota approach
- **Combined Modern**: New York approach

### 6. Uniform Acts Tracker

Track adoption status of uniform laws across states:

#### UCC (Uniform Commercial Code)
```rust
use legalis_us::uniform_acts::{UCCTracker, UCCArticle};

let tracker = UCCTracker::new();

// Check if state adopted UCC Article 2 (Sales)
assert!(tracker.has_adopted("CA", UCCArticle::Article2));
assert!(tracker.has_adopted("NY", UCCArticle::Article2));
assert!(!tracker.has_adopted("LA", UCCArticle::Article2)); // Louisiana exception

// Get state-specific variations
let variations = tracker.state_variations("CA", UCCArticle::Article2);
for variation in variations {
    println!("CA variation: {}", variation);
}

// Compare adoptions across states
let comparison = tracker.compare_adoptions(UCCArticle::Article2);
assert_eq!(comparison.adopted_count, 50); // All states except LA
```

#### UPA (Uniform Partnership Act)
```rust
use legalis_us::uniform_acts::{UPATracker, PartnershipActVersion};

let tracker = UPATracker::new();

// Check which states adopted RUPA (Revised 1997)
let rupa_states = tracker.rupa_states();
assert!(rupa_states.len() > 40); // Majority adoption

// Check adoption percentage
let percentage = tracker.rupa_adoption_percentage();
assert!(percentage > 80.0);

// Louisiana uses Civil Code instead
let la_adoption = tracker.get_adoption("LA").unwrap();
assert_eq!(la_adoption.version, PartnershipActVersion::Custom);
```

### 7. Federal-State Boundary Analysis

Analyze federal preemption and Commerce Clause constraints:

#### Preemption Analysis
```rust
use legalis_us::federal::{PreemptionAnalysis, PreemptionType, FieldPreemptionAnalysis};

// Express preemption example (FAAAA)
let analysis = PreemptionAnalysis::new(
    "Federal Aviation Administration Authorization Act",
    "California AB5 (worker classification)",
)
.with_express_language(
    "a State may not enact or enforce a law related to a price, route, or service"
)
.with_subject_matter("Motor carrier regulation");

let result = analysis.analyze();
assert_eq!(result.preemption_type, PreemptionType::Express);
assert!(result.preempted);
assert!(result.confidence > 0.90);

// Field preemption example (immigration)
let field = FieldPreemptionAnalysis::new()
    .with_comprehensive_scheme(true)
    .with_congressional_intent(true)
    .with_traditionally_federal_domain(true);

let analysis = PreemptionAnalysis::new("Immigration and Nationality Act", "Arizona SB 1070")
    .with_field_analysis(field)
    .with_subject_matter("Immigration enforcement");

let result = analysis.analyze();
assert_eq!(result.preemption_type, PreemptionType::ImpliedField);
assert!(result.preempted);
```

#### Dormant Commerce Clause
```rust
use legalis_us::federal::{CommerceClauseAnalysis, DormantCommerceClauseTest};

// Discrimination test
let analysis = CommerceClauseAnalysis::new("NJ", "Prohibition on out-of-state waste imports")
    .with_discrimination("Explicitly prohibits out-of-state waste while allowing in-state waste")
    .with_subject_matter("Waste disposal");

let result = analysis.analyze();
assert_eq!(result.test, DormantCommerceClauseTest::Discrimination);
assert!(!result.valid); // Discriminatory laws are nearly per se invalid

// Pike balancing test
let analysis = CommerceClauseAnalysis::new("IL", "Inspection requirements for imported produce")
    .with_burden("Requires inspection of out-of-state produce")
    .with_local_benefit("Protects Illinois consumers from contaminated produce")
    .with_subject_matter("Food safety");

let result = analysis.analyze();
assert_eq!(result.test, DormantCommerceClauseTest::PikeBalancing);
assert!(result.valid); // Benefits may outweigh burdens

// Market participant exception
let analysis = CommerceClauseAnalysis::new("MD", "Preference for Maryland residents in state park jobs")
    .with_market_participant_exception()
    .with_subject_matter("State employment");

let result = analysis.analyze();
assert!(result.valid); // Exception applies
```

**Preemption Types:**
- **Express Preemption**: Explicit statutory language preempts state law
- **Implied Field Preemption**: Federal scheme so comprehensive it occupies entire field
- **Implied Conflict Preemption**: State law conflicts with federal law

**Commerce Clause Tests:**
- **Discrimination Test**: Strict scrutiny for laws favoring in-state commerce (nearly per se invalid)
- **Pike Balancing Test**: Burden vs. benefit analysis for non-discriminatory laws

### 8. Professional Licensing Across States

Track attorney, physician, and architect licensing requirements across all US jurisdictions:

#### Attorney Licensing (Bar Admission)

```rust
use legalis_us::professional_licensing::{ube_status, can_transfer_ube_score};

// Check UBE adoption status
let ny_status = ube_status("NY");
assert!(matches!(ny_status, UBEStatus::Adopted { minimum_score: 266, .. }));

// Check UBE score transferability
assert!(can_transfer_ube_score("NY", "CO", 280)); // 280 meets CO's 276 minimum
assert!(!can_transfer_ube_score("NY", "CA", 300)); // CA doesn't use UBE
```

**Uniform Bar Examination (UBE):**
- **40+ jurisdictions adopted** UBE for portable bar scores
- **Minimum scores vary**: 260-280 points (out of 400)
- **Notable non-UBE states**: California, Louisiana, Florida, Nevada
- **Score portability**: Transfer UBE scores between member states

#### Medical Licensing (IMLC)

```rust
use legalis_us::professional_licensing::{is_imlc_member, telemedicine_requirements};

// Check Interstate Medical Licensure Compact membership
assert!(is_imlc_member("TX")); // Texas is a member
assert!(!is_imlc_member("CA")); // California is not

// Get telemedicine regulations
let tx_telemedicine = telemedicine_requirements("TX");
assert!(tx_telemedicine.special_telemedicine_license); // TX has special license
assert!(tx_telemedicine.initial_in_person_required); // TX requires in-person visit
```

**Interstate Medical Licensure Compact (IMLC):**
- **35+ member states** expedite multi-state licensing
- **Processing time**: 30-90 days (vs. 6+ months traditional)
- **Telemedicine rules**: State-specific requirements for remote care
- **Prescribing authority**: Opioid limits and controlled substance regulations

#### Architect Licensing (NCARB)

```rust
use legalis_us::professional_licensing::{can_use_ncarb_certificate, ncarb_status};

// NCARB Certificate provides reciprocity in most states
assert!(can_use_ncarb_certificate("TX")); // Full reciprocity
assert!(can_use_ncarb_certificate("CA")); // Conditional (CSE required)

// Check NCARB reciprocity status
let ca_status = ncarb_status("CA");
// California requires California Supplemental Examination (CSE)
```

**NCARB (National Council of Architectural Registration Boards):**
- **54 jurisdictions** recognize NCARB Certificate
- **Reciprocal licensure**: Streamlined application process
- **State-specific exams**: Some states require additional testing (CA, NY)
- **Continuing education**: Varies by state (12-40 hours per renewal period)

### 9. Tax Law Variations Across States

Comprehensive state-by-state tax analysis covering income, sales, and corporate taxation:

#### State Income Tax

```rust
use legalis_us::tax::income_tax::{has_state_income_tax, income_tax_structure, IncomeTaxType};

// Check if state has income tax
assert!(!has_state_income_tax("TX")); // Texas: no income tax
assert!(!has_state_income_tax("FL")); // Florida: no income tax
assert!(has_state_income_tax("CA"));  // California: progressive

// Get tax structure details
let ca_tax = income_tax_structure("CA");
if let IncomeTaxType::Progressive { top_rate, .. } = ca_tax.tax_type {
    assert_eq!(top_rate, 0.1330); // 13.3% - highest in US
}

// Flat tax states
let il_tax = income_tax_structure("IL");
assert!(matches!(il_tax.tax_type, IncomeTaxType::Flat { rate: 0.0495 }));
```

**State Income Tax Patterns:**
- **9 no-tax states**: AK, FL, NV, SD, TN, TX, WA, WY, NH
- **9 flat-tax states**: CO, IL, IN, KY, MA, MI, NC, PA, UT (rates: 3.07% - 5.00%)
- **33 progressive-tax states**: Majority use graduated brackets
- **Highest rate**: California 13.3% (top bracket for income over $1M)
- **Local income taxes**: NYC (3.876%), Philadelphia (3.79%), and others

#### Sales Tax

```rust
use legalis_us::tax::sales_tax::{has_sales_tax, state_sales_tax_rate, post_wayfair_nexus};

// No sales tax states (5)
assert!(!has_sales_tax("OR")); // Oregon: no sales tax
assert!(!has_sales_tax("DE")); // Delaware: no sales tax

// Sales tax rates
let ca_tax = state_sales_tax_rate("CA");
assert_eq!(ca_tax.state_rate, 0.0725); // 7.25% - highest state rate
assert!(ca_tax.max_combined_rate.unwrap() > 0.10); // Up to 10.25% with locals

// Post-Wayfair economic nexus
let sd_nexus = post_wayfair_nexus("SD"); // South Dakota v. Wayfair origin
assert!(sd_nexus.marketplace_facilitator_law);
```

**Sales Tax Patterns:**
- **5 no-tax states**: AK, DE, MT, NH, OR
- **Lowest state rate**: Colorado 2.9%
- **Highest state rate**: California 7.25%
- **Highest combined**: Louisiana avg 9.55% (state + local)
- **Economic nexus**: Most states use $100k sales or 200 transactions threshold
- **Post-Wayfair**: All states can require remote seller collection

#### Corporate Tax

```rust
use legalis_us::tax::corporate_tax::{corporate_tax_rate, is_tax_haven};

// Corporate tax havens
assert!(is_tax_haven("DE")); // Delaware: most corporations incorporate here
assert!(is_tax_haven("NV")); // Nevada: no corporate income tax
assert!(is_tax_haven("WY")); // Wyoming: no corporate income tax

// Tax rates
let nj_tax = corporate_tax_rate("NJ");
assert_eq!(nj_tax.tax_rate, 0.1150); // 11.5% - highest in US

let de_tax = corporate_tax_rate("DE");
assert_eq!(de_tax.tax_rate, 0.0885); // 8.85% but still tax haven
```

**Corporate Tax Patterns:**
- **6 no-corporate-tax states**: NV, SD, WY, WA (B&O tax), TX (franchise tax), OH (CAT)
- **Corporate tax havens**: Delaware (Court of Chancery), Nevada, Wyoming
- **Highest rate**: New Jersey 11.5%
- **Apportionment**: Most states use single-factor sales formula
- **Combined reporting**: CA, NY, IL, MA, and 20+ others require it
- **Delaware dominance**: 60%+ of Fortune 500 companies incorporated there

### 10. Legislative Tracking (Phase 5)

Track policy adoption and constitutional provisions across all US states.

#### 10.1 Policy Adoption Tracker

Track key policy areas across jurisdictions:

**Cannabis Legalization:**

```rust
use legalis_us::legislative::policy_tracker::{cannabis_status, CannabisStatus};

// Recreational states (25 including DC)
let ca = cannabis_status("CA");
assert_eq!(ca, CannabisStatus::RecreationalLegal { year_enacted: 2016 });

// Medical only states
let fl = cannabis_status("FL");
assert_eq!(fl, CannabisStatus::MedicalOnly { year_enacted: 2016 });

// Fully illegal states
let id = cannabis_status("ID");
assert_eq!(id, CannabisStatus::Illegal);
```

**Data Privacy Laws:**

```rust
use legalis_us::legislative::policy_tracker::{has_comprehensive_privacy_law, comprehensive_privacy_laws, DataPrivacyLaw};

// States with comprehensive privacy laws (17+ as of 2024)
assert!(has_comprehensive_privacy_law("CA")); // CCPA/CPRA (2018)
assert!(has_comprehensive_privacy_law("VA")); // VCDPA (2021)
assert!(has_comprehensive_privacy_law("CO")); // CPA (2021)
assert!(has_comprehensive_privacy_law("CT")); // CTDPA (2022)

let ca_laws = comprehensive_privacy_laws("CA");
assert_eq!(ca_laws.len(), 1);
assert!(matches!(ca_laws[0], DataPrivacyLaw::CCPA { enacted: 2018, cpra_enhanced: true }));
```

**Right to Repair:**

```rust
use legalis_us::legislative::policy_tracker::{right_to_repair_status, RightToRepairStatus};

// Massachusetts: Automotive right to repair (2012, expanded 2020)
let ma = right_to_repair_status("MA");
assert!(matches!(ma, RightToRepairStatus::Comprehensive { automotive: true, electronics: true, .. }));

// California: Electronics right to repair (SB 244, 2023)
let ca = right_to_repair_status("CA");
assert!(matches!(ca, RightToRepairStatus::Comprehensive { electronics: true, .. }));
```

**Policy Patterns:**
- **Cannabis**: 25 recreational (including DC), 16 medical-only, 2 decriminalized, 8 illegal
- **Privacy Laws**: 17+ states with comprehensive privacy laws (CA, VA, CO, CT, UT, MT, OR, TX, IA, TN, DE, FL, IN, KY, NE, NH, NJ)
- **Right to Repair**: 6 states (CA, CO, MA, MN, NY, OR)

#### 10.2 Constitutional Provisions

Track state constitutional rights beyond federal minimums:

**Constitutional Privacy Rights:**

```rust
use legalis_us::legislative::constitutional::{constitutional_privacy_right, ConstitutionalPrivacyRight};

// States with explicit constitutional privacy rights (10 states)
let ca = constitutional_privacy_right("CA");
assert!(matches!(ca, ConstitutionalPrivacyRight::Explicit { year_adopted: 1972, .. }));

let fl = constitutional_privacy_right("FL");
assert!(matches!(fl, ConstitutionalPrivacyRight::Explicit { year_adopted: 1980, .. }));

// States with implicit privacy rights
let ny = constitutional_privacy_right("NY");
assert!(matches!(ny, ConstitutionalPrivacyRight::Implicit { .. }));

// No constitutional privacy right
let tx = constitutional_privacy_right("TX");
assert_eq!(tx, ConstitutionalPrivacyRight::None);
```

**Initiative and Referendum:**

```rust
use legalis_us::legislative::constitutional::{has_initiative_referendum, state_constitutional_provisions};

// States with citizen initiative/referendum (23 states)
assert!(has_initiative_referendum("CA")); // Yes (since 1911)
assert!(has_initiative_referendum("OR")); // Yes (since 1902 - pioneer)
assert!(!has_initiative_referendum("TX")); // No (legislative referral only)

// Detailed provisions
let ca_provisions = state_constitutional_provisions("CA");
assert!(ca_provisions.direct_democracy.signature_threshold.is_some());
assert!(!ca_provisions.direct_democracy.notable_measures.is_empty());
assert!(!ca_provisions.beyond_federal_floor.is_empty());
```

**Constitutional Patterns:**
- **Explicit Privacy Rights**: 10 states (AK, AZ, CA, FL, HI, IL, LA, MT, SC, WA)
- **Citizen Initiative**: 23 states (CA, OR, AZ, CO, etc. - Progressive Era reforms)
- **Direct Democracy**: Pioneered by Oregon (1902), South Dakota (1898)
- **Notable Initiatives**: CA Prop 13 (1978), CO Amendment 64 (2012 cannabis), MA Question 1 (2012 right to repair)
- **Beyond Federal Floor**: Many states provide broader protections than U.S. Constitution

## Common Law vs Civil Law

The US legal system (derived from English Common Law) differs fundamentally from Civil Law systems (Japan, Germany, France):

### Civil Law Approach (大陸法)

```text
Legislature
    ↓
Code/Statute (e.g., 民法709条, BGB §823, Code civil 1240)
    ↓
Courts apply statute to cases
```

### Common Law Approach (英米法)

```text
Case 1 → Precedent A
    ↓
Case 2 cites Case 1 → Refines Precedent A
    ↓
Case 3 distinguishes → Exception to Precedent A
    ↓
Restatement synthesizes → § X: Rule A (non-binding)
    ↓
Case 4 adopts Restatement § X
```

### Key Differences

| Feature | Civil Law | Common Law |
|---------|-----------|------------|
| Primary Source | Statutes/Codes | Cases/Precedents |
| Court Role | Apply code | Make law |
| Reasoning | Deductive (code → case) | Analogical (case → case) |
| Binding Force | Statute text | Prior holdings (stare decisis) |
| Flexibility | Low (legislature must amend) | High (courts distinguish) |

### Why This Matters for Legalis-RS

- **Civil Law modeling** uses `Statute` objects (e.g., 民法709条)
- **Common Law modeling** uses `Case` objects with `precedent_weight()`

The same tort concept appears differently:
- **Civil Law**: Article 709 (statute) → "intent OR negligence"
- **Common Law**: Palsgraf (case) → "duty to foreseeable plaintiff"

### Louisiana: The Exception

Louisiana is the **only US state** using a Civil Law system (French heritage). This creates unique opportunities for cross-jurisdiction comparison:

```rust
use legalis_us::states::louisiana::LouisianaLaw;

let la = LouisianaLaw::default();

// Louisiana Civil Code Article 2315 vs Japan Minpo 709
let comparison = la.compare_with_minpo_709();
assert!(comparison.similarity > 0.70); // Both Civil Law tort provisions

// Louisiana vs France Code civil 1240
let comparison = la.compare_with_code_civil_1240();
assert!(comparison.similarity > 0.80); // French heritage

// Louisiana vs Germany BGB §823
let comparison = la.compare_with_bgb_823();
assert!(comparison.similarity > 0.70); // Civil Law similarities
```

## State Jurisdiction Codes

States are identified using ISO 3166-2:US codes:

- **Federal**: `"US"`
- **Restatement**: `"US-RESTATEMENT"`
- **State**: `"US-CA"`, `"US-NY"`, `"US-TX"`, `"US-LA"`, `"US-FL"`

## Module Organization

```
us/src/
├── lib.rs              # Public API and re-exports
├── cases.rs            # Landmark tort cases
├── restatement.rs      # ALI Restatement sections
│
├── states/             # State-specific laws
│   ├── types.rs        # Core data structures
│   ├── registry.rs     # State metadata
│   ├── comparator.rs   # State comparison engine
│   ├── california.rs
│   ├── new_york.rs
│   ├── texas.rs
│   ├── louisiana.rs
│   └── florida.rs
│
├── choice_of_law/      # Multi-state conflict resolution
│   ├── factors.rs      # US connecting factors
│   ├── restatement_first.rs
│   ├── restatement_second.rs
│   └── analyzer.rs
│
├── uniform_acts/       # UCC and UPA tracking
│   ├── ucc.rs
│   ├── upa.rs
│   └── adoption_status.rs
│
├── federal/            # Federal-state boundary
│   ├── preemption.rs
│   └── commerce_clause.rs
│
├── professional_licensing/  # Professional licensing across states
│   ├── types.rs             # Common licensing types
│   ├── bar_admission.rs     # Attorney licensing (UBE)
│   ├── medical.rs           # Physician licensing (IMLC)
│   └── architect.rs         # Architect licensing (NCARB)
│
├── tax/                     # State tax law variations
│   ├── mod.rs               # Tax module organization
│   ├── income_tax.rs        # State income tax (9 no-tax states)
│   ├── sales_tax.rs         # Sales tax & post-Wayfair nexus
│   └── corporate_tax.rs     # Corporate tax & tax havens
│
└── legislative/             # Policy adoption and constitutional provisions
    ├── mod.rs               # Legislative tracking module
    ├── policy_tracker.rs    # Cannabis, privacy, right to repair
    └── constitutional.rs    # State constitutional provisions
```

## Testing

All features are comprehensively tested with **409 tests** covering:

```bash
# Run all tests
cargo nextest run --all-features

# Run specific module tests
cargo test states::california
cargo test choice_of_law
cargo test federal::preemption

# Check for warnings (zero tolerance policy)
cargo clippy --all-features
```

**Current Test Status:** 166 tests passing, 0 warnings

## Dependencies

- `legalis-core` - Core types and traits
- `serde` / `serde_json` - Serialization
- `chrono` - Date/time handling
- `uuid` - Unique identifiers

## Future Phases

### Phase 2: Remaining 45 States
- Tier 1: 8 major jurisdictions (IL, PA, OH, GA, MA, WA, MI, NJ)
- Tier 2: 10 regional representatives
- Tier 3: 27 remaining states

### Phase 3: Professional Licensing
- Attorney licensing (UBE portability)
- Medical licensing (telemedicine interstate)
- Architect licensing (NCARB)

### Phase 4: Tax Variations
- State income tax comparison
- Sales tax nexus analysis
- Corporate tax comparison

### Phase 5: Legislative Tracking
- Cannabis legalization status
- Data privacy laws (CCPA, SHIELD Act, etc.)
- Right to repair laws

## Contributing

See the main [Legalis-RS repository](https://github.com/cool-japan/legalis) for contribution guidelines.

## License

MIT OR Apache-2.0

## Links

- [American Law Institute](https://www.ali.org/)
- [Uniform Law Commission](https://www.uniformlaws.org/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
