//! Banking Law Types (ປະເພດກົດໝາຍທະນາຄານ)
//!
//! Comprehensive type definitions for Lao banking and financial services law.
//!
//! ## Legal Basis
//!
//! - **Commercial Bank Law 2006** (Law No. 03/NA, amended 2018)
//! - **Bank of Lao PDR Law 2018** (Law No. 50/NA)
//! - **AML/CFT Law 2014** (Law No. 50/NA)
//! - **Payment Systems Decree**
//!
//! ## Banking System Overview
//!
//! The Bank of Lao PDR (BOL) serves as the central bank and primary regulator of
//! the banking sector. Commercial banks must obtain licenses from BOL and comply
//! with capital adequacy, prudential regulations, and AML/CFT requirements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants (ຄ່າຄົງທີ່)
// ============================================================================

/// Minimum capital for commercial banks (300 billion LAK)
/// ທຶນຂັ້ນຕ່ຳສຳລັບທະນາຄານການຄ້າ
pub const MIN_CAPITAL_COMMERCIAL_BANK_LAK: u64 = 300_000_000_000;

/// Minimum capital for foreign bank branches (50 billion LAK)
/// ທຶນຂັ້ນຕ່ຳສຳລັບສາຂາທະນາຄານຕ່າງປະເທດ
pub const MIN_CAPITAL_FOREIGN_BRANCH_LAK: u64 = 50_000_000_000;

/// Minimum capital for deposit-taking MFIs (10 billion LAK)
/// ທຶນຂັ້ນຕ່ຳສຳລັບ MFI ທີ່ຮັບເງິນຝາກ
pub const MIN_CAPITAL_MFI_DEPOSIT_LAK: u64 = 10_000_000_000;

/// Minimum capital for non-deposit MFIs (500 million LAK)
/// ທຶນຂັ້ນຕ່ຳສຳລັບ MFI ທີ່ບໍ່ຮັບເງິນຝາກ
pub const MIN_CAPITAL_MFI_NON_DEPOSIT_LAK: u64 = 500_000_000;

/// Minimum capital adequacy ratio (Basel III: 8%)
/// ອັດຕາສ່ວນຄວາມພຽງພໍຂອງທຶນຂັ້ນຕ່ຳ
pub const MIN_CAPITAL_ADEQUACY_RATIO_PERCENT: f64 = 8.0;

/// Minimum Tier 1 capital ratio (6%)
/// ອັດຕາສ່ວນທຶນຂັ້ນ 1 ຂັ້ນຕ່ຳ
pub const MIN_TIER1_CAPITAL_RATIO_PERCENT: f64 = 6.0;

/// Minimum Common Equity Tier 1 ratio (4.5%)
/// ອັດຕາສ່ວນທຶນຫຸ້ນສ່ວນທົ່ວໄປຂັ້ນ 1 ຂັ້ນຕ່ຳ
pub const MIN_CET1_RATIO_PERCENT: f64 = 4.5;

/// Minimum leverage ratio (3%)
/// ອັດຕາສ່ວນໜີ້ສິນຂັ້ນຕ່ຳ
pub const MIN_LEVERAGE_RATIO_PERCENT: f64 = 3.0;

/// Single borrower limit (25% of capital)
/// ຂີດຈຳກັດຜູ້ກູ້ຢືມລາຍດຽວ
pub const SINGLE_BORROWER_LIMIT_PERCENT: f64 = 25.0;

/// Related party lending limit (15% of capital)
/// ຂີດຈຳກັດການໃຫ້ກູ້ຢືມບຸກຄົນກ່ຽວຂ້ອງ
pub const RELATED_PARTY_LIMIT_PERCENT: f64 = 15.0;

/// Minimum liquidity coverage ratio (100%)
/// ອັດຕາສ່ວນຄຸ້ມຄອງສະພາບຄ່ອງຂັ້ນຕ່ຳ
pub const MIN_LCR_PERCENT: f64 = 100.0;

/// Minimum net stable funding ratio (100%)
/// ອັດຕາສ່ວນແຫຼ່ງທຶນໝັ້ນຄົງສຸດທິຂັ້ນຕ່ຳ
pub const MIN_NSFR_PERCENT: f64 = 100.0;

/// Deposit insurance coverage limit (50 million LAK per depositor)
/// ຂີດຈຳກັດຄຸ້ມຄອງປະກັນເງິນຝາກ
pub const DEPOSIT_INSURANCE_LIMIT_LAK: u64 = 50_000_000;

/// AML record keeping requirement (5 years)
/// ເງື່ອນໄຂການເກັບຮັກສາບັນທຶກ AML
pub const AML_RECORD_KEEPING_YEARS: u32 = 5;

/// STR reporting deadline (24 hours)
/// ກຳນົດລາຍງານ STR
pub const STR_REPORTING_DEADLINE_HOURS: u32 = 24;

/// Banking license validity period (5 years)
/// ໄລຍະເວລາໃບອະນຸຍາດທະນາຄານ
pub const LICENSE_VALIDITY_YEARS: u32 = 5;

/// Maximum lending rate (reference rate + spread, approximately 15-18%)
/// ອັດຕາດອກເບ້ຍກູ້ຢືມສູງສຸດ
pub const MAX_LENDING_RATE_PERCENT: f64 = 18.0;

/// Reserve requirement ratio (5%)
/// ອັດຕາສ່ວນເງິນສຳຮອງ
pub const RESERVE_REQUIREMENT_PERCENT: f64 = 5.0;

// ============================================================================
// Bank Types (ປະເພດທະນາຄານ)
// ============================================================================

/// Types of banks operating in Lao PDR
/// ປະເພດທະນາຄານໃນລາວ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BankType {
    /// State-owned bank (e.g., BCEL, LDB)
    /// ທະນາຄານລັດ
    StateOwned,

    /// Joint venture bank
    /// ທະນາຄານຮ່ວມທຶນ
    JointVenture {
        /// Foreign partner country
        /// ປະເທດຄູ່ຮ່ວມທຶນ
        partner_country: String,
        /// Foreign ownership percentage
        /// ເປີເຊັນຖືຫຸ້ນຕ່າງປະເທດ
        foreign_ownership_percent: f64,
    },

    /// Private domestic bank
    /// ທະນາຄານເອກະຊົນພາຍໃນ
    PrivateDomestic,

    /// Foreign bank branch
    /// ສາຂາທະນາຄານຕ່າງປະເທດ
    ForeignBranch {
        /// Parent bank country
        /// ປະເທດທະນາຄານແມ່
        parent_country: String,
        /// Parent bank name
        /// ຊື່ທະນາຄານແມ່
        parent_bank_name: String,
    },

    /// Subsidiary of foreign bank
    /// ບໍລິສັດລູກຂອງທະນາຄານຕ່າງປະເທດ
    ForeignSubsidiary {
        /// Parent bank country
        /// ປະເທດທະນາຄານແມ່
        parent_country: String,
    },
}

