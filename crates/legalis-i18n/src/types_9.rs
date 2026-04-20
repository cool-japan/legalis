//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::functions::I18nResult;
use super::types::{Address, BrailleGrade, CitationType};
use super::types_3::{
    CompletenessReport, NeuralMachineTranslator, ObligationType, PartyRole, TermEquivalence,
};
use super::types_4::{DayOfWeek, EquivalentTerm, LanguageScope, MTTranslation};
use super::types_5::CitationValidator;
use super::types_6::{CitationComponents, CitationStyle, CitationValidationRule, TimeZone};
use super::types_7::TranslationMemory;
use super::types_10::Locale;
use super::types_11::{CLDREntry, CLDRFieldType, CalendarDate, LanguageType, TranslationEngine};
use super::types_12::{EquivalenceLevel, IndexEntry, LLMProvider, LegalPromptTemplate};

/// Working days configuration for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingDaysConfig {
    /// Weekend days (non-working days)
    pub weekend: Vec<DayOfWeek>,
    /// Fixed public holidays (month, day)
    pub fixed_holidays: Vec<(u32, u32)>,
    /// Jurisdiction ID
    pub jurisdiction_id: String,
}
impl WorkingDaysConfig {
    /// Creates a new working days configuration.
    pub fn new(jurisdiction_id: impl Into<String>) -> Self {
        Self {
            weekend: vec![DayOfWeek::Saturday, DayOfWeek::Sunday],
            fixed_holidays: vec![],
            jurisdiction_id: jurisdiction_id.into(),
        }
    }
    /// Sets the weekend days.
    pub fn with_weekend(mut self, weekend: Vec<DayOfWeek>) -> Self {
        self.weekend = weekend;
        self
    }
    /// Adds a fixed holiday (month, day).
    pub fn add_holiday(mut self, month: u32, day: u32) -> Self {
        self.fixed_holidays.push((month, day));
        self
    }
    /// Creates default configuration for Japan.
    pub fn japan() -> Self {
        Self::new("JP")
            .add_holiday(1, 1)
            .add_holiday(2, 11)
            .add_holiday(2, 23)
            .add_holiday(3, 20)
            .add_holiday(4, 29)
            .add_holiday(5, 3)
            .add_holiday(5, 4)
            .add_holiday(5, 5)
            .add_holiday(8, 11)
            .add_holiday(9, 23)
            .add_holiday(11, 3)
            .add_holiday(11, 23)
    }
    /// Creates default configuration for United States.
    pub fn united_states() -> Self {
        Self::new("US")
            .add_holiday(1, 1)
            .add_holiday(7, 4)
            .add_holiday(11, 11)
            .add_holiday(12, 25)
    }
    /// Creates default configuration for United Kingdom.
    pub fn united_kingdom() -> Self {
        Self::new("GB")
            .add_holiday(1, 1)
            .add_holiday(12, 25)
            .add_holiday(12, 26)
    }
    /// Creates default configuration for Saudi Arabia (weekend: Friday-Saturday).
    pub fn saudi_arabia() -> Self {
        Self::new("SA").with_weekend(vec![DayOfWeek::Friday, DayOfWeek::Saturday])
    }
    /// Creates default configuration for Israel (weekend: Friday-Saturday).
    pub fn israel() -> Self {
        Self::new("IL").with_weekend(vec![DayOfWeek::Friday, DayOfWeek::Saturday])
    }
    /// Creates configuration for a jurisdiction code.
    pub fn for_jurisdiction(code: &str) -> Self {
        match code {
            "JP" => Self::japan(),
            "US" => Self::united_states(),
            "GB" => Self::united_kingdom(),
            "SA" => Self::saudi_arabia(),
            "IL" => Self::israel(),
            _ => Self::new(code),
        }
    }
    /// Checks if a date is a working day.
    pub fn is_working_day(&self, year: i32, month: u32, day: u32) -> bool {
        let day_of_week = self.calculate_day_of_week(year, month, day);
        if self.weekend.contains(&day_of_week) {
            return false;
        }
        if self.fixed_holidays.contains(&(month, day)) {
            return false;
        }
        true
    }
    /// Calculates the day of week using Zeller's congruence.
    pub(crate) fn calculate_day_of_week(&self, year: i32, month: u32, day: u32) -> DayOfWeek {
        let (m, y) = if month < 3 {
            (month + 12, year - 1)
        } else {
            (month, year)
        };
        let k = y % 100;
        let j = y / 100;
        let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        match (h + 5) % 7 {
            0 => DayOfWeek::Monday,
            1 => DayOfWeek::Tuesday,
            2 => DayOfWeek::Wednesday,
            3 => DayOfWeek::Thursday,
            4 => DayOfWeek::Friday,
            5 => DayOfWeek::Saturday,
            _ => DayOfWeek::Sunday,
        }
    }
    /// Adds working days to a date.
    pub fn add_working_days(
        &self,
        year: i32,
        month: u32,
        day: u32,
        working_days: i32,
    ) -> (i32, u32, u32) {
        let mut current_year = year;
        let mut current_month = month;
        let mut current_day = day;
        let mut remaining = working_days;
        while remaining > 0 {
            let (next_y, next_m, next_d) = self.next_day(current_year, current_month, current_day);
            current_year = next_y;
            current_month = next_m;
            current_day = next_d;
            if self.is_working_day(current_year, current_month, current_day) {
                remaining -= 1;
            }
        }
        (current_year, current_month, current_day)
    }
    fn next_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let days_in_month = self.days_in_month(year, month);
        if day < days_in_month {
            (year, month, day + 1)
        } else if month < 12 {
            (year, month + 1, 1)
        } else {
            (year + 1, 1, 1)
        }
    }
    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }
    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}
