//! Banking Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍທະນາຄານ)
//!
//! Comprehensive error types for Lao banking law validation and compliance.
//! All errors include bilingual messages (Lao/English) where applicable.
//!
//! ## Legal Basis
//!
//! - **Commercial Bank Law 2006** (Law No. 03/NA, amended 2018)
//! - **Bank of Lao PDR Law 2018** (Law No. 50/NA)
//! - **AML/CFT Law 2014** (Law No. 50/NA)

use thiserror::Error;

/// Result type for banking law operations
pub type Result<T> = std::result::Result<T, BankingLawError>;

/// Banking law errors (ຄວາມຜິດພາດກົດໝາຍທະນາຄານ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum BankingLawError {
    // ========================================================================
    // License and Registration Errors (ຄວາມຜິດພາດການອະນຸຍາດແລະລົງທະບຽນ)
    // ========================================================================
    /// Insufficient minimum capital for bank type
    /// ທຶນຂັ້ນຕ່ຳບໍ່ພຽງພໍສຳລັບປະເພດທະນາຄານ
    #[error(
        "Insufficient minimum capital: {capital_lak} LAK is below required {required_lak} LAK for {bank_type}\nທຶນຂັ້ນຕ່ຳບໍ່ພຽງພໍ: {capital_lak} ກີບ ຕ່ຳກວ່າທີ່ຕ້ອງການ {required_lak} ກີບ ສຳລັບ {bank_type}"
    )]
    InsufficientCapital {
        capital_lak: u64,
        required_lak: u64,
        bank_type: String,
    },

    /// Banking license expired
    /// ໃບອະນຸຍາດທະນາຄານໝົດອາຍຸ
    #[error(
        "Banking license expired on {expiry_date} for {bank_name}\nໃບອະນຸຍາດທະນາຄານໝົດອາຍຸວັນທີ {expiry_date} ສຳລັບ {bank_name}"
    )]
    LicenseExpired {
        expiry_date: String,
        bank_name: String,
    },

    /// Banking license suspended
    /// ໃບອະນຸຍາດທະນາຄານຖືກລະງັບ
    #[error("Banking license suspended: {reason}\nໃບອະນຸຍາດທະນາຄານຖືກລະງັບ: {reason}")]
    LicenseSuspended { reason: String },

    /// Banking license revoked
    /// ໃບອະນຸຍາດທະນາຄານຖືກຖອນ
    #[error(
        "Banking license revoked: {reason} (BOL Decision No. {decision_number})\nໃບອະນຸຍາດທະນາຄານຖືກຖອນ: {reason} (ຄຳຕັດສິນທະນາຄານກາງເລກທີ {decision_number})"
    )]
    LicenseRevoked {
        reason: String,
        decision_number: String,
    },

    /// Invalid license type for activity
    /// ປະເພດໃບອະນຸຍາດບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ
    #[error(
        "Invalid license type '{license_type}' for activity '{activity}'\nປະເພດໃບອະນຸຍາດ '{license_type}' ບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ '{activity}'"
    )]
    InvalidLicenseType {
        license_type: String,
        activity: String,
    },

    /// Fit and proper test failure
    /// ບໍ່ຜ່ານການທົດສອບຄວາມເໝາະສົມ
    #[error(
        "Director '{name}' failed fit and proper test: {reason}\nກຳມະການ '{name}' ບໍ່ຜ່ານການທົດສອບຄວາມເໝາະສົມ: {reason}"
    )]
    FitAndProperFailure { name: String, reason: String },

    // ========================================================================
    // Capital Adequacy Errors (ຄວາມຜິດພາດກ່ຽວກັບຄວາມພຽງພໍຂອງທຶນ)
    // ========================================================================
    /// Capital adequacy ratio below minimum
    /// ອັດຕາສ່ວນຄວາມພຽງພໍຂອງທຶນຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "Capital adequacy ratio {car_percent}% is below minimum {min_percent}% (Basel III)\nອັດຕາສ່ວນຄວາມພຽງພໍຂອງທຶນ {car_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {min_percent}% (Basel III)"
    )]
    InsufficientCAR { car_percent: f64, min_percent: f64 },

    /// Tier 1 capital below requirement
    /// ທຶນຂັ້ນ 1 ຕ່ຳກວ່າທີ່ກຳນົດ
    #[error(
        "Tier 1 capital ratio {tier1_percent}% is below minimum {min_percent}%\nອັດຕາສ່ວນທຶນຂັ້ນ 1 {tier1_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {min_percent}%"
    )]
    InsufficientTier1Capital {
        tier1_percent: f64,
        min_percent: f64,
    },

    /// Leverage ratio below requirement
    /// ອັດຕາສ່ວນໜີ້ສິນຕ່ຳກວ່າທີ່ກຳນົດ
    #[error(
        "Leverage ratio {leverage_percent}% is below minimum {min_percent}%\nອັດຕາສ່ວນໜີ້ສິນ {leverage_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {min_percent}%"
    )]
    InsufficientLeverageRatio {
        leverage_percent: f64,
        min_percent: f64,
    },

    /// Invalid risk weight
    /// ນ້ຳໜັກຄວາມສ່ຽງບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid risk weight {weight}% for asset class '{asset_class}'\nນ້ຳໜັກຄວາມສ່ຽງ {weight}% ບໍ່ຖືກຕ້ອງສຳລັບປະເພດຊັບສິນ '{asset_class}'"
    )]
    InvalidRiskWeight { weight: f64, asset_class: String },

    // ========================================================================
    // Prudential Regulation Errors (ຄວາມຜິດພາດລະບຽບຄວາມສະຫຼາດສຸຂຸມ)
    // ========================================================================
    /// Single borrower limit exceeded
    /// ເກີນຂີດຈຳກັດຜູ້ກູ້ຢືມລາຍດຽວ
    #[error(
        "Single borrower limit exceeded: {exposure_percent}% of capital (max {max_percent}%) for borrower '{borrower}'\nເກີນຂີດຈຳກັດຜູ້ກູ້ຢືມລາຍດຽວ: {exposure_percent}% ຂອງທຶນ (ສູງສຸດ {max_percent}%) ສຳລັບຜູ້ກູ້ '{borrower}'"
    )]
    SingleBorrowerLimitExceeded {
        exposure_percent: f64,
        max_percent: f64,
        borrower: String,
    },

    /// Related party lending limit exceeded
    /// ເກີນຂີດຈຳກັດການໃຫ້ກູ້ຢືມບຸກຄົນກ່ຽວຂ້ອງ
    #[error(
        "Related party lending limit exceeded: {exposure_percent}% of capital (max {max_percent}%) for party '{party}'\nເກີນຂີດຈຳກັດການໃຫ້ກູ້ຢືມບຸກຄົນກ່ຽວຂ້ອງ: {exposure_percent}% ຂອງທຶນ (ສູງສຸດ {max_percent}%) ສຳລັບ '{party}'"
    )]
    RelatedPartyLimitExceeded {
        exposure_percent: f64,
        max_percent: f64,
        party: String,
    },

    /// Liquidity coverage ratio below minimum
    /// ອັດຕາສ່ວນຄຸ້ມຄອງສະພາບຄ່ອງຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "Liquidity coverage ratio {lcr_percent}% is below minimum {min_percent}%\nອັດຕາສ່ວນຄຸ້ມຄອງສະພາບຄ່ອງ {lcr_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {min_percent}%"
    )]
    InsufficientLCR { lcr_percent: f64, min_percent: f64 },

    /// Net stable funding ratio below minimum
    /// ອັດຕາສ່ວນແຫຼ່ງທຶນໝັ້ນຄົງສຸດທິຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "Net stable funding ratio {nsfr_percent}% is below minimum {min_percent}%\nອັດຕາສ່ວນແຫຼ່ງທຶນໝັ້ນຄົງສຸດທິ {nsfr_percent}% ຕ່ຳກວ່າຂັ້ນຕ່ຳ {min_percent}%"
    )]
    InsufficientNSFR { nsfr_percent: f64, min_percent: f64 },

    // ========================================================================
    // Deposit Protection Errors (ຄວາມຜິດພາດການປົກປ້ອງເງິນຝາກ)
    // ========================================================================
    /// Deposit not insured
    /// ເງິນຝາກບໍ່ໄດ້ປະກັນ
    #[error(
        "Deposit type '{deposit_type}' is not covered by deposit insurance scheme\nປະເພດເງິນຝາກ '{deposit_type}' ບໍ່ຢູ່ໃນແຜນປະກັນເງິນຝາກ"
    )]
    DepositNotInsured { deposit_type: String },

    /// Deposit coverage limit exceeded
    /// ເກີນຂີດຈຳກັດຄຸ້ມຄອງເງິນຝາກ
    #[error(
        "Deposit amount {amount_lak} LAK exceeds coverage limit {limit_lak} LAK per depositor\nຈຳນວນເງິນຝາກ {amount_lak} ກີບ ເກີນຂີດຈຳກັດຄຸ້ມຄອງ {limit_lak} ກີບ ຕໍ່ຜູ້ຝາກ"
    )]
    DepositCoverageLimitExceeded { amount_lak: u64, limit_lak: u64 },

    /// Invalid deposit insurance claim
    /// ການຮຽກຮ້ອງປະກັນເງິນຝາກບໍ່ຖືກຕ້ອງ
    #[error("Invalid deposit insurance claim: {reason}\nການຮຽກຮ້ອງປະກັນເງິນຝາກບໍ່ຖືກຕ້ອງ: {reason}")]
    InvalidDepositClaim { reason: String },

    // ========================================================================
    // Foreign Exchange Errors (ຄວາມຜິດພາດການແລກປ່ຽນເງິນຕາ)
    // ========================================================================
    /// Unauthorized foreign currency transaction
    /// ທຸລະກຳເງິນຕາຕ່າງປະເທດບໍ່ໄດ້ຮັບອະນຸຍາດ
    #[error(
        "Unauthorized foreign currency transaction: {description}\nທຸລະກຳເງິນຕາຕ່າງປະເທດບໍ່ໄດ້ຮັບອະນຸຍາດ: {description}"
    )]
    UnauthorizedFXTransaction { description: String },

    /// Foreign currency account violation
    /// ການລະເມີດບັນຊີເງິນຕາຕ່າງປະເທດ
    #[error(
        "Foreign currency account violation: {reason} for currency {currency}\nການລະເມີດບັນຊີເງິນຕາຕ່າງປະເທດ: {reason} ສຳລັບສະກຸນເງິນ {currency}"
    )]
    ForeignCurrencyAccountViolation { reason: String, currency: String },

    /// Capital flow control violation
    /// ການລະເມີດການຄວບຄຸມກະແສເງິນທຶນ
    #[error(
        "Capital flow control violation: {violation_type} exceeds limit of {limit_usd} USD\nການລະເມີດການຄວບຄຸມກະແສເງິນທຶນ: {violation_type} ເກີນຂີດຈຳກັດ {limit_usd} ໂດລາ"
    )]
    CapitalFlowViolation {
        violation_type: String,
        limit_usd: u64,
    },

    /// Invalid exchange rate
    /// ອັດຕາແລກປ່ຽນບໍ່ຖືກຕ້ອງ
    #[error(
        "Invalid exchange rate: {rate} for {currency_pair} (BOL rate: {bol_rate})\nອັດຕາແລກປ່ຽນບໍ່ຖືກຕ້ອງ: {rate} ສຳລັບ {currency_pair} (ອັດຕາທະນາຄານກາງ: {bol_rate})"
    )]
    InvalidExchangeRate {
        rate: f64,
        currency_pair: String,
        bol_rate: f64,
    },

    // ========================================================================
    // AML/CFT Errors (ຄວາມຜິດພາດການຕ້ານການຟອກເງິນ)
    // ========================================================================
    /// Customer due diligence failure
    /// ການກວດສອບລູກຄ້າບໍ່ຜ່ານ
    #[error(
        "Customer due diligence failure for customer '{customer}': {reason}\nການກວດສອບລູກຄ້າບໍ່ຜ່ານສຳລັບລູກຄ້າ '{customer}': {reason}"
    )]
    CDDFailure { customer: String, reason: String },

    /// Suspicious transaction not reported
    /// ບໍ່ໄດ້ລາຍງານທຸລະກຳທີ່ໜ້າສົງໄສ
    #[error(
        "Suspicious transaction not reported within {deadline_hours} hours (Transaction ID: {transaction_id})\nບໍ່ໄດ້ລາຍງານທຸລະກຳທີ່ໜ້າສົງໄສພາຍໃນ {deadline_hours} ຊົ່ວໂມງ (ລະຫັດທຸລະກຳ: {transaction_id})"
    )]
    STRNotReported {
        transaction_id: String,
        deadline_hours: u32,
    },

    /// PEP identification failure
    /// ການລະບຸຕົວບຸກຄົນການເມືອງບໍ່ຜ່ານ
    #[error(
        "Politically exposed person not identified: {pep_name}\nບໍ່ໄດ້ລະບຸຕົວບຸກຄົນການເມືອງ: {pep_name}"
    )]
    PEPNotIdentified { pep_name: String },

    /// Record keeping violation
    /// ການລະເມີດການເກັບຮັກສາບັນທຶກ
    #[error(
        "Record keeping violation: records for transaction '{transaction_id}' not retained for {required_years} years\nການລະເມີດການເກັບຮັກສາບັນທຶກ: ບັນທຶກທຸລະກຳ '{transaction_id}' ບໍ່ໄດ້ເກັບຮັກສາ {required_years} ປີ"
    )]
    RecordKeepingViolation {
        transaction_id: String,
        required_years: u32,
    },

    /// Sanctions screening failure
    /// ການກວດສອບລາຍຊື່ຄວ່ຳບາດບໍ່ຜ່ານ
    #[error(
        "Sanctions screening failure: {entity} matched against {sanctions_list}\nການກວດສອບລາຍຊື່ຄວ່ຳບາດບໍ່ຜ່ານ: {entity} ກົງກັບ {sanctions_list}"
    )]
    SanctionsScreeningFailure {
        entity: String,
        sanctions_list: String,
    },

    // ========================================================================
    // Interest Rate Errors (ຄວາມຜິດພາດອັດຕາດອກເບ້ຍ)
    // ========================================================================
    /// Lending rate exceeds maximum
    /// ອັດຕາດອກເບ້ຍກູ້ຢືມເກີນຂີດຈຳກັດ
    #[error(
        "Lending rate {rate_percent}% exceeds maximum allowed rate {max_percent}%\nອັດຕາດອກເບ້ຍກູ້ຢືມ {rate_percent}% ເກີນອັດຕາສູງສຸດທີ່ອະນຸຍາດ {max_percent}%"
    )]
    LendingRateExceeded { rate_percent: f64, max_percent: f64 },

    /// Deposit rate below floor
    /// ອັດຕາດອກເບ້ຍເງິນຝາກຕ່ຳກວ່າຂັ້ນຕ່ຳ
    #[error(
        "Deposit rate {rate_percent}% is below floor rate {floor_percent}%\nອັດຕາດອກເບ້ຍເງິນຝາກ {rate_percent}% ຕ່ຳກວ່າອັດຕາຂັ້ນຕ່ຳ {floor_percent}%"
    )]
    DepositRateBelowFloor {
        rate_percent: f64,
        floor_percent: f64,
    },

    /// Usury rate detected
    /// ກວດພົບອັດຕາດອກເບ້ຍເກີນຂອບເຂດ
    #[error(
        "Usury detected: lending rate {rate_percent}% is considered usurious (Article 82, Commercial Bank Law)\nກວດພົບດອກເບ້ຍເກີນຂອບເຂດ: ອັດຕາ {rate_percent}% ຖືວ່າເປັນການກູ້ຢືມເກີນຂອບເຂດ (ມາດຕາ 82, ກົດໝາຍທະນາຄານການຄ້າ)"
    )]
    UsuryRateDetected { rate_percent: f64 },

    // ========================================================================
    // Payment System Errors (ຄວາມຜິດພາດລະບົບການຊຳລະເງິນ)
    // ========================================================================
    /// RTGS transaction failure
    /// ການໂອນ RTGS ລົ້ມເຫຼວ
    #[error(
        "RTGS transaction failed: {reason} (Reference: {reference})\nການໂອນ RTGS ລົ້ມເຫຼວ: {reason} (ເລກອ້າງອິງ: {reference})"
    )]
    RTGSTransactionFailure { reason: String, reference: String },

    /// Unauthorized payment service provider
    /// ຜູ້ໃຫ້ບໍລິການຊຳລະເງິນບໍ່ໄດ້ຮັບອະນຸຍາດ
    #[error(
        "Unauthorized payment service provider: {provider}\nຜູ້ໃຫ້ບໍລິການຊຳລະເງິນບໍ່ໄດ້ຮັບອະນຸຍາດ: {provider}"
    )]
    UnauthorizedPaymentProvider { provider: String },

    /// Mobile banking compliance failure
    /// ການບໍລິການທະນາຄານມືຖືບໍ່ຜ່ານເງື່ອນໄຂ
    #[error(
        "Mobile banking compliance failure: {requirement}\nການບໍລິການທະນາຄານມືຖືບໍ່ຜ່ານເງື່ອນໄຂ: {requirement}"
    )]
    MobileBankingComplianceFailure { requirement: String },

    /// Interbank clearing failure
    /// ການຊຳລະລາຍການລະຫວ່າງທະນາຄານລົ້ມເຫຼວ
    #[error("Interbank clearing failed: {reason}\nການຊຳລະລາຍການລະຫວ່າງທະນາຄານລົ້ມເຫຼວ: {reason}")]
    InterbankClearingFailure { reason: String },

    // ========================================================================
    // BOL Supervision Errors (ຄວາມຜິດພາດການກຳກັບຂອງທະນາຄານກາງ)
    // ========================================================================
    /// BOL reporting deadline missed
    /// ພາດກຳນົດການລາຍງານຕໍ່ທະນາຄານກາງ
    #[error(
        "BOL reporting deadline missed: {report_type} was due on {due_date}\nພາດກຳນົດການລາຍງານຕໍ່ທະນາຄານກາງ: {report_type} ກຳນົດວັນທີ {due_date}"
    )]
    BOLReportingDeadlineMissed {
        report_type: String,
        due_date: String,
    },

    /// Reserve requirement not met
    /// ບໍ່ໄດ້ປະຕິບັດຕາມເງື່ອນໄຂສະຫງວນ
    #[error(
        "Reserve requirement not met: {reserve_percent}% held vs {required_percent}% required\nບໍ່ໄດ້ປະຕິບັດຕາມເງື່ອນໄຂສະຫງວນ: ຖື {reserve_percent}% ແຕ່ຕ້ອງການ {required_percent}%"
    )]
    ReserveRequirementNotMet {
        reserve_percent: f64,
        required_percent: f64,
    },

    /// BOL directive violation
    /// ການລະເມີດຄຳສັ່ງທະນາຄານກາງ
    #[error(
        "BOL directive violation: Directive No. {directive_number} - {violation}\nການລະເມີດຄຳສັ່ງທະນາຄານກາງ: ຄຳສັ່ງເລກທີ {directive_number} - {violation}"
    )]
    BOLDirectiveViolation {
        directive_number: String,
        violation: String,
    },

    // ========================================================================
    // General Errors (ຄວາມຜິດພາດທົ່ວໄປ)
    // ========================================================================
    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// Missing required field
    /// ຂາດຂໍ້ມູນທີ່ຕ້ອງການ
    #[error("Missing required field: {field_name}\nຂາດຂໍ້ມູນທີ່ຕ້ອງການ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date
    /// ວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date: {description}\nວັນທີບໍ່ຖືກຕ້ອງ: {description}")]
    InvalidDate { description: String },

    /// Banking law violation
    /// ການລະເມີດກົດໝາຍທະນາຄານ
    #[error(
        "Banking law violation: {violation} ({law_reference})\nການລະເມີດກົດໝາຍທະນາຄານ: {violation} ({law_reference})"
    )]
    BankingLawViolation {
        violation: String,
        law_reference: String,
    },
}

