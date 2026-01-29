//! Indonesian Banking Law - UU No. 7/1992 as amended by UU No. 10/1998
//!
//! ## Overview
//!
//! Banking Law regulates:
//! - Commercial banks (Bank Umum)
//! - Rural banks (Bank Perkreditan Rakyat - BPR)
//! - Islamic banking (further regulated by UU 21/2008)
//!
//! ## Regulatory Authority
//!
//! - **Otoritas Jasa Keuangan (OJK)**: Financial Services Authority (since 2014)
//! - Previously regulated by Bank Indonesia (BI) - now focuses on monetary policy
//!
//! ## Key Principles
//!
//! - Banking secrecy (rahasia bank) - Pasal 40-47A
//! - Prudential banking principles
//! - Fit and proper test for bank executives
//! - Capital adequacy requirements

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of bank
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankType {
    /// Commercial bank (Bank Umum) - full banking services
    CommercialBank,
    /// Rural bank (Bank Perkreditan Rakyat) - limited services
    RuralBank,
    /// Islamic commercial bank (Bank Umum Syariah)
    IslamicCommercialBank,
    /// Islamic rural bank (Bank Pembiayaan Rakyat Syariah)
    IslamicRuralBank,
}

impl BankType {
    /// Get bank type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::CommercialBank => "Bank Umum",
            Self::RuralBank => "Bank Perkreditan Rakyat (BPR)",
            Self::IslamicCommercialBank => "Bank Umum Syariah",
            Self::IslamicRuralBank => "Bank Pembiayaan Rakyat Syariah (BPRS)",
        }
    }

    /// Get bank type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::CommercialBank => "Commercial Bank",
            Self::RuralBank => "Rural Bank (BPR)",
            Self::IslamicCommercialBank => "Islamic Commercial Bank",
            Self::IslamicRuralBank => "Islamic Rural Bank (BPRS)",
        }
    }

    /// Check if bank is Islamic
    pub fn is_islamic(&self) -> bool {
        matches!(self, Self::IslamicCommercialBank | Self::IslamicRuralBank)
    }
}

/// Bank business activities - Pasal 6
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankActivity {
    /// Collecting public funds (deposits)
    CollectingFunds,
    /// Extending credit
    ExtendingCredit,
    /// Issuing demand deposits, savings, time deposits
    IssuingDeposits,
    /// Purchasing, selling, guaranteeing government securities
    GovernmentSecurities,
    /// Purchasing temporary bank debts
    BankDebts,
    /// Providing letters of credit services
    LettersOfCredit,
    /// Issuing payment cards
    PaymentCards,
    /// Foreign exchange services
    ForeignExchange,
    /// Safe deposit box services
    SafeDepositBox,
    /// Investment banking activities
    InvestmentBanking,
}

impl BankActivity {
    /// Get activity name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::CollectingFunds => "Menghimpun dana dari masyarakat",
            Self::ExtendingCredit => "Memberikan kredit",
            Self::IssuingDeposits => "Menerbitkan giro, tabungan, deposito",
            Self::GovernmentSecurities => "Membeli, menjual, menjamin surat berharga pemerintah",
            Self::BankDebts => "Membeli surat pengakuan hutang",
            Self::LettersOfCredit => {
                "Menyediakan pembiayaan dan atau melakukan kegiatan lain berdasarkan Prinsip Syariah"
            }
            Self::PaymentCards => "Melakukan kegiatan kartu kredit dan atau kartu debit",
            Self::ForeignExchange => "Melakukan kegiatan dalam valuta asing",
            Self::SafeDepositBox => "Menyediakan tempat untuk menyimpan barang dan surat berharga",
            Self::InvestmentBanking => "Kegiatan penyertaan modal pada bank atau perusahaan lain",
        }
    }

    /// Get activity name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::CollectingFunds => "Collecting funds from the public",
            Self::ExtendingCredit => "Extending credit",
            Self::IssuingDeposits => "Issuing demand deposits, savings, time deposits",
            Self::GovernmentSecurities => "Purchasing, selling, guaranteeing government securities",
            Self::BankDebts => "Purchasing temporary bank debts",
            Self::LettersOfCredit => "Providing letters of credit services",
            Self::PaymentCards => "Issuing payment cards",
            Self::ForeignExchange => "Foreign exchange services",
            Self::SafeDepositBox => "Safe deposit box services",
            Self::InvestmentBanking => "Investment banking activities",
        }
    }
}

