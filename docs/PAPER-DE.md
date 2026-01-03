# Legalis-RS: Die Architektur der Generativen Rechtswissenschaft

## Trennung von Recht und Narrativ: Ein Entwurf für "Governance as Code"

---

**Autoren**: Legalis-RS Entwicklungsteam
**Version**: 0.2.0
**Sprache**: Rust (Edition 2024)
**Lizenz**: MIT / Apache 2.0

---

## Zusammenfassung

Dieser Artikel präsentiert **Legalis-RS**, ein Rust-Framework zur rigorosen Trennung und Strukturierung von Rechtsdokumenten in natürlicher Sprache in **deterministische Logik (Code)** und **richterliches Ermessen (Narrativ)**.

Moderne Rechtssysteme enthalten eine Mischung aus Bereichen, die für Computerautomatisierung geeignet sind (Altersanforderungen, Einkommensgrenzen, Fristberechnungen), und Bereichen, die menschliche Interpretation und Urteilskraft erfordern ("wichtiger Grund", "gute Sitten"). Frühere Ansätze haben diese Grenze entweder mehrdeutig gelassen oder versucht, durch übermäßige Automatisierung alles berechenbar zu machen.

Legalis-RS führt einen dreiwertige Logiktyp `LegalResult<T>` ein, der Rusts Typsystem nutzt, um diese Grenze auf Typebene explizit zu machen. Dies ermöglicht ein neues Paradigma für rechtliches Debugging, Simulation und internationales Portieren, während "algorithmische Autokratie" im KI-Zeitalter verhindert wird.

**Wesentliche Technische Beiträge**:
1. Implementierung einer rechtlichen domänenspezifischen Sprache (DSL) und Parser
2. Formale Verifikation mit dem Z3 SMT-Solver
3. ECS-artiger Simulationsmotor für die Vorhersage sozialer Auswirkungen
4. Smart-Contract-Generierung für über 25 Blockchain-Plattformen
5. Linked Open Data (RDF/TTL) Integration für das Semantische Web
6. Rechtssystemimplementierungen für 4 Länder mit kultureller Parameteranpassung (Soft ODA)

**Kernphilosophie**: *"Nicht alles sollte berechenbar sein."*

---

## 1. Einleitung

### 1.1 Hintergrund: Die Beziehung zwischen Recht und Berechnung

Lawrence Lessigs berühmte These "Code is Law" wies darauf hin, dass Architektur (Code) im Cyberspace eine dem Recht gleichwertige Regulierungsmacht hat. Legalis-RS kehrt dies jedoch um und verfolgt einen Ansatz von "**Law becomes Code**" (Recht wird Code).

Die Kodifizierung von Recht bietet folgende Vorteile:

- **Verifizierbarkeit**: Logische Widersprüche zur Kompilierzeit erkennen
- **Simulation**: Soziale Auswirkungen vor der Durchsetzung vorhersagen
- **Interoperabilität**: Zwischen verschiedenen Rechtssystemen konvertieren und vergleichen
- **Transparenz**: Vollständige Prüfpfade der rechtlichen Entscheidungsprozesse

Jedoch ist es sowohl philosophisch als auch praktisch gefährlich, alle Gesetze berechenbar zu machen. Recht enthält inhärent Bereiche, die "menschliches Urteil" erfordern, und Automatisierung, die dies ignoriert, kann zu "KI-Autokratie" führen.

### 1.2 Problemstellung: Herausforderungen der Rechtsverarbeitung im KI-Zeitalter

Moderne Rechtstechnologie (LegalTech) steht vor mehreren grundlegenden Herausforderungen:

1. **Mehrdeutigkeitshandhabung**: Viele Rechtsbegriffe sind absichtlich vage und setzen eine Einzelfallinterpretation voraus
2. **Kontextabhängigkeit**: Dieselbe Vorschrift kann je nach sozialem und kulturellem Kontext unterschiedlich interpretiert werden
3. **Zeitliche Veränderung**: Gesetze werden geändert und aufgehoben, was Konsistenzmanagement über die Zeit erfordert
4. **Internationale Unterschiede**: Die Rechtssysteme jedes Landes unterscheiden sich in ihren philosophischen Grundlagen

