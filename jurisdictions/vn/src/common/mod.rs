//! Common utilities for Vietnamese legal system
//!
//! This module provides shared utilities including:
//! - Vietnamese public holidays (Ngày lễ, Tết)
//! - Vietnamese Dong (VND) currency formatting
//! - Working day calculations

mod dates;

pub use dates::{
    VietnameseHoliday, VietnameseHolidayType, get_public_holidays, is_public_holiday,
    is_working_day, working_days_between,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Vietnamese Dong (VND) amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Vnd(pub i64);

impl Vnd {
    /// Create a new VND amount
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    /// Create from millions (triệu)
    pub fn from_trieu(trieu: i64) -> Self {
        Self(trieu * 1_000_000)
    }

    /// Create from billions (tỷ)
    pub fn from_ty(ty: i64) -> Self {
        Self(ty * 1_000_000_000)
    }

    /// Get the raw amount
    pub fn amount(&self) -> i64 {
        self.0
    }

    /// Format as Vietnamese style (1.000.000 đ)
    pub fn format_vi(&self) -> String {
        let amount = self.0;
        if amount < 0 {
            return format!("-{}", Self(-amount).format_vi());
        }

        let s = amount.to_string();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        if len <= 3 {
            return format!("{} đ", s);
        }

        let mut result = String::new();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                result.push('.');
            }
            result.push(*c);
        }
        format!("{} đ", result)
    }

    /// Format in triệu (millions) if appropriate
    pub fn format_trieu(&self) -> String {
        let trieu = self.0 / 1_000_000;
        let remainder = self.0 % 1_000_000;

        if remainder == 0 && trieu > 0 {
            format!("{} triệu đồng", trieu)
        } else {
            self.format_vi()
        }
    }
}

impl fmt::Display for Vnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_vi())
    }
}

impl std::ops::Add for Vnd {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Vnd {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<i64> for Vnd {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

/// Vietnamese regions for minimum wage calculation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WageRegion {
    /// Vùng I - Hanoi urban, HCMC urban, etc.
    Region1,
    /// Vùng II - Provincial cities
    Region2,
    /// Vùng III - District towns
    Region3,
    /// Vùng IV - Rural areas
    Region4,
}

impl WageRegion {
    /// Get region name in Vietnamese
    pub fn name_vi(&self) -> &str {
        match self {
            Self::Region1 => "Vùng I",
            Self::Region2 => "Vùng II",
            Self::Region3 => "Vùng III",
            Self::Region4 => "Vùng IV",
        }
    }

    /// Get 2024 minimum wage (per Decree 38/2022/ND-CP, adjusted)
    pub fn minimum_wage_2024(&self) -> Vnd {
        match self {
            Self::Region1 => Vnd(4_680_000),
            Self::Region2 => Vnd(4_160_000),
            Self::Region3 => Vnd(3_640_000),
            Self::Region4 => Vnd(3_250_000),
        }
    }

    /// Get description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::Region1 => "Urban districts of Hanoi, Ho Chi Minh City, major cities",
            Self::Region2 => "Provincial cities, industrial zones",
            Self::Region3 => "District towns, smaller cities",
            Self::Region4 => "Rural areas, remaining localities",
        }
    }
}

impl fmt::Display for WageRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_vi())
    }
}

/// Major Vietnamese cities/provinces
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Province {
    /// Hà Nội (Hanoi) - Capital
    HaNoi,
    /// TP. Hồ Chí Minh (Ho Chi Minh City)
    HoChiMinhCity,
    /// Đà Nẵng (Da Nang)
    DaNang,
    /// Hải Phòng (Hai Phong)
    HaiPhong,
    /// Cần Thơ (Can Tho)
    CanTho,
    /// Bình Dương (Binh Duong)
    BinhDuong,
    /// Đồng Nai (Dong Nai)
    DongNai,
    /// Other province
    Other(String),
}

impl Province {
    /// Get wage region for this province
    pub fn wage_region(&self) -> WageRegion {
        match self {
            Self::HaNoi | Self::HoChiMinhCity => WageRegion::Region1,
            Self::DaNang | Self::HaiPhong | Self::CanTho | Self::BinhDuong | Self::DongNai => {
                WageRegion::Region2
            }
            Self::Other(_) => WageRegion::Region3,
        }
    }

    /// Get province name in Vietnamese
    pub fn name_vi(&self) -> &str {
        match self {
            Self::HaNoi => "Hà Nội",
            Self::HoChiMinhCity => "TP. Hồ Chí Minh",
            Self::DaNang => "Đà Nẵng",
            Self::HaiPhong => "Hải Phòng",
            Self::CanTho => "Cần Thơ",
            Self::BinhDuong => "Bình Dương",
            Self::DongNai => "Đồng Nai",
            Self::Other(name) => name,
        }
    }
}

impl fmt::Display for Province {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_vi())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vnd_formatting() {
        assert_eq!(Vnd(1_000).format_vi(), "1.000 đ");
        assert_eq!(Vnd(1_000_000).format_vi(), "1.000.000 đ");
        assert_eq!(Vnd(4_680_000).format_vi(), "4.680.000 đ");
    }

    #[test]
    fn test_vnd_trieu() {
        assert_eq!(Vnd::from_trieu(5).amount(), 5_000_000);
        assert_eq!(Vnd(5_000_000).format_trieu(), "5 triệu đồng");
    }

    #[test]
    fn test_vnd_operations() {
        let a = Vnd(1_000_000);
        let b = Vnd(500_000);

        assert_eq!((a + b).amount(), 1_500_000);
        assert_eq!((a - b).amount(), 500_000);
        assert_eq!((a * 3).amount(), 3_000_000);
    }

    #[test]
    fn test_wage_region() {
        assert_eq!(WageRegion::Region1.minimum_wage_2024().amount(), 4_680_000);
        assert_eq!(Province::HaNoi.wage_region(), WageRegion::Region1);
        assert_eq!(Province::DaNang.wage_region(), WageRegion::Region2);
    }
}
