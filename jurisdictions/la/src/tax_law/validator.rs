//! Tax Law Validators (ການກວດສອບກົດໝາຍພາສີ)
//!
//! This module provides validation functions for tax compliance under:
//! - **Tax Law 2011** (Law No. 05/NA)
//! - **VAT Law**
//! - **Customs Law**
//!
//! All validators return bilingual error messages (Lao/English).

use super::error::{Result, TaxLawError};
use super::types::*;

// ============================================================================
// Tax Residence Validation (ການກວດສອບສະຖານະພັກເຊົາທາງພາສີ)
// ============================================================================

/// Minimum days required for tax residence in Lao PDR
/// ຈຳນວນມື້ຂັ້ນຕ່ຳສຳລັບການມີຖິ່ນພັກເຊົາທາງພາສີໃນລາວ
pub const TAX_RESIDENCE_DAYS_THRESHOLD: u32 = 183;

/// Validate tax residence status according to Tax Law 2011
/// ກວດສອບສະຖານະພັກເຊົາທາງພາສີຕາມກົດໝາຍພາສີ ປີ 2011
///
/// # Arguments
/// * `status` - Tax residence status to validate
///
/// # Returns
/// * `Ok(())` if residence status is valid
/// * `Err(TaxLawError)` if there are validation errors
///
/// # Examples
/// ```
/// use legalis_la::tax_law::{TaxResidenceStatus, validate_tax_residence};
///
/// let resident = TaxResidenceStatus::LaoResident {
///     lao_id: Some("123456789".to_string()),
///     days_in_lao: 200,
/// };
/// assert!(validate_tax_residence(&resident).is_ok());
/// ```
pub fn validate_tax_residence(status: &TaxResidenceStatus) -> Result<()> {
    match status {
        TaxResidenceStatus::LaoResident {
            days_in_lao,
            lao_id,
        } => {
            // Must stay at least 183 days to be a resident
            if *days_in_lao < TAX_RESIDENCE_DAYS_THRESHOLD {
                return Err(TaxLawError::TaxResidencyUnclear {
                    days_in_lao: *days_in_lao,
                });
            }

            // Validate Lao ID format if provided
            if let Some(id) = lao_id {
                validate_lao_id_format(id)?;
            }

            Ok(())
        }
        TaxResidenceStatus::NonResident {
            tax_residence_country,
            passport_number,
        } => {
            // Must have a valid tax residence country
            if tax_residence_country.is_empty() {
                return Err(TaxLawError::MissingRequiredField {
                    field_name: "tax_residence_country / ປະເທດທີ່ມີຖິ່ນພັກເຊົາທາງພາສີ".to_string(),
                });
            }

            // Validate passport format if provided
            if let Some(passport) = passport_number {
                validate_passport_format(passport)?;
            }

            Ok(())
        }
        TaxResidenceStatus::TreatyResident {
            treaty_country,
            treaty_article: _,
        } => {
            // Must have a valid treaty country
            if treaty_country.is_empty() {
                return Err(TaxLawError::MissingRequiredField {
                    field_name: "treaty_country / ປະເທດທີ່ມີສົນທິສັນຍາ".to_string(),
                });
            }

            // Verify treaty country is valid (list of countries with tax treaties)
            let treaty_countries = get_treaty_countries();
            if !treaty_countries.contains(&treaty_country.as_str()) {
                return Err(TaxLawError::ValidationError {
                    message: format!(
                        "No tax treaty with {} / ບໍ່ມີສົນທິສັນຍາພາສີກັບ {}",
                        treaty_country, treaty_country
                    ),
                });
            }

            Ok(())
        }
    }
}

/// Get list of countries with tax treaties with Lao PDR
/// ຮັບລາຍຊື່ປະເທດທີ່ມີສົນທິສັນຍາພາສີກັບລາວ
fn get_treaty_countries() -> Vec<&'static str> {
    vec![
        "China",
        "Thailand",
        "Vietnam",
        "South Korea",
        "Myanmar",
        "Malaysia",
        "Singapore",
        "Brunei",
        "Russia",
        "North Korea",
        "Luxembourg",
        "Indonesia",
        "Cambodia",
        "Philippines",
    ]
}

/// Validate Lao ID format
/// ກວດສອບຮູບແບບເລກບັດປະຈຳຕົວລາວ
fn validate_lao_id_format(id: &str) -> Result<()> {
    // Lao ID should be 9-13 characters
    if id.len() < 9 || id.len() > 13 {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: id.to_string(),
        });
    }

    // Should contain only digits
    if !id.chars().all(|c| c.is_ascii_digit()) {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: id.to_string(),
        });
    }

    Ok(())
}

/// Validate passport format
/// ກວດສອບຮູບແບບໜັງສືຜ່ານແດນ
fn validate_passport_format(passport: &str) -> Result<()> {
    // Passport should be 6-12 characters
    if passport.len() < 6 || passport.len() > 12 {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Invalid passport format: {} / ຮູບແບບໜັງສືຜ່ານແດນບໍ່ຖືກຕ້ອງ: {}",
                passport, passport
            ),
        });
    }

    // Should be alphanumeric
    if !passport.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Invalid passport format: {} / ຮູບແບບໜັງສືຜ່ານແດນບໍ່ຖືກຕ້ອງ: {}",
                passport, passport
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Personal Income Tax Validation (ການກວດສອບພາສີລາຍໄດ້ບຸກຄົນ)
// ============================================================================

