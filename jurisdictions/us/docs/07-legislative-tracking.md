# Legislative Tracking Guide

**Tracking policy adoption patterns across 51 US jurisdictions**

## Overview

States serve as **"laboratories of democracy"** - testing new policies that may spread to other states or influence federal law. Legalis-US tracks three major policy areas with significant interstate variation and active diffusion:

1. **Cannabis legalization** (recreational and medical)
2. **Privacy regulation** (comprehensive privacy laws)
3. **Right to repair** (consumer electronics repair laws)

These policy areas demonstrate:
- **Policy diffusion**: How policies spread from early adopters to other states
- **Regional patterns**: Geographic clustering of similar policies
- **Timeline analysis**: When states adopted policies (waves of adoption)
- **Federal-state tension**: Areas where state law leads federal law

---

## Part 1: Cannabis Legalization

### The Policy Landscape

**Federal Status**: Cannabis is Schedule I controlled substance (illegal federally)

**State Status**: States have legalized despite federal prohibition, creating federal-state conflict

**Three Categories:**
1. **Fully illegal**: No legal cannabis (medical or recreational)
2. **Medical only**: Legal for medical use with prescription
3. **Full legalization**: Legal for both medical and recreational use

### Check State Cannabis Status

```rust
use legalis_us::legislative::cannabis::{cannabis_status, CannabisPolicy};

// Check Colorado (first recreational state)
let co_status = cannabis_status("CO");

match co_status {
    CannabisPolicy::FullyLegal {
        medical_year,
        recreational_year,
        possession_limit,
        home_cultivation_allowed,
    } => {
        println!("Colorado: Fully legal");
        println!("Medical legalized: {}", medical_year); // 2000
        println!("Recreational legalized: {}", recreational_year); // 2012
        println!("Possession limit: {} oz", possession_limit); // 1 oz
        println!("Home cultivation: {}", home_cultivation_allowed); // true (6 plants)
    }
    _ => {}
}
```

### Adoption Timeline

**Medical Cannabis Wave:**
```rust
// Get states by adoption year
let medical_timeline = cannabis_adoption_timeline(CannabisType::Medical);

// Wave 1: California pioneers (1996)
// Wave 2: Western states follow (1998-2000)
// Wave 3: Northeast adopts (2010-2016)
// Wave 4: Conservative states adopt limited programs (2015+)

for (year, states) in medical_timeline {
    println!("{}: {:?}", year, states);
}

// 1996: CA (Proposition 215 - first state)
// 1998: AK, OR, WA
// 1999: ME
// 2000: CO, HI, NV
// ... (39 states total as of 2024)
```

**Recreational Cannabis Wave:**
```rust
let recreational_timeline = cannabis_adoption_timeline(CannabisType::Recreational);

// Wave 1: CO and WA (2012)
// Wave 2: Western states (2014-2016)
// Wave 3: East Coast (2016-2020)
// Wave 4: Recent adopters (2020-2024)

// 2012: CO, WA (first states via ballot initiative)
// 2014: AK, OR, DC
// 2016: CA, MA, ME, NV
// 2018: MI, VT
// 2020: AZ, MT, NJ, SD (SD later reversed)
// 2021: CT, NM, NY, VA
// 2022: DE, MD, MO
// 2023: MN, OH
// ... (24 states as of 2024)
```

### Regional Patterns

```rust
use legalis_us::legislative::cannabis::regional_adoption_map;

// Analyze regional patterns
let regional = regional_adoption_map();

// West Coast: 100% recreational legal (CA, OR, WA, NV, AK)
let west_coast = regional.get("West").unwrap();
println!("West: {:.0}% recreational legal", west_coast.recreational_percentage * 100.0);

// Northeast: High adoption (ME, VT, MA, RI, CT, NY, NJ)
let northeast = regional.get("Northeast").unwrap();
println!("Northeast: {:.0}% recreational legal", northeast.recreational_percentage * 100.0);

// South: Lowest adoption (most still illegal or medical-only)
let south = regional.get("South").unwrap();
println!("South: {:.0}% recreational legal", south.recreational_percentage * 100.0);
// Exceptions: Virginia (legal), Missouri (legal)

// Midwest: Mixed (MI, IL, MN legal; others medical or illegal)
```

### Policy Diffusion Analysis

**Early Adopters (2012-2016):**
```rust
let early_adopters = vec!["CO", "WA", "AK", "OR", "DC", "CA", "MA", "ME", "NV"];

// Characteristics of early adopters:
// - Western or coastal states
// - Ballot initiative process available
// - Progressive political lean
// - Prior medical cannabis programs
```