impl BankType {
    /// Get minimum capital requirement for this bank type
    /// ຮັບທຶນຂັ້ນຕ່ຳສຳລັບປະເພດທະນາຄານນີ້
    pub fn minimum_capital_lak(&self) -> u64 {
        match self {
            BankType::ForeignBranch { .. } => MIN_CAPITAL_FOREIGN_BRANCH_LAK,
            _ => MIN_CAPITAL_COMMERCIAL_BANK_LAK,
        }
    }

    /// Get description in Lao
    /// ຮັບຄຳອະທິບາຍເປັນພາສາລາວ
    pub fn description_lao(&self) -> &'static str {
        match self {
            BankType::StateOwned => "ທະນາຄານລັດ",
            BankType::JointVenture { .. } => "ທະນາຄານຮ່ວມທຶນ",
            BankType::PrivateDomestic => "ທະນາຄານເອກະຊົນພາຍໃນ",
            BankType::ForeignBranch { .. } => "ສາຂາທະນາຄານຕ່າງປະເທດ",
            BankType::ForeignSubsidiary { .. } => "ບໍລິສັດລູກຂອງທະນາຄານຕ່າງປະເທດ",
        }
    }
}

/// Microfinance institution types
/// ປະເພດສະຖາບັນການເງິນຈຸລະພາກ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MicrofinanceType {
    /// Deposit-taking MFI
    /// ສະຖາບັນການເງິນຈຸລະພາກທີ່ຮັບເງິນຝາກ
    DepositTaking,

    /// Non-deposit MFI
    /// ສະຖາບັນການເງິນຈຸລະພາກທີ່ບໍ່ຮັບເງິນຝາກ
    NonDeposit,

    /// Credit cooperative
    /// ສະຫະກອນສິນເຊື່ອ
    CreditCooperative,

    /// Village fund
    /// ກອງທຶນບ້ານ
    VillageFund,
}

impl MicrofinanceType {
    /// Get minimum capital requirement
    /// ຮັບທຶນຂັ້ນຕ່ຳ
    pub fn minimum_capital_lak(&self) -> u64 {
        match self {
            MicrofinanceType::DepositTaking => MIN_CAPITAL_MFI_DEPOSIT_LAK,
            MicrofinanceType::NonDeposit => MIN_CAPITAL_MFI_NON_DEPOSIT_LAK,
            MicrofinanceType::CreditCooperative => MIN_CAPITAL_MFI_NON_DEPOSIT_LAK,
            MicrofinanceType::VillageFund => 0, // No formal capital requirement
        }
    }
}

// ============================================================================
// Banking License (ໃບອະນຸຍາດທະນາຄານ)
// ============================================================================

/// Banking license status
/// ສະຖານະໃບອະນຸຍາດທະນາຄານ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    /// Active license
    /// ໃບອະນຸຍາດມີຜົນ
    Active,

    /// License under review for renewal
    /// ກຳລັງພິຈາລະນາຕໍ່ອາຍຸ
    UnderReview,

    /// License suspended
    /// ໃບອະນຸຍາດຖືກລະງັບ
    Suspended {
        /// Suspension reason
        /// ເຫດຜົນລະງັບ
        reason: String,
        /// Suspension date
        /// ວັນທີລະງັບ
        suspended_at: DateTime<Utc>,
    },

    /// License revoked
    /// ໃບອະນຸຍາດຖືກຖອນ
    Revoked {
        /// Revocation reason
        /// ເຫດຜົນຖອນ
        reason: String,
        /// Revocation date
        /// ວັນທີຖອນ
        revoked_at: DateTime<Utc>,
    },

    /// License expired
    /// ໃບອະນຸຍາດໝົດອາຍຸ
    Expired,
}

/// Banking license
/// ໃບອະນຸຍາດທະນາຄານ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingLicense {
    /// License number
    /// ເລກທີໃບອະນຸຍາດ
    pub license_number: String,

    /// Bank name in Lao
    /// ຊື່ທະນາຄານເປັນພາສາລາວ
    pub bank_name_lao: String,

    /// Bank name in English
    /// ຊື່ທະນາຄານເປັນພາສາອັງກິດ
    pub bank_name_eng: String,

    /// Bank type
    /// ປະເພດທະນາຄານ
    pub bank_type: BankType,

    /// License status
    /// ສະຖານະໃບອະນຸຍາດ
    pub status: LicenseStatus,

    /// Issue date
    /// ວັນທີອອກ
    pub issued_at: DateTime<Utc>,

    /// Expiry date
    /// ວັນທີໝົດອາຍຸ
    pub expires_at: DateTime<Utc>,

    /// Licensed activities
    /// ກິດຈະກຳທີ່ໄດ້ຮັບອະນຸຍາດ
    pub licensed_activities: Vec<BankingActivity>,

    /// Registered capital
    /// ທຶນຈົດທະບຽນ
    pub registered_capital_lak: u64,

    /// Paid-up capital
    /// ທຶນຊຳລະແລ້ວ
    pub paid_up_capital_lak: u64,

    /// Head office address
    /// ທີ່ຢູ່ສຳນັກງານໃຫຍ່
    pub head_office_address: String,
}

impl BankingLicense {
    /// Check if license is valid
    /// ກວດສອບວ່າໃບອະນຸຍາດຖືກຕ້ອງ
    pub fn is_valid(&self) -> bool {
        matches!(self.status, LicenseStatus::Active) && Utc::now() < self.expires_at
    }

    /// Check if license allows specific activity
    /// ກວດສອບວ່າໃບອະນຸຍາດອະນຸຍາດກິດຈະກຳ
    pub fn allows_activity(&self, activity: &BankingActivity) -> bool {
        self.is_valid() && self.licensed_activities.contains(activity)
    }

    /// Days until expiry
    /// ຈຳນວນມື້ຈົນໝົດອາຍຸ
    pub fn days_until_expiry(&self) -> Option<i64> {
        if self.is_valid() {
            let duration = self.expires_at.signed_duration_since(Utc::now());
            Some(duration.num_days())
        } else {
            None
        }
    }
}

/// Banking activities that require licensing
/// ກິດຈະກຳທະນາຄານທີ່ຕ້ອງການໃບອະນຸຍາດ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankingActivity {
    /// Accept deposits
    /// ຮັບເງິນຝາກ
    AcceptDeposits,

    /// Make loans
    /// ໃຫ້ກູ້ຢືມ
    MakeLoans,

    /// Foreign exchange dealing
    /// ແລກປ່ຽນເງິນຕາ
    ForeignExchange,

    /// Trade finance
    /// ການເງິນການຄ້າ
    TradeFinance,

    /// Securities underwriting
    /// ຮັບປະກັນຫຼັກຊັບ
    SecuritiesUnderwriting,

    /// Asset management
    /// ການຄຸ້ມຄອງຊັບສິນ
    AssetManagement,

    /// Payment services
    /// ບໍລິການຊຳລະເງິນ
    PaymentServices,

    /// Mobile banking
    /// ທະນາຄານມືຖື
    MobileBanking,

    /// Internet banking
    /// ທະນາຄານອິນເຕີເນັດ
    InternetBanking,

    /// Credit card issuance
    /// ອອກບັດສິນເຊື່ອ
    CreditCardIssuance,

    /// Other activities
    /// ກິດຈະກຳອື່ນ
    Other { description: String },
}

