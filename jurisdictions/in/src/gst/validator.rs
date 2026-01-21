//! GST Validation Logic
//!
//! Validation functions for GST compliance

use super::error::{GstComplianceReport, GstError, GstResult, ReturnComplianceStatus};
use super::types::*;
use chrono::{Datelike, NaiveDate};

/// Validate GSTIN format
pub fn validate_gstin(gstin: &str) -> GstResult<Gstin> {
    // Length check
    if gstin.len() != 15 {
        return Err(GstError::InvalidGstin {
            gstin: gstin.to_string(),
        });
    }

    // Parse GSTIN
    let parsed = Gstin::parse(gstin).ok_or_else(|| GstError::InvalidGstin {
        gstin: gstin.to_string(),
    })?;

    // Validate format
    if !parsed.is_valid() {
        return Err(GstError::InvalidGstin {
            gstin: gstin.to_string(),
        });
    }

    Ok(parsed)
}

/// Validate registration requirement based on turnover
pub fn validate_registration_requirement(
    aggregate_turnover: f64,
    state: GstState,
    supply_type: SupplyCategory,
    makes_inter_state_supply: bool,
    is_e_commerce_operator: bool,
) -> GstResult<bool> {
    // E-commerce operators must register regardless of turnover
    if is_e_commerce_operator {
        return Ok(true);
    }

    // Inter-state supply requires registration regardless of threshold
    if makes_inter_state_supply {
        return Ok(true);
    }

    // Check threshold based on supply type and state
    let threshold = match supply_type {
        SupplyCategory::Goods => state.goods_threshold(),
        SupplyCategory::Services | SupplyCategory::Mixed | SupplyCategory::Composite => {
            state.services_threshold()
        }
    };

    Ok(aggregate_turnover > threshold as f64)
}

/// Validate composition scheme eligibility
pub fn validate_composition_eligibility(
    turnover: f64,
    business_type: CompositionBusinessType,
    makes_inter_state_supply: bool,
    is_e_commerce_operator: bool,
    _supplies_exempt_goods: bool,
    manufactures_notified_goods: bool,
) -> GstResult<CompositionScheme> {
    // Check turnover limit
    let turnover_limit = business_type.turnover_limit();
    if turnover > turnover_limit as f64 {
        return Err(GstError::CompositionIneligible {
            reason: format!(
                "Turnover Rs. {} exceeds limit of Rs. {} for {}",
                turnover,
                turnover_limit,
                match business_type {
                    CompositionBusinessType::Manufacturer => "manufacturers",
                    CompositionBusinessType::Trader => "traders",
                    CompositionBusinessType::Restaurant => "restaurants",
                    CompositionBusinessType::ServiceProvider => "service providers",
                }
            ),
        });
    }

    // Inter-state supply not allowed
    if makes_inter_state_supply {
        return Err(GstError::CompositionIneligible {
            reason: "Cannot make inter-state taxable supplies under composition scheme".to_string(),
        });
    }

    // E-commerce operators cannot opt for composition
    if is_e_commerce_operator {
        return Err(GstError::CompositionIneligible {
            reason: "E-commerce operators are not eligible for composition scheme".to_string(),
        });
    }

    // Cannot supply goods through e-commerce operator
    // This would need additional validation

    // Check for notified goods (like ice cream, pan masala)
    if manufactures_notified_goods {
        return Err(GstError::CompositionIneligible {
            reason: "Manufacturers of notified goods (ice cream, pan masala, tobacco) cannot opt"
                .to_string(),
        });
    }

    Ok(CompositionScheme {
        eligible: true,
        business_type,
        rate: business_type.composition_rate(),
        turnover_limit,
        previous_turnover: turnover as u64,
    })
}

