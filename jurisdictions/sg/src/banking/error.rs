//! Banking Act Error Types
//!
//! Comprehensive error types for violations of the Banking Act (Cap. 19) and MAS regulations.
//! All error messages are provided in three languages:
//! - English (official business language)
//! - Chinese/中文 (Simplified Chinese for 74% of population)
//! - Malay/Bahasa Melayu (national language, 13% of population)

use thiserror::Error;

/// Result type for banking operations
pub type Result<T> = std::result::Result<T, BankingError>;

/// Banking Act and MAS regulation violation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum BankingError {
    // ========== License Violations (Banking Act) ==========
    /// Bank operating without a valid MAS license
    #[error(
        "Bank operating without valid license (Banking Act s. 4)\n\
         银行在没有有效许可证的情况下运营 (银行法第4条)\n\
         Bank beroperasi tanpa lesen sah (Akta Perbankan s. 4)"
    )]
    NoValidLicense,

    /// License has been suspended by MAS
    #[error(
        "Banking license suspended by MAS (Banking Act s. 17)\n\
         银行许可证已被金融管理局暂停 (银行法第17条)\n\
         Lesen perbankan digantung oleh MAS (Akta Perbankan s. 17)"
    )]
    LicenseSuspended,

    /// License has been revoked
    #[error(
        "Banking license has been revoked (Banking Act s. 18)\n\
         银行许可证已被撤销 (银行法第18条)\n\
         Lesen perbankan telah dibatalkan (Akta Perbankan s. 18)"
    )]
    LicenseRevoked,

    /// License has expired
    #[error(
        "Banking license expired on {expiry_date} (Banking Act s. 4)\n\
         银行许可证已于 {expiry_date} 到期 (银行法第4条)\n\
         Lesen perbankan tamat tempoh pada {expiry_date} (Akta Perbankan s. 4)"
    )]
    LicenseExpired { expiry_date: String },

    /// Wholesale bank accepting deposits below minimum
    #[error(
        "Wholesale bank cannot accept deposits below SGD 250,000, attempted: SGD {amount:.2} (Banking Act s. 4)\n\
         批发银行不能接受低于25万新元的存款, 尝试金额: {amount:.2}新元 (银行法第4条)\n\
         Bank borong tidak boleh terima deposit kurang daripada SGD 250,000, percubaan: SGD {amount:.2} (Akta Perbankan s. 4)"
    )]
    WholesaleBankMinimumDeposit { amount: f64 },

    /// Merchant bank accepting retail deposits
    #[error(
        "Merchant bank cannot accept retail deposits (Banking Act s. 28)\n\
         商业银行不能接受零售存款 (银行法第28条)\n\
         Bank saudagar tidak boleh terima deposit runcit (Akta Perbankan s. 28)"
    )]
    MerchantBankRetailDeposit,

    // ========== Capital Adequacy Violations (MAS Notice 637) ==========
    /// CET1 ratio below regulatory minimum
    #[error(
        "CET1 ratio {ratio:.2}% below minimum 6.5% (MAS Notice 637)\n\
         一级普通股本比率 {ratio:.2}% 低于最低要求6.5% (金管局通知637)\n\
         Nisbah CET1 {ratio:.2}% di bawah minimum 6.5% (Notis MAS 637)"
    )]
    InsufficientCet1 { ratio: f64 },

    /// Tier 1 capital ratio below regulatory minimum
    #[error(
        "Tier 1 capital ratio {ratio:.2}% below minimum 8.0% (MAS Notice 637)\n\
         一级资本比率 {ratio:.2}% 低于最低要求8.0% (金管局通知637)\n\
         Nisbah modal Tier 1 {ratio:.2}% di bawah minimum 8.0% (Notis MAS 637)"
    )]
    InsufficientTier1 { ratio: f64 },

    /// Total capital ratio below regulatory minimum
    #[error(
        "Total capital ratio {ratio:.2}% below minimum 10.0% (MAS Notice 637)\n\
         总资本比率 {ratio:.2}% 低于最低要求10.0% (金管局通知637)\n\
         Nisbah modal jumlah {ratio:.2}% di bawah minimum 10.0% (Notis MAS 637)"
    )]
    InsufficientTotalCapital { ratio: f64 },

    /// Risk-weighted assets calculation missing
    #[error(
        "Risk-weighted assets (RWA) cannot be zero (MAS Notice 637)\n\
         风险加权资产不能为零 (金管局通知637)\n\
         Aset berwajaran risiko tidak boleh sifar (Notis MAS 637)"
    )]
    ZeroRiskWeightedAssets,

    // ========== AML/CFT Violations (MAS Notice 626) ==========
    /// No AML compliance officer appointed
    #[error(
        "Bank must appoint an AML/CFT compliance officer (MAS Notice 626 para 6)\n\
         银行必须任命反洗钱/反恐怖融资合规官 (金管局通知626第6段)\n\
         Bank mesti lantik pegawai pematuhan AML/CFT (Notis MAS 626 perenggan 6)"
    )]
    NoAmlOfficer,

    /// Customer Due Diligence (CDD) not performed
    #[error(
        "Customer Due Diligence (CDD) not performed for account {account_number} (MAS Notice 626 para 7)\n\
         账户 {account_number} 未执行客户尽职调查 (金管局通知626第7段)\n\
         Usaha Wajar Pelanggan tidak dilakukan untuk akaun {account_number} (Notis MAS 626 perenggan 7)"
    )]
    CddNotPerformed { account_number: String },

    /// CDD review overdue
    #[error(
        "CDD review overdue for account {account_number}, last reviewed {days_ago} days ago (MAS Notice 626)\n\
         账户 {account_number} 的客户尽职调查审查已逾期, 上次审查于 {days_ago} 天前 (金管局通知626)\n\
         Semakan CDD tertunggak untuk akaun {account_number}, terakhir disemak {days_ago} hari lalu (Notis MAS 626)"
    )]
    CddReviewOverdue {
        account_number: String,
        days_ago: i64,
    },

    /// Enhanced Due Diligence (EDD) required but not performed
    #[error(
        "Enhanced Due Diligence (EDD) required for high-risk customer {account_number} but not performed (MAS Notice 626 para 8)\n\
         高风险客户 {account_number} 需要进行强化尽职调查但未执行 (金管局通知626第8段)\n\
         Usaha Wajar Diperkukuh diperlukan untuk pelanggan berisiko tinggi {account_number} tetapi tidak dilakukan (Notis MAS 626 perenggan 8)"
    )]
    EddRequired { account_number: String },

    /// Source of funds not verified
    #[error(
        "Source of funds not verified for account {account_number} (MAS Notice 626 para 7.3)\n\
         账户 {account_number} 的资金来源未核实 (金管局通知626第7.3段)\n\
         Sumber dana tidak disahkan untuk akaun {account_number} (Notis MAS 626 perenggan 7.3)"
    )]
    SourceOfFundsNotVerified { account_number: String },

    /// Beneficial owner not identified
    #[error(
        "Beneficial owner not identified for corporate account {account_number} (MAS Notice 626 para 7.2)\n\
         企业账户 {account_number} 的实益所有人未识别 (金管局通知626第7.2段)\n\
         Pemilik benefisial tidak dikenal pasti untuk akaun korporat {account_number} (Notis MAS 626 perenggan 7.2)"
    )]
    BeneficialOwnerNotIdentified { account_number: String },

    /// Suspicious transaction not reported
    #[error(
        "Suspicious transaction not reported to STRO within reasonable timeframe (MAS Notice 626 para 17)\n\
         可疑交易未在合理时间内向可疑交易报告办公室报告 (金管局通知626第17段)\n\
         Transaksi mencurigakan tidak dilaporkan kepada STRO dalam jangka masa munasabah (Notis MAS 626 perenggan 17)"
    )]
    SuspiciousTransactionNotReported,

    /// STR filed late
    #[error(
        "Suspicious Transaction Report filed {days_late} days after transaction (MAS Notice 626 para 17)\n\
         可疑交易报告在交易后 {days_late} 天才提交 (金管局通知626第17段)\n\
         Laporan Transaksi Mencurigakan difailkan {days_late} hari selepas transaksi (Notis MAS 626 perenggan 17)"
    )]
    StrFiledLate { days_late: i64 },

    /// Cash transaction report not filed
    #[error(
        "Cash transaction of SGD {amount:.2} not reported (threshold: SGD 20,000) (MAS Notice 626)\n\
         现金交易 {amount:.2}新元 未报告 (门槛: 2万新元) (金管局通知626)\n\
         Transaksi tunai SGD {amount:.2} tidak dilaporkan (ambang: SGD 20,000) (Notis MAS 626)"
    )]
    CashTransactionNotReported { amount: f64 },

    // ========== Operational Violations ==========
    /// Invalid UEN format
    #[error(
        "Invalid UEN format: {uen} (must be 9-10 digits assigned by ACRA)\n\
         无效的统一实体号码格式: {uen} (必须是会计与企业管制局分配的9-10位数字)\n\
         Format UEN tidak sah: {uen} (mesti 9-10 digit yang diberikan oleh ACRA)"
    )]
    InvalidUen { uen: String },

    /// Bank name empty or too short
    #[error(
        "Bank name must be at least 3 characters\n\
         银行名称必须至少3个字符\n\
         Nama bank mesti sekurang-kurangnya 3 aksara"
    )]
    InvalidBankName,

    /// Total assets less than total deposits (impossible situation)
    #[error(
        "Total assets (SGD {assets:.2}) cannot be less than total deposits (SGD {deposits:.2})\n\
         总资产 ({assets:.2}新元) 不能少于总存款 ({deposits:.2}新元)\n\
         Jumlah aset (SGD {assets:.2}) tidak boleh kurang daripada jumlah deposit (SGD {deposits:.2})"
    )]
    AssetsLessThanDeposits { assets: f64, deposits: f64 },

    /// Deposit-to-asset ratio too high (indicates liquidity risk)
    #[error(
        "Deposit-to-asset ratio {ratio:.2}% exceeds prudent limit 90% (liquidity risk)\n\
         存款资产比率 {ratio:.2}% 超过审慎限额90% (流动性风险)\n\
         Nisbah deposit-ke-aset {ratio:.2}% melebihi had berhemat 90% (risiko kecairan)"
    )]
    ExcessiveDepositRatio { ratio: f64 },

    // ========== General Errors ==========
    /// Missing required field
    #[error(
        "Missing required field: {field}\n\
         缺少必填字段: {field}\n\
         Medan diperlukan hilang: {field}"
    )]
    MissingRequiredField { field: String },

    /// Invalid date (future date where past is required)
    #[error(
        "Invalid date: {description}\n\
         无效日期: {description}\n\
         Tarikh tidak sah: {description}"
    )]
    InvalidDate { description: String },
}

