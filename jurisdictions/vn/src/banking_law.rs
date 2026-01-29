//! Vietnamese Law on Credit Institutions (Luật các Tổ chức tín dụng)
//!
//! Law No. 47/2010/QH12, amended by Laws 17/2017 and 45/2024.
//! Effective dates: Original (January 1, 2011), 2017 Amendment (January 15, 2018).
//!
//! ## Types of Credit Institutions
//!
//! - Commercial banks (Ngân hàng thương mại)
//! - Policy banks (Ngân hàng chính sách)
//! - Cooperative banks (Ngân hàng hợp tác xã)
//! - Microfinance institutions (Tổ chức tài chính vi mô)
//! - Foreign bank branches (Chi nhánh ngân hàng nước ngoài)
//!
//! ## Key Requirements
//!
//! - Minimum charter capital (Vốn điều lệ tối thiểu)
//! - Capital adequacy ratio (CAR) ≥ 9% (Tỷ lệ an toàn vốn)
//! - Reserve requirements (Dự trữ bắt buộc)
//! - Lending limits (Hạn mức tín dụng)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Types of credit institutions (Loại hình tổ chức tín dụng) - Article 21-26
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditInstitutionType {
    /// Commercial bank (Ngân hàng thương mại)
    CommercialBank,
    /// Policy bank (Ngân hàng chính sách)
    PolicyBank,
    /// Cooperative bank (Ngân hàng hợp tác xã)
    CooperativeBank,
    /// Finance company (Công ty tài chính)
    FinanceCompany,
    /// Financial leasing company (Công ty cho thuê tài chính)
    LeasingCompany,
    /// Microfinance institution (Tổ chức tài chính vi mô)
    Microfinance,
    /// Foreign bank branch (Chi nhánh ngân hàng nước ngoài)
    ForeignBankBranch,
    /// Representative office (Văn phòng đại diện)
    RepresentativeOffice,
}

impl CreditInstitutionType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::CommercialBank => "Ngân hàng thương mại",
            Self::PolicyBank => "Ngân hàng chính sách",
            Self::CooperativeBank => "Ngân hàng hợp tác xã",
            Self::FinanceCompany => "Công ty tài chính",
            Self::LeasingCompany => "Công ty cho thuê tài chính",
            Self::Microfinance => "Tổ chức tài chính vi mô",
            Self::ForeignBankBranch => "Chi nhánh ngân hàng nước ngoài",
            Self::RepresentativeOffice => "Văn phòng đại diện ngân hàng nước ngoài",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CommercialBank => "Commercial bank",
            Self::PolicyBank => "Policy bank",
            Self::CooperativeBank => "Cooperative bank",
            Self::FinanceCompany => "Finance company",
            Self::LeasingCompany => "Financial leasing company",
            Self::Microfinance => "Microfinance institution",
            Self::ForeignBankBranch => "Foreign bank branch",
            Self::RepresentativeOffice => "Representative office",
        }
    }

    /// Get minimum charter capital in VND (Article 15)
    pub fn minimum_charter_capital(&self) -> Option<i64> {
        match self {
            Self::CommercialBank => Some(3_000_000_000_000), // 3 trillion VND
            Self::PolicyBank => None,                        // State-owned, different rules
            Self::CooperativeBank => Some(500_000_000_000),  // 500 billion VND
            Self::FinanceCompany => Some(500_000_000_000),   // 500 billion VND
            Self::LeasingCompany => Some(150_000_000_000),   // 150 billion VND
            Self::Microfinance => Some(5_000_000_000),       // 5 billion VND
            Self::ForeignBankBranch => Some(15_000_000),     // 15 million USD equivalent
            Self::RepresentativeOffice => None,              // No lending, no capital requirement
        }
    }

    /// Check if can accept deposits
    pub fn can_accept_deposits(&self) -> bool {
        matches!(
            self,
            Self::CommercialBank | Self::PolicyBank | Self::CooperativeBank
        )
    }

    /// Check if can provide loans
    pub fn can_provide_loans(&self) -> bool {
        !matches!(self, Self::RepresentativeOffice)
    }
}

/// Capital adequacy ratio (CAR) requirements (Tỷ lệ an toàn vốn) - Article 127
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CapitalAdequacyRatio {
    /// Tier 1 capital (Vốn tự có cấp 1)
    pub tier1_capital: i64,
    /// Tier 2 capital (Vốn tự có cấp 2)
    pub tier2_capital: i64,
    /// Risk-weighted assets (Tài sản có rủi ro)
    pub risk_weighted_assets: i64,
}

