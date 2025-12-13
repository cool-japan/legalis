# legalis-i18n

Internationalization support for Legalis-RS.

## Overview

`legalis-i18n` provides comprehensive internationalization support for legal documents, including locale management, jurisdiction registries, cultural parameters, and legal term translation.

## Features

- **Locale Support**: Language, country, and region handling
- **Jurisdiction Registry**: Legal system types and cultural parameters
- **Legal Dictionary**: Term translation between languages
- **Translation Manager**: Multi-language content management
- **Cultural Parameters**: Age of majority, religious observances, calendar systems

## Usage

```rust
use legalis_i18n::{Locale, JurisdictionRegistry, LegalDictionary, TranslationManager};

// Create a locale
let locale = Locale::new("ja", "JP", Some("Kanto"));

// Get jurisdiction info
let registry = JurisdictionRegistry::default();
let japan = registry.get("JP")?;
println!("Legal system: {:?}", japan.legal_system); // CivilLaw
println!("Age of majority: {}", japan.cultural_params.age_of_majority); // 18

// Translate legal terms
let dictionary = LegalDictionary::new();
dictionary.add_term("statute", "en", "statute");
dictionary.add_term("statute", "ja", "法令");
dictionary.add_term("statute", "de", "Gesetz");

let japanese_term = dictionary.translate("statute", "en", "ja")?;
// Returns "法令"
```

## Jurisdiction Types

| Type | Description |
|------|-------------|
| `CommonLaw` | Case law precedent system (UK, US, AU) |
| `CivilLaw` | Codified law system (JP, DE, FR) |
| `ReligiousLaw` | Religion-based legal system |
| `CustomaryLaw` | Traditional/tribal law |
| `MixedSystem` | Combination of systems |

## Cultural Parameters

```rust
pub struct CulturalParams {
    pub age_of_majority: u8,
    pub calendar_system: CalendarSystem,
    pub religious_observances: Vec<String>,
    pub official_languages: Vec<String>,
}
```

## Supported Locales

- English (en-US, en-GB, en-AU)
- Japanese (ja-JP)
- German (de-DE)
- French (fr-FR)
- Spanish (es-ES)
- Chinese (zh-CN, zh-TW)

## License

MIT OR Apache-2.0
