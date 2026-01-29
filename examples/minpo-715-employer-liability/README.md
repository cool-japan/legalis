# Article 715 Employer Liability Demo

Demonstration of Japanese Civil Code Article 715 (使用者責任 / Employer/Supervisor Liability) using the Legalis-RS framework.

## Overview

This example demonstrates how to model employer vicarious liability under Article 715 of the Japanese Civil Code. It shows how employers can be held liable for torts committed by their employees during the course of business, and explores the conditions for such liability and potential defenses.

## What is Article 715?

Article 715 of the Japanese Civil Code establishes vicarious liability:

> A person who employs another person for a certain business shall be liable for damages that the other person inflicts on a third party in the course of performance of such business. However, this shall not apply if the employer exercised reasonable care in appointing the employee or in supervising the business, or if the damage would have occurred even if reasonable care had been exercised.

### Key Elements

1. **Employment relationship** (使用関係) - Master-servant relationship
2. **During business execution** (事業執行性) - Act occurred in course of business
3. **Employee tort** (被用者の不法行為) - Employee committed Article 709 tort
4. **Defense** (免責) - Employer can escape liability if they exercised reasonable care

## Features

### Three Demonstration Scenarios

1. **Direct Employer Liability** - Delivery driver causes traffic accident during work
2. **Supervisor Duty Violation** - Part-time employee leaks customer information
3. **Negligent Hiring** - Transport company hired unlicensed driver

### Employer Defense (免責の抗弁)

Article 715 allows employers to escape liability if they prove:
- **Reasonable care in appointment** (選任の注意) - Proper hiring procedures
- **Reasonable care in supervision** (監督の注意) - Adequate oversight

However, this defense is **extremely difficult** to establish in practice. Courts impose a high burden on employers.

## Usage

```bash
cargo run --bin minpo-715-employer-liability
```

Or from the subcrate directory:

```bash
cargo run
```

## Article 715 Structure

```
Employee commits tort (Article 709)
       ↓
During business execution?
       ↓
Employment relationship exists?
       ↓
Article 715: Employer is JOINTLY LIABLE
       ↓
Defense available?
  • Reasonable care in appointment?
  • Reasonable care in supervision?
       ↓
  (Very hard to prove)
```

## Employment Types Covered

- **FullTime** (正社員) - Regular full-time employees
- **PartTime** (アルバイト/パート) - Part-time workers
- **Contract** (契約社員) - Contract workers
- **Dispatch** (派遣社員) - Temporary agency workers

Article 715 applies to **all employment types** as long as there is effective supervision and control.

## Key Concepts

### Business Execution (事業執行性)

The tort must occur "in the course of business." Japanese courts use the **apparent authority doctrine** (外形理論):
- Was the act **externally recognizable** as business-related?
- Would third parties reasonably believe it was business activity?

### Joint Liability (連帯責任)

Victim can sue:
- **Employee alone** - Individual tortfeasor
- **Employer alone** - Company under Article 715
- **Both jointly** - Both liable together

The victim chooses the most convenient defendant (usually the employer with "deep pockets").

### Right of Recourse (求償権)

If employer pays victim:
- Employer can seek **reimbursement** from employee
- Amount depends on employee's fault level
- Based on employment contract terms
- Courts often limit to partial recovery

## Example Scenarios

### Scenario 1: Delivery Driver Accident

```
Facts: Driver hits pedestrian during delivery
Result: ✅ Employer liable
Reason:
  • Regular delivery business
  • During working hours
  • Clear business context
```

### Scenario 2: Information Leakage

```
Facts: Part-time worker posts customer info on social media
Result: ✅ Employer liable
Reason:
  • Failed to educate on privacy
  • Inadequate social media policy
  • Supervision duty breach
```

### Scenario 3: Unlicensed Driver

```
Facts: Company hired driver without checking license
Result: ✅ Employer liable (cannot escape)
Defense: ❌ Failed
Reason:
  • No license verification
  • Negligent hiring process
  • Cannot prove "reasonable care"
```

## Practical Implications

### For Employers

1. **Hiring**: Verify credentials, check backgrounds
2. **Training**: Provide proper education and safety training
3. **Supervision**: Monitor employees, enforce rules
4. **Insurance**: Maintain liability insurance
5. **Documentation**: Keep records of all care taken

### For Victims

1. **Deep Pockets**: Sue employer for better recovery
2. **Easier Proof**: No need to prove employer's own fault
3. **Joint Liability**: Flexibility in choosing defendants

## Related Legal Provisions

- **Article 709** - Employee's tort liability
- **Article 710** - Non-pecuniary damages
- **Article 714** - Parental liability for minors
- **Article 715** - Employer liability (this example)
- **Article 717** - Possessor's liability for defects

## Related Examples

- `minpo-709-tort` - Basic tort liability
- `minpo-709-builder` - Builder API for Article 709
- `minpo-integrated-tort-damages` - Comprehensive integration

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
