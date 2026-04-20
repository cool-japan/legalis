# multilingual-statute

This example demonstrates multilingual statute display and legal terminology localisation using `legalis-i18n`. Pre-built legal dictionaries for English (US), Japanese, German, French, Spanish, and Chinese (Simplified) are loaded into a `TranslationManager`. The example then showcases locale creation for 20+ jurisdictions, legal-term translation across languages, citation formatting in Bluebook (US), Japanese, and OSCOLA (UK) styles, and cultural parameter lookup (age of majority, protected classes) per jurisdiction.

## Usage

```sh
cargo run -p multilingual-statute --all-features
```

## What It Demonstrates

- `TranslationManager` with six pre-built legal dictionaries
- Locale tags (e.g., `ja-JP`, `en-US`, `de-DE`) for 20+ jurisdictions
- Legal terminology translation across Japanese, German, French, Spanish, and Chinese
- Citation formatting: Bluebook, Japanese style, and OSCOLA
- Cultural parameters: age of majority and protected classes by country
- `LegalSystem` and `Jurisdiction` type integration

Part of [Legalis-RS](https://github.com/cool-japan/legalis)
