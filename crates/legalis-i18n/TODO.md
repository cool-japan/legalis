# legalis-i18n TODO

## Completed

- [x] Locale support with language/country/region
- [x] Jurisdiction registry with legal system types
- [x] Cultural parameters (age of majority, religious observances)
- [x] Legal dictionary for term translation
- [x] Translation manager for multi-language support

## Content

- [x] Add comprehensive legal term dictionaries
  - [x] English (en-US, en-GB)
  - [x] Japanese (ja-JP)
  - [x] German (de-DE)
  - [x] French (fr-FR)
  - [x] Spanish (es-ES)
  - [x] Chinese (zh-CN, zh-TW)
- [x] Create jurisdiction-specific legal glossaries
  - [x] Japan (民法, 戸籍, 株式会社, etc.)
  - [x] United States (due process, Supreme Court, class action, etc.)
  - [x] United Kingdom (barrister, freehold, trust, etc.)
  - [x] Germany (BGB, Bundesgerichtshof, etc.)
  - [x] France (Code civil, Cour de cassation, etc.)
  - [x] China (人民法院, 检察院, etc.)
- [x] Add Latin legal term translations
- [x] Create mapping between civil and common law concepts

## Features

- [x] ICU message format support
- [x] Plural rules handling for different languages
- [x] Date/time localization for legal deadlines
- [x] Currency formatting for monetary values
- [x] Number formatting per locale

## Locale Support

- [x] Add more jurisdiction profiles (20 jurisdictions: JP, US, GB, DE, FR, ES, IT, CN, TW, KR, CA, AU, IN, BR, RU, SA, NL, CH, MX, SG)
- [x] Support for regional variations
  - [x] English variations (US, GB, AU, CA)
  - [x] Spanish variations (ES, MX, AR)
  - [x] Chinese variations (CN-Hans, TW-Hant, HK-Hant)
  - [x] German variations (DE, AT, CH)
  - [x] French variations (FR, CA, BE)
  - [x] Locale matching and fallback chains
  - [x] Parent locale resolution
- [x] Calendar system conversions
- [x] Working day calculations per jurisdiction

## Integration

- [ ] Integration with external translation services
- [ ] Machine translation fallback
- [ ] Translation memory support
- [ ] Terminology extraction from statutes

## Testing

- [x] Add translation roundtrip tests
- [x] Test all supported locales
- [x] Verify cultural parameter accuracy
