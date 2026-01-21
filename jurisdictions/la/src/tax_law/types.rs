//! Tax Law Types (ປະເພດກົດໝາຍພາສີ)
//!
//! Comprehensive type definitions for Lao PDR tax system.
//! Based on Tax Law 2011, VAT Law, and Customs Law.
//!
//! ## Legal Basis
//!
//! - **Tax Law 2011** (Law No. 05/NA, effective October 20, 2011)
//! - **VAT Law** - Value Added Tax regulations
//! - **Customs Law** - Import/Export duties
//!
//! ## Tax Structure in Lao PDR
//!
//! - **Personal Income Tax**: Progressive brackets (0%-25%)
//! - **Corporate Income Tax**: Flat 24%
//! - **Value Added Tax (VAT)**: Standard 10%
//! - **Property Tax**: On land and buildings
//! - **Excise Tax**: On specific goods
//! - **Customs Duties**: On imports
//!
//! ## Tax Treaty Network
//!
//! Lao PDR has tax treaties with several countries to avoid double taxation:
//! - ASEAN countries (Thailand, Vietnam, Myanmar, etc.)
//! - China, South Korea, Russia, Luxembourg

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants (ຄ່າຄົງທີ່)
// ============================================================================

/// Personal income tax brackets in LAK and rates
/// ອັດຕາພາສີລາຍໄດ້ບຸກຄົນ
pub const PERSONAL_INCOME_TAX_BRACKETS: [(u64, f64); 6] = [
    (0, 0.0),           // 0% for income up to 1,300,000 LAK
    (1_300_000, 0.05),  // 5% for 1,300,001 - 8,500,000 LAK
    (8_500_000, 0.10),  // 10% for 8,500,001 - 15,000,000 LAK
    (15_000_000, 0.15), // 15% for 15,000,001 - 24,000,000 LAK
    (24_000_000, 0.20), // 20% for 24,000,001 - 65,000,000 LAK
    (65_000_000, 0.25), // 25% for income above 65,000,000 LAK
];

/// Corporate income tax rate (24%)
/// ອັດຕາພາສີລາຍໄດ້ນິຕິບຸກຄົນ
pub const CORPORATE_INCOME_TAX_RATE: f64 = 0.24;

/// Standard VAT rate (10%)
/// ອັດຕາພາສີມູນຄ່າເພີ່ມມາດຕະຖານ
pub const VAT_STANDARD_RATE: f64 = 0.10;

/// VAT registration threshold (400,000,000 LAK annual turnover)
/// ເກນການຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
pub const VAT_REGISTRATION_THRESHOLD: u64 = 400_000_000;

/// Minimum wage threshold for income tax (1,300,000 LAK/month)
/// ເກນຄ່າແຮງງານຂັ້ນຕ່ຳສຳລັບພາສີລາຍໄດ້
pub const INCOME_TAX_THRESHOLD: u64 = 1_300_000;

/// Property tax rates range
/// ອັດຕາພາສີຊັບສິນ
pub const PROPERTY_TAX_RATE_MIN: f64 = 0.001; // 0.1%
pub const PROPERTY_TAX_RATE_MAX: f64 = 0.005; // 0.5%

/// Customs duty rates range
/// ອັດຕາພາສີສຸນລະກາກອນ
pub const CUSTOMS_DUTY_RATE_MIN: f64 = 0.0;
pub const CUSTOMS_DUTY_RATE_MAX: f64 = 0.40; // Up to 40%

/// Withholding tax rate on dividends
/// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍເງິນປັນຜົນ
pub const WITHHOLDING_TAX_DIVIDEND: f64 = 0.10; // 10%

/// Withholding tax rate on interest
/// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍດອກເບ້ຍ
pub const WITHHOLDING_TAX_INTEREST: f64 = 0.10; // 10%

/// Withholding tax rate on royalties
/// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍຄ່າລິຂະສິດ
pub const WITHHOLDING_TAX_ROYALTY: f64 = 0.05; // 5%

/// Withholding tax rate on service fees to non-residents
/// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍຄ່າບໍລິການໃຫ້ຜູ້ບໍ່ມີຖິ່ນພັກເຊົາ
pub const WITHHOLDING_TAX_SERVICE_NON_RESIDENT: f64 = 0.15; // 15%

// ============================================================================
// Personal Income Tax Brackets (ອັດຕາພາສີລາຍໄດ້ບຸກຄົນ)
// ============================================================================

/// Personal income tax bracket with bilingual descriptions
/// ອັດຕາພາສີລາຍໄດ້ບຸກຄົນພ້ອມຄຳອະທິບາຍສອງພາສາ
///
/// Tax Law 2011, Article 38
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalIncomeTaxBracket {
    /// Minimum income in LAK for this bracket
    /// ລາຍໄດ້ຕ່ຳສຸດສຳລັບອັດຕານີ້
    pub min_income_lak: u64,

    /// Maximum income in LAK for this bracket (None for unlimited)
    /// ລາຍໄດ້ສູງສຸດສຳລັບອັດຕານີ້ (ບໍ່ມີຂອບເຂດຖ້າ None)
    pub max_income_lak: Option<u64>,

    /// Tax rate as percentage (0-100)
    /// ອັດຕາພາສີເປັນເປີເຊັນ (0-100)
    pub rate_percentage: f64,

    /// Description in Lao
    /// ຄຳອະທິບາຍເປັນພາສາລາວ
    pub description_lao: String,

    /// Description in English
    /// ຄຳອະທິບາຍເປັນພາສາອັງກິດ
    pub description_en: String,
}

impl PersonalIncomeTaxBracket {
    /// Create a new tax bracket
    pub fn new(
        min_income_lak: u64,
        max_income_lak: Option<u64>,
        rate_percentage: f64,
        description_lao: impl Into<String>,
        description_en: impl Into<String>,
    ) -> Self {
        Self {
            min_income_lak,
            max_income_lak,
            rate_percentage,
            description_lao: description_lao.into(),
            description_en: description_en.into(),
        }
    }

    /// Get all standard tax brackets
    /// ຮັບອັດຕາພາສີມາດຕະຖານທັງໝົດ
    pub fn standard_brackets() -> Vec<Self> {
        vec![
            Self::new(
                0,
                Some(1_300_000),
                0.0,
                "ບໍ່ຕ້ອງເສຍພາສີ - ລາຍໄດ້ຂັ້ນຕ່ຳ",
                "Tax exempt - minimum income threshold",
            ),
            Self::new(
                1_300_001,
                Some(8_500_000),
                5.0,
                "ອັດຕາ 5% ສຳລັບລາຍໄດ້ລະດັບຕ່ຳ",
                "5% rate for low income level",
            ),
            Self::new(
                8_500_001,
                Some(15_000_000),
                10.0,
                "ອັດຕາ 10% ສຳລັບລາຍໄດ້ລະດັບກາງ-ຕ່ຳ",
                "10% rate for lower-middle income level",
            ),
            Self::new(
                15_000_001,
                Some(24_000_000),
                15.0,
                "ອັດຕາ 15% ສຳລັບລາຍໄດ້ລະດັບກາງ",
                "15% rate for middle income level",
            ),
            Self::new(
                24_000_001,
                Some(65_000_000),
                20.0,
                "ອັດຕາ 20% ສຳລັບລາຍໄດ້ລະດັບກາງ-ສູງ",
                "20% rate for upper-middle income level",
            ),
            Self::new(
                65_000_001,
                None,
                25.0,
                "ອັດຕາ 25% ສຳລັບລາຍໄດ້ລະດັບສູງ",
                "25% rate for high income level",
            ),
        ]
    }
}

// ============================================================================
// VAT Exemptions (ການຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມ)
// ============================================================================

