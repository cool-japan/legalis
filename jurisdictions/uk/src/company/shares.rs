//! Share Capital and Transactions
//!
//! This module implements share capital rules under Companies Act 2006 Part 17.
//!
//! ## Share Allotment (CA 2006 ss.549-559)
//!
//! ### Private Companies
//! - Directors generally have power to allot unless restricted
//! - No need for shareholder authorization (s.550)
//!
//! ### Public Companies
//! - Directors need shareholder authorization (s.551)
//! - Authorization valid for max 5 years
//! - Pre-emption rights apply (ss.561-567)
//!
//! ## Pre-emption Rights (ss.561-567)
//!
//! Existing shareholders have right of first refusal on new shares:
//! - Ordinary shares must be offered to existing holders pro rata
//! - Exception: shares for non-cash consideration
//! - Can be disapplied by special resolution
//!
//! ## Share Transfer (Stock Transfer Act 1963)
//!
//! - Proper instrument of transfer required
//! - Directors may refuse to register (if permitted by articles)
//! - Listed shares: CREST electronic settlement
//!
//! ## Distributions (CA 2006 Part 23)
//!
//! ### Basic Rule (s.830)
//! Company can only distribute from accumulated realized profits less realized losses.
//!
//! ### Public Companies (s.831)
//! Additional capital maintenance requirement:
//! - Can only distribute if net assets exceed aggregate of called-up share capital + undistributable reserves
//! - Net assets cannot be reduced below this amount
//!
//! ## Share Buybacks (CA 2006 ss.690-708)
//!
//! Company may purchase own shares if:
//! - Authorized by articles
//! - Shares fully paid
//! - Payment from distributable profits (or proceeds of fresh issue)
//! - Private companies: can use capital with solvency statement

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::types::CompanyType;

// ============================================================================
// Share Classes
// ============================================================================

/// Share class designation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareClassType {
    /// Ordinary shares - voting rights, variable dividend
    Ordinary,
    /// Preference shares - fixed dividend, priority on winding up
    Preference,
    /// Non-voting shares
    NonVoting,
    /// Deferred shares - dividend after ordinary
    Deferred,
    /// Redeemable shares
    Redeemable,
    /// Alphabet shares (A, B, C classes)
    Alphabet {
        /// Share class letter designation
        class: char,
    },
}

impl ShareClassType {
    /// Get typical characteristics
    pub fn description(&self) -> &'static str {
        match self {
            Self::Ordinary => {
                "Standard shares with voting rights and right to participate in dividends \
                 and capital distribution"
            }
            Self::Preference => {
                "Fixed dividend rate, priority on winding up, typically no voting rights \
                 unless dividend in arrears"
            }
            Self::NonVoting => "No voting rights but entitled to dividends and capital",
            Self::Deferred => {
                "Dividend entitlement only after ordinary shareholders receive specified amount"
            }
            Self::Redeemable => "May be bought back by company under CA 2006 ss.684-689",
            Self::Alphabet { .. } => "Custom share class with specific rights attached",
        }
    }
}

/// Detailed share class specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareClassSpec {
    /// Class name
    pub name: String,
    /// Class type
    pub class_type: ShareClassType,
    /// Nominal value per share
    pub nominal_value: f64,
    /// Currency
    pub currency: String,
    /// Number of shares in class
    pub number_of_shares: u64,
    /// Voting rights per share
    pub votes_per_share: u32,
    /// Dividend entitlement
    pub dividend_entitlement: DividendEntitlement,
    /// Priority on winding up
    pub winding_up_priority: u32,
    /// Is redeemable
    pub redeemable: bool,
    /// Conversion rights
    pub convertible: Option<ConversionRights>,
}

/// Dividend entitlement specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DividendEntitlement {
    /// Participates equally with ordinary shares
    ProRata,
    /// Fixed rate preference dividend
    Fixed {
        /// Rate as percentage of nominal value
        rate_percent: f64,
        /// Is cumulative (arrears carried forward)
        cumulative: bool,
    },
    /// Participating preference (fixed plus participation)
    Participating {
        /// Minimum fixed rate
        fixed_rate: f64,
        /// Additional participation percentage
        participation_percent: f64,
    },
    /// No dividend entitlement
    None,
}

