//! Common utilities for Indonesian legal system
//!
//! This module provides shared utilities including:
//! - Indonesian public holidays (Hari Libur Nasional)
//! - Rupiah currency formatting
//! - Indonesian name handling
//! - Working day calculations

mod dates;

pub use dates::{
    IndonesianHoliday, IndonesianHolidayType, get_national_holidays, is_national_holiday,
    is_working_day, working_days_between,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Indonesian Rupiah amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Rupiah(pub i64);

impl Rupiah {
    /// Create a new Rupiah amount
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    /// Create from millions (juta)
    pub fn from_juta(juta: i64) -> Self {
        Self(juta * 1_000_000)
    }

    /// Create from billions (miliar)
    pub fn from_miliar(miliar: i64) -> Self {
        Self(miliar * 1_000_000_000)
    }

    /// Create from trillions (triliun)
    pub fn from_triliun(triliun: i64) -> Self {
        Self(triliun * 1_000_000_000_000)
    }

    /// Get the raw amount
    pub fn amount(&self) -> i64 {
        self.0
    }

    /// Format as Indonesian style (Rp 1.000.000)
    pub fn format_id(&self) -> String {
        let amount = self.0;
        if amount < 0 {
            return format!("-{}", Self(-amount).format_id());
        }

        let s = amount.to_string();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        if len <= 3 {
            return format!("Rp {}", s);
        }

        let mut result = String::new();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                result.push('.');
            }
            result.push(*c);
        }
        format!("Rp {}", result)
    }

    /// Format in juta (millions) if appropriate
    pub fn format_juta(&self) -> String {
        let juta = self.0 / 1_000_000;
        let remainder = self.0 % 1_000_000;

        if remainder == 0 && juta > 0 {
            format!("Rp {} juta", juta)
        } else {
            self.format_id()
        }
    }
}

impl fmt::Display for Rupiah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_id())
    }
}

impl std::ops::Add for Rupiah {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Rupiah {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i64> for Rupiah {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

/// Indonesian provinces
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Province {
    /// DKI Jakarta
    DkiJakarta,
    /// Jawa Barat
    JawaBarat,
    /// Jawa Tengah
    JawaTengah,
    /// Jawa Timur
    JawaTimur,
    /// Banten
    Banten,
    /// DI Yogyakarta
    DiYogyakarta,
    /// Bali
    Bali,
    /// Sumatera Utara
    SumateraUtara,
    /// Sumatera Barat
    SumateraBarat,
    /// Kalimantan Timur
    KalimantanTimur,
    /// Sulawesi Selatan
    SulawesiSelatan,
    /// Aceh (special region - Sharia law applies)
    Aceh,
    /// Papua
    Papua,
    /// Other province
    Other(String),
}

impl Province {
    /// Get province name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::DkiJakarta => "DKI Jakarta",
            Self::JawaBarat => "Jawa Barat",
            Self::JawaTengah => "Jawa Tengah",
            Self::JawaTimur => "Jawa Timur",
            Self::Banten => "Banten",
            Self::DiYogyakarta => "DI Yogyakarta",
            Self::Bali => "Bali",
            Self::SumateraUtara => "Sumatera Utara",
            Self::SumateraBarat => "Sumatera Barat",
            Self::KalimantanTimur => "Kalimantan Timur",
            Self::SulawesiSelatan => "Sulawesi Selatan",
            Self::Aceh => "Aceh",
            Self::Papua => "Papua",
            Self::Other(name) => name,
        }
    }

    /// Check if province has special Sharia law provisions
    pub fn has_sharia_law(&self) -> bool {
        matches!(self, Self::Aceh)
    }

    /// Get 2024 minimum wage (UMK/UMP) in Rupiah
    pub fn minimum_wage_2024(&self) -> Rupiah {
        match self {
            Self::DkiJakarta => Rupiah(5_067_381),
            Self::JawaBarat => Rupiah(2_057_495),  // UMP base
            Self::JawaTengah => Rupiah(2_035_000), // UMP base
            Self::JawaTimur => Rupiah(2_165_244),  // UMP base
            Self::Banten => Rupiah(2_661_280),
            Self::DiYogyakarta => Rupiah(2_125_898),
            Self::Bali => Rupiah(2_813_672),
            Self::SumateraUtara => Rupiah(2_809_915),
            Self::SumateraBarat => Rupiah(2_742_476),
            Self::KalimantanTimur => Rupiah(3_335_479),
            Self::SulawesiSelatan => Rupiah(3_485_014),
            Self::Aceh => Rupiah(3_413_666),
            Self::Papua => Rupiah(3_864_696),
            Self::Other(_) => Rupiah(2_000_000), // Default estimate
        }
    }
}

impl fmt::Display for Province {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rupiah_formatting() {
        assert_eq!(Rupiah(1_000).format_id(), "Rp 1.000");
        assert_eq!(Rupiah(1_000_000).format_id(), "Rp 1.000.000");
        assert_eq!(Rupiah(5_067_381).format_id(), "Rp 5.067.381");
    }

    #[test]
    fn test_rupiah_juta() {
        assert_eq!(Rupiah::from_juta(5).amount(), 5_000_000);
        assert_eq!(Rupiah(5_000_000).format_juta(), "Rp 5 juta");
        assert_eq!(Rupiah(5_500_000).format_juta(), "Rp 5.500.000"); // Not exact juta
    }

    #[test]
    fn test_rupiah_operations() {
        let a = Rupiah(1_000_000);
        let b = Rupiah(500_000);

        assert_eq!((a + b).amount(), 1_500_000);
        assert_eq!((a - b).amount(), 500_000);
        assert_eq!((a * 3).amount(), 3_000_000);
    }

    #[test]
    fn test_province_minimum_wage() {
        assert_eq!(Province::DkiJakarta.minimum_wage_2024().amount(), 5_067_381);
        assert!(Province::Aceh.has_sharia_law());
        assert!(!Province::Bali.has_sharia_law());
    }
}