/// VAT exemption categories with legal references
/// ປະເພດການຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມພ້ອມອ້າງອິງກົດໝາຍ
///
/// Based on VAT Law and implementing regulations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VatExemption {
    /// Basic foodstuffs (Article 12.1)
    /// ອາຫານພື້ນຖານ (ມາດຕາ 12.1)
    BasicFoodstuffs {
        /// Article reference
        article_reference: u16,
        /// Description in Lao
        description_lao: String,
    },

    /// Educational services (Article 12.2)
    /// ການບໍລິການດ້ານການສຶກສາ (ມາດຕາ 12.2)
    EducationalServices {
        /// Article reference
        article_reference: u16,
        /// Institution type
        institution_type: String,
    },

    /// Medical and healthcare services (Article 12.3)
    /// ການບໍລິການດ້ານການແພດແລະສຸຂະພາບ (ມາດຕາ 12.3)
    MedicalServices {
        /// Article reference
        article_reference: u16,
        /// Facility type
        facility_type: String,
    },

    /// Financial services (Article 12.4)
    /// ການບໍລິການທາງການເງິນ (ມາດຕາ 12.4)
    FinancialServices {
        /// Article reference
        article_reference: u16,
        /// Service type
        service_type: String,
    },

    /// Real estate rental (Article 12.5)
    /// ການເຊົ່າອະສັງຫາລິມະຊັບ (ມາດຕາ 12.5)
    RealEstateRental {
        /// Article reference
        article_reference: u16,
        /// Property type
        property_type: String,
    },

    /// Agricultural products (Article 12.6)
    /// ຜະລິດຕະພັນກະສິກຳ (ມາດຕາ 12.6)
    AgriculturalProducts {
        /// Article reference
        article_reference: u16,
        /// Product category
        product_category: String,
    },

    /// Public transport services (Article 12.7)
    /// ການບໍລິການຂົນສົ່ງສາທາລະນະ (ມາດຕາ 12.7)
    PublicTransport {
        /// Article reference
        article_reference: u16,
        /// Transport type
        transport_type: String,
    },

    /// International transportation (Article 12.8)
    /// ການຂົນສົ່ງສາກົນ (ມາດຕາ 12.8)
    InternationalTransportation {
        /// Article reference
        article_reference: u16,
    },

    /// Postal services (Article 12.9)
    /// ການບໍລິການໄປສະນີ (ມາດຕາ 12.9)
    PostalServices {
        /// Article reference
        article_reference: u16,
    },

    /// Other exemptions with specific legal basis
    /// ການຍົກເວັ້ນອື່ນໆພ້ອມພື້ນຖານທາງກົດໝາຍ
    Other {
        /// Article reference
        article_reference: u16,
        /// Reason in Lao
        reason_lao: String,
        /// Reason in English
        reason_en: String,
    },
}

impl VatExemption {
    /// Get the article reference for this exemption
    pub fn article_reference(&self) -> u16 {
        match self {
            VatExemption::BasicFoodstuffs {
                article_reference, ..
            } => *article_reference,
            VatExemption::EducationalServices {
                article_reference, ..
            } => *article_reference,
            VatExemption::MedicalServices {
                article_reference, ..
            } => *article_reference,
            VatExemption::FinancialServices {
                article_reference, ..
            } => *article_reference,
            VatExemption::RealEstateRental {
                article_reference, ..
            } => *article_reference,
            VatExemption::AgriculturalProducts {
                article_reference, ..
            } => *article_reference,
            VatExemption::PublicTransport {
                article_reference, ..
            } => *article_reference,
            VatExemption::InternationalTransportation {
                article_reference, ..
            } => *article_reference,
            VatExemption::PostalServices {
                article_reference, ..
            } => *article_reference,
            VatExemption::Other {
                article_reference, ..
            } => *article_reference,
        }
    }
}

// ============================================================================
// VAT Registration (ການຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ)
// ============================================================================

/// VAT registration details
/// ລາຍລະອຽດການຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatRegistration {
    /// Business name in Lao
    /// ຊື່ທຸລະກິດເປັນພາສາລາວ
    pub business_name_lao: String,

    /// Business name in English
    /// ຊື່ທຸລະກິດເປັນພາສາອັງກິດ
    pub business_name_en: String,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_identification_number: String,

    /// Annual turnover in LAK
    /// ລາຍຮັບປະຈຳປີເປັນກີບ
    pub annual_turnover_lak: u64,

    /// Registration date
    /// ວັນທີຂຶ້ນທະບຽນ
    pub registration_date: DateTime<Utc>,

    /// Whether business is VAT registered
    /// ວ່າທຸລະກິດໄດ້ຂຶ້ນທະບຽນ VAT ຫຼືບໍ່
    pub is_registered: bool,

    /// VAT registration number (if registered)
    /// ເລກທະບຽນ VAT (ຖ້າຂຶ້ນທະບຽນແລ້ວ)
    pub vat_number: Option<String>,

    /// Business sector
    /// ຂະແໜງທຸລະກິດ
    pub business_sector: Option<String>,

    /// Province/location
    /// ແຂວງ/ທີ່ຕັ້ງ
    pub province: Option<String>,
}

impl VatRegistration {
    /// Check if registration is required based on turnover
    /// ກວດສອບວ່າຕ້ອງຂຶ້ນທະບຽນຫຼືບໍ່ຕາມລາຍຮັບ
    pub fn registration_required(&self) -> bool {
        self.annual_turnover_lak >= VAT_REGISTRATION_THRESHOLD
    }

    /// Check if registration status is compliant
    /// ກວດສອບວ່າສະຖານະການຂຶ້ນທະບຽນຖືກຕ້ອງຫຼືບໍ່
    pub fn is_compliant(&self) -> bool {
        if self.registration_required() {
            self.is_registered && self.vat_number.is_some()
        } else {
            true // Not required, so always compliant
        }
    }
}

// ============================================================================
// Property Types (ປະເພດຊັບສິນ)
// ============================================================================

/// Property type for property tax purposes
/// ປະເພດຊັບສິນສຳລັບພາສີຊັບສິນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Residential land
    /// ທີ່ດິນທີ່ຢູ່ອາໄສ
    ResidentialLand {
        /// Area in square meters
        area_sqm: u64,
    },

    /// Commercial land
    /// ທີ່ດິນການຄ້າ
    CommercialLand {
        /// Area in square meters
        area_sqm: u64,
    },

    /// Industrial land
    /// ທີ່ດິນອຸດສາຫະກຳ
    IndustrialLand {
        /// Area in square meters
        area_sqm: u64,
    },

    /// Agricultural land
    /// ທີ່ດິນກະສິກຳ
    AgriculturalLand {
        /// Area in square meters
        area_sqm: u64,
        /// Crop type
        crop_type: Option<String>,
    },

    /// Building
    /// ອາຄານ
    Building {
        /// Floor area in square meters
        floor_area_sqm: u64,
        /// Building type
        building_type: String,
        /// Number of floors
        floors: u32,
    },

    /// Vehicle
    /// ພາຫະນະ
    Vehicle {
        /// Vehicle type
        vehicle_type: String,
        /// Engine capacity in CC
        engine_capacity_cc: Option<u32>,
        /// Registration year
        registration_year: u32,
    },
}

impl PropertyType {
    /// Get recommended tax rate for this property type
    /// ຮັບອັດຕາພາສີແນະນຳສຳລັບປະເພດຊັບສິນນີ້
    pub fn recommended_tax_rate(&self) -> f64 {
        match self {
            PropertyType::ResidentialLand { .. } => 0.001,  // 0.1%
            PropertyType::CommercialLand { .. } => 0.003,   // 0.3%
            PropertyType::IndustrialLand { .. } => 0.004,   // 0.4%
            PropertyType::AgriculturalLand { .. } => 0.001, // 0.1%
            PropertyType::Building { .. } => 0.002,         // 0.2%
            PropertyType::Vehicle { .. } => 0.005,          // 0.5%
        }
    }
}

// ============================================================================
// Fuel Types (ປະເພດນ້ຳມັນເຊື້ອໄຟ)
// ============================================================================