impl BankingError {
    /// Get the statutory reference for this error
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            BankingError::NoValidLicense
            | BankingError::LicenseExpired { .. }
            | BankingError::WholesaleBankMinimumDeposit { .. } => "Banking Act s. 4",
            BankingError::LicenseSuspended => "Banking Act s. 17",
            BankingError::LicenseRevoked => "Banking Act s. 18",
            BankingError::MerchantBankRetailDeposit => "Banking Act s. 28",
            BankingError::InsufficientCet1 { .. }
            | BankingError::InsufficientTier1 { .. }
            | BankingError::InsufficientTotalCapital { .. }
            | BankingError::ZeroRiskWeightedAssets => "MAS Notice 637",
            BankingError::NoAmlOfficer => "MAS Notice 626 para 6",
            BankingError::CddNotPerformed { .. }
            | BankingError::SourceOfFundsNotVerified { .. } => "MAS Notice 626 para 7",
            BankingError::BeneficialOwnerNotIdentified { .. } => "MAS Notice 626 para 7.2",
            BankingError::EddRequired { .. } => "MAS Notice 626 para 8",
            BankingError::SuspiciousTransactionNotReported | BankingError::StrFiledLate { .. } => {
                "MAS Notice 626 para 17"
            }
            BankingError::CddReviewOverdue { .. }
            | BankingError::CashTransactionNotReported { .. } => "MAS Notice 626",
            _ => "Banking Act / MAS Regulations",
        }
    }

    /// Get the severity level of this violation
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            BankingError::NoValidLicense
            | BankingError::LicenseRevoked
            | BankingError::InsufficientCet1 { .. }
            | BankingError::InsufficientTier1 { .. }
            | BankingError::InsufficientTotalCapital { .. }
            | BankingError::SuspiciousTransactionNotReported => ErrorSeverity::Critical,

            BankingError::LicenseSuspended
            | BankingError::NoAmlOfficer
            | BankingError::EddRequired { .. }
            | BankingError::MerchantBankRetailDeposit
            | BankingError::WholesaleBankMinimumDeposit { .. } => ErrorSeverity::High,

            BankingError::CddNotPerformed { .. }
            | BankingError::BeneficialOwnerNotIdentified { .. }
            | BankingError::SourceOfFundsNotVerified { .. }
            | BankingError::StrFiledLate { .. }
            | BankingError::CashTransactionNotReported { .. }
            | BankingError::ExcessiveDepositRatio { .. } => ErrorSeverity::Medium,

            BankingError::CddReviewOverdue { .. }
            | BankingError::LicenseExpired { .. }
            | BankingError::ZeroRiskWeightedAssets => ErrorSeverity::Medium,

            _ => ErrorSeverity::Low,
        }
    }
}

/// Severity level for banking violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity - administrative issues
    Low,
    /// Medium severity - compliance concerns
    Medium,
    /// High severity - regulatory violations
    High,
    /// Critical severity - immediate action required
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "LOW"),
            ErrorSeverity::Medium => write!(f, "MEDIUM"),
            ErrorSeverity::High => write!(f, "HIGH"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}
