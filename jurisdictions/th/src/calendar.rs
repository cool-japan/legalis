//! Buddhist Era (พุทธศักราช - พ.ศ.) Calendar System for Thailand
//!
//! Thailand uses the Buddhist Era (BE) calendar for official government documents and legal citations.
//! The Buddhist Era is 543 years ahead of the Common Era (CE) / Anno Domini (AD).
//!
//! ## Conversion Formula
//!
//! - **BE to CE**: CE = BE - 543
//! - **CE to BE**: BE = CE + 543
//!
//! ## Examples
//!
//! - 2024 CE = 2567 BE (พ.ศ. 2567)
//! - 2025 CE = 2568 BE (พ.ศ. 2568)
//! - 1997 CE = 2540 BE (พ.ศ. 2540) - Constitution of 1997
//! - 2017 CE = 2560 BE (พ.ศ. 2560) - Constitution of 2017
//!
//! ## Historical Context
//!
//! The Buddhist Era calendar has been officially used in Thailand since 1889.
//! It commemorates the traditional date of the Buddha's passing into Parinirvana (543 BCE).
//!
//! ## Usage in Thai Law
//!
//! All Thai legal documents, statutes, and citations use Buddhist Era years:
//! - พ.ร.บ. (Act/Law) citations: "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562" (PDPA B.E. 2562 / 2019 CE)
//! - Constitutional references: "รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560" (Constitution B.E. 2560 / 2017 CE)
//! - Court judgments: Dated in BE format
//!
//! ```
//! use legalis_th::calendar::{ce_to_be, be_to_ce, BuddhistYear, format_buddhist_year};
//!
//! // Convert CE to BE
//! assert_eq!(ce_to_be(2024), 2567);
//! assert_eq!(ce_to_be(2019), 2562); // PDPA year
//!
//! // Convert BE to CE
//! assert_eq!(be_to_ce(2567), 2024);
//! assert_eq!(be_to_ce(2562), 2019); // PDPA year
//!
//! // Format Buddhist year
//! assert_eq!(format_buddhist_year(2567), "พ.ศ. 2567");
//! ```

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// The offset between Buddhist Era and Common Era (543 years)
pub const BE_CE_OFFSET: i32 = 543;

/// Converts Common Era (CE/AD) year to Buddhist Era (BE) year
///
/// # Examples
///
/// ```
/// use legalis_th::calendar::ce_to_be;
///
/// assert_eq!(ce_to_be(2024), 2567);
/// assert_eq!(ce_to_be(2019), 2562); // PDPA B.E. 2562
/// assert_eq!(ce_to_be(2017), 2560); // Constitution B.E. 2560
/// assert_eq!(ce_to_be(1997), 2540); // Constitution B.E. 2540
/// ```
#[inline]
pub fn ce_to_be(ce_year: i32) -> i32 {
    ce_year + BE_CE_OFFSET
}

/// Converts Buddhist Era (BE) year to Common Era (CE/AD) year
///
/// # Examples
///
/// ```
/// use legalis_th::calendar::be_to_ce;
///
/// assert_eq!(be_to_ce(2567), 2024);
/// assert_eq!(be_to_ce(2562), 2019); // PDPA 2019
/// assert_eq!(be_to_ce(2560), 2017); // Constitution 2017
/// assert_eq!(be_to_ce(2540), 1997); // Constitution 1997
/// ```
#[inline]
pub fn be_to_ce(be_year: i32) -> i32 {
    be_year - BE_CE_OFFSET
}

/// Formats a Buddhist Era year in Thai format: "พ.ศ. [year]"
///
/// # Examples
///
/// ```
/// use legalis_th::calendar::format_buddhist_year;
///
/// assert_eq!(format_buddhist_year(2567), "พ.ศ. 2567");
/// assert_eq!(format_buddhist_year(2562), "พ.ศ. 2562");
/// ```
pub fn format_buddhist_year(be_year: i32) -> String {
    format!("พ.ศ. {}", be_year)
}

/// Formats a Buddhist Era year in English format: "B.E. [year]"
///
/// # Examples
///
/// ```
/// use legalis_th::calendar::format_buddhist_year_en;
///
/// assert_eq!(format_buddhist_year_en(2567), "B.E. 2567");
/// assert_eq!(format_buddhist_year_en(2562), "B.E. 2562");
/// ```
pub fn format_buddhist_year_en(be_year: i32) -> String {
    format!("B.E. {}", be_year)
}

/// Represents a year that can be expressed in both Buddhist Era and Common Era
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BuddhistYear {
    /// Buddhist Era year (e.g., 2567)
    pub be_year: i32,
}