/// Fuel types for excise tax purposes
/// ປະເພດນ້ຳມັນເຊື້ອໄຟສຳລັບພາສີສິນຄ້າພິເສດ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FuelType {
    /// Gasoline (petrol)
    /// ນ້ຳມັນແອັດຊັງ
    Gasoline {
        /// Octane rating (91, 95, etc.)
        octane_rating: u32,
    },

    /// Diesel
    /// ນ້ຳມັນດີເຊວ
    Diesel,

    /// LPG (Liquefied Petroleum Gas)
    /// ແກັສ LPG
    LPG,

    /// CNG (Compressed Natural Gas)
    /// ແກັສ CNG
    CNG,

    /// Jet fuel / Aviation fuel
    /// ນ້ຳມັນເຮືອບິນ
    JetFuel,

    /// Other fuel types
    /// ປະເພດນ້ຳມັນອື່ນໆ
    Other {
        /// Fuel description
        description: String,
    },
}

impl FuelType {
    /// Get excise tax rate per liter for this fuel type
    /// ຮັບອັດຕາພາສີສິນຄ້າພິເສດຕໍ່ລິດສຳລັບປະເພດນ້ຳມັນນີ້
    pub fn excise_rate_per_liter(&self) -> f64 {
        match self {
            FuelType::Gasoline { octane_rating } => {
                if *octane_rating >= 95 {
                    0.25 // 25% for premium
                } else {
                    0.20 // 20% for regular
                }
            }
            FuelType::Diesel => 0.10,       // 10%
            FuelType::LPG => 0.05,          // 5%
            FuelType::CNG => 0.05,          // 5%
            FuelType::JetFuel => 0.00,      // Exempt for international flights
            FuelType::Other { .. } => 0.15, // 15% default
        }
    }
}

// ============================================================================
// Tax Types (ປະເພດພາສີ)
// ============================================================================

/// Types of taxes in Lao PDR
/// ປະເພດພາສີໃນລາວ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxType {
    /// Personal income tax
    /// ພາສີລາຍໄດ້ບຸກຄົນ
    PersonalIncome,

    /// Corporate income tax
    /// ພາສີລາຍໄດ້ນິຕິບຸກຄົນ
    CorporateIncome,

    /// Value added tax
    /// ພາສີມູນຄ່າເພີ່ມ
    Vat,

    /// Excise tax
    /// ພາສີສິນຄ້າພິເສດ
    Excise,

    /// Property tax
    /// ພາສີຊັບສິນ
    Property,

    /// Customs duty
    /// ພາສີສຸນລະກາກອນ
    Customs,

    /// Withholding tax
    /// ພາສີຫັກ ນ ທີ່ຈ່າຍ
    Withholding {
        /// Type of payment
        payment_type: WithholdingPaymentType,
    },

    /// Stamp duty
    /// ພາສີອາກອນ
    StampDuty,

    /// Other taxes
    /// ພາສີອື່ນໆ
    Other {
        /// Description
        description: String,
    },
}

/// Withholding tax payment types
/// ປະເພດການຈ່າຍເງິນພາສີຫັກ ນ ທີ່ຈ່າຍ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithholdingPaymentType {
    /// Dividends
    /// ເງິນປັນຜົນ
    Dividend,

    /// Interest
    /// ດອກເບ້ຍ
    Interest,

    /// Royalties
    /// ຄ່າລິຂະສິດ
    Royalty,

    /// Service fees
    /// ຄ່າບໍລິການ
    ServiceFee,

    /// Rental payments
    /// ຄ່າເຊົ່າ
    Rental,

    /// Other
    /// ອື່ນໆ
    Other { description: String },
}

// ============================================================================
// Tax Filing Periods (ໄລຍະເວລາຍື່ນແບບພາສີ)
// ============================================================================

/// Tax filing period
/// ໄລຍະເວລາຍື່ນແບບພາສີ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxFilingPeriod {
    /// Monthly filing
    /// ຍື່ນແບບປະຈຳເດືອນ
    Monthly,

    /// Quarterly filing
    /// ຍື່ນແບບປະຈຳໄຕມາດ
    Quarterly,

    /// Annual filing
    /// ຍື່ນແບບປະຈຳປີ
    Annual,
}

impl TaxFilingPeriod {
    /// Get filing frequency description in Lao
    pub fn description_lao(&self) -> &'static str {
        match self {
            TaxFilingPeriod::Monthly => "ປະຈຳເດືອນ",
            TaxFilingPeriod::Quarterly => "ປະຈຳໄຕມາດ",
            TaxFilingPeriod::Annual => "ປະຈຳປີ",
        }
    }

    /// Get filing frequency description in English
    pub fn description_en(&self) -> &'static str {
        match self {
            TaxFilingPeriod::Monthly => "Monthly",
            TaxFilingPeriod::Quarterly => "Quarterly",
            TaxFilingPeriod::Annual => "Annual",
        }
    }
}

// ============================================================================
// Tax Filing Record (ບັນທຶກການຍື່ນແບບພາສີ)
// ============================================================================

/// Tax filing record
/// ບັນທຶກການຍື່ນແບບພາສີ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxFiling {
    /// Taxpayer name
    /// ຊື່ຜູ້ເສຍພາສີ
    pub taxpayer_name: String,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_identification_number: String,

    /// Type of tax
    /// ປະເພດພາສີ
    pub tax_type: TaxType,

    /// Filing period
    /// ໄລຍະເວລາຍື່ນແບບ
    pub filing_period: TaxFilingPeriod,

    /// Tax year
    /// ປີພາສີ
    pub tax_year: u32,

    /// Period month (for monthly/quarterly)
    /// ເດືອນ (ສຳລັບປະຈຳເດືອນ/ໄຕມາດ)
    pub period_month: Option<u32>,

    /// Due date
    /// ວັນກຳນົດສົ່ງ
    pub due_date: DateTime<Utc>,

    /// Amount due in LAK
    /// ຈຳນວນພາສີທີ່ຕ້ອງຈ່າຍ
    pub amount_due_lak: u64,

    /// Whether return has been filed
    /// ວ່າໄດ້ຍື່ນແບບແລ້ວຫຼືບໍ່
    pub is_filed: bool,

    /// Filing date (if filed)
    /// ວັນທີຍື່ນແບບ (ຖ້າຍື່ນແລ້ວ)
    pub filing_date: Option<DateTime<Utc>>,

    /// Whether tax has been paid
    /// ວ່າໄດ້ຈ່າຍພາສີແລ້ວຫຼືບໍ່
    pub is_paid: bool,

    /// Payment date (if paid)
    /// ວັນທີຈ່າຍ (ຖ້າຈ່າຍແລ້ວ)
    pub payment_date: Option<DateTime<Utc>>,

    /// Amount paid in LAK
    /// ຈຳນວນທີ່ຈ່າຍແລ້ວ
    pub amount_paid_lak: Option<u64>,

    /// Penalty amount (if any)
    /// ຄ່າປັບ (ຖ້າມີ)
    pub penalty_lak: Option<u64>,
}

impl TaxFiling {
    /// Check if filing is late
    /// ກວດສອບວ່າຍື່ນແບບຊ້າຫຼືບໍ່
    pub fn is_late(&self) -> bool {
        if let Some(filing_date) = self.filing_date {
            filing_date > self.due_date
        } else {
            chrono::Utc::now() > self.due_date
        }
    }

    /// Calculate days late (if late)
    /// ຄຳນວນຈຳນວນມື້ທີ່ຊ້າ
    pub fn days_late(&self) -> Option<i64> {
        if !self.is_late() {
            return None;
        }

        let compare_date = self.filing_date.unwrap_or_else(chrono::Utc::now);
        let duration = compare_date.signed_duration_since(self.due_date);
        Some(duration.num_days())
    }