// ============================================================================
// Capital Adequacy (ຄວາມພຽງພໍຂອງທຶນ)
// ============================================================================

/// Capital adequacy report (Basel III compliant)
/// ລາຍງານຄວາມພຽງພໍຂອງທຶນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalAdequacyReport {
    /// Bank name
    /// ຊື່ທະນາຄານ
    pub bank_name: String,

    /// Report date
    /// ວັນທີລາຍງານ
    pub report_date: DateTime<Utc>,

    /// Tier 1 capital (Core capital)
    /// ທຶນຂັ້ນ 1
    pub tier1_capital: Tier1Capital,

    /// Tier 2 capital (Supplementary capital)
    /// ທຶນຂັ້ນ 2
    pub tier2_capital: Tier2Capital,

    /// Risk-weighted assets
    /// ຊັບສິນທີ່ມີນ້ຳໜັກຄວາມສ່ຽງ
    pub risk_weighted_assets: RiskWeightedAssets,

    /// Total capital
    /// ທຶນລວມ
    pub total_capital_lak: u64,

    /// Capital adequacy ratio
    /// ອັດຕາສ່ວນຄວາມພຽງພໍຂອງທຶນ
    pub car_percent: f64,

    /// Tier 1 capital ratio
    /// ອັດຕາສ່ວນທຶນຂັ້ນ 1
    pub tier1_ratio_percent: f64,

    /// CET1 ratio
    /// ອັດຕາສ່ວນ CET1
    pub cet1_ratio_percent: f64,

    /// Leverage ratio
    /// ອັດຕາສ່ວນໜີ້ສິນ
    pub leverage_ratio_percent: f64,
}

impl CapitalAdequacyReport {
    /// Check if CAR meets minimum requirement
    /// ກວດສອບວ່າ CAR ຜ່ານເກນຂັ້ນຕ່ຳ
    pub fn meets_car_requirement(&self) -> bool {
        self.car_percent >= MIN_CAPITAL_ADEQUACY_RATIO_PERCENT
    }

    /// Check if Tier 1 ratio meets minimum
    /// ກວດສອບວ່າອັດຕາສ່ວນທຶນຂັ້ນ 1 ຜ່ານເກນ
    pub fn meets_tier1_requirement(&self) -> bool {
        self.tier1_ratio_percent >= MIN_TIER1_CAPITAL_RATIO_PERCENT
    }

    /// Check if leverage ratio meets minimum
    /// ກວດສອບວ່າອັດຕາສ່ວນໜີ້ສິນຜ່ານເກນ
    pub fn meets_leverage_requirement(&self) -> bool {
        self.leverage_ratio_percent >= MIN_LEVERAGE_RATIO_PERCENT
    }

    /// Check overall compliance
    /// ກວດສອບການປະຕິບັດຕາມໂດຍລວມ
    pub fn is_compliant(&self) -> bool {
        self.meets_car_requirement()
            && self.meets_tier1_requirement()
            && self.meets_leverage_requirement()
    }
}

/// Tier 1 capital components
/// ອົງປະກອບທຶນຂັ້ນ 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier1Capital {
    /// Common equity
    /// ທຶນຫຸ້ນສ່ວນທົ່ວໄປ
    pub common_equity_lak: u64,

    /// Retained earnings
    /// ກຳໄລສະສົມ
    pub retained_earnings_lak: u64,

    /// Other reserves
    /// ເງິນສຳຮອງອື່ນ
    pub other_reserves_lak: u64,

    /// Regulatory deductions
    /// ການຫັກຕາມລະບຽບ
    pub regulatory_deductions_lak: u64,

    /// Total CET1
    /// CET1 ລວມ
    pub total_cet1_lak: u64,

    /// Additional Tier 1 capital
    /// ທຶນຂັ້ນ 1 ເພີ່ມເຕີມ
    pub additional_tier1_lak: u64,

    /// Total Tier 1
    /// ທຶນຂັ້ນ 1 ລວມ
    pub total_tier1_lak: u64,
}

/// Tier 2 capital components
/// ອົງປະກອບທຶນຂັ້ນ 2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier2Capital {
    /// Subordinated debt
    /// ໜີ້ດ້ອຍສິດ
    pub subordinated_debt_lak: u64,

    /// General provisions
    /// ເງິນສຳຮອງທົ່ວໄປ
    pub general_provisions_lak: u64,

    /// Revaluation reserves
    /// ເງິນສຳຮອງຈາກການປະເມີນມູນຄ່າໃໝ່
    pub revaluation_reserves_lak: u64,

    /// Total Tier 2
    /// ທຶນຂັ້ນ 2 ລວມ
    pub total_tier2_lak: u64,
}

/// Risk-weighted assets breakdown
/// ລາຍລະອຽດຊັບສິນທີ່ມີນ້ຳໜັກຄວາມສ່ຽງ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskWeightedAssets {
    /// Credit risk RWA
    /// RWA ຄວາມສ່ຽງສິນເຊື່ອ
    pub credit_risk_lak: u64,

    /// Market risk RWA
    /// RWA ຄວາມສ່ຽງຕະຫຼາດ
    pub market_risk_lak: u64,

    /// Operational risk RWA
    /// RWA ຄວາມສ່ຽງດ້ານການດຳເນີນງານ
    pub operational_risk_lak: u64,

    /// Total RWA
    /// RWA ລວມ
    pub total_rwa_lak: u64,
}

/// Asset risk weight categories
/// ປະເພດນ້ຳໜັກຄວາມສ່ຽງຊັບສິນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetRiskWeight {
    /// Cash and government securities (0%)
    /// ເງິນສົດແລະພັນທະບັດລັດຖະບານ
    ZeroPercent,

    /// Interbank claims (20%)
    /// ສິດຮຽກຮ້ອງລະຫວ່າງທະນາຄານ
    TwentyPercent,

    /// Residential mortgages (50%)
    /// ສິນເຊື່ອທີ່ຢູ່ອາໄສ
    FiftyPercent,

    /// Corporate loans (100%)
    /// ສິນເຊື່ອວິສາຫະກິດ
    HundredPercent,

    /// High-risk assets (150%)
    /// ຊັບສິນຄວາມສ່ຽງສູງ
    OneHundredFiftyPercent,
}

impl AssetRiskWeight {
    /// Get risk weight as percentage
    /// ຮັບນ້ຳໜັກຄວາມສ່ຽງເປັນເປີເຊັນ
    pub fn weight_percent(&self) -> f64 {
        match self {
            AssetRiskWeight::ZeroPercent => 0.0,
            AssetRiskWeight::TwentyPercent => 20.0,
            AssetRiskWeight::FiftyPercent => 50.0,
            AssetRiskWeight::HundredPercent => 100.0,
            AssetRiskWeight::OneHundredFiftyPercent => 150.0,
        }
    }
}

// ============================================================================
// Prudential Regulations (ລະບຽບຄວາມສະຫຼາດສຸຂຸມ)
// ============================================================================