/// Calculate personal income tax based on progressive brackets
/// ຄຳນວນພາສີລາຍໄດ້ບຸກຄົນຕາມອັດຕາກ້າວໜ້າ
///
/// Tax brackets (monthly income in LAK):
/// - 0 - 1,300,000: 0%
/// - 1,300,001 - 8,500,000: 5%
/// - 8,500,001 - 15,000,000: 10%
/// - 15,000,001 - 24,000,000: 15%
/// - 24,000,001 - 65,000,000: 20%
/// - Over 65,000,000: 25%
///
/// # Arguments
/// * `monthly_income_lak` - Monthly income in LAK
///
/// # Returns
/// * `Ok(u64)` - Calculated tax amount in LAK
/// * `Err(TaxLawError)` - If calculation fails
///
/// # Examples
/// ```
/// use legalis_la::tax_law::calculate_personal_income_tax;
///
/// // Income of 10 million LAK
/// let tax = calculate_personal_income_tax(10_000_000);
/// assert!(tax.is_ok());
/// ```
pub fn calculate_personal_income_tax(monthly_income_lak: u64) -> Result<u64> {
    // Below threshold - no tax
    if monthly_income_lak <= INCOME_TAX_THRESHOLD {
        return Ok(0);
    }

    let mut total_tax: u64 = 0;
    let brackets = &PERSONAL_INCOME_TAX_BRACKETS;

    // Calculate tax for each bracket
    for i in 0..brackets.len() {
        let (bracket_min, rate) = brackets[i];
        let bracket_max = if i + 1 < brackets.len() {
            brackets[i + 1].0
        } else {
            u64::MAX
        };

        if monthly_income_lak > bracket_min {
            let taxable_in_bracket = if monthly_income_lak >= bracket_max {
                bracket_max - bracket_min
            } else {
                monthly_income_lak - bracket_min
            };

            let tax_in_bracket = (taxable_in_bracket as f64 * rate) as u64;
            total_tax = total_tax.saturating_add(tax_in_bracket);
        }
    }

    Ok(total_tax)
}

/// Validate personal income tax return
/// ກວດສອບແບບພາສີລາຍໄດ້ບຸກຄົນ
///
/// # Arguments
/// * `tax_return` - Personal income tax return to validate
///
/// # Returns
/// * `Ok(())` if the return is valid
/// * `Err(TaxLawError)` if there are validation errors
pub fn validate_personal_income_tax(tax_return: &PersonalIncomeTax) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&tax_return.tax_id)?;

    // Validate tax residence
    validate_tax_residence(&tax_return.residence_status)?;

    // Validate taxable income calculation
    let expected_taxable = tax_return
        .gross_income_lak
        .saturating_sub(tax_return.deductible_expenses_lak)
        .saturating_sub(tax_return.personal_allowance_lak)
        .saturating_sub(tax_return.dependent_allowances_lak);

    if tax_return.taxable_income_lak != expected_taxable {
        return Err(TaxLawError::InvalidTaxableIncome {
            taxable_lak: tax_return.taxable_income_lak,
            gross_lak: tax_return.gross_income_lak,
            deductions_lak: tax_return.deductible_expenses_lak
                + tax_return.personal_allowance_lak
                + tax_return.dependent_allowances_lak,
        });
    }

    // Validate tax calculation
    let expected_tax = calculate_personal_income_tax(tax_return.taxable_income_lak)?;
    if tax_return.tax_calculated_lak != expected_tax {
        return Err(TaxLawError::TaxCalculationMismatch {
            calculated_lak: tax_return.tax_calculated_lak,
            expected_lak: expected_tax,
        });
    }

    // Validate income breakdown sums to gross income
    let total_income: u64 = tax_return
        .income_breakdown
        .iter()
        .map(|(_, amount)| amount)
        .sum();
    if total_income != tax_return.gross_income_lak {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Income breakdown {} LAK does not match gross income {} LAK / \
                ລາຍລະອຽດລາຍໄດ້ {} ກີບບໍ່ກົງກັບລາຍໄດ້ລວມ {} ກີບ",
                total_income,
                tax_return.gross_income_lak,
                total_income,
                tax_return.gross_income_lak
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Corporate Income Tax Validation (ການກວດສອບພາສີລາຍໄດ້ນິຕິບຸກຄົນ)
// ============================================================================

/// Calculate corporate income tax (24% flat rate)
/// ຄຳນວນພາສີລາຍໄດ້ນິຕິບຸກຄົນ (ອັດຕາ 24%)
///
/// # Arguments
/// * `taxable_income_lak` - Taxable income in LAK
///
/// # Returns
/// * `Ok(u64)` - Calculated tax amount in LAK
pub fn calculate_corporate_income_tax(taxable_income_lak: u64) -> Result<u64> {
    let tax = (taxable_income_lak as f64 * CORPORATE_INCOME_TAX_RATE) as u64;
    Ok(tax)
}

/// Validate corporate income tax return
/// ກວດສອບແບບພາສີລາຍໄດ້ນິຕິບຸກຄົນ
///
/// # Arguments
/// * `tax_return` - Corporate income tax return to validate
///
/// # Returns
/// * `Ok(())` if the return is valid
/// * `Err(TaxLawError)` if there are validation errors
pub fn validate_corporate_income_tax(tax_return: &CorporateIncomeTax) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&tax_return.tax_id)?;

    // Calculate total expenses
    let total_expenses = tax_return
        .cost_of_goods_sold_lak
        .saturating_add(tax_return.operating_expenses_lak)
        .saturating_add(tax_return.depreciation_lak)
        .saturating_add(tax_return.interest_expenses_lak)
        .saturating_add(tax_return.other_expenses_lak);

    // Check if revenue is less than expenses (loss situation)
    if tax_return.total_revenue_lak < total_expenses {
        return Err(TaxLawError::RevenueLessThanExpenses {
            revenue_lak: tax_return.total_revenue_lak,
            expenses_lak: total_expenses,
        });
    }

    // Validate taxable income calculation
    let expected_taxable = tax_return.total_revenue_lak.saturating_sub(total_expenses);
    if tax_return.taxable_income_lak != expected_taxable {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Taxable income {} LAK does not match revenue {} LAK minus expenses {} LAK / \
                ລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີ {} ກີບບໍ່ກົງກັບລາຍຮັບ {} ກີບ - ລາຍຈ່າຍ {} ກີບ",
                tax_return.taxable_income_lak,
                tax_return.total_revenue_lak,
                total_expenses,
                tax_return.taxable_income_lak,
                tax_return.total_revenue_lak,
                total_expenses
            ),
        });
    }

    // Validate tax calculation (24% rate)
    let expected_tax = calculate_corporate_income_tax(tax_return.taxable_income_lak)?;
    let tax_after_credits = expected_tax.saturating_sub(tax_return.tax_credits_lak);
    if tax_return.tax_calculated_lak != tax_after_credits {
        return Err(TaxLawError::TaxCalculationMismatch {
            calculated_lak: tax_return.tax_calculated_lak,
            expected_lak: tax_after_credits,
        });
    }

    Ok(())
}

