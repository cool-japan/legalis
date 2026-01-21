//! Goods and Services Tax Types
//!
//! Types for GST compliance under CGST Act 2017, SGST Acts, and IGST Act 2017

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// GST registration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Active registration
    Active,
    /// Suspended registration
    Suspended,
    /// Cancelled registration
    Cancelled,
    /// Surrendered registration
    Surrendered,
    /// Provisional registration (migrated)
    Provisional,
}

/// GST registration type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationType {
    /// Regular taxpayer
    Regular,
    /// Composition scheme taxpayer (Section 10)
    Composition,
    /// Casual taxable person
    Casual,
    /// Non-resident taxable person
    NonResident,
    /// Input Service Distributor (ISD)
    InputServiceDistributor,
    /// Tax Deductor at Source (TDS)
    TaxDeductor,
    /// Tax Collector at Source (TCS) - E-commerce
    TaxCollector,
    /// UN body / Embassy / SEZ unit
    SpecialEconomicZone,
    /// Embassy / UN body
    UnitedNationsBody,
    /// E-commerce operator
    EcommerceOperator,
}

impl RegistrationType {
    /// Check if turnover threshold applies
    pub fn has_threshold(&self) -> bool {
        matches!(self, Self::Regular | Self::Composition)
    }

    /// Get applicable form for registration
    pub fn registration_form(&self) -> &'static str {
        match self {
            Self::Regular | Self::Composition | Self::Casual | Self::NonResident => "GST REG-01",
            Self::InputServiceDistributor => "GST REG-01",
            Self::TaxDeductor => "GST REG-07",
            Self::TaxCollector | Self::EcommerceOperator => "GST REG-01",
            Self::SpecialEconomicZone | Self::UnitedNationsBody => "GST REG-13",
        }
    }
}

/// GST taxpayer category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxpayerCategory {
    /// Normal taxpayer
    Normal,
    /// SEZ unit
    SezUnit,
    /// SEZ developer
    SezDeveloper,
    /// Embassy / UN body
    Embassy,
    /// Government department
    Government,
}

/// State for GST purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GstState {
    /// Jammu and Kashmir (01)
    JammuKashmir,
    /// Himachal Pradesh (02)
    HimachalPradesh,
    /// Punjab (03)
    Punjab,
    /// Chandigarh (04)
    Chandigarh,
    /// Uttarakhand (05)
    Uttarakhand,
    /// Haryana (06)
    Haryana,
    /// Delhi (07)
    Delhi,
    /// Rajasthan (08)
    Rajasthan,
    /// Uttar Pradesh (09)
    UttarPradesh,
    /// Bihar (10)
    Bihar,
    /// Sikkim (11)
    Sikkim,
    /// Arunachal Pradesh (12)
    ArunachalPradesh,
    /// Nagaland (13)
    Nagaland,
    /// Manipur (14)
    Manipur,
    /// Mizoram (15)
    Mizoram,
    /// Tripura (16)
    Tripura,
    /// Meghalaya (17)
    Meghalaya,
    /// Assam (18)
    Assam,
    /// West Bengal (19)
    WestBengal,
    /// Jharkhand (20)
    Jharkhand,
    /// Odisha (21)
    Odisha,
    /// Chhattisgarh (22)
    Chhattisgarh,
    /// Madhya Pradesh (23)
    MadhyaPradesh,
    /// Gujarat (24)
    Gujarat,
    /// Dadra and Nagar Haveli (26)
    DadraNagarHaveli,
    /// Maharashtra (27)
    Maharashtra,
    /// Karnataka (29)
    Karnataka,
    /// Goa (30)
    Goa,
    /// Lakshadweep (31)
    Lakshadweep,
    /// Kerala (32)
    Kerala,
    /// Tamil Nadu (33)
    TamilNadu,
    /// Puducherry (34)
    Puducherry,
    /// Andaman and Nicobar (35)
    AndamanNicobar,
    /// Telangana (36)
    Telangana,
    /// Andhra Pradesh (37)
    AndhraPradesh,
    /// Ladakh (38)
    Ladakh,
    /// Other Territory (97)
    OtherTerritory,
}