    /// Check if payment is complete
    /// ກວດສອບວ່າຈ່າຍຄົບຖ້ວນຫຼືບໍ່
    pub fn is_payment_complete(&self) -> bool {
        if let Some(paid) = self.amount_paid_lak {
            paid >= self.amount_due_lak + self.penalty_lak.unwrap_or(0)
        } else {
            false
        }
    }
}

// ============================================================================
// Withholding Tax (ພາສີຫັກ ນ ທີ່ຈ່າຍ)
// ============================================================================

/// Withholding tax record
/// ບັນທຶກພາສີຫັກ ນ ທີ່ຈ່າຍ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithholdingTax {
    /// Payer name (Lao)
    /// ຊື່ຜູ້ຈ່າຍ (ລາວ)
    pub payer_name_lao: String,

    /// Payer name (English)
    /// ຊື່ຜູ້ຈ່າຍ (ອັງກິດ)
    pub payer_name_eng: Option<String>,

    /// Payer tax ID
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີຂອງຜູ້ຈ່າຍ
    pub payer_tax_id: String,

    /// Recipient name (Lao)
    /// ຊື່ຜູ້ຮັບ (ລາວ)
    pub recipient_name_lao: String,

    /// Recipient name (English)
    /// ຊື່ຜູ້ຮັບ (ອັງກິດ)
    pub recipient_name_eng: Option<String>,

    /// Recipient tax ID (if available)
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີຂອງຜູ້ຮັບ
    pub recipient_tax_id: Option<String>,

    /// Is recipient a non-resident?
    /// ຜູ້ຮັບເປັນຜູ້ບໍ່ມີຖິ່ນພັກເຊົາຫຼືບໍ່?
    pub is_non_resident: bool,

    /// Payment type
    /// ປະເພດການຈ່າຍ
    pub payment_type: WithholdingPaymentType,

    /// Gross payment amount in LAK
    /// ຈຳນວນຈ່າຍລວມກ່ອນຫັກພາສີ
    pub gross_payment_lak: u64,

    /// Withholding tax rate
    /// ອັດຕາພາສີຫັກ ນ ທີ່ຈ່າຍ
    pub withholding_rate: f64,

    /// Withholding tax amount in LAK
    /// ຈຳນວນພາສີຫັກ ນ ທີ່ຈ່າຍ
    pub withholding_amount_lak: u64,

    /// Net payment amount in LAK
    /// ຈຳນວນຈ່າຍສຸດທິ
    pub net_payment_lak: u64,

    /// Payment date
    /// ວັນທີຈ່າຍ
    pub payment_date: DateTime<Utc>,

    /// Tax remitted to government?
    /// ໄດ້ສົ່ງພາສີໃຫ້ລັດຖະບານແລ້ວຫຼືບໍ່?
    pub is_remitted: bool,

    /// Remittance date
    /// ວັນທີສົ່ງພາສີ
    pub remittance_date: Option<DateTime<Utc>>,
}

impl WithholdingTax {
    /// Get the applicable withholding rate based on payment type and residency
    pub fn get_applicable_rate(
        payment_type: &WithholdingPaymentType,
        is_non_resident: bool,
    ) -> f64 {
        match (payment_type, is_non_resident) {
            (WithholdingPaymentType::Dividend, false) => WITHHOLDING_TAX_DIVIDEND,
            (WithholdingPaymentType::Dividend, true) => WITHHOLDING_TAX_DIVIDEND,
            (WithholdingPaymentType::Interest, false) => WITHHOLDING_TAX_INTEREST,
            (WithholdingPaymentType::Interest, true) => WITHHOLDING_TAX_INTEREST,
            (WithholdingPaymentType::Royalty, false) => WITHHOLDING_TAX_ROYALTY,
            (WithholdingPaymentType::Royalty, true) => WITHHOLDING_TAX_ROYALTY,
            (WithholdingPaymentType::ServiceFee, false) => 0.0, // No withholding for residents
            (WithholdingPaymentType::ServiceFee, true) => WITHHOLDING_TAX_SERVICE_NON_RESIDENT,
            (WithholdingPaymentType::Rental, _) => 0.05, // 5% for rental
            (WithholdingPaymentType::Other { .. }, false) => 0.0,
            (WithholdingPaymentType::Other { .. }, true) => WITHHOLDING_TAX_SERVICE_NON_RESIDENT,
        }
    }
}

// ============================================================================
// Tax Residence Status (ສະຖານະພັກເຊົາທາງພາສີ)
// ============================================================================

/// Tax residence status in Lao PDR
/// ສະຖານະພັກເຊົາທາງພາສີ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxResidenceStatus {
    /// Lao tax resident (stays 183+ days per year)
    /// ຜູ້ມີຖິ່ນພັກເຊົາທາງພາສີລາວ (ພັກເຊົາ 183+ ມື້ຕໍ່ປີ)
    LaoResident {
        /// Lao ID number
        /// ເລກບັດປະຈຳຕົວ
        lao_id: Option<String>,
        /// Days in Lao PDR in tax year
        /// ຈຳນວນມື້ຢູ່ໃນລາວໃນປີພາສີ
        days_in_lao: u32,
    },

    /// Non-resident (foreign tax resident)
    /// ຜູ້ບໍ່ມີຖິ່ນພັກເຊົາທາງພາສີ
    NonResident {
        /// Passport number
        /// ເລກໜັງສືຜ່ານແດນ
        passport_number: Option<String>,
        /// Country of tax residence
        /// ປະເທດທີ່ມີຖິ່ນພັກເຊົາທາງພາສີ
        tax_residence_country: String,
    },

    /// Treaty resident (covered by tax treaty)
    /// ຜູ້ມີຖິ່ນພັກເຊົາທາງພາສີຕາມສົນທິສັນຍາ
    TreatyResident {
        /// Treaty country
        /// ປະເທດທີ່ມີສົນທິສັນຍາ
        treaty_country: String,
        /// Treaty article applicable
        /// ມາດຕາຂອງສົນທິສັນຍາທີ່ນຳໃຊ້
        treaty_article: Option<u32>,
    },
}

// ============================================================================
// Personal Income Tax (ພາສີລາຍໄດ້ບຸກຄົນ)
// ============================================================================

/// Type of personal income
/// ປະເພດລາຍໄດ້ບຸກຄົນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncomeType {
    /// Employment income (salary, wages)
    /// ລາຍໄດ້ຈາກການຈ້າງງານ (ເງິນເດືອນ, ຄ່າແຮງງານ)
    Employment {
        /// Employer name
        /// ຊື່ນາຍຈ້າງ
        employer: String,
        /// Employer tax ID
        /// ເລກພາສີຂອງນາຍຈ້າງ
        employer_tax_id: Option<String>,
    },

    /// Business income
    /// ລາຍໄດ້ຈາກທຸລະກິດ
    Business {
        /// Business name
        /// ຊື່ທຸລະກິດ
        business_name: String,
        /// Business registration number
        /// ເລກທະບຽນທຸລະກິດ
        registration_number: Option<String>,
    },

    /// Investment income (dividends, interest, capital gains)
    /// ລາຍໄດ້ຈາກການລົງທຶນ (ເງິນປັນຜົນ, ດອກເບ້ຍ, ກຳໄລຈາກການຂາຍຊັບສິນ)
    Investment {
        /// Type of investment
        /// ປະເພດການລົງທຶນ
        investment_type: String,
    },

    /// Rental income
    /// ລາຍໄດ້ຈາກການເຊົ່າ
    Rental {
        /// Property address
        /// ທີ່ຢູ່ຊັບສິນ
        property_address: String,
    },

    /// Professional services
    /// ລາຍໄດ້ຈາກການໃຫ້ບໍລິການວິຊາຊີບ
    ProfessionalServices {
        /// Profession type
        /// ປະເພດວິຊາຊີບ
        profession: String,
    },

    /// Other income
    /// ລາຍໄດ້ອື່ນໆ
    Other {
        /// Description
        /// ລາຍລະອຽດ
        description: String,
    },
}

