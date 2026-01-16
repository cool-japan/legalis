//! Minimum Wage Validation (最低賃金検証)
//!
//! Regional minimum wage data and validation for all 47 prefectures.
//! Data is based on 2024 minimum wage rates (令和6年度).
//!
//! # Legal Basis
//!
//! Minimum Wage Act (最低賃金法 - Saiteichingin-hō, Act No. 137 of 1959)
//!
//! # Example
//!
//! ```
//! use legalis_jp::labor_law::minimum_wage::{Prefecture, get_minimum_wage};
//! use chrono::Utc;
//!
//! let tokyo_minimum = get_minimum_wage(Prefecture::Tokyo, Utc::now().date_naive());
//! assert_eq!(tokyo_minimum, 1_113); // ¥1,113/hour in 2024
//! ```

use super::error::{LaborLawError, Result};
use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Prefecture enumeration for minimum wage validation (都道府県)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Prefecture {
    /// Hokkaido (北海道)
    Hokkaido,
    /// Aomori (青森県)
    Aomori,
    /// Iwate (岩手県)
    Iwate,
    /// Miyagi (宮城県)
    Miyagi,
    /// Akita (秋田県)
    Akita,
    /// Yamagata (山形県)
    Yamagata,
    /// Fukushima (福島県)
    Fukushima,
    /// Ibaraki (茨城県)
    Ibaraki,
    /// Tochigi (栃木県)
    Tochigi,
    /// Gunma (群馬県)
    Gunma,
    /// Saitama (埼玉県)
    Saitama,
    /// Chiba (千葉県)
    Chiba,
    /// Tokyo (東京都)
    Tokyo,
    /// Kanagawa (神奈川県)
    Kanagawa,
    /// Niigata (新潟県)
    Niigata,
    /// Toyama (富山県)
    Toyama,
    /// Ishikawa (石川県)
    Ishikawa,
    /// Fukui (福井県)
    Fukui,
    /// Yamanashi (山梨県)
    Yamanashi,
    /// Nagano (長野県)
    Nagano,
    /// Gifu (岐阜県)
    Gifu,
    /// Shizuoka (静岡県)
    Shizuoka,
    /// Aichi (愛知県)
    Aichi,
    /// Mie (三重県)
    Mie,
    /// Shiga (滋賀県)
    Shiga,
    /// Kyoto (京都府)
    Kyoto,
    /// Osaka (大阪府)
    Osaka,
    /// Hyogo (兵庫県)
    Hyogo,
    /// Nara (奈良県)
    Nara,
    /// Wakayama (和歌山県)
    Wakayama,
    /// Tottori (鳥取県)
    Tottori,
    /// Shimane (島根県)
    Shimane,
    /// Okayama (岡山県)
    Okayama,
    /// Hiroshima (広島県)
    Hiroshima,
    /// Yamaguchi (山口県)
    Yamaguchi,
    /// Tokushima (徳島県)
    Tokushima,
    /// Kagawa (香川県)
    Kagawa,
    /// Ehime (愛媛県)
    Ehime,
    /// Kochi (高知県)
    Kochi,
    /// Fukuoka (福岡県)
    Fukuoka,
    /// Saga (佐賀県)
    Saga,
    /// Nagasaki (長崎県)
    Nagasaki,
    /// Kumamoto (熊本県)
    Kumamoto,
    /// Oita (大分県)
    Oita,
    /// Miyazaki (宮崎県)
    Miyazaki,
    /// Kagoshima (鹿児島県)
    Kagoshima,
    /// Okinawa (沖縄県)
    Okinawa,
}

/// Minimum wage data structure (最低賃金データ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MinimumWageData {
    /// Prefecture
    pub prefecture: Prefecture,
    /// Effective date (施行日)
    pub effective_date: NaiveDate,
    /// Hourly rate in JPY (時間額)
    pub hourly_rate_jpy: u64,
}