/// Large exposure report
/// ລາຍງານການເປີດຮັບຄວາມສ່ຽງຂະໜາດໃຫຍ່
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeExposureReport {
    /// Bank name
    /// ຊື່ທະນາຄານ
    pub bank_name: String,

    /// Report date
    /// ວັນທີລາຍງານ
    pub report_date: DateTime<Utc>,

    /// Total capital
    /// ທຶນລວມ
    pub total_capital_lak: u64,

    /// Large exposures
    /// ການເປີດຮັບຄວາມສ່ຽງຂະໜາດໃຫຍ່
    pub exposures: Vec<BorrowerExposure>,
}

impl LargeExposureReport {
    /// Check all exposures comply with limits
    /// ກວດສອບການເປີດຮັບຄວາມສ່ຽງທັງໝົດ
    pub fn all_within_limits(&self) -> bool {
        self.exposures
            .iter()
            .all(|e| e.is_within_limit(self.total_capital_lak))
    }

    /// Get exposures exceeding limits
    /// ຮັບການເປີດຮັບຄວາມສ່ຽງທີ່ເກີນຂີດຈຳກັດ
    pub fn exceeding_limit_exposures(&self) -> Vec<&BorrowerExposure> {
        self.exposures
            .iter()
            .filter(|e| !e.is_within_limit(self.total_capital_lak))
            .collect()
    }
}

/// Borrower exposure details
/// ລາຍລະອຽດການເປີດຮັບຄວາມສ່ຽງຜູ້ກູ້ຢືມ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerExposure {
    /// Borrower name
    /// ຊື່ຜູ້ກູ້ຢືມ
    pub borrower_name: String,

    /// Borrower ID
    /// ລະຫັດຜູ້ກູ້ຢືມ
    pub borrower_id: String,

    /// Is related party
    /// ເປັນບຸກຄົນກ່ຽວຂ້ອງ
    pub is_related_party: bool,

    /// Total exposure amount
    /// ຈຳນວນການເປີດຮັບຄວາມສ່ຽງລວມ
    pub exposure_amount_lak: u64,

    /// Funded exposure
    /// ການເປີດຮັບຄວາມສ່ຽງທີ່ມີທຶນ
    pub funded_exposure_lak: u64,

    /// Unfunded exposure (guarantees, commitments)
    /// ການເປີດຮັບຄວາມສ່ຽງທີ່ບໍ່ມີທຶນ
    pub unfunded_exposure_lak: u64,
}

impl BorrowerExposure {
    /// Calculate exposure as percentage of capital
    /// ຄຳນວນການເປີດຮັບຄວາມສ່ຽງເປັນເປີເຊັນຂອງທຶນ
    pub fn exposure_percent(&self, total_capital: u64) -> f64 {
        if total_capital == 0 {
            return 0.0;
        }
        (self.exposure_amount_lak as f64 / total_capital as f64) * 100.0
    }

    /// Check if within applicable limit
    /// ກວດສອບວ່າຢູ່ໃນຂີດຈຳກັດ
    pub fn is_within_limit(&self, total_capital: u64) -> bool {
        let limit = if self.is_related_party {
            RELATED_PARTY_LIMIT_PERCENT
        } else {
            SINGLE_BORROWER_LIMIT_PERCENT
        };
        self.exposure_percent(total_capital) <= limit
    }

    /// Get applicable limit
    /// ຮັບຂີດຈຳກັດທີ່ນຳໃຊ້
    pub fn applicable_limit_percent(&self) -> f64 {
        if self.is_related_party {
            RELATED_PARTY_LIMIT_PERCENT
        } else {
            SINGLE_BORROWER_LIMIT_PERCENT
        }
    }
}

/// Liquidity report
/// ລາຍງານສະພາບຄ່ອງ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityReport {
    /// Bank name
    /// ຊື່ທະນາຄານ
    pub bank_name: String,

    /// Report date
    /// ວັນທີລາຍງານ
    pub report_date: DateTime<Utc>,

    /// High-quality liquid assets
    /// ຊັບສິນສະພາບຄ່ອງຄຸນນະພາບສູງ
    pub hqla_lak: u64,

    /// Total net cash outflows (30-day)
    /// ກະແສເງິນສົດອອກສຸດທິລວມ (30 ມື້)
    pub net_cash_outflows_30d_lak: u64,

    /// Liquidity coverage ratio
    /// ອັດຕາສ່ວນຄຸ້ມຄອງສະພາບຄ່ອງ
    pub lcr_percent: f64,

    /// Available stable funding
    /// ແຫຼ່ງທຶນໝັ້ນຄົງທີ່ມີ
    pub available_stable_funding_lak: u64,

    /// Required stable funding
    /// ແຫຼ່ງທຶນໝັ້ນຄົງທີ່ຕ້ອງການ
    pub required_stable_funding_lak: u64,

    /// Net stable funding ratio
    /// ອັດຕາສ່ວນແຫຼ່ງທຶນໝັ້ນຄົງສຸດທິ
    pub nsfr_percent: f64,
}

impl LiquidityReport {
    /// Check LCR compliance
    /// ກວດສອບການປະຕິບັດຕາມ LCR
    pub fn meets_lcr_requirement(&self) -> bool {
        self.lcr_percent >= MIN_LCR_PERCENT
    }

    /// Check NSFR compliance
    /// ກວດສອບການປະຕິບັດຕາມ NSFR
    pub fn meets_nsfr_requirement(&self) -> bool {
        self.nsfr_percent >= MIN_NSFR_PERCENT
    }

    /// Check overall liquidity compliance
    /// ກວດສອບການປະຕິບັດຕາມສະພາບຄ່ອງໂດຍລວມ
    pub fn is_compliant(&self) -> bool {
        self.meets_lcr_requirement() && self.meets_nsfr_requirement()
    }
}

// ============================================================================
// Deposit Protection (ການປົກປ້ອງເງິນຝາກ)
// ============================================================================

/// Deposit types covered by insurance
/// ປະເພດເງິນຝາກທີ່ໄດ້ຮັບການປະກັນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositType {
    /// Savings deposit
    /// ເງິນຝາກປະຢັດ
    Savings,

    /// Current account
    /// ບັນຊີເງິນຝາກກະແສລາຍວັນ
    Current,

    /// Fixed deposit
    /// ເງິນຝາກມີກຳນົດ
    Fixed { term_months: u32 },

    /// Foreign currency deposit
    /// ເງິນຝາກສະກຸນເງິນຕ່າງປະເທດ
    ForeignCurrency { currency: String },
}

impl DepositType {
    /// Check if deposit type is insured
    /// ກວດສອບວ່າປະເພດເງິນຝາກໄດ້ຮັບການປະກັນ
    pub fn is_insured(&self) -> bool {
        match self {
            DepositType::Savings => true,
            DepositType::Current => true,
            DepositType::Fixed { .. } => true,
            // Foreign currency deposits typically not covered
            DepositType::ForeignCurrency { .. } => false,
        }
    }
}