impl BuddhistYear {
    /// Creates a BuddhistYear from a Buddhist Era year
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistYear;
    ///
    /// let year = BuddhistYear::from_be(2567);
    /// assert_eq!(year.be_year, 2567);
    /// assert_eq!(year.ce_year(), 2024);
    /// ```
    pub fn from_be(be_year: i32) -> Self {
        Self { be_year }
    }

    /// Creates a BuddhistYear from a Common Era year
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistYear;
    ///
    /// let year = BuddhistYear::from_ce(2024);
    /// assert_eq!(year.be_year, 2567);
    /// assert_eq!(year.ce_year(), 2024);
    /// ```
    pub fn from_ce(ce_year: i32) -> Self {
        Self {
            be_year: ce_to_be(ce_year),
        }
    }

    /// Returns the Common Era year
    pub fn ce_year(&self) -> i32 {
        be_to_ce(self.be_year)
    }

    /// Formats the year in Thai format: "พ.ศ. [year]"
    pub fn format_th(&self) -> String {
        format_buddhist_year(self.be_year)
    }

    /// Formats the year in English format: "B.E. [year]"
    pub fn format_en(&self) -> String {
        format_buddhist_year_en(self.be_year)
    }

    /// Returns the current Buddhist Era year
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistYear;
    ///
    /// let current = BuddhistYear::current();
    /// // Will be 2567 in 2024, 2568 in 2025, etc.
    /// ```
    pub fn current() -> Self {
        let now = Utc::now();
        Self::from_ce(now.year())
    }
}

impl fmt::Display for BuddhistYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "พ.ศ. {} ({})", self.be_year, self.ce_year())
    }
}

impl From<i32> for BuddhistYear {
    /// Creates a BuddhistYear from a Buddhist Era year
    fn from(be_year: i32) -> Self {
        Self::from_be(be_year)
    }
}

/// Represents a complete date in Buddhist Era calendar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BuddhistDate {
    /// Day of month (1-31)
    pub day: u32,
    /// Month (1-12)
    pub month: u32,
    /// Buddhist Era year
    pub year: BuddhistYear,
}

impl BuddhistDate {
    /// Creates a new BuddhistDate from Buddhist Era components
    ///
    /// # Arguments
    ///
    /// * `day` - Day of month (1-31)
    /// * `month` - Month (1-12)
    /// * `be_year` - Buddhist Era year
    ///
    /// # Returns
    ///
    /// Returns `Some(BuddhistDate)` if the date is valid, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    ///
    /// let date = BuddhistDate::new(10, 5, 2567).expect("valid date");
    /// assert_eq!(date.day, 10);
    /// assert_eq!(date.month, 5);
    /// assert_eq!(date.year.be_year, 2567);
    /// ```
    pub fn new(day: u32, month: u32, be_year: i32) -> Option<Self> {
        // Validate using NaiveDate
        let ce_year = be_to_ce(be_year);
        NaiveDate::from_ymd_opt(ce_year, month, day)?;

        Some(Self {
            day,
            month,
            year: BuddhistYear::from_be(be_year),
        })
    }

    /// Creates a BuddhistDate from a Common Era date
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    ///
    /// let date = BuddhistDate::from_ce_date(10, 5, 2024).expect("valid date");
    /// assert_eq!(date.year.be_year, 2567);
    /// ```
    pub fn from_ce_date(day: u32, month: u32, ce_year: i32) -> Option<Self> {
        Self::new(day, month, ce_to_be(ce_year))
    }

    /// Creates a BuddhistDate from a chrono NaiveDate
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    /// use chrono::NaiveDate;
    ///
    /// let naive = NaiveDate::from_ymd_opt(2024, 5, 10).unwrap();
    /// let date = BuddhistDate::from_naive_date(naive);
    /// assert_eq!(date.year.be_year, 2567);
    /// ```
    pub fn from_naive_date(date: NaiveDate) -> Self {
        Self {
            day: date.day(),
            month: date.month(),
            year: BuddhistYear::from_ce(date.year()),
        }
    }

    /// Creates a BuddhistDate from a chrono DateTime
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    /// use chrono::Utc;
    ///
    /// let now = Utc::now();
    /// let date = BuddhistDate::from_datetime(&now);
    /// ```
    pub fn from_datetime<Tz: chrono::TimeZone>(dt: &DateTime<Tz>) -> Self {
        Self::from_naive_date(dt.date_naive())
    }

