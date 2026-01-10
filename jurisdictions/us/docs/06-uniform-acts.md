# Uniform Acts Guide

**Tracking UCC and UPA adoption across 51 US jurisdictions**

## Overview

**Uniform Acts** are model laws drafted by the **Uniform Law Commission (ULC)** to harmonize state laws across the United States. While federal law applies uniformly, most private law (contracts, sales, partnerships) remains state law, creating potential for 51 different legal regimes.

Legalis-US tracks adoption of major uniform acts:
- **UCC (Uniform Commercial Code)** - Commercial transactions
- **UPA (Uniform Partnership Act)** - Partnership law

---

## Part 1: Uniform Commercial Code (UCC)

### What is the UCC?

The **Uniform Commercial Code** is the most successful uniform law in US history, governing commercial transactions including:

- **Article 1**: General Provisions
- **Article 2**: Sales of Goods
- **Article 2A**: Leases of Goods
- **Article 3**: Negotiable Instruments
- **Article 4**: Bank Deposits
- **Article 4A**: Funds Transfers
- **Article 5**: Letters of Credit
- **Article 6**: Bulk Transfers (mostly repealed)
- **Article 7**: Documents of Title
- **Article 8**: Investment Securities
- **Article 9**: Secured Transactions

**Adoption**: All 50 states + DC + territories have adopted the UCC (with variations)

### Check UCC Adoption Status

```rust
use legalis_us::uniform_acts::ucc::{ucc_adoption_status, UCCArticle};

// Check if state has adopted UCC Article 2 (Sales)
let ca_article2 = ucc_adoption_status("CA", UCCArticle::Article2);

match ca_article2 {
    UCCAdoptionStatus::Adopted {
        year,
        version,
        notable_variations,
    } => {
        println!("California adopted UCC Article 2 in {}", year);
        println!("Version: {:?}", version); // 2003 amendments, etc.

        for variation in notable_variations {
            println!("⚠️ CA variation: {}", variation);
        }
    }
    UCCAdoptionStatus::NotAdopted => {
        println!("Not adopted (extremely rare for UCC)");
    }
}
```

### UCC Article 2: Sales of Goods

**Scope**: Transactions in **goods** (movable personal property)

**Not covered**: Services, real estate, intellectual property

```rust
use legalis_us::uniform_acts::ucc::is_transaction_covered_by_ucc;

// Sale of laptop: YES
assert!(is_transaction_covered_by_ucc("Sale of laptop computer"));

// Software license: MAYBE (depends on form)
let software = is_transaction_covered_by_ucc("Software license");
// Courts split on software (goods vs. license)

// Sale of house: NO
assert!(!is_transaction_covered_by_ucc("Sale of real property"));

// Legal services: NO
assert!(!is_transaction_covered_by_ucc("Attorney fees"));
```

**Key Provisions:**

**§2-207: Battle of the Forms**
```rust
// Classic commercial law problem: conflicting forms

// Buyer sends purchase order with terms
// Seller sends acknowledgment with different terms
// Which terms control?

let battle = UCCBattleOfForms::new()
    .with_buyer_form("Purchase order with arbitration clause")
    .with_seller_form("Acknowledgment with no-arbitration clause")
    .with_conflicting_terms(vec!["arbitration"])
    .with_performance_occurred(true);

let result = battle.analyze();

// Under UCC §2-207:
// Additional terms in acceptance are proposals
// Between merchants: additional terms become part of contract UNLESS:
//   (a) Offer expressly limits acceptance
//   (b) Material alteration
//   (c) Notification of objection

println!("Governing terms: {:?}", result.governing_terms);
// Likely: Arbitration clause knocked out, default UCC rules apply
```

**§2-314: Implied Warranty of Merchantability**
```rust
// Automatic warranty when seller is a "merchant"

let warranty = ImpliedWarrantyAnalysis::new()
    .with_seller_type("Merchant") // vs. casual seller
    .with_goods_type("New laptop")
    .with_defect("Screen doesn't work");

if warranty.is_merchantable() {
    println!("✅ Breach of implied warranty of merchantability");
    println!("Goods must be fit for ordinary purposes");
} else {
    println!("❌ No breach (or warranty disclaimed)");
}

// Disclaimer must be conspicuous: "AS IS" in bold
```

**§2-615: Excuse by Failure of Presupposed Conditions (Force Majeure)**
```rust
// COVID-19 and UCC §2-615

let excuse = UCCExcuseAnalysis::new()
    .with_contract("Supply 10,000 masks per month")
    .with_event("COVID-19 pandemic causes supply shortage")
    .with_foreseeable_at_contracting(false)
    .with_basic_assumption(true); // Assumption of normal supply

let result = excuse.analyze();

// §2-615 requirements:
// (a) Occurrence of contingency whose non-occurrence was basic assumption
// (b) Performance made impracticable
// (c) Not foreseeable at time of contracting

if result.is_excused {
    println!("✅ Performance excused under UCC §2-615");
} else {
    println!("❌ No excuse - must perform or breach");
}
```