/// Deposit insurance claim
/// ການຮຽກຮ້ອງປະກັນເງິນຝາກ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInsuranceClaim {
    /// Depositor name
    /// ຊື່ຜູ້ຝາກ
    pub depositor_name: String,

    /// Depositor ID
    /// ລະຫັດຜູ້ຝາກ
    pub depositor_id: String,

    /// Failed bank name
    /// ຊື່ທະນາຄານທີ່ລົ້ມລະລາຍ
    pub failed_bank_name: String,

    /// Total deposit amount
    /// ຈຳນວນເງິນຝາກລວມ
    pub total_deposit_lak: u64,

    /// Insured amount
    /// ຈຳນວນທີ່ປະກັນ
    pub insured_amount_lak: u64,

    /// Deposit types
    /// ປະເພດເງິນຝາກ
    pub deposit_types: Vec<DepositType>,

    /// Claim date
    /// ວັນທີຮຽກຮ້ອງ
    pub claim_date: DateTime<Utc>,

    /// Claim status
    /// ສະຖານະການຮຽກຮ້ອງ
    pub status: ClaimStatus,
}

impl DepositInsuranceClaim {
    /// Calculate insured amount
    /// ຄຳນວນຈຳນວນທີ່ປະກັນ
    pub fn calculate_insured_amount(&self) -> u64 {
        let insurable: u64 = self
            .deposit_types
            .iter()
            .filter(|dt| dt.is_insured())
            .count() as u64;

        if insurable == 0 {
            return 0;
        }

        self.total_deposit_lak.min(DEPOSIT_INSURANCE_LIMIT_LAK)
    }
}

/// Claim status
/// ສະຖານະການຮຽກຮ້ອງ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimStatus {
    /// Submitted
    /// ສົ່ງແລ້ວ
    Submitted,

    /// Under review
    /// ກຳລັງພິຈາລະນາ
    UnderReview,

    /// Approved
    /// ອະນຸມັດແລ້ວ
    Approved { approved_amount_lak: u64 },

    /// Rejected
    /// ປະຕິເສດ
    Rejected { reason: String },

    /// Paid
    /// ຈ່າຍແລ້ວ
    Paid { paid_date: DateTime<Utc> },
}

// ============================================================================
// Foreign Exchange (ການແລກປ່ຽນເງິນຕາ)
// ============================================================================

/// Foreign exchange transaction
/// ທຸລະກຳແລກປ່ຽນເງິນຕາ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeTransaction {
    /// Transaction ID
    /// ລະຫັດທຸລະກຳ
    pub transaction_id: String,

    /// Transaction type
    /// ປະເພດທຸລະກຳ
    pub transaction_type: FXTransactionType,

    /// Currency pair
    /// ຄູ່ສະກຸນເງິນ
    pub currency_pair: String,

    /// Amount in foreign currency
    /// ຈຳນວນເງິນຕ່າງປະເທດ
    pub foreign_amount: f64,

    /// Exchange rate
    /// ອັດຕາແລກປ່ຽນ
    pub exchange_rate: f64,

    /// LAK equivalent
    /// ເທົ່າກັບກີບ
    pub lak_equivalent: u64,

    /// BOL reference rate
    /// ອັດຕາອ້າງອິງທະນາຄານກາງ
    pub bol_reference_rate: f64,

    /// Transaction date
    /// ວັນທີທຸລະກຳ
    pub transaction_date: DateTime<Utc>,

    /// Customer name
    /// ຊື່ລູກຄ້າ
    pub customer_name: String,

    /// Purpose
    /// ຈຸດປະສົງ
    pub purpose: String,
}

/// Foreign exchange transaction types
/// ປະເພດທຸລະກຳແລກປ່ຽນເງິນຕາ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FXTransactionType {
    /// Spot transaction
    /// ທຸລະກຳທັນທີ
    Spot,

    /// Forward transaction
    /// ທຸລະກຳລ່ວງໜ້າ
    Forward { settlement_date: String },

    /// Trade payment
    /// ການຊຳລະການຄ້າ
    TradePayment,

    /// Investment remittance
    /// ການສົ່ງເງິນລົງທຶນ
    InvestmentRemittance,

    /// Personal remittance
    /// ການສົ່ງເງິນສ່ວນບຸກຄົນ
    PersonalRemittance,
}

// ============================================================================
// AML/CFT (ການຕ້ານການຟອກເງິນ)
// ============================================================================

/// Customer due diligence record
/// ບັນທຶກການກວດສອບລູກຄ້າ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDueDiligence {
    /// Customer ID
    /// ລະຫັດລູກຄ້າ
    pub customer_id: String,

    /// Customer name
    /// ຊື່ລູກຄ້າ
    pub customer_name: String,

    /// Customer type
    /// ປະເພດລູກຄ້າ
    pub customer_type: CustomerType,

    /// CDD level
    /// ລະດັບ CDD
    pub cdd_level: CDDLevel,

    /// Identity verified
    /// ຢືນຢັນຕົວຕົນແລ້ວ
    pub identity_verified: bool,

    /// Address verified
    /// ຢືນຢັນທີ່ຢູ່ແລ້ວ
    pub address_verified: bool,

    /// Source of funds documented
    /// ບັນທຶກແຫຼ່ງເງິນທຶນແລ້ວ
    pub source_of_funds_documented: bool,

    /// PEP status
    /// ສະຖານະ PEP
    pub pep_status: PEPStatus,

    /// Risk rating
    /// ການຈັດອັນດັບຄວາມສ່ຽງ
    pub risk_rating: RiskRating,

    /// Last review date
    /// ວັນທີກວດສອບລ່າສຸດ
    pub last_review_date: DateTime<Utc>,

    /// Next review date
    /// ວັນທີກວດສອບຕໍ່ໄປ
    pub next_review_date: DateTime<Utc>,
}

impl CustomerDueDiligence {
    /// Check if CDD is complete
    /// ກວດສອບວ່າ CDD ສຳເລັດ
    pub fn is_complete(&self) -> bool {
        self.identity_verified && self.address_verified && self.source_of_funds_documented
    }

    /// Check if review is due
    /// ກວດສອບວ່າຮອດກຳນົດກວດສອບ
    pub fn is_review_due(&self) -> bool {
        Utc::now() >= self.next_review_date
    }
}

/// Customer types
/// ປະເພດລູກຄ້າ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomerType {
    /// Individual
    /// ບຸກຄົນ
    Individual,

    /// Corporate
    /// ນິຕິບຸກຄົນ
    Corporate,

    /// Government entity
    /// ໜ່ວຍງານລັດຖະບານ
    Government,

    /// Financial institution
    /// ສະຖາບັນການເງິນ
    FinancialInstitution,

    /// Non-profit organization
    /// ອົງການບໍ່ຫວັງຜົນກຳໄລ
    NonProfit,
}

/// CDD levels
/// ລະດັບ CDD
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CDDLevel {
    /// Simplified due diligence
    /// ການກວດສອບແບບງ່າຍ
    Simplified,

    /// Standard due diligence
    /// ການກວດສອບມາດຕະຖານ
    Standard,

    /// Enhanced due diligence
    /// ການກວດສອບເຂັ້ມງວດ
    Enhanced,
}

