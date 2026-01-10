//! Payment Services Act Error Types
//!
//! Comprehensive error types for violations of the Payment Services Act 2019.
//! All error messages are provided in three languages:
//! - English (official business language)
//! - Chinese/中文 (Simplified Chinese)
//! - Malay/Bahasa Melayu (national language)

use thiserror::Error;

/// Result type for payment services operations
pub type Result<T> = std::result::Result<T, PaymentError>;

/// Payment Services Act 2019 violation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PaymentError {
    // ========== License Violations (PSA s. 5-6) ==========
    /// Operating without a valid license
    #[error(
        "Payment service provider operating without valid license (PSA s. 5)\n\
         支付服务提供商在没有有效许可证的情况下运营 (支付服务法第5条)\n\
         Penyedia perkhidmatan pembayaran beroperasi tanpa lesen sah (PSA s. 5)"
    )]
    NoValidLicense,

    /// License suspended by MAS
    #[error(
        "Payment service license suspended by MAS (PSA s. 8)\n\
         支付服务许可证已被金融管理局暂停 (支付服务法第8条)\n\
         Lesen perkhidmatan pembayaran digantung oleh MAS (PSA s. 8)"
    )]
    LicenseSuspended,

    /// License revoked
    #[error(
        "Payment service license has been revoked (PSA s. 8)\n\
         支付服务许可证已被撤销 (支付服务法第8条)\n\
         Lesen perkhidmatan pembayaran telah dibatalkan (PSA s. 8)"
    )]
    LicenseRevoked,

    /// License expired
    #[error(
        "Payment service license expired on {expiry_date} (PSA s. 7)\n\
         支付服务许可证已于 {expiry_date} 到期 (支付服务法第7条)\n\
         Lesen perkhidmatan pembayaran tamat tempoh pada {expiry_date} (PSA s. 7)"
    )]
    LicenseExpired { expiry_date: String },

    /// Wrong license type for volume
    #[error(
        "Monthly volume SGD {volume:.2} requires Major Payment Institution license (threshold: SGD 3,000,000) (PSA s. 5)\n\
         月交易量 {volume:.2}新元 需要主要支付机构许可证 (门槛: 300万新元) (支付服务法第5条)\n\
         Jumlah bulanan SGD {volume:.2} memerlukan lesen Institusi Pembayaran Utama (ambang: SGD 3,000,000) (PSA s. 5)"
    )]
    RequiresMpiLicense { volume: f64 },

    /// Service not authorized under license
    #[error(
        "Payment service type '{service}' not authorized under current license (PSA s. 5)\n\
         支付服务类型 '{service}' 未在当前许可证下授权 (支付服务法第5条)\n\
         Jenis perkhidmatan pembayaran '{service}' tidak dibenarkan di bawah lesen semasa (PSA s. 5)"
    )]
    UnauthorizedService { service: String },

    // ========== Safeguarding Violations (PSA s. 23) ==========
    /// Safeguarding not implemented when required
    #[error(
        "Safeguarding of customer funds required but not implemented (PSA s. 23)\n\
         需要保护客户资金但未实施 (支付服务法第23条)\n\
         Perlindungan dana pelanggan diperlukan tetapi tidak dilaksanakan (PSA s. 23)"
    )]
    SafeguardingNotImplemented,

    /// Insufficient safeguarding amount
    #[error(
        "Safeguarded amount SGD {safeguarded:.2} less than float outstanding SGD {float:.2} (PSA s. 23)\n\
         保障金额 {safeguarded:.2}新元 低于未偿付浮动金额 {float:.2}新元 (支付服务法第23条)\n\
         Jumlah dilindungi SGD {safeguarded:.2} kurang daripada apungan tertunggak SGD {float:.2} (PSA s. 23)"
    )]
    InsufficientSafeguarding { safeguarded: f64, float: f64 },

    /// Safeguarding verification overdue
    #[error(
        "Safeguarding arrangement verification overdue by {days_overdue} days (PSA s. 23)\n\
         保障安排核查逾期 {days_overdue} 天 (支付服务法第23条)\n\
         Pengesahan pengaturan perlindungan tertunggak {days_overdue} hari (PSA s. 23)"
    )]
    SafeguardingVerificationOverdue { days_overdue: i64 },

    // ========== AML/CFT Violations (PSA s. 20) ==========
    /// No AML/CFT compliance officer appointed
    #[error(
        "Payment service provider must appoint AML/CFT compliance officer (PSA s. 20)\n\
         支付服务提供商必须任命反洗钱/反恐怖融资合规官 (支付服务法第20条)\n\
         Penyedia perkhidmatan pembayaran mesti lantik pegawai pematuhan AML/CFT (PSA s. 20)"
    )]
    NoAmlOfficer,

    /// KYC not completed for customer
    #[error(
        "Know Your Customer (KYC) verification not completed for account {account_id} (PSA s. 20)\n\
         账户 {account_id} 未完成客户身份验证 (支付服务法第20条)\n\
         Pengesahan Kenali Pelanggan Anda tidak selesai untuk akaun {account_id} (PSA s. 20)"
    )]
    KycNotCompleted { account_id: String },

    /// Enhanced verification required but not performed
    #[error(
        "Enhanced verification required for account {account_id} with balance SGD {balance:.2} (threshold: SGD 5,000) (PSA Notice PSN02)\n\
         余额 {balance:.2}新元 的账户 {account_id} 需要加强验证 (门槛: 5000新元) (支付服务通知PSN02)\n\
         Pengesahan diperkukuh diperlukan untuk akaun {account_id} dengan baki SGD {balance:.2} (ambang: SGD 5,000) (Notis PSA PSN02)"
    )]
    EnhancedVerificationRequired { account_id: String, balance: f64 },

    // ========== DPT Service Violations (PSA s. 13-14) ==========
    /// Providing DPT services without authorization
    #[error(
        "Digital Payment Token (DPT) services require specific license authorization (PSA s. 13)\n\
         数字支付代币服务需要特定许可证授权 (支付服务法第13条)\n\
         Perkhidmatan Token Pembayaran Digital memerlukan kebenaran lesen khusus (PSA s. 13)"
    )]
    UnauthorizedDptService,

    /// DPT service without AML compliance
    #[error(
        "DPT service providers must implement AML/CFT measures (PSA s. 20, MAS Notice PSN02)\n\
         数字支付代币服务提供商必须实施反洗钱/反恐怖融资措施 (支付服务法第20条, 金管局通知PSN02)\n\
         Penyedia perkhidmatan DPT mesti laksanakan langkah AML/CFT (PSA s. 20, Notis MAS PSN02)"
    )]
    DptAmlNonCompliance,

    // ========== Operational Violations ==========
    /// Invalid UEN format
    #[error(
        "Invalid UEN format: {uen} (must be 9-10 digits assigned by ACRA)\n\
         无效的统一实体号码格式: {uen} (必须是会计与企业管制局分配的9-10位数字)\n\
         Format UEN tidak sah: {uen} (mesti 9-10 digit yang diberikan oleh ACRA)"
    )]
    InvalidUen { uen: String },

    /// Provider name too short
    #[error(
        "Payment service provider name must be at least 3 characters\n\
         支付服务提供商名称必须至少3个字符\n\
         Nama penyedia perkhidmatan pembayaran mesti sekurang-kurangnya 3 aksara"
    )]
    InvalidProviderName,

    /// No payment services specified
    #[error(
        "At least one payment service type must be specified (PSA s. 3)\n\
         必须指定至少一种支付服务类型 (支付服务法第3条)\n\
         Sekurang-kurangnya satu jenis perkhidmatan pembayaran mesti dinyatakan (PSA s. 3)"
    )]
    NoServicesSpecified,

    /// Excessive float without safeguarding
    #[error(
        "Float outstanding SGD {float:.2} requires safeguarding (PSA s. 23)\n\
         未偿付浮动金额 {float:.2}新元 需要保障措施 (支付服务法第23条)\n\
         Apungan tertunggak SGD {float:.2} memerlukan perlindungan (PSA s. 23)"
    )]
    ExcessiveFloatWithoutSafeguarding { float: f64 },

    /// Transaction reporting threshold not met
    #[error(
        "Transaction of SGD {amount:.2} not reported (threshold: SGD 5,000) (PSA regulations)\n\
         交易金额 {amount:.2}新元 未报告 (门槛: 5000新元) (支付服务法规)\n\
         Transaksi SGD {amount:.2} tidak dilaporkan (ambang: SGD 5,000) (peraturan PSA)"
    )]
    TransactionNotReported { amount: f64 },

    // ========== General Errors ==========
    /// Missing required field
    #[error(
        "Missing required field: {field}\n\
         缺少必填字段: {field}\n\
         Medan diperlukan hilang: {field}"
    )]
    MissingRequiredField { field: String },

    /// Invalid date
    #[error(
        "Invalid date: {description}\n\
         无效日期: {description}\n\
         Tarikh tidak sah: {description}"
    )]
    InvalidDate { description: String },

    /// Invalid amount
    #[error(
        "Invalid amount: {description}\n\
         无效金额: {description}\n\
         Jumlah tidak sah: {description}"
    )]
    InvalidAmount { description: String },
}

