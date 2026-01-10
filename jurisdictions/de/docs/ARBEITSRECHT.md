# Arbeitsrecht - Leitfaden

Umfassender Leitfaden für deutsches Arbeitsrecht in Legalis-DE.

## Inhaltsverzeichnis

1. [Individualarbeitsrecht](#individualarbeitsrecht)
2. [Kollektivarbeitsrecht](#kollektivarbeitsrecht)
3. [Praktische Beispiele](#praktische-beispiele)

---

## Individualarbeitsrecht

### Arbeitsvertrag

```rust
use legalis_de::arbeitsrecht::*;
use chrono::NaiveDate;

let contract = EmploymentContract {
    employee_name: "Max Mustermann".to_string(),
    employer_name: "Tech GmbH".to_string(),
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    end_date: None, // Unbefristeter Vertrag
    contract_type: ContractType::Permanent,
    probation_period: Some(ProbationPeriod {
        duration_months: 6, // Max. 6 Monate nach §622 Abs. 3 BGB
    }),
    salary: Salary::Monthly {
        gross_amount: Capital::from_euros(4_500),
    },
    working_hours: WorkingHours {
        hours_per_week: 40,
        days_per_week: 5,
    },
    leave_entitlement: LeaveEntitlement {
        days_per_year: 28, // Minimum 24 nach §3 BUrlG
    },
    workplace: "Berlin".to_string(),
    job_description: "Softwareentwickler".to_string(),
};

validate_employment_contract(&contract)?;
```

### Arbeitszeitgesetz (ArbZG)

**§3 ArbZG**: Max. 8 Stunden pro Tag (10 Stunden mit Ausgleich)

```rust
let working_hours = WorkingHours {
    hours_per_week: 40,
    days_per_week: 5,
};

// Prüfung auf ArbZG-Konformität
if working_hours.complies_with_arbzg() {
    println!("✅ Arbeitszeit ArbZG-konform");
} else {
    println!("❌ Verstoß gegen §3 ArbZG");
}
```

### Bundesurlaubsgesetz (BUrlG)

**§3 BUrlG**: Mindestens 24 Werktage Jahresurlaub (bei 6-Tage-Woche)

```rust
let min_leave = LeaveEntitlement::calculate_minimum(5); // 5-Tage-Woche
assert_eq!(min_leave, 20); // 20 Arbeitstage bei 5-Tage-Woche

let min_leave_6day = LeaveEntitlement::calculate_minimum(6); // 6-Tage-Woche
assert_eq!(min_leave_6day, 24); // 24 Werktage bei 6-Tage-Woche
```

### Kündigungsschutz (KSchG)

**§1 KSchG**: Soziale Rechtfertigung erforderlich

```rust
let dismissal = Dismissal {
    employee_name: "Max Mustermann".to_string(),
    employer_name: "Tech GmbH".to_string(),
    dismissal_type: DismissalType::Ordinary,
    dismissal_ground: DismissalGround::Operational(
        "Betriebsstilllegung".to_string()
    ),
    notice_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
    notice_period_weeks: 4, // Mindestens 4 Wochen nach §622 BGB
    works_council_consulted: true, // Pflicht nach §102 BetrVG
    written_form: true, // Pflicht nach §623 BGB
};

validate_dismissal(&dismissal)?;
```

**Kündigungsgründe** (§1 Abs. 2 KSchG):
- **Verhaltensbedingt**: Fehlverhalten des Arbeitnehmers
- **Personenbedingt**: Persönliche Eigenschaften (z.B. Krankheit)
- **Betriebsbedingt**: Betriebliche Erfordernisse

### Mutterschutz (MuSchG)

**§3 MuSchG**: 6 Wochen vor + 8 Wochen nach Geburt

```rust
let maternity_leave = MaternityLeave {
    employee_name: "Maria Schmidt".to_string(),
    expected_delivery_date: NaiveDate::from_ymd_opt(2024, 7, 15).unwrap(),
    leave_start: NaiveDate::from_ymd_opt(2024, 6, 3).unwrap(), // 6 Wochen vor
    leave_end: NaiveDate::from_ymd_opt(2024, 9, 9).unwrap(), // 8 Wochen nach
    dismissal_protection: true, // §17 MuSchG
};
```

### Betriebsrat (BetrVG)

**§1 BetrVG**: Schwelle 5+ Arbeitnehmer

```rust
let employee_count = 250;

if WorksCouncil::is_required(employee_count) {
    let size = WorksCouncil::required_size(employee_count);
    println!("Betriebsrat erforderlich: {} Mitglieder (§9 BetrVG)", size);
}
```

**Größe nach §9 BetrVG:**
- 5-20 Arbeitnehmer: 1 Mitglied
- 21-50: 3 Mitglieder
- 51-100: 5 Mitglieder
- 101-200: 7 Mitglieder
- 201-400: 9 Mitglieder

---

## Kollektivarbeitsrecht

### Tarifvertrag (TVG)

**§1 TVG**: Normative und schuldrechtliche Bestimmungen

```rust
let agreement = CollectiveBargainingAgreement {
    agreement_name: "Tarifvertrag Metallindustrie NRW".to_string(),
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
        region: "Nordrhein-Westfalen".to_string(),
    },
    normative_provisions: vec![
        NormativeProvision {
            provision_type: ProvisionType::Compensation,
            description: "Lohngruppen 1-12".to_string(),
            details: ProvisionDetails::WageScale {
                grades: vec![
                    WageGrade {
                        grade: 1,
                        description: "Ungelernte Arbeitskräfte".to_string(),
                        monthly_wage: Capital::from_euros(3_200),
                    },
                    WageGrade {
                        grade: 5,
                        description: "Facharbeiter".to_string(),
                        monthly_wage: Capital::from_euros(4_200),
                    },
                ],
            },
        },
    ],
    obligational_provisions: vec![
        "Friedenspflicht".to_string(),
        "Quartalsweise Tarifkommission".to_string(),
    ],
    registered: true,
};

validate_collective_agreement(&agreement)?;
```

**Nachwirkung** (§4 Abs. 5 TVG): Bestimmungen wirken nach Ablauf fort, bis neue Regelung.

### Mitbestimmung (MitbestG, DrittelbG)

**Aufsichtsrat-Mitbestimmung:**

| Arbeitnehmerzahl | Gesetz | Typ | Arbeitnehmervertreter |
|------------------|--------|-----|----------------------|
| < 500 | - | Keine | 0% |
| 500-1.999 | DrittelbG | Drittelbeteiligung | 33% |
| 2.000+ | MitbestG | Paritätisch | 50% |

```rust
let employee_count = 3_500;

let required_type = SupervisoryBoard::required_codetermination(employee_count);
// => CodeterminationType::Full (MitbestG)

let board_size = SupervisoryBoard::required_size(employee_count);
// => 12 Mitglieder (6 Arbeitnehmer, 6 Anteilseigner)

let board = SupervisoryBoard {
    company_name: "Großunternehmen AG".to_string(),
    employee_count: 3_500,
    codetermination_type: CodeterminationType::Full,
    total_members: 12,
    employee_representatives: 6, // 50%
    shareholder_representatives: 6, // 50%
    members: vec![/* ... */],
};

validate_supervisory_board(&board)?;
```

### Betriebsverfassung (BetrVG §87)

**Mitbestimmungsrechte** (§87 BetrVG - Soziale Angelegenheiten):

```rust
let rights = CodeterminationRights {
    company_name: "Tech GmbH".to_string(),
    employee_count: 250,
    rights: vec![
        CodeterminationRight {
            right_type: CodeterminationRightType::WorkingHours,
            description: "Beginn und Ende der Arbeitszeit".to_string(),
            legal_basis: "§87 Abs. 1 Nr. 2 BetrVG".to_string(),
        },
        CodeterminationRight {
            right_type: CodeterminationRightType::Overtime,
            description: "Überstunden und Kurzarbeit".to_string(),
            legal_basis: "§87 Abs. 1 Nr. 3 BetrVG".to_string(),
        },
        CodeterminationRight {
            right_type: CodeterminationRightType::LeaveScheduling,
            description: "Urlaubsgrundsätze".to_string(),
            legal_basis: "§87 Abs. 1 Nr. 5 BetrVG".to_string(),
        },
    ],
};

validate_codetermination_rights(&rights)?;
```

**Volle Mitbestimmung**: Arbeitgeber kann ohne Zustimmung des Betriebsrats nicht entscheiden.

---

## Praktische Beispiele

### Beispiel 1: Arbeitsvertrag Validierung

```bash
cargo run --example employment-contract-validation
```

**Ausgabe:**
```
✅ Arbeitsvertrag gültig
   - Arbeitnehmer: Max Mustermann
   - Beginn: 2024-01-01
   - Gehalt: €4.500 brutto/Monat
   - Arbeitszeit: 40h/Woche (ArbZG-konform)
   - Urlaub: 28 Tage/Jahr (>= 20 nach BUrlG)
   - Probezeit: 6 Monate (§622 Abs. 3 BGB)
```

### Beispiel 2: Kündigungsschutz Analyse

```bash
cargo run --example dismissal-protection-analysis
```

Zeigt verschiedene Kündigungsszenarien mit KSchG-Validierung.

### Beispiel 3: Tarifvertrag

```bash
cargo run --example collective-bargaining-agreement
```

Vollständiges Beispiel eines Branchentarifvertrags mit Lohngruppen.

### Beispiel 4: Mitbestimmung

```bash
cargo run --example supervisory-board-codetermination
```

Aufsichtsrat-Beispiele für DrittelbG und MitbestG.

### Beispiel 5: Betriebsrat Rechte

```bash
cargo run --example works-council-rights
```

Mitbestimmungsrechte nach §87, §98, §99 BetrVG.

---

## Fehlerbehandlung

```rust
match validate_dismissal(&dismissal) {
    Err(LaborLawError::NoticePeriodTooShort { actual, required }) => {
        println!("❌ Kündigungsfrist zu kurz");
        println!("   Ist: {} Wochen", actual);
        println!("   Mindestens: {} Wochen (§622 BGB)", required);
    }
    Err(LaborLawError::WrittenFormRequired) => {
        println!("❌ Schriftformerfordernis nicht erfüllt (§623 BGB)");
    }
    Err(LaborLawError::WorksCouncilNotConsulted) => {
        println!("❌ Betriebsrat nicht angehört (§102 BetrVG)");
    }
    Ok(()) => println!("✅ Kündigung rechtswirksam"),
}
```

## Weiterführende Ressourcen

- [BGB §§611-630](https://www.gesetze-im-internet.de/bgb/__611.html)
- [ArbZG](https://www.gesetze-im-internet.de/arbzg/)
- [BUrlG](https://www.gesetze-im-internet.de/burlg/)
- [KSchG](https://www.gesetze-im-internet.de/kschg/)
- [TVG](https://www.gesetze-im-internet.de/tvg/)
- [BetrVG](https://www.gesetze-im-internet.de/betrvg/)
- [MitbestG](https://www.gesetze-im-internet.de/mitbestg/)
- [Beispiele](../examples/)
