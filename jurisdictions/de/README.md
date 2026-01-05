# legalis-de

Deutsche Rechtsordnung-Unterstützung für Legalis-RS

## Überblick

`legalis-de` bietet Unterstützung für das deutsche Rechtssystem im Legalis-RS-Framework, einschließlich wichtiger Gesetze wie BGB, StGB und Grundgesetz.

## Funktionen

### BGB (Bürgerliches Gesetzbuch)

Das deutsche Zivilrecht basiert auf dem BGB von 1900. Dieser Crate bietet strukturierte Implementierungen wichtiger Paragraphen:

- **§ 823 Abs. 1 BGB** - Schadensersatzpflicht bei Rechtsgutsverletzung
- **§ 823 Abs. 2 BGB** - Schutzgesetzverletzung
- **§ 826 BGB** - Sittenwidrige vorsätzliche Schädigung

```rust
use legalis_de::{bgb_823_1, bgb_823_2, bgb_826};

let bgb_823_1 = bgb_823_1();
assert_eq!(bgb_823_1.number, "823");
```

### StGB (Strafgesetzbuch)

Unterstützung für das deutsche Strafrecht (in Entwicklung).

### Grundgesetz (GG)

Das Grundgesetz für die Bundesrepublik Deutschland (in Entwicklung).

## Rechtssystem-Merkmale

Das deutsche Recht gehört zum **zivilrechtlichen Rechtssystem (Civil Law)** und weist folgende Merkmale auf:

- **Kodifizierung**: Gesetzbücher als Hauptrechtsquelle
- **Deduktives Denken**: Vom Gesetz zum Einzelfall
- **Gesetzesvorrang**: Gesetze haben Vorrang vor Präzedenzfällen

### Vergleich mit Common Law

| Merkmal | Zivilrecht (Deutschland) | Common Law (USA) |
|---------|-------------------------|------------------|
| Hauptquelle | Gesetzbücher | Präzedenzfälle |
| Rolle der Gerichte | Gesetzesanwendung | Rechtsschöpfung |
| Denkweise | Deduktiv (Gesetz → Fall) | Analogisch (Fall → Fall) |
| Bindungswirkung | Gesetzestext | Stare decisis |
| Flexibilität | Niedrig (Gesetzgeber muss ändern) | Hoch (Gerichte unterscheiden) |

## BGB-Struktur

Das BGB ist in fünf Bücher unterteilt:

1. **Allgemeiner Teil** - Grundlegende Rechtsbegriffe
2. **Schuldrecht** - Vertragsrecht und Deliktsrecht (§§ 823-826)
3. **Sachenrecht** - Eigentumsrecht
4. **Familienrecht** - Ehe, Verwandtschaft
5. **Erbrecht** - Nachfolge bei Tod

## Abhängigkeiten

- `legalis-core` - Kerntypen und Traits
- `serde` - Serialisierung
- `chrono` - Datums-/Zeitbehandlung

## Lizenz

MIT OR Apache-2.0

## Links

- [Gesetze im Internet](https://www.gesetze-im-internet.de/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
