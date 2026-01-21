//! Validation functions for German stock corporations (AktG)
//!
//! Multi-stage validation implementing AktG requirements for AG formation.

use crate::aktg::error::{AktGError, Result};
use crate::aktg::types::{
    AG, BoardMember, FiscalYearEnd, ManagementBoard, Share, ShareType, SupervisoryBoard,
};
use crate::gmbhg::Capital;

// =============================================================================
// Share Capital Validation
// =============================================================================

/// Validate AG share capital (§7 AktG)
///
/// Requirements:
/// - Minimum €50,000 (§7 AktG)
/// - Must be non-zero
pub fn validate_share_capital(capital: &Capital) -> Result<()> {
    const AG_MINIMUM_CENTS: u64 = 5_000_000; // €50,000

    if capital.amount_cents == 0 {
        return Err(AktGError::ZeroCapital);
    }

    if capital.amount_cents < AG_MINIMUM_CENTS {
        return Err(AktGError::CapitalBelowMinimum {
            actual: capital.to_euros(),
        });
    }

    Ok(())
}

// =============================================================================
// Share Validation
// =============================================================================

/// Validate shares and ensure they match capital
pub fn validate_shares(shares: &[Share], share_capital: &Capital) -> Result<()> {
    if shares.is_empty() {
        return Err(AktGError::NoShares);
    }

    // Calculate total par value from all shares
    let total_par_value_cents: u64 = shares.iter().map(calculate_share_par_value).sum();

    if total_par_value_cents != share_capital.amount_cents {
        return Err(AktGError::SharesMismatchCapital {
            shares_total: (total_par_value_cents as f64) / 100.0,
            capital: share_capital.to_euros(),
        });
    }

    // Validate each share
    for share in shares {
        validate_share(share, share_capital)?;
    }

    Ok(())
}

/// Calculate total par value for a share allocation
fn calculate_share_par_value(share: &Share) -> u64 {
    match share.share_type {
        ShareType::ParValue { par_value_cents } => par_value_cents * share.quantity,
        ShareType::NoPar => {
            // For no-par shares, notional value is handled separately
            0 // Will be validated against total capital
        }
    }
}

/// Validate individual share
fn validate_share(share: &Share, total_capital: &Capital) -> Result<()> {
    // Validate par value for par value shares
    if let ShareType::ParValue { par_value_cents } = share.share_type
        && par_value_cents < 100
    {
        // €1 minimum
        return Err(AktGError::ParValueTooLow {
            par_value: (par_value_cents as f64) / 100.0,
        });
    }

    // For no-par shares, validate notional value
    if share.share_type == ShareType::NoPar {
        // Calculate total no-par shares across all allocations
        // For simplicity, we check that notional value >= €1
        // This would need the total count of all no-par shares in practice
        let total_shares = share.quantity;
        if total_shares > 0 {
            let notional_value_cents = total_capital.amount_cents / total_shares;
            if notional_value_cents < 100 {
                // €1 minimum
                return Err(AktGError::NotionalValueTooLow {
                    notional_value: (notional_value_cents as f64) / 100.0,
                });
            }
        }
    }

    // Validate initial payment (§36a AktG)
    // Must be at least 25% of par value/issue price + full premium
    let minimum_payment = calculate_minimum_initial_payment(share);
    if share.amount_paid.amount_cents < minimum_payment {
        return Err(AktGError::InsufficientInitialPayment {
            paid: share.amount_paid.to_euros(),
            required: (minimum_payment as f64) / 100.0,
        });
    }

    // Validate amount paid does not exceed issue price
    if share.amount_paid.amount_cents > share.issue_price.amount_cents * share.quantity {
        return Err(AktGError::PaidExceedsIssuePrice {
            paid: share.amount_paid.to_euros(),
            issue_price: share.issue_price.to_euros(),
        });
    }

    Ok(())
}

/// Calculate minimum initial payment per §36a AktG
///
/// For each share: 25% of par value/notional value + full premium
fn calculate_minimum_initial_payment(share: &Share) -> u64 {
    let par_value_per_share = match share.share_type {
        ShareType::ParValue { par_value_cents } => par_value_cents,
        ShareType::NoPar => share.issue_price.amount_cents, // Simplified
    };

    // Premium (Agio) = Issue price - Par value
    let premium = share
        .issue_price
        .amount_cents
        .saturating_sub(par_value_per_share);

    // Minimum = 25% of par value + full premium
    let minimum_per_share = (par_value_per_share / 4) + premium;
    minimum_per_share * share.quantity
}

// =============================================================================
// Company Name Validation
// =============================================================================

