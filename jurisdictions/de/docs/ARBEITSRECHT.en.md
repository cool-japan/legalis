# Labor Law - Guide

Comprehensive guide to German labor law in Legalis-DE.

## Table of Contents

1. [Individual Labor Law](#individual-labor-law)
2. [Collective Labor Law](#collective-labor-law)
3. [Practical Examples](#practical-examples)

---

## Individual Labor Law

### Employment Contract

```rust
use legalis_de::arbeitsrecht::*;
use chrono::NaiveDate;

let contract = EmploymentContract {
    employee_name: "John Doe".to_string(),
    employer_name: "Tech GmbH".to_string(),
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    end_date: None, // Permanent contract
    contract_type: ContractType::Permanent,
    probation_period: Some(ProbationPeriod {
        duration_months: 6, // Max. 6 months per §622 para. 3 BGB
    }),
    salary: Salary::Monthly {
        gross_amount: Capital::from_euros(4_500),
    },
    working_hours: WorkingHours {
        hours_per_week: 40,
        days_per_week: 5,
    },
    leave_entitlement: LeaveEntitlement {
        days_per_year: 28, // Minimum 24 per §3 BUrlG
    },
    workplace: "Berlin".to_string(),
    job_description: "Software Developer".to_string(),
};

validate_employment_contract(&contract)?;
```

### Working Hours Act (ArbZG)

**§3 ArbZG**: Max. 8 hours per day (10 hours with compensation)

```rust
let working_hours = WorkingHours {
    hours_per_week: 40,
    days_per_week: 5,
};

// Check ArbZG compliance
if working_hours.complies_with_arbzg() {
    println!("✅ Working hours ArbZG-compliant");
} else {
    println!("❌ Violation of §3 ArbZG");
}
```

### Federal Leave Act (BUrlG)

**§3 BUrlG**: Minimum 24 working days annual leave (6-day week)

```rust
let min_leave = LeaveEntitlement::calculate_minimum(5); // 5-day week
assert_eq!(min_leave, 20); // 20 working days for 5-day week

let min_leave_6day = LeaveEntitlement::calculate_minimum(6); // 6-day week
assert_eq!(min_leave_6day, 24); // 24 working days for 6-day week
```

### Protection Against Dismissal (KSchG)

**§1 KSchG**: Social justification required

```rust
let dismissal = Dismissal {
    employee_name: "John Doe".to_string(),
    employer_name: "Tech GmbH".to_string(),
    dismissal_type: DismissalType::Ordinary,
    dismissal_ground: DismissalGround::Operational(
        "Plant closure".to_string()
    ),
    notice_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
    notice_period_weeks: 4, // Minimum 4 weeks per §622 BGB
    works_council_consulted: true, // Required per §102 BetrVG
    written_form: true, // Required per §623 BGB
};

validate_dismissal(&dismissal)?;
```

**Grounds for Dismissal** (§1 para. 2 KSchG):
- **Conduct-related**: Employee misconduct
- **Personal**: Personal characteristics (e.g. illness)
- **Operational**: Business requirements

### Works Council (BetrVG)

**§1 BetrVG**: Threshold 5+ employees

```rust
let employee_count = 250;

if WorksCouncil::is_required(employee_count) {
    let size = WorksCouncil::required_size(employee_count);
    println!("Works council required: {} members (§9 BetrVG)", size);
}
```

**Size per §9 BetrVG:**
- 5-20 employees: 1 member
- 21-50: 3 members
- 51-100: 5 members
- 101-200: 7 members
- 201-400: 9 members

---

## Collective Labor Law

### Collective Bargaining Agreement (TVG)

**§1 TVG**: Normative and obligational provisions

```rust
let agreement = CollectiveBargainingAgreement {
    agreement_name: "Collective Agreement Metal Industry NRW".to_string(),
    parties: BargainingParties {
        union: Union {
            name: "IG Metall".to_string(),
            registered: true,
            member_count: 2_200_000,
        },
        employer_association: Some(EmployerAssociation {
            name: "Gesamtmetall".to_string(),
            member_companies: 6_500,
        }),
        individual_employer: None,
    },
    effective_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    expiry_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
    agreement_type: AgreementType::IndustryWide,
    coverage: AgreementCoverage::Regional {
        region: "North Rhine-Westphalia".to_string(),
    },
    normative_provisions: vec![
        NormativeProvision {
            provision_type: ProvisionType::Compensation,
            description: "Wage grades 1-12".to_string(),
            details: ProvisionDetails::WageScale {
                grades: vec![
                    WageGrade {
                        grade: 1,
                        description: "Unskilled workers".to_string(),
                        monthly_wage: Capital::from_euros(3_200),
                    },
                    WageGrade {
                        grade: 5,
                        description: "Skilled workers".to_string(),
                        monthly_wage: Capital::from_euros(4_200),
                    },
                ],
            },
        },
    ],
    obligational_provisions: vec![
        "Peace obligation".to_string(),
        "Quarterly committee meetings".to_string(),
    ],
    registered: true,
};

validate_collective_agreement(&agreement)?;
```

**After-Effect** (§4 para. 5 TVG): Provisions continue after expiry until new agreement.

### Co-Determination (MitbestG, DrittelbG)

**Supervisory Board Co-Determination:**

| Employee Count | Law | Type | Employee Representatives |
|----------------|-----|------|--------------------------|
| < 500 | - | None | 0% |
| 500-1,999 | DrittelbG | One-Third | 33% |
| 2,000+ | MitbestG | Parity | 50% |

```rust
let employee_count = 3_500;

let required_type = SupervisoryBoard::required_codetermination(employee_count);
// => CodeterminationType::Full (MitbestG)

let board_size = SupervisoryBoard::required_size(employee_count);
// => 12 members (6 employees, 6 shareholders)

let board = SupervisoryBoard {
    company_name: "Large Corp AG".to_string(),
    employee_count: 3_500,
    codetermination_type: CodeterminationType::Full,
    total_members: 12,
    employee_representatives: 6, // 50%
    shareholder_representatives: 6, // 50%
    members: vec![/* ... */],
};

validate_supervisory_board(&board)?;
```

### Works Constitution (BetrVG §87)

**Co-Determination Rights** (§87 BetrVG - Social Matters):

```rust
let rights = CodeterminationRights {
    company_name: "Tech GmbH".to_string(),
    employee_count: 250,
    rights: vec![
        CodeterminationRight {
            right_type: CodeterminationRightType::WorkingHours,
            description: "Start and end of working hours".to_string(),
            legal_basis: "§87 para. 1 no. 2 BetrVG".to_string(),
        },
        CodeterminationRight {
            right_type: CodeterminationRightType::Overtime,
            description: "Overtime and short-time work".to_string(),
            legal_basis: "§87 para. 1 no. 3 BetrVG".to_string(),
        },
        CodeterminationRight {
            right_type: CodeterminationRightType::LeaveScheduling,
            description: "Leave scheduling principles".to_string(),
            legal_basis: "§87 para. 1 no. 5 BetrVG".to_string(),
        },
    ],
};

validate_codetermination_rights(&rights)?;
```

**Full Co-Determination**: Employer cannot decide without works council consent.

---

## Practical Examples

### Example 1: Employment Contract Validation

```bash
cargo run --example employment-contract-validation
```

**Output:**
```
✅ Employment contract valid
   - Employee: John Doe
   - Start: 2024-01-01
   - Salary: €4,500 gross/month
   - Hours: 40h/week (ArbZG-compliant)
   - Leave: 28 days/year (>= 20 per BUrlG)
   - Probation: 6 months (§622 para. 3 BGB)
```

### Example 2: Dismissal Protection Analysis

```bash
cargo run --example dismissal-protection-analysis
```

Shows various dismissal scenarios with KSchG validation.

### Example 3: Collective Bargaining Agreement

```bash
cargo run --example collective-bargaining-agreement
```

Complete example of industry-wide agreement with wage grades.

### Example 4: Co-Determination

```bash
cargo run --example supervisory-board-codetermination
```

Supervisory board examples for DrittelbG and MitbestG.

### Example 5: Works Council Rights

```bash
cargo run --example works-council-rights
```

Co-determination rights per §87, §98, §99 BetrVG.

---

## Error Handling

```rust
match validate_dismissal(&dismissal) {
    Err(LaborLawError::NoticePeriodTooShort { actual, required }) => {
        println!("❌ Notice period too short");
        println!("   Actual: {} weeks", actual);
        println!("   Required: {} weeks (§622 BGB)", required);
    }
    Err(LaborLawError::WrittenFormRequired) => {
        println!("❌ Written form required (§623 BGB)");
    }
    Err(LaborLawError::WorksCouncilNotConsulted) => {
        println!("❌ Works council not consulted (§102 BetrVG)");
    }
    Ok(()) => println!("✅ Dismissal legally valid"),
}
```

## Further Resources

- [BGB §§611-630](https://www.gesetze-im-internet.de/englisch_bgb/)
- [ArbZG](https://www.gesetze-im-internet.de/arbzg/)
- [BUrlG](https://www.gesetze-im-internet.de/burlg/)
- [KSchG](https://www.gesetze-im-internet.de/kschg/)
- [TVG](https://www.gesetze-im-internet.de/tvg/)
- [BetrVG](https://www.gesetze-im-internet.de/betrvg/)
- [MitbestG](https://www.gesetze-im-internet.de/mitbestg/)
- [Examples](../examples/)
