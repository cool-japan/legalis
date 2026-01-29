//! Indonesian Capital Markets Law - UU No. 8/1995
//!
//! ## Overview
//!
//! Law No. 8 of 1995 on Capital Markets (Pasar Modal) regulates:
//! - Securities trading and issuance
//! - Stock exchanges (Bursa Efek Indonesia - BEI/IDX)
//! - Public offerings (IPO)
//! - Market participants and intermediaries
//!
//! ## Regulatory Authority
//!
//! - **Otoritas Jasa Keuangan (OJK)**: Financial Services Authority
//! - Previously regulated by Bapepam-LK (Capital Market and Financial Institution Supervisory Agency)
//!
//! ## Key Concepts
//!
//! - **Efek (Securities)**: Shares, bonds, mutual funds, derivatives
//! - **Emiten (Issuer)**: Company issuing securities
//! - **Perusahaan Publik (Public Company)**: Company with 300+ shareholders or Rp 3B+ paid-up capital
//! - **Penawaran Umum (Public Offering)**: Securities offering to public

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of security (Efek) - Pasal 1(5)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityType {
    /// Shares (saham)
    Shares,
    /// Bonds (obligasi)
    Bonds,
    /// Sukuk (Islamic bonds)
    Sukuk,
    /// Mutual fund units (unit penyertaan reksa dana)
    MutualFundUnits,
    /// Derivatives (kontrak berjangka, opsi)
    Derivatives,
    /// Asset-backed securities (efek beragun aset)
    AssetBackedSecurities,
    /// Real estate investment trust units (unit penyertaan dana investasi real estat)
    ReitUnits,
    /// Other securities
    Other(String),
}

impl SecurityType {
    /// Get security type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Shares => "Saham",
            Self::Bonds => "Obligasi",
            Self::Sukuk => "Sukuk (Obligasi Syariah)",
            Self::MutualFundUnits => "Unit Penyertaan Reksa Dana",
            Self::Derivatives => "Kontrak Derivatif",
            Self::AssetBackedSecurities => "Efek Beragun Aset",
            Self::ReitUnits => "Unit Penyertaan Dana Investasi Real Estat (DIRE)",
            Self::Other(name) => name,
        }
    }

    /// Get security type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Shares => "Shares",
            Self::Bonds => "Bonds",
            Self::Sukuk => "Sukuk (Islamic Bonds)",
            Self::MutualFundUnits => "Mutual Fund Units",
            Self::Derivatives => "Derivatives",
            Self::AssetBackedSecurities => "Asset-Backed Securities",
            Self::ReitUnits => "Real Estate Investment Trust Units",
            Self::Other(name) => name,
        }
    }
}

/// Public company criteria - Pasal 1(22)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicCompanyCriteria {
    /// Number of shareholders
    pub num_shareholders: u32,
    /// Paid-up capital (Rupiah)
    pub paid_up_capital: i64,
}

impl PublicCompanyCriteria {
    /// Minimum shareholders for public company status
    pub fn minimum_shareholders() -> u32 {
        300
    }

    /// Minimum paid-up capital for public company status (Rp 3 billion)
    pub fn minimum_paid_up_capital() -> i64 {
        3_000_000_000
    }

    /// Check if company meets public company criteria
    pub fn is_public_company(&self) -> bool {
        self.num_shareholders >= Self::minimum_shareholders()
            || self.paid_up_capital >= Self::minimum_paid_up_capital()
    }
}

/// Type of public offering - Pasal 1(15)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicOfferingType {
    /// Initial Public Offering (IPO) - Penawaran Umum Perdana
    InitialPublicOffering,
    /// Seasoned offering - Additional shares by listed company
    SeasonedOffering,
    /// Rights issue (Hak Memesan Efek Terlebih Dahulu - HMETD)
    RightsIssue,
    /// Private placement
    PrivatePlacement,
    /// Bond issuance
    BondIssuance,
}

impl PublicOfferingType {
    /// Get offering type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::InitialPublicOffering => "Penawaran Umum Perdana (IPO)",
            Self::SeasonedOffering => "Penawaran Umum Terbatas",
            Self::RightsIssue => "Hak Memesan Efek Terlebih Dahulu (HMETD)",
            Self::PrivatePlacement => "Penempatan Terbatas",
            Self::BondIssuance => "Penawaran Umum Obligasi",
        }
    }

    /// Get offering type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::InitialPublicOffering => "Initial Public Offering (IPO)",
            Self::SeasonedOffering => "Seasoned Equity Offering",
            Self::RightsIssue => "Rights Issue",
            Self::PrivatePlacement => "Private Placement",
            Self::BondIssuance => "Bond Offering",
        }
    }
}