### UCC Article 2A: Leases

**Scope**: Leases of goods (not sales)

**Adoption**: All 50 states + DC have adopted Article 2A

```rust
// Equipment lease analysis
let lease = ucc_adoption_status("TX", UCCArticle::Article2A);

// Article 2A mirrors Article 2 for leases
// Finance leases have special rules (§2A-209)
```

### UCC Article 9: Secured Transactions

**Scope**: Security interests in personal property (collateral for loans)

**Critical for**: Banking, commercial lending, secured creditors

```rust
use legalis_us::uniform_acts::ucc::{Article9Analysis, CollateralType};

// Bank lends $100k to business, secured by equipment
let security_interest = Article9Analysis::new()
    .with_collateral_type(CollateralType::Equipment)
    .with_collateral_value(100_000.0)
    .with_loan_amount(100_000.0)
    .with_financing_statement_filed(true) // UCC-1 filing
    .with_filing_state("DE"); // Where debtor is located

// Perfection rules (§9-310 to §9-316)
if security_interest.is_perfected() {
    println!("✅ Security interest perfected");
    println!("Priority over unsecured creditors and later secured parties");
} else {
    println!("❌ Unperfected - vulnerable to other creditors");
}

// Priority rules (§9-317 to §9-339)
let priority = security_interest.determine_priority(vec![
    "First filed security interest",
    "Purchase money security interest",
    "Lien creditor",
]);

println!("Priority ranking: {:?}", priority);
```

**2022 Amendments to Article 9:**
```rust
// Check if state adopted 2022 amendments
let article9_status = ucc_adoption_status("CA", UCCArticle::Article9);

match article9_status {
    UCCAdoptionStatus::Adopted { version, .. } => {
        if version.contains("2022") {
            println!("✅ Adopted 2022 amendments");
            // Updated rules for digital assets, control agreements
        } else {
            println!("⚠️ Using older version (pre-2022)");
        }
    }
    _ => {}
}
```

### State Variations in UCC

Despite "uniform" law, states have adopted non-uniform variations:

```rust
// Get state-specific UCC variations
let variations = ucc_state_variations("LA");

// Louisiana (Civil Law state) has significant variations:
for variation in variations {
    println!("LA variation: {}", variation);
}

// Common variations:
// - Louisiana: Extensive modifications to fit Civil Law tradition
// - California: Consumer protection additions
// - New York: Financial markets modifications
```

**Louisiana Special Case:**
```rust
// Louisiana uses Civil Law, not Common Law
let la_ucc = ucc_adoption_status("LA", UCCArticle::Article2);

// Louisiana calls it "Louisiana Commercial Code" not UCC
// Terminology differences:
// - "Stipulation" not "warranty"
// - "Redhibition" not "revocation"
// - Civil Code provisions supplement UCC
```

---

## Part 2: Uniform Partnership Act (UPA)

### What is the UPA?

The **Uniform Partnership Act** governs **general partnerships** (not corporations or LLCs).

**Two Versions:**
1. **UPA (1914)**: Original version ("aggregate theory")
2. **RUPA (1997)**: Revised UPA ("entity theory")

### Check UPA Adoption

```rust
use legalis_us::uniform_acts::upa::{upa_adoption_status, UPAVersion};

// Check which version state uses
let ny_upa = upa_adoption_status("NY");

match ny_upa {
    UPAAdoptionStatus::RUPA {
        adoption_year,
        notable_provisions,
    } => {
        println!("New York adopted RUPA (1997) in {}", adoption_year);
        println!("Uses entity theory of partnerships");
    }
    UPAAdoptionStatus::UPA1914 => {
        println!("New York still uses UPA (1914)");
        println!("Uses aggregate theory of partnerships");
    }
    UPAAdoptionStatus::CustomStatute => {
        println!("Custom partnership statute");
    }
}
```

### RUPA Adoption Map

```rust
// Get nationwide RUPA adoption status
let rupa_states = vec![];
let upa1914_states = vec![];

for state_code in all_us_states() {
    match upa_adoption_status(state_code) {
        UPAAdoptionStatus::RUPA { .. } => rupa_states.push(state_code),
        UPAAdoptionStatus::UPA1914 => upa1914_states.push(state_code),
        _ => {}
    }
}

println!("RUPA (1997) states: {} jurisdictions", rupa_states.len());
println!("UPA (1914) states: {} jurisdictions", upa1914_states.len());

// As of 2024: ~40 states have adopted RUPA
```

