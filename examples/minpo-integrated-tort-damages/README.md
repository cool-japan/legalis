# Integrated Tort and Contract Damages Example

Comprehensive demonstration integrating multiple articles of the Japanese Civil Code: Articles 415, 709, 710, and 715.

## Overview

This example presents a realistic scenario that triggers multiple legal provisions simultaneously, demonstrating how contract law and tort law intersect in practice. A single accident creates liability under both contract and tort regimes, with different victims claiming under different legal bases.

## The Scenario

**Restaurant Delivery Accident** (レストラン配達事故)

A restaurant outsources delivery service. During delivery:
1. Driver runs red light and hits pedestrian (serious injury)
2. Delivery to customer is delayed by 2 hours (missed party)

This single event triggers **four separate legal analyses**:
- Article 709: Driver's tort liability to pedestrian
- Article 710: Non-pecuniary damages to pedestrian
- Article 715: Delivery company's vicarious liability
- Article 415: Restaurant's breach of contract to customer

## Legal Provisions Integrated

### 1. Article 709 - Tort Liability (不法行為責任)

Driver commits tort:
- Negligence: Traffic signal violation
- Rights infringed: Pedestrian's body and health
- Damages: ¥3,000,000 (medical expenses)
- Causation: Direct

### 2. Article 710 - Non-Pecuniary Damages (慰謝料)

Pedestrian suffers emotional distress:
- Damage type: Body and health
- Severity: Severe (2-month hospitalization)
- Consolation money: ¥1,500,000 (recommended)
- Total: ¥4,500,000

### 3. Article 715 - Employer Liability (使用者責任)

Delivery company is vicariously liable:
- Employment: Contract worker
- Business execution: During delivery service
- Defense: Cannot establish reasonable care
- Result: Jointly liable with driver

### 4. Article 415 - Breach of Contract (債務不履行)

Restaurant breaches contract with customer:
- Obligation: 30-minute delivery service
- Breach: 2-hour delay
- Attribution: Driver's accident (restaurant's risk)
- Damages: ¥50,000 (substitute meal + party cancellation)

## Features

### Comprehensive Legal Analysis

Each step shows:
- Legal requirements validation
- Bilingual explanations (Japanese/English)
- Damage calculations
- Liability relationships
- Right of recourse

### Multiple Claim Paths

**Pedestrian (Victim A)** can claim:
- From driver personally: ¥4,500,000 (Articles 709 + 710)
- From delivery company: ¥4,500,000 (Article 715 joint liability)

**Customer (Victim B)** can claim:
- From restaurant: ¥50,000 (Article 415 contract breach)

### Reimbursement Chain

```
Delivery company pays pedestrian ¥4,500,000
    ↓
Company seeks reimbursement from driver
    (based on employment contract)

Restaurant pays customer ¥50,000
    ↓
Restaurant seeks reimbursement from delivery company
    (based on outsourcing contract)
```

## Usage

```bash
cargo run --bin minpo-integrated-tort-damages
```

Or from the subcrate directory:

```bash
cargo run
```

## What You'll Learn

### Contract vs. Tort Intersection

Same accident, different legal bases:
- **Third-party harm** → Tort law (Articles 709, 710, 715)
- **Contractual party harm** → Contract law (Article 415)

### Strategic Claim Choices

Victims choose defendants strategically:
- **Deep pockets**: Sue employer, not employee
- **Easier proof**: Vicarious liability vs. direct fault
- **Contract vs. tort**: Whichever provides better recovery

### Japanese Legal Integration

How different civil code provisions work together:
- Article 709: Foundation of tort liability
- Article 710: Enhances recovery with consolation money
- Article 715: Extends liability to employers
- Article 415: Parallel contract regime

### Practical Legal Reasoning

Real-world complexity:
- Multiple claimants
- Multiple defendants
- Multiple legal grounds
- Reimbursement relationships
- Insurance considerations

## Example Output Structure

```
STEP 1: Article 709 - Driver's tort liability
  ✅ Liability established

STEP 2: Article 710 - Consolation money
  ✅ ¥1,500,000 recommended

STEP 3: Article 715 - Employer vicarious liability
  ✅ Delivery company jointly liable

STEP 4: Article 415 - Contract breach
  ✅ Restaurant liable to customer

Summary: Total claims and reimbursement chain
```

## Comparison of Legal Regimes

| Aspect | Tort (Articles 709-715) | Contract (Article 415) |
|--------|------------------------|------------------------|
| **Parties** | Any third party | Contracting parties only |
| **Basis** | Rights violation | Breach of obligation |
| **Fault** | Intent/Negligence | Attribution |
| **Damages** | Pecuniary + Non-pecuniary | Foreseeable damages |
| **Vicarious** | Yes (Article 715) | No direct provision |

## Related Examples

- `minpo-415-breach-damages` - Contract breach only
- `minpo-709-tort` - Tort liability only
- `minpo-710-damages-builder` - Non-pecuniary damages only
- `minpo-715-employer-liability` - Employer liability only
- `comparative-tort-law` - Cross-jurisdictional comparison

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
