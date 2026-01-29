//! Saudi Public Holidays and Hijri Calendar Support
//!
//! Saudi Arabia uses the Hijri (Islamic) calendar as its official calendar.
//! All official documents, laws, and government operations use Hijri dates.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Hijri (Islamic) month names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HijriMonth {
    /// محرم - First month
    Muharram,
    /// صفر - Second month
    Safar,
    /// ربيع الأول - Third month
    RabiAlAwwal,
    /// ربيع الآخر - Fourth month
    RabiAlThani,
    /// جمادى الأولى - Fifth month
    JumadaAlUla,
    /// جمادى الآخرة - Sixth month
    JumadaAlAkhirah,
    /// رجب - Seventh month
    Rajab,
    /// شعبان - Eighth month
    Shaban,
    /// رمضان - Ninth month (Fasting month)
    Ramadan,
    /// شوال - Tenth month
    Shawwal,
    /// ذو القعدة - Eleventh month
    DhulQadah,
    /// ذو الحجة - Twelfth month (Hajj month)
    DhulHijjah,
}

impl HijriMonth {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Muharram => "محرم",
            Self::Safar => "صفر",
            Self::RabiAlAwwal => "ربيع الأول",
            Self::RabiAlThani => "ربيع الآخر",
            Self::JumadaAlUla => "جمادى الأولى",
            Self::JumadaAlAkhirah => "جمادى الآخرة",
            Self::Rajab => "رجب",
            Self::Shaban => "شعبان",
            Self::Ramadan => "رمضان",
            Self::Shawwal => "شوال",
            Self::DhulQadah => "ذو القعدة",
            Self::DhulHijjah => "ذو الحجة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Muharram => "Muharram",
            Self::Safar => "Safar",
            Self::RabiAlAwwal => "Rabi' al-Awwal",
            Self::RabiAlThani => "Rabi' al-Thani",
            Self::JumadaAlUla => "Jumada al-Ula",
            Self::JumadaAlAkhirah => "Jumada al-Akhirah",
            Self::Rajab => "Rajab",
            Self::Shaban => "Sha'ban",
            Self::Ramadan => "Ramadan",
            Self::Shawwal => "Shawwal",
            Self::DhulQadah => "Dhul Qa'dah",
            Self::DhulHijjah => "Dhul Hijjah",
        }
    }

    /// Get month number (1-12)
    pub fn number(&self) -> u32 {
        match self {
            Self::Muharram => 1,
            Self::Safar => 2,
            Self::RabiAlAwwal => 3,
            Self::RabiAlThani => 4,
            Self::JumadaAlUla => 5,
            Self::JumadaAlAkhirah => 6,
            Self::Rajab => 7,
            Self::Shaban => 8,
            Self::Ramadan => 9,
            Self::Shawwal => 10,
            Self::DhulQadah => 11,
            Self::DhulHijjah => 12,
        }
    }

    /// Create from month number (1-12)
    pub fn from_number(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::Muharram),
            2 => Some(Self::Safar),
            3 => Some(Self::RabiAlAwwal),
            4 => Some(Self::RabiAlThani),
            5 => Some(Self::JumadaAlUla),
            6 => Some(Self::JumadaAlAkhirah),
            7 => Some(Self::Rajab),
            8 => Some(Self::Shaban),
            9 => Some(Self::Ramadan),
            10 => Some(Self::Shawwal),
            11 => Some(Self::DhulQadah),
            12 => Some(Self::DhulHijjah),
            _ => None,
        }
    }
}

/// Hijri (Islamic) date representation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HijriDate {
    /// Day of month (1-30)
    pub day: u32,
    /// Month
    pub month: HijriMonth,
    /// Year
    pub year: i32,
}

impl HijriDate {
    /// Create a new Hijri date
    pub fn new(day: u32, month: HijriMonth, year: i32) -> Option<Self> {
        if !(1..=30).contains(&day) {
            return None;
        }
        Some(Self { day, month, year })
    }

    /// Format as Arabic string
    pub fn format_ar(&self) -> String {
        format!("{}/{}/{}", self.day, self.month.name_ar(), self.year)
    }