/// Get minimum wage for a prefecture on a specific date (最低賃金取得)
///
/// Returns the applicable minimum hourly wage in JPY.
/// Data is based on 2024 rates (令和6年度).
///
/// # Example
///
/// ```
/// use legalis_jp::labor_law::minimum_wage::{Prefecture, get_minimum_wage};
/// use chrono::Utc;
///
/// let tokyo = get_minimum_wage(Prefecture::Tokyo, Utc::now().date_naive());
/// assert_eq!(tokyo, 1_113);
///
/// let osaka = get_minimum_wage(Prefecture::Osaka, Utc::now().date_naive());
/// assert_eq!(osaka, 1_064);
/// ```
pub fn get_minimum_wage(prefecture: Prefecture, _date: NaiveDate) -> u64 {
    // TODO: In the future, support historical minimum wage data
    // For now, return 2024 rates
    match prefecture {
        Prefecture::Tokyo => 1_113,
        Prefecture::Kanagawa => 1_112,
        Prefecture::Osaka => 1_064,
        Prefecture::Saitama => 1_028,
        Prefecture::Aichi => 1_027,
        Prefecture::Chiba => 1_026,
        Prefecture::Kyoto => 1_008,
        Prefecture::Hyogo => 1_001,
        Prefecture::Hiroshima => 970,
        Prefecture::Fukuoka => 941,
        Prefecture::Hokkaido => 960,
        Prefecture::Miyagi => 923,
        Prefecture::Niigata => 931,
        Prefecture::Shizuoka => 984,
        Prefecture::Gifu => 950,
        Prefecture::Mie => 973,
        Prefecture::Shiga => 967,
        Prefecture::Nara => 936,
        Prefecture::Wakayama => 929,
        Prefecture::Okayama => 932,
        Prefecture::Yamaguchi => 928,
        Prefecture::Kagawa => 918,
        Prefecture::Ehime => 897,
        Prefecture::Fukui => 931,
        Prefecture::Toyama => 948,
        Prefecture::Ishikawa => 933,
        Prefecture::Nagano => 948,
        Prefecture::Yamanashi => 938,
        Prefecture::Tochigi => 954,
        Prefecture::Gunma => 935,
        Prefecture::Ibaraki => 953,
        Prefecture::Aomori => 898,
        Prefecture::Iwate => 893,
        Prefecture::Akita => 897,
        Prefecture::Yamagata => 900,
        Prefecture::Fukushima => 900,
        Prefecture::Tottori => 900,
        Prefecture::Shimane => 904,
        Prefecture::Tokushima => 896,
        Prefecture::Kochi => 897,
        Prefecture::Saga => 900,
        Prefecture::Nagasaki => 898,
        Prefecture::Kumamoto => 898,
        Prefecture::Oita => 899,
        Prefecture::Miyazaki => 897,
        Prefecture::Kagoshima => 897,
        Prefecture::Okinawa => 896,
    }
}

/// Validate salary against regional minimum wage (最低賃金検証)
///
/// Calculates the effective hourly rate and compares against the
/// prefecture's minimum wage.
///
/// # Arguments
///
/// * `base_salary_monthly` - Monthly base salary in JPY
/// * `hours_per_month` - Expected working hours per month
/// * `prefecture` - Prefecture for minimum wage lookup
///
/// # Returns
///
/// Ok(()) if salary meets minimum wage requirements
/// Err if salary is below minimum wage
///
/// # Example
///
/// ```
/// use legalis_jp::labor_law::minimum_wage::{Prefecture, validate_salary_against_minimum};
///
/// // ¥400,000/month for 173.3 hours (40h/week * 52 weeks / 12 months)
/// let result = validate_salary_against_minimum(400_000, 173.3, Prefecture::Tokyo);
/// assert!(result.is_ok()); // ¥2,308/hour > ¥1,113 minimum
///
/// // ¥150,000/month is too low
/// let result = validate_salary_against_minimum(150_000, 173.3, Prefecture::Tokyo);
/// assert!(result.is_err());
/// ```
pub fn validate_salary_against_minimum(
    base_salary_monthly: u64,
    hours_per_month: f64,
    prefecture: Prefecture,
) -> Result<()> {
    if hours_per_month <= 0.0 {
        return Err(LaborLawError::InvalidCalculation {
            reason: "Hours per month must be positive".to_string(),
        });
    }

    // Calculate hourly rate
    let hourly_rate = base_salary_monthly as f64 / hours_per_month;

    // Get regional minimum wage
    let minimum_wage = get_minimum_wage(prefecture, chrono::Utc::now().date_naive());

    // Check if salary meets minimum
    if (hourly_rate as u64) < minimum_wage {
        return Err(LaborLawError::BelowMinimumWage {
            actual_hourly: hourly_rate as u64,
            required_minimum: minimum_wage,
            prefecture,
        });
    }

    Ok(())
}