impl PaymentError {
    /// Get the statutory reference for this error
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            PaymentError::NoValidLicense
            | PaymentError::RequiresMpiLicense { .. }
            | PaymentError::UnauthorizedService { .. }
            | PaymentError::NoServicesSpecified => "PSA s. 5",
            PaymentError::LicenseExpired { .. } => "PSA s. 7",
            PaymentError::LicenseSuspended | PaymentError::LicenseRevoked => "PSA s. 8",
            PaymentError::UnauthorizedDptService => "PSA s. 13",
            PaymentError::NoAmlOfficer
            | PaymentError::KycNotCompleted { .. }
            | PaymentError::DptAmlNonCompliance => "PSA s. 20",
            PaymentError::SafeguardingNotImplemented
            | PaymentError::InsufficientSafeguarding { .. }
            | PaymentError::SafeguardingVerificationOverdue { .. }
            | PaymentError::ExcessiveFloatWithoutSafeguarding { .. } => "PSA s. 23",
            PaymentError::EnhancedVerificationRequired { .. } => "PSA Notice PSN02",
            PaymentError::TransactionNotReported { .. } => "PSA Regulations",
            _ => "Payment Services Act 2019",
        }
    }

    /// Get the severity level of this violation
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            PaymentError::NoValidLicense
            | PaymentError::LicenseRevoked
            | PaymentError::UnauthorizedDptService
            | PaymentError::DptAmlNonCompliance => ErrorSeverity::Critical,

            PaymentError::LicenseSuspended
            | PaymentError::SafeguardingNotImplemented
            | PaymentError::InsufficientSafeguarding { .. }
            | PaymentError::RequiresMpiLicense { .. }
            | PaymentError::NoAmlOfficer
            | PaymentError::UnauthorizedService { .. } => ErrorSeverity::High,

            PaymentError::KycNotCompleted { .. }
            | PaymentError::EnhancedVerificationRequired { .. }
            | PaymentError::SafeguardingVerificationOverdue { .. }
            | PaymentError::ExcessiveFloatWithoutSafeguarding { .. }
            | PaymentError::TransactionNotReported { .. } => ErrorSeverity::Medium,

            PaymentError::LicenseExpired { .. }
            | PaymentError::NoServicesSpecified
            | PaymentError::InvalidProviderName => ErrorSeverity::Medium,

            _ => ErrorSeverity::Low,
        }
    }
}

/// Severity level for payment service violations
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