impl GstState {
    /// Get state code
    pub fn code(&self) -> &'static str {
        match self {
            Self::JammuKashmir => "01",
            Self::HimachalPradesh => "02",
            Self::Punjab => "03",
            Self::Chandigarh => "04",
            Self::Uttarakhand => "05",
            Self::Haryana => "06",
            Self::Delhi => "07",
            Self::Rajasthan => "08",
            Self::UttarPradesh => "09",
            Self::Bihar => "10",
            Self::Sikkim => "11",
            Self::ArunachalPradesh => "12",
            Self::Nagaland => "13",
            Self::Manipur => "14",
            Self::Mizoram => "15",
            Self::Tripura => "16",
            Self::Meghalaya => "17",
            Self::Assam => "18",
            Self::WestBengal => "19",
            Self::Jharkhand => "20",
            Self::Odisha => "21",
            Self::Chhattisgarh => "22",
            Self::MadhyaPradesh => "23",
            Self::Gujarat => "24",
            Self::DadraNagarHaveli => "26",
            Self::Maharashtra => "27",
            Self::Karnataka => "29",
            Self::Goa => "30",
            Self::Lakshadweep => "31",
            Self::Kerala => "32",
            Self::TamilNadu => "33",
            Self::Puducherry => "34",
            Self::AndamanNicobar => "35",
            Self::Telangana => "36",
            Self::AndhraPradesh => "37",
            Self::Ladakh => "38",
            Self::OtherTerritory => "97",
        }
    }

    /// Check if special category state (lower threshold)
    pub fn is_special_category(&self) -> bool {
        matches!(
            self,
            Self::ArunachalPradesh
                | Self::Assam
                | Self::JammuKashmir
                | Self::Manipur
                | Self::Meghalaya
                | Self::Mizoram
                | Self::Nagaland
                | Self::Sikkim
                | Self::Tripura
                | Self::HimachalPradesh
                | Self::Uttarakhand
                | Self::Ladakh
        )
    }

    /// Get registration threshold for goods
    pub fn goods_threshold(&self) -> u64 {
        if self.is_special_category() {
            2_000_000 // Rs. 20 lakhs
        } else {
            4_000_000 // Rs. 40 lakhs
        }
    }

    /// Get registration threshold for services
    pub fn services_threshold(&self) -> u64 {
        if self.is_special_category() {
            1_000_000 // Rs. 10 lakhs
        } else {
            2_000_000 // Rs. 20 lakhs
        }
    }
}

/// GSTIN (GST Identification Number)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gstin {
    /// State code (2 digits)
    pub state_code: String,
    /// PAN (10 characters)
    pub pan: String,
    /// Entity number (2 characters)
    pub entity_number: String,
    /// Default code ('Z')
    pub default_code: char,
    /// Check digit
    pub check_digit: char,
    /// Full GSTIN string
    pub full: String,
}

impl Gstin {
    /// Parse from string
    pub fn parse(gstin: &str) -> Option<Self> {
        if gstin.len() != 15 {
            return None;
        }

        let chars: Vec<char> = gstin.chars().collect();

        Some(Self {
            state_code: chars[0..2].iter().collect(),
            pan: chars[2..12].iter().collect(),
            entity_number: chars[12..14].iter().collect(),
            default_code: chars[13],
            check_digit: chars[14],
            full: gstin.to_string(),
        })
    }

    /// Validate GSTIN format
    pub fn is_valid(&self) -> bool {
        // Basic format validation
        if self.full.len() != 15 {
            return false;
        }

        // State code should be valid
        let state_code: u32 = self.state_code.parse().unwrap_or(0);
        if state_code == 0 || state_code > 38 || state_code == 25 || state_code == 28 {
            return false;
        }

        // PAN should be alphanumeric
        if !self.pan.chars().all(|c| c.is_alphanumeric()) {
            return false;
        }

        true
    }
}

/// Supply type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupplyType {
    /// Intra-state supply (CGST + SGST)
    IntraState,
    /// Inter-state supply (IGST)
    InterState,
    /// Export (zero-rated)
    Export,
    /// SEZ supply (zero-rated)
    SezSupply,
}

impl SupplyType {
    /// Get applicable taxes
    pub fn applicable_taxes(&self) -> Vec<&'static str> {
        match self {
            Self::IntraState => vec!["CGST", "SGST"],
            Self::InterState => vec!["IGST"],
            Self::Export | Self::SezSupply => vec![], // Zero-rated
        }
    }
}

/// Supply category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupplyCategory {
    /// Supply of goods
    Goods,
    /// Supply of services
    Services,
    /// Mixed supply (Section 2(74))
    Mixed,
    /// Composite supply (Section 2(30))
    Composite,
}