/// Conversion rights for convertible shares
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionRights {
    /// Class to convert into
    pub target_class: String,
    /// Conversion ratio
    pub ratio: f64,
    /// Conversion start date
    pub conversion_start: Option<NaiveDate>,
    /// Conversion end date
    pub conversion_end: Option<NaiveDate>,
    /// Conversion price
    pub conversion_price: Option<f64>,
}

// ============================================================================
// Share Allotment
// ============================================================================

/// Share allotment under CA 2006 ss.549-559
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareAllotment {
    /// Company type
    pub company_type: CompanyType,
    /// Number of shares to allot
    pub number_of_shares: u64,
    /// Share class
    pub share_class: String,
    /// Nominal value per share
    pub nominal_value: f64,
    /// Issue price per share
    pub issue_price: f64,
    /// Allottees
    pub allottees: Vec<Allottee>,
    /// Is for cash consideration
    pub cash_consideration: bool,
    /// Authorization details
    pub authorization: AllotmentAuthorization,
    /// Pre-emption analysis
    pub pre_emption: PreEmptionAnalysis,
}

/// Allottee receiving shares
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Allottee {
    /// Name
    pub name: String,
    /// Number of shares
    pub shares: u64,
    /// Consideration paid
    pub consideration: f64,
    /// Is existing shareholder
    pub existing_shareholder: bool,
}

/// Allotment authorization (CA 2006 s.551)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllotmentAuthorization {
    /// Has valid authorization
    pub has_authorization: bool,
    /// Date of authorization
    pub authorization_date: Option<NaiveDate>,
    /// Expiry date (max 5 years)
    pub expiry_date: Option<NaiveDate>,
    /// Maximum shares authorized
    pub max_shares_authorized: Option<u64>,
    /// Resolution reference
    pub resolution_reference: Option<String>,
    /// Analysis
    pub analysis: String,
}

impl AllotmentAuthorization {
    /// Check if authorization valid for private company
    pub fn analyze_private(has_restriction_in_articles: bool) -> Self {
        if has_restriction_in_articles {
            Self {
                has_authorization: false,
                authorization_date: None,
                expiry_date: None,
                max_shares_authorized: None,
                resolution_reference: None,
                analysis: "Private company with articles restricting allotment power. \
                          Shareholder authorization required under CA 2006 s.551."
                    .to_string(),
            }
        } else {
            Self {
                has_authorization: true,
                authorization_date: None,
                expiry_date: None,
                max_shares_authorized: None,
                resolution_reference: None,
                analysis: "Private company - directors have power to allot under CA 2006 s.550 \
                          (one class of shares, no restriction in articles)."
                    .to_string(),
            }
        }
    }

    /// Check if authorization valid for public company
    pub fn analyze_public(
        authorization_date: Option<NaiveDate>,
        expiry_date: Option<NaiveDate>,
        max_authorized: Option<u64>,
        shares_to_allot: u64,
        current_date: NaiveDate,
    ) -> Self {
        let has_authorization = authorization_date.is_some()
            && expiry_date.is_some_and(|exp| exp > current_date)
            && max_authorized.is_some_and(|max| max >= shares_to_allot);

        let analysis = if has_authorization {
            "Public company has valid s.551 authorization from shareholders.".to_string()
        } else if authorization_date.is_none() {
            "Public company - no shareholder authorization for allotment. \
             CA 2006 s.551 requires authorization by ordinary resolution."
                .to_string()
        } else if expiry_date.is_none_or(|exp| exp <= current_date) {
            "Authorization has expired (max 5 years). Fresh authorization required.".to_string()
        } else {
            format!(
                "Authorization insufficient - max {} shares authorized, {} requested.",
                max_authorized.unwrap_or(0),
                shares_to_allot
            )
        };

        Self {
            has_authorization,
            authorization_date,
            expiry_date,
            max_shares_authorized: max_authorized,
            resolution_reference: None,
            analysis,
        }
    }
}

/// Pre-emption rights analysis (CA 2006 ss.561-567)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreEmptionAnalysis {
    /// Do pre-emption rights apply?
    pub rights_apply: bool,
    /// Have rights been disapplied?
    pub rights_disapplied: bool,
    /// Is for non-cash consideration (exempt)?
    pub non_cash_consideration: bool,
    /// Is bonus issue (exempt)?
    pub bonus_issue: bool,
    /// Have existing shareholders been offered shares?
    pub offer_made_to_existing: bool,
    /// Offer period (min 14 days)
    pub offer_period_compliant: bool,
    /// Analysis
    pub analysis: String,
}