### 1.3 Vorschlag: Trennung von Berechenbarkeit und richterlichem Ermessen

Der Kern von Legalis-RS ist die Einführung dreiwertige Logik durch den Typ `LegalResult<T>`:

```rust
pub enum LegalResult<T> {
    /// [Deterministischer Bereich] Automatisch verarbeitbare rechtliche Ergebnisse
    Deterministic(T),

    /// [Ermessensbereich] Bereich, der menschliches Urteil erfordert
    JudicialDiscretion {
        issue: String,           // Die vorliegende Frage
        context_id: Uuid,        // Kontextdaten
        narrative_hint: Option<String>, // Referenzmeinung durch LLM
    },

    /// [Logischer Zusammenbruch] Fehler im Gesetz selbst
    Void { reason: String },
}
```

Dieser Typ garantiert, dass das Ergebnis der Rechtsverarbeitung immer in eine der drei Kategorien klassifiziert wird. Das System stoppt die Verarbeitung beim Erreichen von `JudicialDiscretion` und delegiert das Urteil an Menschen. Dies wird zum "Bollwerk auf Typebene" gegen KI-Autokratie.

---

## 2. Verwandte Arbeiten

### 2.1 Geschichte des Computational Law

Die Beziehung zwischen Recht und Computern geht zurück auf das LARC-Projekt (Legal Analysis and Research Computer) in den 1950er Jahren. Sie hat sich seitdem durch Expertensysteme, regelbasierte Systeme und moderne Machine-Learning-Ansätze entwickelt.

| Ära | Technologie | Merkmale |
|-----|-------------|----------|
| 1950er | LARC | Erstes rechtliches Informationsabrufsystem |
| 1970er | MYCIN-artige Expertensysteme | Regelbasiertes Schlussfolgern |
| 1980er | HYPO | Fallbasiertes Schlussfolgern |
| 1990er | XML/SGML-Standardisierung | Strukturierung von Rechtsdokumenten |
| 2000er | Semantisches Web | Ontologiebasierte rechtliche Wissensrepräsentation |
| 2010er | Maschinelles Lernen | Rechtliche Vorhersagemodelle |
| 2020er | LLM + Formale Verifikation | Hybridansatz |

### 2.2 Bestehende Rechtliche DSLs

#### Catala (Inria, Frankreich)
- **Merkmale**: Literate Programming, scope-basiert, starke Typisierung
- **Einschränkungen**: Keine explizite Markierung von Ermessensbereichen

#### L4 (Singapur)
- **Merkmale**: Deontische Logik (MUST/MAY/SHANT), regelbasiertes Schlussfolgern
- **Einschränkungen**: Keine Simulationsfunktionalität

#### Stipula (Universität Bologna, Italien)
- **Merkmale**: Smart-Contract-orientiert, Zustandsmaschinen, Partei/Vermögenswert-Modell
- **Einschränkungen**: Keine formale Verifikation

---

## 3. Philosophie und Designprinzipien

### 3.1 "Governance as Code, Justice as Narrative"

Der Slogan von Legalis-RS spiegelt den wesentlichen Unterschied zwischen Governance und Gerechtigkeit wider:

- **Governance**: Regelanwendung, Verfahrenskonformität, Anspruchsbestimmung → **Kodifizierbar**
- **Gerechtigkeit**: Verwirklichung von Billigkeit, kontextuelle Interpretation, Werturteil → **Als Narrativ erzählt**

### 3.2 Design der Dreiwertigen Logik

Die drei Werte von `LegalResult<T>` entsprechen den folgenden rechtsphilosophischen Konzepten:

| Typ | Rechtsphilosophisches Konzept | Verarbeitungsinstanz |
|-----|-------------------------------|----------------------|
| `Deterministic(T)` | Mechanisch anwendbare Regeln | Computer |
| `JudicialDiscretion` | Interpretationsbedürftige Prinzipien | Mensch |
| `Void` | Rechtslücken/Widersprüche | Gesetzgeber (Korrektur erforderlich) |

### 3.3 "Nicht alles sollte berechenbar sein"

Gegen die Versuchung, alles berechenbar zu machen, sagt Legalis-RS klar "Nein". Folgende Bereiche sind absichtlich als nicht-berechenbar konzipiert:

1. **Wichtiger Grund**
2. **Öffentliche Ordnung und gute Sitten**
3. **Treu und Glauben**
4. **Angemessenheit**

---

## 4. Systemarchitektur

### 4.1 Überblick über die 7-Schichten-Architektur

```
┌─────────────────────────────────────────────────────────┐
│                  Infrastrukturschicht                    │
│              (legalis-audit, legalis-api, legalis-cli)  │
├─────────────────────────────────────────────────────────┤
│                     Ausgabeschicht                       │
│         (legalis-viz, legalis-chain, legalis-lod)       │
├─────────────────────────────────────────────────────────┤
│                 Interoperabilitätsschicht                │
│                    (legalis-interop)                     │
├─────────────────────────────────────────────────────────┤
│              Internationalisierungsschicht               │
│              (legalis-i18n, legalis-porting)            │
├─────────────────────────────────────────────────────────┤
│              Simulations- & Analyseschicht               │
│                (legalis-sim, legalis-diff)              │
├─────────────────────────────────────────────────────────┤
│                  Intelligenzschicht                      │
│              (legalis-llm, legalis-verifier)            │
├─────────────────────────────────────────────────────────┤
│                      Kernschicht                         │
│          (legalis-core, legalis-dsl, legalis-registry)  │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Kernschicht

#### legalis-core
Das Crate, das den philosophischen Kern des Projekts implementiert.

**Wichtige Typdefinitionen**:
- `LegalResult<T>`: Dreiwertige Logiktyp
- `Statute`: Grunddarstellung von Gesetzen
- `Condition`: Bedingungsausdrücke (AND/OR/NOT, Alter, Einkommen usw.)
- `Effect`: Rechtliche Wirkungen (Grant/Revoke/Obligation/Prohibition)

#### legalis-dsl
Parser für die rechtliche domänenspezifische Sprache.

**DSL-Syntaxbeispiel**:
```
STATUTE adult-voting: "Wahlrecht für Erwachsene" {
    JURISDICTION "DE"
    VERSION 2
    EFFECTIVE_DATE 2024-01-01

    WHEN AGE >= 18 AND HAS citizen
    THEN GRANT "Wahlrecht"

    EXCEPTION WHEN HAS disqualified
    DISCRETION "Die Feststellung der geistigen Kapazität erfordert ein ärztliches Gutachten"
}
```

### 4.3 Intelligenzschicht

#### legalis-llm
LLM-Anbieter-Abstraktionsschicht.

**Unterstützte Anbieter**: OpenAI, Anthropic, Google Gemini, Lokale LLM

#### legalis-verifier
Motor für formale Verifikation mit Z3 SMT-Solver-Integration.

**Verifikationsziele**:
- Erkennung zirkulärer Verweise
- Erkennung unerreichbarer Gesetze
- Erkennung logischer Widersprüche
- Prüfung von Verfassungskonflikten

### 4.4 Simulationsschicht

#### legalis-sim
ECS-artiger Simulationsmotor.

**Funktionen**:
- Populationsbasierte Simulation (unterstützt Millionen von Agenten)
- Monte-Carlo-Simulation
- Sensitivitätsanalyse
- A/B-Tests
- GPU-Beschleunigung (CUDA/OpenCL/WebGPU)

### 4.5 Ausgabeschicht

#### legalis-chain
Smart-Contract-Generierung.

**Unterstützte Plattformen (25+)**:
- EVM: Solidity, Vyper
- Substrate: Ink!
- Move: Aptos, Sui
- StarkNet: Cairo
- Cosmos: CosmWasm

**Einschränkung**: Nur `Deterministic` kann konvertiert werden (`JudicialDiscretion` kann nicht konvertiert werden)

#### legalis-lod
Linked Open Data Ausgabe.

**Unterstützte Ontologien**: ELI, FaBiO, LKIF-Core, Akoma Ntoso, Dublin Core, SKOS

**RDF-Formate**: Turtle, N-Triples, RDF/XML, JSON-LD, TriG

---

## 5. Kerntechnologien

### 5.1 Rechtliche DSL

**Grundstruktur**:
```
STATUTE <id>: "<titel>" {
    [JURISDICTION "<zuständigkeit>"]
    [VERSION <nummer>]
    [EFFECTIVE_DATE <datum>]

    WHEN <bedingung>
    THEN <wirkung>

    [EXCEPTION WHEN <bedingung>]
    [DISCRETION "<beschreibung>"]
}
```

### 5.2 LegalResult<T>-Typ und Partielle Wahrheitswerte

Die Bedingungsauswertung verwendet die 4-wertige Logik `PartialBool`:

```rust
pub enum PartialBool {
    True,
    False,
    Unknown,      // Unzureichende Information
    Contradiction, // Widerspruch
}
```

### 5.3 Formale Verifikation mit dem Z3 SMT-Solver

Rechtliche Bedingungsausdrücke werden in das SMT-LIB-Format konvertiert:

```smt2
(declare-const age Int)
(declare-const income Int)
(declare-const has_citizen Bool)