/// Bank ownership category - Pasal 21
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankOwnership {
    /// State-owned bank (BUMN)
    StateOwned,
    /// Regional development bank (BPD)
    RegionalDevelopment,
    /// Private national bank
    PrivateNational,
    /// Joint venture bank
    JointVenture,
    /// Foreign bank branch
    ForeignBranch,
}

impl BankOwnership {
    /// Get ownership type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::StateOwned => "Bank Milik Negara (BUMN)",
            Self::RegionalDevelopment => "Bank Pembangunan Daerah (BPD)",
            Self::PrivateNational => "Bank Swasta Nasional",
            Self::JointVenture => "Bank Campuran",
            Self::ForeignBranch => "Kantor Cabang Bank Asing",
        }
    }

    /// Get ownership type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::StateOwned => "State-Owned Bank",
            Self::RegionalDevelopment => "Regional Development Bank",
            Self::PrivateNational => "Private National Bank",
            Self::JointVenture => "Joint Venture Bank",
            Self::ForeignBranch => "Foreign Bank Branch",
        }
    }
}

/// Minimum capital requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCapital {
    /// Bank type
    pub bank_type: BankType,
    /// Minimum capital (Rupiah)
    pub minimum_capital: i64,
    /// Effective date
    pub effective_date: NaiveDate,
}

impl MinimumCapital {
    /// Get current minimum capital for commercial bank (Rp 3 trillion)
    pub fn commercial_bank_minimum() -> i64 {
        3_000_000_000_000
    }

    /// Get current minimum capital for BPR (varies by location, typically Rp 1-6 billion)
    pub fn rural_bank_minimum() -> i64 {
        1_000_000_000 // Minimum threshold
    }
}

/// Capital adequacy ratio (CAR/KPMM) - Pasal 29
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalAdequacy {
    /// Bank name
    pub bank_name: String,
    /// Tier 1 capital (modal inti)
    pub tier1_capital: i64,
    /// Tier 2 capital (modal pelengkap)
    pub tier2_capital: i64,
    /// Risk-weighted assets (ATMR)
    pub risk_weighted_assets: i64,
    /// CAR percentage
    pub car_percentage: f64,
}

impl CapitalAdequacy {
    /// Minimum CAR requirement (8% under Basel standards, OJK may set higher)
    pub fn minimum_car_percent() -> f64 {
        8.0
    }

    /// Calculate CAR
    pub fn calculate_car(total_capital: i64, risk_weighted_assets: i64) -> f64 {
        if risk_weighted_assets == 0 {
            0.0
        } else {
            (total_capital as f64 / risk_weighted_assets as f64) * 100.0
        }
    }

    /// Check if CAR meets minimum requirement
    pub fn is_adequate(&self) -> bool {
        self.car_percentage >= Self::minimum_car_percent()
    }
}

/// Credit type - Pasal 1(11)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditType {
    /// Working capital credit
    WorkingCapital,
    /// Investment credit
    Investment,
    /// Consumer credit
    Consumer,
    /// Micro credit (Kredit Usaha Rakyat - KUR)
    MicroCredit,
    /// Housing credit (Kredit Pemilikan Rumah - KPR)
    HousingCredit,
    /// Vehicle credit (Kredit Kendaraan Bermotor)
    VehicleCredit,
}

impl CreditType {
    /// Get credit type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::WorkingCapital => "Kredit Modal Kerja",
            Self::Investment => "Kredit Investasi",
            Self::Consumer => "Kredit Konsumsi",
            Self::MicroCredit => "Kredit Usaha Rakyat (KUR)",
            Self::HousingCredit => "Kredit Pemilikan Rumah (KPR)",
            Self::VehicleCredit => "Kredit Kendaraan Bermotor",
        }
    }

    /// Get credit type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::WorkingCapital => "Working Capital Credit",
            Self::Investment => "Investment Credit",
            Self::Consumer => "Consumer Credit",
            Self::MicroCredit => "Micro Credit (KUR)",
            Self::HousingCredit => "Housing Credit (KPR)",
            Self::VehicleCredit => "Vehicle Credit",
        }
    }
}

/// Credit quality classification - based on OJK regulations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditQuality {
    /// Current (Lancar) - 0 days overdue
    Current,
    /// Special mention (Dalam Perhatian Khusus) - 1-90 days overdue
    SpecialMention,
    /// Substandard (Kurang Lancar) - 91-120 days overdue
    Substandard,
    /// Doubtful (Diragukan) - 121-180 days overdue
    Doubtful,
    /// Loss (Macet) - over 180 days overdue
    Loss,
}