/// Personal income tax return
/// ການຍື່ນແບບພາສີລາຍໄດ້ບຸກຄົນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalIncomeTax {
    /// Taxpayer name (Lao)
    /// ຊື່ຜູ້ເສຍພາສີ (ລາວ)
    pub taxpayer_name_lao: String,

    /// Taxpayer name (English)
    /// ຊື່ຜູ້ເສຍພາສີ (ອັງກິດ)
    pub taxpayer_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// Tax residence status
    /// ສະຖານະພັກເຊົາທາງພາສີ
    pub residence_status: TaxResidenceStatus,

    /// Tax year
    /// ປີພາສີ
    pub tax_year: u32,

    /// Total gross income in LAK
    /// ລາຍໄດ້ລວມກ່ອນຫັກຄ່າໃຊ້ຈ່າຍ
    pub gross_income_lak: u64,

    /// Income breakdown by type
    /// ລາຍລະອຽດລາຍໄດ້ຕາມປະເພດ
    pub income_breakdown: Vec<(IncomeType, u64)>,

    /// Deductible expenses in LAK
    /// ຄ່າໃຊ້ຈ່າຍທີ່ຫັກໄດ້
    pub deductible_expenses_lak: u64,

    /// Personal allowance in LAK
    /// ເງິນຫຼຸດໄດ້ຂອງບຸກຄົນ
    pub personal_allowance_lak: u64,

    /// Dependent allowances in LAK
    /// ເງິນຫຼຸດໄດ້ຂອງຜູ້ທີ່ພຶ່ງພາອາໄສ
    pub dependent_allowances_lak: u64,

    /// Taxable income in LAK (gross - deductions - allowances)
    /// ລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີ
    pub taxable_income_lak: u64,

    /// Tax calculated in LAK
    /// ພາສີທີ່ຄຳນວນໄດ້
    pub tax_calculated_lak: u64,

    /// Tax withheld by employer in LAK
    /// ພາສີທີ່ນາຍຈ້າງຫັກໄວ້ແລ້ວ
    pub tax_withheld_lak: u64,

    /// Tax payment date
    /// ວັນທີຈ່າຍພາສີ
    pub payment_date: Option<DateTime<Utc>>,

    /// Filing date
    /// ວັນທີຍື່ນແບບ
    pub filing_date: DateTime<Utc>,
}

// ============================================================================
// Corporate Income Tax (ພາສີລາຍໄດ້ນິຕິບຸກຄົນ)
// ============================================================================

/// Type of corporate entity
/// ປະເພດນິຕິບຸກຄົນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateEntityType {
    /// Limited company
    /// ບໍລິສັດຈຳກັດ
    LimitedCompany,

    /// Public company
    /// ບໍລິສັດມະຫາຊົນ
    PublicCompany,

    /// Partnership
    /// ຫ້າງຫຸ້ນສ່ວນ
    Partnership,

    /// State-owned enterprise
    /// ວິສາຫະກິດລັດ
    StateOwnedEnterprise,

    /// Foreign company branch
    /// ສາຂາຂອງບໍລິສັດຕ່າງປະເທດ
    ForeignBranch {
        /// Home country
        /// ປະເທດຕົ້ນທາງ
        home_country: String,
    },

    /// Other
    /// ອື່ນໆ
    Other {
        /// Description
        /// ລາຍລະອຽດ
        description: String,
    },
}

/// Corporate income tax return
/// ການຍື່ນແບບພາສີລາຍໄດ້ນິຕິບຸກຄົນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateIncomeTax {
    /// Company name (Lao)
    /// ຊື່ບໍລິສັດ (ລາວ)
    pub company_name_lao: String,

    /// Company name (English)
    /// ຊື່ບໍລິສັດ (ອັງກິດ)
    pub company_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// Corporate entity type
    /// ປະເພດນິຕິບຸກຄົນ
    pub entity_type: CorporateEntityType,

    /// Tax year
    /// ປີພາສີ
    pub tax_year: u32,

    /// Total revenue in LAK
    /// ລາຍຮັບລວມ
    pub total_revenue_lak: u64,

    /// Cost of goods sold in LAK
    /// ຕົ້ນທຶນສິນຄ້າຂາຍ
    pub cost_of_goods_sold_lak: u64,

    /// Operating expenses in LAK
    /// ຄ່າໃຊ້ຈ່າຍດຳເນີນງານ
    pub operating_expenses_lak: u64,

    /// Depreciation in LAK
    /// ຄ່າເສື່ອມລາຄາ
    pub depreciation_lak: u64,

    /// Interest expenses in LAK
    /// ຄ່າດອກເບ້ຍ
    pub interest_expenses_lak: u64,

    /// Other deductible expenses in LAK
    /// ຄ່າໃຊ້ຈ່າຍອື່ນທີ່ຫັກໄດ້
    pub other_expenses_lak: u64,

    /// Taxable income in LAK
    /// ລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີ
    pub taxable_income_lak: u64,

    /// Tax calculated in LAK (24% of taxable income)
    /// ພາສີທີ່ຄຳນວນໄດ້ (24% ຂອງລາຍໄດ້ທີ່ຕ້ອງເສຍພາສີ)
    pub tax_calculated_lak: u64,

    /// Tax credits in LAK
    /// ເງິນຫຼຸດໄດ້ພາສີ
    pub tax_credits_lak: u64,

    /// Tax payment date
    /// ວັນທີຈ່າຍພາສີ
    pub payment_date: Option<DateTime<Utc>>,

    /// Filing date
    /// ວັນທີຍື່ນແບບ
    pub filing_date: DateTime<Utc>,
}

// ============================================================================
// Value Added Tax (VAT) (ພາສີມູນຄ່າເພີ່ມ)
// ============================================================================

/// VAT registration status
/// ສະຖານະການຂຶ້ນທະບຽນພາສີມູນຄ່າເພີ່ມ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VATRegistrationStatus {
    /// Registered for VAT
    /// ຂຶ້ນທະບຽນແລ້ວ
    Registered {
        /// VAT registration number
        /// ເລກທະບຽນພາສີມູນຄ່າເພີ່ມ
        vat_number: String,
        /// Registration date
        /// ວັນທີຂຶ້ນທະບຽນ
        registration_date: DateTime<Utc>,
    },

    /// Not registered (below threshold)
    /// ບໍ່ໄດ້ຂຶ້ນທະບຽນ (ຕ່ຳກວ່າເກນ)
    NotRegistered {
        /// Annual turnover in LAK
        /// ລາຍຮັບປະຈຳປີ
        annual_turnover_lak: u64,
    },

    /// Exempt from VAT
    /// ຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມ
    Exempt {
        /// Exemption reason
        /// ເຫດຜົນຍົກເວັ້ນ
        exemption_reason: String,
    },
}

/// VAT rate type
/// ປະເພດອັດຕາພາສີມູນຄ່າເພີ່ມ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VATRateType {
    /// Standard rate (10%)
    /// ອັດຕາມາດຕະຖານ (10%)
    Standard,

    /// Zero-rated (0% - exports)
    /// ອັດຕາສູນ (0% - ສິນຄ້າສົ່ງອອກ)
    ZeroRated,

    /// Exempt (no VAT charged)
    /// ຍົກເວັ້ນ (ບໍ່ຄິດພາສີມູນຄ່າເພີ່ມ)
    Exempt {
        /// Exemption category
        /// ປະເພດການຍົກເວັ້ນ
        category: VATExemptCategory,
    },
}