// ============================================================================
// VAT Validation (ການກວດສອບພາສີມູນຄ່າເພີ່ມ)
// ============================================================================

/// Calculate VAT amount
/// ຄຳນວນພາສີມູນຄ່າເພີ່ມ
///
/// # Arguments
/// * `amount_lak` - Base amount in LAK (before VAT)
/// * `rate` - VAT rate (e.g., 0.10 for 10%)
///
/// # Returns
/// * `Ok(u64)` - VAT amount in LAK
/// * `Err(TaxLawError)` - If rate is invalid
pub fn calculate_vat(amount_lak: u64, rate: f64) -> Result<u64> {
    // Validate rate
    if !(0.0..=1.0).contains(&rate) {
        return Err(TaxLawError::InvalidVATRate {
            rate: rate * 100.0,
            correct_rate: VAT_STANDARD_RATE * 100.0,
        });
    }

    let vat = (amount_lak as f64 * rate) as u64;
    Ok(vat)
}

/// Validate VAT rate
/// ກວດສອບອັດຕາພາສີມູນຄ່າເພີ່ມ
///
/// # Arguments
/// * `rate` - VAT rate to validate (as decimal, e.g., 0.10 for 10%)
///
/// # Returns
/// * `Ok(())` if rate is valid
/// * `Err(TaxLawError)` if rate is invalid
pub fn validate_vat_rate(rate: f64) -> Result<()> {
    // Valid rates: 0% (zero-rated), 10% (standard)
    let valid_rates = [0.0, VAT_STANDARD_RATE];
    let tolerance = 0.001;

    let is_valid = valid_rates
        .iter()
        .any(|&valid| (rate - valid).abs() < tolerance);

    if !is_valid {
        return Err(TaxLawError::InvalidVATRate {
            rate: rate * 100.0,
            correct_rate: VAT_STANDARD_RATE * 100.0,
        });
    }

    Ok(())
}

/// Validate VAT registration requirement
/// ກວດສອບຄວາມຕ້ອງການຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
///
/// # Arguments
/// * `status` - Current VAT registration status
/// * `annual_turnover_lak` - Annual turnover in LAK
///
/// # Returns
/// * `Ok(())` if registration status is correct
/// * `Err(TaxLawError)` if registration is required but not done
pub fn validate_vat_registration(
    status: &VATRegistrationStatus,
    annual_turnover_lak: u64,
) -> Result<()> {
    match status {
        VATRegistrationStatus::Registered { vat_number, .. } => {
            // Validate VAT number format
            validate_vat_number_format(vat_number)?;
            Ok(())
        }
        VATRegistrationStatus::NotRegistered {
            annual_turnover_lak: declared_turnover,
        } => {
            // Check if registration is actually required
            if *declared_turnover >= VAT_REGISTRATION_THRESHOLD {
                return Err(TaxLawError::VATRegistrationRequired {
                    turnover_lak: *declared_turnover,
                    threshold_lak: VAT_REGISTRATION_THRESHOLD,
                });
            }
            if annual_turnover_lak >= VAT_REGISTRATION_THRESHOLD {
                return Err(TaxLawError::VATRegistrationRequired {
                    turnover_lak: annual_turnover_lak,
                    threshold_lak: VAT_REGISTRATION_THRESHOLD,
                });
            }
            Ok(())
        }
        VATRegistrationStatus::Exempt { exemption_reason } => {
            // Validate exemption reason is provided
            if exemption_reason.is_empty() {
                return Err(TaxLawError::MissingRequiredField {
                    field_name: "exemption_reason / ເຫດຜົນຍົກເວັ້ນ".to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Validate VAT number format
/// ກວດສອບຮູບແບບເລກທະບຽນພາສີມູນຄ່າເພີ່ມ
fn validate_vat_number_format(vat_number: &str) -> Result<()> {
    // VAT number should be 10-15 characters
    if vat_number.len() < 10 || vat_number.len() > 15 {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: vat_number.to_string(),
        });
    }

    // Should contain only digits and possibly dashes
    if !vat_number.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: vat_number.to_string(),
        });
    }

    Ok(())
}

/// Validate VAT calculation
/// ກວດສອບການຄຳນວນພາສີມູນຄ່າເພີ່ມ
///
/// # Arguments
/// * `vat_return` - VAT return to validate
///
/// # Returns
/// * `Ok(())` if calculations are correct
/// * `Err(TaxLawError)` if there are calculation errors
pub fn validate_vat_calculation(vat_return: &VATReturn) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&vat_return.tax_id)?;

    // Validate registration status
    let annual_turnover = vat_return.total_sales_lak.saturating_mul(12);
    validate_vat_registration(&vat_return.registration_status, annual_turnover)?;

    // Validate output VAT calculation (10% of sales)
    let expected_output_vat = calculate_vat(vat_return.total_sales_lak, VAT_STANDARD_RATE)?;
    let output_tolerance = expected_output_vat / 100; // 1% tolerance for rounding
    if (vat_return.output_vat_lak as i64 - expected_output_vat as i64).unsigned_abs()
        > output_tolerance
    {
        return Err(TaxLawError::VATCalculationError {
            output_lak: vat_return.output_vat_lak,
            input_lak: vat_return.input_vat_lak,
            expected_lak: (expected_output_vat as i64) - (vat_return.input_vat_lak as i64),
            actual_lak: vat_return.vat_payable_lak,
        });
    }

    // Validate VAT payable calculation (output - input)
    let expected_payable = vat_return.output_vat_lak as i64 - vat_return.input_vat_lak as i64;
    if vat_return.vat_payable_lak != expected_payable {
        return Err(TaxLawError::VATCalculationError {
            output_lak: vat_return.output_vat_lak,
            input_lak: vat_return.input_vat_lak,
            expected_lak: expected_payable,
            actual_lak: vat_return.vat_payable_lak,
        });
    }

    Ok(())
}