/// Politically exposed person status
/// ສະຖານະບຸກຄົນການເມືອງ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PEPStatus {
    /// Not a PEP
    /// ບໍ່ແມ່ນ PEP
    NotPEP,

    /// Domestic PEP
    /// PEP ພາຍໃນ
    DomesticPEP { position: String },

    /// Foreign PEP
    /// PEP ຕ່າງປະເທດ
    ForeignPEP { country: String, position: String },

    /// PEP family member
    /// ສະມາຊິກຄອບຄົວ PEP
    PEPFamilyMember { relationship: String },

    /// PEP close associate
    /// ຜູ້ໃກ້ຊິດ PEP
    PEPCloseAssociate { relationship: String },
}

/// Risk rating levels
/// ລະດັບການຈັດອັນດັບຄວາມສ່ຽງ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskRating {
    /// Low risk
    /// ຄວາມສ່ຽງຕ່ຳ
    Low,

    /// Medium risk
    /// ຄວາມສ່ຽງປານກາງ
    Medium,

    /// High risk
    /// ຄວາມສ່ຽງສູງ
    High,

    /// Prohibited
    /// ຫ້າມ
    Prohibited,
}

/// Suspicious transaction report
/// ລາຍງານທຸລະກຳທີ່ໜ້າສົງໄສ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousTransactionReport {
    /// Report ID
    /// ລະຫັດລາຍງານ
    pub report_id: String,

    /// Reporting bank
    /// ທະນາຄານທີ່ລາຍງານ
    pub reporting_bank: String,

    /// Customer ID
    /// ລະຫັດລູກຄ້າ
    pub customer_id: String,

    /// Customer name
    /// ຊື່ລູກຄ້າ
    pub customer_name: String,

    /// Transaction IDs
    /// ລະຫັດທຸລະກຳ
    pub transaction_ids: Vec<String>,

    /// Total amount
    /// ຈຳນວນລວມ
    pub total_amount_lak: u64,

    /// Suspicion indicators
    /// ຕົວຊີ້ວັດຄວາມສົງໄສ
    pub suspicion_indicators: Vec<SuspicionIndicator>,

    /// Report date
    /// ວັນທີລາຍງານ
    pub report_date: DateTime<Utc>,

    /// Status
    /// ສະຖານະ
    pub status: STRStatus,
}

/// Suspicion indicators
/// ຕົວຊີ້ວັດຄວາມສົງໄສ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuspicionIndicator {
    /// Unusual transaction pattern
    /// ຮູບແບບທຸລະກຳທີ່ຜິດປົກກະຕິ
    UnusualPattern,

    /// Structuring
    /// ການແບ່ງຍ່ອຍ
    Structuring,

    /// Inconsistent with profile
    /// ບໍ່ສອດຄ່ອງກັບໂປຣໄຟລ໌
    InconsistentWithProfile,

    /// Cash-intensive
    /// ໃຊ້ເງິນສົດຫຼາຍ
    CashIntensive,

    /// High-risk jurisdiction
    /// ເຂດອຳນາດຄວາມສ່ຽງສູງ
    HighRiskJurisdiction { jurisdiction: String },

    /// Sanctions match
    /// ກົງກັບລາຍຊື່ຄວ່ຳບາດ
    SanctionsMatch { list_name: String },

    /// Other
    /// ອື່ນ
    Other { description: String },
}

/// STR status
/// ສະຖານະ STR
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum STRStatus {
    /// Draft
    /// ຮ່າງ
    Draft,

    /// Submitted to FIU
    /// ສົ່ງໃຫ້ FIU ແລ້ວ
    SubmittedToFIU { submitted_at: DateTime<Utc> },

    /// Under investigation
    /// ກຳລັງສືບສວນ
    UnderInvestigation,

    /// Closed
    /// ປິດແລ້ວ
    Closed { outcome: String },
}

// ============================================================================
// Interest Rate (ອັດຕາດອກເບ້ຍ)
// ============================================================================

/// Interest rate structure
/// ໂຄງສ້າງອັດຕາດອກເບ້ຍ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateStructure {
    /// BOL reference rate
    /// ອັດຕາອ້າງອິງທະນາຄານກາງ
    pub bol_reference_rate: f64,

    /// Effective date
    /// ວັນທີມີຜົນ
    pub effective_date: DateTime<Utc>,

    /// Deposit rates
    /// ອັດຕາເງິນຝາກ
    pub deposit_rates: Vec<DepositRate>,

    /// Lending rates
    /// ອັດຕາກູ້ຢືມ
    pub lending_rates: Vec<LendingRate>,
}

/// Deposit rate by term
/// ອັດຕາເງິນຝາກຕາມກຳນົດ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRate {
    /// Term in months (0 = savings/current)
    /// ກຳນົດເປັນເດືອນ
    pub term_months: u32,

    /// Interest rate (annual)
    /// ອັດຕາດອກເບ້ຍ (ຕໍ່ປີ)
    pub rate_percent: f64,

    /// Minimum deposit amount
    /// ຈຳນວນເງິນຝາກຂັ້ນຕ່ຳ
    pub minimum_amount_lak: Option<u64>,
}

/// Lending rate by type
/// ອັດຕາກູ້ຢືມຕາມປະເພດ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LendingRate {
    /// Loan type
    /// ປະເພດສິນເຊື່ອ
    pub loan_type: LoanType,

    /// Interest rate (annual)
    /// ອັດຕາດອກເບ້ຍ (ຕໍ່ປີ)
    pub rate_percent: f64,

    /// Spread over reference rate
    /// ສ່ວນຕ່າງເໜືອອັດຕາອ້າງອິງ
    pub spread_percent: f64,
}

/// Loan types
/// ປະເພດສິນເຊື່ອ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanType {
    /// Personal loan
    /// ສິນເຊື່ອສ່ວນບຸກຄົນ
    Personal,

    /// Housing loan
    /// ສິນເຊື່ອທີ່ຢູ່ອາໄສ
    Housing,

    /// Auto loan
    /// ສິນເຊື່ອລົດຍົນ
    Auto,

    /// Business loan
    /// ສິນເຊື່ອທຸລະກິດ
    Business,

    /// Agricultural loan
    /// ສິນເຊື່ອກະສິກຳ
    Agricultural,

    /// SME loan
    /// ສິນເຊື່ອ SME
    SME,

    /// Microfinance
    /// ສິນເຊື່ອຈຸລະພາກ
    Microfinance,
}

// ============================================================================
// Payment Systems (ລະບົບການຊຳລະເງິນ)
// ============================================================================

/// RTGS transaction
/// ທຸລະກຳ RTGS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTGSTransaction {
    /// Transaction reference
    /// ເລກອ້າງອິງທຸລະກຳ
    pub reference: String,

    /// Sending bank
    /// ທະນາຄານຜູ້ສົ່ງ
    pub sending_bank: String,

    /// Receiving bank
    /// ທະນາຄານຜູ້ຮັບ
    pub receiving_bank: String,

    /// Amount
    /// ຈຳນວນ
    pub amount_lak: u64,

    /// Transaction status
    /// ສະຖານະທຸລະກຳ
    pub status: RTGSStatus,

    /// Submission time
    /// ເວລາສົ່ງ
    pub submitted_at: DateTime<Utc>,

    /// Settlement time
    /// ເວລາຊຳລະ
    pub settled_at: Option<DateTime<Utc>>,
}