/// Market participant type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketParticipant {
    /// Securities company (perusahaan efek) - Pasal 1(21)
    SecuritiesCompany,
    /// Investment manager (manajer investasi)
    InvestmentManager,
    /// Underwriter (penjamin emisi)
    Underwriter,
    /// Broker-dealer (perantara pedagang efek)
    BrokerDealer,
    /// Custodian bank (bank kustodian)
    CustodianBank,
    /// Securities administration bureau (biro administrasi efek)
    SecuritiesAdministrationBureau,
    /// Credit rating agency (pemeringkat efek)
    CreditRatingAgency,
}

impl MarketParticipant {
    /// Get participant type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::SecuritiesCompany => "Perusahaan Efek",
            Self::InvestmentManager => "Manajer Investasi",
            Self::Underwriter => "Penjamin Emisi Efek",
            Self::BrokerDealer => "Perantara Pedagang Efek",
            Self::CustodianBank => "Bank Kustodian",
            Self::SecuritiesAdministrationBureau => "Biro Administrasi Efek",
            Self::CreditRatingAgency => "Lembaga Pemeringkat Efek",
        }
    }

    /// Get participant type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::SecuritiesCompany => "Securities Company",
            Self::InvestmentManager => "Investment Manager",
            Self::Underwriter => "Underwriter",
            Self::BrokerDealer => "Broker-Dealer",
            Self::CustodianBank => "Custodian Bank",
            Self::SecuritiesAdministrationBureau => "Securities Administration Bureau",
            Self::CreditRatingAgency => "Credit Rating Agency",
        }
    }

    /// Check if participant requires OJK license
    pub fn requires_ojk_license(&self) -> bool {
        true // All market participants require OJK license
    }
}

/// Stock exchange board classification (BEI)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExchangeBoard {
    /// Main Board - established companies
    MainBoard,
    /// Development Board - growing companies
    DevelopmentBoard,
    /// Acceleration Board - SMEs and startups
    AccelerationBoard,
}

impl ExchangeBoard {
    /// Get board name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::MainBoard => "Papan Utama",
            Self::DevelopmentBoard => "Papan Pengembangan",
            Self::AccelerationBoard => "Papan Akselerasi",
        }
    }

    /// Get board name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::MainBoard => "Main Board",
            Self::DevelopmentBoard => "Development Board",
            Self::AccelerationBoard => "Acceleration Board",
        }
    }

    /// Get minimum paid-up capital requirement (approximate)
    pub fn minimum_paid_up_capital(&self) -> i64 {
        match self {
            Self::MainBoard => 100_000_000_000,       // Rp 100 billion
            Self::DevelopmentBoard => 50_000_000_000, // Rp 50 billion
            Self::AccelerationBoard => 5_000_000_000, // Rp 5 billion
        }
    }

    /// Get minimum public shareholding percentage
    pub fn minimum_public_shareholding_percent(&self) -> f64 {
        match self {
            Self::MainBoard => 7.5,
            Self::DevelopmentBoard => 7.5,
            Self::AccelerationBoard => 10.0,
        }
    }
}

/// Issuer record (Emiten)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issuer {
    /// Company name
    pub name: String,
    /// Stock code (ticker symbol)
    pub ticker: Option<String>,
    /// Registration number
    pub registration_number: String,
    /// OJK registration date
    pub ojk_registration_date: Option<NaiveDate>,
    /// Listing date (if listed)
    pub listing_date: Option<NaiveDate>,
    /// Exchange board (if listed)
    pub exchange_board: Option<ExchangeBoard>,
    /// Paid-up capital
    pub paid_up_capital: i64,
    /// Number of listed shares
    pub listed_shares: Option<u64>,
    /// Market capitalization
    pub market_cap: Option<i64>,
    /// Industry sector
    pub sector: String,
}

impl Issuer {
    /// Check if company is listed on exchange
    pub fn is_listed(&self) -> bool {
        self.listing_date.is_some() && self.ticker.is_some()
    }

    /// Calculate public shareholding percentage
    pub fn public_shareholding_percent(&self, public_shares: u64) -> Option<f64> {
        self.listed_shares.map(|total| {
            if total == 0 {
                0.0
            } else {
                (public_shares as f64 / total as f64) * 100.0
            }
        })
    }
}

/// Material transaction requiring disclosure - Pasal 82-86
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialTransaction {
    /// Merger or consolidation
    Merger,
    /// Acquisition of shares (>20%)
    Acquisition { percentage: u8 },
    /// Asset purchase/sale (>20% of assets)
    AssetTransaction { percentage: u8 },
    /// Related party transaction
    RelatedPartyTransaction,
    /// Change in business activities
    BusinessActivityChange,
    /// Bankruptcy or insolvency
    BankruptcyProceedings,
}

impl MaterialTransaction {
    /// Check if transaction requires shareholder approval
    pub fn requires_shareholder_approval(&self) -> bool {
        match self {
            Self::Merger => true,
            Self::Acquisition { percentage } => *percentage >= 50,
            Self::AssetTransaction { percentage } => *percentage >= 50,
            Self::RelatedPartyTransaction => true,
            Self::BusinessActivityChange => true,
            Self::BankruptcyProceedings => false,
        }
    }