/// Validate invoice
pub fn validate_invoice(invoice: &Invoice, registration: &GstRegistration) -> GstResult<()> {
    // Check supplier GSTIN matches registration
    if invoice.supplier_gstin != registration.gstin.full {
        return Err(GstError::InvalidInvoice {
            reason: "Supplier GSTIN does not match registration".to_string(),
        });
    }

    // Check invoice date
    let today = chrono::Local::now().date_naive();
    if invoice.date > today {
        return Err(GstError::InvalidInvoice {
            reason: "Invoice date cannot be in the future".to_string(),
        });
    }

    // Validate tax calculation
    let calculated_total =
        invoice.taxable_value + invoice.cgst + invoice.sgst + invoice.igst + invoice.cess;
    let diff = (calculated_total - invoice.total_value).abs();
    if diff > 1.0 {
        // Allow Rs. 1 rounding difference
        return Err(GstError::InvalidInvoice {
            reason: format!(
                "Tax calculation mismatch: calculated {} vs stated {}",
                calculated_total, invoice.total_value
            ),
        });
    }

    // Check supply type matches tax components
    match invoice.supply_type {
        SupplyType::IntraState => {
            if invoice.igst > 0.0 {
                return Err(GstError::InvalidInvoice {
                    reason: "Intra-state supply should not have IGST".to_string(),
                });
            }
        }
        SupplyType::InterState => {
            if invoice.cgst > 0.0 || invoice.sgst > 0.0 {
                return Err(GstError::InvalidInvoice {
                    reason: "Inter-state supply should only have IGST".to_string(),
                });
            }
        }
        SupplyType::Export | SupplyType::SezSupply => {
            // Zero-rated - may have tax if export with tax payment
        }
    }

    // Composition dealer cannot issue tax invoice
    if registration.composition_opted && !matches!(invoice.invoice_type, InvoiceType::BillOfSupply)
    {
        return Err(GstError::InvalidInvoice {
            reason: "Composition dealer must issue Bill of Supply, not Tax Invoice".to_string(),
        });
    }

    Ok(())
}

/// Validate E-way bill requirement
pub fn validate_eway_bill_requirement(
    taxable_value: f64,
    supply_type: SupplyType,
    from_state: GstState,
    to_state: GstState,
    goods_type: &str,
) -> GstResult<bool> {
    // Exempt goods list (simplified)
    let exempt_goods = [
        "fruits",
        "vegetables",
        "milk",
        "curd",
        "bread",
        "salt",
        "natural honey",
        "fresh meat",
        "fish",
        "live trees",
        "plants",
    ];

    if exempt_goods
        .iter()
        .any(|g| goods_type.to_lowercase().contains(g))
    {
        return Ok(false);
    }

    // Check value threshold
    let required = EwayBill::is_required(taxable_value, supply_type);

    // Inter-state supply of goods
    if from_state.code() != to_state.code() && taxable_value > 50_000.0 {
        return Ok(true);
    }

    Ok(required)
}

/// Validate ITC eligibility
pub fn validate_itc_eligibility(
    itc: &InputTaxCredit,
    invoice: &Invoice,
    recipient_registered: bool,
    goods_received: bool,
    tax_paid_by_supplier: bool,
    return_filed: bool,
    claimed_before_deadline: bool,
) -> GstResult<()> {
    // Section 16(2) conditions
    // (a) Possession of tax invoice/debit note
    // Assumed if invoice is provided

    // (b) Receipt of goods/services
    if !goods_received {
        return Err(GstError::ItcConditionsNotMet {
            condition: "Goods/services not received".to_string(),
        });
    }

    // (c) Tax actually paid to government
    if !tax_paid_by_supplier {
        return Err(GstError::ItcConditionsNotMet {
            condition: "Tax not paid by supplier to government (not reflected in GSTR-2B)"
                .to_string(),
        });
    }

    // (d) Return filed
    if !return_filed {
        return Err(GstError::ItcConditionsNotMet {
            condition: "Return not filed by recipient".to_string(),
        });
    }

    // Section 16(4) - Time limit
    if !claimed_before_deadline {
        return Err(GstError::ItcTimeBarred {
            invoice_date: invoice.date.to_string(),
        });
    }

    // Check recipient registration
    if !recipient_registered {
        return Err(GstError::ItcNotAvailable {
            reason: "Recipient not registered under GST".to_string(),
        });
    }

    // Check for blocked credit
    if let Some(ref reason) = itc.blocked_reason {
        return Err(GstError::ItcBlocked {
            item: reason.description().to_string(),
        });
    }

    Ok(())
}