impl CapitalAdequacyRatio {
    /// Minimum CAR requirement: 9% (changed from 8% in 2017 amendment)
    pub const MIN_CAR: f64 = 9.0;

    /// Calculate CAR as percentage
    pub fn calculate_car(&self) -> f64 {
        if self.risk_weighted_assets == 0 {
            return 0.0;
        }

        let total_capital = self.tier1_capital + self.tier2_capital;
        (total_capital as f64 / self.risk_weighted_assets as f64) * 100.0
    }

    /// Check if meets minimum CAR requirement
    pub fn meets_requirement(&self) -> bool {
        self.calculate_car() >= Self::MIN_CAR
    }
}

/// Reserve requirement (Dự trữ bắt buộc) - Article 131
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReserveRequirement {
    /// Total deposits (Tổng tiền gửi)
    pub total_deposits: i64,
    /// Reserve ratio percentage (Tỷ lệ dự trữ bắt buộc)
    pub reserve_ratio_percent: u8,
}

impl ReserveRequirement {
    /// Standard reserve requirement: varies by deposit type
    /// VND deposits: typically 3-5%
    /// Foreign currency deposits: typically 6-8%
    pub const STANDARD_VND: u8 = 5;
    pub const STANDARD_FX: u8 = 8;

    /// Calculate required reserve amount
    pub fn calculate_required_reserve(&self) -> i64 {
        (self.total_deposits as f64 * self.reserve_ratio_percent as f64 / 100.0) as i64
    }

    /// Check if reserve ratio is within legal range
    pub fn is_valid(&self) -> bool {
        self.reserve_ratio_percent <= 20 // Max 20% per regulation
    }
}

/// Lending limits (Hạn mức tín dụng) - Article 128-130
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LendingLimit;

impl LendingLimit {
    /// Maximum lending to one customer: 15% of equity capital
    pub const MAX_SINGLE_BORROWER: f64 = 15.0;

    /// Maximum lending to related parties: 25% of equity capital
    pub const MAX_RELATED_PARTIES: f64 = 25.0;

    /// Maximum total lending to related parties: 50% of equity capital
    pub const MAX_TOTAL_RELATED: f64 = 50.0;

    /// Check if single loan complies with limit
    pub fn check_single_loan(loan_amount: i64, equity_capital: i64) -> bool {
        let ratio = (loan_amount as f64 / equity_capital as f64) * 100.0;
        ratio <= Self::MAX_SINGLE_BORROWER
    }

    /// Check if related party lending complies with limit
    pub fn check_related_party_lending(related_party_loans: i64, equity_capital: i64) -> bool {
        let ratio = (related_party_loans as f64 / equity_capital as f64) * 100.0;
        ratio <= Self::MAX_TOTAL_RELATED
    }
}

/// Banking activities (Hoạt động ngân hàng) - Article 4
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankingActivity {
    /// Accepting deposits (Nhận tiền gửi)
    AcceptingDeposits,
    /// Providing loans (Cấp tín dụng)
    ProvidingLoans,
    /// Payment services (Dịch vụ thanh toán)
    PaymentServices,
    /// Foreign exchange (Ngoại hối)
    ForeignExchange,
    /// Investment in securities (Đầu tư chứng khoán)
    SecuritiesInvestment,
    /// Trust services (Dịch vụ ủy thác)
    TrustServices,
    /// Safe deposit box (Két an toàn)
    SafeDepositBox,
    /// Financial advisory (Tư vấn tài chính)
    FinancialAdvisory,
    /// Other banking services
    Other(String),
}

impl BankingActivity {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::AcceptingDeposits => "Nhận tiền gửi của tổ chức, cá nhân".to_string(),
            Self::ProvidingLoans => "Cấp tín dụng cho tổ chức, cá nhân".to_string(),
            Self::PaymentServices => "Cung ứng dịch vụ thanh toán".to_string(),
            Self::ForeignExchange => "Kinh doanh ngoại hối".to_string(),
            Self::SecuritiesInvestment => "Đầu tư, kinh doanh chứng khoán".to_string(),
            Self::TrustServices => "Dịch vụ ủy thác, đại lý".to_string(),
            Self::SafeDepositBox => "Dịch vụ két an toàn".to_string(),
            Self::FinancialAdvisory => "Tư vấn tài chính".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }

    /// Check if requires banking license
    pub fn requires_banking_license(&self) -> bool {
        matches!(
            self,
            Self::AcceptingDeposits | Self::ProvidingLoans | Self::PaymentServices
        )
    }
}