/// GST rate slab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GstRate {
    /// Nil rated (0%)
    Nil,
    /// 5% rate
    Rate5,
    /// 12% rate
    Rate12,
    /// 18% rate
    Rate18,
    /// 28% rate
    Rate28,
    /// Exempt
    Exempt,
    /// Special rate (custom)
    Special(u32),
}

impl GstRate {
    /// Get rate percentage
    pub fn percentage(&self) -> f64 {
        match self {
            Self::Nil | Self::Exempt => 0.0,
            Self::Rate5 => 5.0,
            Self::Rate12 => 12.0,
            Self::Rate18 => 18.0,
            Self::Rate28 => 28.0,
            Self::Special(rate) => *rate as f64,
        }
    }

    /// Get CGST component (for intra-state)
    pub fn cgst(&self) -> f64 {
        self.percentage() / 2.0
    }

    /// Get SGST component (for intra-state)
    pub fn sgst(&self) -> f64 {
        self.percentage() / 2.0
    }

    /// Get IGST component (for inter-state)
    pub fn igst(&self) -> f64 {
        self.percentage()
    }
}

/// Compensation cess items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompensationCess {
    /// Luxury cars
    LuxuryCar,
    /// Tobacco products
    Tobacco,
    /// Aerated beverages
    AeratedBeverages,
    /// Coal and lignite
    Coal,
    /// Pan masala
    PanMasala,
    /// None
    None,
}

impl CompensationCess {
    /// Get cess rate (ad valorem or specific)
    pub fn rate_description(&self) -> &'static str {
        match self {
            Self::LuxuryCar => "15-22% depending on specifications",
            Self::Tobacco => "Rs. 4170 per 1000 + 5% or higher",
            Self::AeratedBeverages => "12%",
            Self::Coal => "Rs. 400 per tonne",
            Self::PanMasala => "60%",
            Self::None => "0%",
        }
    }
}

/// Invoice type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvoiceType {
    /// Tax invoice (Section 31)
    TaxInvoice,
    /// Bill of supply (for exempt/nil rated)
    BillOfSupply,
    /// Debit note (Section 34)
    DebitNote,
    /// Credit note (Section 34)
    CreditNote,
    /// Revised invoice
    RevisedInvoice,
    /// Export invoice
    ExportInvoice,
    /// SEZ invoice
    SezInvoice,
}

impl InvoiceType {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::TaxInvoice | Self::BillOfSupply => "Section 31",
            Self::DebitNote | Self::CreditNote => "Section 34",
            Self::RevisedInvoice => "Section 31(3)",
            Self::ExportInvoice | Self::SezInvoice => "Section 31",
        }
    }
}

/// Invoice details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    /// Invoice number
    pub number: String,
    /// Invoice date
    pub date: NaiveDate,
    /// Invoice type
    pub invoice_type: InvoiceType,
    /// Supplier GSTIN
    pub supplier_gstin: String,
    /// Recipient GSTIN (optional for B2C)
    pub recipient_gstin: Option<String>,
    /// Supply type
    pub supply_type: SupplyType,
    /// Taxable value
    pub taxable_value: f64,
    /// CGST amount
    pub cgst: f64,
    /// SGST amount
    pub sgst: f64,
    /// IGST amount
    pub igst: f64,
    /// Cess amount
    pub cess: f64,
    /// Total value
    pub total_value: f64,
    /// Place of supply
    pub place_of_supply: GstState,
    /// HSN/SAC code
    pub hsn_sac: String,
    /// Reverse charge applicable
    pub reverse_charge: bool,
}

/// E-way bill
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EwayBill {
    /// E-way bill number
    pub ewb_number: String,
    /// Generation date
    pub generated_date: NaiveDate,
    /// Valid until
    pub valid_until: NaiveDate,
    /// Document type
    pub doc_type: EwayDocType,
    /// Document number
    pub doc_number: String,
    /// Supplier GSTIN
    pub supplier_gstin: String,
    /// Recipient GSTIN
    pub recipient_gstin: Option<String>,
    /// From state
    pub from_state: GstState,
    /// To state
    pub to_state: GstState,
    /// Distance in km
    pub distance_km: u32,
    /// Vehicle number
    pub vehicle_number: Option<String>,
    /// Transport mode
    pub transport_mode: TransportMode,
    /// Taxable value
    pub taxable_value: f64,
    /// Total value with tax
    pub total_value: f64,
}