/// Calculate late fee for return
pub fn calculate_late_fee(return_type: ReturnType, days_late: u32, is_nil_return: bool) -> f64 {
    if days_late == 0 {
        return 0.0;
    }

    match return_type {
        ReturnType::Gstr1 | ReturnType::Gstr3b => {
            if is_nil_return {
                // Rs. 20 per day (Rs. 10 CGST + Rs. 10 SGST) max Rs. 500
                (days_late * 20).min(500) as f64
            } else {
                // Rs. 50 per day (Rs. 25 CGST + Rs. 25 SGST) max Rs. 5000
                (days_late * 50).min(5000) as f64
            }
        }
        ReturnType::Gstr4 | ReturnType::Cmp08 => {
            // Rs. 50 per day max Rs. 2000
            (days_late * 50).min(2000) as f64
        }
        ReturnType::Gstr9 | ReturnType::Gstr9c => {
            // Rs. 200 per day (Rs. 100 CGST + Rs. 100 SGST) max 0.25% of turnover
            // Simplified: Rs. 200 per day max Rs. 50000
            (days_late * 200).min(50000) as f64
        }
        _ => (days_late * 50).min(5000) as f64,
    }
}

/// Calculate interest on delayed payment
pub fn calculate_interest(tax_amount: f64, days_delayed: u32, is_wrong_itc_claim: bool) -> f64 {
    let rate = if is_wrong_itc_claim { 24.0 } else { 18.0 };
    tax_amount * rate / 100.0 * days_delayed as f64 / 365.0
}

/// Validate return filing status
pub fn validate_return_filing(
    return_type: ReturnType,
    due_date: NaiveDate,
    filing_date: Option<NaiveDate>,
    tax_paid: f64,
    tax_liability: f64,
) -> GstResult<ReturnComplianceStatus> {
    let today = chrono::Local::now().date_naive();

    match filing_date {
        Some(filed) => {
            let days_late = if filed > due_date {
                (filed - due_date).num_days() as u32
            } else {
                0
            };

            let late_fee = calculate_late_fee(return_type, days_late, tax_liability == 0.0);

            if days_late > 0 {
                return Err(GstError::ReturnFiledLate {
                    return_type: format!("{:?}", return_type),
                    late_fee,
                });
            }

            // Check tax payment
            if tax_paid < tax_liability {
                return Err(GstError::TaxNotPaid {
                    period: format!("{:?}", return_type),
                    amount: tax_liability - tax_paid,
                });
            }

            Ok(ReturnComplianceStatus {
                return_type: format!("{:?}", return_type),
                period: due_date.format("%b-%Y").to_string(),
                filed_on_time: true,
                late_fee: 0.0,
            })
        }
        None => {
            if today > due_date {
                Err(GstError::ReturnNotFiled {
                    return_type: format!("{:?}", return_type),
                    period: due_date.format("%b-%Y").to_string(),
                })
            } else {
                Ok(ReturnComplianceStatus {
                    return_type: format!("{:?}", return_type),
                    period: due_date.format("%b-%Y").to_string(),
                    filed_on_time: true, // Not yet due
                    late_fee: 0.0,
                })
            }
        }
    }
}

/// Determine supply type based on place of supply
pub fn determine_supply_type(
    supplier_state: GstState,
    place_of_supply: GstState,
    is_export: bool,
    is_sez: bool,
) -> SupplyType {
    if is_export {
        return SupplyType::Export;
    }

    if is_sez {
        return SupplyType::SezSupply;
    }

    if supplier_state.code() == place_of_supply.code() {
        SupplyType::IntraState
    } else {
        SupplyType::InterState
    }
}