impl BankingLawError {
    /// Get the error message in Lao language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        let full_msg = format!("{}", self);
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
        if let Some((english, _lao)) = full_msg.split_once('\n') {
            english.to_string()
        } else {
            full_msg
        }
    }

    /// Check if this is a critical violation requiring immediate action
    /// ກວດສອບວ່າເປັນການລະເມີດຮ້າຍແຮງທີ່ຕ້ອງແກ້ໄຂທັນທີ
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            BankingLawError::LicenseRevoked { .. }
                | BankingLawError::InsufficientCAR { .. }
                | BankingLawError::SanctionsScreeningFailure { .. }
                | BankingLawError::STRNotReported { .. }
                | BankingLawError::BankingLawViolation { .. }
        )
    }

    /// Check if this error may result in BOL sanctions
    /// ກວດສອບວ່າອາດຖືກທະນາຄານກາງລົງໂທດ
    pub fn may_result_in_sanctions(&self) -> bool {
        matches!(
            self,
            BankingLawError::InsufficientCapital { .. }
                | BankingLawError::InsufficientCAR { .. }
                | BankingLawError::SingleBorrowerLimitExceeded { .. }
                | BankingLawError::CDDFailure { .. }
                | BankingLawError::STRNotReported { .. }
                | BankingLawError::ReserveRequirementNotMet { .. }
        )
    }

    /// Get the relevant law reference for this error
    /// ຮັບການອ້າງອິງກົດໝາຍທີ່ກ່ຽວຂ້ອງ
    pub fn law_reference(&self) -> Option<String> {
        match self {
            BankingLawError::InsufficientCapital { .. } => {
                Some("Commercial Bank Law 2006 (amended 2018), Article 15".to_string())
            }
            BankingLawError::InsufficientCAR { .. } => {
                Some("BOL Directive on Capital Adequacy, Basel III".to_string())
            }
            BankingLawError::SingleBorrowerLimitExceeded { .. } => {
                Some("Commercial Bank Law 2006, Article 45".to_string())
            }
            BankingLawError::RelatedPartyLimitExceeded { .. } => {
                Some("Commercial Bank Law 2006, Article 46".to_string())
            }
            BankingLawError::CDDFailure { .. } | BankingLawError::STRNotReported { .. } => {
                Some("AML/CFT Law 2014, Articles 8-12".to_string())
            }
            BankingLawError::RecordKeepingViolation { .. } => {
                Some("AML/CFT Law 2014, Article 17".to_string())
            }
            BankingLawError::UsuryRateDetected { .. } => {
                Some("Commercial Bank Law 2006, Article 82".to_string())
            }
            BankingLawError::BankingLawViolation { law_reference, .. } => {
                Some(law_reference.clone())
            }
            _ => None,
        }
    }

    /// Check if this error requires BOL notification
    /// ກວດສອບວ່າຕ້ອງແຈ້ງທະນາຄານກາງ
    pub fn requires_bol_notification(&self) -> bool {
        matches!(
            self,
            BankingLawError::InsufficientCAR { .. }
                | BankingLawError::InsufficientLCR { .. }
                | BankingLawError::InsufficientNSFR { .. }
                | BankingLawError::SingleBorrowerLimitExceeded { .. }
                | BankingLawError::STRNotReported { .. }
                | BankingLawError::SanctionsScreeningFailure { .. }
                | BankingLawError::ReserveRequirementNotMet { .. }
        )
    }

    /// Create a validation error with bilingual message
    pub fn validation(en: impl Into<String>, lo: impl Into<String>) -> Self {
        Self::ValidationError {
            message: format!("{} / {}", en.into(), lo.into()),
        }
    }

    /// Create a banking law violation error
    pub fn violation(
        violation_en: impl Into<String>,
        violation_lo: impl Into<String>,
        law_reference: impl Into<String>,
    ) -> Self {
        Self::BankingLawViolation {
            violation: format!("{} / {}", violation_en.into(), violation_lo.into()),
            law_reference: law_reference.into(),
        }
    }

    /// Get the error category
    /// ຮັບປະເພດຄວາມຜິດພາດ
    pub fn category(&self) -> &'static str {
        match self {
            BankingLawError::InsufficientCapital { .. }
            | BankingLawError::LicenseExpired { .. }
            | BankingLawError::LicenseSuspended { .. }
            | BankingLawError::LicenseRevoked { .. }
            | BankingLawError::InvalidLicenseType { .. }
            | BankingLawError::FitAndProperFailure { .. } => "License and Registration",

            BankingLawError::InsufficientCAR { .. }
            | BankingLawError::InsufficientTier1Capital { .. }
            | BankingLawError::InsufficientLeverageRatio { .. }
            | BankingLawError::InvalidRiskWeight { .. } => "Capital Adequacy",

            BankingLawError::SingleBorrowerLimitExceeded { .. }
            | BankingLawError::RelatedPartyLimitExceeded { .. }
            | BankingLawError::InsufficientLCR { .. }
            | BankingLawError::InsufficientNSFR { .. } => "Prudential Regulations",

            BankingLawError::DepositNotInsured { .. }
            | BankingLawError::DepositCoverageLimitExceeded { .. }
            | BankingLawError::InvalidDepositClaim { .. } => "Deposit Protection",

            BankingLawError::UnauthorizedFXTransaction { .. }
            | BankingLawError::ForeignCurrencyAccountViolation { .. }
            | BankingLawError::CapitalFlowViolation { .. }
            | BankingLawError::InvalidExchangeRate { .. } => "Foreign Exchange",

            BankingLawError::CDDFailure { .. }
            | BankingLawError::STRNotReported { .. }
            | BankingLawError::PEPNotIdentified { .. }
            | BankingLawError::RecordKeepingViolation { .. }
            | BankingLawError::SanctionsScreeningFailure { .. } => "AML/CFT",

            BankingLawError::LendingRateExceeded { .. }
            | BankingLawError::DepositRateBelowFloor { .. }
            | BankingLawError::UsuryRateDetected { .. } => "Interest Rate Regulations",

            BankingLawError::RTGSTransactionFailure { .. }
            | BankingLawError::UnauthorizedPaymentProvider { .. }
            | BankingLawError::MobileBankingComplianceFailure { .. }
            | BankingLawError::InterbankClearingFailure { .. } => "Payment Systems",

            BankingLawError::BOLReportingDeadlineMissed { .. }
            | BankingLawError::ReserveRequirementNotMet { .. }
            | BankingLawError::BOLDirectiveViolation { .. } => "BOL Supervision",

            BankingLawError::ValidationError { .. }
            | BankingLawError::MissingRequiredField { .. }
            | BankingLawError::InvalidDate { .. }
            | BankingLawError::BankingLawViolation { .. } => "General",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = BankingLawError::InsufficientCapital {
            capital_lak: 100_000_000_000,
            required_lak: 300_000_000_000,
            bank_type: "Commercial Bank".to_string(),
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Insufficient minimum capital"));
        assert!(lao.contains("ທຶນຂັ້ນຕ່ຳບໍ່ພຽງພໍ"));
    }

    #[test]
    fn test_critical_violations() {
        let car_error = BankingLawError::InsufficientCAR {
            car_percent: 6.5,
            min_percent: 8.0,
        };
        assert!(car_error.is_critical());

        let license_revoked = BankingLawError::LicenseRevoked {
            reason: "Fraud".to_string(),
            decision_number: "BOL-2024-001".to_string(),
        };
        assert!(license_revoked.is_critical());

        let lending_rate = BankingLawError::LendingRateExceeded {
            rate_percent: 25.0,
            max_percent: 20.0,
        };
        assert!(!lending_rate.is_critical());
    }

    #[test]
    fn test_law_references() {
        let capital_error = BankingLawError::InsufficientCapital {
            capital_lak: 100_000_000_000,
            required_lak: 300_000_000_000,
            bank_type: "Commercial Bank".to_string(),
        };
        assert!(capital_error.law_reference().is_some());

        let aml_error = BankingLawError::CDDFailure {
            customer: "Test Customer".to_string(),
            reason: "Missing ID".to_string(),
        };
        assert!(aml_error.law_reference().is_some());
    }

    #[test]
    fn test_bol_notification_required() {
        let car_error = BankingLawError::InsufficientCAR {
            car_percent: 6.5,
            min_percent: 8.0,
        };
        assert!(car_error.requires_bol_notification());

        let str_error = BankingLawError::STRNotReported {
            transaction_id: "TXN-001".to_string(),
            deadline_hours: 24,
        };
        assert!(str_error.requires_bol_notification());
    }

    #[test]
    fn test_may_result_in_sanctions() {
        let capital_error = BankingLawError::InsufficientCapital {
            capital_lak: 100_000_000_000,
            required_lak: 300_000_000_000,
            bank_type: "Commercial Bank".to_string(),
        };
        assert!(capital_error.may_result_in_sanctions());

        let cdd_error = BankingLawError::CDDFailure {
            customer: "Test".to_string(),
            reason: "Missing docs".to_string(),
        };
        assert!(cdd_error.may_result_in_sanctions());
    }

    #[test]
    fn test_error_categories() {
        let license_error = BankingLawError::LicenseExpired {
            expiry_date: "2024-01-01".to_string(),
            bank_name: "Test Bank".to_string(),
        };
        assert_eq!(license_error.category(), "License and Registration");

        let car_error = BankingLawError::InsufficientCAR {
            car_percent: 6.5,
            min_percent: 8.0,
        };
        assert_eq!(car_error.category(), "Capital Adequacy");

        let aml_error = BankingLawError::CDDFailure {
            customer: "Test".to_string(),
            reason: "Missing ID".to_string(),
        };
        assert_eq!(aml_error.category(), "AML/CFT");

        let fx_error = BankingLawError::UnauthorizedFXTransaction {
            description: "Test".to_string(),
        };
        assert_eq!(fx_error.category(), "Foreign Exchange");

        let payment_error = BankingLawError::RTGSTransactionFailure {
            reason: "Timeout".to_string(),
            reference: "REF-001".to_string(),
        };
        assert_eq!(payment_error.category(), "Payment Systems");
    }

    #[test]
    fn test_validation_helper() {
        let error = BankingLawError::validation("Invalid input", "ຂໍ້ມູນບໍ່ຖືກຕ້ອງ");
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid input"));
        assert!(msg.contains("ຂໍ້ມູນບໍ່ຖືກຕ້ອງ"));
    }

    #[test]
    fn test_violation_helper() {
        let error = BankingLawError::violation(
            "Unauthorized lending",
            "ການໃຫ້ກູ້ຢືມບໍ່ໄດ້ຮັບອະນຸຍາດ",
            "Commercial Bank Law 2006, Article 40",
        );
        let msg = format!("{}", error);
        assert!(msg.contains("Unauthorized lending"));
        assert!(msg.contains("Article 40"));
    }

    #[test]
    fn test_single_borrower_limit_error() {
        let error = BankingLawError::SingleBorrowerLimitExceeded {
            exposure_percent: 30.0,
            max_percent: 25.0,
            borrower: "ABC Company".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("30%"));
        assert!(msg.contains("25%"));
        assert!(msg.contains("ABC Company"));
    }

    #[test]
    fn test_related_party_limit_error() {
        let error = BankingLawError::RelatedPartyLimitExceeded {
            exposure_percent: 20.0,
            max_percent: 15.0,
            party: "Board Director".to_string(),
        };
        assert!(error.law_reference().is_some());
        assert_eq!(error.category(), "Prudential Regulations");
    }

    #[test]
    fn test_deposit_protection_errors() {
        let not_insured = BankingLawError::DepositNotInsured {
            deposit_type: "Foreign Currency Deposit".to_string(),
        };
        assert_eq!(not_insured.category(), "Deposit Protection");

        let exceeded = BankingLawError::DepositCoverageLimitExceeded {
            amount_lak: 100_000_000,
            limit_lak: 50_000_000,
        };
        let msg = format!("{}", exceeded);
        assert!(msg.contains("100000000"));
        assert!(msg.contains("50000000"));
    }

    #[test]
    fn test_interest_rate_errors() {
        let usury = BankingLawError::UsuryRateDetected { rate_percent: 50.0 };
        assert!(usury.law_reference().is_some());
        assert_eq!(usury.category(), "Interest Rate Regulations");

        let lending = BankingLawError::LendingRateExceeded {
            rate_percent: 25.0,
            max_percent: 20.0,
        };
        let msg = format!("{}", lending);
        assert!(msg.contains("25%"));
        assert!(msg.contains("20%"));
    }

    #[test]
    fn test_bol_supervision_errors() {
        let reporting = BankingLawError::BOLReportingDeadlineMissed {
            report_type: "Monthly Capital Adequacy".to_string(),
            due_date: "2024-01-15".to_string(),
        };
        assert_eq!(reporting.category(), "BOL Supervision");

        let reserve = BankingLawError::ReserveRequirementNotMet {
            reserve_percent: 4.0,
            required_percent: 5.0,
        };
        assert!(reserve.requires_bol_notification());
    }

    #[test]
    fn test_aml_errors() {
        let pep_error = BankingLawError::PEPNotIdentified {
            pep_name: "John Doe".to_string(),
        };
        assert_eq!(pep_error.category(), "AML/CFT");

        let sanctions = BankingLawError::SanctionsScreeningFailure {
            entity: "Test Entity".to_string(),
            sanctions_list: "UN Sanctions List".to_string(),
        };
        assert!(sanctions.is_critical());
    }

    #[test]
    fn test_fx_errors() {
        let fx_error = BankingLawError::InvalidExchangeRate {
            rate: 25000.0,
            currency_pair: "USD/LAK".to_string(),
            bol_rate: 20000.0,
        };
        let msg = format!("{}", fx_error);
        assert!(msg.contains("25000"));
        assert!(msg.contains("USD/LAK"));
        assert!(msg.contains("20000"));
    }
}
