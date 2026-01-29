//! Saudi Tax System (الأنظمة الضريبية)
//!
//! Saudi Arabia's tax system includes:
//! - VAT (ضريبة القيمة المضافة) - 15%
//! - Zakat (الزكاة) - 2.5% for Saudi/GCC nationals
//! - Corporate Income Tax (ضريبة الدخل) - 20% for foreign companies
//!
//! Administered by ZATCA (Zakat, Tax and Customs Authority - هيئة الزكاة والضريبة والجمارك)

pub mod corporate_income_tax;
pub mod vat;
pub mod zakat;

pub use corporate_income_tax::{CorporateIncomeTax, calculate_corporate_tax};
pub use vat::{VatRate, VatRegistration, calculate_vat};
pub use zakat::{ZakatRate, calculate_zakat};

use thiserror::Error;

/// Result type for tax operations
pub type TaxResult<T> = Result<T, TaxError>;

/// Tax errors
#[derive(Debug, Error)]
pub enum TaxError {
    /// Invalid tax calculation
    #[error("حساب ضريبي غير صالح: {reason}")]
    InvalidCalculation { reason: String },

    /// Registration error
    #[error("خطأ في التسجيل الضريبي: {reason}")]
    RegistrationError { reason: String },

    /// Compliance violation
    #[error("انتهاك الامتثال الضريبي: {description}")]
    ComplianceViolation { description: String },
}

/// Get tax compliance checklist
pub fn get_tax_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "التسجيل في هيئة الزكاة والضريبة والجمارك",
            "Register with ZATCA",
        ),
        (
            "الحصول على الرقم الضريبي",
            "Obtain Tax Identification Number (TIN)",
        ),
        (
            "تسجيل ضريبة القيمة المضافة",
            "VAT registration (if applicable)",
        ),
        ("تسجيل الزكاة", "Zakat registration (if applicable)"),
        (
            "تسجيل ضريبة الدخل",
            "Income tax registration (if applicable)",
        ),
        ("الفوترة الإلكترونية", "E-invoicing compliance"),
        (
            "التقديم الشهري لضريبة القيمة المضافة",
            "Monthly VAT returns",
        ),
        ("التقديم السنوي للزكاة", "Annual Zakat declaration"),
        ("حفظ السجلات", "Record keeping (minimum 6 years)"),
        ("الفحص الضريبي", "Tax audit readiness"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_checklist() {
        let checklist = get_tax_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