/// RTGS transaction status
/// ສະຖານະທຸລະກຳ RTGS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RTGSStatus {
    /// Pending
    /// ລໍຖ້າ
    Pending,

    /// Queued
    /// ຢູ່ໃນຄິວ
    Queued,

    /// Settled
    /// ຊຳລະແລ້ວ
    Settled,

    /// Rejected
    /// ປະຕິເສດ
    Rejected { reason: String },

    /// Cancelled
    /// ຍົກເລີກ
    Cancelled,
}

/// Payment service provider license
/// ໃບອະນຸຍາດຜູ້ໃຫ້ບໍລິການຊຳລະເງິນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentServiceLicense {
    /// License number
    /// ເລກທີໃບອະນຸຍາດ
    pub license_number: String,

    /// Provider name
    /// ຊື່ຜູ້ໃຫ້ບໍລິການ
    pub provider_name: String,

    /// Services authorized
    /// ບໍລິການທີ່ໄດ້ຮັບອະນຸຍາດ
    pub authorized_services: Vec<PaymentService>,

    /// Issue date
    /// ວັນທີອອກ
    pub issued_at: DateTime<Utc>,

    /// Expiry date
    /// ວັນທີໝົດອາຍຸ
    pub expires_at: DateTime<Utc>,

    /// Status
    /// ສະຖານະ
    pub status: LicenseStatus,
}

/// Payment services
/// ບໍລິການຊຳລະເງິນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentService {
    /// Mobile money
    /// ເງິນມືຖື
    MobileMoney,

    /// Internet payment gateway
    /// ປະຕູການຊຳລະເງິນອິນເຕີເນັດ
    InternetPaymentGateway,

    /// Bill payment
    /// ການຊຳລະບິນ
    BillPayment,

    /// Remittance
    /// ການສົ່ງເງິນ
    Remittance,

    /// Card processing
    /// ການປະມວນຜົນບັດ
    CardProcessing,

    /// QR payment
    /// ການຊຳລະ QR
    QRPayment,
}

// ============================================================================
// BOL Supervision (ການກຳກັບຂອງທະນາຄານກາງ)
// ============================================================================

/// BOL regulatory report types
/// ປະເພດລາຍງານລະບຽບທະນາຄານກາງ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BOLReportType {
    /// Daily liquidity report
    /// ລາຍງານສະພາບຄ່ອງປະຈຳວັນ
    DailyLiquidity,

    /// Monthly capital adequacy
    /// ລາຍງານຄວາມພຽງພໍຂອງທຶນປະຈຳເດືອນ
    MonthlyCapitalAdequacy,

    /// Quarterly large exposure
    /// ລາຍງານການເປີດຮັບຄວາມສ່ຽງຂະໜາດໃຫຍ່ປະຈຳໄຕມາດ
    QuarterlyLargeExposure,

    /// Annual audit report
    /// ລາຍງານກວດສອບປະຈຳປີ
    AnnualAudit,

    /// STR summary
    /// ສະຫຼຸບ STR
    STRSummary,

    /// Foreign exchange report
    /// ລາຍງານການແລກປ່ຽນເງິນຕາ
    ForeignExchangeReport,
}

impl BOLReportType {
    /// Get submission deadline description
    /// ຮັບຄຳອະທິບາຍກຳນົດສົ່ງ
    pub fn submission_deadline(&self) -> &'static str {
        match self {
            BOLReportType::DailyLiquidity => "By 10:00 AM next business day",
            BOLReportType::MonthlyCapitalAdequacy => "Within 15 days after month end",
            BOLReportType::QuarterlyLargeExposure => "Within 30 days after quarter end",
            BOLReportType::AnnualAudit => "Within 120 days after fiscal year end",
            BOLReportType::STRSummary => "Monthly, within 10 days after month end",
            BOLReportType::ForeignExchangeReport => "Weekly, by Monday noon",
        }
    }
}

/// Director fit and proper assessment
/// ການປະເມີນຄວາມເໝາະສົມຂອງກຳມະການ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitAndProperAssessment {
    /// Director name
    /// ຊື່ກຳມະການ
    pub director_name: String,

    /// Position
    /// ຕຳແໜ່ງ
    pub position: String,

    /// Assessment date
    /// ວັນທີປະເມີນ
    pub assessment_date: DateTime<Utc>,

    /// Educational qualifications met
    /// ຄຸນວຸດທິການສຶກສາຜ່ານ
    pub education_met: bool,

    /// Experience requirements met
    /// ເງື່ອນໄຂປະສົບການຜ່ານ
    pub experience_met: bool,

    /// No criminal record
    /// ບໍ່ມີປະຫວັດອາຊະຍາກຳ
    pub no_criminal_record: bool,

    /// No bankruptcy
    /// ບໍ່ເຄີຍລົ້ມລະລາຍ
    pub no_bankruptcy: bool,

    /// Not disqualified from other positions
    /// ບໍ່ຖືກຫ້າມຈາກຕຳແໜ່ງອື່ນ
    pub not_disqualified: bool,

    /// Overall result
    /// ຜົນລວມ
    pub passed: bool,

    /// Conditions (if any)
    /// ເງື່ອນໄຂ (ຖ້າມີ)
    pub conditions: Option<String>,
}