/// Validate VAT exemption
/// ກວດສອບການຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມ
///
/// # Arguments
/// * `category` - VAT exempt category claimed
/// * `description` - Description of goods/services
///
/// # Returns
/// * `Ok(())` if exemption is valid
/// * `Err(TaxLawError)` if exemption is not applicable
pub fn validate_vat_exemption(category: &VATExemptCategory, description: &str) -> Result<()> {
    if description.is_empty() {
        return Err(TaxLawError::MissingRequiredField {
            field_name: "description / ລາຍລະອຽດ".to_string(),
        });
    }

    // Validate category-specific requirements
    match category {
        VATExemptCategory::FinancialServices => {
            // Must be a licensed financial institution
            if !description.to_lowercase().contains("bank")
                && !description.to_lowercase().contains("insurance")
                && !description.to_lowercase().contains("finance")
            {
                return Err(TaxLawError::ValidationError {
                    message:
                        "Financial services exemption requires licensed financial institution / \
                              ການຍົກເວັ້ນການບໍລິການທາງການເງິນຕ້ອງເປັນສະຖາບັນການເງິນທີ່ໄດ້ຮັບໃບອະນຸຍາດ"
                            .to_string(),
                });
            }
        }
        VATExemptCategory::Education => {
            // Must be educational services
            if !description.to_lowercase().contains("school")
                && !description.to_lowercase().contains("education")
                && !description.to_lowercase().contains("training")
                && !description.to_lowercase().contains("university")
            {
                return Err(TaxLawError::ValidationError {
                    message: "Education exemption requires educational institution / \
                              ການຍົກເວັ້ນການສຶກສາຕ້ອງເປັນສະຖາບັນການສຶກສາ"
                        .to_string(),
                });
            }
        }
        VATExemptCategory::Healthcare => {
            // Must be healthcare services
            if !description.to_lowercase().contains("hospital")
                && !description.to_lowercase().contains("clinic")
                && !description.to_lowercase().contains("medical")
                && !description.to_lowercase().contains("health")
            {
                return Err(TaxLawError::ValidationError {
                    message: "Healthcare exemption requires medical facility / \
                              ການຍົກເວັ້ນການແພດຕ້ອງເປັນສະຖານທີ່ແພດ"
                        .to_string(),
                });
            }
        }
        VATExemptCategory::Agriculture => {
            // Must be agricultural products
            if !description.to_lowercase().contains("farm")
                && !description.to_lowercase().contains("agriculture")
                && !description.to_lowercase().contains("crop")
                && !description.to_lowercase().contains("livestock")
            {
                return Err(TaxLawError::ValidationError {
                    message: "Agriculture exemption requires agricultural products / \
                              ການຍົກເວັ້ນກະສິກຳຕ້ອງເປັນຜະລິດຕະພັນກະສິກຳ"
                        .to_string(),
                });
            }
        }
        VATExemptCategory::PublicServices => {
            // Must be public services
            if !description.to_lowercase().contains("public")
                && !description.to_lowercase().contains("government")
                && !description.to_lowercase().contains("state")
            {
                return Err(TaxLawError::ValidationError {
                    message: "Public services exemption requires government services / \
                              ການຍົກເວັ້ນການບໍລິການສາທາລະນະຕ້ອງເປັນການບໍລິການລັດຖະບານ"
                        .to_string(),
                });
            }
        }
        VATExemptCategory::Other {
            description: exempt_desc,
        } => {
            if exempt_desc.is_empty() {
                return Err(TaxLawError::MissingRequiredField {
                    field_name: "exemption description / ລາຍລະອຽດການຍົກເວັ້ນ".to_string(),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Property Tax Validation (ການກວດສອບພາສີຊັບສິນ)
// ============================================================================

/// Calculate property tax
/// ຄຳນວນພາສີຊັບສິນ
///
/// # Arguments
/// * `assessed_value_lak` - Assessed value in LAK
/// * `rate` - Property tax rate (e.g., 0.001 for 0.1%)
///
/// # Returns
/// * `Ok(u64)` - Property tax amount in LAK
pub fn calculate_property_tax(assessed_value_lak: u64, rate: f64) -> Result<u64> {
    // Validate rate
    validate_property_tax_rate(rate)?;

    let tax = (assessed_value_lak as f64 * rate) as u64;
    Ok(tax)
}

/// Validate property tax rate
/// ກວດສອບອັດຕາພາສີຊັບສິນ
///
/// # Arguments
/// * `rate` - Property tax rate to validate (as decimal)
///
/// # Returns
/// * `Ok(())` if rate is valid (0.1% - 0.5%)
/// * `Err(TaxLawError)` if rate is outside valid range
pub fn validate_property_tax_rate(rate: f64) -> Result<()> {
    if !(PROPERTY_TAX_RATE_MIN..=PROPERTY_TAX_RATE_MAX).contains(&rate) {
        return Err(TaxLawError::InvalidPropertyTaxRate {
            rate: rate * 100.0,
            min_rate: PROPERTY_TAX_RATE_MIN * 100.0,
            max_rate: PROPERTY_TAX_RATE_MAX * 100.0,
        });
    }
    Ok(())
}

/// Validate property tax assessment
/// ກວດສອບການປະເມີນພາສີຊັບສິນ
///
/// # Arguments
/// * `property_tax` - Property tax assessment to validate
///
/// # Returns
/// * `Ok(())` if assessment is valid
/// * `Err(TaxLawError)` if there are validation errors
pub fn validate_property_tax(property_tax: &PropertyTax) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&property_tax.tax_id)?;

    // Validate tax rate
    validate_property_tax_rate(property_tax.tax_rate)?;

    // Validate assessed value is reasonable (minimum 1 million LAK)
    if property_tax.assessed_value_lak < 1_000_000 {
        return Err(TaxLawError::PropertyAssessmentError {
            assessed_lak: property_tax.assessed_value_lak,
        });
    }

    // Validate tax calculation
    let expected_tax =
        calculate_property_tax(property_tax.assessed_value_lak, property_tax.tax_rate)?;
    if property_tax.tax_amount_lak != expected_tax {
        return Err(TaxLawError::PropertyTaxCalculationError {
            assessed_value_lak: property_tax.assessed_value_lak,
            rate: property_tax.tax_rate * 100.0,
            expected_lak: expected_tax,
            actual_lak: property_tax.tax_amount_lak,
        });
    }

    Ok(())
}

// ============================================================================
// Excise Tax Validation (ການກວດສອບພາສີສິນຄ້າພິເສດ)
// ============================================================================

/// Calculate excise tax
/// ຄຳນວນພາສີສິນຄ້າພິເສດ
///
/// # Arguments
/// * `value_lak` - Value of goods in LAK
/// * `rate` - Excise tax rate
///
/// # Returns
/// * `Ok(u64)` - Excise tax amount in LAK
pub fn calculate_excise_tax(value_lak: u64, rate: f64) -> Result<u64> {
    if !(0.0..=1.0).contains(&rate) {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Invalid excise tax rate: {:.2}% / ອັດຕາພາສີບໍ່ຖືກຕ້ອງ: {:.2}%",
                rate * 100.0,
                rate * 100.0
            ),
        });
    }

    let tax = (value_lak as f64 * rate) as u64;
    Ok(tax)
}

/// Validate excise tax declaration
/// ກວດສອບການແຈ້ງພາສີສິນຄ້າພິເສດ
///
/// # Arguments
/// * `excise_tax` - Excise tax declaration to validate
///
/// # Returns
/// * `Ok(())` if declaration is valid
/// * `Err(TaxLawError)` if there are validation errors
pub fn validate_excise_tax(excise_tax: &ExciseTax) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&excise_tax.tax_id)?;

    // Get rate from category
    let rate = match &excise_tax.category {
        ExciseTaxCategory::Tobacco { tax_rate, .. } => *tax_rate,
        ExciseTaxCategory::Alcohol { tax_rate, .. } => *tax_rate,
        ExciseTaxCategory::Fuel {
            tax_rate_per_liter, ..
        } => *tax_rate_per_liter,
        ExciseTaxCategory::Vehicles { tax_rate, .. } => *tax_rate,
        ExciseTaxCategory::LuxuryGoods { tax_rate, .. } => *tax_rate,
        ExciseTaxCategory::Other { tax_rate, .. } => *tax_rate,
    };

    // Validate rate is within reasonable bounds (up to 200% for some products)
    if !(0.0..=2.0).contains(&rate) {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Invalid excise tax rate: {:.2}% / ອັດຕາພາສີບໍ່ຖືກຕ້ອງ: {:.2}%",
                rate * 100.0,
                rate * 100.0
            ),
        });
    }

    // Validate total value calculation
    let expected_total = (excise_tax.quantity * excise_tax.unit_price_lak as f64) as u64;
    let tolerance = expected_total / 100; // 1% tolerance
    if (excise_tax.total_value_lak as i64 - expected_total as i64).unsigned_abs() > tolerance {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Total value {} LAK does not match quantity {} x price {} LAK / \
                ມູນຄ່າລວມ {} ກີບບໍ່ກົງກັບປະລິມານ {} x ລາຄາ {} ກີບ",
                excise_tax.total_value_lak,
                excise_tax.quantity,
                excise_tax.unit_price_lak,
                excise_tax.total_value_lak,
                excise_tax.quantity,
                excise_tax.unit_price_lak
            ),
        });
    }

    // Validate excise tax calculation
    let expected_tax = calculate_excise_tax(excise_tax.total_value_lak, rate)?;
    let tax_tolerance = expected_tax / 100; // 1% tolerance
    if (excise_tax.excise_tax_lak as i64 - expected_tax as i64).unsigned_abs() > tax_tolerance {
        return Err(TaxLawError::TaxCalculationMismatch {
            calculated_lak: excise_tax.excise_tax_lak,
            expected_lak: expected_tax,
        });
    }

    Ok(())
}