/// VAT exempt categories
/// ປະເພດການຍົກເວັ້ນພາສີມູນຄ່າເພີ່ມ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VATExemptCategory {
    /// Financial services
    /// ການບໍລິການທາງດ້ານການເງິນ
    FinancialServices,

    /// Education services
    /// ການບໍລິການດ້ານການສຶກສາ
    Education,

    /// Healthcare services
    /// ການບໍລິການດ້ານສຸຂະພາບ
    Healthcare,

    /// Agricultural products
    /// ຜະລິດຕະພັນກະສິກຳ
    Agriculture,

    /// Public services
    /// ການບໍລິການສາທາລະນະ
    PublicServices,

    /// Other
    /// ອື່ນໆ
    Other {
        /// Description
        /// ລາຍລະອຽດ
        description: String,
    },
}

/// VAT return
/// ການຍື່ນແບບພາສີມູນຄ່າເພີ່ມ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VATReturn {
    /// Taxpayer name (Lao)
    /// ຊື່ຜູ້ເສຍພາສີ (ລາວ)
    pub taxpayer_name_lao: String,

    /// Taxpayer name (English)
    /// ຊື່ຜູ້ເສຍພາສີ (ອັງກິດ)
    pub taxpayer_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// VAT registration status
    /// ສະຖານະການຂຶ້ນທະບຽນ
    pub registration_status: VATRegistrationStatus,

    /// Return period (month/year)
    /// ໄລຍະເວລາ (ເດືອນ/ປີ)
    pub period_month: u32,
    pub period_year: u32,

    /// Output VAT (VAT on sales) in LAK
    /// ພາສີມູນຄ່າເພີ່ມຂາຍອອກ
    pub output_vat_lak: u64,

    /// Input VAT (VAT on purchases) in LAK
    /// ພາສີມູນຄ່າເພີ່ມຊື້ເຂົ້າ
    pub input_vat_lak: u64,

    /// VAT payable (output - input) in LAK
    /// ພາສີມູນຄ່າເພີ່ມທີ່ຕ້ອງຈ່າຍ
    pub vat_payable_lak: i64,

    /// Total sales (excluding VAT) in LAK
    /// ລາຍຮັບລວມ (ບໍ່ລວມພາສີ)
    pub total_sales_lak: u64,

    /// Total purchases (excluding VAT) in LAK
    /// ຕົ້ນທຶນລວມ (ບໍ່ລວມພາສີ)
    pub total_purchases_lak: u64,

    /// Filing date
    /// ວັນທີຍື່ນແບບ
    pub filing_date: DateTime<Utc>,

    /// Payment date
    /// ວັນທີຈ່າຍພາສີ
    pub payment_date: Option<DateTime<Utc>>,
}

// ============================================================================
// Property Tax (ພາສີຊັບສິນ)
// ============================================================================

/// Type of property for tax purposes
/// ປະເພດຊັບສິນສຳລັບພາສີ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyTaxType {
    /// Land
    /// ທີ່ດິນ
    Land {
        /// Land area in square meters
        /// ເນື້ອທີ່ດິນເປັນຕາແມັດ
        area_sqm: u64,
        /// Land classification
        /// ປະເພດທີ່ດິນ
        classification: String,
    },

    /// Building
    /// ອາຄານ
    Building {
        /// Building area in square meters
        /// ເນື້ອທີ່ອາຄານເປັນຕາແມັດ
        area_sqm: u64,
        /// Building type
        /// ປະເພດອາຄານ
        building_type: String,
    },

    /// Land and building
    /// ທີ່ດິນແລະອາຄານ
    LandAndBuilding {
        /// Total area
        /// ເນື້ອທີ່ລວມ
        total_area_sqm: u64,
    },
}

/// Property tax assessment
/// ການປະເມີນພາສີຊັບສິນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTax {
    /// Property owner name (Lao)
    /// ຊື່ເຈົ້າຂອງຊັບສິນ (ລາວ)
    pub owner_name_lao: String,

    /// Property owner name (English)
    /// ຊື່ເຈົ້າຂອງຊັບສິນ (ອັງກິດ)
    pub owner_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// Property address (Lao)
    /// ທີ່ຢູ່ຊັບສິນ (ລາວ)
    pub property_address_lao: String,

    /// Property address (English)
    /// ທີ່ຢູ່ຊັບສິນ (ອັງກິດ)
    pub property_address_eng: Option<String>,

    /// Property type
    /// ປະເພດຊັບສິນ
    pub property_type: PropertyTaxType,

    /// Assessed value in LAK
    /// ມູນຄ່າທີ່ປະເມີນ
    pub assessed_value_lak: u64,

    /// Tax rate (0.1% - 0.5%)
    /// ອັດຕາພາສີ (0.1% - 0.5%)
    pub tax_rate: f64,

    /// Tax amount in LAK
    /// ຈຳນວນພາສີ
    pub tax_amount_lak: u64,

    /// Tax year
    /// ປີພາສີ
    pub tax_year: u32,

    /// Payment date
    /// ວັນທີຈ່າຍພາສີ
    pub payment_date: Option<DateTime<Utc>>,
}

// ============================================================================
// Excise Tax (ພາສີສິນຄ້າພິເສດ)
// ============================================================================

/// Excise tax categories
/// ປະເພດສິນຄ້າພິເສດ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExciseTaxCategory {
    /// Tobacco products
    /// ຜະລິດຕະພັນຢາສູບ
    Tobacco {
        /// Product type
        /// ປະເພດຜະລິດຕະພັນ
        product_type: String,
        /// Tax rate
        /// ອັດຕາພາສີ
        tax_rate: f64,
    },

    /// Alcoholic beverages
    /// ເຄື່ອງດື່ມແອນກໍຮໍ
    Alcohol {
        /// Alcohol percentage
        /// ເປີເຊັນແອນກໍຮໍ
        alcohol_percentage: f64,
        /// Tax rate
        /// ອັດຕາພາສີ
        tax_rate: f64,
    },

    /// Fuel
    /// ນ້ຳມັນເຊື້ອໄຟ
    Fuel {
        /// Fuel type
        /// ປະເພດນ້ຳມັນ
        fuel_type: String,
        /// Tax rate per liter
        /// ອັດຕາພາສີຕໍ່ລິດ
        tax_rate_per_liter: f64,
    },

    /// Vehicles
    /// ພາຫະນະ
    Vehicles {
        /// Engine capacity (cc)
        /// ຂະໜາດເຄື່ອງຍົນ (cc)
        engine_capacity_cc: u32,
        /// Tax rate
        /// ອັດຕາພາສີ
        tax_rate: f64,
    },

    /// Luxury goods
    /// ສິນຄ້າຟຸ່ມເຟືອຍ
    LuxuryGoods {
        /// Product category
        /// ປະເພດສິນຄ້າ
        category: String,
        /// Tax rate
        /// ອັດຕາພາສີ
        tax_rate: f64,
    },

    /// Other
    /// ອື່ນໆ
    Other {
        /// Description
        /// ລາຍລະອຽດ
        description: String,
        /// Tax rate
        /// ອັດຕາພາສີ
        tax_rate: f64,
    },
}

/// Excise tax declaration
/// ການແຈ້ງພາສີສິນຄ້າພິເສດ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExciseTax {
    /// Taxpayer name (Lao)
    /// ຊື່ຜູ້ເສຍພາສີ (ລາວ)
    pub taxpayer_name_lao: String,

    /// Taxpayer name (English)
    /// ຊື່ຜູ້ເສຍພາສີ (ອັງກິດ)
    pub taxpayer_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// Excise category
    /// ປະເພດສິນຄ້າພິເສດ
    pub category: ExciseTaxCategory,

    /// Quantity
    /// ປະລິມານ
    pub quantity: f64,

    /// Unit price in LAK
    /// ລາຄາຕໍ່ຫົວໜ່ວຍ
    pub unit_price_lak: u64,

    /// Total value in LAK
    /// ມູນຄ່າລວມ
    pub total_value_lak: u64,

    /// Excise tax amount in LAK
    /// ຈຳນວນພາສີສິນຄ້າພິເສດ
    pub excise_tax_lak: u64,

    /// Period (month/year)
    /// ໄລຍະເວລາ (ເດືອນ/ປີ)
    pub period_month: u32,
    pub period_year: u32,

    /// Filing date
    /// ວັນທີຍື່ນແບບ
    pub filing_date: DateTime<Utc>,
}

