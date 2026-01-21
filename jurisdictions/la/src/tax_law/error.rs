//! Tax Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍພາສີ)
//!
//! Comprehensive error types for Lao tax law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.

use thiserror::Error;

/// Result type for tax law operations
pub type Result<T> = std::result::Result<T, TaxLawError>;

/// Tax law errors (ຄວາມຜິດພາດກົດໝາຍພາສີ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum TaxLawError {
    // ========================================================================
    // Personal Income Tax Errors (ຄວາມຜິດພາດພາສີລາຍໄດ້ບຸກຄົນ)
    // ========================================================================
    /// Invalid tax bracket calculation
    /// ການຄຳນວນອັດຕາພາສີບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid tax bracket calculation for income {income_lak} LAK\nການຄຳນວນອັດຕາພາສີບໍ່ຖືກຕ້ອງສຳລັບລາຍໄດ້ {income_lak} ກີບ"
    )]
    InvalidTaxBracket { income_lak: u64 },

    /// Taxable income calculation error
    /// ການຄຳນວນລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີບໍ່ຖືກຕ້ອງ
    #[error(
        "Taxable income {taxable_lak} LAK does not match gross {gross_lak} LAK minus deductions {deductions_lak} LAK\nລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີ {taxable_lak} ກີບບໍ່ຖືກຕ້ອງ (ລາຍໄດ້ລວມ {gross_lak} ກີບ - ຫັກ {deductions_lak} ກີບ)"
    )]
    InvalidTaxableIncome {
        taxable_lak: u64,
        gross_lak: u64,
        deductions_lak: u64,
    },

    /// Tax calculation mismatch
    /// ຈຳນວນພາສີຄຳນວນບໍ່ຖືກຕ້ອງ
    #[error(
        "Tax calculation mismatch: calculated {calculated_lak} LAK, expected {expected_lak} LAK\nຈຳນວນພາສີຄຳນວນບໍ່ຖືກຕ້ອງ: ຄຳນວນໄດ້ {calculated_lak} ກີບ, ຄວນເປັນ {expected_lak} ກີບ"
    )]
    TaxCalculationMismatch {
        calculated_lak: u64,
        expected_lak: u64,
    },

    /// Below income tax threshold
    /// ຕ່ຳກວ່າເກນພາສີລາຍໄດ້
    #[error(
        "Income {income_lak} LAK is below tax threshold {threshold_lak} LAK - no tax required\nລາຍໄດ້ {income_lak} ກີບຕ່ຳກວ່າເກນພາສີ {threshold_lak} ກີບ - ບໍ່ຕ້ອງເສຍພາສີ"
    )]
    BelowTaxThreshold { income_lak: u64, threshold_lak: u64 },

    // ========================================================================
    // Corporate Income Tax Errors (ຄວາມຜິດພາດພາສີລາຍໄດ້ນິຕິບຸກຄົນ)
    // ========================================================================
    /// Invalid corporate tax rate
    /// ອັດຕາພາສີນິຕິບຸກຄົນບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid corporate tax rate {rate}% - should be {correct_rate}% (Tax Law 2011)\nອັດຕາພາສີບໍ່ຖືກຕ້ອງ {rate}% - ຄວນເປັນ {correct_rate}% (ກົດໝາຍພາສີ ປີ 2011)"
    )]
    InvalidCorporateTaxRate { rate: f64, correct_rate: f64 },

    /// Corporate taxable income negative
    /// ລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີເປັນລົບ
    #[error(
        "Corporate taxable income cannot be negative: {taxable_income_lak} LAK\nລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີບໍ່ສາມາດເປັນລົບໄດ້: {taxable_income_lak} ກີບ"
    )]
    NegativeTaxableIncome { taxable_income_lak: i64 },

    /// Revenue less than expenses
    /// ລາຍຮັບຕ່ຳກວ່າລາຍຈ່າຍ
    #[error(
        "Revenue {revenue_lak} LAK is less than total expenses {expenses_lak} LAK - possible loss\nລາຍຮັບ {revenue_lak} ກີບຕ່ຳກວ່າລາຍຈ່າຍ {expenses_lak} ກີບ - ເປັນຂາດທຶນ"
    )]
    RevenueLessThanExpenses { revenue_lak: u64, expenses_lak: u64 },

    // ========================================================================
    // VAT Errors (ຄວາມຜິດພາດພາສີມູນຄ່າເພີ່ມ)
    // ========================================================================
    /// VAT registration required
    /// ຕ້ອງຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
    #[error(
        "VAT registration required: annual turnover {turnover_lak} LAK exceeds threshold {threshold_lak} LAK\nຕ້ອງຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ: ລາຍຮັບປະຈຳປີ {turnover_lak} ກີບເກີນເກນ {threshold_lak} ກີບ"
    )]
    VATRegistrationRequired {
        turnover_lak: u64,
        threshold_lak: u64,
    },

    /// VAT calculation error
    /// ການຄຳນວນພາສີມູນຄ່າເພີ່ມບໍ່ຖືກຕ້ອງ
    #[error(
        "VAT calculation error: output VAT {output_lak} LAK minus input VAT {input_lak} LAK should equal {expected_lak} LAK, got {actual_lak} LAK\nການຄຳນວນພາສີບໍ່ຖືກຕ້ອງ: ພາສີຂາຍອອກ {output_lak} ກີບ - ພາສີຊື້ເຂົ້າ {input_lak} ກີບ = {expected_lak} ກີບ, ແຕ່ໄດ້ {actual_lak} ກີບ"
    )]
    VATCalculationError {
        output_lak: u64,
        input_lak: u64,
        expected_lak: i64,
        actual_lak: i64,
    },

    /// Invalid VAT rate
    /// ອັດຕາພາສີມູນຄ່າເພີ່ມບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid VAT rate {rate}% - should be {correct_rate}% for standard rate\nອັດຕາພາສີບໍ່ຖືກຕ້ອງ {rate}% - ຄວນເປັນ {correct_rate}% ສຳລັບອັດຕາມາດຕະຖານ"
    )]
    InvalidVATRate { rate: f64, correct_rate: f64 },

    /// Not registered for VAT
    /// ບໍ່ໄດ້ຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
    #[error(
        "Business is not registered for VAT but annual turnover {turnover_lak} LAK exceeds threshold\nບໍ່ໄດ້ຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ ແຕ່ລາຍຮັບປະຈຳປີ {turnover_lak} ກີບເກີນເກນ"
    )]
    NotRegisteredForVAT { turnover_lak: u64 },

    // ========================================================================
    // Property Tax Errors (ຄວາມຜິດພາດພາສີຊັບສິນ)
    // ========================================================================
    /// Invalid property tax rate
    /// ອັດຕາພາສີຊັບສິນບໍ່ຖືກຕ້ອງ
    #[error(
        "Property tax rate {rate}% is outside valid range {min_rate}% - {max_rate}%\nອັດຕາພາສີຊັບສິນ {rate}% ເກີນຂອບເຂດທີ່ຖືກຕ້ອງ {min_rate}% - {max_rate}%"
    )]
    InvalidPropertyTaxRate {
        rate: f64,
        min_rate: f64,
        max_rate: f64,
    },

    /// Property assessment error
    /// ການປະເມີນຊັບສິນບໍ່ຖືກຕ້ອງ
    #[error(
        "Property assessment error: assessed value {assessed_lak} LAK is unreasonable\nການປະເມີນຊັບສິນບໍ່ຖືກຕ້ອງ: ມູນຄ່າທີ່ປະເມີນ {assessed_lak} ກີບບໍ່ສົມເຫດສົມຜົນ"
    )]
    PropertyAssessmentError { assessed_lak: u64 },

    /// Property tax calculation error
    /// ການຄຳນວນພາສີຊັບສິນບໍ່ຖືກຕ້ອງ
    #[error(
        "Property tax calculation error: {assessed_value_lak} LAK × {rate}% should be {expected_lak} LAK, got {actual_lak} LAK\nການຄຳນວນພາສີຊັບສິນບໍ່ຖືກຕ້ອງ: {assessed_value_lak} ກີບ × {rate}% = {expected_lak} ກີບ, ແຕ່ໄດ້ {actual_lak} ກີບ"
    )]
    PropertyTaxCalculationError {
        assessed_value_lak: u64,
        rate: f64,
        expected_lak: u64,
        actual_lak: u64,
    },

    // ========================================================================
    // Customs Duty Errors (ຄວາມຜິດພາດພາສີສຸນລະກາກອນ)
    // ========================================================================
    /// Invalid customs duty rate
    /// ອັດຕາພາສີສຸນລະກາກອນບໍ່ຖືກຕ້ອງ
    #[error(
        "Customs duty rate {rate}% is outside valid range {min_rate}% - {max_rate}%\nອັດຕາພາສີສຸນລະກາກອນ {rate}% ເກີນຂອບເຂດທີ່ຖືກຕ້ອງ {min_rate}% - {max_rate}%"
    )]
    InvalidCustomsDutyRate {
        rate: f64,
        min_rate: f64,
        max_rate: f64,
    },

    /// Invalid HS code
    /// ລະຫັດ HS ບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid HS code: {hs_code} (must be 6-10 digits)\nລະຫັດ HS ບໍ່ຖືກຕ້ອງ: {hs_code} (ຕ້ອງເປັນ 6-10 ຕົວເລກ)"
    )]
    InvalidHSCode { hs_code: String },

    /// CIF value calculation error
    /// ການຄຳນວນມູນຄ່າ CIF ບໍ່ຖືກຕ້ອງ
    #[error(
        "CIF value calculation error for customs declaration {declaration_number}\nການຄຳນວນມູນຄ່າ CIF ບໍ່ຖືກຕ້ອງສຳລັບໃບແຈ້ງ {declaration_number}"
    )]
    CIFValueError { declaration_number: String },

    /// Customs duty calculation error
    /// ການຄຳນວນພາສີສຸນລະກາກອນບໍ່ຖືກຕ້ອງ
    #[error(
        "Customs duty calculation error: {cif_value_lak} LAK × {rate}% should be {expected_lak} LAK, got {actual_lak} LAK\nການຄຳນວນພາສີສຸນລະກາກອນບໍ່ຖືກຕ້ອງ: {cif_value_lak} ກີບ × {rate}% = {expected_lak} ກີບ, ແຕ່ໄດ້ {actual_lak} ກີບ"
    )]
    CustomsDutyCalculationError {
        cif_value_lak: u64,
        rate: f64,
        expected_lak: u64,
        actual_lak: u64,
    },

    // ========================================================================
    // Tax Residence Errors (ຄວາມຜິດພາດສະຖານະພັກເຊົາທາງພາສີ)
    // ========================================================================
    /// Tax residency unclear
    /// ສະຖານະພັກເຊົາທາງພາສີບໍ່ຊັດເຈນ
    #[error(
        "Tax residency status unclear: {days_in_lao} days in Lao PDR (183+ days = resident)\nສະຖານະພັກເຊົາທາງພາສີບໍ່ຊັດເຈນ: ພັກເຊົາ {days_in_lao} ມື້ໃນລາວ (183+ ມື້ = ຜູ້ມີຖິ່ນພັກເຊົາ)"
    )]
    TaxResidencyUnclear { days_in_lao: u32 },

    /// Missing tax ID
    /// ຂາດເລກປະຈຳຕົວຜ้ູເສຍພາສີ
    #[error(
        "Missing tax identification number (required for all taxpayers)\nຂາດເລກປະຈຳຕົວຜູ້ເສຍພາສີ (ບັງຄັບສຳລັບຜູ້ເສຍພາສີທຸກຄົນ)"
    )]
    MissingTaxID,

    /// Invalid tax ID format
    /// ຮູບແບບເລກປະຈຳຕົວຜູ້ເສຍພາສີບໍ່ຖືກຕ້ອງ
    #[error("Invalid tax ID format: {tax_id}\nຮູບແບບເລກປະຈຳຕົວຜູ້ເສຍພາສີບໍ່ຖືກຕ້ອງ: {tax_id}")]
    InvalidTaxIDFormat { tax_id: String },

    // ========================================================================
    // Filing and Payment Errors (ຄວາມຜິດພາດການຍື່ນແບບແລະການຈ່າຍພາສີ)
    // ========================================================================
    /// Late filing
    /// ຍື່ນແບບຊ້າ
    #[error(
        "Tax return filed late: due date {due_date}, filed on {filing_date} ({days_late} days late)\nຍື່ນແບບພາສີຊ້າ: ກຳນົດ {due_date}, ຍື່ນວັນທີ {filing_date} (ຊ້າ {days_late} ມື້)"
    )]
    LateFiling {
        due_date: String,
        filing_date: String,
        days_late: i64,
    },

    /// Late payment
    /// ຈ່າຍພາສີຊ້າ
    #[error(
        "Tax payment late: due date {due_date}, paid on {payment_date} ({days_late} days late)\nຈ່າຍພາສີຊ້າ: ກຳນົດ {due_date}, ຈ່າຍວັນທີ {payment_date} (ຊ້າ {days_late} ມື້)"
    )]
    LatePayment {
        due_date: String,
        payment_date: String,
        days_late: i64,
    },

    /// Underpayment
    /// ຈ່າຍພາສີບໍ່ພຽງພໍ
    #[error(
        "Tax underpayment: paid {paid_lak} LAK, should pay {due_lak} LAK (short {shortage_lak} LAK)\nຈ່າຍພາສີບໍ່ພຽງພໍ: ຈ່າຍ {paid_lak} ກີບ, ຄວນຈ່າຍ {due_lak} ກີບ (ຂາດ {shortage_lak} ກີບ)"
    )]
    Underpayment {
        paid_lak: u64,
        due_lak: u64,
        shortage_lak: u64,
    },

    /// Not filed
    /// ຍັງບໍ່ທັນຍື່ນແບບ
    #[error("Tax return not filed for tax year {tax_year}\nຍັງບໍ່ທັນຍື່ນແບບພາສີສຳລັບປີພາສີ {tax_year}")]
    NotFiled { tax_year: u32 },

    // ========================================================================
    // Excise Tax Errors (ຄວາມຜິດພາດພາສີສິນຄ້າພິເສດ)
    // ========================================================================
    /// Invalid excise tax rate
    /// ອັດຕາພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid excise tax rate {rate}% for category {category}\nອັດຕາພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ {rate}% ສຳລັບປະເພດ {category}"
    )]
    InvalidExciseTaxRate { rate: f64, category: String },

    /// Excise tax calculation error
    /// ການຄຳນວນພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ
    #[error(
        "Excise tax calculation error: {value_lak} LAK × {rate}% should be {expected_lak} LAK, got {actual_lak} LAK\nການຄຳນວນພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ: {value_lak} ກີບ × {rate}% = {expected_lak} ກີບ, ແຕ່ໄດ້ {actual_lak} ກີບ"
    )]
    ExciseTaxCalculationError {
        value_lak: u64,
        rate: f64,
        expected_lak: u64,
        actual_lak: u64,
    },

    /// Invalid excise tax category
    /// ປະເພດພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ
    #[error("Invalid excise tax category: {category}\nປະເພດພາສີສິນຄ້າພິເສດບໍ່ຖືກຕ້ອງ: {category}")]
    InvalidExciseCategory { category: String },

    // ========================================================================
    // Withholding Tax Errors (ຄວາມຜິດພາດພາສີຫັກ ນ ທີ່ຈ່າຍ)
    // ========================================================================
    /// Invalid withholding tax rate
    /// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid withholding tax rate {rate}% for payment type {payment_type}\nອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍບໍ່ຖືກຕ້ອງ {rate}% ສຳລັບປະເພດການຈ່າຍ {payment_type}"
    )]
    InvalidWithholdingTaxRate { rate: f64, payment_type: String },

    /// Withholding tax calculation error
    /// ການຄຳນວນພາສີຫັກ ນ ທີ່ຈ່າຍບໍ່ຖືກຕ້ອງ
    #[error(
        "Withholding tax calculation error: gross {gross_lak} LAK × {rate}% should be {expected_lak} LAK, got {actual_lak} LAK\nການຄຳນວນພາສີຫັກ ນ ທີ່ຈ່າຍບໍ່ຖືກຕ້ອງ: ລວມ {gross_lak} ກີບ × {rate}% = {expected_lak} ກີບ, ແຕ່ໄດ້ {actual_lak} ກີບ"
    )]
    WithholdingTaxCalculationError {
        gross_lak: u64,
        rate: f64,
        expected_lak: u64,
        actual_lak: u64,
    },

    /// Withholding tax not remitted
    /// ພາສີຫັກ ນ ທີ່ຈ່າຍບໍ່ໄດ້ສົ່ງໃຫ້ລັດຖະບານ
    #[error(
        "Withholding tax {amount_lak} LAK not remitted to government within deadline\nພາສີຫັກ ນ ທີ່ຈ່າຍ {amount_lak} ກີບບໍ່ໄດ້ສົ່ງໃຫ້ລັດຖະບານພາຍໃນກຳນົດ"
    )]
    WithholdingTaxNotRemitted { amount_lak: u64 },

    // ========================================================================
    // VAT Exemption Errors (ຄວາມຜິດພາດການຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມ)
    // ========================================================================
    /// Invalid VAT exemption claim
    /// ການຮຽກຮ້ອງຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid VAT exemption claim: {reason} (Article {article})\nການຮຽກຮ້ອງຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມບໍ່ຖືກຕ້ອງ: {reason} (ມາດຕາ {article})"
    )]
    InvalidVATExemption { reason: String, article: u16 },

    /// VAT exemption documentation missing
    /// ເອກະສານຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມຂາດ
    #[error(
        "VAT exemption documentation missing for category {category}\nເອກະສານຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມຂາດສຳລັບປະເພດ {category}"
    )]
    VATExemptionDocumentationMissing { category: String },

    // ========================================================================
    // General Errors (ຄວາມຜິດພາດທົ່ວໄປ)
    // ========================================================================
    /// Missing required field
    /// ຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ
    #[error("Missing required field: {field_name}\nຂາດຊ່ອງຂໍ້ມູນທີ່ຈຳເປັນ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date
    /// ວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date: {date_description}\nວັນທີບໍ່ຖືກຕ້ອງ: {date_description}")]
    InvalidDate { date_description: String },

    /// Invalid period
    /// ໄລຍະເວລາບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid filing period: month {month}, year {year}\nໄລຍະເວລາຍື່ນແບບບໍ່ຖືກຕ້ອງ: ເດືອນ {month}, ປີ {year}"
    )]
    InvalidPeriod { month: u32, year: u32 },

    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// Tax law violation
    /// ການລະເມີດກົດໝາຍພາສີ
    #[error(
        "Tax law violation: {violation} (Tax Law {law_reference})\nການລະເມີດກົດໝາຍພາສີ: {violation} ({law_reference})"
    )]
    TaxLawViolation {
        violation: String,
        law_reference: String,
    },

    /// Tax treaty violation
    /// ການລະເມີດສົນທິສັນຍາພາສີ
    #[error(
        "Tax treaty violation: {violation} (Treaty with {country}, Article {article})\nການລະເມີດສົນທິສັນຍາພາສີ: {violation} (ສົນທິສັນຍາກັບ {country}, ມາດຕາ {article})"
    )]
    TaxTreatyViolation {
        violation: String,
        country: String,
        article: u32,
    },

    /// Duplicate tax filing
    /// ການຍື່ນແບບພາສີຊ້ຳກັນ
    #[error(
        "Duplicate tax filing for period {period} of tax year {tax_year}\nການຍື່ນແບບພາສີຊ້ຳກັນສຳລັບໄລຍະ {period} ຂອງປີພາສີ {tax_year}"
    )]
    DuplicateFiling { period: String, tax_year: u32 },

    /// Tax audit pending
    /// ກຳລັງຖືກກວດສອບພາສີ
    #[error(
        "Tax audit pending for tax year {tax_year} - cannot amend return\nກຳລັງຖືກກວດສອບພາສີສຳລັບປີ {tax_year} - ບໍ່ສາມາດແກ້ໄຂແບບໄດ້"
    )]
    TaxAuditPending { tax_year: u32 },
}