### Key Differences: UPA vs. RUPA

**1. Entity vs. Aggregate Theory**

```rust
// UPA (1914): Aggregate theory
// Partnership is NOT separate legal entity
// Partners own partnership property as "tenants in partnership"

// RUPA (1997): Entity theory
// Partnership IS separate legal entity
// Partnership owns its own property
```

**2. Partner Liability**

```rust
// Under both: Partners have joint and several liability
// But procedural differences:

// RUPA: Must exhaust partnership assets first (§307)
let rupa_liability = RUPAPartnerLiability::new()
    .with_partnership_assets(50_000.0)
    .with_judgment_amount(100_000.0)
    .with_exhaustion_attempted(true);

if rupa_liability.can_pursue_partner_assets() {
    println!("✅ Can pursue partner assets (after exhaustion)");
    println!("Partner liable for remaining: $50,000");
}
```

**3. Partnership Dissolution**

```rust
// UPA (1914): Dissolution = end of partnership
// Any partner can force dissolution by withdrawing

// RUPA (1997): Dissociation ≠ Dissolution
// Partner can leave without dissolving partnership

let dissociation = RUPADissociation::new()
    .with_partner_withdrawal("Partner A withdraws")
    .with_remaining_partners(3)
    .with_partnership_term("At-will partnership");

let result = dissociation.analyze();

if result.causes_dissolution {
    println!("Partnership dissolves (all partners must be paid out)");
} else {
    println!("Partnership continues (buyout departing partner)");
}

// RUPA default: Partnership continues
```

**4. Fiduciary Duties**

```rust
// RUPA explicitly codifies fiduciary duties (§404)

let fiduciary = RUPAFiduciaryDuties::new()
    .with_duty_of_loyalty(vec![
        "No self-dealing",
        "No usurping partnership opportunities",
        "No competing with partnership",
    ])
    .with_duty_of_care("Gross negligence standard");

// Can partners modify fiduciary duties?
// RUPA §103(b): Can modify SOME duties but not eliminate entirely
```

---

## Part 3: Other Uniform Acts

### Uniform Limited Partnership Act (ULPA)

```rust
// Limited partnerships: General partners + limited partners
// Limited partners have limited liability (like shareholders)

let ulpa = uniform_act_status("CA", "ULPA");

// Most states adopted ULPA (2001) or ULPA (2013)
```

### Uniform Limited Liability Company Act (ULLCA)

```rust
// LLCs: Hybrid entity (partnership tax treatment + limited liability)

let ullca = uniform_act_status("DE", "ULLCA");

// Note: Delaware has its own LLC statute (not ULLCA)
// Delaware LLC Act is most popular (50%+ of LLCs formed there)
```

### Uniform Trust Code (UTC)

```rust
// Governs trusts (not covered by UCC/UPA)

let utc = uniform_act_status("NV", "UTC");

// ~30 states have adopted UTC
```

### Uniform Probate Code (UPC)

```rust
// Governs wills, estates, intestate succession

let upc = uniform_act_status("CA", "UPC");

// ~20 states have adopted UPC
// Many states have own probate statutes
```

---

## Practical Use Cases

### Use Case 1: Multi-State Contract Dispute

**Problem**: Contract for sale of goods between CA buyer and TX seller. Which UCC version applies?

```rust
// Check UCC Article 2 versions
let ca_ucc = ucc_adoption_status("CA", UCCArticle::Article2);
let tx_ucc = ucc_adoption_status("TX", UCCArticle::Article2);

println!("CA version: {:?}", ca_ucc);
println!("TX version: {:?}", tx_ucc);

// If versions differ, choice of law analysis needed
// (See Choice of Law Guide for analysis)
```

### Use Case 2: Secured Lending Across States

**Problem**: Bank in NY lends to business in DE with collateral in CA, NV, TX.

```rust
// Article 9: Filing location is debtor's location
let filing_state = "DE"; // Debtor incorporated in Delaware

let security_interest = Article9Analysis::new()
    .with_filing_state("DE")
    .with_collateral_locations(vec!["CA", "NV", "TX"])
    .with_collateral_type(CollateralType::Equipment);

// File UCC-1 in Delaware (where debtor is located)
// Perfection follows debtor, not collateral location
```

### Use Case 3: Partnership Dissolution

**Problem**: 4-person partnership in Illinois. One partner wants to leave.

