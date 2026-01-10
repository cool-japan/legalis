# Tax Law Guide

**State income, sales, and corporate tax across 51 US jurisdictions**

## Overview

State tax policy creates one of the most significant areas of interstate variation in the US. States compete for residents and businesses through favorable tax policies, creating a "tax competition" dynamic.

Legalis-US tracks three major tax systems:
- **Income Tax** (personal)
- **Sales Tax** (consumption)
- **Corporate Tax** (business)

---

## Part 1: State Income Tax

### The Big Picture

**Three approaches:**
1. **No Income Tax** (9 states) - Attract residents/businesses
2. **Flat Tax** (9 states) - Simple, predictable
3. **Progressive Tax** (33 states) - Higher rates on higher incomes

### Check If State Has Income Tax

```rust
use legalis_us::tax::income_tax::{has_state_income_tax, no_income_tax_states};

// Quick check
if has_state_income_tax("TX") {
    println!("Texas has state income tax");
} else {
    println!("Texas has NO state income tax ✅");
}

// Get all no-tax states
let no_tax = no_income_tax_states();
println!("States with no income tax: {:?}", no_tax);
// ["AK", "FL", "NV", "SD", "TN", "TX", "WA", "WY", "NH"]
```

### Why 9 States Have No Income Tax

**Alternative revenue sources:**

**Alaska**: Oil revenue
- 70%+ of revenue from petroleum production taxes
- Alaska Permanent Fund pays residents annual dividend (~$1,600)

**Florida**: Tourism + sales tax
- 108 million tourists annually
- 6% state sales tax + local

**Nevada**: Gaming tax
- Casino tax provides 15-20% of revenue
- 6.85% sales tax

**Texas**: Sales tax + property tax
- Property taxes among highest in US
- 6.25% sales tax

**Washington**: Sales tax + B&O tax
- High sales tax (combined up to 10.4% in Seattle)
- Business & Occupation gross receipts tax

And so on...

### Get Complete Tax Structure

```rust
use legalis_us::tax::income_tax::income_tax_structure;

// California - highest rate in US
let ca = income_tax_structure("CA");

match ca.tax_type {
    IncomeTaxType::Progressive { top_rate, brackets } => {
        println!("California: {:.1}% top rate", top_rate * 100.0);
        // 13.3% top rate (highest in US)

        println!("Tax brackets:");
        for bracket in brackets {
            println!("  ${}: {:.1}%", bracket.threshold, bracket.rate * 100.0);
        }
    }
    _ => {}
}

// Check for local income tax
if ca.has_local_income_tax {
    println!("⚠️ Also has local income taxes");
}
```

### Compare Tax Burden Across States

```rust
// Compare high-tax vs. low-tax vs. no-tax states
let states = vec![
    ("CA", "California"),
    ("NY", "New York"),
    ("TX", "Texas"),
    ("FL", "Florida"),
    ("IL", "Illinois"),
];

for (code, name) in states {
    let structure = income_tax_structure(code);

    match structure.tax_type {
        IncomeTaxType::None => {
            println!("{}: NO INCOME TAX", name);
        }
        IncomeTaxType::Flat { rate } => {
            println!("{}: {:.2}% flat tax", name, rate * 100.0);
        }
        IncomeTaxType::Progressive { top_rate, .. } => {
            println!("{}: {:.1}% top rate (progressive)", name, top_rate * 100.0);
        }
    }
}

// Output:
// California: 13.3% top rate (progressive)
// New York: 10.9% top rate (progressive)
// Texas: NO INCOME TAX
// Florida: NO INCOME TAX
// Illinois: 4.95% flat tax
```

### Top Rates by State (2024)

**Highest Progressive Rates:**
1. California: 13.3% (>$1M)
2. Hawaii: 11% (>$200k)
3. New York: 10.9% (>$25M)
4. New Jersey: 10.75% (>$1M)
5. DC: 10.75% (>$1M)

**Flat Tax Rates:**
- Illinois: 4.95%
- Utah: 4.85%
- Massachusetts: 5.00%
- Colorado: 4.40%
- Pennsylvania: 3.07% (lowest)

### Local Income Taxes

Some states allow cities/counties to impose additional income tax:

```rust
// New York City residents pay state + city tax
let ny = income_tax_structure("NY");

if ny.has_local_income_tax {
    println!("New York has local income tax");
    for feature in ny.notable_features {
        println!("  - {}", feature);
        // "NYC adds up to 3.876% local tax"
    }
}

// Combined: 10.9% (NYS) + 3.876% (NYC) = 14.776% total
```

**States with significant local income tax:**
- **New York**: NYC (3.876%), Yonkers (1.61%)
- **Pennsylvania**: Philadelphia (3.79%), Pittsburgh (3%)
- **Ohio**: ~600 municipalities impose local tax
- **Indiana**: All 92 counties (0.25%-3.38%)
- **Maryland**: All 23 counties + Baltimore City (2.25%-3.20%)

---

## Part 2: Sales Tax

### Overview

**Key Developments:**
- **South Dakota v. Wayfair (2018)**: States can tax remote sales
- **Economic nexus**: $100k sales or 200 transactions triggers obligation
- **Marketplace facilitator laws**: Amazon, eBay collect tax

