# Gesellschaftsrecht - Leitfaden

Umfassender Leitfaden für deutsches Gesellschaftsrecht in Legalis-DE.

## Inhaltsverzeichnis

1. [GmbH-Gesetz (GmbHG)](#gmbh-gesetz)
2. [Handelsgesetzbuch (HGB)](#handelsgesetzbuch)
3. [Aktiengesetz (AktG)](#aktiengesetz)
4. [Praktische Beispiele](#praktische-beispiele)

---

## GmbH-Gesetz

Das GmbH-Gesetz regelt die Gründung und Organisation von Gesellschaften mit beschränkter Haftung.

### Grundlagen

**Gesellschaftsformen:**
- **GmbH**: Stammkapital mindestens €25.000 (§5 Abs. 1 GmbHG)
- **UG (haftungsbeschränkt)**: Stammkapital ab €1 (§5a GmbHG)

### Stammkapital (§5 GmbHG)

```rust
use legalis_de::gmbhg::*;

// GmbH mit €25.000 Stammkapital
let capital_gmbh = Capital::from_euros(25_000);
assert!(capital_gmbh.is_valid_for_gmbh());

// UG mit €5.000 Stammkapital
let capital_ug = Capital::from_euros(5_000);
assert!(capital_ug.is_valid_for_ug());
assert!(!capital_ug.is_valid_for_gmbh()); // Zu niedrig für GmbH

// Validierung
match validate_capital(&capital_gmbh, CompanyType::GmbH) {
    Ok(()) => println!("✅ Stammkapital gültig"),
    Err(e) => println!("❌ Fehler: {} ({})", e, e.article_reference()),
}
```

### Gesellschaftsvertrag (§3 GmbHG)

Der Gesellschaftsvertrag muss folgende essentialia negotii enthalten:

```rust
let articles = ArticlesOfAssociation {
    company_name: "Innovative Tech GmbH".to_string(),
    registered_office: "Berlin".to_string(),
    business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
    share_capital: Capital::from_euros(25_000),
    fiscal_year_end: FiscalYearEnd::CalendarYear,
    duration: Duration::Unlimited,
    shareholders: vec![
        Shareholder {
            name: "Max Mustermann".to_string(),
            shareholder_type: ShareholderType::NaturalPerson,
            share_allocation: ShareAllocation {
                nominal_value: Capital::from_euros(12_500),
                percentage: 50.0,
            },
        },
        Shareholder {
            name: "Erika Schmidt".to_string(),
            shareholder_type: ShareholderType::NaturalPerson,
            share_allocation: ShareAllocation {
                nominal_value: Capital::from_euros(12_500),
                percentage: 50.0,
            },
        },
    ],
};

// Validierung
validate_articles_of_association(&articles)?;
```

**Notarielle Beurkundung**: Erforderlich nach §2 Abs. 1 GmbHG (nicht in dieser Bibliothek implementiert).

### Geschäftsführung (§35 GmbHG)

```rust
let managing_directors = ManagingDirectors {
    directors: vec![
        ManagingDirector {
            name: "Dr. Thomas Weber".to_string(),
            appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            representation_authority: RepresentationAuthority::Joint, // §35 Abs. 2 Nr. 2
        },
        ManagingDirector {
            name: "Julia Schneider".to_string(),
            appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            representation_authority: RepresentationAuthority::Joint,
        },
    ],
    representation_type: RepresentationAuthority::Joint,
};

validate_managing_directors(&managing_directors)?;
```

**Vertretungsregelungen:**
- `Individual`: Einzelvertretung (§35 Abs. 1 GmbHG)
- `Joint`: Gesamtvertretung (§35 Abs. 2 Nr. 2 GmbHG)
- `WithProkura`: Mit Prokurist gemeinsam (§35 Abs. 2 Nr. 1 GmbHG)

### UG (haftungsbeschränkt) - Mini-GmbH

Besonderheiten nach §5a GmbHG:

```rust
let ug_capital = Capital::from_euros(1_000); // Minimum €1

// UG muss "haftungsbeschränkt" im Namen führen
let company_name = "Startup UG (haftungsbeschränkt)";

// Ansparrücklage (§5a Abs. 3 GmbHG)
// 25% des Jahresüberschusses müssen angespart werden,
// bis Stammkapital von €25.000 erreicht ist
```

---

## Handelsgesetzbuch

Das HGB regelt Personengesellschaften und Kaufleute.

### Offene Handelsgesellschaft (OHG)

**Merkmale:**
- Unbeschränkte Haftung aller Gesellschafter
- Mindestens 2 Gesellschafter erforderlich

```rust
use legalis_de::hgb::*;

let ohg = OHG {
    partnership_name: "Müller & Schmidt OHG".to_string(),
    registered_office: "Hamburg".to_string(),
    business_purpose: "Großhandel mit Elektronik".to_string(),
    partners: vec![
        Partner {
            name: "Hans Müller".to_string(),
            address: "Hamburg".to_string(),
            contribution: Some(Capital::from_euros(50_000)),
            contribution_paid: Some(Capital::from_euros(50_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        },
        Partner {
            name: "Petra Schmidt".to_string(),
            address: "Hamburg".to_string(),
            contribution: Some(Capital::from_euros(50_000)),
            contribution_paid: Some(Capital::from_euros(50_000)),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
        },
    ],
    formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    fiscal_year_end: Some(FiscalYearEnd::CalendarYear),
    unlimited_liability: true,
};

validate_ohg(&ohg)?;
```

**Haftung**: Persönliche, unbeschränkte und gesamtschuldnerische Haftung (§128 HGB).

### Kommanditgesellschaft (KG)

**Merkmale:**
- Mindestens 1 Komplementär (unbeschränkte Haftung)
- Mindestens 1 Kommanditist (beschränkte Haftung)

```rust
let kg = KG {
    partnership_name: "Weber & Co. KG".to_string(),
    registered_office: "München".to_string(),
    business_purpose: "Immobilienverwaltung".to_string(),
    general_partners: vec![
        Partner {
            name: "Klaus Weber".to_string(),
            address: "München".to_string(),
            partner_type: PartnerType::NaturalPerson,
            has_management_authority: true,
            has_representation_authority: true,
            contribution: Some(Capital::from_euros(100_000)),
            contribution_paid: Some(Capital::from_euros(100_000)),
        },
    ],
    limited_partners: vec![
        LimitedPartner {
            name: "Maria Bauer".to_string(),
            address: "München".to_string(),
            partner_type: PartnerType::NaturalPerson,
            contribution: Capital::from_euros(50_000),
            contribution_paid: Capital::from_euros(50_000),
            liability_limit: Capital::from_euros(50_000), // Haftsumme §171 HGB
        },
    ],
    formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    fiscal_year_end: Some(FiscalYearEnd::CalendarYear),
};

validate_kg(&kg)?;
```

**Haftung Kommanditist**: Beschränkt auf Haftsumme nach §171 HGB.

### GmbH & Co. KG

Hybridstruktur mit GmbH als Komplementär:

```rust
let gmbh_co_kg = GmbHCoKG {
    partnership_name: "Tech Invest GmbH & Co. KG".to_string(),
    registered_office: "Frankfurt".to_string(),
    business_purpose: "Beteiligungen und Vermögensverwaltung".to_string(),
    gmbh_partner: GmbHPartner {
        company_name: "Tech Invest Verwaltungs-GmbH".to_string(),
        share_capital: Capital::from_euros(25_000),
        managing_directors: vec!["Dr. Stefan Meyer".to_string()],
    },
    limited_partners: vec![
        LimitedPartner {
            name: "Investment Fund Alpha".to_string(),
            address: "Frankfurt".to_string(),
            partner_type: PartnerType::LegalEntity,
            contribution: Capital::from_euros(1_000_000),
            contribution_paid: Capital::from_euros(1_000_000),
            liability_limit: Capital::from_euros(1_000_000),
        },
    ],
    formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    fiscal_year_end: Some(FiscalYearEnd::CalendarYear),
};

validate_gmbh_co_kg(&gmbh_co_kg)?;
```

**Vorteil**: Haftungsbeschränkung durch GmbH als Komplementär.

---

## Aktiengesetz

Das AktG regelt die Aktiengesellschaft.

### Grundkapital (§7 AktG)

**Mindestbetrag**: €50.000 (§7 AktG)

```rust
use legalis_de::aktg::*;

let share_capital = Capital::from_euros(50_000);

// Aktienarten nach §8 AktG
let shares = vec![
    ShareType::ParValue {
        nominal_value: Capital::from_euros(1), // Nennwert
        quantity: 50_000, // 50.000 Aktien à €1
    },
];
```

### Aktientypen

#### Nennbetragsaktien (§8 Abs. 2 AktG)

```rust
ShareType::ParValue {
    nominal_value: Capital::from_euros(5),
    quantity: 10_000, // 10.000 Aktien à €5 = €50.000
}
```

#### Stückaktien (§8 Abs. 3 AktG)

```rust
ShareType::NoParValue {
    quantity: 50_000, // 50.000 Stückaktien
    share_of_capital: 100.0, // Anteil am Grundkapital
}
```

### Verbriefungsarten (§10 AktG)

```rust
// Inhaberaktien (§10 Abs. 1 AktG)
let bearer_shares = ShareCertificateType::Bearer {
    quantity: 40_000,
    transferable_freely: true,
};

// Namensaktien (§10 Abs. 1 S. 2 AktG)
let registered_shares = ShareCertificateType::Registered {
    shareholders: vec![
        "Max Mustermann".to_string(),
        "Erika Schmidt".to_string(),
    ],
};

// Vinkulierte Namensaktien (§68 Abs. 2 AktG)
let restricted_shares = ShareCertificateType::RestrictedRegistered {
    shareholders: vec!["Founder GmbH".to_string()],
    transfer_requires_approval: true,
};
```

### Dualistische Führungsstruktur

#### Vorstand (§76-94 AktG)

```rust
let management_board = ManagementBoard {
    members: vec![
        ManagementBoardMember {
            name: "Dr. Julia Weber".to_string(),
            position: BoardPosition::CEO, // Vorstandsvorsitzender
            appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            term_years: 5, // Max. 5 Jahre nach §84 Abs. 1 AktG
        },
        ManagementBoardMember {
            name: "Michael Fischer".to_string(),
            position: BoardPosition::CFO,
            appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            term_years: 5,
        },
    ],
    representation_type: RepresentationType::JointRepresentation, // §78 AktG
};
```

#### Aufsichtsrat (§95-116 AktG)

```rust
let supervisory_board = SupervisoryBoard {
    members: vec![
        SupervisoryBoardMember {
            name: "Prof. Dr. Hans Müller".to_string(),
            position: SupervisoryPosition::Chairman, // §107 AktG
            appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            term_years: 4, // Max. 4 Jahre nach §102 Abs. 1 AktG
            employee_representative: false,
        },
        // ... weitere Mitglieder
    ],
    total_members: 9, // Größe abhängig von Grundkapital §95 AktG
};
```

---

## Praktische Beispiele

### Beispiel 1: GmbH-Gründung

Vollständiges Beispiel einer GmbH-Gründung:

```rust
use legalis_de::gmbhg::*;
use chrono::NaiveDate;

fn main() -> Result<()> {
    // Schritt 1: Gesellschaftsvertrag
    let articles = ArticlesOfAssociation {
        company_name: "Digital Solutions GmbH".to_string(),
        registered_office: "Berlin".to_string(),
        business_purpose: "Entwicklung und Vertrieb von Software".to_string(),
        share_capital: Capital::from_euros(25_000),
        fiscal_year_end: FiscalYearEnd::CalendarYear,
        duration: Duration::Unlimited,
        shareholders: vec![
            Shareholder {
                name: "Anna Müller".to_string(),
                shareholder_type: ShareholderType::NaturalPerson,
                share_allocation: ShareAllocation {
                    nominal_value: Capital::from_euros(12_500),
                    percentage: 50.0,
                },
            },
            Shareholder {
                name: "Thomas Weber".to_string(),
                shareholder_type: ShareholderType::NaturalPerson,
                share_allocation: ShareAllocation {
                    nominal_value: Capital::from_euros(12_500),
                    percentage: 50.0,
                },
            },
        ],
    };

    // Validierung des Gesellschaftsvertrags
    validate_articles_of_association(&articles)?;
    println!("✅ Gesellschaftsvertrag gültig");

    // Schritt 2: Stammkapital prüfen
    validate_capital(&articles.share_capital, CompanyType::GmbH)?;
    println!("✅ Stammkapital von €{:.2} gültig",
             articles.share_capital.to_euros());

    // Schritt 3: Geschäftsführer bestellen
    let managing_directors = ManagingDirectors {
        directors: vec![
            ManagingDirector {
                name: "Anna Müller".to_string(),
                appointed_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                representation_authority: RepresentationAuthority::Individual,
            },
        ],
        representation_type: RepresentationAuthority::Individual,
    };

    validate_managing_directors(&managing_directors)?;
    println!("✅ Geschäftsführer bestellt");

    println!("\n🎉 GmbH-Gründung erfolgreich validiert");
    Ok(())
}
```

**Ausgabe:**
```
✅ Gesellschaftsvertrag gültig
✅ Stammkapital von €25000.00 gültig
✅ Geschäftsführer bestellt

🎉 GmbH-Gründung erfolgreich validiert
```

### Beispiel 2: UG-Gründung mit Ansparrücklage

```rust
// Mini-GmbH mit €5.000 Stammkapital
let ug_capital = Capital::from_euros(5_000);

validate_capital(&ug_capital, CompanyType::UG)?;

// Ansparrücklage berechnen (§5a Abs. 3 GmbHG)
let annual_profit = Capital::from_euros(20_000);
let reserve_required = Capital::from_cents(
    (annual_profit.to_cents() * 25) / 100
); // 25% = €5.000

println!("Jahresüberschuss: €{:.2}", annual_profit.to_euros());
println!("Pflicht-Rücklage: €{:.2}", reserve_required.to_euros());

// Nach 4 Jahren mit €5.000 Rücklage pro Jahr: €25.000 erreicht
// -> Umwandlung in GmbH möglich
```

### Beispiel 3: KG mit mehreren Kommanditisten

```bash
cargo run --example kg-limited-partnership
```

Siehe [examples/kg-limited-partnership.rs](../examples/kg-limited-partnership.rs) für vollständiges Beispiel.

---

## Fehlerbehandlung

Alle Validierungsfunktionen geben detaillierte Fehler mit Gesetzesreferenzen zurück:

```rust
match validate_capital(&capital, CompanyType::GmbH) {
    Err(GmbHError::InsufficientCapital { actual, required }) => {
        println!("Fehler: Stammkapital zu niedrig");
        println!("  Ist: €{:.2}", actual.to_euros());
        println!("  Soll: €{:.2}", required.to_euros());
        println!("  Rechtsgrundlage: §5 Abs. 1 GmbHG");
    }
    Err(GmbHError::InvalidCompanyName { reason }) => {
        println!("Fehler: Ungültiger Firmenname");
        println!("  Grund: {}", reason);
        println!("  Rechtsgrundlage: §4 GmbHG");
    }
    Ok(()) => println!("✅ Validierung erfolgreich"),
}
```

## Weiterführende Ressourcen

- [GmbHG Volltext](https://www.gesetze-im-internet.de/gmbhg/)
- [HGB Volltext](https://www.gesetze-im-internet.de/hgb/)
- [AktG Volltext](https://www.gesetze-im-internet.de/aktg/)
- [Beispiele](../examples/)
- [API-Dokumentation](API.md)
