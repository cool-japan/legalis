# API-Referenz

Praktischer Leitfaden zur Verwendung der Legalis-DE API.

## Installation

```toml
[dependencies]
legalis-de = "0.1.1"
chrono = "0.4"  # Für Datumswerte
serde = { version = "1.0", features = ["derive"] }  # Optional: Serialisierung
```

## Grundlegende Verwendung

### Module importieren

```rust
// Gesellschaftsrecht
use legalis_de::gmbhg::*;
use legalis_de::hgb::*;
use legalis_de::aktg::*;

// BGB
use legalis_de::bgb::schuldrecht::*;
use legalis_de::bgb::unerlaubte_handlungen::*;

// Verfassungsrecht
use legalis_de::grundgesetz::*;

// Arbeitsrecht
use legalis_de::arbeitsrecht::*;

// Hilfsdienste
use chrono::NaiveDate;
```

### Kapitalbetrag erstellen

```rust
use legalis_de::gmbhg::Capital;

// Von Euro-Betrag
let capital = Capital::from_euros(25_000);
assert_eq!(capital.to_euros(), 25_000.0);

// Von Cent-Betrag
let capital = Capital::from_cents(2_500_000);
assert_eq!(capital.to_cents(), 2_500_000);

// Operationen
let sum = capital1.add(&capital2);
let diff = capital1.subtract(&capital2)?; // Fehler wenn negativ
```

## Validierungsmuster

### Einfache Validierung

```rust
use legalis_de::gmbhg::*;

let capital = Capital::from_euros(25_000);

match validate_capital(&capital, CompanyType::GmbH) {
    Ok(()) => println!("✅ Gültig"),
    Err(e) => {
        println!("❌ Fehler: {}", e);
        println!("   Rechtsgrundlage: {}", e.article_reference());
    }
}
```

### Strukturierte Fehlerbehandlung

```rust
use legalis_de::gmbhg::{GmbHError, Result};

fn process_gmbh(capital: &Capital) -> Result<()> {
    validate_capital(capital, CompanyType::GmbH)?;
    // Weitere Verarbeitung...
    Ok(())
}

match process_gmbh(&capital) {
    Ok(()) => println!("Erfolg"),
    Err(GmbHError::InsufficientCapital { actual, required }) => {
        eprintln!("Kapital zu niedrig: €{:.2} < €{:.2}",
                  actual.to_euros(), required.to_euros());
    }
    Err(e) => eprintln!("Fehler: {}", e),
}
```

### Fehlermethoden

Alle Fehlertypen implementieren Hilfsmethoden:

```rust
let error = GmbHError::InsufficientCapital {
    actual: Capital::from_euros(20_000),
    required: Capital::from_euros(25_000),
};

// Gesetzesreferenz
assert_eq!(error.article_reference(), "§5 Abs. 1 GmbHG");

// Ungültigkeit prüfen
if error.makes_contract_void() {
    println!("Vertrag nichtig");
} else if error.makes_contract_voidable() {
    println!("Vertrag anfechtbar");
}
```

## Builder-Muster

### TortClaim Builder

```rust
use legalis_de::bgb::unerlaubte_handlungen::*;

let claim = TortClaim823_1Builder::new()
    .plaintiff("Max Mustermann".to_string())
    .defendant("Schädiger GmbH".to_string())
    .protected_interest(ProtectedInterest::Property)
    .unlawful_act("Fahrlässige Sachbeschädigung".to_string())
    .fault(Verschulden::OrdinaryNegligence)
    .damage_amount(Capital::from_euros(5_000))
    .build()?;

validate_tort_claim_823_1(&claim)?;
```

### Bewegliches Sachenrecht Builder

```rust
use legalis_de::bgb::sachenrecht::*;

let transfer = MovableTransferBuilder::new()
    .transferor("Verkäufer".to_string())
    .transferee("Käufer".to_string())
    .item_description("Gebrauchter PKW VW Golf".to_string())
    .transfer_agreement_exists(true)
    .delivery_completed(true)
    .transferor_is_owner(true)
    .build()?;
```

## Serialisierung

Alle Typen unterstützen Serde:

```rust
use legalis_de::gmbhg::*;
use serde_json;

let capital = Capital::from_euros(25_000);

// Nach JSON
let json = serde_json::to_string(&capital)?;
println!("{}", json); // {"cents":2500000}

// Von JSON
let deserialized: Capital = serde_json::from_str(&json)?;
assert_eq!(capital, deserialized);
```

## Datum-Handling

```rust
use chrono::NaiveDate;

// Datum erstellen
let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

// Vergleiche
let later = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
assert!(later > date);

// Berechnungen
use chrono::Duration;
let future = date + Duration::days(30);
```

## Typische Workflows

### Workflow 1: GmbH Gründung