/// Non-performing loan (NPL) classification (Phân loại nợ) - Circular 11/2021
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoanClassification {
    /// Current (Nợ đủ tiêu chuẩn) - Group 1
    Current,
    /// Special mention (Nợ cần chú ý) - Group 2
    SpecialMention,
    /// Substandard (Nợ dưới tiêu chuẩn) - Group 3
    Substandard,
    /// Doubtful (Nợ nghi ngờ) - Group 4
    Doubtful,
    /// Loss (Nợ có khả năng mất vốn) - Group 5
    Loss,
}

impl LoanClassification {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::Current => "Nhóm 1 - Nợ đủ tiêu chuẩn",
            Self::SpecialMention => "Nhóm 2 - Nợ cần chú ý",
            Self::Substandard => "Nhóm 3 - Nợ dưới tiêu chuẩn",
            Self::Doubtful => "Nhóm 4 - Nợ nghi ngờ",
            Self::Loss => "Nhóm 5 - Nợ có khả năng mất vốn",
        }
    }

    /// Check if NPL (non-performing loan)
    pub fn is_npl(&self) -> bool {
        matches!(self, Self::Substandard | Self::Doubtful | Self::Loss)
    }

    /// Get minimum provisioning rate (%)
    pub fn minimum_provision_rate(&self) -> f64 {
        match self {
            Self::Current => 0.0,
            Self::SpecialMention => 5.0,
            Self::Substandard => 20.0,
            Self::Doubtful => 50.0,
            Self::Loss => 100.0,
        }
    }
}

/// Result type for banking law operations
pub type BankingResult<T> = Result<T, BankingError>;

/// Errors related to Banking Law
#[derive(Debug, Error)]
pub enum BankingError {
    /// Insufficient charter capital
    #[error("Vốn điều lệ không đủ (Điều 15): {actual} < {required} VND")]
    InsufficientCapital { actual: i64, required: i64 },

    /// CAR below requirement
    #[error("Tỷ lệ an toàn vốn không đạt (Điều 127): {car:.2}% < {min}%")]
    InsufficientCar { car: f64, min: f64 },

    /// Lending limit exceeded
    #[error("Vượt quá hạn mức tín dụng (Điều 128): {actual:.2}% > {limit}%")]
    LendingLimitExceeded { actual: f64, limit: f64 },

    /// Unauthorized banking activity
    #[error("Hoạt động ngân hàng không được phép (Điều 4): {activity}")]
    UnauthorizedActivity { activity: String },

    /// Other banking law violation
    #[error("Vi phạm Luật các Tổ chức tín dụng: {reason}")]
    BankingViolation { reason: String },
}

/// Validate charter capital
pub fn validate_charter_capital(
    institution_type: &CreditInstitutionType,
    actual_capital: i64,
) -> BankingResult<()> {
    if let Some(required) = institution_type.minimum_charter_capital()
        && actual_capital < required
    {
        return Err(BankingError::InsufficientCapital {
            actual: actual_capital,
            required,
        });
    }
    Ok(())
}

/// Validate capital adequacy ratio
pub fn validate_car(car: &CapitalAdequacyRatio) -> BankingResult<()> {
    if !car.meets_requirement() {
        Err(BankingError::InsufficientCar {
            car: car.calculate_car(),
            min: CapitalAdequacyRatio::MIN_CAR,
        })
    } else {
        Ok(())
    }
}

/// Validate lending limit
pub fn validate_lending_limit(loan_amount: i64, equity_capital: i64) -> BankingResult<()> {
    if !LendingLimit::check_single_loan(loan_amount, equity_capital) {
        let ratio = (loan_amount as f64 / equity_capital as f64) * 100.0;
        Err(BankingError::LendingLimitExceeded {
            actual: ratio,
            limit: LendingLimit::MAX_SINGLE_BORROWER,
        })
    } else {
        Ok(())
    }
}

