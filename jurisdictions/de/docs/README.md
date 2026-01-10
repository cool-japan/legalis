# Legalis-DE: Deutsche Rechtsbibliothek

Umfassende Rust-Bibliothek für deutsches Recht mit strukturierten Typen, Validatoren und Beispielen.

## Überblick

Legalis-DE bietet typsichere Rust-Implementierungen deutscher Rechtsvorschriften mit:

- **Umfassende Gesetzesabdeckung**: 20+ deutsche Gesetze implementiert
- **Typsicherheit**: Enums und Structs für alle rechtlichen Konzepte
- **Mehrsprachig**: Deutsche und englische Fehlermeldungen
- **Validierung**: Mehrstufige Validierung mit detaillierten Fehlern
- **Beispiele**: 20+ Arbeitsbeispiele für alle Hauptmerkmale
- **Produktionsreif**: 365 Tests, 0 Warnungen, ~25.000 Zeilen Code

## Abgedeckte Rechtsbereiche

### 1. Gesellschaftsrecht (7.240 Zeilen)

| Gesetz | Beschreibung | Module |
|--------|--------------|---------|
| **GmbHG** | GmbH & UG (haftungsbeschränkt) | `gmbhg` |
| **HGB** | Personengesellschaften (OHG, KG, GmbH & Co. KG) | `hgb` |
| **AktG** | Aktiengesellschaft (AG) | `aktg` |

### 2. Bürgerliches Gesetzbuch - BGB (11.913 Zeilen)

| Bereich | Paragraphen | Modul |
|---------|-------------|-------|
| **Schuldrecht** | §§104-361 (Vertragsrecht) | `bgb::schuldrecht` |
| **Deliktsrecht** | §§823, 826 (Unerlaubte Handlungen) | `bgb::unerlaubte_handlungen` |
| **Sachenrecht** | §§873-1259 (Eigentum, Pfandrechte) | `bgb::sachenrecht` |
| **Familienrecht** | §§1303-1698 (Ehe, Scheidung, Sorgerecht) | `bgb::familienrecht` |
| **Erbrecht** | §§1922-2385 (Testament, Pflichtteil) | `bgb::erbrecht` |

### 3. Grundgesetz (2.845 Zeilen)

| Bereich | Artikel | Modul |
|---------|---------|-------|
| **Grundrechte** | Art. 1-19 (Menschenwürde, Freiheitsrechte) | `grundgesetz` |
| **Staatsorganisation** | Art. 20-146 (Bundestag, Bundesrat, Regierung) | `grundgesetz` |

### 4. Arbeitsrecht (3.057 Zeilen)

| Gesetz | Bereich | Modul |
|--------|---------|-------|
| **BGB** | Individualarbeitsrecht (§§611-630) | `arbeitsrecht` |
| **ArbZG** | Arbeitszeitgesetz | `arbeitsrecht` |
| **BUrlG** | Bundesurlaubsgesetz | `arbeitsrecht` |
| **KSchG** | Kündigungsschutzgesetz | `arbeitsrecht` |
| **TVG** | Tarifvertragsgesetz | `arbeitsrecht` |
| **BetrVG** | Betriebsverfassungsgesetz | `arbeitsrecht` |
| **MitbestG** | Mitbestimmungsgesetz | `arbeitsrecht` |

## Schnellstart

### Installation

```toml
[dependencies]
legalis-de = "0.1.1"
```

### Beispiel: GmbH-Gründung

```rust
use legalis_de::gmbhg::*;

// Stammkapital von €25.000 erstellen
let capital = Capital::from_euros(25_000);

// Validierung
match validate_capital(&capital, CompanyType::GmbH) {
    Ok(()) => println!("✅ Stammkapital gültig"),
    Err(e) => println!("❌ Fehler: {}", e),
}
```

### Beispiel: Arbeitsvertrag

```rust
use legalis_de::arbeitsrecht::*;

let contract = EmploymentContract {
    employee_name: "Max Mustermann".to_string(),
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    salary: Salary::Monthly {
        gross_amount: Capital::from_euros(4_500),
    },
    working_hours: WorkingHours {
        hours_per_week: 40,
        days_per_week: 5,
    },
    // ... weitere Felder
};

validate_employment_contract(&contract)?;
```

## Dokumentationsstruktur

| Datei | Beschreibung |
|-------|--------------|
| [GESELLSCHAFTSRECHT.md](GESELLSCHAFTSRECHT.md) | GmbHG, HGB, AktG Leitfaden |
| [BGB.md](BGB.md) | Bürgerliches Gesetzbuch Leitfaden |
| [GRUNDGESETZ.md](GRUNDGESETZ.md) | Verfassungsrecht Leitfaden |
| [ARBEITSRECHT.md](ARBEITSRECHT.md) | Individual- und Kollektivarbeitsrecht |
| [API.md](API.md) | API-Referenz und Verwendung |

Englische Versionen: `.en.md` Erweiterung (z.B. `README.en.md`)

## Hauptmerkmale

### Typsicherheit