    /// Converts to a chrono NaiveDate
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    ///
    /// let date = BuddhistDate::new(10, 5, 2567).unwrap();
    /// let naive = date.to_naive_date();
    /// assert_eq!(naive.year(), 2024);
    /// ```
    pub fn to_naive_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year.ce_year(), self.month, self.day)
            .expect("BuddhistDate should always be valid")
    }

    /// Returns the current Buddhist Era date
    pub fn today() -> Self {
        Self::from_datetime(&Utc::now())
    }

    /// Formats the date in Thai format: "วันที่ [day] เดือน[month] พ.ศ. [year]"
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::calendar::BuddhistDate;
    ///
    /// let date = BuddhistDate::new(10, 5, 2567).unwrap();
    /// assert_eq!(date.format_th(), "10 พฤษภาคม พ.ศ. 2567");
    /// ```
    pub fn format_th(&self) -> String {
        format!(
            "{} {} พ.ศ. {}",
            self.day,
            thai_month_name(self.month),
            self.year.be_year
        )
    }

    /// Formats the date in English format: "[day] [month] B.E. [year]"
    pub fn format_en(&self) -> String {
        format!(
            "{} {} B.E. {}",
            self.day,
            english_month_name(self.month),
            self.year.be_year
        )
    }

    /// Formats the date in ISO 8601 format with BE year
    pub fn format_iso_be(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year.be_year, self.month, self.day)
    }

    /// Formats the date in ISO 8601 format with CE year
    pub fn format_iso_ce(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.year.ce_year(),
            self.month,
            self.day
        )
    }
}

impl fmt::Display for BuddhistDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Returns Thai month name
fn thai_month_name(month: u32) -> &'static str {
    match month {
        1 => "มกราคม",
        2 => "กุมภาพันธ์",
        3 => "มีนาคม",
        4 => "เมษายน",
        5 => "พฤษภาคม",
        6 => "มิถุนายน",
        7 => "กรกฎาคม",
        8 => "สิงหาคม",
        9 => "กันยายน",
        10 => "ตุลาคม",
        11 => "พฤศจิกายน",
        12 => "ธันวาคม",
        _ => "ไม่ทราบ",
    }
}

/// Returns English month name
fn english_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Thai Era periods for historical reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThaiEra {
    /// Rattanakosin Era (from 1782 CE / 2325 BE) - Current Bangkok dynasty
    Rattanakosin,
    /// Buddhist Era (official calendar since 1889)
    Buddhist,
}