/// E-way bill document type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EwayDocType {
    /// Tax invoice
    TaxInvoice,
    /// Bill of supply
    BillOfSupply,
    /// Bill of entry
    BillOfEntry,
    /// Delivery challan
    DeliveryChallan,
    /// Others
    Others,
}

/// Transport mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportMode {
    /// Road
    Road,
    /// Rail
    Rail,
    /// Air
    Air,
    /// Ship
    Ship,
}

impl EwayBill {
    /// Check if e-way bill is required based on value
    pub fn is_required(taxable_value: f64, supply_type: SupplyType) -> bool {
        match supply_type {
            SupplyType::InterState => taxable_value > 50_000.0,
            SupplyType::IntraState => taxable_value > 50_000.0, // May vary by state
            _ => false,
        }
    }

    /// Get validity period based on distance
    pub fn validity_days(distance_km: u32) -> u32 {
        // 1 day per 200 km (or part thereof) for normal cargo
        // 1 day per 20 km for over-dimensional cargo
        distance_km.div_ceil(200).max(1)
    }
}

/// GST return type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReturnType {
    /// GSTR-1: Outward supplies
    Gstr1,
    /// GSTR-2A: Auto-populated inward supplies (view only)
    Gstr2a,
    /// GSTR-2B: Auto-drafted ITC statement (view only)
    Gstr2b,
    /// GSTR-3B: Summary return with tax payment
    Gstr3b,
    /// GSTR-4: Composition taxpayer quarterly
    Gstr4,
    /// GSTR-5: Non-resident taxpayer
    Gstr5,
    /// GSTR-6: Input Service Distributor
    Gstr6,
    /// GSTR-7: TDS return
    Gstr7,
    /// GSTR-8: TCS return (e-commerce)
    Gstr8,
    /// GSTR-9: Annual return
    Gstr9,
    /// GSTR-9C: Reconciliation statement (audit)
    Gstr9c,
    /// GSTR-10: Final return
    Gstr10,
    /// GSTR-11: UIN holder return
    Gstr11,
    /// ITC-04: Job work return
    Itc04,
    /// CMP-08: Composition scheme quarterly statement
    Cmp08,
}

impl ReturnType {
    /// Get due date description
    pub fn due_date_description(&self) -> &'static str {
        match self {
            Self::Gstr1 => "11th of following month (monthly) / 13th after quarter (QRMP)",
            Self::Gstr3b => "20th of following month (monthly) / 22nd-24th (QRMP)",
            Self::Gstr4 => "18th of month following quarter",
            Self::Gstr9 => "31st December of following FY",
            Self::Gstr9c => "31st December of following FY",
            Self::Gstr10 => "Within 3 months of cancellation",
            Self::Cmp08 => "18th of month following quarter",
            _ => "As prescribed",
        }
    }

    /// Is this return filed monthly?
    pub fn is_monthly(&self) -> bool {
        matches!(
            self,
            Self::Gstr1 | Self::Gstr3b | Self::Gstr6 | Self::Gstr7 | Self::Gstr8
        )
    }

    /// Is this return filed quarterly?
    pub fn is_quarterly(&self) -> bool {
        matches!(self, Self::Gstr4 | Self::Cmp08 | Self::Itc04)
    }

    /// Is this return filed annually?
    pub fn is_annual(&self) -> bool {
        matches!(self, Self::Gstr9 | Self::Gstr9c)
    }
}

/// Return filing status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatus {
    /// Return type
    pub return_type: ReturnType,
    /// Tax period
    pub tax_period: String,
    /// Filing status
    pub status: FilingStatus,
    /// Date of filing
    pub filing_date: Option<NaiveDate>,
    /// Due date
    pub due_date: NaiveDate,
    /// Late fee paid
    pub late_fee: f64,
    /// ARN (Acknowledgement Reference Number)
    pub arn: Option<String>,
}

/// Filing status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilingStatus {
    /// Not yet filed
    NotFiled,
    /// Filed within due date
    Filed,
    /// Filed after due date
    FiledLate,
    /// Nil return filed
    NilReturn,
}

/// Input Tax Credit (ITC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputTaxCredit {
    /// CGST credit
    pub cgst: f64,
    /// SGST credit
    pub sgst: f64,
    /// IGST credit
    pub igst: f64,
    /// Cess credit
    pub cess: f64,
    /// Credit type
    pub credit_type: ItcType,
    /// Eligible for credit
    pub eligible: bool,
    /// If blocked, reason
    pub blocked_reason: Option<BlockedItcReason>,
}