impl CreditQuality {
    /// Get quality name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Current => "Lancar",
            Self::SpecialMention => "Dalam Perhatian Khusus (DPK)",
            Self::Substandard => "Kurang Lancar",
            Self::Doubtful => "Diragukan",
            Self::Loss => "Macet",
        }
    }

    /// Get quality name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Current => "Current",
            Self::SpecialMention => "Special Mention",
            Self::Substandard => "Substandard",
            Self::Doubtful => "Doubtful",
            Self::Loss => "Loss",
        }
    }

    /// Check if credit is non-performing (NPL)
    pub fn is_non_performing(&self) -> bool {
        matches!(self, Self::Substandard | Self::Doubtful | Self::Loss)
    }

    /// Get provision percentage required
    pub fn provision_percentage(&self) -> f64 {
        match self {
            Self::Current => 1.0,
            Self::SpecialMention => 5.0,
            Self::Substandard => 15.0,
            Self::Doubtful => 50.0,
            Self::Loss => 100.0,
        }
    }
}

/// Banking secrecy exception - Pasal 40-44
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BankingSecrecyException {
    /// Tax purposes (with Minister of Finance authorization)
    TaxPurposes,
    /// Criminal investigation (with police authorization)
    CriminalInvestigation,
    /// Civil litigation (with court order)
    CivilLitigation,
    /// Inheritance settlement (with court order)
    InheritanceSettlement,
    /// Customer's own request
    CustomerRequest,
    /// Inter-bank information exchange
    InterbankExchange,
    /// Request by other bank for customer applying for credit
    CreditApplication,
}

impl BankingSecrecyException {
    /// Get exception description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::TaxPurposes => "Untuk kepentingan perpajakan",
            Self::CriminalInvestigation => "Untuk kepentingan penyelesaian piutang bank",
            Self::CivilLitigation => "Untuk kepentingan peradilan dalam perkara perdata",
            Self::InheritanceSettlement => "Untuk kepentingan penyelesaian warisan",
            Self::CustomerRequest => "Atas permintaan nasabah sendiri",
            Self::InterbankExchange => "Untuk tukar menukar informasi antar bank",
            Self::CreditApplication => "Untuk permohonan kredit nasabah",
        }
    }

    /// Get exception description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::TaxPurposes => "For tax purposes",
            Self::CriminalInvestigation => "For criminal investigation",
            Self::CivilLitigation => "For civil litigation",
            Self::InheritanceSettlement => "For inheritance settlement",
            Self::CustomerRequest => "At customer's own request",
            Self::InterbankExchange => "For inter-bank information exchange",
            Self::CreditApplication => "For credit application",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_type() {
        let commercial = BankType::CommercialBank;
        assert_eq!(commercial.name_id(), "Bank Umum");
        assert!(!commercial.is_islamic());

        let islamic = BankType::IslamicCommercialBank;
        assert!(islamic.is_islamic());
    }

    #[test]
    fn test_minimum_capital() {
        assert_eq!(MinimumCapital::commercial_bank_minimum(), 3_000_000_000_000);
        assert_eq!(MinimumCapital::rural_bank_minimum(), 1_000_000_000);
    }

    #[test]
    fn test_capital_adequacy() {
        let car = CapitalAdequacy::calculate_car(100_000_000, 1_000_000_000);
        assert!((car - 10.0).abs() < 0.0001);

        let car_low = CapitalAdequacy::calculate_car(70_000_000, 1_000_000_000);
        assert!((car_low - 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_car_adequacy() {
        let adequate = CapitalAdequacy {
            bank_name: "Test Bank".to_string(),
            tier1_capital: 80_000_000,
            tier2_capital: 20_000_000,
            risk_weighted_assets: 1_000_000_000,
            car_percentage: 10.0,
        };
        assert!(adequate.is_adequate());

        let inadequate = CapitalAdequacy {
            bank_name: "Test Bank 2".to_string(),
            tier1_capital: 60_000_000,
            tier2_capital: 10_000_000,
            risk_weighted_assets: 1_000_000_000,
            car_percentage: 7.0,
        };
        assert!(!inadequate.is_adequate());
    }

    #[test]
    fn test_credit_quality() {
        let current = CreditQuality::Current;
        assert!(!current.is_non_performing());
        assert_eq!(current.provision_percentage(), 1.0);

        let loss = CreditQuality::Loss;
        assert!(loss.is_non_performing());
        assert_eq!(loss.provision_percentage(), 100.0);
    }

    #[test]
    fn test_bank_ownership() {
        let state = BankOwnership::StateOwned;
        assert_eq!(state.name_id(), "Bank Milik Negara (BUMN)");
        assert_eq!(state.name_en(), "State-Owned Bank");
    }
}