/// Citation completeness checker.
#[derive(Debug, Clone)]
pub struct CitationCompletenessChecker {
    style: CitationStyle,
}
impl CitationCompletenessChecker {
    /// Creates a new completeness checker.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }
    /// Checks completeness of a case citation.
    pub fn check_case(&self, components: &CitationComponents) -> CompletenessReport {
        let validator = CitationValidator::new(self.style.clone());
        let rules = validator.get_case_rules();
        self.check_against_rules(components, &rules, CitationType::Case)
    }
    /// Checks completeness of a statute citation.
    pub fn check_statute(&self, components: &CitationComponents) -> CompletenessReport {
        let validator = CitationValidator::new(self.style.clone());
        let rules = validator.get_statute_rules();
        self.check_against_rules(components, &rules, CitationType::Statute)
    }
    fn check_against_rules(
        &self,
        components: &CitationComponents,
        rules: &[CitationValidationRule],
        citation_type: CitationType,
    ) -> CompletenessReport {
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();
        let mut present = Vec::new();
        for rule in rules {
            let value = match rule.field.as_str() {
                "title" => Some(&components.title),
                "volume" => components.volume.as_ref(),
                "reporter" => components.reporter.as_ref(),
                "page" => components.page.as_ref(),
                "court" => components.court.as_ref(),
                "year" => components.year.as_ref().map(|_| &components.title),
                "jurisdiction" => components.jurisdiction.as_ref(),
                _ => None,
            };
            if value.is_none() {
                if rule.required {
                    missing_required.push(rule.field.clone());
                } else {
                    missing_optional.push(rule.field.clone());
                }
            } else {
                present.push(rule.field.clone());
            }
        }
        let total_fields = rules.len();
        let present_count = present.len();
        let completeness_score = if total_fields > 0 {
            (present_count as f64 / total_fields as f64) * 100.0
        } else {
            0.0
        };
        CompletenessReport {
            citation_type,
            style: self.style.clone(),
            completeness_score,
            missing_required,
            missing_optional,
            present,
        }
    }
}
/// Calendar converter for converting dates between calendar systems.
#[derive(Debug, Clone)]
pub struct CalendarConverter {
    locale: Locale,
}
impl CalendarConverter {
    /// Creates a new calendar converter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Converts a Gregorian date to the locale's preferred calendar.
    pub fn from_gregorian(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let system = self.get_preferred_calendar();
        match system {
            CalendarSystem::Gregorian => CalendarDate::new(system, year, month, day),
            CalendarSystem::Japanese => self.to_japanese_calendar(year, month, day),
            CalendarSystem::Buddhist => CalendarDate::new(system, year + 543, month, day),
            CalendarSystem::Islamic => self.to_islamic_approximate(year, month, day),
            CalendarSystem::Hebrew => self.to_hebrew_calendar(year, month, day),
            CalendarSystem::Persian => self.to_persian_calendar(year, month, day),
        }
    }
    fn get_preferred_calendar(&self) -> CalendarSystem {
        match self.locale.country.as_deref() {
            Some("JP") => CalendarSystem::Japanese,
            Some("TH") => CalendarSystem::Buddhist,
            Some("SA") | Some("AE") | Some("IQ") => CalendarSystem::Islamic,
            Some("IL") => CalendarSystem::Hebrew,
            Some("IR") => CalendarSystem::Persian,
            _ => CalendarSystem::Gregorian,
        }
    }
    fn to_japanese_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let (era, era_year) = if year >= 2019 {
            ("Reiwa", year - 2019 + 1)
        } else if year >= 1989 {
            ("Heisei", year - 1989 + 1)
        } else if year >= 1926 {
            ("Showa", year - 1926 + 1)
        } else if year >= 1912 {
            ("Taisho", year - 1912 + 1)
        } else if year >= 1868 {
            ("Meiji", year - 1868 + 1)
        } else {
            ("Gregorian", year)
        };
        CalendarDate::new(CalendarSystem::Japanese, era_year, month, day).with_era(era)
    }
    fn to_islamic_approximate(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let jd = self.gregorian_to_julian_day(year, month as i32, day as i32);
        let (h_year, h_month, h_day) = self.julian_day_to_islamic(jd);
        CalendarDate::new(
            CalendarSystem::Islamic,
            h_year,
            h_month as u32,
            h_day as u32,
        )
    }
    fn to_hebrew_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let hebrew_year = year + 3760;
        CalendarDate::new(CalendarSystem::Hebrew, hebrew_year, month, day)
    }
    fn to_persian_calendar(&self, year: i32, month: u32, day: u32) -> CalendarDate {
        let persian_year = year - 621;
        CalendarDate::new(CalendarSystem::Persian, persian_year, month, day)
    }
    fn gregorian_to_julian_day(&self, year: i32, month: i32, day: i32) -> i32 {
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;
        day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    }
    fn julian_day_to_islamic(&self, jd: i32) -> (i32, i32, i32) {
        let l = jd - 1948440 + 10632;
        let n = (l - 1) / 10631;
        let l = l - 10631 * n + 354;
        let j = ((10985 - l) / 5316) * ((50 * l) / 17719) + (l / 5670) * ((43 * l) / 15238);
        let l = l - ((30 - j) / 15) * ((17719 * j) / 50) - (j / 16) * ((15238 * j) / 43) + 29;
        let month = (24 * l) / 709;
        let day = l - (709 * month) / 24;
        let year = 30 * n + j - 30;
        (year, month, day)
    }
    /// Converts from Islamic calendar to Gregorian
    pub fn to_gregorian_from_islamic(
        &self,
        h_year: i32,
        h_month: u32,
        h_day: u32,
    ) -> (i32, u32, u32) {
        let jd = self.islamic_to_julian_day(h_year, h_month as i32, h_day as i32);
        self.julian_day_to_gregorian(jd)
    }
    fn islamic_to_julian_day(&self, year: i32, month: i32, day: i32) -> i32 {
        ((11 * year + 3) / 30) + 354 * year + 30 * month - ((month - 1) / 2) + day + 1948440 - 385
    }
    fn julian_day_to_gregorian(&self, jd: i32) -> (i32, u32, u32) {
        let a = jd + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;
        (year, month as u32, day as u32)
    }
    /// Formats a calendar date according to locale conventions.
    pub fn format_date(&self, date: &CalendarDate) -> String {
        match date.system {
            CalendarSystem::Japanese => {
                if let Some(ref era) = date.era {
                    format!("{}{}年{}月{}日", era, date.year, date.month, date.day)
                } else {
                    format!("{}年{}月{}日", date.year, date.month, date.day)
                }
            }
            CalendarSystem::Buddhist => {
                format!("พ.ศ. {} {}/{}", date.year, date.day, date.month)
            }
            CalendarSystem::Islamic => {
                let month_names = [
                    "Muharram",
                    "Safar",
                    "Rabi' al-awwal",
                    "Rabi' al-thani",
                    "Jumada al-awwal",
                    "Jumada al-thani",
                    "Rajab",
                    "Sha'ban",
                    "Ramadan",
                    "Shawwal",
                    "Dhu al-Qi'dah",
                    "Dhu al-Hijjah",
                ];
                let month_name = month_names.get((date.month - 1) as usize).unwrap_or(&"");
                format!("{} {} {} AH", date.day, month_name, date.year)
            }
            CalendarSystem::Hebrew => {
                format!(
                    "{} {}, {}",
                    date.day,
                    self.get_hebrew_month_name(date.month),
                    date.year
                )
            }
            CalendarSystem::Persian => {
                format!("{}/{}/{} SH", date.year, date.month, date.day)
            }
            CalendarSystem::Gregorian => {
                format!("{}-{:02}-{:02}", date.year, date.month, date.day)
            }
        }
    }
    fn get_hebrew_month_name(&self, month: u32) -> &'static str {
        match month {
            1 => "Nisan",
            2 => "Iyar",
            3 => "Sivan",
            4 => "Tammuz",
            5 => "Av",
            6 => "Elul",
            7 => "Tishrei",
            8 => "Cheshvan",
            9 => "Kislev",
            10 => "Tevet",
            11 => "Shevat",
            12 => "Adar",
            _ => "Unknown",
        }
    }
}
/// Index generator.
#[derive(Debug, Clone)]
pub struct IndexGenerator {
    pub(super) entries: Vec<IndexEntry>,
    locale: Locale,
}
impl IndexGenerator {
    /// Creates a new index generator.
    pub fn new(locale: Locale) -> Self {
        Self {
            entries: Vec::new(),
            locale,
        }
    }
    /// Adds an entry to the index.
    pub fn add_entry(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
    }
    /// Sorts entries alphabetically.
    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.term.cmp(&b.term));
        for entry in &mut self.entries {
            entry.sub_entries.sort_by(|a, b| a.term.cmp(&b.term));
        }
    }
    /// Generates the formatted index.
    pub fn generate(&self) -> String {
        let mut result = String::new();
        let header = match self.locale.language.as_str() {
            "en" => "Index",
            "ja" => "索引",
            "de" => "Index",
            "fr" => "Index",
            "es" => "Índice",
            "it" => "Indice",
            "pt" => "Índice",
            "nl" => "Index",
            "pl" => "Indeks",
            "ko" => "색인",
            _ => "Index",
        };
        result.push_str(header);
        result.push_str("\n\n");
        for entry in &self.entries {
            self.format_entry(&mut result, entry, 0);
        }
        result
    }
    #[allow(clippy::only_used_in_recursion)]
    fn format_entry(&self, result: &mut String, entry: &IndexEntry, level: usize) {
        let indent = "  ".repeat(level);
        let pages = entry
            .pages
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&format!("{}{}, {}\n", indent, entry.term, pages));
        for sub in &entry.sub_entries {
            self.format_entry(result, sub, level + 1);
        }
    }
}
/// EU member state variation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EUMemberStateVariation {
    /// Member state locale
    pub member_state_locale: Locale,
    /// Country name
    pub country_name: String,
    /// EU accession date (year)
    pub accession_year: u32,
    /// Legal system type
    pub legal_system: String,
    /// Key EU law adaptations
    pub eu_adaptations: Vec<String>,
    /// National legal specialties
    pub specialties: Vec<String>,
}
impl EUMemberStateVariation {
    /// Creates a new EU member state variation.
    pub fn new(
        member_state_locale: Locale,
        country_name: impl Into<String>,
        accession_year: u32,
        legal_system: impl Into<String>,
    ) -> Self {
        Self {
            member_state_locale,
            country_name: country_name.into(),
            accession_year,
            legal_system: legal_system.into(),
            eu_adaptations: vec![],
            specialties: vec![],
        }
    }
    /// Adds an EU law adaptation description.
    pub fn add_eu_adaptation(mut self, adaptation: impl Into<String>) -> Self {
        self.eu_adaptations.push(adaptation.into());
        self
    }
    /// Adds a national legal specialty.
    pub fn add_specialty(mut self, specialty: impl Into<String>) -> Self {
        self.specialties.push(specialty.into());
        self
    }
}
/// CLDR (Common Locale Data Repository) integration.
#[derive(Debug, Clone)]
pub struct CLDRData {
    entries: HashMap<String, Vec<CLDREntry>>,
}
impl CLDRData {
    /// Creates a new CLDR data store.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    /// Creates CLDR data with default legal localization data.
    pub fn with_defaults() -> Self {
        let mut cldr = Self::new();
        let en_us = Locale::new("en").with_country("US");
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "en",
            "English",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "ja",
            "Japanese",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "fr",
            "French",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Languages,
            "de",
            "German",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "US",
            "United States",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "GB",
            "United Kingdom",
        ));
        cldr.add_entry(CLDREntry::new(
            en_us.clone(),
            CLDRFieldType::Territories,
            "JP",
            "Japan",
        ));
        let ja_jp = Locale::new("ja").with_country("JP");
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "en",
            "英語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "ja",
            "日本語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Languages,
            "fr",
            "フランス語",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "US",
            "アメリカ合衆国",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "GB",
            "イギリス",
        ));
        cldr.add_entry(CLDREntry::new(
            ja_jp.clone(),
            CLDRFieldType::Territories,
            "JP",
            "日本",
        ));
        let fr_fr = Locale::new("fr").with_country("FR");
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "en",
            "anglais",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "ja",
            "japonais",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Languages,
            "fr",
            "français",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "US",
            "États-Unis",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "GB",
            "Royaume-Uni",
        ));
        cldr.add_entry(CLDREntry::new(
            fr_fr.clone(),
            CLDRFieldType::Territories,
            "FR",
            "France",
        ));
        cldr
    }
    /// Adds a CLDR entry.
    pub fn add_entry(&mut self, entry: CLDREntry) {
        let key = format!("{}:{}", entry.locale, entry.field_type);
        self.entries.entry(key).or_default().push(entry);
    }
    /// Gets CLDR entries for a locale and field type.
    pub fn get_entries(&self, locale: &Locale, field_type: CLDRFieldType) -> Vec<&CLDREntry> {
        let key = format!("{}:{}", locale, field_type);
        self.entries
            .get(&key)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }
    /// Gets a specific CLDR value.
    pub fn get_value(
        &self,
        locale: &Locale,
        field_type: CLDRFieldType,
        key: &str,
    ) -> Option<String> {
        self.get_entries(locale, field_type)
            .into_iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.clone())
    }
    /// Returns the number of locales with CLDR data.
    pub fn locale_count(&self) -> usize {
        self.entries.len()
    }
    /// Returns the total number of entries.
    pub fn entry_count(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}