/// Calculate monthly hours from daily and weekly parameters
///
/// # Example
///
/// ```
/// use legalis_jp::labor_law::minimum_wage::calculate_monthly_hours;
///
/// // 8 hours/day, 5 days/week = 40 hours/week
/// // 40 hours * 52 weeks / 12 months = 173.3 hours/month
/// let monthly = calculate_monthly_hours(8, 5);
/// assert!((monthly - 173.33).abs() < 0.1);
/// ```
pub fn calculate_monthly_hours(hours_per_day: u32, days_per_week: u32) -> f64 {
    let weekly_hours = hours_per_day as f64 * days_per_week as f64;
    weekly_hours * 52.0 / 12.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_minimum_wage_tokyo() {
        let wage = get_minimum_wage(Prefecture::Tokyo, chrono::Utc::now().date_naive());
        assert_eq!(wage, 1_113);
    }

    #[test]
    fn test_get_minimum_wage_osaka() {
        let wage = get_minimum_wage(Prefecture::Osaka, chrono::Utc::now().date_naive());
        assert_eq!(wage, 1_064);
    }

    #[test]
    fn test_get_minimum_wage_okinawa() {
        let wage = get_minimum_wage(Prefecture::Okinawa, chrono::Utc::now().date_naive());
        assert_eq!(wage, 896);
    }

    #[test]
    fn test_validate_salary_above_minimum() {
        // ¥400,000/month, 173.3 hours/month = ¥2,308/hour
        let result = validate_salary_against_minimum(400_000, 173.3, Prefecture::Tokyo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_salary_below_minimum() {
        // ¥150,000/month, 173.3 hours/month = ¥865/hour (below Tokyo minimum ¥1,113)
        let result = validate_salary_against_minimum(150_000, 173.3, Prefecture::Tokyo);
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::BelowMinimumWage {
                actual_hourly,
                required_minimum,
                prefecture,
            } => {
                assert_eq!(actual_hourly, 865);
                assert_eq!(required_minimum, 1_113);
                assert_eq!(prefecture, Prefecture::Tokyo);
            }
            _ => panic!("Expected BelowMinimumWage error"),
        }
    }

    #[test]
    fn test_validate_salary_regional_differences() {
        let salary = 170_000;
        let hours = 173.3; // ¥981/hour

        // Should pass in lower-wage prefectures
        assert!(validate_salary_against_minimum(salary, hours, Prefecture::Okinawa).is_ok());
        assert!(validate_salary_against_minimum(salary, hours, Prefecture::Aomori).is_ok());

        // Should fail in higher-wage prefectures
        assert!(validate_salary_against_minimum(salary, hours, Prefecture::Tokyo).is_err());
        assert!(validate_salary_against_minimum(salary, hours, Prefecture::Osaka).is_err());
    }

    #[test]
    fn test_calculate_monthly_hours_standard() {
        // Standard 8h/day, 5 days/week
        let monthly = calculate_monthly_hours(8, 5);
        assert!((monthly - 173.33).abs() < 0.1);
    }

    #[test]
    fn test_calculate_monthly_hours_part_time() {
        // Part-time 4h/day, 5 days/week
        let monthly = calculate_monthly_hours(4, 5);
        assert!((monthly - 86.67).abs() < 0.1);
    }

    #[test]
    fn test_all_prefectures_have_minimum_wage() {
        let prefectures = vec![
            Prefecture::Hokkaido,
            Prefecture::Aomori,
            Prefecture::Iwate,
            Prefecture::Miyagi,
            Prefecture::Akita,
            Prefecture::Yamagata,
            Prefecture::Fukushima,
            Prefecture::Ibaraki,
            Prefecture::Tochigi,
            Prefecture::Gunma,
            Prefecture::Saitama,
            Prefecture::Chiba,
            Prefecture::Tokyo,
            Prefecture::Kanagawa,
            Prefecture::Niigata,
            Prefecture::Toyama,
            Prefecture::Ishikawa,
            Prefecture::Fukui,
            Prefecture::Yamanashi,
            Prefecture::Nagano,
            Prefecture::Gifu,
            Prefecture::Shizuoka,
            Prefecture::Aichi,
            Prefecture::Mie,
            Prefecture::Shiga,
            Prefecture::Kyoto,
            Prefecture::Osaka,
            Prefecture::Hyogo,
            Prefecture::Nara,
            Prefecture::Wakayama,
            Prefecture::Tottori,
            Prefecture::Shimane,
            Prefecture::Okayama,
            Prefecture::Hiroshima,
            Prefecture::Yamaguchi,
            Prefecture::Tokushima,
            Prefecture::Kagawa,
            Prefecture::Ehime,
            Prefecture::Kochi,
            Prefecture::Fukuoka,
            Prefecture::Saga,
            Prefecture::Nagasaki,
            Prefecture::Kumamoto,
            Prefecture::Oita,
            Prefecture::Miyazaki,
            Prefecture::Kagoshima,
            Prefecture::Okinawa,
        ];

        for prefecture in prefectures {
            let wage = get_minimum_wage(prefecture, chrono::Utc::now().date_naive());
            assert!(
                (893..=1_113).contains(&wage),
                "Prefecture {:?} has invalid wage: {}",
                prefecture,
                wage
            );
        }
    }

    #[test]
    fn test_validate_with_zero_hours() {
        let result = validate_salary_against_minimum(400_000, 0.0, Prefecture::Tokyo);
        assert!(result.is_err());

        match result.unwrap_err() {
            LaborLawError::InvalidCalculation { reason } => {
                assert!(reason.contains("positive"));
            }
            _ => panic!("Expected InvalidCalculation error"),
        }
    }
}