impl FitAndProperAssessment {
    /// Check if all criteria are met
    /// ກວດສອບວ່າຜ່ານທຸກເກນ
    pub fn all_criteria_met(&self) -> bool {
        self.education_met
            && self.experience_met
            && self.no_criminal_record
            && self.no_bankruptcy
            && self.not_disqualified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_type_minimum_capital() {
        let commercial = BankType::StateOwned;
        assert_eq!(
            commercial.minimum_capital_lak(),
            MIN_CAPITAL_COMMERCIAL_BANK_LAK
        );

        let foreign_branch = BankType::ForeignBranch {
            parent_country: "Thailand".to_string(),
            parent_bank_name: "Thai Bank".to_string(),
        };
        assert_eq!(
            foreign_branch.minimum_capital_lak(),
            MIN_CAPITAL_FOREIGN_BRANCH_LAK
        );
    }

    #[test]
    fn test_mfi_minimum_capital() {
        let deposit_taking = MicrofinanceType::DepositTaking;
        assert_eq!(
            deposit_taking.minimum_capital_lak(),
            MIN_CAPITAL_MFI_DEPOSIT_LAK
        );

        let non_deposit = MicrofinanceType::NonDeposit;
        assert_eq!(
            non_deposit.minimum_capital_lak(),
            MIN_CAPITAL_MFI_NON_DEPOSIT_LAK
        );
    }

    #[test]
    fn test_asset_risk_weight() {
        assert_eq!(AssetRiskWeight::ZeroPercent.weight_percent(), 0.0);
        assert_eq!(AssetRiskWeight::TwentyPercent.weight_percent(), 20.0);
        assert_eq!(AssetRiskWeight::HundredPercent.weight_percent(), 100.0);
    }

    #[test]
    fn test_borrower_exposure_limits() {
        let exposure = BorrowerExposure {
            borrower_name: "ABC Company".to_string(),
            borrower_id: "ABC-001".to_string(),
            is_related_party: false,
            exposure_amount_lak: 50_000_000_000,
            funded_exposure_lak: 40_000_000_000,
            unfunded_exposure_lak: 10_000_000_000,
        };

        let capital = 300_000_000_000u64;
        let percent = exposure.exposure_percent(capital);
        assert!(percent < SINGLE_BORROWER_LIMIT_PERCENT);
        assert!(exposure.is_within_limit(capital));
    }

    #[test]
    fn test_related_party_limits() {
        let exposure = BorrowerExposure {
            borrower_name: "Director's Company".to_string(),
            borrower_id: "DIR-001".to_string(),
            is_related_party: true,
            exposure_amount_lak: 50_000_000_000,
            funded_exposure_lak: 50_000_000_000,
            unfunded_exposure_lak: 0,
        };

        assert_eq!(
            exposure.applicable_limit_percent(),
            RELATED_PARTY_LIMIT_PERCENT
        );
    }

    #[test]
    fn test_deposit_insurance() {
        assert!(DepositType::Savings.is_insured());
        assert!(DepositType::Current.is_insured());
        assert!(DepositType::Fixed { term_months: 12 }.is_insured());
        assert!(
            !DepositType::ForeignCurrency {
                currency: "USD".to_string()
            }
            .is_insured()
        );
    }

    #[test]
    fn test_cdd_completion() {
        let cdd = CustomerDueDiligence {
            customer_id: "C001".to_string(),
            customer_name: "Test Customer".to_string(),
            customer_type: CustomerType::Individual,
            cdd_level: CDDLevel::Standard,
            identity_verified: true,
            address_verified: true,
            source_of_funds_documented: true,
            pep_status: PEPStatus::NotPEP,
            risk_rating: RiskRating::Low,
            last_review_date: Utc::now(),
            next_review_date: Utc::now() + chrono::Duration::days(365),
        };

        assert!(cdd.is_complete());
        assert!(!cdd.is_review_due());
    }

    #[test]
    fn test_cdd_incomplete() {
        let cdd = CustomerDueDiligence {
            customer_id: "C002".to_string(),
            customer_name: "Incomplete Customer".to_string(),
            customer_type: CustomerType::Corporate,
            cdd_level: CDDLevel::Enhanced,
            identity_verified: true,
            address_verified: false,
            source_of_funds_documented: false,
            pep_status: PEPStatus::DomesticPEP {
                position: "Minister".to_string(),
            },
            risk_rating: RiskRating::High,
            last_review_date: Utc::now(),
            next_review_date: Utc::now() + chrono::Duration::days(90),
        };

        assert!(!cdd.is_complete());
    }

    #[test]
    fn test_pep_status() {
        let not_pep = PEPStatus::NotPEP;
        assert!(matches!(not_pep, PEPStatus::NotPEP));

        let domestic_pep = PEPStatus::DomesticPEP {
            position: "Minister".to_string(),
        };
        assert!(matches!(domestic_pep, PEPStatus::DomesticPEP { .. }));

        let foreign_pep = PEPStatus::ForeignPEP {
            country: "Thailand".to_string(),
            position: "Ambassador".to_string(),
        };
        assert!(matches!(foreign_pep, PEPStatus::ForeignPEP { .. }));
    }

    #[test]
    fn test_bol_report_deadlines() {
        let daily = BOLReportType::DailyLiquidity;
        assert!(daily.submission_deadline().contains("10:00 AM"));

        let monthly = BOLReportType::MonthlyCapitalAdequacy;
        assert!(monthly.submission_deadline().contains("15 days"));

        let annual = BOLReportType::AnnualAudit;
        assert!(annual.submission_deadline().contains("120 days"));
    }

    #[test]
    fn test_fit_and_proper_assessment() {
        let assessment = FitAndProperAssessment {
            director_name: "Test Director".to_string(),
            position: "CEO".to_string(),
            assessment_date: Utc::now(),
            education_met: true,
            experience_met: true,
            no_criminal_record: true,
            no_bankruptcy: true,
            not_disqualified: true,
            passed: true,
            conditions: None,
        };

        assert!(assessment.all_criteria_met());
    }

    #[test]
    fn test_fit_and_proper_failure() {
        let assessment = FitAndProperAssessment {
            director_name: "Failed Director".to_string(),
            position: "CFO".to_string(),
            assessment_date: Utc::now(),
            education_met: true,
            experience_met: false,
            no_criminal_record: true,
            no_bankruptcy: true,
            not_disqualified: true,
            passed: false,
            conditions: Some("Must gain 2 more years experience".to_string()),
        };

        assert!(!assessment.all_criteria_met());
    }

    #[test]
    fn test_rtgs_status() {
        let pending = RTGSStatus::Pending;
        assert!(matches!(pending, RTGSStatus::Pending));

        let rejected = RTGSStatus::Rejected {
            reason: "Insufficient funds".to_string(),
        };
        assert!(matches!(rejected, RTGSStatus::Rejected { .. }));
    }

    #[test]
    fn test_constants() {
        assert_eq!(MIN_CAPITAL_COMMERCIAL_BANK_LAK, 300_000_000_000);
        assert_eq!(MIN_CAPITAL_FOREIGN_BRANCH_LAK, 50_000_000_000);
        assert_eq!(MIN_CAPITAL_ADEQUACY_RATIO_PERCENT, 8.0);
        assert_eq!(SINGLE_BORROWER_LIMIT_PERCENT, 25.0);
        assert_eq!(RELATED_PARTY_LIMIT_PERCENT, 15.0);
        assert_eq!(DEPOSIT_INSURANCE_LIMIT_LAK, 50_000_000);
        assert_eq!(AML_RECORD_KEEPING_YEARS, 5);
    }

    #[test]
    fn test_bank_type_lao_description() {
        let state_owned = BankType::StateOwned;
        assert_eq!(state_owned.description_lao(), "ທະນາຄານລັດ");

        let private = BankType::PrivateDomestic;
        assert_eq!(private.description_lao(), "ທະນາຄານເອກະຊົນພາຍໃນ");
    }

    #[test]
    fn test_suspicion_indicators() {
        let structuring = SuspicionIndicator::Structuring;
        assert!(matches!(structuring, SuspicionIndicator::Structuring));

        let high_risk = SuspicionIndicator::HighRiskJurisdiction {
            jurisdiction: "Test Country".to_string(),
        };
        assert!(matches!(
            high_risk,
            SuspicionIndicator::HighRiskJurisdiction { .. }
        ));
    }

    #[test]
    fn test_payment_services() {
        let mobile = PaymentService::MobileMoney;
        assert!(matches!(mobile, PaymentService::MobileMoney));

        let qr = PaymentService::QRPayment;
        assert!(matches!(qr, PaymentService::QRPayment));
    }
}
