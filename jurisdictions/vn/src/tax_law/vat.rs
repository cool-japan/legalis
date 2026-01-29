//! Value Added Tax (VAT) - Thuế Giá trị Gia tăng (GTGT)
//!
//! Law on Value Added Tax No. 13/2008/QH12 (amended by Law 31/2013, 106/2016).
//!
//! ## VAT Rates
//!
//! - **0%**: Exports, international transport
//! - **5%**: Essential goods (clean water, education materials, medical equipment)
//! - **10%**: Standard rate (most goods and services)
//!
//! ## VAT Methods
//!
//! - **Credit method** (Khấu trừ thuế): Input VAT - Output VAT
//! - **Direct method** (Tính trực tiếp): % of revenue

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// VAT rates in Vietnam (Thuế suất GTGT) - Article 8-10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRate {
    /// 0% - Exports and international transport (Điều 9)
    Zero,
    /// 5% - Essential goods and services (Điều 10)
    FivePercent,
    /// 10% - Standard rate (Điều 8)
    TenPercent,
}

impl VatRate {
    /// Get rate as decimal (0.0, 0.05, 0.10)
    pub fn as_decimal(&self) -> f64 {
        match self {
            Self::Zero => 0.0,
            Self::FivePercent => 0.05,
            Self::TenPercent => 0.10,
        }
    }

    /// Get rate as percentage (0, 5, 10)
    pub fn as_percentage(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::FivePercent => 5,
            Self::TenPercent => 10,
        }
    }

    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::Zero => "Thuế suất 0% - Hàng xuất khẩu, vận tải quốc tế",
            Self::FivePercent => "Thuế suất 5% - Hàng hóa, dịch vụ thiết yếu",
            Self::TenPercent => "Thuế suất 10% - Thuế suất cơ bản",
        }
    }

    /// Get English description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Zero => "0% rate - Exports and international transport",
            Self::FivePercent => "5% rate - Essential goods and services",
            Self::TenPercent => "10% rate - Standard rate",
        }
    }
}

/// VAT calculation method (Phương pháp tính thuế) - Article 12
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatMethod {
    /// Credit method (Phương pháp khấu trừ thuế) - Article 12.1
    Credit,
    /// Direct method (Phương pháp tính trực tiếp trên GTGT) - Article 12.2
    Direct {
        /// VAT rate as percentage of revenue (usually 1-5%)
        rate_percent: u8,
    },
}

impl VatMethod {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Credit => "Phương pháp khấu trừ thuế".to_string(),
            Self::Direct { rate_percent } => {
                format!("Phương pháp trực tiếp ({}% doanh thu)", rate_percent)
            }
        }
    }

    /// Get English name
    pub fn name_en(&self) -> String {
        match self {
            Self::Credit => "Credit method".to_string(),
            Self::Direct { rate_percent } => {
                format!("Direct method ({}% of revenue)", rate_percent)
            }
        }
    }
}

/// VAT-exempt goods and services (Hàng hóa, dịch vụ không chịu thuế GTGT) - Article 5
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatExemptCategory {
    /// Agricultural products (Sản phẩm nông nghiệp chưa chế biến)
    UnprocessedAgricultural,
    /// Health care services (Dịch vụ y tế)
    Healthcare,
    /// Education services (Dịch vụ giáo dục)
    Education,
    /// Financial services (Dịch vụ tài chính)
    Financial,
    /// Land use rights transfer (Chuyển nhượng quyền sử dụng đất)
    LandUseRights,
    /// Insurance services (Dịch vụ bảo hiểm)
    Insurance,
    /// Other
    Other(String),
}

impl VatExemptCategory {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::UnprocessedAgricultural => {
                "Sản phẩm trồng trọt, chăn nuôi, thủy sản chưa chế biến".to_string()
            }
            Self::Healthcare => "Dịch vụ khám bệnh, chữa bệnh".to_string(),
            Self::Education => "Dịch vụ giáo dục, dạy nghề".to_string(),
            Self::Financial => "Dịch vụ tài chính, ngân hàng".to_string(),
            Self::LandUseRights => "Chuyển nhượng quyền sử dụng đất".to_string(),
            Self::Insurance => "Dịch vụ bảo hiểm".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }
}

/// VAT declaration (Khai thuế GTGT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatDeclaration {
    /// Tax period (month or quarter)
    pub period: String,
    /// Output VAT (Thuế GTGT đầu ra)
    pub output_vat: i64,
    /// Input VAT (Thuế GTGT đầu vào)
    pub input_vat: i64,
    /// VAT method used
    pub method: VatMethod,
}

impl VatDeclaration {
    /// Calculate VAT payable (Thuế GTGT phải nộp)
    /// Returns positive if tax owed, negative if refund
    pub fn calculate_payable(&self) -> i64 {
        match self.method {
            VatMethod::Credit => self.output_vat - self.input_vat,
            VatMethod::Direct { .. } => self.output_vat,
        }
    }

    /// Check if entitled to refund (Được hoàn thuế)
    pub fn is_refundable(&self) -> bool {
        self.calculate_payable() < 0
    }
}

/// Calculate VAT amount
pub fn calculate_vat(amount: i64, rate: VatRate) -> i64 {
    (amount as f64 * rate.as_decimal()) as i64
}

/// Calculate price including VAT (Giá bao gồm thuế)
pub fn calculate_price_including_vat(base_price: i64, rate: VatRate) -> i64 {
    base_price + calculate_vat(base_price, rate)
}