// ============================================================================
// Customs Duty Validation (ການກວດສອບພາສີສຸນລະກາກອນ)
// ============================================================================

/// Validate HS code format
/// ກວດສອບຮູບແບບລະຫັດ HS
///
/// # Arguments
/// * `hs_code` - HS code to validate (6-10 digits)
///
/// # Returns
/// * `Ok(())` if format is valid
/// * `Err(TaxLawError)` if format is invalid
pub fn validate_hs_code(hs_code: &str) -> Result<()> {
    // HS code should be 6-10 digits
    let digits_only: String = hs_code.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits_only.len() < 6 || digits_only.len() > 10 {
        return Err(TaxLawError::InvalidHSCode {
            hs_code: hs_code.to_string(),
        });
    }

    Ok(())
}

/// Validate customs duty rate
/// ກວດສອບອັດຕາພາສີສຸນລະກາກອນ
///
/// # Arguments
/// * `rate` - Customs duty rate to validate (as decimal, e.g., 0.10 for 10%)
///
/// # Returns
/// * `Ok(())` if rate is within valid range (0% - 40%)
/// * `Err(TaxLawError)` if rate is outside valid range
pub fn validate_customs_duty_rate(rate: f64) -> Result<()> {
    if !(CUSTOMS_DUTY_RATE_MIN..=CUSTOMS_DUTY_RATE_MAX).contains(&rate) {
        return Err(TaxLawError::InvalidCustomsDutyRate {
            rate: rate * 100.0,
            min_rate: CUSTOMS_DUTY_RATE_MIN * 100.0,
            max_rate: CUSTOMS_DUTY_RATE_MAX * 100.0,
        });
    }
    Ok(())
}