/// ITC type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItcType {
    /// Inputs
    Inputs,
    /// Input services
    InputServices,
    /// Capital goods
    CapitalGoods,
    /// Import of goods
    ImportGoods,
    /// Import of services
    ImportServices,
}

/// Blocked ITC reasons (Section 17(5))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockedItcReason {
    /// Motor vehicles (except specified)
    MotorVehicle,
    /// Food and beverages
    FoodBeverages,
    /// Health services
    HealthServices,
    /// Rent-a-cab
    RentACab,
    /// Travel benefits to employees
    TravelBenefits,
    /// Works contract for immovable property
    WorksContract,
    /// Construction of immovable property
    Construction,
    /// Beauty treatment
    BeautyTreatment,
    /// Membership of club
    ClubMembership,
    /// Life insurance
    LifeInsurance,
    /// Personal consumption
    PersonalConsumption,
    /// Gifts and samples
    GiftsSamples,
    /// Lost, stolen, destroyed goods
    LostStolenDestroyed,
}

impl BlockedItcReason {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        "Section 17(5)"
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::MotorVehicle => {
                "Motor vehicles except when used for transport, training, or specified purposes"
            }
            Self::FoodBeverages => {
                "Food and beverages, outdoor catering, beauty treatment, health services, etc."
            }
            Self::HealthServices => "Health services (unless obligatory under law)",
            Self::RentACab => "Rent-a-cab, life insurance, health insurance (except obligatory)",
            Self::TravelBenefits => "Travel benefits extended to employees on vacation",
            Self::WorksContract => "Works contract services for construction of immovable property",
            Self::Construction => "Construction of immovable property on own account",
            Self::BeautyTreatment => {
                "Beauty treatment, health services, cosmetic and plastic surgery"
            }
            Self::ClubMembership => "Membership of club, health and fitness centre",
            Self::LifeInsurance => "Life insurance, health insurance (except where obligatory)",
            Self::PersonalConsumption => "Goods/services for personal consumption",
            Self::GiftsSamples => {
                "Goods lost, stolen, destroyed, written off or disposed as gift/free sample"
            }
            Self::LostStolenDestroyed => "Goods lost, stolen, destroyed, written off",
        }
    }
}

/// Composition scheme details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositionScheme {
    /// Eligible for composition
    pub eligible: bool,
    /// Business type
    pub business_type: CompositionBusinessType,
    /// Tax rate applicable
    pub rate: f64,
    /// Turnover limit
    pub turnover_limit: u64,
    /// Previous year turnover
    pub previous_turnover: u64,
}

/// Composition business type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompositionBusinessType {
    /// Manufacturer
    Manufacturer,
    /// Trader
    Trader,
    /// Restaurant service
    Restaurant,
    /// Other service provider
    ServiceProvider,
}

impl CompositionBusinessType {
    /// Get composition rate
    pub fn composition_rate(&self) -> f64 {
        match self {
            Self::Manufacturer | Self::Trader => 1.0, // 1% (0.5% CGST + 0.5% SGST)
            Self::Restaurant => 5.0,                  // 5% (2.5% CGST + 2.5% SGST)
            Self::ServiceProvider => 6.0,             // 6% (3% CGST + 3% SGST)
        }
    }

    /// Get turnover limit
    pub fn turnover_limit(&self) -> u64 {
        match self {
            Self::Manufacturer | Self::Trader => 15_000_000, // Rs. 1.5 crore
            Self::Restaurant => 15_000_000,                  // Rs. 1.5 crore
            Self::ServiceProvider => 5_000_000,              // Rs. 50 lakh
        }
    }
}

/// Reverse charge mechanism
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReverseCharge {
    /// Import of services
    ImportServices,
    /// Legal services from advocate
    LegalServices,
    /// Goods transport agency
    GoodsTransportAgency,
    /// Sponsorship services
    SponsorshipServices,
    /// Security services
    SecurityServices,
    /// Renting of motor vehicle
    RentingMotorVehicle,
    /// Director's remuneration
    DirectorRemuneration,
    /// Supply by unregistered person (Section 9(4))
    UnregisteredSupply,
}

