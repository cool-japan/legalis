# Choice of Law Guide

**Determining which state's law applies to multi-state disputes**

## Overview

When a legal dispute involves multiple states (e.g., car accident in Texas involving California and New York residents), courts must determine **which state's substantive law applies**. This is the "choice of law" problem.

Different states use different approaches to resolve this problem. Legalis-US implements all 5 major approaches used across US jurisdictions.

---

## The Five Approaches

### 1. Restatement (First) - Traditional (6 states)

**Mechanical territorial rules:**
- **Torts**: Law of place where injury occurred (*lex loci delicti*)
- **Contracts**: Law of place where contract was made (*lex loci contractus*)

**States using this approach:**
Alabama, Maryland, Kansas, New Mexico, North Carolina, Wyoming

```rust
use legalis_us::choice_of_law::restatement_first::{RestatementFirst, RestatementFirstRule};

let rf = RestatementFirst::new();

// Tort: Apply law of where injury occurred
let tort_result = rf.determine_law_for_tort(
    "TX", // Place of injury
    "CA", // Place of conduct (irrelevant under Restatement First)
);
assert_eq!(tort_result.governing_law, "TX"); // Texas law applies
assert_eq!(tort_result.rule, RestatementFirstRule::LexLociDelicti);

// Contract: Apply law of where contract was made
let contract_result = rf.determine_law_for_contract(
    Some("NY"), // Place of execution
    Some("CA"), // Place of negotiation
    None,       // Place of performance
);
assert_eq!(contract_result.governing_law, "NY"); // New York law applies
```

**Advantages:**
- ✅ Simple and predictable
- ✅ Easy to apply
- ✅ Clear ex ante (parties know governing law)

**Disadvantages:**
- ❌ Ignores relevant policies of other states
- ❌ Can produce arbitrary results
- ❌ Doesn't consider party contacts or interests

---

### 2. Restatement (Second) - Modern (44 states) ⭐ **MAJORITY**

**Most Significant Relationship Test:**

Considers multiple factors to find the state with the "most significant relationship" to the dispute.

**For Torts (§ 145):**
1. Place of injury (weight: 3.0)
2. Place of conduct causing injury (weight: 2.0)
3. Domicile/residence of parties (weight: 1.5)
4. Place where relationship is centered (weight: 2.5)

**For Contracts (§ 188):**
1. Place of contracting (weight: 2.0)
2. Place of negotiation (weight: 1.5)
3. Place of performance (weight: 2.5) - often most important
4. Location of subject matter (weight: 2.0)
5. Domicile/residence of parties (weight: 1.0)

**Plus § 6 policy factors:**
1. Needs of interstate system
2. Relevant policies of forum
3. Relevant policies of other states
4. Protection of justified expectations
5. Basic policies underlying particular field
6. Certainty, predictability, uniformity
7. Ease of determination and application

```rust
use legalis_us::choice_of_law::restatement_second::RestatementSecond;
use legalis_us::choice_of_law::factors::ContactingFactor;

let rs = RestatementSecond::new();

// Car accident: NY driver hits CA driver in Texas
let tort_result = rs.analyze_tort(vec![
    (ContactingFactor::PlaceOfInjury, "TX", 3.0),        // Accident in Texas
    (ContactingFactor::PlaceOfConduct, "TX", 2.0),       // Conduct in Texas
    (ContactingFactor::DomicileOfPlaintiff, "CA", 1.5),  // Plaintiff from CA
    (ContactingFactor::DomicileOfDefendant, "NY", 1.5),  // Defendant from NY
]);

// Texas has highest score (3.0 + 2.0 = 5.0)
assert_eq!(tort_result.recommended_law, "TX");
println!("Reasoning: {}", tort_result.reasoning);
```

**Used by 44 states** including: California, Texas, Florida, Illinois, Pennsylvania, Ohio, Georgia, New Jersey, Virginia, Washington, Massachusetts, Indiana, Arizona, Tennessee, Missouri, Maryland, Wisconsin, Colorado, Oregon, Connecticut

---

### 3. Interest Analysis - California/New Jersey

**Governmental Interest Analysis** (Professor Brainerd Currie):

1. **Identify** each state's interest in applying its law
2. **Classify** conflict type:
   - **False conflict**: Only one state has interest → Apply that state's law
   - **True conflict**: Multiple states have interests → Apply forum law
   - **No conflict**: No state has interest → Apply forum law