**Mechanisms of Diffusion:**
```rust
// Identify factors that predict adoption

let diffusion = CannabisDiffusionAnalysis::new()
    .with_neighboring_state_adoption(3) // 3 neighbors have legalized
    .with_ballot_initiative_available(true)
    .with_prior_medical_program(true)
    .with_fiscal_incentive(true); // Tax revenue potential

let prediction = diffusion.predict_adoption_likelihood();

println!("Likelihood of adoption: {:.0}%", prediction * 100.0);

// Strong predictors:
// - Neighboring states already legalized (network effects)
// - Ballot initiative process (bypasses legislature)
// - Existing medical program (normalization)
// - State fiscal stress (tax revenue appeal)
```

### Federal-State Conflict

```rust
// Federal Controlled Substances Act vs. State legalization

let conflict = FederalStateConflict::new()
    .with_federal_law("Cannabis is Schedule I controlled substance (illegal)")
    .with_state_law("Colorado: Recreational cannabis legal")
    .with_supremacy_clause_issue(true);

// Technically: Federal law prevails (Supremacy Clause)
// In practice: Federal government has not enforced against compliant state programs

// Cole Memo (2013-2018): DOJ deprioritized enforcement in legal states
// Current status: Federal non-enforcement policy (case-by-case)

println!("Federal enforcement risk: {}", conflict.enforcement_likelihood);
// Low for state-legal operations complying with state law
// High for interstate trafficking or other federal violations
```

### Tax Revenue Analysis

```rust
// States use cannabis taxes to fund various programs

let co_revenue = cannabis_tax_revenue("CO", 2023);

println!("Colorado 2023 cannabis tax revenue: ${:.1}M", co_revenue / 1_000_000.0);
// ~$430M annually in Colorado

// Revenue allocation (varies by state):
// - Education (common)
// - Drug treatment programs
// - Law enforcement
// - General fund
```

---

## Part 2: Privacy Regulation

### The Policy Landscape

**Federal Status**: No comprehensive federal privacy law (sector-specific laws only: HIPAA, GLBA, COPPA)

**State Status**: States filling federal gap with comprehensive privacy laws

**California leads**: CCPA/CPRA model spreading to other states

### Check State Privacy Law Status

```rust
use legalis_us::legislative::privacy::{privacy_law_status, PrivacyRegulation};

// California: First comprehensive privacy law
let ca_privacy = privacy_law_status("CA");

match ca_privacy {
    PrivacyRegulation::ComprehensiveLaw {
        statute_name,
        effective_date,
        key_provisions,
        enforcement_agency,
    } => {
        println!("California: {}", statute_name);
        // California Consumer Privacy Act (CCPA) / California Privacy Rights Act (CPRA)

        println!("Effective: {}", effective_date); // CCPA: 2020, CPRA: 2023

        for provision in key_provisions {
            println!("  - {}", provision);
        }
        // Right to know, right to delete, right to opt-out, right to correct

        println!("Enforced by: {}", enforcement_agency);
        // California Privacy Protection Agency (CPPA)
    }
    PrivacyRegulation::SectorSpecific => {
        println!("Sector-specific privacy laws only");
    }
    PrivacyRegulation::NoComprehensiveLaw => {
        println!("No comprehensive privacy law");
    }
}
```

### State Privacy Law Adoption

**Comprehensive Privacy Laws (as of 2024):**
```rust
let comprehensive_states = vec![
    ("CA", "CCPA/CPRA", 2020), // First state
    ("VA", "VCDPA", 2023),      // Virginia Consumer Data Protection Act
    ("CO", "CPA", 2023),        // Colorado Privacy Act
    ("CT", "CTDPA", 2023),      // Connecticut Data Privacy Act
    ("UT", "UCPA", 2023),       // Utah Consumer Privacy Act
    ("IA", "ICDPA", 2025),      // Iowa Consumer Data Protection Act
    ("IN", "ICDPA", 2026),      // Indiana Consumer Data Protection Act
    ("TN", "TIPA", 2025),       // Tennessee Information Protection Act
    ("MT", "MCDPA", 2024),      // Montana Consumer Data Privacy Act
    ("OR", "OCPA", 2024),       // Oregon Consumer Privacy Act
    // ... ~15 states total
];

for (state, statute, year) in comprehensive_states {
    println!("{}: {} (effective {})", state, statute, year);
}
```

### CCPA as Model Law