```rust
// Check if Illinois uses UPA or RUPA
let il_upa = upa_adoption_status("IL");

match il_upa {
    UPAAdoptionStatus::RUPA { .. } => {
        println!("Illinois uses RUPA (entity theory)");
        println!("Partner can dissociate without dissolving partnership");
        println!("Remaining 3 partners can continue business");
        println!("Departing partner entitled to buyout");
    }
    UPAAdoptionStatus::UPA1914 => {
        println!("Illinois uses UPA (1914)");
        println!("Partner withdrawal causes dissolution");
        println!("Partnership must wind up and liquidate");
    }
    _ => {}
}
```

### Use Case 4: Software Transaction

**Problem**: Is software sale covered by UCC Article 2?

```rust
// Courts split on software transactions

let software_analysis = UCCScopeAnalysis::new()
    .with_transaction_type("Software purchase")
    .with_delivery_method("Download") // vs. physical media
    .with_license_restrictions(true); // Click-wrap license

let result = software_analysis.analyze();

match result.ucc_applies {
    UCCApplicability::Applies => {
        println!("✅ UCC Article 2 applies");
        println!("Software is 'goods' (minority view)");
    }
    UCCApplicability::DoesNotApply => {
        println!("❌ UCC does not apply");
        println!("Software is license, not sale (majority view)");
        println!("Common law contract principles apply");
    }
    UCCApplicability::Uncertain => {
        println!("⚠️ Circuit split / jurisdiction-specific");
    }
}

// Some states (e.g., UCITA states) have specific statutes
```

---

## Enactment Patterns

### Why UCC Succeeded

**UCC Adoption**: Nearly 100% (all 51 jurisdictions)

**Reasons:**
1. **Commercial necessity**: Interstate commerce requires uniform rules
2. **Network effects**: More states adopt → more pressure on holdouts
3. **Business lobby**: Strong support from commercial interests
4. **Comprehensive**: Covers entire commercial law ecosystem
5. **ALI/NCCUSL prestige**: Respected drafting organizations

### Why Other Uniform Acts Lag

**RUPA Adoption**: ~80% of states

**UPC Adoption**: ~40% of states

**Reasons for slower adoption:**
1. **Less urgent**: Probate law mostly intrastate
2. **Local variation**: States prefer own traditions
3. **Bar opposition**: Lawyers trained in existing law resist change
4. **Political inertia**: No strong lobby for uniformity

---

## Tracking Amendments

Uniform acts are periodically updated:

```rust
// Track which amendments state has adopted

let article9 = ucc_adoption_status("CA", UCCArticle::Article9);

match article9 {
    UCCAdoptionStatus::Adopted { version, .. } => {
        println!("Version: {}", version);

        // Check for specific amendments
        if version.contains("2022") {
            println!("✅ Includes 2022 amendments (digital assets)");
        }

        if version.contains("2010") {
            println!("✅ Includes 2010 amendments");
        }
    }
    _ => {}
}
```

**Major UCC Revisions:**
- **2022**: Article 9 (digital assets, control agreements)
- **2003**: Article 2/2A (minor updates)
- **2001**: Article 9 (complete revision)
- **1990**: Article 3/4 (negotiable instruments)

---

## Best Practices

### 1. Always Check State Version

```rust
// Don't assume uniform = identical
let status = ucc_adoption_status(state_code, article);

// Check for non-uniform amendments
if let UCCAdoptionStatus::Adopted { notable_variations, .. } = status {
    for variation in notable_variations {
        println!("⚠️ State-specific variation: {}", variation);
    }
}
```

### 2. Consider Choice of Law Clauses

```rust
// Contracts can specify which state's UCC applies

// Example contract clause:
// "This Agreement shall be governed by the UCC as enacted in New York"

// Check if chosen state's law differs materially
let ny_ucc = ucc_adoption_status("NY", UCCArticle::Article2);
let ca_ucc = ucc_adoption_status("CA", UCCArticle::Article2);

// Compare versions and variations
```

### 3. Track Pending Amendments

```rust
// Uniform acts are regularly amended
// States may not adopt amendments immediately

// Check if amendment is pending in legislature
let pending = check_pending_amendments("CA", UCCArticle::Article9);

for amendment in pending {
    println!("⚠️ Pending: {}", amendment.description);
    println!("Status: {}", amendment.legislative_status);
}
```

---

## Further Reading

**UCC Resources:**
- **Official Text**: Available from ALI/NCCUSL
- **State Versions**: Check state legislature websites
- **Commentary**: Anderson on the UCC (treatise)

**UPA/RUPA Resources:**
- Revised Uniform Partnership Act (1997) with Comments
- State statutes (vary by jurisdiction)

**Organizations:**
- **Uniform Law Commission (ULC)**: Drafts uniform acts
- **American Law Institute (ALI)**: Co-sponsors UCC

---

**Next**: [Legislative Tracking Guide](07-legislative-tracking.md)