/// Legal deadline calculator with time zone and business day support.
#[derive(Debug, Clone)]
pub struct DeadlineCalculator {
    jurisdiction: WorkingDaysConfig,
    timezone: Option<TimeZone>,
}
impl DeadlineCalculator {
    /// Creates a new deadline calculator.
    pub fn new(jurisdiction: WorkingDaysConfig) -> Self {
        Self {
            jurisdiction,
            timezone: None,
        }
    }
    /// Sets the time zone for deadline calculations.
    pub fn with_timezone(mut self, timezone: TimeZone) -> Self {
        self.timezone = Some(timezone);
        self
    }
    /// Calculates a deadline by adding business days to a start date.
    pub fn calculate_deadline(
        &self,
        start_year: i32,
        start_month: u32,
        start_day: u32,
        business_days: i32,
    ) -> (i32, u32, u32) {
        self.jurisdiction
            .add_working_days(start_year, start_month, start_day, business_days)
    }
    /// Calculates a deadline with time component and timezone conversion.
    pub fn calculate_deadline_with_time(
        &self,
        start_year: i32,
        start_month: u32,
        start_day: u32,
        start_hour: u32,
        start_minute: u32,
        business_days: i32,
    ) -> (i32, u32, u32, u32, u32) {
        let (end_year, end_month, end_day) =
            self.calculate_deadline(start_year, start_month, start_day, business_days);
        (end_year, end_month, end_day, start_hour, start_minute)
    }
    /// Converts a deadline from one timezone to another.
    #[allow(clippy::too_many_arguments)]
    pub fn convert_timezone(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        from_tz: &TimeZone,
        to_tz: &TimeZone,
    ) -> (i32, u32, u32, u32, u32) {
        let (utc_y, utc_m, utc_d, utc_h, utc_min) =
            from_tz.local_to_utc(year, month, day, hour, minute);
        to_tz.utc_to_local(utc_y, utc_m, utc_d, utc_h, utc_min)
    }
    /// Checks if a deadline has passed (considering timezone if set).
    pub fn is_deadline_passed(
        &self,
        deadline_year: i32,
        deadline_month: u32,
        deadline_day: u32,
        current_year: i32,
        current_month: u32,
        current_day: u32,
    ) -> bool {
        if deadline_year < current_year {
            return true;
        }
        if deadline_year > current_year {
            return false;
        }
        if deadline_month < current_month {
            return true;
        }
        if deadline_month > current_month {
            return false;
        }
        deadline_day < current_day
    }
    /// Calculates statute of limitations deadline.
    /// Returns the final date when a claim must be filed.
    pub fn statute_of_limitations(
        &self,
        incident_year: i32,
        incident_month: u32,
        incident_day: u32,
        years: i32,
    ) -> (i32, u32, u32) {
        let final_year = incident_year + years;
        (final_year, incident_month, incident_day)
    }
    /// Applies holiday rollover rules.
    /// If a deadline falls on a non-working day, roll to the next working day.
    pub fn apply_holiday_rollover(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        if self.jurisdiction.is_working_day(year, month, day) {
            return (year, month, day);
        }
        let mut current = (year, month, day);
        for _ in 0..7 {
            current = self.add_one_day(current.0, current.1, current.2);
            if self
                .jurisdiction
                .is_working_day(current.0, current.1, current.2)
            {
                return current;
            }
        }
        (year, month, day)
    }
    /// Adds a grace period (in calendar days) to a deadline.
    pub fn add_grace_period(
        &self,
        deadline_year: i32,
        deadline_month: u32,
        deadline_day: u32,
        grace_days: i32,
    ) -> (i32, u32, u32) {
        let mut result = (deadline_year, deadline_month, deadline_day);
        for _ in 0..grace_days {
            result = self.add_one_day(result.0, result.1, result.2);
        }
        result
    }
    /// Checks for deadline conflicts (if two deadlines are too close).
    /// Returns true if deadlines are within threshold_days of each other.
    #[allow(clippy::too_many_arguments)]
    pub fn has_deadline_conflict(
        &self,
        deadline1_year: i32,
        deadline1_month: u32,
        deadline1_day: u32,
        deadline2_year: i32,
        deadline2_month: u32,
        deadline2_day: u32,
        threshold_days: i32,
    ) -> bool {
        let days_between = self.days_between(
            deadline1_year,
            deadline1_month,
            deadline1_day,
            deadline2_year,
            deadline2_month,
            deadline2_day,
        );
        days_between.abs() <= threshold_days
    }
    /// Helper: adds one calendar day to a date.
    fn add_one_day(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        };
        if day < days_in_month {
            (year, month, day + 1)
        } else if month < 12 {
            (year, month + 1, 1)
        } else {
            (year + 1, 1, 1)
        }
    }
    /// Helper: calculates days between two dates (approximate).
    fn days_between(&self, y1: i32, m1: u32, d1: u32, y2: i32, m2: u32, d2: u32) -> i32 {
        let days1 = y1 * 365 + (m1 as i32) * 30 + (d1 as i32);
        let days2 = y2 * 365 + (m2 as i32) * 30 + (d2 as i32);
        days2 - days1
    }
    /// Helper: checks if a year is a leap year.
    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}
