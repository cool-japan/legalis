//! Commercial Law Validators (商法・会社法のバリデーター)
//!
//! Comprehensive validation functions for commercial law compliance.

use crate::commercial_law::error::{CommercialLawError, Result};
use crate::commercial_law::types::*;
use chrono::{Duration, Utc};

// ============================================================================
// Capital Validation (資本金の検証)
// ============================================================================

/// Validates capital amount meets legal requirements
/// (資本金の法的要件を検証 - Shihon-kin no hōteki yōken wo kenshō)
pub fn validate_capital(capital: &Capital) -> Result<()> {
    if capital.amount_jpy < Capital::MINIMUM {
        return Err(CommercialLawError::CapitalBelowMinimum {
            actual: capital.amount_jpy,
            minimum: Capital::MINIMUM,
        });
    }
    Ok(())
}

// ============================================================================
// Company Formation Validation (会社設立の検証)
// ============================================================================

/// Validates articles of incorporation for company formation
/// (定款の検証 - Teikan no kenshō)
pub fn validate_articles_of_incorporation(
    articles: &ArticlesOfIncorporation,
    company_type: CompanyType,
) -> Result<()> {
    // Validate company name (商号検証 - Shōgō kenshō)
    validate_company_name(&articles.company_name, company_type)?;

    // Validate business purposes (事業目的検証 - Jigyō mokuteki kenshō)
    validate_business_purposes(&articles.business_purposes)?;

    // Validate head office location (本店所在地検証 - Honten shozaichi kenshō)
    validate_head_office_location(&articles.head_office_location)?;

    // Validate capital (資本金検証 - Shihon-kin kenshō)
    validate_capital(&articles.capital)?;

    // Validate fiscal year end (決算期検証 - Kessan-ki kenshō)
    validate_fiscal_year_end(articles.fiscal_year_end_month)?;

    // Validate incorporators (発起人検証 - Hokki-nin kenshō)
    validate_incorporators(&articles.incorporators, &articles.capital)?;

    // For stock companies, validate authorized shares
    // (株式会社の場合、発行可能株式総数を検証)
    if company_type == CompanyType::StockCompany {
        validate_authorized_shares_for_stock_company(articles)?;
    }

    Ok(())
}

/// Validates company name includes proper suffix
/// (商号に適切な表示があるか検証 - Shōgō ni tekisetsu na hyōji ga aru ka kenshō)
pub fn validate_company_name(name: &str, company_type: CompanyType) -> Result<()> {
    if name.is_empty() {
        return Err(CommercialLawError::InvalidCompanyName {
            reason: "Company name cannot be empty".to_string(),
        });
    }

    // Check for required suffix based on company type
    let required_suffix = match company_type {
        CompanyType::StockCompany => "株式会社",
        CompanyType::LLC => "合同会社",
        CompanyType::LimitedPartnership => "合資会社",
        CompanyType::GeneralPartnership => "合名会社",
    };

    if !name.contains(required_suffix) {
        return Err(CommercialLawError::MissingCompanyTypeSuffix {
            name: name.to_string(),
        });
    }

    Ok(())
}

/// Validates business purposes are specified and valid
/// (事業目的が指定され有効であるか検証 - Jigyō mokuteki ga shitei sare yūkō de aru ka kenshō)
pub fn validate_business_purposes(purposes: &[String]) -> Result<()> {
    if purposes.is_empty() {
        return Err(CommercialLawError::NoBusinessPurposes);
    }

    for purpose in purposes {
        if purpose.trim().is_empty() {
            return Err(CommercialLawError::InvalidBusinessPurpose {
                reason: "Business purpose cannot be empty or whitespace".to_string(),
            });
        }
        if purpose.len() < 3 {
            return Err(CommercialLawError::InvalidBusinessPurpose {
                reason: format!(
                    "Business purpose '{}' is too short (minimum 3 characters)",
                    purpose
                ),
            });
        }
    }

    Ok(())
}

/// Validates head office location
/// (本店所在地を検証 - Honten shozaichi wo kenshō)
pub fn validate_head_office_location(location: &str) -> Result<()> {
    if location.trim().is_empty() {
        return Err(CommercialLawError::MissingRequiredField {
            field_name: "head_office_location".to_string(),
        });
    }
    Ok(())
}

/// Validates fiscal year end month
/// (決算期を検証 - Kessan-ki wo kenshō)
pub fn validate_fiscal_year_end(month: u8) -> Result<()> {
    if !(1..=12).contains(&month) {
        return Err(CommercialLawError::InvalidFiscalYearEnd { month });
    }
    Ok(())
}