// ============================================================================
// Customs Duty (ພາສີສຸນລະກາກອນ)
// ============================================================================

/// Customs declaration type
/// ປະເພດການແຈ້ງສຸນລະກາກອນ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustomsDeclarationType {
    /// Import
    /// ນຳເຂົ້າ
    Import,

    /// Export
    /// ສົ່ງອອກ
    Export,

    /// Transit
    /// ຜ່ານແດນ
    Transit,

    /// Re-export
    /// ສົ່ງອອກຄືນ
    ReExport,
}

/// Customs duty calculation
/// ການຄຳນວນພາສີສຸນລະກາກອນ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsDuty {
    /// Importer/Exporter name (Lao)
    /// ຊື່ຜູ້ນຳເຂົ້າ/ສົ່ງອອກ (ລາວ)
    pub trader_name_lao: String,

    /// Importer/Exporter name (English)
    /// ຊື່ຜູ້ນຳເຂົ້າ/ສົ່ງອອກ (ອັງກິດ)
    pub trader_name_eng: Option<String>,

    /// Tax identification number
    /// ເລກປະຈຳຕົວຜູ້ເສຍພາສີ
    pub tax_id: String,

    /// Declaration type
    /// ປະເພດການແຈ້ງ
    pub declaration_type: CustomsDeclarationType,

    /// Customs declaration number
    /// ເລກທີໃບແຈ້ງສຸນລະກາກອນ
    pub declaration_number: String,

    /// HS code (Harmonized System code)
    /// ລະຫັດ HS
    pub hs_code: String,

    /// Product description (Lao)
    /// ລາຍລະອຽດສິນຄ້າ (ລາວ)
    pub product_description_lao: String,

    /// Product description (English)
    /// ລາຍລະອຽດສິນຄ້າ (ອັງກິດ)
    pub product_description_eng: Option<String>,

    /// Quantity
    /// ປະລິມານ
    pub quantity: f64,

    /// Unit of measurement
    /// ຫົວໜ່ວຍ
    pub unit: String,

    /// CIF value (Cost, Insurance, Freight) in LAK
    /// ມູນຄ່າ CIF (ຕົ້ນທຶນ, ປະກັນໄພ, ຄ່າຂົນສົ່ງ)
    pub cif_value_lak: u64,

    /// Customs duty rate
    /// ອັດຕາພາສີສຸນລະກາກອນ
    pub duty_rate: f64,

    /// Customs duty amount in LAK
    /// ຈຳນວນພາສີສຸນລະກາກອນ
    pub duty_amount_lak: u64,

    /// VAT on import in LAK
    /// ພາສີມູນຄ່າເພີ່ມນຳເຂົ້າ
    pub import_vat_lak: u64,

    /// Excise tax on import in LAK (if applicable)
    /// ພາສີສິນຄ້າພິເສດນຳເຂົ້າ (ຖ້າມີ)
    pub import_excise_lak: Option<u64>,

    /// Total tax payable in LAK
    /// ພາສີລວມທີ່ຕ້ອງຈ່າຍ
    pub total_tax_lak: u64,

    /// Declaration date
    /// ວັນທີແຈ້ງ
    pub declaration_date: DateTime<Utc>,

    /// Payment date
    /// ວັນທີຈ່າຍພາສີ
    pub payment_date: Option<DateTime<Utc>>,
}

// ============================================================================
// Tax Filing and Payment (ການຍື່ນແບບແລະການຈ່າຍພາສີ)
// ============================================================================

/// Tax payment method
/// ວິທີການຈ່າຍພາສີ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxPaymentMethod {
    /// Cash
    /// ເງິນສົດ
    Cash,

    /// Bank transfer
    /// ໂອນເງິນຜ່ານທະນາຄານ
    BankTransfer {
        /// Bank name
        /// ຊື່ທະນາຄານ
        bank_name: String,
        /// Transaction reference
        /// ເລກອ້າງອິງການໂອນເງິນ
        reference: String,
    },

    /// Check
    /// ເຊັກ
    Check {
        /// Check number
        /// ເລກທີເຊັກ
        check_number: String,
    },

    /// Electronic payment
    /// ການຈ່າຍເງິນທາງອີເລັກໂທຣນິກ
    Electronic {
        /// Platform name
        /// ຊື່ແພລດຟອມ
        platform: String,
        /// Transaction ID
        /// ລະຫັດການເຮັດທຸລະກຳ
        transaction_id: String,
    },
}

