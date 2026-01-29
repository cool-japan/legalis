//! Value Added Tax (PPN - Pajak Pertambahan Nilai)
//!
//! ## Overview
//!
//! VAT in Indonesia is governed by:
//! - UU No. 8/1983 as last amended by UU No. 7/2021 (UU HPP - Harmonization of Tax Regulations)
//! - Standard rate: 11% (2022+), increased to 12% from 2025
//! - Luxury goods tax (PPnBM) ranges from 10% to 125%
//!
//! ## Taxable Transactions
//!
//! - Delivery of taxable goods (BKP) in customs area
//! - Import of taxable goods
//! - Delivery of taxable services (JKP)
//! - Utilization of intangible taxable goods from outside customs area
//! - Utilization of taxable services from outside customs area
//! - Export of taxable goods (0% rate)

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// VAT rate applicable to transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VatRate {
    /// Standard rate - 11% (2022-2024)
    Standard11Percent,
    /// Standard rate - 12% (2025+)
    Standard12Percent,
    /// Export rate - 0%
    Export,
    /// Exempt
    Exempt,
    /// Final VAT (certain sectors)
    Final { rate_percent: f64 },
}

impl VatRate {
    /// Get applicable VAT rate for given date
    pub fn standard_rate_for_date(date: NaiveDate) -> Self {
        if date.year() >= 2025 {
            Self::Standard12Percent
        } else {
            Self::Standard11Percent // 11% from 2022 onwards (was 10% before 2022)
        }
    }

    /// Get rate as decimal (e.g., 0.11 for 11%)
    pub fn rate_decimal(&self) -> f64 {
        match self {
            Self::Standard11Percent => 0.11,
            Self::Standard12Percent => 0.12,
            Self::Export => 0.0,
            Self::Exempt => 0.0,
            Self::Final { rate_percent } => rate_percent / 100.0,
        }
    }

    /// Get rate as percentage
    pub fn rate_percent(&self) -> f64 {
        self.rate_decimal() * 100.0
    }
}

/// Type of taxable transaction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    /// Delivery of taxable goods (penyerahan BKP)
    DeliveryOfGoods,
    /// Import of goods (impor BKP)
    ImportOfGoods,
    /// Delivery of taxable services (penyerahan JKP)
    DeliveryOfServices,
    /// Utilization of intangible goods from outside
    UtilizationOfIntangibleGoods,
    /// Utilization of services from outside
    UtilizationOfServices,
    /// Export of goods (ekspor BKP)
    ExportOfGoods,
    /// Export of services (ekspor JKP)
    ExportOfServices,
}

impl TransactionType {
    /// Get transaction type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::DeliveryOfGoods => "Penyerahan Barang Kena Pajak (BKP)",
            Self::ImportOfGoods => "Impor Barang Kena Pajak",
            Self::DeliveryOfServices => "Penyerahan Jasa Kena Pajak (JKP)",
            Self::UtilizationOfIntangibleGoods => {
                "Pemanfaatan BKP Tidak Berwujud dari Luar Daerah Pabean"
            }
            Self::UtilizationOfServices => "Pemanfaatan JKP dari Luar Daerah Pabean",
            Self::ExportOfGoods => "Ekspor Barang Kena Pajak",
            Self::ExportOfServices => "Ekspor Jasa Kena Pajak",
        }
    }

    /// Get transaction type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::DeliveryOfGoods => "Delivery of Taxable Goods",
            Self::ImportOfGoods => "Import of Taxable Goods",
            Self::DeliveryOfServices => "Delivery of Taxable Services",
            Self::UtilizationOfIntangibleGoods => {
                "Utilization of Intangible Taxable Goods from Outside Customs Area"
            }
            Self::UtilizationOfServices => {
                "Utilization of Taxable Services from Outside Customs Area"
            }
            Self::ExportOfGoods => "Export of Taxable Goods",
            Self::ExportOfServices => "Export of Taxable Services",
        }
    }

    /// Check if transaction is export (0% rate)
    pub fn is_export(&self) -> bool {
        matches!(self, Self::ExportOfGoods | Self::ExportOfServices)
    }
}

/// VAT registration status - Pengusaha Kena Pajak (PKP)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatRegistrationStatus {
    /// Registered as PKP (mandatory if turnover > Rp 4.8 billion)
    Registered {
        /// PKP registration number (NPWP with 000 branch code)
        pkp_number: String,
        /// Registration date
        registration_date: NaiveDate,
    },
    /// Not registered (turnover below threshold or exempt)
    NotRegistered,
    /// Voluntarily registered (below threshold but chose to register)
    VoluntarilyRegistered {
        pkp_number: String,
        registration_date: NaiveDate,
    },
}

impl VatRegistrationStatus {
    /// Turnover threshold for mandatory VAT registration (Rp 4.8 billion)
    pub fn mandatory_threshold() -> i64 {
        4_800_000_000
    }

    /// Check if registered as PKP
    pub fn is_registered(&self) -> bool {
        !matches!(self, Self::NotRegistered)
    }
}

/// VAT transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatTransaction {
    /// Transaction ID
    pub id: String,
    /// Transaction date
    pub date: NaiveDate,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Tax invoice number (Faktur Pajak)
    pub invoice_number: Option<String>,
    /// Base amount (DPP - Dasar Pengenaan Pajak)
    pub base_amount: i64,
    /// VAT rate
    pub vat_rate: VatRate,
    /// VAT amount calculated
    pub vat_amount: i64,
    /// Seller PKP number
    pub seller_pkp: Option<String>,
    /// Buyer PKP number (if applicable)
    pub buyer_pkp: Option<String>,
    /// Whether this is input VAT (can be credited)
    pub is_input_vat: bool,
    /// Whether this is output VAT (must be remitted)
    pub is_output_vat: bool,
}