```rust
// Compiler verhindert ungültige Zustände
let capital = Capital::from_cents(2_499_900); // €24.999
assert!(!capital.is_valid_for_gmbh()); // Zu niedrig für GmbH

let capital = Capital::from_euros(25_000);
assert!(capital.is_valid_for_gmbh()); // ✅ Gültig
```

### Mehrsprachige Fehler

```rust
match validate_capital(&capital, CompanyType::GmbH) {
    Err(GmbHError::InsufficientCapital { actual, required }) => {
        // Deutsche Fehlermeldung mit Artikelreferenz
        println!("{}", e);
        // "Stammkapital von €24.999,00 ist unzureichend für GmbH.
        //  Erforderlich: €25.000,00 (§5 Abs. 1 GmbHG)"
    }
    Ok(()) => println!("✅ Gültig"),
}
```

### Builder-Muster

```rust
use legalis_de::bgb::unerlaubte_handlungen::*;

let claim = TortClaim823_1Builder::new()
    .plaintiff("Max Mustermann".to_string())
    .defendant("Unfall GmbH".to_string())
    .protected_interest(ProtectedInterest::Body)
    .unlawful_act("Fahrlässiger Verkehrsunfall".to_string())
    .fault(Verschulden::GrosseNachlaessigkeit)
    .damage_amount(Capital::from_euros(15_000))
    .build()?;
```

## Rechtsgenauigkeit

**Wichtig**: Diese Bibliothek dient zu Bildungs- und Entwicklungszwecken. Sie ersetzt **keine** Rechtsberatung.

### Quellen

Alle Implementierungen basieren auf:
- Offiziellen Gesetzestexten (dejure.org, gesetze-im-internet.de)
- BGH-Rechtsprechung (bundesgerichtshof.de)
- BVerfG-Entscheidungen (bundesverfassungsgericht.de)
- Rechtswissenschaftlicher Fachliteratur

### Artikelreferenzen

Jeder Fehler enthält präzise Gesetzesreferenzen:

```rust
pub fn article_reference(&self) -> &'static str {
    match self {
        GmbHError::InsufficientCapital { .. } => "§5 Abs. 1 GmbHG",
        GmbHError::InvalidCompanyName { .. } => "§4 GmbHG",
        // ...
    }
}
```

## Projektstruktur

```
jurisdictions/de/
├── src/
│   ├── gmbhg/          # GmbH-Gesetz
│   ├── hgb/            # Handelsgesetzbuch
│   ├── aktg/           # Aktiengesetz
│   ├── bgb/            # Bürgerliches Gesetzbuch
│   │   ├── schuldrecht/           # Vertragsrecht
│   │   ├── unerlaubte_handlungen/ # Deliktsrecht
│   │   ├── sachenrecht/           # Sachenrecht
│   │   ├── familienrecht/         # Familienrecht
│   │   └── erbrecht/              # Erbrecht
│   ├── grundgesetz/    # Verfassungsrecht
│   └── arbeitsrecht/   # Arbeitsrecht
├── examples/           # 20+ Arbeitsbeispiele
├── tests/             # 365 Tests
├── docs/              # Dokumentation (DE/EN)
└── TODO.md            # Projekt-Roadmap
```

## Beispiele

Alle Beispiele können ausgeführt werden mit:

```bash
# GmbH-Gründung
cargo run --example gmbh-formation-valid

# Vertragsbildung
cargo run --example contract-formation

# Deliktshaftung
cargo run --example tort-claim-823-1

# Arbeitsvertrag
cargo run --example employment-contract-validation

# Tarifvertrag
cargo run --example collective-bargaining-agreement

# Aufsichtsrat
cargo run --example supervisory-board-codetermination
```

## Tests

```bash
# Alle Tests ausführen
cargo nextest run

# Spezifisches Modul
cargo test gmbhg_validation_tests

# Mit Coverage
cargo tarpaulin --out Html
```

## Qualitätsmetriken

- **Tests**: 365 Tests (365 bestanden, 0 fehlgeschlagen)
- **Warnungen**: 0 (cargo clippy --all-targets)
- **Codezeilen**: ~25.000 (Quelle + Tests + Beispiele)
- **Dokumentation**: 100% öffentliche API dokumentiert
- **Gesetze**: 20+ deutsche Gesetze abgedeckt

## Beitragen

Contributions willkommen! Bitte beachten:

1. **Rechtsgenauigkeit**: Alle Änderungen mit offiziellen Quellen belegen
2. **Tests**: Neue Funktionen benötigen Tests
3. **Dokumentation**: Deutsche und englische Dokumentation aktualisieren
4. **Keine Warnungen**: `cargo clippy` muss sauber durchlaufen

## Lizenz

Siehe LICENSE-Datei im Hauptverzeichnis.

## Haftungsausschluss

Diese Software dient zu Bildungs- und Entwicklungszwecken. Sie stellt keine Rechtsberatung dar und ersetzt nicht die Konsultation eines Rechtsanwalts. Die Autoren übernehmen keine Haftung für rechtliche Konsequenzen aus der Verwendung dieser Software.

## Weiterführende Ressourcen

- [API-Dokumentation](API.md)
- [Beispiele](../examples/)
- [Projekt-Roadmap](../TODO.md)
- [Changelog](../CHANGELOG.md)