    /// Get transaction type description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::Merger => "Penggabungan, Peleburan, atau Pengambilalihan",
            Self::Acquisition { .. } => "Pembelian atau Penjualan Saham Perusahaan",
            Self::AssetTransaction { .. } => "Pembelian atau Penjualan Aset",
            Self::RelatedPartyTransaction => "Transaksi Material dengan Pihak Afiliasi",
            Self::BusinessActivityChange => "Perubahan Kegiatan Usaha Utama",
            Self::BankruptcyProceedings => "Kepailitan atau Penundaan Kewajiban Pembayaran Utang",
        }
    }

    /// Get transaction type description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::Merger => "Merger, Consolidation, or Acquisition",
            Self::Acquisition { .. } => "Share Purchase or Sale",
            Self::AssetTransaction { .. } => "Asset Purchase or Sale",
            Self::RelatedPartyTransaction => "Material Related Party Transaction",
            Self::BusinessActivityChange => "Change in Main Business Activities",
            Self::BankruptcyProceedings => "Bankruptcy or Suspension of Payment Obligations",
        }
    }
}

/// Insider trading prohibition - Pasal 95-97
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTradingViolation {
    /// Person/entity involved
    pub party_name: String,
    /// Whether party is insider (director, commissioner, major shareholder)
    pub is_insider: bool,
    /// Material non-public information used
    pub material_info: String,
    /// Transaction date
    pub transaction_date: NaiveDate,
    /// Securities traded
    pub security_type: SecurityType,
    /// Volume traded
    pub volume: u64,
    /// Profit or loss from transaction
    pub profit_loss: i64,
}

/// Market manipulation prohibition - Pasal 91-94
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketManipulation {
    /// False or misleading statements
    FalseStatements,
    /// Price manipulation
    PriceManipulation,
    /// Wash trading (self-dealing)
    WashTrading,
    /// Pump and dump schemes
    PumpAndDump,
    /// Cornering the market
    Cornering,
}

impl MarketManipulation {
    /// Get manipulation type description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::FalseStatements => "Informasi Palsu atau Menyesatkan",
            Self::PriceManipulation => "Manipulasi Harga",
            Self::WashTrading => "Transaksi Semu",
            Self::PumpAndDump => "Skema Pump and Dump",
            Self::Cornering => "Penguasaan Pasar",
        }
    }

    /// Get manipulation type description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::FalseStatements => "False or Misleading Statements",
            Self::PriceManipulation => "Price Manipulation",
            Self::WashTrading => "Wash Trading",
            Self::PumpAndDump => "Pump and Dump Scheme",
            Self::Cornering => "Cornering the Market",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_type() {
        let shares = SecurityType::Shares;
        assert_eq!(shares.name_id(), "Saham");
        assert_eq!(shares.name_en(), "Shares");
    }

    #[test]
    fn test_public_company_criteria() {
        let criteria = PublicCompanyCriteria {
            num_shareholders: 350,
            paid_up_capital: 2_000_000_000,
        };
        assert!(criteria.is_public_company()); // 350 shareholders > 300

        let criteria2 = PublicCompanyCriteria {
            num_shareholders: 200,
            paid_up_capital: 5_000_000_000,
        };
        assert!(criteria2.is_public_company()); // Capital > Rp 3B

        let criteria3 = PublicCompanyCriteria {
            num_shareholders: 100,
            paid_up_capital: 1_000_000_000,
        };
        assert!(!criteria3.is_public_company());
    }

    #[test]
    fn test_exchange_board() {
        let main = ExchangeBoard::MainBoard;
        assert_eq!(main.minimum_paid_up_capital(), 100_000_000_000);
        assert_eq!(main.minimum_public_shareholding_percent(), 7.5);

        let accel = ExchangeBoard::AccelerationBoard;
        assert_eq!(accel.minimum_public_shareholding_percent(), 10.0);
    }

    #[test]
    fn test_material_transaction_approval() {
        let merger = MaterialTransaction::Merger;
        assert!(merger.requires_shareholder_approval());

        let small_acquisition = MaterialTransaction::Acquisition { percentage: 30 };
        assert!(!small_acquisition.requires_shareholder_approval());

        let large_acquisition = MaterialTransaction::Acquisition { percentage: 60 };
        assert!(large_acquisition.requires_shareholder_approval());
    }

    #[test]
    fn test_market_participant() {
        let broker = MarketParticipant::BrokerDealer;
        assert!(broker.requires_ojk_license());
        assert_eq!(broker.name_id(), "Perantara Pedagang Efek");
    }

    #[test]
    fn test_public_offering_type() {
        let ipo = PublicOfferingType::InitialPublicOffering;
        assert_eq!(ipo.name_id(), "Penawaran Umum Perdana (IPO)");
        assert_eq!(ipo.name_en(), "Initial Public Offering (IPO)");
    }
}
