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
**Lines of Code:** 8,692 lines (comprehensive legal i18n support)
**Features Implemented:**
- 5 new languages (Korean, Portuguese, Italian, Dutch, Polish)
- 5 new citation styles (Harvard, APA, Chicago, Indian, Custom templates)
- 6 calendar systems (Gregorian, Japanese, Buddhist, Islamic, Hebrew, Persian)
- 8 specialized legal domains (IP, Tax, Environmental, Labor, Corporate, Criminal, Civil Procedure, General)
- 73+ specialized legal terms across multiple languages
- Fiscal year calculations for 10+ jurisdictions
- Complete formatting suite (document numbering, footnotes, cross-refs, TOC, index)
- Advanced deadline calculator with statute of limitations, grace periods, conflict detection

## Recent Enhancements (Latest Session)

### v0.1.1: Additional Languages
- Added Korean (ko-KR) number-to-words and ordinal formatting
- Added Portuguese (pt-BR, pt-PT) number-to-words and ordinal formatting
- Added Italian (it-IT) number-to-words and ordinal formatting
- Added Dutch (nl-NL) number-to-words and ordinal formatting
- Added Polish (pl-PL) number-to-words and ordinal formatting

### v0.1.2: Citation Styles
- Added Harvard citation style for cases and statutes
- Added APA legal citation style for cases and statutes
- Added Chicago Manual of Style (legal) for cases and statutes
- Added Indian legal citation style for cases and statutes
- Added Custom citation template support with variable substitution
- Updated CitationStyle enum to support Custom(String) variant

### v0.1.5: Formatting Extensions
- Added DocumentNumbering with 5 styles (Article, Section, Chapter, Hierarchical, Parenthetical)
- Added FootnoteFormatter with 3 styles (Numeric, Symbol, Letter)
- Added CrossReferenceFormatter with multi-language support
- Added TableOfContents generator with hierarchical entries
- Added IndexGenerator with sub-entries and alphabetical sorting

### v0.1.6: Deadline Calculator Extensions
- Added statute_of_limitations calculator
- Added apply_holiday_rollover for non-working day adjustment
- Added add_grace_period for deadline extensions
- Added has_deadline_conflict for detecting conflicting deadlines
- Added helper functions: add_one_day, days_between, is_leap_year

### v0.1.3: Calendar Systems
- Implemented Islamic (Hijri) calendar using Kuwaiti algorithm approximation
- Implemented Hebrew calendar conversion (simplified lunisolar approximation)
- Enhanced Japanese Imperial era calendar (Reiwa, Heisei, Showa, Taisho, Meiji)
- Implemented Persian (Solar Hijri) calendar
- Added FiscalYearConfig for jurisdiction-specific fiscal years (10 jurisdictions)
- Added Julian Day Number conversion helpers for accurate date calculations
- Added bidirectional calendar conversions (to_gregorian_from_islamic)
- Added month name formatting for Islamic and Hebrew calendars
- Implemented fiscal year calculations: get_fiscal_year, get_fiscal_year_start, get_fiscal_year_end

### v0.1.4: Specialized Legal Term Dictionaries
- Created LegalDomain enum with 8 specializations
- Implemented IP law dictionary (14 terms in English, 10 in Japanese, 5 in German)
- Implemented Tax law dictionary (14 terms in English, 8 in Japanese, 5 in German)
- Implemented Environmental law dictionary (12 terms in English, 7 in Japanese, 4 in German)
- Implemented Labor law dictionary (13 terms in English, 9 in Japanese, 5 in German)
- Implemented Corporate law dictionary (7 terms in English, 5 in Japanese)
- Implemented Criminal law dictionary (6 terms in English, 3 in Japanese)
- Implemented Civil Procedure dictionary (7 terms in English, 3 in Japanese)
- Added create_dictionary() method for domain-specific dictionary generation
- Total: 73+ specialized legal terms across multiple languages

## Roadmap for 0.1.0 Series

### Additional Languages (v0.1.1) - COMPLETED
- [x] Add Korean (ko-KR) - South Korean legal terminology
- [x] Add Portuguese (pt-BR, pt-PT) - Brazilian and Portuguese legal terms
- [x] Add Italian (it-IT) - Italian legal terminology
- [x] Add Dutch (nl-NL) - Dutch legal terminology
- [x] Add Polish (pl-PL) - Polish legal terminology

### Citation Styles (v0.1.2) - COMPLETED
- [x] Add Harvard citation style
- [x] Add APA legal citation style
- [x] Add Chicago Manual of Style (legal)
- [x] Add Indian legal citation style
- [x] Add Custom citation template support

### Calendar Systems (v0.1.3) - COMPLETED
- [x] Add Islamic (Hijri) calendar support with Kuwaiti algorithm
- [x] Add Hebrew calendar support
- [x] Add Japanese Imperial era calendar (Reiwa, Heisei, Showa, Taisho, Meiji)
- [x] Add Thai Buddhist calendar
- [x] Add Persian (Solar Hijri) calendar support
- [x] Add fiscal year calculations per jurisdiction (US, GB, JP, AU, CA, IN, EU)
- [x] Add Julian Day Number conversion helpers
- [x] Add bidirectional calendar conversions (Gregorian ↔ Islamic)

### Legal Term Dictionaries (v0.1.4) - COMPLETED
- [x] Add specialized IP law terminology (patent, trademark, copyright, trade secret)
- [x] Add specialized tax law terminology (income tax, VAT, capital gains, deductions)
- [x] Add specialized environmental law terminology (pollution, emissions, sustainability)
- [x] Add specialized labor law terminology (employment contract, wrongful termination, collective bargaining)
- [x] Add specialized corporate law terminology (M&A, due diligence, fiduciary duty)
- [x] Add specialized criminal law terminology (indictment, plea bargain, Miranda rights)
- [x] Add specialized civil procedure terminology (discovery, deposition, summary judgment)
- [x] Multi-language support for all specialized dictionaries (English, Japanese, German)

### Formatting Extensions (v0.1.5) - COMPLETED
- [x] Add legal document numbering (Article 1, Section 2, etc.)
- [x] Add footnote/endnote formatting
- [x] Add cross-reference formatting
- [x] Add table of contents generation
- [x] Add index generation

### Deadline Calculator Extensions (v0.1.6) - COMPLETED
- [x] Add statute of limitations calculator
- [x] Add holiday rollover rules
- [x] Add grace period calculations
- [x] Add deadline conflict detection

### Translation Memory (v0.1.7)
- [ ] Add persistent translation memory storage
- [ ] Add translation memory sharing between projects
- [ ] Add fuzzy match scoring for similar terms
- [ ] Add translation memory import/export (TMX format)
- [ ] Add context-aware translation suggestions

### Accessibility (v0.1.8)
- [ ] Add screen reader friendly formatting
- [ ] Add plain language alternatives
- [ ] Add reading level assessment
- [ ] Add Braille formatting support
- [ ] Add audio description generation

### Regional Variations (v0.1.9)
- [ ] Add state/province level variations (US states, Canadian provinces)
- [ ] Add EU member state variations
- [ ] Add dialect-aware terminology
- [ ] Add regional legal concept mapping
- [ ] Add cross-regional term equivalence