/// Extracted obligation from a legal document.
#[derive(Debug, Clone)]
pub struct ExtractedObligation {
    /// Type of obligation
    pub obligation_type: ObligationType,
    /// Text describing the obligation
    pub text: String,
    /// Subject of the obligation (who must perform it)
    pub subject: Option<String>,
    /// Position in document
    pub position: usize,
    /// Confidence score
    pub confidence: f64,
}
/// Writing style attribute for translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleAttribute {
    /// Formality level (formal, informal, neutral).
    Formality,
    /// Tone (professional, conversational, authoritative).
    Tone,
    /// Person (first, second, third).
    Person,
    /// Voice (active, passive).
    Voice,
    /// Tense (present, past, future).
    Tense,
}
/// Identified party in a legal document.
#[derive(Debug, Clone)]
pub struct IdentifiedParty {
    /// Name of the party
    pub name: String,
    /// Role of the party
    pub role: PartyRole,
    /// Position in document
    pub position: usize,
    /// Confidence score
    pub confidence: f64,
}
/// LLM-based legal translator (infrastructure for external LLM integration).
#[derive(Debug, Clone)]
pub struct LLMTranslator {
    /// The LLM provider to use.
    pub provider: LLMProvider,
    /// The model name (e.g., "gpt-4", "claude-3-opus").
    pub model_name: String,
    /// The prompt template for translation.
    pub prompt_template: LegalPromptTemplate,
    /// Maximum tokens for the response.
    pub max_tokens: usize,
    /// Temperature for generation (0.0 to 1.0).
    pub temperature: f32,
}
impl LLMTranslator {
    /// Creates a new LLM translator.
    pub fn new(provider: LLMProvider, model_name: &str) -> Self {
        Self {
            provider,
            model_name: model_name.to_string(),
            prompt_template: LegalPromptTemplate::default_legal_translation(),
            max_tokens: 2000,
            temperature: 0.3,
        }
    }
    /// Creates an OpenAI GPT-4 translator.
    pub fn openai_gpt4() -> Self {
        Self::new(LLMProvider::OpenAI, "gpt-4")
    }
    /// Creates an Anthropic Claude translator.
    pub fn anthropic_claude() -> Self {
        Self::new(LLMProvider::Anthropic, "claude-3-opus-20240229")
    }
    /// Sets a custom prompt template.
    pub fn with_prompt_template(mut self, template: LegalPromptTemplate) -> Self {
        self.prompt_template = template;
        self
    }
    /// Sets the maximum tokens.
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }
    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }
    /// Generates a translation prompt for the given text.
    pub fn generate_prompt(
        &self,
        text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        legal_context: Option<&str>,
    ) -> String {
        self.prompt_template
            .render(text, source_locale, target_locale, legal_context)
    }
    /// Gets the system prompt.
    pub fn get_system_prompt(&self) -> &str {
        &self.prompt_template.system_prompt
    }
}
/// Context disambiguation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisambiguationType {
    /// Disambiguate by legal domain (e.g., criminal vs. civil).
    LegalDomain,
    /// Disambiguate by jurisdiction.
    Jurisdiction,
    /// Disambiguate by document type.
    DocumentType,
    /// Disambiguate by temporal context (historical vs. modern).
    Temporal,
    /// Disambiguate by formality level.
    Formality,
}
/// Braille formatter for visual accessibility.
/// Supports Grade 1 (uncontracted) and Grade 2 (contracted) Braille.
#[derive(Debug)]
pub struct BrailleFormatter {
    #[allow(dead_code)]
    grade: BrailleGrade,
}
impl BrailleFormatter {
    /// Creates a new Braille formatter.
    pub fn new(grade: BrailleGrade) -> Self {
        Self { grade }
    }
    /// Converts text to Braille Unicode representation.
    pub fn to_braille(&self, text: &str) -> String {
        let text = text.to_lowercase();
        let mut result = String::new();
        for ch in text.chars() {
            if let Some(braille) = self.char_to_braille(ch) {
                result.push(braille);
            } else {
                result.push(ch);
            }
        }
        result
    }
    /// Converts a single character to Braille.
    fn char_to_braille(&self, ch: char) -> Option<char> {
        let braille_code = match ch {
            'a' => 0x2801,
            'b' => 0x2803,
            'c' => 0x2809,
            'd' => 0x2819,
            'e' => 0x2811,
            'f' => 0x280B,
            'g' => 0x281B,
            'h' => 0x2813,
            'i' => 0x280A,
            'j' => 0x281A,
            'k' => 0x2805,
            'l' => 0x2807,
            'm' => 0x280D,
            'n' => 0x281D,
            'o' => 0x2815,
            'p' => 0x280F,
            'q' => 0x281F,
            'r' => 0x2817,
            's' => 0x280E,
            't' => 0x281E,
            'u' => 0x2825,
            'v' => 0x2827,
            'w' => 0x283A,
            'x' => 0x282D,
            'y' => 0x283D,
            'z' => 0x2835,
            ' ' => 0x2800,
            _ => return None,
        };
        char::from_u32(braille_code)
    }
    /// Formats legal document section numbers in Braille.
    pub fn format_section_number(&self, section: &str) -> String {
        format!("§ {}", self.to_braille(section))
    }
}
/// Address formatter for legal documents per jurisdiction.
#[derive(Debug, Clone)]
pub struct AddressFormatter {
    pub(super) locale: Locale,
}
impl AddressFormatter {
    /// Creates a new address formatter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Formats an address according to jurisdiction conventions.
    pub fn format(&self, address: &Address) -> String {
        match self.locale.country.as_deref() {
            Some("US") => self.format_us(address),
            Some("GB") => self.format_uk(address),
            Some("JP") => self.format_japan(address),
            Some("DE") | Some("FR") | Some("IT") | Some("ES") => self.format_european(address),
            Some("CN") => self.format_china(address),
            Some("KR") => self.format_korea(address),
            _ => self.format_default(address),
        }
    }
    /// Formats a US address.
    fn format_us(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];
        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }
        let city_line = if let Some(state) = &addr.state {
            format!("{}, {} {}", addr.city, state, addr.postal_code)
        } else {
            format!("{} {}", addr.city, addr.postal_code)
        };
        lines.push(city_line);
        lines.push(addr.country.clone());
        lines.join("\n")
    }
    /// Formats a UK address.
    fn format_uk(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];
        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }
        lines.push(addr.city.clone());
        if let Some(state) = &addr.state {
            lines.push(state.clone());
        }
        lines.push(addr.postal_code.clone());
        lines.push(addr.country.clone());
        lines.join("\n")
    }
    /// Formats a Japanese address (reverse order: country → postal → prefecture → city → street).
    fn format_japan(&self, addr: &Address) -> String {
        let mut result = String::new();
        result.push('〒');
        result.push_str(&addr.postal_code);
        result.push('\n');
        if let Some(state) = &addr.state {
            result.push_str(state);
        }
        result.push_str(&addr.city);
        result.push_str(&addr.street1);
        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }
        result
    }
    /// Formats a European address.
    fn format_european(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];
        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }
        lines.push(format!("{} {}", addr.postal_code, addr.city));
        if let Some(state) = &addr.state {
            lines.push(state.clone());
        }
        lines.push(addr.country.clone());
        lines.join("\n")
    }
    /// Formats a Chinese address (reverse order like Japanese).
    fn format_china(&self, addr: &Address) -> String {
        let mut result = String::new();
        result.push_str(&addr.country);
        result.push(' ');
        if let Some(state) = &addr.state {
            result.push_str(state);
        }
        result.push_str(&addr.city);
        result.push_str(&addr.street1);
        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }
        result.push(' ');
        result.push_str(&addr.postal_code);
        result
    }
    /// Formats a Korean address.
    fn format_korea(&self, addr: &Address) -> String {
        let mut result = String::new();
        if let Some(state) = &addr.state {
            result.push_str(state);
            result.push(' ');
        }
        result.push_str(&addr.city);
        result.push(' ');
        result.push_str(&addr.street1);
        if let Some(building) = &addr.building {
            result.push(' ');
            result.push_str(building);
        }
        result.push_str(" (");
        result.push_str(&addr.postal_code);
        result.push(')');
        result
    }
    /// Formats a default international address.
    fn format_default(&self, addr: &Address) -> String {
        let mut lines = vec![addr.street1.clone()];
        if let Some(street2) = &addr.street2 {
            lines.push(street2.clone());
        }
        let city_line = if let Some(state) = &addr.state {
            format!("{}, {}", addr.city, state)
        } else {
            addr.city.clone()
        };
        lines.push(city_line);
        lines.push(addr.postal_code.clone());
        lines.push(addr.country.clone());
        lines.join("\n")
    }
    /// Formats a single-line address (for forms).
    pub fn format_single_line(&self, address: &Address) -> String {
        self.format(address).replace('\n', ", ")
    }
}
/// Mock translation service for testing and fallback.
#[derive(Debug, Clone)]
pub struct MockTranslationService {
    pub(super) available: bool,
}
impl MockTranslationService {
    /// Creates a new mock translation service.
    pub fn new() -> Self {
        Self { available: true }
    }
    /// Sets availability status.
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FootnoteStyle {
    /// Numeric: 1, 2, 3, ...
    Numeric,
    /// Symbols: *, †, ‡, §, ...
    Symbol,
    /// Lowercase letters: a, b, c, ...
    Letter,
}
/// Translation with memory integration.
#[derive(Debug, Clone)]
pub struct MTWithMemory {
    /// Neural MT translator
    pub(super) mt_translator: Arc<NeuralMachineTranslator>,
    /// Translation memory
    pub(super) memory: Arc<Mutex<TranslationMemory>>,
    /// Minimum fuzzy match threshold
    pub(super) fuzzy_threshold: f32,
}
impl MTWithMemory {
    /// Creates a new MT with memory integration.
    pub fn new(
        mt_translator: NeuralMachineTranslator,
        memory: Arc<Mutex<TranslationMemory>>,
    ) -> Self {
        Self {
            mt_translator: Arc::new(mt_translator),
            memory,
            fuzzy_threshold: 0.85,
        }
    }
    /// Sets fuzzy match threshold.
    pub fn with_fuzzy_threshold(mut self, threshold: f32) -> Self {
        self.fuzzy_threshold = threshold.clamp(0.0, 1.0);
        self
    }
    /// Translates with memory lookup and MT fallback.
    pub fn translate(
        &self,
        text: &str,
        source: &Locale,
        target: &Locale,
    ) -> I18nResult<MTTranslation> {
        {
            let memory_guard = self.memory.lock().expect("memory mutex poisoned");
            let exact_matches = memory_guard.find_exact(text, source, target);
            if !exact_matches.is_empty() {
                let target_text = exact_matches[0].target_text.clone();
                drop(memory_guard);
                return Ok(MTTranslation {
                    text: target_text,
                    quality_score: 1.0,
                    source_locale: source.clone(),
                    target_locale: target.clone(),
                    engine: TranslationEngine::Generic,
                    alternatives: vec![],
                });
            }
            let fuzzy_matches =
                memory_guard.find_fuzzy_levenshtein(text, source, target, self.fuzzy_threshold);
            if !fuzzy_matches.is_empty() {
                let (entry, score) = fuzzy_matches[0];
                let target_text = entry.target_text.clone();
                drop(memory_guard);
                return Ok(MTTranslation {
                    text: target_text,
                    quality_score: score,
                    source_locale: source.clone(),
                    target_locale: target.clone(),
                    engine: TranslationEngine::Generic,
                    alternatives: vec![],
                });
            }
        }
        let mt_result = self.mt_translator.translate(text, source, target)?;
        if mt_result.quality_score >= 0.8 {
            let mut memory_guard = self.memory.lock().expect("memory mutex poisoned");
            memory_guard.add_translation(
                text.to_string(),
                source.clone(),
                mt_result.text.clone(),
                target.clone(),
            );
        }
        Ok(mt_result)
    }
    /// Returns the fuzzy match threshold.
    pub fn fuzzy_threshold(&self) -> f32 {
        self.fuzzy_threshold
    }
}
/// Calendar system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalendarSystem {
    /// Gregorian calendar (most common worldwide)
    Gregorian,
    /// Islamic/Hijri calendar
    Islamic,
    /// Hebrew/Jewish calendar
    Hebrew,
    /// Japanese calendar (Imperial era)
    Japanese,
    /// Buddhist calendar
    Buddhist,
    /// Persian/Solar Hijri calendar
    Persian,
}
/// Registry of cross-regional term equivalences.
#[derive(Debug, Default)]
pub struct CrossRegionalTermEquivalenceRegistry {
    pub(super) equivalences: Vec<TermEquivalence>,
}
impl CrossRegionalTermEquivalenceRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default term equivalences.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_equivalence(
            TermEquivalence::new("attorney", "US")
                .add_equivalent("GB", "solicitor", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "avocat", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Rechtsanwalt", EquivalenceLevel::Exact)
                .add_equivalent("JP", "bengoshi", EquivalenceLevel::Exact)
                .add_note_to_equivalent("GB", "UK distinguishes solicitors and barristers"),
        );
        registry.add_equivalence(
            TermEquivalence::new("corporation", "US")
                .add_equivalent("GB", "limited company", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "société anonyme", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "Aktiengesellschaft", EquivalenceLevel::Exact)
                .add_equivalent("JP", "kabushiki kaisha", EquivalenceLevel::Exact)
                .add_note_to_equivalent("FR", "SA is public company; SARL is private")
                .add_note_to_equivalent("DE", "AG is stock corporation"),
        );
        registry.add_equivalence(
            TermEquivalence::new("contract", "US")
                .add_equivalent("GB", "contract", EquivalenceLevel::Exact)
                .add_equivalent("FR", "contrat", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Vertrag", EquivalenceLevel::Exact)
                .add_equivalent("JP", "keiyaku", EquivalenceLevel::Exact),
        );
        registry.add_equivalence(
            TermEquivalence::new("tort", "US")
                .add_equivalent("GB", "tort", EquivalenceLevel::Exact)
                .add_equivalent(
                    "FR",
                    "responsabilité civile délictuelle",
                    EquivalenceLevel::Approximate,
                )
                .add_equivalent("DE", "unerlaubte Handlung", EquivalenceLevel::Approximate)
                .add_equivalent("JP", "fuhōkōi", EquivalenceLevel::Approximate)
                .add_note_to_equivalent("FR", "Civil law tort concept differs from common law")
                .add_note_to_equivalent("DE", "Part of BGB obligations law"),
        );
        registry.add_equivalence(
            TermEquivalence::new("trust", "GB")
                .add_equivalent("US", "trust", EquivalenceLevel::Exact)
                .add_equivalent("FR", "fiducie", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "Treuhand", EquivalenceLevel::Loose)
                .add_equivalent("JP", "shintaku", EquivalenceLevel::Approximate)
                .add_note_to_equivalent("FR", "Introduced in 2007, not traditional civil law")
                .add_note_to_equivalent("DE", "Not a true trust, more like agency")
                .add_note_to_equivalent("JP", "Modern adoption of trust concept"),
        );
        registry.add_equivalence(
            TermEquivalence::new("due_process", "US")
                .add_equivalent("GB", "natural justice", EquivalenceLevel::Approximate)
                .add_equivalent("FR", "droits de la défense", EquivalenceLevel::Approximate)
                .add_equivalent("DE", "rechtliches Gehör", EquivalenceLevel::Approximate)
                .add_equivalent("JP", "tekisei tetsuzuki", EquivalenceLevel::Exact)
                .add_note_to_equivalent("GB", "Natural justice is broader concept")
                .add_note_to_equivalent("FR", "Rights of defense in French law")
                .add_note_to_equivalent("DE", "Right to be heard in German law"),
        );
        registry.add_equivalence(
            TermEquivalence::new("plaintiff", "US")
                .add_equivalent("GB", "claimant", EquivalenceLevel::Exact)
                .add_equivalent("FR", "demandeur", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Kläger", EquivalenceLevel::Exact)
                .add_equivalent("JP", "genkoku", EquivalenceLevel::Exact),
        );
        registry.add_equivalence(
            TermEquivalence::new("statute_of_limitations", "US")
                .add_equivalent("GB", "limitation period", EquivalenceLevel::Exact)
                .add_equivalent("FR", "prescription", EquivalenceLevel::Exact)
                .add_equivalent("DE", "Verjährung", EquivalenceLevel::Exact)
                .add_equivalent("JP", "shōmetsu jikō", EquivalenceLevel::Exact),
        );
        registry
    }
    /// Adds a term equivalence to the registry.
    pub fn add_equivalence(&mut self, equivalence: TermEquivalence) {
        self.equivalences.push(equivalence);
    }
    /// Finds term equivalence.
    pub fn find_equivalence(
        &self,
        term: &str,
        base_jurisdiction: &str,
    ) -> Option<&TermEquivalence> {
        self.equivalences
            .iter()
            .find(|e| e.base_term == term && e.base_jurisdiction == base_jurisdiction)
    }
    /// Gets equivalent term in target jurisdiction.
    pub fn get_equivalent_term(
        &self,
        term: &str,
        base_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> Option<&EquivalentTerm> {
        self.find_equivalence(term, base_jurisdiction)
            .and_then(|e| e.get_equivalent(target_jurisdiction))
    }
}
/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WCAGLevel {
    /// Level A (minimum)
    A,
    /// Level AA (mid-range)
    AA,
    /// Level AAA (highest)
    AAA,
}
/// Emphasis level for SSML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmphasisLevel {
    None,
    Reduced,
    Moderate,
    Strong,
}
/// Registry for ISO 639-3 language codes.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct ISO639_3_Registry {
    codes: HashMap<String, ISO639_3>,
}
impl ISO639_3_Registry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }
    /// Creates a registry with default legal language codes.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_code(ISO639_3::new(
            "eng",
            "English",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "fra",
            "French",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "deu",
            "German",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "spa",
            "Spanish",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "jpn",
            "Japanese",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "zho",
            "Chinese",
            LanguageType::Living,
            LanguageScope::Macrolanguage,
        ));
        registry.add_code(ISO639_3::new(
            "ara",
            "Arabic",
            LanguageType::Living,
            LanguageScope::Macrolanguage,
        ));
        registry.add_code(ISO639_3::new(
            "rus",
            "Russian",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "por",
            "Portuguese",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "ita",
            "Italian",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "nld",
            "Dutch",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "pol",
            "Polish",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "kor",
            "Korean",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "heb",
            "Hebrew",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "hin",
            "Hindi",
            LanguageType::Living,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "lat",
            "Latin",
            LanguageType::Ancient,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "ang",
            "Old English",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "enm",
            "Middle English",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));
        registry.add_code(ISO639_3::new(
            "fro",
            "Old French",
            LanguageType::Historical,
            LanguageScope::Individual,
        ));
        registry
    }
    /// Adds a language code to the registry.
    pub fn add_code(&mut self, code: ISO639_3) {
        self.codes.insert(code.code.clone(), code);
    }
    /// Gets a language code by its ISO 639-3 code.
    pub fn get_code(&self, code: &str) -> Option<&ISO639_3> {
        self.codes.get(&code.to_lowercase())
    }
    /// Gets all legal languages in the registry.
    pub fn get_legal_languages(&self) -> Vec<&ISO639_3> {
        self.codes
            .values()
            .filter(|code| code.is_legal_language())
            .collect()
    }
    /// Gets all historical/ancient languages.
    pub fn get_historical_languages(&self) -> Vec<&ISO639_3> {
        self.codes
            .values()
            .filter(|code| {
                matches!(
                    code.language_type,
                    LanguageType::Ancient | LanguageType::Historical
                )
            })
            .collect()
    }
    /// Returns the number of language codes in the registry.
    pub fn code_count(&self) -> usize {
        self.codes.len()
    }
}
/// Transcription segment with timing information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// The transcribed text for this segment.
    pub text: String,
    /// Start time in milliseconds from recording start.
    pub start_ms: u64,
    /// End time in milliseconds from recording start.
    pub end_ms: u64,
    /// Speaker identifier (if available).
    pub speaker: Option<String>,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f64,
    /// Detected language locale.
    pub locale: Locale,
}
impl TranscriptionSegment {
    /// Creates a new transcription segment.
    pub fn new(text: impl Into<String>, start_ms: u64, end_ms: u64, locale: Locale) -> Self {
        Self {
            text: text.into(),
            start_ms,
            end_ms,
            speaker: None,
            confidence: 1.0,
            locale,
        }
    }
    /// Sets the speaker identifier.
    pub fn with_speaker(mut self, speaker: impl Into<String>) -> Self {
        self.speaker = Some(speaker.into());
        self
    }
    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
    /// Returns the duration of this segment in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        self.end_ms.saturating_sub(self.start_ms)
    }
    /// Formats the segment with timestamp.
    pub fn format_with_timestamp(&self) -> String {
        let start_sec = self.start_ms / 1000;
        let end_sec = self.end_ms / 1000;
        let speaker_label = self.speaker.as_deref().unwrap_or("Unknown");
        format!(
            "[{:02}:{:02} - {:02}:{:02}] {}: {}",
            start_sec / 60,
            start_sec % 60,
            end_sec / 60,
            end_sec % 60,
            speaker_label,
            self.text
        )
    }
}
/// ISO 639-3 language code (3-letter code).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISO639_3 {
    /// 3-letter language code (e.g., "eng", "jpn", "fra").
    pub code: String,
    /// English name of the language.
    pub name: String,
    /// Type of language (Individual, Macrolanguage, Special).
    pub language_type: LanguageType,
    /// Scope (Individual, Macrolanguage, Special).
    pub scope: LanguageScope,
}
impl ISO639_3 {
    /// Creates a new ISO 639-3 language code.
    pub fn new(code: &str, name: &str, language_type: LanguageType, scope: LanguageScope) -> Self {
        Self {
            code: code.to_lowercase(),
            name: name.to_string(),
            language_type,
            scope,
        }
    }
    /// Converts to ISO 639-1 (2-letter code) if possible.
    pub fn to_iso639_1(&self) -> Option<String> {
        match self.code.as_str() {
            "eng" => Some("en".to_string()),
            "jpn" => Some("ja".to_string()),
            "fra" => Some("fr".to_string()),
            "deu" => Some("de".to_string()),
            "spa" => Some("es".to_string()),
            "zho" => Some("zh".to_string()),
            "ara" => Some("ar".to_string()),
            "rus" => Some("ru".to_string()),
            "por" => Some("pt".to_string()),
            "ita" => Some("it".to_string()),
            "nld" => Some("nl".to_string()),
            "pol" => Some("pl".to_string()),
            "kor" => Some("ko".to_string()),
            "heb" => Some("he".to_string()),
            "hin" => Some("hi".to_string()),
            "fas" => Some("fa".to_string()),
            "tha" => Some("th".to_string()),
            "vie" => Some("vi".to_string()),
            "ind" => Some("id".to_string()),
            "swe" => Some("sv".to_string()),
            "dan" => Some("da".to_string()),
            "fin" => Some("fi".to_string()),
            "nor" => Some("no".to_string()),
            "tur" => Some("tr".to_string()),
            "lat" => Some("la".to_string()),
            _ => None,
        }
    }
    /// Checks if this is a legal language (used in legal contexts).
    pub fn is_legal_language(&self) -> bool {
        matches!(
            self.code.as_str(),
            "eng" | "fra" | "deu" | "spa" | "lat" | "jpn" | "zho" | "ara" | "rus" | "por" | "ita"
        )
    }
}