**California model features:**
```rust
let ccpa = ComprehensivePrivacyLaw::new("CA")
    .with_consumer_rights(vec![
        "Right to know what data is collected",
        "Right to delete personal information",
        "Right to opt-out of sale/sharing",
        "Right to correct inaccurate data",
        "Right to limit use of sensitive data",
    ])
    .with_business_obligations(vec![
        "Provide privacy notice",
        "Honor consumer requests",
        "Don't discriminate against opt-outs",
        "Data security requirements",
    ])
    .with_enforcement(vec![
        "Attorney General enforcement",
        "Private right of action (data breaches)",
        "California Privacy Protection Agency",
    ])
    .with_threshold(vec![
        "$25M+ annual revenue",
        "50,000+ consumers/households data",
        "50%+ revenue from selling data",
    ]);

// Most other states followed CCPA/CPRA structure
```

### State-by-State Variations

```rust
// Compare privacy laws across states

let states = vec!["CA", "VA", "CO", "CT", "UT"];

for state in states {
    let law = privacy_law_status(state);

    if let PrivacyRegulation::ComprehensiveLaw { key_provisions, .. } = law {
        println!("\n{} provisions:", state);

        // Right to delete
        if key_provisions.contains("right to delete") {
            println!("  ✅ Right to delete");
        }

        // Private right of action
        if key_provisions.contains("private right of action") {
            println!("  ✅ Private right of action (like CA)");
        } else {
            println!("  ❌ No private right of action (AG only)");
        }
        // Most states NO private right (VA, CO, CT, UT)
        // Only CA has broad private right of action

        // Opt-in vs. opt-out
        if key_provisions.contains("opt-in for sensitive data") {
            println!("  ✅ Opt-in for sensitive data (stricter)");
        } else {
            println!("  ⚠️ Opt-out model (default processing allowed)");
        }
    }
}

// Key difference: CA has private right of action (creates liability)
// Other states: AG enforcement only (less liability risk)
```

### Federal Preemption Debate

```rust
// Will federal privacy law preempt state laws?

let preemption_scenarios = vec![
    ("Federal floor", "States can exceed federal minimum", false),
    ("Complete preemption", "Federal law replaces all state laws", true),
    ("Hybrid", "Federal law for some issues, state for others", false),
];

// Current status: No federal comprehensive privacy law
// Proposed: American Data Privacy and Protection Act (ADPPA)
// Industry prefers: Single federal standard (vs. 50 state laws)
// Privacy advocates prefer: State experimentation
```

### Practical Impact

```rust
// Multi-state compliance burden

let business_analysis = PrivacyComplianceAnalysis::new()
    .operates_in_states(vec!["CA", "VA", "CO", "CT", "UT"])
    .with_revenue(100_000_000.0) // $100M revenue
    .with_consumer_count(1_000_000);

let compliance_obligations = business_analysis.determine_obligations();

for (state, obligations) in compliance_obligations {
    println!("\n{} compliance:", state);
    for obligation in obligations {
        println!("  - {}", obligation);
    }
}

// Result: Must comply with 5+ different state laws
// Businesses often comply with strictest law (CCPA) nationwide
```

---

## Part 3: Right to Repair

### The Policy Landscape

**Issue**: Manufacturers restrict access to repair parts, tools, and documentation

**Consumer advocates**: Support right to repair (reduce e-waste, save money)

**Manufacturers**: Oppose (IP concerns, safety, revenue from authorized repairs)

### Check State Right to Repair Status

```rust
use legalis_us::legislative::right_to_repair::{repair_law_status, RepairPolicy};

// New York: First comprehensive electronics right to repair law
let ny_repair = repair_law_status("NY");

match ny_repair {
    RepairPolicy::ComprehensiveLaw {
        effective_date,
        covered_products,
        manufacturer_obligations,
    } => {
        println!("New York: Digital Fair Repair Act");
        println!("Effective: {}", effective_date); // 2023

        println!("\nCovered products:");
        for product in covered_products {
            println!("  - {}", product);
        }
        // Electronics, appliances, etc.

        println!("\nManufacturer must provide:");
        for obligation in manufacturer_obligations {
            println!("  - {}", obligation);
        }
        // Parts, tools, documentation to independent repair shops
    }
    RepairPolicy::SectorSpecific { sectors } => {
        println!("Sector-specific laws only: {:?}", sectors);
        // Many states have auto repair laws
    }
    RepairPolicy::NoBroadLaw => {
        println!("No broad right to repair law");
    }
}
```

### Sector-Specific vs. Comprehensive Laws

**Automobiles (widespread):**
```rust
// Auto right to repair: ~30 states

let auto_repair_states = repair_laws_by_sector("automotive");

// Massachusetts: Strongest auto repair law (2012, expanded 2020)
// Requires manufacturers provide diagnostic tools to independent shops

for state in auto_repair_states {
    println!("{}: Has auto right to repair law", state);
}

// Why auto succeeded: Strong independent repair shop lobby
```