/// Validate customs duty declaration
/// ກວດສອບໃບແຈ້ງພາສີສຸນລະກາກອນ
///
/// # Arguments
/// * `customs_duty` - Customs duty declaration to validate
///
/// # Returns
/// * `Ok(())` if declaration is valid
/// * `Err(TaxLawError)` if there are validation errors
pub fn validate_customs_duty(customs_duty: &CustomsDuty) -> Result<()> {
    // Validate tax ID
    validate_tax_id_format(&customs_duty.tax_id)?;

    // Validate HS code
    validate_hs_code(&customs_duty.hs_code)?;

    // Validate duty rate
    validate_customs_duty_rate(customs_duty.duty_rate)?;

    // Validate CIF value is positive
    if customs_duty.cif_value_lak == 0 {
        return Err(TaxLawError::CIFValueError {
            declaration_number: customs_duty.declaration_number.clone(),
        });
    }

    // Validate customs duty calculation
    let expected_duty = (customs_duty.cif_value_lak as f64 * customs_duty.duty_rate) as u64;
    let tolerance = expected_duty / 100; // 1% tolerance
    if (customs_duty.duty_amount_lak as i64 - expected_duty as i64).unsigned_abs() > tolerance {
        return Err(TaxLawError::CustomsDutyCalculationError {
            cif_value_lak: customs_duty.cif_value_lak,
            rate: customs_duty.duty_rate * 100.0,
            expected_lak: expected_duty,
            actual_lak: customs_duty.duty_amount_lak,
        });
    }

    // Validate import VAT calculation (10% of CIF + duty)
    let vat_base = customs_duty.cif_value_lak + customs_duty.duty_amount_lak;
    let expected_vat = (vat_base as f64 * VAT_STANDARD_RATE) as u64;
    let vat_tolerance = expected_vat / 100; // 1% tolerance
    if (customs_duty.import_vat_lak as i64 - expected_vat as i64).unsigned_abs() > vat_tolerance {
        return Err(TaxLawError::VATCalculationError {
            output_lak: expected_vat,
            input_lak: 0,
            expected_lak: expected_vat as i64,
            actual_lak: customs_duty.import_vat_lak as i64,
        });
    }

    // Validate total tax calculation
    let expected_total = customs_duty.duty_amount_lak
        + customs_duty.import_vat_lak
        + customs_duty.import_excise_lak.unwrap_or(0);
    if customs_duty.total_tax_lak != expected_total {
        return Err(TaxLawError::ValidationError {
            message: format!(
                "Total tax {} LAK does not match sum of duty, VAT, and excise {} LAK / \
                ພາສີລວມ {} ກີບບໍ່ກົງກັບຜົນລວມຂອງພາສີ, VAT, ແລະພາສີສິນຄ້າພິເສດ {} ກີບ",
                customs_duty.total_tax_lak,
                expected_total,
                customs_duty.total_tax_lak,
                expected_total
            ),
        });
    }

    Ok(())
}