(assert (and (>= age 18) has_citizen))
(check-sat)
```

---

## 6. Jurisdiktionelle Implementierungen

### 6.1 Japanisches Rechtssystem

#### Verfassung von Japan
Das legalis-jp Crate bietet eine strukturierte Darstellung der japanischen Verfassung.

#### Bürgerliches Gesetzbuch Artikel 709 (Delikt)
```
STATUTE minpo-709: "Schadensersatz bei Delikt" {
    JURISDICTION "JP"

    WHEN HAS intentional_act OR HAS negligence
    AND HAS violation_of_rights
    AND HAS causation
    AND HAS damages

    THEN OBLIGATION "Schadensersatz"

    DISCRETION "Die Feststellung der Fahrlässigkeit und die Berechnung des Schadens
                liegen im Ermessen des Gerichts"
}
```

### 6.2 Geplante Jurisdiktionen

| Jurisdiktion | Status | Schwerpunktbereiche |
|--------------|--------|---------------------|
| Deutschland (DE) | In Entwicklung | BGB, GG |
| Frankreich (FR) | In Entwicklung | Code civil, Verfassung |
| USA (US) | In Entwicklung | UCC, Verfassung, Fallrecht |

---

## 7. Fallstudien

### 7.1 System zur Bestimmung der Anspruchsberechtigung für Sozialleistungen

Automatische Anspruchsbestimmung für 6 Leistungsprogramme:
1. Grundlegende Sozialhilfe
2. Rentenzuschuss für Senioren
3. Kindergeld
4. Behindertenunterstützung
5. Notfall-Wohnungshilfe
6. Gesundheitszuschuss

**Ergebnisse**:
- Deterministische Entscheidungen: 85% der Fälle
- JudicialDiscretion: 15% der Fälle

### 7.2 Simulation des Bürgerlichen Gesetzbuches Artikel 709 (Delikt)

5 simulierte Szenarien:
1. Klares vorsätzliches Delikt → `Deterministic(Liable)`
2. Fahrlässiges Delikt → `Deterministic(Liable)`
3. Grenzfall → `JudicialDiscretion`
4. Kein Delikt → `Deterministic(NotLiable)`
5. Keine Kausalität → `Deterministic(NotLiable)`

### 7.3 Vergleichende Deliktsrechtsanalyse in 4 Ländern

| Land | Gesetz | Merkmale |
|------|--------|----------|
| Japan | BGB Art. 709 | Generalklausel (weites Ermessen) |
| Deutschland | BGB §823/§826 | Aufgezählte geschützte Interessen |
| Frankreich | Code civil Art. 1240 | Maximale Abstraktion |
| USA | Fallrecht | Typisiert (Battery usw.) |

---

## 8. API-Referenz und Technische Details

### 8.1 Wichtige Typen und Traits

```rust
// Dreiwertige Logiktyp
pub enum LegalResult<T> {
    Deterministic(T),
    JudicialDiscretion { issue: String, context_id: Uuid, narrative_hint: Option<String> },
    Void { reason: String },
}