/// Tax filing status
/// ສະຖານະການຍື່ນແບບພາສີ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxFilingStatus {
    /// Not filed
    /// ຍັງບໍ່ທັນຍື່ນແບບ
    NotFiled,

    /// Filed on time
    /// ຍື່ນແບບທັນເວລາ
    FiledOnTime {
        /// Filing date
        /// ວັນທີຍື່ນແບບ
        filing_date: DateTime<Utc>,
    },

    /// Filed late
    /// ຍື່ນແບບຊ້າ
    FiledLate {
        /// Filing date
        /// ວັນທີຍື່ນແບບ
        filing_date: DateTime<Utc>,
        /// Days late
        /// ຈຳນວນວັນທີ່ຊ້າ
        days_late: i64,
    },

    /// Under review
    /// ກຳລັງກວດສອບ
    UnderReview,

    /// Accepted
    /// ຮັບຮອງແລ້ວ
    Accepted {
        /// Acceptance date
        /// ວັນທີຮັບຮອງ
        acceptance_date: DateTime<Utc>,
    },

    /// Rejected
    /// ປະຕິເສດ
    Rejected {
        /// Rejection reason
        /// ເຫດຜົນປະຕິເສດ
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_brackets() {
        assert_eq!(PERSONAL_INCOME_TAX_BRACKETS.len(), 6);
        assert_eq!(PERSONAL_INCOME_TAX_BRACKETS[0], (0, 0.0));
        assert_eq!(PERSONAL_INCOME_TAX_BRACKETS[5], (65_000_000, 0.25));
    }

    #[test]
    fn test_corporate_tax_rate() {
        assert_eq!(CORPORATE_INCOME_TAX_RATE, 0.24);
    }

    #[test]
    fn test_vat_rate() {
        assert_eq!(VAT_STANDARD_RATE, 0.10);
    }

    #[test]
    fn test_vat_threshold() {
        assert_eq!(VAT_REGISTRATION_THRESHOLD, 400_000_000);
    }

    #[test]
    fn test_income_tax_threshold() {
        assert_eq!(INCOME_TAX_THRESHOLD, 1_300_000);
    }

    #[test]
    fn test_property_tax_rates() {
        assert_eq!(PROPERTY_TAX_RATE_MIN, 0.001);
        assert_eq!(PROPERTY_TAX_RATE_MAX, 0.005);
    }

    #[test]
    fn test_customs_duty_rates() {
        assert_eq!(CUSTOMS_DUTY_RATE_MIN, 0.0);
        assert_eq!(CUSTOMS_DUTY_RATE_MAX, 0.40);
    }

    #[test]
    fn test_withholding_tax_rates() {
        assert_eq!(WITHHOLDING_TAX_DIVIDEND, 0.10);
        assert_eq!(WITHHOLDING_TAX_INTEREST, 0.10);
        assert_eq!(WITHHOLDING_TAX_ROYALTY, 0.05);
        assert_eq!(WITHHOLDING_TAX_SERVICE_NON_RESIDENT, 0.15);
    }

    #[test]
    fn test_tax_residence_status() {
        let lao_resident = TaxResidenceStatus::LaoResident {
            lao_id: Some("123456789".to_string()),
            days_in_lao: 200,
        };
        assert!(matches!(
            lao_resident,
            TaxResidenceStatus::LaoResident { .. }
        ));

        let non_resident = TaxResidenceStatus::NonResident {
            passport_number: Some("AB1234567".to_string()),
            tax_residence_country: "Thailand".to_string(),
        };
        assert!(matches!(
            non_resident,
            TaxResidenceStatus::NonResident { .. }
        ));
    }

    #[test]
    fn test_vat_rate_types() {
        let standard = VATRateType::Standard;
        assert!(matches!(standard, VATRateType::Standard));

        let zero_rated = VATRateType::ZeroRated;
        assert!(matches!(zero_rated, VATRateType::ZeroRated));

        let exempt = VATRateType::Exempt {
            category: VATExemptCategory::Healthcare,
        };
        assert!(matches!(exempt, VATRateType::Exempt { .. }));
    }

    #[test]
    fn test_personal_income_tax_bracket() {
        let bracket =
            PersonalIncomeTaxBracket::new(1_300_001, Some(8_500_000), 5.0, "ອັດຕາ 5%", "5% rate");
        assert_eq!(bracket.min_income_lak, 1_300_001);
        assert_eq!(bracket.max_income_lak, Some(8_500_000));
        assert_eq!(bracket.rate_percentage, 5.0);
    }

    #[test]
    fn test_standard_brackets() {
        let brackets = PersonalIncomeTaxBracket::standard_brackets();
        assert_eq!(brackets.len(), 6);
        assert_eq!(brackets[0].rate_percentage, 0.0);
        assert_eq!(brackets[5].rate_percentage, 25.0);
    }

    #[test]
    fn test_vat_exemption_article_reference() {
        let exemption = VatExemption::BasicFoodstuffs {
            article_reference: 12,
            description_lao: "ອາຫານພື້ນຖານ".to_string(),
        };
        assert_eq!(exemption.article_reference(), 12);
    }

    #[test]
    fn test_vat_registration() {
        let registration = VatRegistration {
            business_name_lao: "ບໍລິສັດ ABC".to_string(),
            business_name_en: "ABC Company".to_string(),
            tax_identification_number: "1234567890".to_string(),
            annual_turnover_lak: 500_000_000,
            registration_date: Utc::now(),
            is_registered: true,
            vat_number: Some("VAT-2024-001".to_string()),
            business_sector: Some("Manufacturing".to_string()),
            province: Some("Vientiane".to_string()),
        };
        assert!(registration.registration_required());
        assert!(registration.is_compliant());
    }

    #[test]
    fn test_vat_registration_not_required() {
        let registration = VatRegistration {
            business_name_lao: "ຮ້ານຄ້າ ABC".to_string(),
            business_name_en: "ABC Shop".to_string(),
            tax_identification_number: "1234567890".to_string(),
            annual_turnover_lak: 100_000_000, // Below threshold
            registration_date: Utc::now(),
            is_registered: false,
            vat_number: None,
            business_sector: None,
            province: None,
        };
        assert!(!registration.registration_required());
        assert!(registration.is_compliant());
    }

    #[test]
    fn test_property_type_tax_rates() {
        let residential = PropertyType::ResidentialLand { area_sqm: 1000 };
        assert_eq!(residential.recommended_tax_rate(), 0.001);

        let commercial = PropertyType::CommercialLand { area_sqm: 500 };
        assert_eq!(commercial.recommended_tax_rate(), 0.003);

        let vehicle = PropertyType::Vehicle {
            vehicle_type: "Car".to_string(),
            engine_capacity_cc: Some(2000),
            registration_year: 2024,
        };
        assert_eq!(vehicle.recommended_tax_rate(), 0.005);
    }

    #[test]
    fn test_fuel_type_excise_rates() {
        let gasoline_91 = FuelType::Gasoline { octane_rating: 91 };
        assert_eq!(gasoline_91.excise_rate_per_liter(), 0.20);

        let gasoline_95 = FuelType::Gasoline { octane_rating: 95 };
        assert_eq!(gasoline_95.excise_rate_per_liter(), 0.25);

        let diesel = FuelType::Diesel;
        assert_eq!(diesel.excise_rate_per_liter(), 0.10);

        let jet_fuel = FuelType::JetFuel;
        assert_eq!(jet_fuel.excise_rate_per_liter(), 0.00);
    }

    #[test]
    fn test_tax_type() {
        let pit = TaxType::PersonalIncome;
        assert!(matches!(pit, TaxType::PersonalIncome));

        let withholding = TaxType::Withholding {
            payment_type: WithholdingPaymentType::Dividend,
        };
        assert!(matches!(withholding, TaxType::Withholding { .. }));
    }

    #[test]
    fn test_tax_filing_period() {
        let monthly = TaxFilingPeriod::Monthly;
        assert_eq!(monthly.description_en(), "Monthly");
        assert_eq!(monthly.description_lao(), "ປະຈຳເດືອນ");

        let annual = TaxFilingPeriod::Annual;
        assert_eq!(annual.description_en(), "Annual");
        assert_eq!(annual.description_lao(), "ປະຈຳປີ");
    }

    #[test]
    fn test_tax_filing() {
        let now = Utc::now();
        let filing = TaxFiling {
            taxpayer_name: "Test Company".to_string(),
            tax_identification_number: "1234567890".to_string(),
            tax_type: TaxType::CorporateIncome,
            filing_period: TaxFilingPeriod::Annual,
            tax_year: 2023,
            period_month: None,
            due_date: now + chrono::Duration::days(30),
            amount_due_lak: 10_000_000,
            is_filed: false,
            filing_date: None,
            is_paid: false,
            payment_date: None,
            amount_paid_lak: None,
            penalty_lak: None,
        };
        assert!(!filing.is_late());
        assert!(filing.days_late().is_none());
        assert!(!filing.is_payment_complete());
    }

    #[test]
    fn test_withholding_tax_applicable_rates() {
        // Dividend to resident
        let rate = WithholdingTax::get_applicable_rate(&WithholdingPaymentType::Dividend, false);
        assert_eq!(rate, 0.10);

        // Service fee to non-resident
        let rate = WithholdingTax::get_applicable_rate(&WithholdingPaymentType::ServiceFee, true);
        assert_eq!(rate, 0.15);

        // Service fee to resident (no withholding)
        let rate = WithholdingTax::get_applicable_rate(&WithholdingPaymentType::ServiceFee, false);
        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_income_type() {
        let employment = IncomeType::Employment {
            employer: "ABC Company".to_string(),
            employer_tax_id: Some("1234567890".to_string()),
        };
        assert!(matches!(employment, IncomeType::Employment { .. }));

        let business = IncomeType::Business {
            business_name: "My Shop".to_string(),
            registration_number: Some("REG-001".to_string()),
        };
        assert!(matches!(business, IncomeType::Business { .. }));
    }

    #[test]
    fn test_corporate_entity_type() {
        let limited = CorporateEntityType::LimitedCompany;
        assert!(matches!(limited, CorporateEntityType::LimitedCompany));

        let foreign_branch = CorporateEntityType::ForeignBranch {
            home_country: "Thailand".to_string(),
        };
        assert!(matches!(
            foreign_branch,
            CorporateEntityType::ForeignBranch { .. }
        ));
    }

    #[test]
    fn test_customs_declaration_type() {
        let import = CustomsDeclarationType::Import;
        assert!(matches!(import, CustomsDeclarationType::Import));

        let export = CustomsDeclarationType::Export;
        assert!(matches!(export, CustomsDeclarationType::Export));
    }

    #[test]
    fn test_tax_payment_method() {
        let cash = TaxPaymentMethod::Cash;
        assert!(matches!(cash, TaxPaymentMethod::Cash));

        let bank = TaxPaymentMethod::BankTransfer {
            bank_name: "BCEL".to_string(),
            reference: "TXN-2024-001".to_string(),
        };
        assert!(matches!(bank, TaxPaymentMethod::BankTransfer { .. }));
    }
}