```rust
use legalis_us::choice_of_law::analyzer::USChoiceOfLawAnalyzer;
use legalis_us::choice_of_law::factors::USChoiceOfLawFactors;

// Example: CA plaintiff vs. CA defendant injured in NV
let factors = USChoiceOfLawFactors::new()
    .with_factor(ContactingFactor::PlaceOfInjury, "NV")
    .with_factor(ContactingFactor::DomicileOfPlaintiff, "CA")
    .with_factor(ContactingFactor::DomicileOfDefendant, "CA")
    .with_state_interest("CA", "Compensate CA resident, deter CA resident")
    .with_state_interest("NV", "Regulate conduct in NV")
    .with_forum_state("CA");

let analyzer = USChoiceOfLawAnalyzer::new();
let result = analyzer.analyze_for_state("CA", factors)?;

// FALSE CONFLICT: Both parties are CA residents
// Nevada has minimal interest (just place of injury)
// California has strong interest (both parties)
assert_eq!(result.recommended_law, "CA");
```

**True Conflict Example:**
- NY plaintiff vs. CA defendant, injury in CA
- NY interest: Compensate NY resident
- CA interest: Protect CA resident from liability
- Resolution: Forum law applies (if forum is NY → NY law; if CA → CA law)

**States using this:** California, New Jersey, and a few others

---

### 4. Better Law - Minnesota/Wisconsin

**Comparative Impairment / Better Law:**

When faced with true conflict, apply the law that:
- Better advances the policies of both states
- Is more modern or progressive
- Is better-reasoned

```rust
// Minnesota/Wisconsin approach - requires substantive law analysis
let result = analyzer.analyze_for_state("MN", factors)?;

// Note: Full implementation requires comparing actual state laws
// for quality, modernity, and policy advancement
```

**Note**: This approach requires substantive evaluation of competing laws, which is complex to automate. Legalis-US provides a simplified implementation.

**States using this:** Minnesota, Wisconsin

---

### 5. Combined Modern - New York

**Neumeier Rules (hybrid approach):**

Combines interest analysis with specific rules for different scenarios.

**For tort cases involving loss-distribution rules (e.g., guest statutes, wrongful death damages):**

1. **Both parties same domicile**: Apply common domicile law
2. **Different domiciles, conduct in one party's domicile**: Apply conduct-state law
3. **Different domiciles, conduct in third state**:
   - If conflicting laws, apply forum law (NY)
   - Unless displacing forum law would "advance relevant substantive law purposes"

```rust
// New York's sophisticated approach
let result = analyzer.analyze_for_state("NY", factors)?;

// NY considers both Restatement (Second) factors
// AND governmental interests AND specific Neumeier rules
```

**State using this:** New York (most sophisticated approach)

---

## Practical Example: Multi-State Car Accident

**Scenario:**
- **Plaintiff**: California resident
- **Defendant**: Texas resident
- **Accident location**: Nevada
- **Forum**: California state court

**Relevant Laws:**
- **California**: Pure comparative negligence (plaintiff can recover even if 99% at fault)
- **Texas**: Modified comparative negligence 51% bar (no recovery if >50% at fault)
- **Nevada**: Modified comparative negligence 51% bar

### Step 1: Identify Contacts

```rust
let factors = USChoiceOfLawFactors::new()
    .with_factor(ContactingFactor::PlaceOfInjury, "NV")
    .with_factor(ContactingFactor::PlaceOfConduct, "NV")
    .with_factor(ContactingFactor::DomicileOfPlaintiff, "CA")
    .with_factor(ContactingFactor::DomicileOfDefendant, "TX")
    .with_forum_state("CA");
```

### Step 2: Apply California's Approach (Restatement Second + Interest Analysis)

```rust
let analyzer = USChoiceOfLawAnalyzer::new();
let result = analyzer.analyze_for_state("CA", factors)?;

println!("Recommended law: {}", result.recommended_law);
println!("Confidence: {:.0}%", result.confidence * 100.0);
println!("Approach used: {:?}", result.approach);
println!("\nReasoning:\n{}", result.reasoning);
```