    /// Format as English string
    pub fn format_en(&self) -> String {
        format!("{} {}, {}", self.day, self.month.name_en(), self.year)
    }
}

/// Types of Saudi public holidays
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SaudiHolidayType {
    /// National Day (اليوم الوطني السعودي)
    NationalDay,
    /// Founding Day (يوم التأسيس)
    FoundingDay,
    /// Eid al-Fitr (عيد الفطر) - After Ramadan
    EidAlFitr,
    /// Eid al-Adha (عيد الأضحى) - Hajj period
    EidAlAdha,
}

impl SaudiHolidayType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::NationalDay => "اليوم الوطني السعودي",
            Self::FoundingDay => "يوم التأسيس",
            Self::EidAlFitr => "عيد الفطر",
            Self::EidAlAdha => "عيد الأضحى",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::NationalDay => "Saudi National Day",
            Self::FoundingDay => "Saudi Founding Day",
            Self::EidAlFitr => "Eid al-Fitr",
            Self::EidAlAdha => "Eid al-Adha",
        }
    }

    /// Get typical duration in days
    pub fn typical_duration_days(&self) -> u32 {
        match self {
            Self::NationalDay => 1,
            Self::FoundingDay => 1,
            Self::EidAlFitr => 4, // Typically 4 days
            Self::EidAlAdha => 5, // Typically 5 days including Arafat Day
        }
    }
}

/// Saudi public holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaudiHoliday {
    /// Holiday type
    pub holiday_type: SaudiHolidayType,
    /// Hijri date
    pub hijri_date: HijriDate,
    /// Approximate Gregorian date (varies by moon sighting)
    pub gregorian_date: NaiveDate,
    /// Duration in days
    pub duration_days: u32,
}