/// Get Banking Law checklist
pub fn get_banking_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Vốn điều lệ tối thiểu",
            "Minimum charter capital",
            "Điều 15",
        ),
        ("Giấy phép thành lập", "Establishment license", "Điều 51-60"),
        (
            "Tỷ lệ an toàn vốn (CAR)",
            "Capital adequacy ratio",
            "Điều 127",
        ),
        ("Dự trữ bắt buộc", "Reserve requirements", "Điều 131"),
        ("Hạn mức tín dụng", "Lending limits", "Điều 128-130"),
        (
            "Phân loại nợ và trích lập dự phòng",
            "Loan classification and provisioning",
            "Circular 11/2021",
        ),
        ("Báo cáo giám sát", "Supervisory reporting", "Điều 145-146"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_institution_types() {
        let commercial = CreditInstitutionType::CommercialBank;
        assert_eq!(
            commercial.minimum_charter_capital(),
            Some(3_000_000_000_000)
        );
        assert!(commercial.can_accept_deposits());
        assert!(commercial.can_provide_loans());

        let microfinance = CreditInstitutionType::Microfinance;
        assert_eq!(microfinance.minimum_charter_capital(), Some(5_000_000_000));
        assert!(!microfinance.can_accept_deposits());
        assert!(microfinance.can_provide_loans());

        let rep_office = CreditInstitutionType::RepresentativeOffice;
        assert_eq!(rep_office.minimum_charter_capital(), None);
        assert!(!rep_office.can_provide_loans());
    }

    #[test]
    fn test_capital_adequacy_ratio() {
        let good_car = CapitalAdequacyRatio {
            tier1_capital: 1_000_000_000,
            tier2_capital: 500_000_000,
            risk_weighted_assets: 10_000_000_000,
        };

        assert_eq!(good_car.calculate_car(), 15.0); // (1000M + 500M) / 10000M * 100 = 15%
        assert!(good_car.meets_requirement());

        let poor_car = CapitalAdequacyRatio {
            tier1_capital: 500_000_000,
            tier2_capital: 200_000_000,
            risk_weighted_assets: 10_000_000_000,
        };

        assert!((poor_car.calculate_car() - 7.0).abs() < 0.01); // Below 9%
        assert!(!poor_car.meets_requirement());
    }

    #[test]
    fn test_reserve_requirement() {
        let reserve = ReserveRequirement {
            total_deposits: 100_000_000_000, // 100 billion
            reserve_ratio_percent: 5,
        };

        assert_eq!(reserve.calculate_required_reserve(), 5_000_000_000); // 5 billion
        assert!(reserve.is_valid());

        let invalid = ReserveRequirement {
            total_deposits: 100_000_000_000,
            reserve_ratio_percent: 25, // Exceeds max
        };

        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_lending_limits() {
        let equity = 10_000_000_000; // 10 billion VND

        // Valid single loan (10% of equity)
        assert!(LendingLimit::check_single_loan(1_000_000_000, equity));

        // Invalid single loan (20% of equity - exceeds 15% limit)
        assert!(!LendingLimit::check_single_loan(2_000_000_000, equity));

        // Valid related party lending (40% of equity)
        assert!(LendingLimit::check_related_party_lending(
            4_000_000_000,
            equity
        ));

        // Invalid related party lending (60% of equity - exceeds 50% limit)
        assert!(!LendingLimit::check_related_party_lending(
            6_000_000_000,
            equity
        ));
    }

    #[test]
    fn test_loan_classification() {
        assert!(!LoanClassification::Current.is_npl());
        assert!(!LoanClassification::SpecialMention.is_npl());
        assert!(LoanClassification::Substandard.is_npl());
        assert!(LoanClassification::Doubtful.is_npl());
        assert!(LoanClassification::Loss.is_npl());

        assert_eq!(LoanClassification::Current.minimum_provision_rate(), 0.0);
        assert_eq!(
            LoanClassification::SpecialMention.minimum_provision_rate(),
            5.0
        );
        assert_eq!(
            LoanClassification::Substandard.minimum_provision_rate(),
            20.0
        );
        assert_eq!(LoanClassification::Loss.minimum_provision_rate(), 100.0);
    }

    #[test]
    fn test_banking_activities() {
        let deposits = BankingActivity::AcceptingDeposits;
        assert!(deposits.requires_banking_license());

        let advisory = BankingActivity::FinancialAdvisory;
        assert!(!advisory.requires_banking_license());
    }

    #[test]
    fn test_validation() {
        // Valid charter capital
        let commercial = CreditInstitutionType::CommercialBank;
        assert!(validate_charter_capital(&commercial, 5_000_000_000_000).is_ok());
        assert!(validate_charter_capital(&commercial, 2_000_000_000_000).is_err());

        // Valid CAR
        let good_car = CapitalAdequacyRatio {
            tier1_capital: 1_000_000_000,
            tier2_capital: 500_000_000,
            risk_weighted_assets: 10_000_000_000,
        };
        assert!(validate_car(&good_car).is_ok());

        // Invalid CAR
        let poor_car = CapitalAdequacyRatio {
            tier1_capital: 500_000_000,
            tier2_capital: 0,
            risk_weighted_assets: 10_000_000_000,
        };
        assert!(validate_car(&poor_car).is_err());

        // Valid lending limit
        assert!(validate_lending_limit(1_000_000_000, 10_000_000_000).is_ok());

        // Invalid lending limit
        assert!(validate_lending_limit(2_000_000_000, 10_000_000_000).is_err());
    }

    #[test]
    fn test_banking_checklist() {
        let checklist = get_banking_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 7);
    }
}