// Rechtliche Entität Trait
pub trait LegalEntity: Send + Sync {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn attributes(&self) -> &[String];
}
```

### 8.2 REST API / GraphQL Endpunkte

| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | /api/v1/statutes | Liste der Gesetze abrufen |
| POST | /api/v1/verify | Verifikation ausführen |
| POST | /api/v1/simulate | Simulation ausführen |

### 8.3 CLI-Befehlssystem

```bash
legalis parse <datei.dsl> [--format json|yaml]
legalis verify <datei.dsl> [--strict]
legalis simulate <datei.dsl> --population 1000
legalis visualize <datei.dsl> --output tree.svg
legalis export <datei.dsl> --format solidity|catala|l4|rdf
```

---

## 9. Evaluation

### 9.1 Leistungs-Benchmarks

| Operation | Ziel | Zeit |
|-----------|------|------|
| DSL-Parsing | 100 Gesetze | 15ms |
| Verifikation | 100 Gesetze | 250ms |
| Simulation | 10.000 Agenten | 1,2s |
| Simulation | 100.000 Agenten | 8,5s |

### 9.2 Code-Qualität

- **Testabdeckung**: Integrationstests, Property-Tests, Snapshot-Tests
- **Statische Analyse**: Clippy (Null-Warnungen-Richtlinie)
- **Dokumentation**: rustdoc für alle öffentlichen APIs

---

## 10. Zukünftige Arbeiten

- Web-UI-Frontend (React)
- VS Code Erweiterung
- Jupyter Notebook Integration
- Zusätzliche Jurisdiktionen (EU-Recht, Internationales Recht)

---

## 11. Schlussfolgerung

Legalis-RS präsentiert einen neuen Ansatz zur Kodifizierung von Recht, indem die "Grenze zwischen Berechenbarkeit und menschlichem Urteil" im Typsystem explizit gemacht wird.

**Wichtige Errungenschaften**:
1. **Philosophische Grundlage**: "Governance as Code, Justice as Narrative"
2. **Typsystem**: Dreiwertige Logik über `LegalResult<T>`
3. **Integrierte Architektur**: Umfassendes Design mit 7 Schichten und 16 Crates
4. **Implementierung**: Etwa 450.000 Zeilen Rust-Code
5. **Verifikation**: Z3 SMT-Solver Integration
6. **Simulation**: ECS-artiger Motor (GPU-Beschleunigungsunterstützung)
7. **Ausgabe**: 25+ Blockchains, RDF/TTL, mehrere Formate

**Kernphilosophie**: *"Nicht alles sollte berechenbar sein."*

---

## Referenzen

1. Lessig, L. (1999). *Code and Other Laws of Cyberspace*. Basic Books.
2. Dworkin, R. (1977). *Taking Rights Seriously*. Harvard University Press.
3. Merigoux, D., Chataing, N., & Protzenko, J. (2021). Catala: A Programming Language for the Law. *ICFP 2021*.
4. de Moura, L., & Bjørner, N. (2008). Z3: An Efficient SMT Solver. *TACAS 2008*.

---

*"Code is Law", sagt man, aber wir verfolgen den Ansatz "Law becomes Code". Jedoch betten wir in diesen Code einen Typ namens 'Menschlichkeit' ein.*

---

**Legalis-RS Entwicklungsteam**
Version 0.2.0 | 2024