impl SaudiHoliday {
    /// Create a new holiday
    pub fn new(
        holiday_type: SaudiHolidayType,
        hijri_date: HijriDate,
        gregorian_date: NaiveDate,
    ) -> Self {
        let duration_days = holiday_type.typical_duration_days();
        Self {
            holiday_type,
            hijri_date,
            gregorian_date,
            duration_days,
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> String {
        self.holiday_type.name_en().to_string()
    }

    /// Get description in Arabic
    pub fn description_ar(&self) -> String {
        self.holiday_type.name_ar().to_string()
    }
}

/// Get Saudi public holidays for a Hijri year
///
/// Note: Islamic holidays are based on lunar calendar and moon sighting,
/// so exact Gregorian dates may vary by 1-2 days.
pub fn get_public_holidays(hijri_year: i32) -> Vec<SaudiHoliday> {
    let mut holidays = Vec::new();

    // Eid al-Fitr (1 Shawwal)
    if let Some(eid_fitr_date) = HijriDate::new(1, HijriMonth::Shawwal, hijri_year) {
        // Approximate Gregorian conversion (simplified)
        let gregorian = approximate_hijri_to_gregorian(hijri_year, 10, 1);
        holidays.push(SaudiHoliday::new(
            SaudiHolidayType::EidAlFitr,
            eid_fitr_date,
            gregorian,
        ));
    }

    // Eid al-Adha (10 Dhul Hijjah)
    if let Some(eid_adha_date) = HijriDate::new(10, HijriMonth::DhulHijjah, hijri_year) {
        let gregorian = approximate_hijri_to_gregorian(hijri_year, 12, 10);
        holidays.push(SaudiHoliday::new(
            SaudiHolidayType::EidAlAdha,
            eid_adha_date,
            gregorian,
        ));
    }

    // Saudi National Day (September 23 - Gregorian calendar)
    // This is fixed to Gregorian calendar
    let national_day = NaiveDate::from_ymd_opt(hijri_year_to_gregorian_approx(hijri_year), 9, 23);
    if let Some(nd) = national_day
        && let Some(hijri) = convert_gregorian_to_hijri(nd)
    {
        holidays.push(SaudiHoliday::new(SaudiHolidayType::NationalDay, hijri, nd));
    }

    // Saudi Founding Day (February 22 - Gregorian calendar)
    let founding_day = NaiveDate::from_ymd_opt(hijri_year_to_gregorian_approx(hijri_year), 2, 22);
    if let Some(fd) = founding_day
        && let Some(hijri) = convert_gregorian_to_hijri(fd)
    {
        holidays.push(SaudiHoliday::new(SaudiHolidayType::FoundingDay, hijri, fd));
    }

    holidays
}

/// Check if a date is a Saudi public holiday
pub fn is_public_holiday(date: NaiveDate) -> bool {
    // Check National Day (September 23)
    if date.month() == 9 && date.day() == 23 {
        return true;
    }

    // Check Founding Day (February 22)
    if date.month() == 2 && date.day() == 22 {
        return true;
    }

    // For Eid holidays, would need lunar calendar calculation
    // This is a simplified check
    false
}

/// Check if a date is a working day in Saudi Arabia
///
/// Weekend in Saudi Arabia is Friday and Saturday
pub fn is_working_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    // Friday = 4, Saturday = 5 in chrono
    if weekday == chrono::Weekday::Fri || weekday == chrono::Weekday::Sat {
        return false;
    }

    !is_public_holiday(date)
}

/// Calculate working days between two dates
pub fn working_days_between(start: NaiveDate, end: NaiveDate) -> i64 {
    let mut count = 0;
    let mut current = start;

    while current <= end {
        if is_working_day(current) {
            count += 1;
        }
        current = current.succ_opt().unwrap_or(current);
        if current == start {
            break;
        }
    }

    count
}

/// Convert Gregorian date to Hijri (simplified approximation)
///
/// Note: This is a simplified calculation. For exact conversions,
/// use a proper Islamic calendar library.
pub fn convert_gregorian_to_hijri(gregorian: NaiveDate) -> Option<HijriDate> {
    // Simplified conversion formula
    let julian_day = gregorian_to_julian(gregorian);
    let hijri_year = ((julian_day - 1948440.0) / 354.367).floor() as i32;
    let hijri_month = ((julian_day - 1948440.0) % 354.367 / 29.531).floor() as u32 + 1;
    let hijri_day = ((julian_day - 1948440.0) % 29.531).floor() as u32 + 1;

    let month = HijriMonth::from_number(hijri_month.clamp(1, 12))?;
    HijriDate::new(hijri_day.clamp(1, 30), month, hijri_year)
}

/// Convert Hijri date to Gregorian (simplified approximation)
pub fn convert_hijri_to_gregorian(hijri: &HijriDate) -> Option<NaiveDate> {
    // Simplified conversion formula
    let julian_day = 1948440.0
        + (hijri.year as f64 * 354.367)
        + ((hijri.month.number() - 1) as f64 * 29.531)
        + hijri.day as f64;

    julian_to_gregorian(julian_day)
}

/// Convert Gregorian date to Julian day number
fn gregorian_to_julian(date: NaiveDate) -> f64 {
    let a = (14 - date.month() as i32) / 12;
    let y = date.year() + 4800 - a;
    let m = date.month() as i32 + 12 * a - 3;

    date.day() as f64 + ((153 * m + 2) / 5) as f64 + (365 * y) as f64 + (y / 4) as f64
        - (y / 100) as f64
        + (y / 400) as f64
        - 32045.0
}

/// Convert Julian day number to Gregorian date
fn julian_to_gregorian(jd: f64) -> Option<NaiveDate> {
    let j = jd.floor() as i32 + 32044;
    let g = j / 146097;
    let dg = j % 146097;
    let c = ((dg / 36524 + 1) * 3) / 4;
    let dc = dg - c * 36524;
    let b = dc / 1461;
    let db = dc % 1461;
    let a = ((db / 365 + 1) * 3) / 4;
    let da = db - a * 365;
    let y = g * 400 + c * 100 + b * 4 + a;
    let m = (da * 5 + 308) / 153 - 2;
    let d = da - ((m + 4) * 153) / 5 + 122;

    let year = y - 4800 + (m + 2) / 12;
    let month = ((m + 2) % 12 + 1) as u32;
    let day = (d + 1) as u32;

    NaiveDate::from_ymd_opt(year, month, day)
}

/// Approximate Hijri to Gregorian conversion helper
fn approximate_hijri_to_gregorian(hijri_year: i32, hijri_month: u32, hijri_day: u32) -> NaiveDate {
    // Very rough approximation: Hijri year 1446 ≈ Gregorian year 2024-2025
    let gregorian_year = hijri_year - 1446 + 2024;
    NaiveDate::from_ymd_opt(gregorian_year, hijri_month, hijri_day)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(gregorian_year, 1, 1).unwrap())
}