/// Validate company name (§4 AktG)
pub fn validate_company_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(AktGError::EmptyCompanyName);
    }

    if name.trim().len() < 3 {
        return Err(AktGError::CompanyNameTooShort {
            name: name.to_string(),
        });
    }

    // Check for required suffix (AG or Aktiengesellschaft)
    let normalized = name.replace([' ', '.'], "").to_lowercase();
    let valid = normalized.contains("ag") || normalized.contains("aktiengesellschaft");

    if !valid {
        return Err(AktGError::MissingLegalFormSuffix {
            name: name.to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Management Board Validation
// =============================================================================

/// Validate management board (§76-94 AktG)
pub fn validate_management_board(board: &ManagementBoard) -> Result<()> {
    if board.members.is_empty() {
        return Err(AktGError::NoManagementBoardMembers);
    }

    for member in &board.members {
        validate_board_member(member)?;
    }

    Ok(())
}

/// Validate individual board member
fn validate_board_member(member: &BoardMember) -> Result<()> {
    if member.name.trim().is_empty() {
        return Err(AktGError::EmptyBoardMemberName);
    }

    if member.address.trim().is_empty() {
        return Err(AktGError::EmptyBoardMemberAddress {
            name: member.name.clone(),
        });
    }

    if !member.has_capacity {
        return Err(AktGError::BoardMemberLacksCapacity {
            name: member.name.clone(),
        });
    }

    // Validate term does not exceed 5 years (§84 Abs. 1 AktG)
    if let Some(term_end) = member.term_end_date {
        let duration = term_end.signed_duration_since(member.appointment_date);
        let max_duration = chrono::Duration::days(5 * 365 + 1); // 5 years + leap day
        if duration > max_duration {
            return Err(AktGError::BoardMemberTermTooLong {
                name: member.name.clone(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Supervisory Board Validation
// =============================================================================

/// Validate supervisory board (§95-116 AktG)
pub fn validate_supervisory_board(board: &SupervisoryBoard) -> Result<()> {
    if board.members.len() < 3 {
        return Err(AktGError::InsufficientSupervisoryBoardMembers);
    }

    // Board size must be divisible by 3 (§95, §101 AktG)
    if !board.members.len().is_multiple_of(3) {
        return Err(AktGError::SupervisoryBoardSizeNotDivisibleByThree {
            size: board.members.len(),
        });
    }

    // Validate chairman is a member
    if !board.members.iter().any(|m| m.name == board.chairman_name) {
        return Err(AktGError::ChairmanNotMember {
            chairman: board.chairman_name.clone(),
        });
    }

    // Validate deputy chairman if specified
    if let Some(deputy) = &board.deputy_chairman_name
        && !board.members.iter().any(|m| &m.name == deputy)
    {
        return Err(AktGError::DeputyChairmanNotMember {
            deputy: deputy.clone(),
        });
    }

    // Validate each member
    for member in &board.members {
        if member.name.trim().is_empty() {
            return Err(AktGError::EmptySupervisoryBoardMemberName);
        }

        // Validate term does not exceed 4 years (§102 AktG)
        if let Some(term_end) = member.term_end_date {
            let duration = term_end.signed_duration_since(member.appointment_date);
            let max_duration = chrono::Duration::days(4 * 365 + 1); // 4 years
            if duration > max_duration {
                return Err(AktGError::SupervisoryBoardMemberTermTooLong {
                    name: member.name.clone(),
                });
            }
        }
    }

    Ok(())
}

// =============================================================================
// Business Purpose & Office Validation
// =============================================================================

/// Validate business purpose
pub fn validate_business_purpose(purpose: &str) -> Result<()> {
    if purpose.trim().is_empty() {
        return Err(AktGError::EmptyBusinessPurpose);
    }

    if purpose.trim().len() < 10 {
        return Err(AktGError::InvalidBusinessPurpose);
    }

    Ok(())
}

/// Validate registered office
pub fn validate_registered_office(city: &str) -> Result<()> {
    if city.trim().is_empty() {
        return Err(AktGError::EmptyRegisteredOffice);
    }

    Ok(())
}

/// Validate fiscal year end
pub fn validate_fiscal_year_end(fye: FiscalYearEnd) -> Result<()> {
    if !(1..=12).contains(&fye.month) {
        return Err(AktGError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    let max_day = match fye.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29,
        _ => {
            return Err(AktGError::InvalidFiscalYearEnd {
                month: fye.month,
                day: fye.day,
            });
        }
    };

    if fye.day < 1 || fye.day > max_day {
        return Err(AktGError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    Ok(())
}

// =============================================================================
// Master AG Validation
// =============================================================================

/// Validate complete AG structure
pub fn validate_ag(ag: &AG) -> Result<()> {
    validate_company_name(&ag.company_name)?;
    validate_registered_office(&ag.registered_office)?;
    validate_business_purpose(&ag.business_purpose)?;
    validate_share_capital(&ag.share_capital)?;
    validate_shares(&ag.shares, &ag.share_capital)?;
    validate_management_board(&ag.management_board)?;
    validate_supervisory_board(&ag.supervisory_board)?;

    if let Some(fye) = ag.fiscal_year_end {
        validate_fiscal_year_end(fye)?;
    }

    Ok(())
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_share_capital_valid() {
        let capital = Capital::from_euros(50_000);
        assert!(validate_share_capital(&capital).is_ok());
    }

    #[test]
    fn test_validate_share_capital_below_minimum() {
        let capital = Capital::from_euros(49_999);
        let result = validate_share_capital(&capital);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AktGError::CapitalBelowMinimum { .. }
        ));
    }

    #[test]
    fn test_validate_share_capital_zero() {
        let capital = Capital::from_euros(0);
        let result = validate_share_capital(&capital);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AktGError::ZeroCapital));
    }

    #[test]
    fn test_validate_company_name_valid() {
        assert!(validate_company_name("Tech Solutions AG").is_ok());
        assert!(validate_company_name("Mustermann Aktiengesellschaft").is_ok());
        assert!(validate_company_name("Company A.G.").is_ok());
    }

    #[test]
    fn test_validate_company_name_missing_suffix() {
        let result = validate_company_name("Tech Solutions");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AktGError::MissingLegalFormSuffix { .. }
        ));
    }

    #[test]
    fn test_validate_company_name_empty() {
        let result = validate_company_name("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AktGError::EmptyCompanyName));
    }
}