```rust
use legalis_de::gmbhg::*;

// 1. Kapital prüfen
let capital = Capital::from_euros(25_000);
validate_capital(&capital, CompanyType::GmbH)?;

// 2. Gesellschaftsvertrag
let articles = ArticlesOfAssociation {
    company_name: "Meine GmbH".to_string(),
    share_capital: capital,
    // ... weitere Felder
};
validate_articles_of_association(&articles)?;

// 3. Geschäftsführer
let directors = ManagingDirectors {
    // ...
};
validate_managing_directors(&directors)?;

println!("✅ GmbH-Gründung validiert");
```

### Workflow 2: Vertragsschluss

```rust
use legalis_de::bgb::schuldrecht::*;

// 1. Parteien erstellen
let seller = Party {
    name: "Verkäufer".to_string(),
    legal_capacity: LegalCapacity::Full,
    // ...
};

// 2. Angebot
let offer = Offer {
    offeror: seller.clone(),
    offeree: buyer.clone(),
    terms: ContractTerms { /* ... */ },
    // ...
};
validate_offer(&offer)?;

// 3. Annahme
let acceptance = Acceptance {
    offer: offer.clone(),
    accepted_at: Utc::now(),
    // ...
};
validate_acceptance(&acceptance, &offer)?;

// 4. Vertrag
let contract = Contract {
    parties: (seller, buyer),
    terms: offer.terms.clone(),
    // ...
};
validate_contract(&contract)?;
```

### Workflow 3: Kündigung prüfen

```rust
use legalis_de::arbeitsrecht::*;

let dismissal = Dismissal {
    employee_name: "Mitarbeiter".to_string(),
    dismissal_type: DismissalType::Ordinary,
    dismissal_ground: DismissalGround::Operational("Standortschließung".to_string()),
    notice_period_weeks: 4,
    works_council_consulted: true,
    written_form: true,
    // ...
};

// Umfassende Validierung
validate_dismissal(&dismissal)?;
```

## Häufige Muster

### Pattern Matching

```rust
match validate_employment_contract(&contract) {
    Ok(()) => {
        // Erfolg
    }
    Err(LaborLawError::WorkingHoursExceedLimit { hours, limit }) => {
        println!("Arbeitszeit {}h überschreitet Limit {}h", hours, limit);
    }
    Err(LaborLawError::InsufficientLeave { provided, required }) => {
        println!("Urlaub zu gering: {} < {} Tage", provided, required);
    }
    Err(e) => {
        eprintln!("Fehler: {}", e);
    }
}
```

### Conditional Checks

```rust
// Kapital prüfen
if capital.is_valid_for_gmbh() {
    println!("Für GmbH geeignet");
}

// Betriebsrat erforderlich?
if WorksCouncil::is_required(employee_count) {
    let size = WorksCouncil::required_size(employee_count);
    // Betriebsrat bilden
}

// Arbeitszeit konform?
if working_hours.complies_with_arbzg() {
    println!("ArbZG-konform");
}
```

### Iterationen

```rust
// Über Gesellschafter
for shareholder in &articles.shareholders {
    println!("{}: {}%",
             shareholder.name,
             shareholder.share_allocation.percentage);
}

// Mitbestimmungsrechte
for right in &codetermination_rights.rights {
    println!("{:?}: {}", right.right_type, right.legal_basis);
}
```

## Performance-Tipps

### Clone vs. Borrow

```rust
// Gut: Borgen wenn möglich
fn validate(capital: &Capital) -> Result<()> {
    validate_capital(capital, CompanyType::GmbH)
}

// Weniger gut: Clone wenn nötig
fn store(capital: Capital) -> Capital {
    capital // Ownership übertragen
}
```

### Fehler-Propagierung

```rust
// Effizient mit ?
fn process() -> Result<()> {
    validate_capital(&capital, CompanyType::GmbH)?;
    validate_articles(&articles)?;
    validate_directors(&directors)?;
    Ok(())
}
```

### Vorausberechnungen

```rust
// Werte cachen wenn mehrfach verwendet
let required_size = WorksCouncil::required_size(employee_count);
let required_type = SupervisoryBoard::required_codetermination(employee_count);

// Statt mehrfach aufzurufen
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gmbh_capital() {
        let capital = Capital::from_euros(25_000);
        assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_invalid_gmbh_capital() {
        let capital = Capital::from_euros(20_000);
        let result = validate_capital(&capital, CompanyType::GmbH);
        assert!(result.is_err());

        match result {
            Err(GmbHError::InsufficientCapital { .. }) => {},
            _ => panic!("Falscher Fehlertyp"),
        }
    }
}
```

### Integration Tests

```rust
#[test]
fn test_complete_gmbh_formation() -> Result<()> {
    let articles = create_test_articles();
    validate_articles_of_association(&articles)?;

    let directors = create_test_directors();
    validate_managing_directors(&directors)?;

    Ok(())
}
```

## Weiterführende Ressourcen

- [Beispiele](../examples/)
- [Tests](../tests/)
- [TODO](../TODO.md)
- [Rust-Dokumentation](https://docs.rs/legalis-de/)