**Farm Equipment:**
```rust
// Farmers need to repair tractors (increasingly computerized)

let farm_equipment_states = repair_laws_by_sector("agricultural");

// Colorado: First ag equipment repair law (2023)
// Nebraska, Minnesota: Considering similar laws

for state in farm_equipment_states {
    println!("{}: Has farm equipment repair law", state);
}

// Issue: John Deere software locks prevent farmer repairs
```

**Consumer Electronics (emerging):**
```rust
// Smartphones, laptops, appliances

let electronics_states = repair_laws_by_sector("electronics");

// New York: First state (2022, effective 2023)
// California: Passed 2023, effective 2024
// Minnesota: Passed 2023

for state in electronics_states {
    println!("{}: Has electronics repair law", state);
}

// Strongest industry opposition here (Apple, Microsoft, etc.)
```

### Adoption Patterns

```rust
// Right to repair diffusion slower than cannabis/privacy

let adoption_timeline = repair_adoption_timeline();

// 2012: Massachusetts (auto only)
// 2020: Massachusetts expands auto law
// 2022: New York (first electronics law)
// 2023: California, Minnesota, Colorado (various sectors)
// 2024: Pending in ~20 state legislatures

// Why slower adoption?
// - Strong industry opposition (tech lobby)
// - IP/safety concerns
// - Less consumer salience (vs. cannabis/privacy)
// - No "California effect" yet (too new)
```

### Industry Opposition

```rust
// Manufacturer arguments against right to repair

let opposition_arguments = vec![
    "Safety concerns (untrained repairs)",
    "Intellectual property protection",
    "Cybersecurity risks",
    "Loss of quality control",
    "Reduced innovation incentive",
];

// Effectiveness: Industry successfully blocked laws in many states
// But: Tide turning as e-waste concerns mount
```

### Federal Action

```rust
// FTC support for right to repair (2021 policy statement)

let ftc_position = FederalAgencyPosition::new("FTC")
    .with_position("Support right to repair")
    .with_reasoning("Manufacturers use repair restrictions anticompetitively")
    .with_enforcement_actions(vec![
        "Warning letters to manufacturers",
        "Challenging warranty restrictions",
    ]);

// But: No federal right to repair law yet
// States leading, federal following
```

---

## Policy Diffusion Framework

### Identifying Patterns

```rust
use legalis_us::legislative::diffusion::{PolicyDiffusionAnalysis, DiffusionPattern};

// Analyze how policies spread between states

let cannabis_diffusion = PolicyDiffusionAnalysis::new("Cannabis Legalization")
    .with_adoption_timeline(cannabis_timeline)
    .with_geographic_data(state_locations);

let pattern = cannabis_diffusion.identify_pattern();

match pattern {
    DiffusionPattern::RegionalClustering { regions } => {
        println!("Regional clustering detected:");
        for region in regions {
            println!("  - {}: {:.0}% adopted", region.name, region.adoption_rate * 100.0);
        }
    }
    DiffusionPattern::NeighborEffect { correlation } => {
        println!("Neighbor effect: {:.2} correlation", correlation);
        // States more likely to adopt if neighbors have
    }
    DiffusionPattern::LeaderFollower { leader, followers } => {
        println!("Leader: {}", leader);
        println!("Followers: {:?}", followers);
        // California-led privacy diffusion
    }
    _ => {}
}
```

### Predictive Modeling

```rust
// Predict which states will adopt next

let prediction = PolicyDiffusionAnalysis::new("Comprehensive Privacy Law")
    .with_current_adopters(vec!["CA", "VA", "CO", "CT", "UT", "IA", "IN", "TN"])
    .predict_next_adopters(5); // Top 5 most likely

for (state, probability) in prediction {
    println!("{}: {:.0}% likelihood of adoption", state, probability * 100.0);
}

// Factors:
// - Political lean (privacy: blue states; right to repair: bipartisan)
// - Neighboring state adoption
// - Ballot initiative availability (cannabis)
// - Industry presence (privacy: tech states; repair: farm states)
```

---

## Best Practices

### 1. Track Multiple Jurisdictions

```rust
// For businesses operating nationwide

let jurisdictions = vec!["CA", "NY", "TX", "FL", "IL"];

for state in jurisdictions {
    println!("\n{} Legislative Summary:", state);

    // Cannabis
    let cannabis = cannabis_status(state);
    println!("  Cannabis: {:?}", cannabis);

    // Privacy
    let privacy = privacy_law_status(state);
    println!("  Privacy: {:?}", privacy);

    // Repair
    let repair = repair_law_status(state);
    println!("  Right to Repair: {:?}", repair);
}
```