impl TaxLawError {
    /// Get the error message in Lao language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the Lao part after the newline
        if let Some((_english, lao)) = full_msg.split_once('\n') {
            lao.to_string()
        } else {
            full_msg
        }
    }

    /// Get the error message in English language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the English part before the newline
        if let Some((english, _lao)) = full_msg.split_once('\n') {
            english.to_string()
        } else {
            full_msg
        }
    }

    /// Check if this is a critical tax violation requiring immediate action
    /// ກວດສອບວ່າເປັນການລະເມີດພາສີຮ້າຍແຮງທີ່ຕ້ອງແກ້ໄຂທັນທີ
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            TaxLawError::VATRegistrationRequired { .. }
                | TaxLawError::NotRegisteredForVAT { .. }
                | TaxLawError::NotFiled { .. }
                | TaxLawError::TaxLawViolation { .. }
        )
    }

    /// Check if this error indicates a penalty may apply
    /// ກວດສອບວ່າອາດມີການປັບໄໝ
    pub fn has_penalty(&self) -> bool {
        matches!(
            self,
            TaxLawError::LateFiling { .. }
                | TaxLawError::LatePayment { .. }
                | TaxLawError::Underpayment { .. }
                | TaxLawError::NotFiled { .. }
                | TaxLawError::TaxLawViolation { .. }
        )
    }

    /// Get the law reference for this error, if applicable
    /// ຮັບການອ້າງອິງກົດໝາຍສຳລັບຄວາມຜິດພາດນີ້
    pub fn law_reference(&self) -> Option<String> {
        match self {
            TaxLawError::InvalidCorporateTaxRate { .. } => {
                Some("Tax Law 2011, Article 16".to_string())
            }
            TaxLawError::VATRegistrationRequired { .. } => Some("VAT Law, Article 8".to_string()),
            TaxLawError::InvalidVATRate { .. } => Some("VAT Law, Article 12".to_string()),
            TaxLawError::InvalidVATExemption { article, .. } => {
                Some(format!("VAT Law, Article {}", article))
            }
            TaxLawError::InvalidPropertyTaxRate { .. } => {
                Some("Tax Law 2011, Article 44".to_string())
            }
            TaxLawError::InvalidCustomsDutyRate { .. } => {
                Some("Customs Law, Chapter 4".to_string())
            }
            TaxLawError::InvalidHSCode { .. } => Some("Customs Law, Article 15".to_string()),
            TaxLawError::InvalidExciseTaxRate { .. } => {
                Some("Excise Tax Decree, Article 5".to_string())
            }
            TaxLawError::InvalidWithholdingTaxRate { .. } => {
                Some("Tax Law 2011, Article 47".to_string())
            }
            TaxLawError::TaxLawViolation { law_reference, .. } => Some(law_reference.clone()),
            TaxLawError::TaxTreatyViolation {
                country, article, ..
            } => Some(format!("Tax Treaty with {}, Article {}", country, article)),
            _ => None,
        }
    }

    /// Check if this error requires immediate correction
    /// ກວດສອບວ່າຄວາມຜິດພາດນີ້ຕ້ອງການແກ້ໄຂທັນທີຫຼືບໍ່
    pub fn requires_immediate_correction(&self) -> bool {
        matches!(
            self,
            TaxLawError::NotRegisteredForVAT { .. }
                | TaxLawError::TaxLawViolation { .. }
                | TaxLawError::TaxTreatyViolation { .. }
                | TaxLawError::WithholdingTaxNotRemitted { .. }
                | TaxLawError::TaxAuditPending { .. }
        )
    }

    /// Get estimated penalty rate for this error type
    /// ຮັບອັດຕາຄ່າປັບໂດຍປະມານສຳລັບປະເພດຄວາມຜິດພາດນີ້
    pub fn estimated_penalty_rate(&self) -> Option<f64> {
        match self {
            TaxLawError::LateFiling { days_late, .. } => {
                // 0.1% per day, max 30%
                let rate = (*days_late as f64 * 0.001).min(0.30);
                Some(rate)
            }
            TaxLawError::LatePayment { days_late, .. } => {
                // 0.1% per day, max 30%
                let rate = (*days_late as f64 * 0.001).min(0.30);
                Some(rate)
            }
            TaxLawError::Underpayment { .. } => {
                // 20% penalty on shortage
                Some(0.20)
            }
            TaxLawError::NotFiled { .. } => {
                // 50% penalty for non-filing
                Some(0.50)
            }
            TaxLawError::WithholdingTaxNotRemitted { .. } => {
                // 30% penalty for non-remittance
                Some(0.30)
            }
            _ => None,
        }
    }

    /// Get the tax type related to this error
    /// ຮັບປະເພດພາສີທີ່ກ່ຽວຂ້ອງກັບຄວາມຜິດພາດນີ້
    pub fn related_tax_type(&self) -> Option<&'static str> {
        match self {
            TaxLawError::InvalidTaxBracket { .. }
            | TaxLawError::InvalidTaxableIncome { .. }
            | TaxLawError::BelowTaxThreshold { .. } => Some("Personal Income Tax"),

            TaxLawError::InvalidCorporateTaxRate { .. }
            | TaxLawError::NegativeTaxableIncome { .. }
            | TaxLawError::RevenueLessThanExpenses { .. } => Some("Corporate Income Tax"),

            TaxLawError::VATRegistrationRequired { .. }
            | TaxLawError::VATCalculationError { .. }
            | TaxLawError::InvalidVATRate { .. }
            | TaxLawError::NotRegisteredForVAT { .. }
            | TaxLawError::InvalidVATExemption { .. }
            | TaxLawError::VATExemptionDocumentationMissing { .. } => Some("Value Added Tax"),

            TaxLawError::InvalidPropertyTaxRate { .. }
            | TaxLawError::PropertyAssessmentError { .. }
            | TaxLawError::PropertyTaxCalculationError { .. } => Some("Property Tax"),

            TaxLawError::InvalidCustomsDutyRate { .. }
            | TaxLawError::InvalidHSCode { .. }
            | TaxLawError::CIFValueError { .. }
            | TaxLawError::CustomsDutyCalculationError { .. } => Some("Customs Duty"),

            TaxLawError::InvalidExciseTaxRate { .. }
            | TaxLawError::ExciseTaxCalculationError { .. }
            | TaxLawError::InvalidExciseCategory { .. } => Some("Excise Tax"),

            TaxLawError::InvalidWithholdingTaxRate { .. }
            | TaxLawError::WithholdingTaxCalculationError { .. }
            | TaxLawError::WithholdingTaxNotRemitted { .. } => Some("Withholding Tax"),

            _ => None,
        }
    }

    /// Create a new validation error with bilingual message
    pub fn validation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::ValidationError {
            message: format!("{} / {}", en.into(), lo.into()),
        }
    }

    /// Create a new tax law violation error
    pub fn violation(
        violation_en: impl Into<String>,
        violation_lo: impl Into<String>,
        law_reference: impl Into<String>,
    ) -> Self {
        Self::TaxLawViolation {
            violation: format!("{} / {}", violation_en.into(), violation_lo.into()),
            law_reference: law_reference.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = TaxLawError::InvalidTaxBracket {
            income_lak: 10_000_000,
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Invalid tax bracket"));
        assert!(lao.contains("ການຄຳນວນອັດຕາພາສີບໍ່ຖືກຕ້ອງ"));
    }

    #[test]
    fn test_critical_violations() {
        let vat_error = TaxLawError::VATRegistrationRequired {
            turnover_lak: 500_000_000,
            threshold_lak: 400_000_000,
        };
        assert!(vat_error.is_critical());

        let tax_calc_error = TaxLawError::TaxCalculationMismatch {
            calculated_lak: 100_000,
            expected_lak: 120_000,
        };
        assert!(!tax_calc_error.is_critical());
    }

    #[test]
    fn test_penalty_errors() {
        let late_filing = TaxLawError::LateFiling {
            due_date: "2024-03-31".to_string(),
            filing_date: "2024-04-15".to_string(),
            days_late: 15,
        };
        assert!(late_filing.has_penalty());

        let invalid_bracket = TaxLawError::InvalidTaxBracket {
            income_lak: 10_000_000,
        };
        assert!(!invalid_bracket.has_penalty());
    }

    #[test]
    fn test_law_references() {
        let corporate_error = TaxLawError::InvalidCorporateTaxRate {
            rate: 20.0,
            correct_rate: 24.0,
        };
        assert!(corporate_error.law_reference().is_some());

        let vat_error = TaxLawError::VATRegistrationRequired {
            turnover_lak: 500_000_000,
            threshold_lak: 400_000_000,
        };
        assert!(vat_error.law_reference().is_some());
    }

    #[test]
    fn test_requires_immediate_correction() {
        let not_registered = TaxLawError::NotRegisteredForVAT {
            turnover_lak: 500_000_000,
        };
        assert!(not_registered.requires_immediate_correction());

        let withholding_not_remitted = TaxLawError::WithholdingTaxNotRemitted {
            amount_lak: 1_000_000,
        };
        assert!(withholding_not_remitted.requires_immediate_correction());

        let invalid_rate = TaxLawError::InvalidVATRate {
            rate: 15.0,
            correct_rate: 10.0,
        };
        assert!(!invalid_rate.requires_immediate_correction());
    }

    #[test]
    fn test_estimated_penalty_rate() {
        let late_15_days = TaxLawError::LateFiling {
            due_date: "2024-03-31".to_string(),
            filing_date: "2024-04-15".to_string(),
            days_late: 15,
        };
        let rate = late_15_days.estimated_penalty_rate();
        assert!(rate.is_some());
        assert_eq!(rate.expect("Rate should be present"), 0.015); // 1.5%

        let underpayment = TaxLawError::Underpayment {
            paid_lak: 500_000,
            due_lak: 1_000_000,
            shortage_lak: 500_000,
        };
        let rate = underpayment.estimated_penalty_rate();
        assert!(rate.is_some());
        assert_eq!(rate.expect("Rate should be present"), 0.20); // 20%

        let not_filed = TaxLawError::NotFiled { tax_year: 2023 };
        let rate = not_filed.estimated_penalty_rate();
        assert!(rate.is_some());
        assert_eq!(rate.expect("Rate should be present"), 0.50); // 50%
    }

    #[test]
    fn test_related_tax_type() {
        let pit_error = TaxLawError::InvalidTaxBracket {
            income_lak: 10_000_000,
        };
        assert_eq!(pit_error.related_tax_type(), Some("Personal Income Tax"));

        let cit_error = TaxLawError::InvalidCorporateTaxRate {
            rate: 20.0,
            correct_rate: 24.0,
        };
        assert_eq!(cit_error.related_tax_type(), Some("Corporate Income Tax"));

        let vat_error = TaxLawError::InvalidVATRate {
            rate: 15.0,
            correct_rate: 10.0,
        };
        assert_eq!(vat_error.related_tax_type(), Some("Value Added Tax"));

        let property_error = TaxLawError::InvalidPropertyTaxRate {
            rate: 1.0,
            min_rate: 0.1,
            max_rate: 0.5,
        };
        assert_eq!(property_error.related_tax_type(), Some("Property Tax"));

        let customs_error = TaxLawError::InvalidHSCode {
            hs_code: "12345".to_string(),
        };
        assert_eq!(customs_error.related_tax_type(), Some("Customs Duty"));

        let excise_error = TaxLawError::InvalidExciseTaxRate {
            rate: 100.0,
            category: "Tobacco".to_string(),
        };
        assert_eq!(excise_error.related_tax_type(), Some("Excise Tax"));

        let withholding_error = TaxLawError::InvalidWithholdingTaxRate {
            rate: 50.0,
            payment_type: "Dividend".to_string(),
        };
        assert_eq!(
            withholding_error.related_tax_type(),
            Some("Withholding Tax")
        );
    }

    #[test]
    fn test_validation_helper() {
        let error = TaxLawError::validation("Invalid value", "ຄ່າບໍ່ຖືກຕ້ອງ");
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid value"));
        assert!(msg.contains("ຄ່າບໍ່ຖືກຕ້ອງ"));
    }

    #[test]
    fn test_violation_helper() {
        let error = TaxLawError::violation(
            "Tax evasion detected",
            "ກວດພົບການຫຼີກລ່ຽງພາສີ",
            "Tax Law 2011, Article 80",
        );
        let msg = format!("{}", error);
        assert!(msg.contains("Tax evasion detected"));
        assert!(msg.contains("Tax Law 2011"));
    }

    #[test]
    fn test_excise_tax_errors() {
        let rate_error = TaxLawError::InvalidExciseTaxRate {
            rate: 250.0,
            category: "Alcohol".to_string(),
        };
        let msg = format!("{}", rate_error);
        assert!(msg.contains("250%"));
        assert!(msg.contains("Alcohol"));

        let calc_error = TaxLawError::ExciseTaxCalculationError {
            value_lak: 1_000_000,
            rate: 30.0,
            expected_lak: 300_000,
            actual_lak: 350_000,
        };
        let msg = format!("{}", calc_error);
        assert!(msg.contains("1000000"));
        assert!(msg.contains("30%"));
    }

    #[test]
    fn test_withholding_tax_errors() {
        let rate_error = TaxLawError::InvalidWithholdingTaxRate {
            rate: 50.0,
            payment_type: "Interest".to_string(),
        };
        let msg = format!("{}", rate_error);
        assert!(msg.contains("50%"));
        assert!(msg.contains("Interest"));

        let not_remitted = TaxLawError::WithholdingTaxNotRemitted {
            amount_lak: 500_000,
        };
        let msg = format!("{}", not_remitted);
        assert!(msg.contains("500000"));
        assert!(msg.contains("not remitted"));
    }

    #[test]
    fn test_vat_exemption_errors() {
        let invalid_exemption = TaxLawError::InvalidVATExemption {
            reason: "Not a valid category".to_string(),
            article: 12,
        };
        let msg = format!("{}", invalid_exemption);
        assert!(msg.contains("Not a valid category"));
        assert!(msg.contains("Article 12"));

        let doc_missing = TaxLawError::VATExemptionDocumentationMissing {
            category: "Healthcare".to_string(),
        };
        let msg = format!("{}", doc_missing);
        assert!(msg.contains("Healthcare"));
        assert!(msg.contains("documentation"));
    }

    #[test]
    fn test_treaty_violation() {
        let treaty_error = TaxLawError::TaxTreatyViolation {
            violation: "Excessive withholding".to_string(),
            country: "Thailand".to_string(),
            article: 11,
        };
        let msg = format!("{}", treaty_error);
        assert!(msg.contains("Thailand"));
        assert!(msg.contains("Article 11"));

        let ref_text = treaty_error.law_reference();
        assert!(ref_text.is_some());
        assert!(
            ref_text
                .expect("Reference should be present")
                .contains("Thailand")
        );
    }

    #[test]
    fn test_duplicate_filing() {
        let dup_error = TaxLawError::DuplicateFiling {
            period: "January 2024".to_string(),
            tax_year: 2024,
        };
        let msg = format!("{}", dup_error);
        assert!(msg.contains("January 2024"));
        assert!(msg.contains("2024"));
    }

    #[test]
    fn test_tax_audit_pending() {
        let audit_error = TaxLawError::TaxAuditPending { tax_year: 2023 };
        assert!(audit_error.requires_immediate_correction());
        let msg = format!("{}", audit_error);
        assert!(msg.contains("2023"));
        assert!(msg.contains("audit"));
    }

    #[test]
    fn test_invalid_period() {
        let period_error = TaxLawError::InvalidPeriod {
            month: 13,
            year: 2024,
        };
        let msg = format!("{}", period_error);
        assert!(msg.contains("13"));
        assert!(msg.contains("2024"));
    }
}