impl PreEmptionAnalysis {
    /// Analyze pre-emption compliance
    pub fn analyze(
        cash_consideration: bool,
        bonus_issue: bool,
        rights_disapplied: bool,
        offered_to_existing: bool,
        offer_period_days: Option<u32>,
    ) -> Self {
        let non_cash = !cash_consideration;
        let rights_apply = cash_consideration && !bonus_issue && !rights_disapplied;
        let offer_period_ok = offer_period_days.is_some_and(|d| d >= 14);

        let compliant = !rights_apply || (offered_to_existing && offer_period_ok);

        let analysis = if non_cash {
            "Pre-emption rights do not apply - shares issued for non-cash consideration \
             (CA 2006 s.565)."
                .to_string()
        } else if bonus_issue {
            "Pre-emption rights do not apply - bonus issue (CA 2006 s.564).".to_string()
        } else if rights_disapplied {
            "Pre-emption rights disapplied by special resolution under CA 2006 s.570/571."
                .to_string()
        } else if compliant {
            format!(
                "Pre-emption rights satisfied. Offer made to existing shareholders for \
                 {} days (minimum 14 required under s.562).",
                offer_period_days.unwrap_or(0)
            )
        } else if !offered_to_existing {
            "Pre-emption rights BREACH - shares not offered to existing shareholders first \
             (CA 2006 s.561)."
                .to_string()
        } else {
            format!(
                "Pre-emption rights BREACH - offer period {} days insufficient (minimum 14).",
                offer_period_days.unwrap_or(0)
            )
        };

        Self {
            rights_apply,
            rights_disapplied,
            non_cash_consideration: non_cash,
            bonus_issue,
            offer_made_to_existing: offered_to_existing,
            offer_period_compliant: offer_period_ok,
            analysis,
        }
    }
}

// ============================================================================
// Share Transfer
// ============================================================================

/// Share transfer analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareTransfer {
    /// Transferor
    pub transferor: String,
    /// Transferee
    pub transferee: String,
    /// Number of shares
    pub number_of_shares: u64,
    /// Share class
    pub share_class: String,
    /// Consideration paid
    pub consideration: f64,
    /// Transfer method
    pub transfer_method: TransferMethod,
    /// Transfer restrictions
    pub restrictions: TransferRestrictions,
    /// Is transfer valid?
    pub valid: bool,
    /// Analysis
    pub analysis: String,
}

/// Method of share transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferMethod {
    /// Stock transfer form (Stock Transfer Act 1963)
    StockTransferForm,
    /// CREST electronic settlement (for listed shares)
    Crest,
    /// Transmission on death
    TransmissionOnDeath,
    /// Court order
    CourtOrder,
}

/// Transfer restrictions in articles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRestrictions {
    /// Directors can refuse to register
    pub directors_discretion: bool,
    /// Pre-emption on transfer to other members first
    pub pre_emption_on_transfer: bool,
    /// Board approval required
    pub board_approval_required: bool,
    /// Tag-along rights
    pub tag_along: bool,
    /// Drag-along rights
    pub drag_along: bool,
    /// Lock-up period
    pub lock_up_period: Option<LockUpPeriod>,
}

/// Lock-up period restriction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockUpPeriod {
    /// Start date
    pub start: NaiveDate,
    /// End date
    pub end: NaiveDate,
    /// Restriction type
    pub restriction: LockUpRestriction,
}

/// Type of lock-up restriction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LockUpRestriction {
    /// Complete prohibition on transfer
    NoTransfer,
    /// Transfer only to permitted transferees
    PermittedTransfereesOnly,
    /// Board consent required
    BoardConsentRequired,
}