**Analysis:**
1. **Nevada contacts**: Place of injury (3.0) + Place of conduct (2.0) = **5.0 points**
2. **California contacts**: Plaintiff domicile (1.5) = **1.5 points**
3. **Texas contacts**: Defendant domicile (1.5) = **1.5 points**

**Interest Analysis:**
- **Nevada interest**: Regulate conduct on its highways (regulatory)
- **California interest**: Compensate CA resident (compensatory) - STRONG
- **Texas interest**: Protect TX resident from excessive liability - STRONG

**Result**: Likely **Nevada law** applies under Restatement (Second) most significant relationship test (highest points). However, California court might find California has stronger interest in compensating its resident → TRUE CONFLICT → Forum law (CA) applies.

---

## Choosing the Right Approach

### Automatic Detection

The library can automatically detect which approach to use:

```rust
let analyzer = USChoiceOfLawAnalyzer::new();

// Automatically uses forum state's approach
let result = analyzer.analyze_for_state("CA", factors)?;
println!("Approach: {:?}", result.approach);
// Output: ChoiceOfLawApproach::RestatementSecond (CA uses this)
```

### Manual Selection

```rust
// Force specific approach
let rs = RestatementSecond::new();
let result = rs.analyze_tort(contact_factors);
```

---

## Best Practices

### 1. Identify All Contacts Early

```rust
let factors = USChoiceOfLawFactors::new()
    .with_factor(ContactingFactor::PlaceOfInjury, "TX")
    .with_factor(ContactingFactor::PlaceOfConduct, "CA")
    .with_factor(ContactingFactor::DomicileOfPlaintiff, "NY")
    .with_factor(ContactingFactor::DomicileOfDefendant, "FL")
    .with_factor(ContactingFactor::PlaceWhereRelationshipCentered, "CA")
    .with_forum_state("NY");
```

### 2. Consider State Interests

```rust
factors = factors
    .with_state_interest("CA", "Regulate conduct on CA roads")
    .with_state_interest("NY", "Compensate NY residents")
    .with_state_interest("TX", "Limit liability for out-of-state injuries");
```

### 3. Check for False Conflicts

If all parties share common domicile, often a false conflict:

```rust
if plaintiff_domicile == defendant_domicile {
    // Likely false conflict - apply common domicile law
    return Ok(plaintiff_domicile);
}
```

### 4. Review Confidence Scores

```rust
if result.confidence < 0.6 {
    println!("⚠️ Low confidence - conflict may require detailed analysis");
}
```

---

## Common Patterns

### Pattern 1: Common Domicile

**Both parties from same state, injury elsewhere:**
```
Plaintiff: California
Defendant: California
Injury: Nevada
→ Apply California law (false conflict)
```

### Pattern 2: Conduct State

**Injury where defendant acted:**
```
Plaintiff: New York
Defendant: Texas
Injury: Texas (where defendant drove)
→ Apply Texas law (conduct state has strong interest)
```

### Pattern 3: Split Domiciles, Third State Injury

**Most complex scenario:**
```
Plaintiff: California
Defendant: Texas
Injury: Nevada
→ Analyze under forum's approach
   - Restatement (Second): Count points
   - Interest Analysis: Identify true vs. false conflict
```

---

## State-by-State Approach Map

| Approach | States | Count |
|----------|--------|-------|
| **Restatement (Second)** | CA, TX, FL, IL, PA, OH, GA, NJ, VA, WA, MA, IN, AZ, TN, MO, MD, WI, CO, OR, CT, SC, IA, MS, UT, NV, AR, KS, LA, KY, HI, ME, RI, ID, WV, NH, MT, SD, VT, DE, AK, ND, DC, OK, WY | **44** |
| **Restatement (First)** | AL, MD, KS, NM, NC, WY | **6** |
| **Interest Analysis** | CA, NJ | **2** |
| **Better Law** | MN, WI | **2** |
| **Combined Modern** | NY | **1** |

**Note**: Some states (CA, NJ, WI) appear in multiple categories because they use hybrid approaches.

---

## Further Reading

- Restatement (First) of Conflict of Laws (1934)
- Restatement (Second) of Conflict of Laws (1971)
- Brainerd Currie, *Selected Essays on the Conflict of Laws* (1963)
- *Neumeier v. Kuehner*, 31 N.Y.2d 121 (1972) - New York's approach

---

**Next**: [State Comparison Guide](02-state-comparison.md)
