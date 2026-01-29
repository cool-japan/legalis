//! Common Utilities for Saudi Legal System
//!
//! Provides shared types and functions including:
//! - SAR (Saudi Riyal) currency formatting
//! - Hijri (Islamic) calendar support
//! - Saudi public holidays

pub mod currency;
pub mod holidays;

pub use currency::Sar;
pub use holidays::{
    HijriDate, HijriMonth, SaudiHoliday, SaudiHolidayType, convert_gregorian_to_hijri,
    convert_hijri_to_gregorian, get_public_holidays, is_public_holiday, is_working_day,
    working_days_between,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sar_creation() {
        let sar = Sar::from_riyals(1000);
        assert_eq!(sar.riyals(), 1000);
        assert_eq!(sar.halalas(), 100000);
    }

    #[test]
    fn test_sar_formatting() {
        let sar = Sar::from_riyals(10000);
        assert_eq!(sar.format_en(), "SAR 10,000.00");
        assert!(sar.format_ar().contains("ر.س"));
    }

    #[test]
    fn test_hijri_month_names() {
        assert_eq!(HijriMonth::Muharram.name_ar(), "محرم");
        assert_eq!(HijriMonth::Ramadan.name_en(), "Ramadan");
        assert_eq!(HijriMonth::DhulHijjah.name_en(), "Dhul Hijjah");
    }
}