### Check Sales Tax Status

```rust
use legalis_us::tax::sales_tax::{has_sales_tax, state_sales_tax_rate};

// Check if state has sales tax
if has_sales_tax("OR") {
    println!("Oregon has sales tax");
} else {
    println!("Oregon has NO sales tax ✅");
}

// Get state rate
let ca_rate = state_sales_tax_rate("CA");
println!("California: {:.2}%", ca_rate * 100.0); // 7.25%
```

### Five No-Sales-Tax States

1. **Alaska** (AK) - Local sales taxes allowed
2. **Delaware** (DE) - No sales tax
3. **Montana** (MT) - No sales tax
4. **New Hampshire** (NH) - No sales tax
5. **Oregon** (OR) - No sales tax

### Post-Wayfair Economic Nexus

**Before Wayfair (pre-2018)**: Physical presence required
**After Wayfair (2018+)**: Economic nexus based on sales volume

```rust
use legalis_us::tax::sales_tax::post_wayfair_nexus;

// E-commerce seller: $150,000 in California sales
let nexus = post_wayfair_nexus("CA", 150_000, 180);

if let NexusType::Economic { threshold_amount, threshold_transactions } = nexus {
    if 150_000 >= threshold_amount {
        println!("✅ Economic nexus triggered in California");
        println!("Must collect and remit CA sales tax");
    }
}
```

**Typical thresholds** (post-Wayfair):
- **$100,000** in sales OR
- **200 transactions**

Many states have adopted this threshold.

### Highest Combined Rates

**State + Local:**
1. Louisiana: 11.55% combined (highest)
2. Tennessee: 9.55% combined
3. Arkansas: 9.53% combined
4. Alabama: 9.24% combined
5. Washington: 9.23% combined

```rust
// Louisiana has highest combined rate
let la = sales_tax_info("LA");
println!("State rate: {:.2}%", la.state_rate * 100.0); // 4.45%
println!("Combined: {:.2}%", la.average_combined_rate * 100.0); // 11.55%
```

### Marketplace Facilitator Laws

**All 46 states with sales tax** now have marketplace facilitator laws requiring platforms to collect tax.

```rust
// Check if marketplace facilitators must collect
let ca_info = sales_tax_info("CA");

if ca_info.marketplace_facilitator_law {
    println!("California requires marketplace facilitators to collect");
    // Amazon, eBay, Etsy must collect CA sales tax
}
```

---

## Part 3: Corporate Income Tax

### Overview

**Corporate tax competition** is intense:
- States offer low/no corporate tax to attract businesses
- "Tax havens" (DE, NV, WY) have special advantages
- Some states use franchise taxes instead

### Check Corporate Tax Rate

```rust
use legalis_us::tax::corporate_tax::{corporate_tax_rate, is_tax_haven};

// Compare corporate tax rates
let states = vec!["NJ", "CA", "TX", "NV", "WY"];

for state in states {
    let rate = corporate_tax_rate(state);

    if rate > 0.0 {
        println!("{}: {:.2}% corporate tax", state, rate * 100.0);
    } else {
        println!("{}: NO corporate income tax", state);

        if is_tax_haven(state) {
            println!("   ✅ Recognized tax haven");
        }
    }
}

// Output:
// NJ: 11.50% corporate tax (highest)
// CA: 8.84% corporate tax
// TX: NO corporate income tax
// NV: NO corporate income tax
//    ✅ Recognized tax haven
// WY: NO corporate income tax
//    ✅ Recognized tax haven
```

### Tax Haven States

**Three major corporate tax havens:**

**Delaware:**
```rust
let de_status = tax_haven_status("DE");

match de_status {
    TaxHavenStatus::TaxHaven { advantages } => {
        for advantage in advantages {
            println!("  - {}", advantage);
        }
    }
    _ => {}
}

// Advantages:
// - Court of Chancery (specialized business court)
// - Favorable corporate law (permissive for management)
// - No sales tax
// - Franchise tax system (not income tax based)
```

**Nevada:**
- No corporate income tax
- No franchise tax
- No personal income tax
- Low regulation, business-friendly

**Wyoming:**
- No corporate income tax
- No franchise tax
- No personal income tax
- Privacy protections for LLC owners

### Apportionment Formulas

**Issue**: Multi-state corporation earns income in many states. How to allocate?

**Three factors (traditional):**
1. **Property** (where assets located)
2. **Payroll** (where employees work)
3. **Sales** (where customers are)

**Modern trend**: **Single-factor sales apportionment** (only sales matter)

```rust
use legalis_us::tax::corporate_tax::apportionment_formula;

let formula = apportionment_formula("CA");

match formula {
    ApportionmentFormula::SingleFactorSales => {
        println!("California uses single-factor sales");
        println!("Only customer location matters");
    }
    ApportionmentFormula::ThreeFactor { property_weight, payroll_weight, sales_weight } => {
        println!("Traditional three-factor:");
        println!("  Property: {:.0}%", property_weight * 100.0);
        println!("  Payroll: {:.0}%", payroll_weight * 100.0);
        println!("  Sales: {:.0}%", sales_weight * 100.0);
    }
    _ => {}
}
```

