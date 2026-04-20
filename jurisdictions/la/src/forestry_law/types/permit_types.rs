//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::functions::{MIN_DIAMETER_HARDWOOD_CM, MIN_DIAMETER_ROSEWOOD_CM, MIN_DIAMETER_TEAK_CM};
use super::types_3::{ConcessionType, NtfpType, PermitStatus, ProtectionCategory};

/// Log entry for tracking (ບັນທຶກໄມ້ທ່ອນສຳລັບການຕິດຕາມ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogEntry {
    /// Log ID/marking number (ເລກໝາຍໄມ້ທ່ອນ)
    pub log_id: String,
    /// Species (ຊະນິດ)
    pub species: TreeSpecies,
    /// Length in meters (ຄວາມຍາວເປັນແມັດ)
    pub length_meters: f64,
    /// Diameter at breast height in cm (ເສັ້ນຜ່ານສູນກາງ DBH)
    pub diameter_cm: u32,
    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,
    /// Harvest permit reference (ອ້າງອິງໃບອະນຸຍາດຕັດ)
    pub harvest_permit_reference: String,
    /// Harvest date (ວັນທີຕັດ)
    pub harvest_date: String,
    /// Harvest location province (ແຂວງທີ່ຕັດ)
    pub harvest_province: String,
    /// Harvest location district (ເມືອງທີ່ຕັດ)
    pub harvest_district: String,
    /// Current location (ສະຖານທີ່ປະຈຸບັນ)
    pub current_location: String,
    /// Chain of custody entries (ບັນທຶກຕ່ອງໂສ້ການຄຸ້ມຄອງ)
    pub chain_of_custody: Vec<ChainOfCustodyEntry>,
    /// Is CITES listed (ຢູ່ໃນບັນຊີ CITES)
    pub is_cites_listed: bool,
    /// Quality grade (ເກຣດຄຸນນະພາບ)
    pub quality_grade: Option<String>,
}
/// Builder for ForestProductExportPermit
#[derive(Debug, Default)]
pub struct ForestProductExportPermitBuilder {
    pub(super) permit: ForestProductExportPermit,
}
impl ForestProductExportPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }
    /// Set exporter name
    pub fn exporter_name(mut self, name: impl Into<String>) -> Self {
        self.permit.exporter_name = name.into();
        self
    }
    /// Set product type
    pub fn product_type(mut self, product_type: ExportProductType) -> Self {
        self.permit.product_type = product_type;
        self
    }
    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = Some(species);
        self
    }
    /// Set quantity
    pub fn quantity(mut self, quantity: f64, unit: impl Into<String>) -> Self {
        self.permit.quantity = quantity;
        self.permit.quantity_unit = unit.into();
        self
    }
    /// Set destination country
    pub fn destination_country(mut self, country: impl Into<String>) -> Self {
        self.permit.destination_country = country.into();
        self
    }
    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }
    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }
    /// Set CITES permit number
    pub fn cites_permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.cites_permit_number = Some(number.into());
        self
    }
    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }
    /// Add source permit
    pub fn add_source_permit(mut self, permit: impl Into<String>) -> Self {
        self.permit.source_permits.push(permit.into());
        self
    }
    /// Build the permit
    pub fn build(self) -> ForestProductExportPermit {
        self.permit
    }
}
/// Builder for LogEntry
#[derive(Debug, Default)]
pub struct LogEntryBuilder {
    pub(super) entry: LogEntry,
}
impl LogEntryBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set log ID
    pub fn log_id(mut self, id: impl Into<String>) -> Self {
        self.entry.log_id = id.into();
        self
    }
    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.entry.species = species;
        self.entry.is_cites_listed = species.is_cites_listed();
        self
    }
    /// Set dimensions
    pub fn dimensions(mut self, length_m: f64, diameter_cm: u32) -> Self {
        self.entry.length_meters = length_m;
        self.entry.diameter_cm = diameter_cm;
        let radius_m = f64::from(diameter_cm) / 200.0;
        self.entry.volume_cubic_meters =
            std::f64::consts::PI * radius_m * radius_m * length_m * 0.7;
        self
    }
    /// Set harvest permit reference
    pub fn harvest_permit_reference(mut self, reference: impl Into<String>) -> Self {
        self.entry.harvest_permit_reference = reference.into();
        self
    }
    /// Set harvest date
    pub fn harvest_date(mut self, date: impl Into<String>) -> Self {
        self.entry.harvest_date = date.into();
        self
    }
    /// Set harvest location
    pub fn harvest_location(
        mut self,
        province: impl Into<String>,
        district: impl Into<String>,
    ) -> Self {
        self.entry.harvest_province = province.into();
        self.entry.harvest_district = district.into();
        self
    }
    /// Set current location
    pub fn current_location(mut self, location: impl Into<String>) -> Self {
        self.entry.current_location = location.into();
        self
    }
    /// Add chain of custody entry
    pub fn add_custody_entry(mut self, entry: ChainOfCustodyEntry) -> Self {
        self.entry.chain_of_custody.push(entry);
        self
    }
    /// Build the log entry
    pub fn build(self) -> LogEntry {
        self.entry
    }
}
/// Forest product export permit (ໃບອະນຸຍາດສົ່ງອອກຜະລິດຕະພັນປ່າໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestProductExportPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Exporter name (ຊື່ຜູ້ສົ່ງອອກ)
    pub exporter_name: String,
    /// Exporter name in Lao (ຊື່ຜູ້ສົ່ງອອກເປັນພາສາລາວ)
    pub exporter_name_lao: Option<String>,
    /// Product type (ປະເພດຜະລິດຕະພັນ)
    pub product_type: ExportProductType,
    /// Species if applicable (ຊະນິດຖ້າມີ)
    pub species: Option<TreeSpecies>,
    /// Quantity (ປະລິມານ)
    pub quantity: f64,
    /// Quantity unit (ໜ່ວຍ)
    pub quantity_unit: String,
    /// Value in USD (ມູນຄ່າເປັນ USD)
    pub value_usd: Option<f64>,
    /// Destination country (ປະເທດປາຍທາງ)
    pub destination_country: String,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
    /// CITES permit number if required (ເລກໃບອະນຸຍາດ CITES)
    pub cites_permit_number: Option<String>,
    /// Phytosanitary certificate (ໃບຢັ້ງຢືນສຸຂານາໄມພືດ)
    pub phytosanitary_certificate: Option<String>,
    /// Origin certificate (ໃບຢັ້ງຢືນແຫຼ່ງກຳເນີດ)
    pub origin_certificate: Option<String>,
    /// Source permit references (ອ້າງອິງໃບອະນຸຍາດແຫຼ່ງທີ່ມາ)
    pub source_permits: Vec<String>,
}
/// Tree species type (ປະເພດຊະນິດໄມ້)
///
/// Articles 49-50, 76-80: Species classification and protection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreeSpecies {
    /// Teak (ໄມ້ສັກ) - Tectona grandis
    Teak,
    /// Rosewood (ໄມ້ກໍ່ຫຼວງ/ໄມ້ແດງ) - Dalbergia spp.
    Rosewood,
    /// Agarwood (ໄມ້ກຳລັງ) - Aquilaria spp.
    Agarwood,
    /// Mai Dou (ໄມ້ດູ່) - Pterocarpus macrocarpus
    MaiDou,
    /// Mai Kha (ໄມ້ຄາ) - Shorea spp.
    MaiKha,
    /// Mai Nyeng (ໄມ້ຍາງ) - Dipterocarpus spp.
    MaiNyeng,
    /// Mai Tae (ໄມ້ແຕ້) - Bamboo species
    Bamboo,
    /// Mai Pao (ໄມ້ປໍ) - Various species
    MaiPao,
    /// Pine (ໄມ້ແປກ) - Pinus spp.
    Pine,
    /// Eucalyptus (ໄມ້ຢູຄາລິບຕັດ) - Eucalyptus spp.
    Eucalyptus,
    /// Acacia (ໄມ້ອາເຄເຊຍ) - Acacia spp.
    Acacia,
    /// Rubber (ໄມ້ຢາງພາລາ) - Hevea brasiliensis
    Rubber,
    /// Other hardwood (ໄມ້ແຂງອື່ນໆ)
    OtherHardwood,
    /// Other softwood (ໄມ້ອ່ອນອື່ນໆ)
    OtherSoftwood,
}
impl TreeSpecies {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            TreeSpecies::Teak => "ໄມ້ສັກ",
            TreeSpecies::Rosewood => "ໄມ້ກໍ່ຫຼວງ",
            TreeSpecies::Agarwood => "ໄມ້ກຳລັງ",
            TreeSpecies::MaiDou => "ໄມ້ດູ່",
            TreeSpecies::MaiKha => "ໄມ້ຄາ",
            TreeSpecies::MaiNyeng => "ໄມ້ຍາງ",
            TreeSpecies::Bamboo => "ໄມ້ແຕ້",
            TreeSpecies::MaiPao => "ໄມ້ປໍ",
            TreeSpecies::Pine => "ໄມ້ແປກ",
            TreeSpecies::Eucalyptus => "ໄມ້ຢູຄາລິບຕັດ",
            TreeSpecies::Acacia => "ໄມ້ອາເຄເຊຍ",
            TreeSpecies::Rubber => "ໄມ້ຢາງພາລາ",
            TreeSpecies::OtherHardwood => "ໄມ້ແຂງອື່ນໆ",
            TreeSpecies::OtherSoftwood => "ໄມ້ອ່ອນອື່ນໆ",
        }
    }
    /// Get the scientific name
    pub fn scientific_name(&self) -> &'static str {
        match self {
            TreeSpecies::Teak => "Tectona grandis",
            TreeSpecies::Rosewood => "Dalbergia spp.",
            TreeSpecies::Agarwood => "Aquilaria spp.",
            TreeSpecies::MaiDou => "Pterocarpus macrocarpus",
            TreeSpecies::MaiKha => "Shorea spp.",
            TreeSpecies::MaiNyeng => "Dipterocarpus spp.",
            TreeSpecies::Bamboo => "Bambusa spp.",
            TreeSpecies::MaiPao => "Various spp.",
            TreeSpecies::Pine => "Pinus spp.",
            TreeSpecies::Eucalyptus => "Eucalyptus spp.",
            TreeSpecies::Acacia => "Acacia spp.",
            TreeSpecies::Rubber => "Hevea brasiliensis",
            TreeSpecies::OtherHardwood => "Various hardwood spp.",
            TreeSpecies::OtherSoftwood => "Various softwood spp.",
        }
    }
    /// Get protection category (Articles 77-79)
    pub fn protection_category(&self) -> ProtectionCategory {
        match self {
            TreeSpecies::Rosewood | TreeSpecies::Agarwood => ProtectionCategory::CategoryI,
            TreeSpecies::Teak | TreeSpecies::MaiDou => ProtectionCategory::CategoryII,
            _ => ProtectionCategory::CategoryIII,
        }
    }
    /// Get minimum cutting diameter in cm (Article 49)
    pub fn minimum_diameter_cm(&self) -> u32 {
        match self {
            TreeSpecies::Teak => MIN_DIAMETER_TEAK_CM,
            TreeSpecies::Rosewood => MIN_DIAMETER_ROSEWOOD_CM,
            TreeSpecies::MaiDou | TreeSpecies::MaiKha | TreeSpecies::MaiNyeng => {
                MIN_DIAMETER_HARDWOOD_CM
            }
            TreeSpecies::Agarwood => 25,
            TreeSpecies::Bamboo => 5,
            _ => MIN_DIAMETER_HARDWOOD_CM,
        }
    }
    /// Check if CITES listed
    pub fn is_cites_listed(&self) -> bool {
        matches!(
            self,
            TreeSpecies::Rosewood | TreeSpecies::Agarwood | TreeSpecies::MaiDou
        )
    }
    /// Get CITES appendix if listed
    pub fn cites_appendix(&self) -> Option<&'static str> {
        match self {
            TreeSpecies::Rosewood => Some("II"),
            TreeSpecies::Agarwood => Some("II"),
            TreeSpecies::MaiDou => Some("II"),
            _ => None,
        }
    }
}
/// Community forest enterprise (ວິສາຫະກິດປ່າໄມ້ຊຸມຊົນ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityForestEnterprise {
    /// Enterprise name (ຊື່ວິສາຫະກິດ)
    pub name: String,
    /// Enterprise name in Lao (ຊື່ວິສາຫະກິດເປັນພາສາລາວ)
    pub name_lao: String,
    /// Registration number (ເລກທະບຽນ)
    pub registration_number: String,
    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Member count (ຈຳນວນສະມາຊິກ)
    pub member_count: u32,
    /// Registration date (ວັນທີຂຶ້ນທະບຽນ)
    pub registration_date: String,
    /// Products/services (ຜະລິດຕະພັນ/ບໍລິການ)
    pub products_services: Vec<String>,
    /// Annual revenue in LAK (ລາຍໄດ້ປະຈຳປີ)
    pub annual_revenue_lak: Option<u64>,
    /// Has benefit sharing agreement (ມີຂໍ້ຕົກລົງແບ່ງປັນຜົນປະໂຫຍດ)
    pub has_benefit_sharing: bool,
}
/// Sawmill license (ໃບອະນຸຍາດໂຮງເລື່ອຍ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SawmillLicense {
    /// License number (ເລກໃບອະນຸຍາດ)
    pub license_number: String,
    /// Facility name (ຊື່ໂຮງງານ)
    pub facility_name: String,
    /// Facility name in Lao (ຊື່ໂຮງງານເປັນພາສາລາວ)
    pub facility_name_lao: Option<String>,
    /// Owner name (ຊື່ເຈົ້າຂອງ)
    pub owner_name: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Annual capacity in cubic meters (ກຳລັງການຜະລິດປະຈຳປີ)
    pub annual_capacity_cubic_meters: f64,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
    /// Has environmental compliance (ມີການປະຕິບັດຕາມສິ່ງແວດລ້ອມ)
    pub environmental_compliance: bool,
    /// Log intake tracking system (ລະບົບຕິດຕາມການຮັບໄມ້ເຂົ້າ)
    pub has_log_tracking: bool,
    /// Permitted species (ຊະນິດທີ່ໄດ້ຮັບອະນຸຍາດ)
    pub permitted_species: Vec<TreeSpecies>,
}
/// Concession status (ສະຖານະສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionStatus {
    /// Application pending (ຄຳຮ້ອງລໍຖ້າ)
    ApplicationPending,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Approved (ອະນຸມັດແລ້ວ)
    Approved,
    /// Active (ດຳເນີນການຢູ່)
    Active,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Terminated (ຢຸດຕິ)
    Terminated,
    /// Expired (ໝົດອາຍຸ)
    Expired,
}
impl ConcessionStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionStatus::ApplicationPending => "ຄຳຮ້ອງລໍຖ້າ",
            ConcessionStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            ConcessionStatus::Approved => "ອະນຸມັດແລ້ວ",
            ConcessionStatus::Active => "ດຳເນີນການຢູ່",
            ConcessionStatus::Suspended => "ໂຈະ",
            ConcessionStatus::Terminated => "ຢຸດຕິ",
            ConcessionStatus::Expired => "ໝົດອາຍຸ",
        }
    }
}
/// Export product type (ປະເພດຜະລິດຕະພັນສົ່ງອອກ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExportProductType {
    /// Logs (ໄມ້ທ່ອນ)
    Logs,
    /// Sawn timber (ໄມ້ແປຮູບ)
    SawnTimber,
    /// Plywood (ໄມ້ອັດ)
    Plywood,
    /// Furniture (ເຟີນິເຈີ)
    Furniture,
    /// Wood chips (ໄມ້ສັບ)
    WoodChips,
    /// Pulp (ເຍື່ອໄມ້)
    Pulp,
    /// NTFP (ຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້)
    Ntfp,
    /// Charcoal (ຖ່ານ)
    Charcoal,
    /// Other (ອື່ນໆ)
    Other,
}
impl ExportProductType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ExportProductType::Logs => "ໄມ້ທ່ອນ",
            ExportProductType::SawnTimber => "ໄມ້ແປຮູບ",
            ExportProductType::Plywood => "ໄມ້ອັດ",
            ExportProductType::Furniture => "ເຟີນິເຈີ",
            ExportProductType::WoodChips => "ໄມ້ສັບ",
            ExportProductType::Pulp => "ເຍື່ອໄມ້",
            ExportProductType::Ntfp => "ຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້",
            ExportProductType::Charcoal => "ຖ່ານ",
            ExportProductType::Other => "ອື່ນໆ",
        }
    }
    /// Check if log export is restricted
    pub fn is_restricted(&self) -> bool {
        matches!(self, ExportProductType::Logs | ExportProductType::Charcoal)
    }
}
/// Village forest (ປ່າບ້ານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VillageForest {
    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,
    /// Village name in Lao (ຊື່ບ້ານເປັນພາສາລາວ)
    pub village_name_lao: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,
    /// Registration date (ວັນທີຂຶ້ນທະບຽນ)
    pub registration_date: String,
    /// Has management agreement (ມີຂໍ້ຕົກລົງການຄຸ້ມຄອງ)
    pub has_management_agreement: bool,
    /// Agreement expiry date (ວັນທີໝົດອາຍຸຂໍ້ຕົກລົງ)
    pub agreement_expiry: Option<String>,
    /// Number of households (ຈຳນວນຄົວເຮືອນ)
    pub household_count: u32,
    /// Key species present (ຊະນິດພັນຫຼັກທີ່ມີຢູ່)
    pub key_species: Vec<TreeSpecies>,
    /// Traditional use practices (ການນຳໃຊ້ແບບດັ້ງເດີມ)
    pub traditional_uses: Vec<String>,
    /// Has community enterprise (ມີວິສາຫະກິດຊຸມຊົນ)
    pub has_community_enterprise: bool,
}
/// Violation status (ສະຖານະການລະເມີດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationStatus {
    /// Under investigation (ກຳລັງສືບສວນ)
    UnderInvestigation,
    /// Pending prosecution (ລໍຖ້າການດຳເນີນຄະດີ)
    PendingProsecution,
    /// Fine imposed (ປັບໄໝແລ້ວ)
    FineImposed,
    /// Fine paid (ຈ່າຍຄ່າປັບໄໝແລ້ວ)
    FinePaid,
    /// Criminal prosecution (ດຳເນີນຄະດີອາຍາ)
    CriminalProsecution,
    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,
    /// Appeal pending (ລໍຖ້າອຸທອນ)
    AppealPending,
}
impl ViolationStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ViolationStatus::UnderInvestigation => "ກຳລັງສືບສວນ",
            ViolationStatus::PendingProsecution => "ລໍຖ້າການດຳເນີນຄະດີ",
            ViolationStatus::FineImposed => "ປັບໄໝແລ້ວ",
            ViolationStatus::FinePaid => "ຈ່າຍຄ່າປັບໄໝແລ້ວ",
            ViolationStatus::CriminalProsecution => "ດຳເນີນຄະດີອາຍາ",
            ViolationStatus::Resolved => "ແກ້ໄຂແລ້ວ",
            ViolationStatus::AppealPending => "ລໍຖ້າອຸທອນ",
        }
    }
}
/// NTFP Permit (ໃບອະນຸຍາດ NTFP)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NtfpPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Holder name (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,
    /// Holder name in Lao (ຊື່ຜູ້ຖືໃບອະນຸຍາດເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,
    /// NTFP type (ປະເພດ NTFP)
    pub ntfp_type: NtfpType,
    /// Collection area province (ແຂວງເຂດເກັບກ່ຽວ)
    pub province: String,
    /// Collection area district (ເມືອງເຂດເກັບກ່ຽວ)
    pub district: String,
    /// Collection area village (ບ້ານເຂດເກັບກ່ຽວ)
    pub village: Option<String>,
    /// Quantity allowed (ປະລິມານທີ່ອະນຸຍາດ)
    pub quantity_allowed: f64,
    /// Quantity unit (ໜ່ວຍ)
    pub quantity_unit: String,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
    /// Is for commercial use (ເພື່ອການຄ້າ)
    pub commercial_use: bool,
    /// Fee paid in LAK (ຄ່າທຳນຽມທີ່ຈ່າຍແລ້ວ)
    pub fee_paid_lak: Option<u64>,
}
/// Forest classification type (ປະເພດການຈັດແບ່ງປ່າໄມ້)
///
/// Articles 10-15: Forest categories under Lao law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ForestClassification {
    /// Protection Forest (ປ່າປ້ອງກັນ) - Article 11
    /// Watershed protection, erosion control, national security
    Protection,
    /// Conservation Forest (ປ່າສະຫງວນ) - Article 12
    /// Biodiversity conservation, wildlife habitat, protected areas
    Conservation,
    /// Production Forest (ປ່າຜະລິດ) - Article 13
    /// Sustainable timber harvesting, commercial utilization
    Production,
    /// Rehabilitation Forest (ປ່າຟື້ນຟູ) - Article 14
    /// Degraded forest restoration, reforestation projects
    Rehabilitation,
    /// Village Forest (ປ່າບ້ານ) - Article 15
    /// Community management, traditional use, local benefits
    Village,
}
impl ForestClassification {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ForestClassification::Protection => "ປ່າປ້ອງກັນ",
            ForestClassification::Conservation => "ປ່າສະຫງວນ",
            ForestClassification::Production => "ປ່າຜະລິດ",
            ForestClassification::Rehabilitation => "ປ່າຟື້ນຟູ",
            ForestClassification::Village => "ປ່າບ້ານ",
        }
    }
    /// Get the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            ForestClassification::Protection => "Protection Forest",
            ForestClassification::Conservation => "Conservation Forest",
            ForestClassification::Production => "Production Forest",
            ForestClassification::Rehabilitation => "Rehabilitation Forest",
            ForestClassification::Village => "Village Forest",
        }
    }
    /// Check if commercial harvesting is allowed
    pub fn allows_commercial_harvesting(&self) -> bool {
        matches!(self, ForestClassification::Production)
    }
    /// Check if limited harvesting is allowed
    pub fn allows_limited_harvesting(&self) -> bool {
        matches!(
            self,
            ForestClassification::Production | ForestClassification::Village
        )
    }
    /// Get the article number in Forestry Law 2019
    pub fn article_number(&self) -> u32 {
        match self {
            ForestClassification::Protection => 11,
            ForestClassification::Conservation => 12,
            ForestClassification::Production => 13,
            ForestClassification::Rehabilitation => 14,
            ForestClassification::Village => 15,
        }
    }
}
/// Builder for TimberHarvestingPermit
#[derive(Debug, Default)]
pub struct TimberHarvestingPermitBuilder {
    pub(super) permit: TimberHarvestingPermit,
}
impl TimberHarvestingPermitBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set permit number
    pub fn permit_number(mut self, number: impl Into<String>) -> Self {
        self.permit.permit_number = number.into();
        self
    }
    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name = name.into();
        self
    }
    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.permit.holder_name_lao = Some(name.into());
        self
    }
    /// Set forest type
    pub fn forest_type(mut self, forest_type: ForestClassification) -> Self {
        self.permit.forest_type = forest_type;
        self
    }
    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.permit.province = province.into();
        self
    }
    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.permit.district = district.into();
        self
    }
    /// Set village
    pub fn village(mut self, village: impl Into<String>) -> Self {
        self.permit.village = Some(village.into());
        self
    }
    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = species;
        self.permit.minimum_diameter_cm = species.minimum_diameter_cm();
        self
    }
    /// Set volume in cubic meters
    pub fn volume_cubic_meters(mut self, volume: f64) -> Self {
        self.permit.volume_cubic_meters = volume;
        self
    }
    /// Set tree count
    pub fn tree_count(mut self, count: u32) -> Self {
        self.permit.tree_count = Some(count);
        self
    }
    /// Set harvesting month
    pub fn harvesting_month(mut self, month: u8) -> Self {
        self.permit.harvesting_month = month;
        self
    }
    /// Set harvesting year
    pub fn harvesting_year(mut self, year: u32) -> Self {
        self.permit.harvesting_year = year;
        self
    }
    /// Set minimum diameter
    pub fn minimum_diameter_cm(mut self, diameter: u32) -> Self {
        self.permit.minimum_diameter_cm = diameter;
        self
    }
    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.permit.issue_date = date.into();
        self
    }
    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.permit.expiry_date = date.into();
        self
    }
    /// Set issuing authority
    pub fn issuing_authority(mut self, authority: impl Into<String>) -> Self {
        self.permit.issuing_authority = authority.into();
        self
    }
    /// Set AAC allocation
    pub fn aac_allocation(mut self, allocation: f64) -> Self {
        self.permit.aac_allocation = Some(allocation);
        self
    }
    /// Set quota reference
    pub fn quota_reference(mut self, reference: impl Into<String>) -> Self {
        self.permit.quota_reference = Some(reference.into());
        self
    }
    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }
    /// Set reforestation required
    pub fn reforestation_required(mut self, required: bool) -> Self {
        self.permit.reforestation_required = required;
        self
    }
    /// Set reforestation area
    pub fn reforestation_area_hectares(mut self, area: f64) -> Self {
        self.permit.reforestation_area_hectares = Some(area);
        self
    }
    /// Build the permit
    pub fn build(self) -> TimberHarvestingPermit {
        self.permit
    }
}
/// Village forest management agreement (ຂໍ້ຕົກລົງການຄຸ້ມຄອງປ່າບ້ານ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VillageForestAgreement {
    /// Agreement number (ເລກຂໍ້ຕົກລົງ)
    pub agreement_number: String,
    /// Village name (ຊື່ບ້ານ)
    pub village_name: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,
    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,
    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,
    /// Permitted activities (ກິດຈະກຳທີ່ໄດ້ຮັບອະນຸຍາດ)
    pub permitted_activities: Vec<String>,
    /// Prohibited activities (ກິດຈະກຳທີ່ຫ້າມ)
    pub prohibited_activities: Vec<String>,
    /// Benefit sharing arrangement (ການແບ່ງປັນຜົນປະໂຫຍດ)
    pub benefit_sharing: Option<BenefitSharingArrangement>,
    /// Has management plan (ມີແຜນການຄຸ້ມຄອງ)
    pub has_management_plan: bool,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
}
/// Timber harvesting permit (ໃບອະນຸຍາດຕັດໄມ້)
///
/// Article 32: Timber harvesting permit requirements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimberHarvestingPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Holder name in English (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,
    /// Holder name in Lao (ຊື່ຜູ້ຖືໃບອະນຸຍາດເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,
    /// Forest classification (ປະເພດປ່າ)
    pub forest_type: ForestClassification,
    /// Province (ແຂວງ)
    pub province: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Village (ບ້ານ)
    pub village: Option<String>,
    /// Tree species (ຊະນິດໄມ້)
    pub species: TreeSpecies,
    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,
    /// Number of trees (ຈຳນວນຕົ້ນໄມ້)
    pub tree_count: Option<u32>,
    /// Harvesting month (ເດືອນຕັດໄມ້)
    pub harvesting_month: u8,
    /// Harvesting year (ປີຕັດໄມ້)
    pub harvesting_year: u32,
    /// Minimum diameter in cm (ເສັ້ນຜ່ານສູນກາງຂັ້ນຕ່ຳ)
    pub minimum_diameter_cm: u32,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Issuing authority (ອົງການອອກໃບອະນຸຍາດ)
    pub issuing_authority: String,
    /// Annual Allowable Cut allocation (ໂກຕ້າ AAC)
    pub aac_allocation: Option<f64>,
    /// Quota reference number (ເລກອ້າງອິງໂກຕ້າ)
    pub quota_reference: Option<String>,
    /// Permit status (ສະຖານະໃບອະນຸຍາດ)
    pub status: PermitStatus,
    /// Reforestation requirement (ຂໍ້ກຳນົດການປູກປ່າ)
    pub reforestation_required: bool,
    /// Reforestation area in hectares (ເນື້ອທີ່ປູກປ່າ)
    pub reforestation_area_hectares: Option<f64>,
}
/// Chain of custody entry (ບັນທຶກຕ່ອງໂສ້ການຄຸ້ມຄອງ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChainOfCustodyEntry {
    /// Date (ວັນທີ)
    pub date: String,
    /// From location (ຈາກສະຖານທີ່)
    pub from_location: String,
    /// To location (ໄປສະຖານທີ່)
    pub to_location: String,
    /// Transport permit number (ເລກໃບອະນຸຍາດຂົນສົ່ງ)
    pub transport_permit: Option<String>,
    /// Handler name (ຊື່ຜູ້ຮັບຜິດຊອບ)
    pub handler_name: String,
    /// Remarks (ໝາຍເຫດ)
    pub remarks: Option<String>,
}
/// Benefit sharing arrangement (ການຈັດສັນການແບ່ງປັນຜົນປະໂຫຍດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BenefitSharingArrangement {
    /// Village share percentage (ສ່ວນແບ່ງບ້ານ %)
    pub village_share_percent: f64,
    /// District share percentage (ສ່ວນແບ່ງເມືອງ %)
    pub district_share_percent: f64,
    /// National share percentage (ສ່ວນແບ່ງລັດ %)
    pub national_share_percent: f64,
    /// Agreement date (ວັນທີຕົກລົງ)
    pub agreement_date: String,
    /// Agreement validity years (ໄລຍະຂໍ້ຕົກລົງ)
    pub validity_years: u32,
}
/// Penalty assessment (ການປະເມີນໂທດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PenaltyAssessment {
    /// Fine amount in LAK (ຈຳນວນຄ່າປັບໄໝ)
    pub fine_amount_lak: u64,
    /// Fine multiplier used (ຕົວຄູນຄ່າປັບໄໝທີ່ໃຊ້)
    pub fine_multiplier: f64,
    /// Equipment confiscation (ການຍຶດອຸປະກອນ)
    pub equipment_confiscation: bool,
    /// Confiscated equipment description (ລາຍລະອຽດອຸປະກອນທີ່ຖືກຍຶດ)
    pub confiscated_equipment: Vec<String>,
    /// Imprisonment months (ເດືອນຈຳຄຸກ)
    pub imprisonment_months: Option<u32>,
    /// Reforestation obligation (ພັນທະປູກປ່າຄືນ)
    pub reforestation_obligation_hectares: Option<f64>,
    /// License suspension (ການໂຈະໃບອະນຸຍາດ)
    pub license_suspension: bool,
    /// License revocation (ການຖອນໃບອະນຸຍາດ)
    pub license_revocation: bool,
    /// Assessment date (ວັນທີປະເມີນ)
    pub assessment_date: String,
    /// Payment deadline (ກຳນົດຈ່າຍ)
    pub payment_deadline: Option<String>,
}
/// Transport permit (ໃບອະນຸຍາດຂົນສົ່ງໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransportPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,
    /// Holder name (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,
    /// Origin province (ແຂວງຕົ້ນທາງ)
    pub origin_province: String,
    /// Origin district (ເມືອງຕົ້ນທາງ)
    pub origin_district: String,
    /// Destination province (ແຂວງປາຍທາງ)
    pub destination_province: String,
    /// Destination district (ເມືອງປາຍທາງ)
    pub destination_district: String,
    /// Destination facility (ສະຖານທີ່ປາຍທາງ)
    pub destination_facility: Option<String>,
    /// Species (ຊະນິດ)
    pub species: TreeSpecies,
    /// Volume in cubic meters (ປະລິມານເປັນແມັດກ້ອນ)
    pub volume_cubic_meters: f64,
    /// Number of logs (ຈຳນວນທ່ອນໄມ້)
    pub log_count: u32,
    /// Vehicle registration (ທະບຽນລົດ)
    pub vehicle_registration: String,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Specified route (ເສັ້ນທາງທີ່ກຳນົດ)
    pub specified_route: Option<String>,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
    /// Harvest permit reference (ອ້າງອິງໃບອະນຸຍາດຕັດ)
    pub harvest_permit_reference: String,
}
/// Forest concession (ສຳປະທານປ່າໄມ້)
///
/// Articles 61-75: Forest concession requirements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestConcession {
    /// Concession number (ເລກສຳປະທານ)
    pub concession_number: String,
    /// Holder name (ຊື່ຜູ້ຖືສຳປະທານ)
    pub holder_name: String,
    /// Holder name in Lao (ຊື່ຜູ້ຖືສຳປະທານເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,
    /// Concession type (ປະເພດສຳປະທານ)
    pub concession_type: ConcessionType,
    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,
    /// Term in years (ໄລຍະເປັນປີ)
    pub term_years: u32,
    /// Province (ແຂວງ)
    pub province: String,
    /// Districts covered (ເມືອງທີ່ກວມເອົາ)
    pub districts: Vec<String>,
    /// Start date (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,
    /// End date (ວັນທີສິ້ນສຸດ)
    pub end_date: String,
    /// Performance bond amount in LAK (ເງິນຄ້ຳປະກັນ)
    pub performance_bond_lak: u64,
    /// Project value in LAK (ມູນຄ່າໂຄງການ)
    pub project_value_lak: Option<u64>,
    /// Has Environmental Impact Assessment (ມີ EIA)
    pub has_eia: bool,
    /// Has management plan (ມີແຜນຄຸ້ມຄອງ)
    pub has_management_plan: bool,
    /// Status (ສະຖານະ)
    pub status: ConcessionStatus,
    /// Primary species (ຊະນິດໄມ້ຫຼັກ)
    pub primary_species: Vec<TreeSpecies>,
    /// Annual production quota (ໂກຕ້າການຜະລິດປະຈຳປີ)
    pub annual_production_quota_m3: Option<f64>,
    /// Reforestation commitment hectares (ຄຳໝັ້ນສັນຍາປູກປ່າ)
    pub reforestation_commitment_hectares: Option<f64>,
    /// Community benefit agreements (ຂໍ້ຕົກລົງຜົນປະໂຫຍດຊຸມຊົນ)
    pub community_agreements: Vec<String>,
}