/// Validates incorporators meet requirements
/// (発起人が要件を満たすか検証 - Hokki-nin ga yōken wo mitasu ka kenshō)
pub fn validate_incorporators(incorporators: &[Incorporator], capital: &Capital) -> Result<()> {
    if incorporators.is_empty() {
        return Err(CommercialLawError::NoIncorporators);
    }

    // Calculate total investment
    let total_investment: u64 = incorporators
        .iter()
        .map(|inc| inc.investment_amount_jpy)
        .sum();

    // Verify total investment matches capital
    if total_investment != capital.amount_jpy {
        return Err(CommercialLawError::InvestmentCapitalMismatch {
            total_investment,
            capital: capital.amount_jpy,
        });
    }

    // Validate each incorporator
    for incorporator in incorporators {
        validate_incorporator(incorporator)?;
    }

    Ok(())
}

/// Validates individual incorporator data
/// (個別の発起人データを検証 - Kobetsu no hokki-nin dēta wo kenshō)
pub fn validate_incorporator(incorporator: &Incorporator) -> Result<()> {
    if incorporator.name.trim().is_empty() {
        return Err(CommercialLawError::InvalidIncorporatorInvestment {
            name: "(empty name)".to_string(),
            reason: "Incorporator name cannot be empty".to_string(),
        });
    }

    if incorporator.address.trim().is_empty() {
        return Err(CommercialLawError::InvalidIncorporatorInvestment {
            name: incorporator.name.clone(),
            reason: "Incorporator address cannot be empty".to_string(),
        });
    }

    if incorporator.investment_amount_jpy == 0 {
        return Err(CommercialLawError::InvalidIncorporatorInvestment {
            name: incorporator.name.clone(),
            reason: "Investment amount must be greater than zero".to_string(),
        });
    }

    Ok(())
}