// ============================================================================
// Tax Filing Validation (ການກວດສອບການຍື່ນແບບພາສີ)
// ============================================================================

/// Validate tax ID format
/// ກວດສອບຮູບແບບເລກປະຈຳຕົວຜູ້ເສຍພາສີ
///
/// # Arguments
/// * `tax_id` - Tax identification number to validate
///
/// # Returns
/// * `Ok(())` if format is valid
/// * `Err(TaxLawError)` if format is invalid
pub fn validate_tax_id_format(tax_id: &str) -> Result<()> {
    if tax_id.is_empty() {
        return Err(TaxLawError::MissingTaxID);
    }

    // Tax ID should be 10-15 characters
    if tax_id.len() < 10 || tax_id.len() > 15 {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: tax_id.to_string(),
        });
    }

    // Should contain only digits and possibly dashes
    if !tax_id.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(TaxLawError::InvalidTaxIDFormat {
            tax_id: tax_id.to_string(),
        });
    }

    Ok(())
}

/// Validate tax filing status and deadlines
/// ກວດສອບສະຖານະແລະກຳນົດເວລາການຍື່ນແບບພາສີ
///
/// # Arguments
/// * `status` - Tax filing status
/// * `tax_year` - Tax year
///
/// # Returns
/// * `Ok(())` if filing is valid
/// * `Err(TaxLawError)` if there are issues
pub fn validate_tax_filing(status: &TaxFilingStatus, tax_year: u32) -> Result<()> {
    match status {
        TaxFilingStatus::NotFiled => Err(TaxLawError::NotFiled { tax_year }),
        TaxFilingStatus::FiledOnTime { filing_date: _ } => {
            // Filing is on time, no issues
            Ok(())
        }
        TaxFilingStatus::FiledLate {
            filing_date,
            days_late,
        } => {
            // Filing is late, return warning with penalty indication
            Err(TaxLawError::LateFiling {
                due_date: format!("{}-03-31", tax_year + 1),
                filing_date: format!("{}", filing_date.format("%Y-%m-%d")),
                days_late: *days_late,
            })
        }
        TaxFilingStatus::UnderReview => {
            // Under review is valid but return info
            Ok(())
        }
        TaxFilingStatus::Accepted { acceptance_date: _ } => {
            // Accepted is valid
            Ok(())
        }
        TaxFilingStatus::Rejected { reason } => {
            // Rejected filing
            Err(TaxLawError::ValidationError {
                message: format!(
                    "Tax return rejected: {} / ແບບພາສີຖືກປະຕິເສດ: {}",
                    reason, reason
                ),
            })
        }
    }
}