/// Calculate tax on supply
pub fn calculate_tax(
    taxable_value: f64,
    gst_rate: GstRate,
    supply_type: SupplyType,
    cess: Option<CompensationCess>,
) -> TaxLiability {
    let mut liability = TaxLiability {
        period: String::new(),
        cgst: 0.0,
        sgst: 0.0,
        igst: 0.0,
        cess: 0.0,
        interest: 0.0,
        late_fee: 0.0,
        penalty: 0.0,
    };

    match supply_type {
        SupplyType::IntraState => {
            liability.cgst = taxable_value * gst_rate.cgst() / 100.0;
            liability.sgst = taxable_value * gst_rate.sgst() / 100.0;
        }
        SupplyType::InterState => {
            liability.igst = taxable_value * gst_rate.igst() / 100.0;
        }
        SupplyType::Export | SupplyType::SezSupply => {
            // Zero-rated - no tax unless with payment option
        }
    }

    // Add compensation cess if applicable
    if let Some(cess_item) = cess
        && !matches!(cess_item, CompensationCess::None)
    {
        // Simplified cess calculation (would need actual rates)
        liability.cess = match cess_item {
            CompensationCess::AeratedBeverages => taxable_value * 0.12,
            CompensationCess::PanMasala => taxable_value * 0.60,
            _ => 0.0, // Would need specific rate
        };
    }

    liability
}

/// Validate ITC utilization order (Section 49)
pub fn validate_itc_utilization(
    igst_credit: f64,
    cgst_credit: f64,
    sgst_credit: f64,
    igst_liability: f64,
    cgst_liability: f64,
    sgst_liability: f64,
) -> GstResult<ItcUtilizationPlan> {
    let mut plan = ItcUtilizationPlan {
        steps: Vec::new(),
        remaining_igst_credit: igst_credit,
        remaining_cgst_credit: cgst_credit,
        remaining_sgst_credit: sgst_credit,
        remaining_igst_liability: igst_liability,
        remaining_cgst_liability: cgst_liability,
        remaining_sgst_liability: sgst_liability,
        cash_payment_required: 0.0,
    };

    // Step 1: IGST credit for IGST liability
    let igst_for_igst = plan
        .remaining_igst_credit
        .min(plan.remaining_igst_liability);
    if igst_for_igst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::IgstForIgst,
            amount: igst_for_igst,
        });
        plan.remaining_igst_credit -= igst_for_igst;
        plan.remaining_igst_liability -= igst_for_igst;
    }

    // Step 2: IGST credit for CGST liability
    let igst_for_cgst = plan
        .remaining_igst_credit
        .min(plan.remaining_cgst_liability);
    if igst_for_cgst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::IgstForCgst,
            amount: igst_for_cgst,
        });
        plan.remaining_igst_credit -= igst_for_cgst;
        plan.remaining_cgst_liability -= igst_for_cgst;
    }

    // Step 3: IGST credit for SGST liability
    let igst_for_sgst = plan
        .remaining_igst_credit
        .min(plan.remaining_sgst_liability);
    if igst_for_sgst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::IgstForSgst,
            amount: igst_for_sgst,
        });
        plan.remaining_igst_credit -= igst_for_sgst;
        plan.remaining_sgst_liability -= igst_for_sgst;
    }

    // Step 4: CGST credit for CGST liability
    let cgst_for_cgst = plan
        .remaining_cgst_credit
        .min(plan.remaining_cgst_liability);
    if cgst_for_cgst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::CgstForCgst,
            amount: cgst_for_cgst,
        });
        plan.remaining_cgst_credit -= cgst_for_cgst;
        plan.remaining_cgst_liability -= cgst_for_cgst;
    }

    // Step 5: CGST credit for IGST liability
    let cgst_for_igst = plan
        .remaining_cgst_credit
        .min(plan.remaining_igst_liability);
    if cgst_for_igst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::CgstForIgst,
            amount: cgst_for_igst,
        });
        plan.remaining_cgst_credit -= cgst_for_igst;
        plan.remaining_igst_liability -= cgst_for_igst;
    }

    // Step 6: SGST credit for SGST liability
    let sgst_for_sgst = plan
        .remaining_sgst_credit
        .min(plan.remaining_sgst_liability);
    if sgst_for_sgst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::SgstForSgst,
            amount: sgst_for_sgst,
        });
        plan.remaining_sgst_credit -= sgst_for_sgst;
        plan.remaining_sgst_liability -= sgst_for_sgst;
    }

    // Step 7: SGST credit for IGST liability
    let sgst_for_igst = plan
        .remaining_sgst_credit
        .min(plan.remaining_igst_liability);
    if sgst_for_igst > 0.0 {
        plan.steps.push(ItcUtilizationStep {
            utilization: ItcUtilization::SgstForIgst,
            amount: sgst_for_igst,
        });
        plan.remaining_sgst_credit -= sgst_for_igst;
        plan.remaining_igst_liability -= sgst_for_igst;
    }

    // Calculate cash payment required
    plan.cash_payment_required = plan.remaining_igst_liability
        + plan.remaining_cgst_liability
        + plan.remaining_sgst_liability;

    Ok(plan)
}