/// Validates authorized shares for stock companies
/// (株式会社の発行可能株式総数を検証)
pub fn validate_authorized_shares_for_stock_company(
    articles: &ArticlesOfIncorporation,
) -> Result<()> {
    let authorized = articles
        .authorized_shares
        .ok_or(CommercialLawError::AuthorizedSharesNotSpecified)?;

    if authorized == 0 {
        return Err(CommercialLawError::InvalidShareIssuance {
            reason: "Authorized shares must be greater than zero".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Corporate Governance Validation (コーポレートガバナンスの検証)
// ============================================================================

/// Validates board of directors meets legal requirements
/// (取締役会が法的要件を満たすか検証 - Torishimari-yaku kai ga hōteki yōken wo mitasu ka kenshō)
pub fn validate_board_of_directors(board: &BoardOfDirectors, requires_board: bool) -> Result<()> {
    if requires_board && board.directors.len() < 3 {
        return Err(CommercialLawError::InsufficientDirectors {
            required: 3,
            actual: board.directors.len(),
        });
    }

    // Validate each director's term
    for director in &board.directors {
        validate_director_term(director)?;
    }

    Ok(())
}

/// Validates director term does not exceed maximum (2 years for stock companies)
/// (取締役の任期が上限を超えないか検証 - Torishimari-yaku no ninki ga jōgen wo koenai ka kenshō)
pub fn validate_director_term(director: &Director) -> Result<()> {
    if let Some(term_end) = director.term_end {
        let duration = term_end - director.term_start;
        let max_duration = Duration::days(365 * 2 + 1); // ~2 years with leap day

        if duration > max_duration {
            return Err(CommercialLawError::DirectorTermTooLong { max_years: 2 });
        }
    }

    Ok(())
}

/// Validates corporate auditors meet legal requirements
/// (監査役が法的要件を満たすか検証 - Kansa-yaku ga hōteki yōken wo mitasu ka kenshō)
pub fn validate_corporate_auditors(
    auditors: &CorporateAuditors,
    is_large_company: bool,
    requires_audit_board: bool,
) -> Result<()> {
    if requires_audit_board && auditors.auditors.len() < 3 {
        return Err(CommercialLawError::InsufficientAuditors {
            required: 3,
            actual: auditors.auditors.len(),
        });
    }

    // For large companies with audit board, at least half must be outside
    if is_large_company && requires_audit_board {
        let outside_count = auditors.auditors.iter().filter(|a| a.is_outside).count();
        let total = auditors.auditors.len();

        if outside_count * 2 < total {
            return Err(CommercialLawError::InsufficientOutsideAuditors {
                outside: outside_count,
                total,
            });
        }
    }

    // Validate each auditor's term
    for auditor in &auditors.auditors {
        validate_auditor_term(auditor)?;
    }

    Ok(())
}

/// Validates auditor term does not exceed maximum (4 years)
/// (監査役の任期が上限を超えないか検証 - Kansa-yaku no ninki ga jōgen wo koenai ka kenshō)
pub fn validate_auditor_term(auditor: &CorporateAuditor) -> Result<()> {
    if let Some(term_end) = auditor.term_end {
        let duration = term_end - auditor.term_start;
        let max_duration = Duration::days(365 * 4 + 1); // ~4 years with leap day

        if duration > max_duration {
            return Err(CommercialLawError::AuditorTermTooLong);
        }
    }

    Ok(())
}

/// Validates shareholders meeting resolution
/// (株主総会決議を検証 - Kabunushi sōkai ketsugi wo kenshō)
pub fn validate_shareholders_meeting_resolution(meeting: &ShareholdersMeeting) -> Result<()> {
    // Check quorum (定足数確認 - Teisoku-sū kakunin)
    if !meeting.quorum_met {
        return Err(CommercialLawError::QuorumNotMet {
            present: meeting.voting_rights_present,
            total: meeting.voting_rights_total,
            required_percent: 50, // Simplified; actual requirements vary
        });
    }

    // Validate each agenda item resolution
    for item in &meeting.agenda_items {
        validate_resolution(item, meeting.voting_rights_present)?;
    }

    Ok(())
}

/// Validates individual agenda item resolution
/// (個別の議案決議を検証 - Kobetsu no gian ketsugi wo kenshō)
pub fn validate_resolution(item: &AgendaItem, voting_rights_present: u64) -> Result<()> {
    let _total_votes = item.votes_favor + item.votes_against + item.abstentions;

    // Determine required threshold based on resolution type
    let (required_percent, base_votes) = match item.resolution_type {
        ResolutionType::OrdinaryResolution => {
            // Majority of voting rights present (Article 309-1)
            (50, voting_rights_present)
        }
        ResolutionType::SpecialResolution => {
            // 2/3 of voting rights present (Article 309-2)
            (67, voting_rights_present) // Using 67% for 2/3
        }
        ResolutionType::ExtraordinaryResolution => {
            // Higher thresholds for specific matters
            (75, voting_rights_present)
        }
    };

    // Calculate if resolution passes
    let required_votes = (base_votes * required_percent as u64) / 100;
    let passes = item.votes_favor > required_votes;

    // Verify result matches calculated outcome
    if let Some(result) = item.result {
        let expected_result = if passes {
            ResolutionResult::Approved
        } else {
            ResolutionResult::Rejected
        };

        if result != expected_result {
            return Err(CommercialLawError::InsufficientVotes {
                favor: item.votes_favor,
                against: item.votes_against,
                base: base_votes,
                required_percent,
            });
        }
    }

    Ok(())
}

// ============================================================================
// Share Transfer Validation (株式譲渡の検証)
// ============================================================================

/// Validates share transfer compliance
/// (株式譲渡のコンプライアンスを検証 - Kabushiki jōto no konpuraiansu wo kenshō)
pub fn validate_share_transfer(transfer: &ShareTransfer) -> Result<()> {
    if transfer.number_of_shares == 0 {
        return Err(CommercialLawError::InvalidShareIssuance {
            reason: "Number of shares to transfer must be greater than zero".to_string(),
        });
    }

    // If board approval required, check it was obtained
    if transfer.requires_board_approval && !transfer.board_approval_obtained {
        return Err(CommercialLawError::ShareTransferApprovalRequired);
    }

    Ok(())
}

// ============================================================================
// Share Issuance Validation (株式発行の検証)
// ============================================================================

/// Validates share issuance compliance
/// (株式発行のコンプライアンスを検証 - Kabushiki hakkō no konpuraiansu wo kenshō)
pub fn validate_share_issuance(
    issuance: &ShareIssuance,
    current_issued_shares: u64,
    authorized_shares: u64,
) -> Result<()> {
    if issuance.shares_to_issue == 0 {
        return Err(CommercialLawError::InvalidShareIssuance {
            reason: "Number of shares to issue must be greater than zero".to_string(),
        });
    }

    if issuance.price_per_share_jpy == 0 {
        return Err(CommercialLawError::InvalidShareIssuance {
            reason: "Issue price must be greater than zero".to_string(),
        });
    }

    // Check authorized shares limit
    let new_total = current_issued_shares + issuance.shares_to_issue;
    if new_total > authorized_shares {
        return Err(CommercialLawError::ExceedsAuthorizedShares {
            new_total,
            authorized: authorized_shares,
        });
    }

    // Check payment deadline is in the future
    if issuance.payment_deadline < Utc::now() {
        return Err(CommercialLawError::InvalidDate {
            reason: "Payment deadline must be in the future".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Commercial Transaction Validation (商取引の検証)
// ============================================================================

/// Validates commercial transaction
/// (商取引を検証 - Shō torihiki wo kenshō)
pub fn validate_commercial_transaction(transaction: &CommercialTransaction) -> Result<()> {
    if transaction.amount_jpy == 0 {
        return Err(CommercialLawError::InvalidCommercialTransaction {
            reason: "Transaction amount must be greater than zero".to_string(),
        });
    }

    if transaction.parties.is_empty() {
        return Err(CommercialLawError::InvalidCommercialTransaction {
            reason: "At least one party must be specified".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_capital() {
        let valid_capital = Capital::new(1_000_000);
        assert!(validate_capital(&valid_capital).is_ok());

        let invalid_capital = Capital::new(0);
        assert!(validate_capital(&invalid_capital).is_err());
    }

    #[test]
    fn test_validate_company_name() {
        assert!(validate_company_name("テスト株式会社", CompanyType::StockCompany).is_ok());

        assert!(validate_company_name("テスト", CompanyType::StockCompany).is_err());

        assert!(validate_company_name("テスト合同会社", CompanyType::LLC).is_ok());
    }

    #[test]
    fn test_validate_business_purposes() {
        let valid_purposes = vec!["Software development".to_string()];
        assert!(validate_business_purposes(&valid_purposes).is_ok());

        let empty_purposes: Vec<String> = vec![];
        assert!(validate_business_purposes(&empty_purposes).is_err());

        let invalid_purposes = vec!["".to_string()];
        assert!(validate_business_purposes(&invalid_purposes).is_err());
    }

    #[test]
    fn test_validate_fiscal_year_end() {
        assert!(validate_fiscal_year_end(12).is_ok());
        assert!(validate_fiscal_year_end(1).is_ok());
        assert!(validate_fiscal_year_end(0).is_err());
        assert!(validate_fiscal_year_end(13).is_err());
    }

    #[test]
    fn test_validate_incorporators() {
        let capital = Capital::new(1_000_000);
        let incorporators = vec![Incorporator {
            name: "John Doe".to_string(),
            address: "Tokyo".to_string(),
            shares_subscribed: Some(100),
            investment_amount_jpy: 1_000_000,
        }];

        assert!(validate_incorporators(&incorporators, &capital).is_ok());

        // Test empty incorporators
        let empty_incorporators: Vec<Incorporator> = vec![];
        assert!(validate_incorporators(&empty_incorporators, &capital).is_err());

        // Test investment mismatch
        let mismatched_incorporators = vec![Incorporator {
            name: "Jane Doe".to_string(),
            address: "Osaka".to_string(),
            shares_subscribed: Some(100),
            investment_amount_jpy: 500_000, // Doesn't match capital
        }];
        assert!(validate_incorporators(&mismatched_incorporators, &capital).is_err());
    }

    #[test]
    fn test_validate_board_of_directors() {
        let directors = vec![
            Director {
                name: "Director 1".to_string(),
                position: DirectorPosition::President,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + Duration::days(365)),
            },
            Director {
                name: "Director 2".to_string(),
                position: DirectorPosition::Director,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + Duration::days(365)),
            },
            Director {
                name: "Director 3".to_string(),
                position: DirectorPosition::Director,
                term_start: Utc::now(),
                term_end: Some(Utc::now() + Duration::days(365)),
            },
        ];

        let board = BoardOfDirectors {
            directors: directors.clone(),
            meeting_frequency: Some("Monthly".to_string()),
        };

        assert!(validate_board_of_directors(&board, true).is_ok());

        // Test insufficient directors
        let insufficient_board = BoardOfDirectors {
            directors: directors[..2].to_vec(),
            meeting_frequency: Some("Monthly".to_string()),
        };
        assert!(validate_board_of_directors(&insufficient_board, true).is_err());
    }

    #[test]
    fn test_validate_share_transfer() {
        let valid_transfer = ShareTransfer {
            transferor: "Alice".to_string(),
            transferee: "Bob".to_string(),
            number_of_shares: 100,
            share_class: "Common".to_string(),
            transfer_date: Utc::now(),
            requires_board_approval: false,
            board_approval_obtained: false,
        };
        assert!(validate_share_transfer(&valid_transfer).is_ok());

        let invalid_transfer = ShareTransfer {
            transferor: "Alice".to_string(),
            transferee: "Bob".to_string(),
            number_of_shares: 100,
            share_class: "Common".to_string(),
            transfer_date: Utc::now(),
            requires_board_approval: true,
            board_approval_obtained: false,
        };
        assert!(validate_share_transfer(&invalid_transfer).is_err());
    }

    #[test]
    fn test_validate_share_issuance() {
        let future_date = Utc::now() + Duration::days(30);
        let valid_issuance = ShareIssuance {
            shares_to_issue: 1000,
            price_per_share_jpy: 50000,
            payment_deadline: future_date,
            preemptive_rights_offered: true,
        };

        assert!(validate_share_issuance(&valid_issuance, 5000, 10000).is_ok());

        // Test exceeding authorized shares
        assert!(validate_share_issuance(&valid_issuance, 9500, 10000).is_err());
    }
}