### 2. Monitor Pending Legislation

```rust
// Check for bills pending in legislature

let pending = pending_legislation("WA", "comprehensive privacy law");

for bill in pending {
    println!("Bill {}: {}", bill.number, bill.title);
    println!("Status: {}", bill.status); // Committee, Floor, Passed, etc.
    println!("Sponsor: {}", bill.sponsor);
    println!("Effective date: {:?}", bill.effective_date);
}
```

### 3. Analyze Diffusion Patterns

```rust
// Learn from how policies spread

let diffusion = PolicyDiffusionAnalysis::new("Cannabis Legalization");

// Which states were early adopters?
let early = diffusion.early_adopters(5);
println!("Early adopters: {:?}", early);
// CO, WA (first movers)

// Which states followed neighbors?
let neighbor_effect = diffusion.neighbor_correlation();
println!("Neighbor effect: {:.2}", neighbor_effect);
// 0.65+ correlation (strong neighbor effect)

// Regional leaders
let leaders = diffusion.regional_leaders();
for (region, leader) in leaders {
    println!("{} leader: {}", region, leader);
}
// West: CA, Northeast: MA, etc.
```

---

## Future Predictions

### Cannabis

**Likely trajectory:**
- More states adopt recreational (reaching ~35-40 states by 2030)
- Federal rescheduling (Schedule III) or decriminalization likely
- Interstate commerce remains prohibited until federal legalization

```rust
// Predict federal action timeline
let federal_prediction = predict_federal_cannabis_reform();

println!("Probability of federal reform by 2030: {:.0}%", federal_prediction * 100.0);
// High probability of at least rescheduling
```

### Privacy

**Likely trajectory:**
- Federal comprehensive privacy law eventually passes
- Preemption debate: Will federal law supersede state laws?
- Industry prefers federal preemption; states/advocates oppose

```rust
// Track federal privacy legislation
let federal_privacy = track_federal_bill("American Data Privacy and Protection Act");

if federal_privacy.status == "Pending" {
    println!("Federal privacy law under consideration");
    println!("Preemption clause: {:?}", federal_privacy.preemption_provision);
}
```

### Right to Repair

**Likely trajectory:**
- Slower adoption than cannabis/privacy (industry resistance)
- Sector-by-sector approach (ag equipment before smartphones)
- Potential federal FTC regulations

```rust
// Predict state adoption
let repair_prediction = predict_state_adoption("right to repair");

println!("States likely to adopt by 2027: {:?}", repair_prediction);
// Farm states for ag equipment
// Consumer protection states for electronics
```

---

## Comparative Analysis

```rust
// Compare diffusion across all three policies

let policies = vec!["Cannabis", "Privacy", "Right to Repair"];

for policy in policies {
    let diffusion = PolicyDiffusionAnalysis::new(policy);

    let stats = diffusion.summary_statistics();

    println!("\n{}:", policy);
    println!("  States adopted: {}", stats.total_adopters);
    println!("  Years since first: {}", stats.years_elapsed);
    println!("  Adoption rate: {:.1}/year", stats.adoption_rate);
    println!("  Regional clustering: {:.0}%", stats.clustering_coefficient * 100.0);
}

// Cannabis: Fastest diffusion (24 states in 12 years)
// Privacy: Moderate diffusion (15 states in 4 years)
// Right to Repair: Slowest diffusion (3 states in 2 years)
```

---

## Further Reading

**Cannabis:**
- Caulkins et al., *Marijuana Legalization* (2016)
- State cannabis regulatory frameworks (CA, CO, WA)

**Privacy:**
- California Consumer Privacy Act (CCPA/CPRA)
- Virginia Consumer Data Protection Act (VCDPA)
- IAPP resources on state privacy laws

**Right to Repair:**
- iFixit advocacy materials
- Repair Association resources
- State legislative tracking

---

**Documentation Complete**: All 7 guides created for Legalis-US
- ✅ [Getting Started](00-getting-started.md)
- ✅ [Choice of Law](01-choice-of-law.md)
- ✅ [State Comparison](02-state-comparison.md)
- ✅ [Professional Licensing](03-professional-licensing.md)
- ✅ [Tax Law](04-tax-law.md)
- ✅ [Federal Preemption](05-federal-preemption.md)
- ✅ [Uniform Acts](06-uniform-acts.md)
- ✅ [Legislative Tracking](07-legislative-tracking.md)