impl ShareTransfer {
    /// Analyze share transfer validity
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        transferor: &str,
        transferee: &str,
        shares: u64,
        share_class: &str,
        consideration: f64,
        method: TransferMethod,
        restrictions: TransferRestrictions,
        board_approved: bool,
        pre_emption_satisfied: bool,
        current_date: NaiveDate,
    ) -> Self {
        let lock_up_active = restrictions
            .lock_up_period
            .as_ref()
            .is_some_and(|p| current_date >= p.start && current_date <= p.end);

        let mut issues = Vec::new();

        if restrictions.directors_discretion && !board_approved {
            issues.push("Board approval required but not obtained");
        }
        if restrictions.pre_emption_on_transfer && !pre_emption_satisfied {
            issues.push("Pre-emption rights on transfer not satisfied");
        }
        if restrictions.board_approval_required && !board_approved {
            issues.push("Board approval specifically required");
        }
        if lock_up_active {
            issues.push("Transfer during lock-up period");
        }

        let valid = issues.is_empty();

        let analysis = if valid {
            format!(
                "Transfer of {} {} shares from {} to {} for £{:.2} is valid. \
                 Method: {:?}. All restrictions satisfied.",
                shares, share_class, transferor, transferee, consideration, method
            )
        } else {
            format!("Transfer INVALID. Issues: {}.", issues.join("; "))
        };

        Self {
            transferor: transferor.to_string(),
            transferee: transferee.to_string(),
            number_of_shares: shares,
            share_class: share_class.to_string(),
            consideration,
            transfer_method: method,
            restrictions,
            valid,
            analysis,
        }
    }
}

// ============================================================================
// Distributions (Dividends)
// ============================================================================

/// Dividend distribution analysis (CA 2006 Part 23)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionAnalysis {
    /// Company type
    pub company_type: CompanyType,
    /// Proposed distribution amount
    pub distribution_amount: f64,
    /// Accumulated realized profits
    pub realized_profits: f64,
    /// Accumulated realized losses
    pub realized_losses: f64,
    /// Net assets
    pub net_assets: f64,
    /// Called-up share capital
    pub called_up_capital: f64,
    /// Undistributable reserves
    pub undistributable_reserves: f64,
    /// Is distribution lawful?
    pub lawful: bool,
    /// Maximum distributable amount
    pub max_distributable: f64,
    /// Analysis
    pub analysis: String,
}

impl DistributionAnalysis {
    /// Analyze distribution legality
    pub fn analyze(
        company_type: CompanyType,
        distribution_amount: f64,
        realized_profits: f64,
        realized_losses: f64,
        net_assets: f64,
        called_up_capital: f64,
        undistributable_reserves: f64,
    ) -> Self {
        // Basic distributable profits (s.830)
        let distributable = realized_profits - realized_losses;
        let distributable = if distributable > 0.0 {
            distributable
        } else {
            0.0
        };

        // Public company additional test (s.831)
        let max_distributable = match company_type {
            CompanyType::PublicLimitedCompany => {
                let capital_threshold = called_up_capital + undistributable_reserves;
                let headroom = net_assets - capital_threshold;
                let headroom = if headroom > 0.0 { headroom } else { 0.0 };
                distributable.min(headroom)
            }
            _ => distributable,
        };

        let lawful = distribution_amount <= max_distributable;

        let analysis = match company_type {
            CompanyType::PublicLimitedCompany => {
                if lawful {
                    format!(
                        "Distribution of £{:.2} is LAWFUL. Distributable profits: £{:.2} \
                         (realized profits £{:.2} less losses £{:.2}). Capital maintenance \
                         test passed (net assets £{:.2} exceed threshold £{:.2}).",
                        distribution_amount,
                        distributable,
                        realized_profits,
                        realized_losses,
                        net_assets,
                        called_up_capital + undistributable_reserves
                    )
                } else {
                    format!(
                        "Distribution of £{:.2} is UNLAWFUL. Maximum distributable: £{:.2}. \
                         CA 2006 s.830 (distributable profits) and s.831 (capital maintenance) \
                         tests must both be satisfied.",
                        distribution_amount, max_distributable
                    )
                }
            }
            _ => {
                if lawful {
                    format!(
                        "Distribution of £{:.2} is LAWFUL. Distributable profits: £{:.2} \
                         (CA 2006 s.830).",
                        distribution_amount, max_distributable
                    )
                } else {
                    format!(
                        "Distribution of £{:.2} is UNLAWFUL. Exceeds distributable profits \
                         of £{:.2} (CA 2006 s.830). Directors may be personally liable (s.847).",
                        distribution_amount, max_distributable
                    )
                }
            }
        };

        Self {
            company_type,
            distribution_amount,
            realized_profits,
            realized_losses,
            net_assets,
            called_up_capital,
            undistributable_reserves,
            lawful,
            max_distributable,
            analysis,
        }
    }
}

