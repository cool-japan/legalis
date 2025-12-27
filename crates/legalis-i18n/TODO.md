# legalis-i18n TODO

## Completed

- [x] Locale support with language/country/region
- [x] Jurisdiction registry with legal system types
- [x] Cultural parameters (age of majority, religious observances)
- [x] Legal dictionary for term translation
- [x] Translation manager for multi-language support
- [x] Translation caching for improved performance
- [x] Display trait implementations for all enums (LegalSystem, PluralCategory, CalendarSystem, DayOfWeek, Locale)
- [x] Locale validation helpers (is_valid_language_code, is_valid_country_code, is_valid_script_code, is_valid_locale_tag)
- [x] Legal term abbreviation support with bidirectional lookup
- [x] Context-aware translation support for disambiguating terms
- [x] Dictionary import/export functionality (JSON serialization with custom serde)
- [x] Ordinal number formatting for legal citations (1st, 2nd, 3rd, etc.) in multiple languages
- [x] Number-to-words conversion for legal documents (English, Japanese, Spanish, French, German)
- [x] Locale-aware text collation/sorting with TextCollator
- [x] Text normalization for accent handling (German umlauts, French/Spanish accents)
- [x] Helper functions (common_legal_locales, suggest_best_locale, normalize_locale_string)
- [x] Dictionary merge functionality for combining translations
- [x] Dictionary statistics methods (translation_count, abbreviation_count, etc.)

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
- [x] Time zone support for international legal deadlines
  - [x] TimeZone struct with UTC offset and DST support
  - [x] TimeZoneRegistry with common legal time zones (18 zones)
  - [x] UTC/local time conversion
  - [x] Timezone conversion between jurisdictions
- [x] Legal citation formatting
  - [x] Bluebook style (United States)
  - [x] OSCOLA style (United Kingdom)
  - [x] AGLC style (Australia)
  - [x] McGill Guide style (Canada)
  - [x] European citation style
  - [x] Japanese legal citation
  - [x] Case and statute citation support
- [x] Deadline calculator with business days and timezone awareness
  - [x] Business day calculation per jurisdiction
  - [x] Timezone-aware deadline computation
  - [x] Deadline expiration checking
- [x] RTL (Right-to-Left) text support for Arabic/Hebrew
  - [x] Text direction detection (LTR/RTL)
  - [x] Unicode bidirectional formatting characters
  - [x] Eastern Arabic numeral conversion (٠١٢٣٤٥٦٧٨٩)
  - [x] Persian numeral conversion (۰۱۲۳۴۵۶۷۸۹)
  - [x] RTL date formatting
  - [x] Paragraph and list formatting with direction
  - [x] Mixed bidirectional text handling

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

- [x] Integration with external translation services
- [x] Machine translation fallback
- [x] Translation memory support
- [x] Terminology extraction from statutes

## Testing

- [x] Add translation roundtrip tests
- [x] Test all supported locales
- [x] Verify cultural parameter accuracy
- [x] Test external translation service integration
- [x] Test machine translation fallback
- [x] Test translation memory operations
- [x] Test terminology extraction
- [x] Test timezone UTC/local conversions
- [x] Test timezone registry and jurisdiction mapping
- [x] Test deadline calculator with business days
- [x] Test citation formatting (all 6 styles)
- [x] Test RTL text direction detection
- [x] Test Arabic and Persian numeral conversion
- [x] Test bidirectional text formatting
- [x] Test RTL paragraph and list formatting

## Additional Features (Extended)

- [x] Personal name formatting per culture
  - [x] Western name order (Given Middle Family)
  - [x] East Asian name order (Family Given) - Japanese, Korean, Chinese
  - [x] Russian names with patronymic
  - [x] Arabic names with patronymic
  - [x] Name citation formatting (Family, Given Middle)
  - [x] Name initials formatting
  - [x] Formal name formatting with titles
- [x] Address formatting per jurisdiction
  - [x] US address format (Street, City, State ZIP, Country)
  - [x] UK address format (Street, City, County, Postcode, Country)
  - [x] Japanese address format (〒Postal, Prefecture City Street Building)
  - [x] European address format (Street, Postal City, Country)
  - [x] Chinese address format (Country State City Street Building Postal)
  - [x] Korean address format
  - [x] Single-line address formatting for forms

## Summary

**Total Tests:** 112 unit tests + 15 doc tests = 127 tests
**Test Coverage:** All features fully tested with no warnings or errors
**Clippy:** No warnings (clean)
**Lines of Code:** 7,170 lines (comprehensive legal i18n support)