/// ITC utilization plan
#[derive(Debug, Clone)]
pub struct ItcUtilizationPlan {
    /// Utilization steps
    pub steps: Vec<ItcUtilizationStep>,
    /// Remaining IGST credit
    pub remaining_igst_credit: f64,
    /// Remaining CGST credit
    pub remaining_cgst_credit: f64,
    /// Remaining SGST credit
    pub remaining_sgst_credit: f64,
    /// Remaining IGST liability
    pub remaining_igst_liability: f64,
    /// Remaining CGST liability
    pub remaining_cgst_liability: f64,
    /// Remaining SGST liability
    pub remaining_sgst_liability: f64,
    /// Cash payment required
    pub cash_payment_required: f64,
}

/// ITC utilization step
#[derive(Debug, Clone)]
pub struct ItcUtilizationStep {
    /// Utilization type
    pub utilization: ItcUtilization,
    /// Amount utilized
    pub amount: f64,
}

/// Validate GST compliance comprehensively
pub fn validate_gst_compliance(
    registration: &GstRegistration,
    turnover: f64,
    returns_filed: Vec<(ReturnType, Option<NaiveDate>)>,
    itc_claimed: f64,
    itc_available: f64,
) -> GstComplianceReport {
    let mut report = GstComplianceReport {
        compliant: true,
        gstin_valid: registration.gstin.is_valid(),
        returns_status: Vec::new(),
        itc_compliant: true,
        violations: Vec::new(),
        warnings: Vec::new(),
        recommendations: Vec::new(),
        penalty_exposure: 0.0,
        interest_liability: 0.0,
    };

    // Validate GSTIN
    if !report.gstin_valid {
        report.compliant = false;
        report.violations.push(GstError::InvalidGstin {
            gstin: registration.gstin.full.clone(),
        });
    }

    // Check registration status
    if registration.status != RegistrationStatus::Active {
        report.compliant = false;
        match registration.status {
            RegistrationStatus::Cancelled => {
                report.violations.push(GstError::RegistrationCancelled);
            }
            RegistrationStatus::Suspended => {
                report.violations.push(GstError::RegistrationSuspended);
            }
            _ => {}
        }
    }

    // Check returns filing
    let today = chrono::Local::now().date_naive();
    for (return_type, filing_date) in returns_filed {
        let due_date = get_return_due_date(return_type, today);

        match filing_date {
            Some(filed) if filed <= due_date => {
                report.returns_status.push(ReturnComplianceStatus {
                    return_type: format!("{:?}", return_type),
                    period: due_date.format("%b-%Y").to_string(),
                    filed_on_time: true,
                    late_fee: 0.0,
                });
            }
            Some(filed) => {
                let days_late = (filed - due_date).num_days() as u32;
                let late_fee = calculate_late_fee(return_type, days_late, false);
                report.returns_status.push(ReturnComplianceStatus {
                    return_type: format!("{:?}", return_type),
                    period: due_date.format("%b-%Y").to_string(),
                    filed_on_time: false,
                    late_fee,
                });
                report.penalty_exposure += late_fee;
            }
            None if today > due_date => {
                report.compliant = false;
                report.violations.push(GstError::ReturnNotFiled {
                    return_type: format!("{:?}", return_type),
                    period: due_date.format("%b-%Y").to_string(),
                });
            }
            _ => {}
        }
    }

    // Check ITC compliance
    if itc_claimed > itc_available * 1.05 {
        // 5% tolerance
        report.itc_compliant = false;
        report.warnings.push(format!(
            "ITC claimed (Rs. {}) exceeds available ITC (Rs. {}) by more than 5%",
            itc_claimed, itc_available
        ));
    }

    // Composition scheme validation
    if registration.composition_opted {
        let limit = if turnover > 0.0 {
            CompositionBusinessType::Manufacturer.turnover_limit() as f64
        } else {
            15_000_000.0
        };

        if turnover > limit {
            report.warnings.push(format!(
                "Turnover Rs. {} exceeds composition limit of Rs. {}",
                turnover, limit
            ));
            report
                .recommendations
                .push("Consider migrating to regular GST registration".to_string());
        }
    }

    // Add general recommendations
    if !report.returns_status.iter().all(|r| r.filed_on_time) {
        report
            .recommendations
            .push("Set up automated reminders for return due dates".to_string());
    }

    report
}