/// Convert Hijri year to approximate Gregorian year
fn hijri_year_to_gregorian_approx(hijri_year: i32) -> i32 {
    hijri_year - 1446 + 2024
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hijri_month_names() {
        assert_eq!(HijriMonth::Muharram.name_ar(), "محرم");
        assert_eq!(HijriMonth::Ramadan.name_en(), "Ramadan");
        assert_eq!(HijriMonth::DhulHijjah.name_ar(), "ذو الحجة");
    }

    #[test]
    fn test_hijri_month_numbers() {
        assert_eq!(HijriMonth::Muharram.number(), 1);
        assert_eq!(HijriMonth::Ramadan.number(), 9);
        assert_eq!(HijriMonth::DhulHijjah.number(), 12);
    }

    #[test]
    fn test_hijri_month_from_number() {
        assert_eq!(HijriMonth::from_number(1), Some(HijriMonth::Muharram));
        assert_eq!(HijriMonth::from_number(9), Some(HijriMonth::Ramadan));
        assert_eq!(HijriMonth::from_number(13), None);
    }

    #[test]
    fn test_hijri_date_creation() {
        let date = HijriDate::new(1, HijriMonth::Muharram, 1446);
        assert!(date.is_some());

        let invalid_date = HijriDate::new(35, HijriMonth::Muharram, 1446);
        assert!(invalid_date.is_none());
    }

    #[test]
    fn test_hijri_date_formatting() {
        let date = HijriDate::new(15, HijriMonth::Ramadan, 1446).unwrap();
        let ar = date.format_ar();
        assert!(ar.contains("رمضان"));

        let en = date.format_en();
        assert!(en.contains("Ramadan"));
        assert!(en.contains("1446"));
    }

    #[test]
    fn test_holiday_types() {
        assert_eq!(
            SaudiHolidayType::NationalDay.name_en(),
            "Saudi National Day"
        );
        assert_eq!(SaudiHolidayType::EidAlFitr.name_ar(), "عيد الفطر");
        assert_eq!(SaudiHolidayType::EidAlFitr.typical_duration_days(), 4);
        assert_eq!(SaudiHolidayType::EidAlAdha.typical_duration_days(), 5);
    }

    #[test]
    fn test_public_holidays() {
        let holidays = get_public_holidays(1446);
        assert!(!holidays.is_empty());
        assert!(holidays.len() >= 4); // At least 4 major holidays
    }

    #[test]
    fn test_is_public_holiday() {
        // National Day - September 23
        let national_day = NaiveDate::from_ymd_opt(2024, 9, 23).unwrap();
        assert!(is_public_holiday(national_day));

        // Founding Day - February 22
        let founding_day = NaiveDate::from_ymd_opt(2024, 2, 22).unwrap();
        assert!(is_public_holiday(founding_day));

        // Regular day
        let regular_day = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
        assert!(!is_public_holiday(regular_day));
    }

    #[test]
    fn test_is_working_day() {
        // Sunday (working day in Saudi Arabia)
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        assert!(is_working_day(sunday));

        // Friday (weekend)
        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
        assert!(!is_working_day(friday));

        // Saturday (weekend)
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        assert!(!is_working_day(saturday));
    }

    #[test]
    fn test_working_days_between() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        let working_days = working_days_between(start, end);
        assert!(working_days > 0);
    }
}