impl ThaiEra {
    /// Returns the Thai name of the era
    pub fn name_th(&self) -> &'static str {
        match self {
            ThaiEra::Rattanakosin => "รัตนโกสินทร์",
            ThaiEra::Buddhist => "พุทธศักราช",
        }
    }

    /// Returns the English name of the era
    pub fn name_en(&self) -> &'static str {
        match self {
            ThaiEra::Rattanakosin => "Rattanakosin Era",
            ThaiEra::Buddhist => "Buddhist Era",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ce_to_be() {
        assert_eq!(ce_to_be(2024), 2567);
        assert_eq!(ce_to_be(2019), 2562);
        assert_eq!(ce_to_be(2017), 2560);
        assert_eq!(ce_to_be(1997), 2540);
        assert_eq!(ce_to_be(1932), 2475); // Siamese Revolution
    }

    #[test]
    fn test_be_to_ce() {
        assert_eq!(be_to_ce(2567), 2024);
        assert_eq!(be_to_ce(2562), 2019);
        assert_eq!(be_to_ce(2560), 2017);
        assert_eq!(be_to_ce(2540), 1997);
        assert_eq!(be_to_ce(2475), 1932);
    }

    #[test]
    fn test_round_trip() {
        let ce_year = 2024;
        assert_eq!(be_to_ce(ce_to_be(ce_year)), ce_year);

        let be_year = 2567;
        assert_eq!(ce_to_be(be_to_ce(be_year)), be_year);
    }

    #[test]
    fn test_format_buddhist_year() {
        assert_eq!(format_buddhist_year(2567), "พ.ศ. 2567");
        assert_eq!(format_buddhist_year(2562), "พ.ศ. 2562");
    }

    #[test]
    fn test_format_buddhist_year_en() {
        assert_eq!(format_buddhist_year_en(2567), "B.E. 2567");
        assert_eq!(format_buddhist_year_en(2562), "B.E. 2562");
    }

    #[test]
    fn test_buddhist_year() {
        let year = BuddhistYear::from_be(2567);
        assert_eq!(year.be_year, 2567);
        assert_eq!(year.ce_year(), 2024);
        assert_eq!(year.format_th(), "พ.ศ. 2567");
        assert_eq!(year.format_en(), "B.E. 2567");
    }

    #[test]
    fn test_buddhist_year_from_ce() {
        let year = BuddhistYear::from_ce(2024);
        assert_eq!(year.be_year, 2567);
        assert_eq!(year.ce_year(), 2024);
    }

    #[test]
    fn test_buddhist_date() {
        let date = BuddhistDate::new(10, 5, 2567).expect("valid date");
        assert_eq!(date.day, 10);
        assert_eq!(date.month, 5);
        assert_eq!(date.year.be_year, 2567);
        assert_eq!(date.year.ce_year(), 2024);
    }

    #[test]
    fn test_buddhist_date_from_ce() {
        let date = BuddhistDate::from_ce_date(10, 5, 2024).expect("valid date");
        assert_eq!(date.day, 10);
        assert_eq!(date.month, 5);
        assert_eq!(date.year.be_year, 2567);
    }

    #[test]
    fn test_buddhist_date_invalid() {
        // Invalid dates
        assert!(BuddhistDate::new(32, 1, 2567).is_none()); // Day 32
        assert!(BuddhistDate::new(1, 13, 2567).is_none()); // Month 13
        assert!(BuddhistDate::new(30, 2, 2567).is_none()); // Feb 30
    }

    #[test]
    fn test_buddhist_date_format_th() {
        let date = BuddhistDate::new(10, 5, 2567).unwrap();
        assert_eq!(date.format_th(), "10 พฤษภาคม พ.ศ. 2567");

        let date2 = BuddhistDate::new(1, 1, 2562).unwrap();
        assert_eq!(date2.format_th(), "1 มกราคม พ.ศ. 2562");
    }

    #[test]
    fn test_buddhist_date_format_en() {
        let date = BuddhistDate::new(10, 5, 2567).unwrap();
        assert_eq!(date.format_en(), "10 May B.E. 2567");
    }

    #[test]
    fn test_buddhist_date_iso() {
        let date = BuddhistDate::new(10, 5, 2567).unwrap();
        assert_eq!(date.format_iso_be(), "2567-05-10");
        assert_eq!(date.format_iso_ce(), "2024-05-10");
    }

    #[test]
    fn test_buddhist_date_to_naive() {
        let date = BuddhistDate::new(10, 5, 2567).unwrap();
        let naive = date.to_naive_date();
        assert_eq!(naive.year(), 2024);
        assert_eq!(naive.month(), 5);
        assert_eq!(naive.day(), 10);
    }

    #[test]
    fn test_buddhist_date_from_naive() {
        let naive = NaiveDate::from_ymd_opt(2024, 5, 10).unwrap();
        let date = BuddhistDate::from_naive_date(naive);
        assert_eq!(date.day, 10);
        assert_eq!(date.month, 5);
        assert_eq!(date.year.be_year, 2567);
    }

    #[test]
    fn test_thai_month_names() {
        assert_eq!(thai_month_name(1), "มกราคม");
        assert_eq!(thai_month_name(5), "พฤษภาคม");
        assert_eq!(thai_month_name(12), "ธันวาคม");
    }

    #[test]
    fn test_english_month_names() {
        assert_eq!(english_month_name(1), "January");
        assert_eq!(english_month_name(5), "May");
        assert_eq!(english_month_name(12), "December");
    }

    #[test]
    fn test_thai_era() {
        let era = ThaiEra::Buddhist;
        assert_eq!(era.name_th(), "พุทธศักราช");
        assert_eq!(era.name_en(), "Buddhist Era");

        let rattanakosin = ThaiEra::Rattanakosin;
        assert_eq!(rattanakosin.name_th(), "รัตนโกสินทร์");
        assert_eq!(rattanakosin.name_en(), "Rattanakosin Era");
    }

    #[test]
    fn test_major_thai_law_years() {
        // Constitution 2017
        assert_eq!(ce_to_be(2017), 2560);
        // PDPA 2019
        assert_eq!(ce_to_be(2019), 2562);
        // Civil and Commercial Code 1992 (actual enactment)
        assert_eq!(ce_to_be(1992), 2535);
        // Foreign Business Act 1999
        assert_eq!(ce_to_be(1999), 2542);
        // Labour Protection Act 1998
        assert_eq!(ce_to_be(1998), 2541);
        // BOI Act 1977
        assert_eq!(ce_to_be(1977), 2520);
        // Land Code 1954
        assert_eq!(ce_to_be(1954), 2497);
    }
}
