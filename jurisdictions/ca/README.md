# legalis-ca

Canada / Le Canada - Legal System Support for Legalis-RS

**Version 0.1.3** - Charter of Rights, Federal/Provincial Law, Quebec Civil Law

## Overview / Aperçu

`legalis-ca` provides comprehensive support for the Canadian legal system within the Legalis-RS framework. Canada has a unique bijural system with common law in most provinces and civil law in Quebec.

## Canadian Legal System / Système juridique canadien

The Canadian legal system is characterized by:
- **Bijural system** - Common law (9 provinces + 3 territories) and Civil law (Quebec)
- **Federal structure** - Division of powers between federal and provincial governments
- **Constitutional supremacy** - Charter of Rights and Freedoms (1982)
- **Bilingual** - English and French as official languages
- **Indigenous rights** - Section 35 recognition of Aboriginal and treaty rights

### Comparison with Other Legal Systems

| Feature | Canada | USA | UK | France |
|---------|--------|-----|-----|--------|
| Legal Family | Bijural (Common/Civil) | Common Law | Common Law | Civil Law |
| Main Source | Case Law & Codes | Case Law | Case Law | Codes |
| Constitution | 1867/1982 | 1787 | Uncodified | 1958 |
| Charter/Bill of Rights | Charter 1982 | Bill of Rights | HRA 1998 | DDHC 1789 |
| Court System | Supreme Court → Provincial/Federal | Federal & State | Supreme Court | Conseil d'État |

## Implemented Features / Fonctionnalités implémentées

### ✅ Constitution / La Constitution

Constitution Act, 1867 and Constitution Act, 1982
- ✅ Division of powers (ss 91-92)
- ✅ Charter of Rights and Freedoms
  - Fundamental freedoms (s 2)
  - Democratic rights (ss 3-5)
  - Mobility rights (s 6)
  - Legal rights (ss 7-14)
  - Equality rights (s 15)
  - Language rights (ss 16-23)
- ✅ Section 1 limitations (Oakes test)
- ✅ Section 33 notwithstanding clause
- ✅ Section 35 Aboriginal rights

```rust
use legalis_ca::constitution::{CharterRight, validate_charter_claim, OakesTest};

let claim = CharterClaim::new()
    .right(CharterRight::FreedomOfExpression) // s 2(b)
    .claimant("Individual")
    .government_action("Legislation restricting speech")
    .build()?;

// Apply section 1 analysis (Oakes test)
let oakes = OakesTest::new()
    .pressing_objective(true)
    .rational_connection(true)
    .minimal_impairment(false) // Fails here
    .proportionality(true)
    .build()?;

assert!(!oakes.is_justified());
```

### ✅ Contract Law / Droit des contrats

Common law contracts (and Quebec Civil Code)
- ✅ Formation (offer, acceptance, consideration)
- ✅ Interpretation principles
- ✅ Breach and remedies
- ✅ Quebec: Civil Code of Quebec (CCQ) Book 5

```rust
use legalis_ca::contract::{Contract, Jurisdiction};

let contract = Contract::new()
    .jurisdiction(Jurisdiction::Ontario) // Common law
    .parties(vec!["Party A", "Party B"])
    .consideration(50_000.00)
    .build()?;

// For Quebec (civil law)
let quebec_contract = Contract::new()
    .jurisdiction(Jurisdiction::Quebec) // Civil law (CCQ)
    .parties(vec!["Partie A", "Partie B"])
    .cause("Lawful cause") // Quebec uses "cause" not "consideration"
    .build()?;
```

### ✅ Employment Law / Droit du travail

Federal and provincial employment standards
- ✅ Canada Labour Code (federal employees)
- ✅ Provincial employment standards
- ✅ Common law reasonable notice
- ✅ Wrongful dismissal
- ✅ Human rights protections

```rust
use legalis_ca::employment::{EmploymentContract, Termination};

let termination = Termination::new()
    .employee_tenure_years(10)
    .position("Manager")
    .age(55)
    .build()?;

// Calculate reasonable notice (Bardal factors)
let notice_months = termination.calculate_reasonable_notice()?;
// Considers: length of service, age, position, availability of similar employment
```

### ✅ Corporate Law / Droit des sociétés

Canada Business Corporations Act (CBCA) and provincial equivalents
- ✅ Corporation types (federal vs provincial)
- ✅ Directors' duties (fiduciary, care)
- ✅ Shareholder remedies (oppression, derivative)
- ✅ Unanimous shareholder agreements

### ✅ Criminal Law / Droit pénal

Criminal Code of Canada (federal jurisdiction)
- ✅ Offence classifications (summary, indictable, hybrid)
- ✅ Charter protections in criminal matters
- ✅ Sentencing principles

### ✅ Tort Law / Responsabilité civile

- ✅ Negligence (common law provinces)
- ✅ Quebec civil liability (CCQ arts 1457-1481)
- ✅ Defamation

### ✅ Family Law / Droit de la famille

- ✅ Divorce Act (federal)
- ✅ Provincial family property regimes
- ✅ Child support guidelines
- ✅ Quebec: Family Patrimony

### ✅ Property Law / Droit des biens

- ✅ Land titles (Torrens) vs registry systems
- ✅ Indigenous land rights
- ✅ Quebec: real rights (CCQ)

## 📊 Current Implementation Status

**Version 0.1.3 Statistics:**
- ✅ **Modules**: 11 modules (constitution, contract, corporate, criminal, employment, family, property, tort, common, reasoning)
- ✅ **Charter Analysis**: Full section 1 (Oakes test) implementation
- ✅ **Bijural Support**: Common law and Quebec civil law
- ✅ **Bilingual**: English and French support

## Dependencies / Dépendances

- `legalis-core` - Core types and traits
- `legalis-i18n` - Internationalization support
- `legalis-verifier` - Validation framework
- `legalis-sim` - Simulation support
- `chrono` - Date/time handling
- `serde` - Serialization
- `thiserror` - Error handling

## License / Licence

MIT OR Apache-2.0

## Related Links / Liens connexes

- [Justice Laws Website / Site Web de la législation](https://laws-lois.justice.gc.ca/)
- [Supreme Court of Canada / Cour suprême du Canada](https://www.scc-csc.ca/)
- [CanLII](https://www.canlii.org/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