/// Get due date for return type
pub fn get_return_due_date(return_type: ReturnType, reference_date: NaiveDate) -> NaiveDate {
    let year = reference_date.year();
    let month = reference_date.month();

    match return_type {
        ReturnType::Gstr1 => {
            // 11th of following month
            if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 11).expect("valid date")
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 11).expect("valid date")
            }
        }
        ReturnType::Gstr3b => {
            // 20th of following month
            if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 20).expect("valid date")
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 20).expect("valid date")
            }
        }
        ReturnType::Gstr9 | ReturnType::Gstr9c => {
            // 31st December of following FY
            let fy_end_year = if month >= 4 { year + 1 } else { year };
            NaiveDate::from_ymd_opt(fy_end_year, 12, 31).expect("valid date")
        }
        _ => {
            // Default to 20th of following month
            if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 20).expect("valid date")
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 20).expect("valid date")
            }
        }
    }
}

/// Check if refund claim is within time limit
pub fn is_refund_within_time_limit(relevant_date: NaiveDate, claim_date: NaiveDate) -> bool {
    let months_elapsed = (claim_date.year() - relevant_date.year()) * 12
        + (claim_date.month() as i32 - relevant_date.month() as i32);
    months_elapsed <= 24
}

/// Validate reverse charge applicability
pub fn validate_reverse_charge(
    service_type: ReverseCharge,
    recipient_registered: bool,
    _supplier_registered: bool,
) -> GstResult<bool> {
    match service_type {
        ReverseCharge::ImportServices => {
            // RCM always applies on import of services
            Ok(true)
        }
        ReverseCharge::UnregisteredSupply => {
            // Section 9(4) - currently suspended for most cases
            Ok(false)
        }
        _ => {
            // Section 9(3) services - RCM if recipient is registered
            if recipient_registered {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gstin_validation() {
        assert!(validate_gstin("27AABCU9603R1ZM").is_ok());
        assert!(validate_gstin("invalid").is_err());
        assert!(validate_gstin("27AABCU9603R1Z").is_err()); // Too short
    }

    #[test]
    fn test_registration_requirement() {
        // Below threshold for goods in Maharashtra
        let result = validate_registration_requirement(
            3_000_000.0,
            GstState::Maharashtra,
            SupplyCategory::Goods,
            false,
            false,
        );
        assert!(result.is_ok());
        assert!(!result.expect("valid result")); // Not required

        // Above threshold
        let result = validate_registration_requirement(
            5_000_000.0,
            GstState::Maharashtra,
            SupplyCategory::Goods,
            false,
            false,
        );
        assert!(result.is_ok());
        assert!(result.expect("valid result")); // Required
    }

    #[test]
    fn test_composition_eligibility() {
        let result = validate_composition_eligibility(
            10_000_000.0, // Rs. 1 crore
            CompositionBusinessType::Manufacturer,
            false,
            false,
            false,
            false,
        );
        assert!(result.is_ok());

        // Inter-state supply not allowed
        let result = validate_composition_eligibility(
            10_000_000.0,
            CompositionBusinessType::Manufacturer,
            true, // Makes inter-state supply
            false,
            false,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_late_fee_calculation() {
        // GSTR-3B late by 10 days (non-nil)
        let late_fee = calculate_late_fee(ReturnType::Gstr3b, 10, false);
        assert_eq!(late_fee, 500.0); // Rs. 50 * 10 days

        // GSTR-3B late by 200 days (hits max)
        let late_fee = calculate_late_fee(ReturnType::Gstr3b, 200, false);
        assert_eq!(late_fee, 5000.0); // Max Rs. 5000

        // Nil return
        let late_fee = calculate_late_fee(ReturnType::Gstr3b, 10, true);
        assert_eq!(late_fee, 200.0); // Rs. 20 * 10 days
    }

    #[test]
    fn test_interest_calculation() {
        let interest = calculate_interest(100_000.0, 30, false);
        // 100000 * 18% * 30/365 = Rs. 1479.45
        assert!((interest - 1479.45).abs() < 1.0);

        let interest_wrong_itc = calculate_interest(100_000.0, 30, true);
        // 100000 * 24% * 30/365 = Rs. 1972.60
        assert!((interest_wrong_itc - 1972.60).abs() < 1.0);
    }

    #[test]
    fn test_supply_type_determination() {
        assert_eq!(
            determine_supply_type(GstState::Maharashtra, GstState::Maharashtra, false, false),
            SupplyType::IntraState
        );

        assert_eq!(
            determine_supply_type(GstState::Maharashtra, GstState::Karnataka, false, false),
            SupplyType::InterState
        );

        assert_eq!(
            determine_supply_type(GstState::Maharashtra, GstState::Maharashtra, true, false),
            SupplyType::Export
        );
    }

    #[test]
    fn test_tax_calculation() {
        let liability = calculate_tax(100_000.0, GstRate::Rate18, SupplyType::IntraState, None);
        assert_eq!(liability.cgst, 9000.0);
        assert_eq!(liability.sgst, 9000.0);
        assert_eq!(liability.igst, 0.0);

        let liability = calculate_tax(100_000.0, GstRate::Rate18, SupplyType::InterState, None);
        assert_eq!(liability.cgst, 0.0);
        assert_eq!(liability.sgst, 0.0);
        assert_eq!(liability.igst, 18000.0);
    }

    #[test]
    fn test_itc_utilization() {
        let plan = validate_itc_utilization(
            10000.0, // IGST credit
            5000.0,  // CGST credit
            5000.0,  // SGST credit
            8000.0,  // IGST liability
            4000.0,  // CGST liability
            4000.0,  // SGST liability
        )
        .expect("valid plan");

        // Should use IGST first for IGST liability
        assert!(!plan.steps.is_empty());
        assert_eq!(plan.steps[0].utilization, ItcUtilization::IgstForIgst);
        assert_eq!(plan.steps[0].amount, 8000.0);

        // Cash payment should be minimal
        assert!(plan.cash_payment_required < 100.0);
    }

    #[test]
    fn test_refund_time_limit() {
        let relevant_date = NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date");
        let within_limit = NaiveDate::from_ymd_opt(2024, 12, 31).expect("valid date");
        let outside_limit = NaiveDate::from_ymd_opt(2025, 2, 1).expect("valid date");

        assert!(is_refund_within_time_limit(relevant_date, within_limit));
        assert!(!is_refund_within_time_limit(relevant_date, outside_limit));
    }

    #[test]
    fn test_eway_bill_requirement() {
        // Inter-state above threshold
        let result = validate_eway_bill_requirement(
            60_000.0,
            SupplyType::InterState,
            GstState::Maharashtra,
            GstState::Karnataka,
            "Electronics",
        );
        assert!(result.expect("valid result"));

        // Exempt goods
        let result = validate_eway_bill_requirement(
            100_000.0,
            SupplyType::InterState,
            GstState::Maharashtra,
            GstState::Karnataka,
            "Fresh vegetables",
        );
        assert!(!result.expect("valid result"));
    }
}