// ============================================================================
// Share Buyback
// ============================================================================

/// Share buyback analysis (CA 2006 ss.690-708)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareBuybackAnalysis {
    /// Company type
    pub company_type: CompanyType,
    /// Number of shares to buy back
    pub number_of_shares: u64,
    /// Purchase price
    pub purchase_price: f64,
    /// Are shares fully paid?
    pub shares_fully_paid: bool,
    /// Authorized by articles?
    pub authorized_by_articles: bool,
    /// Source of funds
    pub funding_source: BuybackFunding,
    /// Will company have shares remaining?
    pub shares_remaining_after: bool,
    /// Solvency statement (private company from capital)
    pub solvency_statement: Option<SolvencyStatement>,
    /// Is buyback lawful?
    pub lawful: bool,
    /// Analysis
    pub analysis: String,
}

/// Source of funding for buyback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuybackFunding {
    /// From distributable profits
    DistributableProfits,
    /// From proceeds of fresh issue
    FreshIssue,
    /// From capital (private companies only with solvency statement)
    Capital,
}

/// Solvency statement for buyback from capital (s.714)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolvencyStatement {
    /// Date of statement
    pub date: NaiveDate,
    /// Directors making statement
    pub directors: Vec<String>,
    /// Can pay debts immediately due?
    pub can_pay_current_debts: bool,
    /// Will be able to pay debts for 12 months?
    pub can_pay_future_debts: bool,
    /// Grounds for opinion
    pub grounds: String,
    /// Is statement valid?
    pub valid: bool,
}

impl ShareBuybackAnalysis {
    /// Analyze share buyback legality
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        company_type: CompanyType,
        shares: u64,
        price: f64,
        fully_paid: bool,
        authorized: bool,
        funding: BuybackFunding,
        shares_remaining: bool,
        solvency_statement: Option<SolvencyStatement>,
    ) -> Self {
        let mut issues = Vec::new();

        if !fully_paid {
            issues.push("Shares not fully paid (CA 2006 s.691)");
        }
        if !authorized {
            issues.push("Not authorized by articles");
        }
        if !shares_remaining {
            issues.push("No shares would remain after buyback");
        }

        // Check funding source rules
        if funding == BuybackFunding::Capital {
            if company_type == CompanyType::PublicLimitedCompany {
                issues.push("Public companies cannot buy back from capital");
            } else if solvency_statement.as_ref().is_none_or(|s| !s.valid) {
                issues.push("Capital buyback requires valid solvency statement (s.714)");
            }
        }

        let lawful = issues.is_empty();

        let analysis = if lawful {
            format!(
                "Buyback of {} shares for £{:.2} is LAWFUL. Funding: {:?}. \
                 Requirements: shares fully paid, authorized by articles, \
                 shares will remain in issue.",
                shares, price, funding
            )
        } else {
            format!("Buyback UNLAWFUL. Issues: {}.", issues.join("; "))
        };

        Self {
            company_type,
            number_of_shares: shares,
            purchase_price: price,
            shares_fully_paid: fully_paid,
            authorized_by_articles: authorized,
            funding_source: funding,
            shares_remaining_after: shares_remaining,
            solvency_statement,
            lawful,
            analysis,
        }
    }
}

// ============================================================================
// Reduction of Capital
// ============================================================================

/// Capital reduction analysis (CA 2006 ss.641-657)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapitalReductionAnalysis {
    /// Company type
    pub company_type: CompanyType,
    /// Method of reduction
    pub method: CapitalReductionMethod,
    /// Amount of reduction
    pub reduction_amount: f64,
    /// Confirmation procedure used
    pub confirmation_procedure: ConfirmationProcedure,
    /// Analysis
    pub analysis: String,
}

/// Method of capital reduction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapitalReductionMethod {
    /// Cancel shares
    CancelShares,
    /// Reduce nominal value
    ReduceNominalValue,
    /// Return capital to shareholders
    ReturnCapital,
    /// Write off losses
    WriteOffLosses,
}

/// Procedure for confirming capital reduction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfirmationProcedure {
    /// Court confirmation (public and private)
    CourtConfirmation,
    /// Solvency statement (private company only)
    SolvencyStatement,
}