impl ReverseCharge {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::UnregisteredSupply => "Section 9(4)",
            _ => "Section 9(3)",
        }
    }

    /// Get notification reference
    pub fn notification(&self) -> &'static str {
        match self {
            Self::ImportServices => "N/N 10/2017-IGST",
            Self::LegalServices
            | Self::GoodsTransportAgency
            | Self::SponsorshipServices
            | Self::SecurityServices
            | Self::RentingMotorVehicle
            | Self::DirectorRemuneration => "N/N 13/2017-CT (Rate)",
            Self::UnregisteredSupply => "Section 9(4) - Currently suspended",
        }
    }
}

/// HSN/SAC code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HsnSacCode {
    /// Code
    pub code: String,
    /// Description
    pub description: String,
    /// Type (goods or services)
    pub code_type: HsnSacType,
    /// Applicable GST rate
    pub gst_rate: GstRate,
    /// Cess applicable
    pub cess: Option<CompensationCess>,
}

/// HSN/SAC type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsnSacType {
    /// HSN for goods
    Hsn,
    /// SAC for services
    Sac,
}

/// GST registration details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GstRegistration {
    /// GSTIN
    pub gstin: Gstin,
    /// Legal name
    pub legal_name: String,
    /// Trade name
    pub trade_name: Option<String>,
    /// Registration type
    pub registration_type: RegistrationType,
    /// Status
    pub status: RegistrationStatus,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Cancellation date (if cancelled)
    pub cancellation_date: Option<NaiveDate>,
    /// Principal place of business state
    pub state: GstState,
    /// Nature of business
    pub nature_of_business: Vec<NatureOfBusiness>,
    /// Composition scheme opted
    pub composition_opted: bool,
}

/// Nature of business
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NatureOfBusiness {
    /// Factory/Manufacturing
    Factory,
    /// Wholesale business
    Wholesale,
    /// Retail business
    Retail,
    /// Warehouse/Depot
    WarehouseDepot,
    /// Bonded warehouse
    BondedWarehouse,
    /// Supplier of services
    ServiceSupplier,
    /// Office/Sale office
    Office,
    /// Works contract
    WorksContract,
    /// Leasing business
    LeasingBusiness,
    /// Export
    Export,
    /// Import
    Import,
    /// EOU/STP/EHTP
    EouStpEhtp,
    /// SEZ
    Sez,
    /// Others
    Others,
}

/// Tax liability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxLiability {
    /// Tax period
    pub period: String,
    /// CGST liability
    pub cgst: f64,
    /// SGST liability
    pub sgst: f64,
    /// IGST liability
    pub igst: f64,
    /// Cess liability
    pub cess: f64,
    /// Interest
    pub interest: f64,
    /// Late fee
    pub late_fee: f64,
    /// Penalty
    pub penalty: f64,
}

impl TaxLiability {
    /// Total tax liability
    pub fn total_tax(&self) -> f64 {
        self.cgst + self.sgst + self.igst + self.cess
    }

    /// Total liability including interest, fee, penalty
    pub fn total_liability(&self) -> f64 {
        self.total_tax() + self.interest + self.late_fee + self.penalty
    }
}

/// ITC utilization order (Section 49)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItcUtilization {
    /// IGST credit for IGST liability
    IgstForIgst,
    /// IGST credit for CGST liability
    IgstForCgst,
    /// IGST credit for SGST liability
    IgstForSgst,
    /// CGST credit for CGST liability
    CgstForCgst,
    /// CGST credit for IGST liability
    CgstForIgst,
    /// SGST credit for SGST liability
    SgstForSgst,
    /// SGST credit for IGST liability
    SgstForIgst,
}

impl ItcUtilization {
    /// Get utilization order priority
    pub fn priority(&self) -> u32 {
        match self {
            Self::IgstForIgst => 1,
            Self::IgstForCgst => 2,
            Self::IgstForSgst => 3,
            Self::CgstForCgst => 4,
            Self::CgstForIgst => 5,
            Self::SgstForSgst => 6,
            Self::SgstForIgst => 7,
        }
    }
}

/// Refund type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefundType {
    /// Export with payment of tax
    ExportWithTax,
    /// Export under LUT (zero-rated)
    ExportUnderLut,
    /// Supply to SEZ with tax
    SezWithTax,
    /// Supply to SEZ under LUT
    SezUnderLut,
    /// Inverted duty structure
    InvertedDutyStructure,
    /// Excess balance in electronic cash ledger
    ExcessBalance,
    /// Assessment/appeal order
    AssessmentOrder,
    /// Provisional assessment
    ProvisionalAssessment,
    /// Tax wrongly collected
    TaxWronglyCollected,
    /// International tourist
    InternationalTourist,
}