**Trend**: Most states moving to single-factor sales to encourage in-state jobs/facilities.

---

## Tax Competition Analysis

### High-Tax vs. Low-Tax States

**High-Tax States (Income + Sales + Corporate):**
- California: 13.3% + 7.25% + 8.84%
- New York: 10.9% + 4% + 6.5%
- New Jersey: 10.75% + 6.625% + 11.5%

**Low/No-Tax States:**
- Texas: 0% + 6.25% + 0% (margin tax instead)
- Florida: 0% + 6% + 5.5%
- Nevada: 0% + 6.85% + 0%
- Wyoming: 0% + 4% + 0%

### Migration Patterns

```rust
// Compare tax burden for high earner
let income = 500_000.0;

let ca_burden = calculate_tax_burden("CA", income);
let tx_burden = calculate_tax_burden("TX", income);

let savings = ca_burden - tx_burden;
println!("Moving from CA to TX saves: ${:.0}/year", savings);

// Typical savings: $40,000-$60,000 annually for high earners
```

**Empirical evidence:**
- Modest migration from high-tax to low-tax states
- More corporate relocations than individual
- Other factors (jobs, weather, family) dominate

---

## Practical Use Cases

### Use Case 1: Personal Relocation Decision

```rust
// Compare total tax burden for relocation
fn compare_relocation(current: &str, target: &str, income: f64) {
    println!("\nRelocation Analysis: {} → {}", current, target);

    // Income tax
    if has_state_income_tax(target) {
        let structure = income_tax_structure(target);
        println!("Income tax: Yes");
    } else {
        println!("Income tax: NONE ✅");
    }

    // Sales tax
    let sales_rate = state_sales_tax_rate(target);
    println!("Sales tax: {:.2}%", sales_rate * 100.0);

    // Property tax (not in this module, but relevant)
}

compare_relocation("CA", "TX", 200_000.0);
compare_relocation("NY", "FL", 200_000.0);
```

### Use Case 2: Corporate Site Selection

```rust
// Evaluate states for new facility
let candidates = vec!["TX", "NV", "WY", "FL", "TN"];

for state in candidates {
    println!("\n{} Corporate Tax Profile:", state);

    let corp_rate = corporate_tax_rate(state);
    if corp_rate == 0.0 {
        println!("  Corporate tax: NONE ✅");
    } else {
        println!("  Corporate tax: {:.2}%", corp_rate * 100.0);
    }

    if is_tax_haven(state) {
        println!("  Tax haven: YES ✅");
    }

    let formula = apportionment_formula(state);
    println!("  Apportionment: {:?}", formula);
}
```

### Use Case 3: E-Commerce Sales Tax Compliance

```rust
// Determine where to collect sales tax
let annual_sales_by_state = vec![
    ("CA", 500_000, 1_200),
    ("TX", 150_000, 400),
    ("OR", 50_000, 120),  // No sales tax state
    ("DE", 75_000, 180),  // No sales tax state
];

for (state, sales, transactions) in annual_sales_by_state {
    if !has_sales_tax(state) {
        println!("{}: No sales tax (skip)", state);
        continue;
    }

    let nexus = post_wayfair_nexus(state, sales, transactions);

    match nexus {
        NexusType::Economic { .. } => {
            println!("{}: ✅ Must collect (${} sales)", state, sales);
        }
        NexusType::NoNexus => {
            println!("{}: No obligation (${} sales)", state, sales);
        }
        _ => {}
    }
}
```

---

## Future Trends

### Income Tax

**Flat Tax Movement:**
- North Carolina (2014): 7.75% progressive → 4.75% flat
- Arizona (2021): Moving to 2.5% flat
- Iowa (2023): Transitioning to 3.9% flat
- Mississippi (2022): Moving to 4% flat

**Rationale:**
- Supply-side economics ("lower taxes → growth")
- Tax competition with neighboring states
- Simplification

### Sales Tax

**Post-Wayfair Evolution:**
- Thresholds stabilizing at $100k/200 transactions
- Marketplace facilitator laws now universal
- Streamlined Sales Tax Project (SST) growing

**Challenges:**
- Compliance burden for small sellers
- Software costs (sales tax calculation)
- Audit risk across 50 states

### Corporate Tax

**Race to the Bottom:**
- States lowering rates to attract corporations
- Single-factor sales apportionment spreading
- "Economic development" tax incentives

**Delaware's Dominance:**
- 68% of Fortune 500 incorporated in Delaware
- Court of Chancery expertise
- Unlikely to be displaced

---

## Best Practices

1. **Income Tax**: Check both state AND local rates (NYC, Philadelphia, etc.)
2. **Sales Tax**: Use nexus calculators; don't ignore Wayfair thresholds
3. **Corporate Tax**: Consider total tax burden, not just headline rate
4. **Planning**: Tax shouldn't be the only factor (workforce, infrastructure matter)
5. **Compliance**: Multi-state operations require professional tax advice

---

**Next**: [Federal Preemption Guide](05-federal-preemption.md)