impl CapitalReductionAnalysis {
    /// Analyze capital reduction
    pub fn analyze(
        company_type: CompanyType,
        method: CapitalReductionMethod,
        amount: f64,
        procedure: ConfirmationProcedure,
    ) -> Self {
        let procedure_valid = !matches!(
            (company_type, procedure),
            (
                CompanyType::PublicLimitedCompany,
                ConfirmationProcedure::SolvencyStatement
            )
        );

        let analysis = if !procedure_valid {
            "Capital reduction INVALID. Public companies must use court confirmation \
             (CA 2006 s.645). Cannot use solvency statement procedure."
                .to_string()
        } else {
            match procedure {
                ConfirmationProcedure::SolvencyStatement => {
                    format!(
                        "Capital reduction of £{:.2} using {:?} method. Private company using \
                         solvency statement procedure (CA 2006 s.641(1)(a)). Requires special \
                         resolution and directors' solvency statement.",
                        amount, method
                    )
                }
                ConfirmationProcedure::CourtConfirmation => {
                    format!(
                        "Capital reduction of £{:.2} using {:?} method. Court confirmation \
                         procedure (CA 2006 s.645). Requires special resolution and court \
                         order. Creditors may object.",
                        amount, method
                    )
                }
            }
        };

        Self {
            company_type,
            method,
            reduction_amount: amount,
            confirmation_procedure: procedure,
            analysis,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_emption_cash_issue() {
        let analysis = PreEmptionAnalysis::analyze(true, false, false, true, Some(21));
        assert!(analysis.rights_apply);
        assert!(analysis.offer_period_compliant);
        assert!(analysis.analysis.contains("satisfied"));
    }

    #[test]
    fn test_pre_emption_non_cash() {
        let analysis = PreEmptionAnalysis::analyze(false, false, false, false, None);
        assert!(!analysis.rights_apply);
        assert!(analysis.analysis.contains("non-cash"));
    }

    #[test]
    fn test_pre_emption_breach() {
        let analysis = PreEmptionAnalysis::analyze(true, false, false, false, None);
        assert!(analysis.rights_apply);
        assert!(analysis.analysis.contains("BREACH"));
    }

    #[test]
    fn test_distribution_private_company() {
        let analysis = DistributionAnalysis::analyze(
            CompanyType::PrivateLimitedByShares,
            50_000.0,
            100_000.0,
            20_000.0,
            200_000.0,
            50_000.0,
            10_000.0,
        );
        assert!(analysis.lawful);
        assert_eq!(analysis.max_distributable, 80_000.0);
    }

    #[test]
    fn test_distribution_exceeds_profits() {
        let analysis = DistributionAnalysis::analyze(
            CompanyType::PrivateLimitedByShares,
            100_000.0,
            50_000.0,
            10_000.0,
            200_000.0,
            50_000.0,
            0.0,
        );
        assert!(!analysis.lawful);
        assert_eq!(analysis.max_distributable, 40_000.0);
    }

    #[test]
    fn test_buyback_lawful() {
        let analysis = ShareBuybackAnalysis::analyze(
            CompanyType::PrivateLimitedByShares,
            1000,
            10_000.0,
            true,
            true,
            BuybackFunding::DistributableProfits,
            true,
            None,
        );
        assert!(analysis.lawful);
    }

    #[test]
    fn test_buyback_not_fully_paid() {
        let analysis = ShareBuybackAnalysis::analyze(
            CompanyType::PrivateLimitedByShares,
            1000,
            10_000.0,
            false,
            true,
            BuybackFunding::DistributableProfits,
            true,
            None,
        );
        assert!(!analysis.lawful);
        assert!(analysis.analysis.contains("fully paid"));
    }

    #[test]
    fn test_capital_reduction_private_solvency() {
        let analysis = CapitalReductionAnalysis::analyze(
            CompanyType::PrivateLimitedByShares,
            CapitalReductionMethod::WriteOffLosses,
            50_000.0,
            ConfirmationProcedure::SolvencyStatement,
        );
        assert!(analysis.analysis.contains("solvency statement"));
    }

    #[test]
    fn test_capital_reduction_public_must_use_court() {
        let analysis = CapitalReductionAnalysis::analyze(
            CompanyType::PublicLimitedCompany,
            CapitalReductionMethod::ReturnCapital,
            100_000.0,
            ConfirmationProcedure::SolvencyStatement,
        );
        assert!(analysis.analysis.contains("INVALID"));
    }
}