impl VatTransaction {
    /// Calculate VAT amount from base
    pub fn calculate_vat(&self) -> i64 {
        (self.base_amount as f64 * self.vat_rate.rate_decimal()).round() as i64
    }

    /// Validate VAT calculation
    pub fn is_valid_calculation(&self) -> bool {
        let calculated = self.calculate_vat();
        self.vat_amount == calculated
    }
}

/// Luxury goods tax (PPnBM - Pajak Penjualan atas Barang Mewah)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LuxuryGoodsTaxRate {
    /// 10% - certain motor vehicles
    Rate10Percent,
    /// 20% - certain motor vehicles, alcoholic beverages
    Rate20Percent,
    /// 40% - certain motor vehicles, aircraft
    Rate40Percent,
    /// 50% - certain aircraft, yachts
    Rate50Percent,
    /// 75% - certain yachts, luxury goods
    Rate75Percent,
    /// 125% - certain luxury goods (rare cases)
    Rate125Percent,
}

impl LuxuryGoodsTaxRate {
    /// Get rate as decimal
    pub fn rate_decimal(&self) -> f64 {
        match self {
            Self::Rate10Percent => 0.10,
            Self::Rate20Percent => 0.20,
            Self::Rate40Percent => 0.40,
            Self::Rate50Percent => 0.50,
            Self::Rate75Percent => 0.75,
            Self::Rate125Percent => 1.25,
        }
    }

    /// Get rate as percentage
    pub fn rate_percent(&self) -> f64 {
        self.rate_decimal() * 100.0
    }
}

/// VAT exempt goods and services - Pasal 4A
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VatExemptCategory {
    /// Basic necessities (e.g., rice, vegetables, meat)
    BasicNecessities,
    /// Health services
    HealthServices,
    /// Education services
    EducationServices,
    /// Social services
    SocialServices,
    /// Public transportation services
    PublicTransportation,
    /// Manpower services
    ManpowerServices,
    /// Postal services with stamps
    PostalServices,
    /// Financial services
    FinancialServices,
    /// Insurance services
    InsuranceServices,
    /// Religious services
    ReligiousServices,
}

impl VatExemptCategory {
    /// Get category name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::BasicNecessities => "Barang kebutuhan pokok",
            Self::HealthServices => "Jasa kesehatan",
            Self::EducationServices => "Jasa pendidikan",
            Self::SocialServices => "Jasa sosial",
            Self::PublicTransportation => "Jasa angkutan umum",
            Self::ManpowerServices => "Jasa tenaga kerja",
            Self::PostalServices => "Jasa pos dengan perangko",
            Self::FinancialServices => "Jasa keuangan",
            Self::InsuranceServices => "Jasa asuransi",
            Self::ReligiousServices => "Jasa keagamaan",
        }
    }

    /// Get category name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::BasicNecessities => "Basic necessities",
            Self::HealthServices => "Health services",
            Self::EducationServices => "Education services",
            Self::SocialServices => "Social services",
            Self::PublicTransportation => "Public transportation services",
            Self::ManpowerServices => "Manpower services",
            Self::PostalServices => "Postal services with stamps",
            Self::FinancialServices => "Financial services",
            Self::InsuranceServices => "Insurance services",
            Self::ReligiousServices => "Religious services",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_rate_for_date() {
        let date_2024 = NaiveDate::from_ymd_opt(2024, 6, 1).expect("Valid date");
        let rate_2024 = VatRate::standard_rate_for_date(date_2024);
        assert_eq!(rate_2024.rate_percent(), 11.0);

        let date_2025 = NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date");
        let rate_2025 = VatRate::standard_rate_for_date(date_2025);
        assert_eq!(rate_2025.rate_percent(), 12.0);
    }

    #[test]
    fn test_vat_rate_decimal() {
        let rate = VatRate::Standard11Percent;
        assert_eq!(rate.rate_decimal(), 0.11);
        assert_eq!(rate.rate_percent(), 11.0);

        let export = VatRate::Export;
        assert_eq!(export.rate_decimal(), 0.0);
    }

    #[test]
    fn test_transaction_type_export() {
        let export_goods = TransactionType::ExportOfGoods;
        assert!(export_goods.is_export());

        let delivery = TransactionType::DeliveryOfGoods;
        assert!(!delivery.is_export());
    }

    #[test]
    fn test_vat_registration_threshold() {
        assert_eq!(VatRegistrationStatus::mandatory_threshold(), 4_800_000_000);
    }

    #[test]
    fn test_vat_calculation() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date");
        let transaction = VatTransaction {
            id: "VAT001".to_string(),
            date,
            transaction_type: TransactionType::DeliveryOfGoods,
            invoice_number: Some("010.000-24.00000001".to_string()),
            base_amount: 10_000_000,
            vat_rate: VatRate::Standard11Percent,
            vat_amount: 1_100_000,
            seller_pkp: Some("01.234.567.8-901.000".to_string()),
            buyer_pkp: None,
            is_input_vat: false,
            is_output_vat: true,
        };

        assert_eq!(transaction.calculate_vat(), 1_100_000);
        assert!(transaction.is_valid_calculation());
    }

    #[test]
    fn test_luxury_goods_tax_rates() {
        let rate10 = LuxuryGoodsTaxRate::Rate10Percent;
        assert_eq!(rate10.rate_percent(), 10.0);

        let rate125 = LuxuryGoodsTaxRate::Rate125Percent;
        assert_eq!(rate125.rate_percent(), 125.0);
    }

    #[test]
    fn test_vat_exempt_categories() {
        let basic = VatExemptCategory::BasicNecessities;
        assert_eq!(basic.name_id(), "Barang kebutuhan pokok");
        assert_eq!(basic.name_en(), "Basic necessities");
    }
}