impl RefundType {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        "Section 54"
    }

    /// Time limit for claiming refund (months)
    pub fn time_limit_months(&self) -> u32 {
        24 // 2 years from relevant date
    }
}

/// Anti-profiteering provisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntiProfiteering {
    /// Original price before rate reduction
    pub original_price: f64,
    /// Original tax rate
    pub original_tax_rate: f64,
    /// New price after rate reduction
    pub new_price: f64,
    /// New tax rate
    pub new_tax_rate: f64,
    /// Commensurate reduction expected
    pub expected_reduction: f64,
    /// Actual reduction
    pub actual_reduction: f64,
    /// Profiteering amount (if any)
    pub profiteering_amount: f64,
}

impl AntiProfiteering {
    /// Calculate expected price reduction
    pub fn calculate_expected_reduction(
        original_price: f64,
        original_rate: f64,
        new_rate: f64,
    ) -> f64 {
        let tax_reduction = original_rate - new_rate;
        let base_price = original_price / (1.0 + original_rate / 100.0);
        base_price * tax_reduction / 100.0
    }

    /// Check if profiteering occurred
    pub fn is_profiteering(&self) -> bool {
        self.actual_reduction < self.expected_reduction * 0.95 // 5% tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gstin_validation() {
        let gstin = Gstin::parse("27AABCU9603R1ZM").expect("valid GSTIN");
        assert!(gstin.is_valid());
        assert_eq!(gstin.state_code, "27");
        assert_eq!(gstin.pan, "AABCU9603R");
    }

    #[test]
    fn test_gst_rate() {
        assert_eq!(GstRate::Rate18.percentage(), 18.0);
        assert_eq!(GstRate::Rate18.cgst(), 9.0);
        assert_eq!(GstRate::Rate18.sgst(), 9.0);
        assert_eq!(GstRate::Rate18.igst(), 18.0);
    }

    #[test]
    fn test_special_category_state() {
        assert!(GstState::ArunachalPradesh.is_special_category());
        assert!(!GstState::Maharashtra.is_special_category());
    }

    #[test]
    fn test_threshold_amounts() {
        assert_eq!(GstState::Maharashtra.goods_threshold(), 4_000_000);
        assert_eq!(GstState::ArunachalPradesh.goods_threshold(), 2_000_000);
    }

    #[test]
    fn test_eway_bill_validity() {
        assert_eq!(EwayBill::validity_days(100), 1);
        assert_eq!(EwayBill::validity_days(200), 1);
        assert_eq!(EwayBill::validity_days(201), 2);
        assert_eq!(EwayBill::validity_days(500), 3);
    }

    #[test]
    fn test_composition_rate() {
        assert_eq!(
            CompositionBusinessType::Manufacturer.composition_rate(),
            1.0
        );
        assert_eq!(CompositionBusinessType::Restaurant.composition_rate(), 5.0);
        assert_eq!(
            CompositionBusinessType::ServiceProvider.composition_rate(),
            6.0
        );
    }

    #[test]
    fn test_itc_utilization_priority() {
        assert!(ItcUtilization::IgstForIgst.priority() < ItcUtilization::CgstForCgst.priority());
    }

    #[test]
    fn test_supply_type_taxes() {
        assert_eq!(
            SupplyType::IntraState.applicable_taxes(),
            vec!["CGST", "SGST"]
        );
        assert_eq!(SupplyType::InterState.applicable_taxes(), vec!["IGST"]);
        assert!(SupplyType::Export.applicable_taxes().is_empty());
    }

    #[test]
    fn test_tax_liability_total() {
        let liability = TaxLiability {
            period: "Mar-2024".to_string(),
            cgst: 10000.0,
            sgst: 10000.0,
            igst: 0.0,
            cess: 1000.0,
            interest: 500.0,
            late_fee: 100.0,
            penalty: 0.0,
        };
        assert_eq!(liability.total_tax(), 21000.0);
        assert_eq!(liability.total_liability(), 21600.0);
    }

    #[test]
    fn test_anti_profiteering() {
        let expected = AntiProfiteering::calculate_expected_reduction(118.0, 18.0, 12.0);
        // Base price = 118 / 1.18 = 100
        // Expected reduction = 100 * 6 / 100 = 6
        assert!((expected - 6.0).abs() < 0.01);
    }
}