/// Validate tax payment
/// ກວດສອບການຈ່າຍພາສີ
///
/// # Arguments
/// * `amount_due_lak` - Amount due in LAK
/// * `amount_paid_lak` - Amount paid in LAK
///
/// # Returns
/// * `Ok(())` if payment is correct
/// * `Err(TaxLawError)` if there is underpayment
pub fn validate_tax_payment(amount_due_lak: u64, amount_paid_lak: u64) -> Result<()> {
    if amount_paid_lak < amount_due_lak {
        return Err(TaxLawError::Underpayment {
            paid_lak: amount_paid_lak,
            due_lak: amount_due_lak,
            shortage_lak: amount_due_lak - amount_paid_lak,
        });
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_tax_residence_lao_resident() {
        let resident = TaxResidenceStatus::LaoResident {
            lao_id: Some("123456789".to_string()),
            days_in_lao: 200,
        };
        assert!(validate_tax_residence(&resident).is_ok());
    }

    #[test]
    fn test_validate_tax_residence_insufficient_days() {
        let resident = TaxResidenceStatus::LaoResident {
            lao_id: Some("123456789".to_string()),
            days_in_lao: 100,
        };
        assert!(validate_tax_residence(&resident).is_err());
    }

    #[test]
    fn test_validate_tax_residence_non_resident() {
        let non_resident = TaxResidenceStatus::NonResident {
            passport_number: Some("AB1234567".to_string()),
            tax_residence_country: "Thailand".to_string(),
        };
        assert!(validate_tax_residence(&non_resident).is_ok());
    }

    #[test]
    fn test_validate_tax_residence_treaty_resident() {
        let treaty_resident = TaxResidenceStatus::TreatyResident {
            treaty_country: "China".to_string(),
            treaty_article: Some(4),
        };
        assert!(validate_tax_residence(&treaty_resident).is_ok());
    }

    #[test]
    fn test_validate_tax_residence_invalid_treaty_country() {
        let treaty_resident = TaxResidenceStatus::TreatyResident {
            treaty_country: "Mars".to_string(),
            treaty_article: None,
        };
        assert!(validate_tax_residence(&treaty_resident).is_err());
    }

    #[test]
    fn test_calculate_personal_income_tax_below_threshold() {
        let tax = calculate_personal_income_tax(1_000_000);
        assert!(tax.is_ok());
        assert_eq!(tax.expect("Tax calculation should succeed"), 0);
    }

    #[test]
    fn test_calculate_personal_income_tax_first_bracket() {
        let tax = calculate_personal_income_tax(5_000_000);
        assert!(tax.is_ok());
        // 5M - 1.3M = 3.7M taxable at 5% = 185,000
        assert_eq!(tax.expect("Tax calculation should succeed"), 185_000);
    }

    #[test]
    fn test_calculate_personal_income_tax_multiple_brackets() {
        let tax = calculate_personal_income_tax(20_000_000);
        assert!(tax.is_ok());
        // Complex calculation across multiple brackets
        let expected =
            calculate_personal_income_tax(20_000_000).expect("Tax calculation should succeed");
        assert!(expected > 0);
    }

    #[test]
    fn test_calculate_corporate_income_tax() {
        let tax = calculate_corporate_income_tax(100_000_000);
        assert!(tax.is_ok());
        // 100M x 24% = 24M
        assert_eq!(tax.expect("Tax calculation should succeed"), 24_000_000);
    }

    #[test]
    fn test_calculate_vat() {
        let vat = calculate_vat(10_000_000, 0.10);
        assert!(vat.is_ok());
        // 10M x 10% = 1M
        assert_eq!(vat.expect("VAT calculation should succeed"), 1_000_000);
    }

    #[test]
    fn test_validate_vat_rate_standard() {
        assert!(validate_vat_rate(0.10).is_ok());
    }

    #[test]
    fn test_validate_vat_rate_zero() {
        assert!(validate_vat_rate(0.0).is_ok());
    }

    #[test]
    fn test_validate_vat_rate_invalid() {
        assert!(validate_vat_rate(0.15).is_err());
    }

    #[test]
    fn test_validate_vat_registration_required() {
        let status = VATRegistrationStatus::NotRegistered {
            annual_turnover_lak: 500_000_000,
        };
        assert!(validate_vat_registration(&status, 500_000_000).is_err());
    }

    #[test]
    fn test_validate_vat_registration_not_required() {
        let status = VATRegistrationStatus::NotRegistered {
            annual_turnover_lak: 100_000_000,
        };
        assert!(validate_vat_registration(&status, 100_000_000).is_ok());
    }

    #[test]
    fn test_validate_property_tax_rate_valid() {
        assert!(validate_property_tax_rate(0.002).is_ok());
    }

    #[test]
    fn test_validate_property_tax_rate_invalid() {
        assert!(validate_property_tax_rate(0.01).is_err());
    }

    #[test]
    fn test_calculate_property_tax() {
        let tax = calculate_property_tax(100_000_000, 0.002);
        assert!(tax.is_ok());
        // 100M x 0.2% = 200,000
        assert_eq!(
            tax.expect("Property tax calculation should succeed"),
            200_000
        );
    }

    #[test]
    fn test_validate_hs_code_valid() {
        assert!(validate_hs_code("8471300000").is_ok());
    }

    #[test]
    fn test_validate_hs_code_too_short() {
        assert!(validate_hs_code("12345").is_err());
    }

    #[test]
    fn test_validate_customs_duty_rate_valid() {
        assert!(validate_customs_duty_rate(0.10).is_ok());
    }

    #[test]
    fn test_validate_customs_duty_rate_invalid() {
        assert!(validate_customs_duty_rate(0.50).is_err());
    }

    #[test]
    fn test_validate_tax_id_format_valid() {
        assert!(validate_tax_id_format("1234567890").is_ok());
    }

    #[test]
    fn test_validate_tax_id_format_too_short() {
        assert!(validate_tax_id_format("123").is_err());
    }

    #[test]
    fn test_validate_tax_id_format_empty() {
        assert!(validate_tax_id_format("").is_err());
    }

    #[test]
    fn test_validate_tax_filing_not_filed() {
        assert!(validate_tax_filing(&TaxFilingStatus::NotFiled, 2023).is_err());
    }

    #[test]
    fn test_validate_tax_filing_on_time() {
        let status = TaxFilingStatus::FiledOnTime {
            filing_date: Utc::now(),
        };
        assert!(validate_tax_filing(&status, 2023).is_ok());
    }

    #[test]
    fn test_validate_tax_payment_sufficient() {
        assert!(validate_tax_payment(1_000_000, 1_000_000).is_ok());
    }

    #[test]
    fn test_validate_tax_payment_overpayment() {
        assert!(validate_tax_payment(1_000_000, 1_500_000).is_ok());
    }

    #[test]
    fn test_validate_tax_payment_underpayment() {
        assert!(validate_tax_payment(1_000_000, 500_000).is_err());
    }

    #[test]
    fn test_calculate_excise_tax() {
        let tax = calculate_excise_tax(10_000_000, 0.30);
        assert!(tax.is_ok());
        // 10M x 30% = 3M
        assert_eq!(
            tax.expect("Excise tax calculation should succeed"),
            3_000_000
        );
    }

    #[test]
    fn test_validate_vat_exemption_financial() {
        assert!(
            validate_vat_exemption(&VATExemptCategory::FinancialServices, "Bank of Lao").is_ok()
        );
    }

    #[test]
    fn test_validate_vat_exemption_healthcare() {
        assert!(
            validate_vat_exemption(&VATExemptCategory::Healthcare, "Hospital services").is_ok()
        );
    }

    #[test]
    fn test_validate_vat_exemption_education() {
        assert!(validate_vat_exemption(&VATExemptCategory::Education, "University fees").is_ok());
    }

    #[test]
    fn test_validate_vat_exemption_agriculture() {
        assert!(validate_vat_exemption(&VATExemptCategory::Agriculture, "Farm products").is_ok());
    }

    #[test]
    fn test_treaty_countries_list() {
        let countries = get_treaty_countries();
        assert!(countries.contains(&"China"));
        assert!(countries.contains(&"Thailand"));
        assert!(countries.contains(&"Vietnam"));
    }
}