/// Calculate base price from VAT-inclusive price (Giá chưa thuế)
pub fn calculate_base_price_from_inclusive(inclusive_price: i64, rate: VatRate) -> i64 {
    (inclusive_price as f64 / (1.0 + rate.as_decimal())) as i64
}

/// Result type for VAT operations
pub type VatResult<T> = Result<T, VatError>;

/// Errors related to VAT
#[derive(Debug, Error)]
pub enum VatError {
    /// Invalid VAT rate
    #[error("Thuế suất GTGT không hợp lệ: {rate}%")]
    InvalidRate { rate: u8 },

    /// Invalid VAT amount
    #[error("Số tiền thuế GTGT không hợp lệ: {amount} VND")]
    InvalidAmount { amount: i64 },

    /// Ineligible for VAT refund
    #[error("Không đủ điều kiện hoàn thuế GTGT: {reason}")]
    IneligibleForRefund { reason: String },

    /// Invalid declaration
    #[error("Khai báo thuế GTGT không hợp lệ: {reason}")]
    InvalidDeclaration { reason: String },
}

/// Validate VAT declaration
pub fn validate_vat_declaration(declaration: &VatDeclaration) -> VatResult<()> {
    if declaration.output_vat < 0 {
        return Err(VatError::InvalidAmount {
            amount: declaration.output_vat,
        });
    }

    if declaration.input_vat < 0 {
        return Err(VatError::InvalidAmount {
            amount: declaration.input_vat,
        });
    }

    if matches!(declaration.method, VatMethod::Direct { rate_percent } if rate_percent > 5) {
        return Err(VatError::InvalidDeclaration {
            reason: "Tỷ lệ tính trực tiếp không được vượt quá 5%".to_string(),
        });
    }

    Ok(())
}

/// Get VAT checklist
pub fn get_vat_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Đăng ký nộp thuế GTGT", "VAT registration", "Điều 4"),
        (
            "Lựa chọn phương pháp tính thuế",
            "Choose VAT calculation method",
            "Điều 12",
        ),
        (
            "Khai thuế GTGT theo tháng/quý",
            "Monthly/Quarterly VAT declaration",
            "Điều 13",
        ),
        ("Xuất hóa đơn GTGT", "Issue VAT invoice", "Điều 6"),
        ("Khấu trừ thuế GTGT đầu vào", "Deduct input VAT", "Điều 14"),
        ("Hoàn thuế GTGT", "VAT refund", "Điều 18"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_rates() {
        assert_eq!(VatRate::Zero.as_percentage(), 0);
        assert_eq!(VatRate::FivePercent.as_percentage(), 5);
        assert_eq!(VatRate::TenPercent.as_percentage(), 10);

        assert_eq!(VatRate::Zero.as_decimal(), 0.0);
        assert_eq!(VatRate::FivePercent.as_decimal(), 0.05);
        assert_eq!(VatRate::TenPercent.as_decimal(), 0.10);
    }

    #[test]
    fn test_calculate_vat() {
        let base_price = 1_000_000;

        let vat_10 = calculate_vat(base_price, VatRate::TenPercent);
        assert_eq!(vat_10, 100_000);

        let vat_5 = calculate_vat(base_price, VatRate::FivePercent);
        assert_eq!(vat_5, 50_000);

        let vat_0 = calculate_vat(base_price, VatRate::Zero);
        assert_eq!(vat_0, 0);
    }

    #[test]
    fn test_price_including_vat() {
        let base_price = 1_000_000;

        let inclusive_10 = calculate_price_including_vat(base_price, VatRate::TenPercent);
        assert_eq!(inclusive_10, 1_100_000);

        let inclusive_5 = calculate_price_including_vat(base_price, VatRate::FivePercent);
        assert_eq!(inclusive_5, 1_050_000);
    }

    #[test]
    fn test_base_price_from_inclusive() {
        let inclusive_price = 1_100_000;

        let base = calculate_base_price_from_inclusive(inclusive_price, VatRate::TenPercent);
        assert_eq!(base, 999_999); // Due to integer rounding: 1100000 / 1.1 = 999999.xx
    }

    #[test]
    fn test_vat_declaration() {
        let declaration = VatDeclaration {
            period: "2024-01".to_string(),
            output_vat: 100_000_000,
            input_vat: 60_000_000,
            method: VatMethod::Credit,
        };

        assert_eq!(declaration.calculate_payable(), 40_000_000);
        assert!(!declaration.is_refundable());

        let refund_declaration = VatDeclaration {
            period: "2024-02".to_string(),
            output_vat: 50_000_000,
            input_vat: 80_000_000,
            method: VatMethod::Credit,
        };

        assert!(refund_declaration.is_refundable());
        assert_eq!(refund_declaration.calculate_payable(), -30_000_000);
    }

    #[test]
    fn test_vat_declaration_validation() {
        let valid = VatDeclaration {
            period: "2024-01".to_string(),
            output_vat: 100_000_000,
            input_vat: 60_000_000,
            method: VatMethod::Credit,
        };
        assert!(validate_vat_declaration(&valid).is_ok());

        let invalid = VatDeclaration {
            period: "2024-01".to_string(),
            output_vat: -100_000_000,
            input_vat: 60_000_000,
            method: VatMethod::Credit,
        };
        assert!(validate_vat_declaration(&invalid).is_err());
    }

    #[test]
    fn test_vat_method() {
        let credit = VatMethod::Credit;
        assert!(credit.name_vi().contains("khấu trừ"));

        let direct = VatMethod::Direct { rate_percent: 2 };
        assert!(direct.name_vi().contains("2%"));
    }

    #[test]
    fn test_vat_checklist() {
        let checklist = get_vat_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 6);
    }
}
