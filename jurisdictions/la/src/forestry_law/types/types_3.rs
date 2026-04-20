//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::functions::{
    ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX, ILLEGAL_LOGGING_FINE_MULTIPLIER_MIN,
    MANAGEMENT_CONCESSION_BOND_PERCENT, MAX_MANAGEMENT_CONCESSION_HECTARES,
    MAX_MANAGEMENT_CONCESSION_YEARS, MAX_PLANTATION_CONCESSION_HECTARES,
    MAX_PLANTATION_CONCESSION_YEARS, PLANTATION_CONCESSION_BOND_PERCENT,
    WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX, WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MIN,
};
use super::permit_types::{
    ConcessionStatus, ForestConcession, NtfpPermit, PenaltyAssessment, SawmillLicense,
    TransportPermit, TreeSpecies, VillageForest, ViolationStatus,
};

/// Non-Timber Forest Product type (ປະເພດຜະລິດຕະພັນປ່າໄມ້ທີ່ບໍ່ແມ່ນໄມ້)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NtfpType {
    /// Bamboo shoots (ໜໍ່ໄມ້)
    BambooShoots,
    /// Rattan (ຫວາຍ)
    Rattan,
    /// Honey (ເຜິ້ງ)
    Honey,
    /// Resin (ຢາງ)
    Resin,
    /// Cardamom (ໝາກແໜ່ງ)
    Cardamom,
    /// Mushrooms (ເຫັດ)
    Mushrooms,
    /// Medicinal plants (ພືດສະໝຸນໄພ)
    MedicinalPlants,
    /// Tree bark (ເປືອກໄມ້)
    TreeBark,
    /// Forest fruits (ໝາກໄມ້ປ່າ)
    ForestFruits,
    /// Insects (ແມງໄມ້)
    Insects,
    /// Other (ອື່ນໆ)
    Other,
}
impl NtfpType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            NtfpType::BambooShoots => "ໜໍ່ໄມ້",
            NtfpType::Rattan => "ຫວາຍ",
            NtfpType::Honey => "ເຜິ້ງ",
            NtfpType::Resin => "ຢາງ",
            NtfpType::Cardamom => "ໝາກແໜ່ງ",
            NtfpType::Mushrooms => "ເຫັດ",
            NtfpType::MedicinalPlants => "ພືດສະໝຸນໄພ",
            NtfpType::TreeBark => "ເປືອກໄມ້",
            NtfpType::ForestFruits => "ໝາກໄມ້ປ່າ",
            NtfpType::Insects => "ແມງໄມ້",
            NtfpType::Other => "ອື່ນໆ",
        }
    }
    /// Check if requires special permit
    pub fn requires_special_permit(&self) -> bool {
        matches!(self, NtfpType::MedicinalPlants | NtfpType::Resin)
    }
}
/// Builder for NtfpPermit
#[derive(Debug, Default)]
pub struct NtfpPermitBuilder {
    permit: NtfpPermit,
}
impl NtfpPermitBuilder {
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
    /// Set NTFP type
    pub fn ntfp_type(mut self, ntfp_type: NtfpType) -> Self {
        self.permit.ntfp_type = ntfp_type;
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
    /// Set quantity allowed
    pub fn quantity_allowed(mut self, quantity: f64, unit: impl Into<String>) -> Self {
        self.permit.quantity_allowed = quantity;
        self.permit.quantity_unit = unit.into();
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
    /// Set commercial use
    pub fn commercial_use(mut self, commercial: bool) -> Self {
        self.permit.commercial_use = commercial;
        self
    }
    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }
    /// Build the permit
    pub fn build(self) -> NtfpPermit {
        self.permit
    }
}
/// Builder for TransportPermit
#[derive(Debug, Default)]
pub struct TransportPermitBuilder {
    permit: TransportPermit,
}
impl TransportPermitBuilder {
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
    /// Set origin
    pub fn origin(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.permit.origin_province = province.into();
        self.permit.origin_district = district.into();
        self
    }
    /// Set destination
    pub fn destination(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.permit.destination_province = province.into();
        self.permit.destination_district = district.into();
        self
    }
    /// Set species
    pub fn species(mut self, species: TreeSpecies) -> Self {
        self.permit.species = species;
        self
    }
    /// Set volume
    pub fn volume_cubic_meters(mut self, volume: f64) -> Self {
        self.permit.volume_cubic_meters = volume;
        self
    }
    /// Set log count
    pub fn log_count(mut self, count: u32) -> Self {
        self.permit.log_count = count;
        self
    }
    /// Set vehicle registration
    pub fn vehicle_registration(mut self, registration: impl Into<String>) -> Self {
        self.permit.vehicle_registration = registration.into();
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
    /// Set harvest permit reference
    pub fn harvest_permit_reference(mut self, reference: impl Into<String>) -> Self {
        self.permit.harvest_permit_reference = reference.into();
        self
    }
    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.permit.status = status;
        self
    }
    /// Build the permit
    pub fn build(self) -> TransportPermit {
        self.permit
    }
}
/// Permit status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PermitStatus {
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Revoked (ຖືກຖອນ)
    Revoked,
    /// Completed (ສຳເລັດ)
    Completed,
}
impl PermitStatus {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            PermitStatus::Pending => "ລໍຖ້າ",
            PermitStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            PermitStatus::Expired => "ໝົດອາຍຸ",
            PermitStatus::Suspended => "ໂຈະ",
            PermitStatus::Revoked => "ຖືກຖອນ",
            PermitStatus::Completed => "ສຳເລັດ",
        }
    }
}
/// Builder for VillageForest
#[derive(Debug, Default)]
pub struct VillageForestBuilder {
    forest: VillageForest,
}
impl VillageForestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set village name
    pub fn village_name(mut self, name: impl Into<String>) -> Self {
        self.forest.village_name = name.into();
        self
    }
    /// Set village name in Lao
    pub fn village_name_lao(mut self, name: impl Into<String>) -> Self {
        self.forest.village_name_lao = name.into();
        self
    }
    /// Set district
    pub fn district(mut self, district: impl Into<String>) -> Self {
        self.forest.district = district.into();
        self
    }
    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.forest.province = province.into();
        self
    }
    /// Set area
    pub fn area_hectares(mut self, area: f64) -> Self {
        self.forest.area_hectares = area;
        self
    }
    /// Set registration date
    pub fn registration_date(mut self, date: impl Into<String>) -> Self {
        self.forest.registration_date = date.into();
        self
    }
    /// Set has management agreement
    pub fn has_management_agreement(mut self, has: bool) -> Self {
        self.forest.has_management_agreement = has;
        self
    }
    /// Set household count
    pub fn household_count(mut self, count: u32) -> Self {
        self.forest.household_count = count;
        self
    }
    /// Add key species
    pub fn add_species(mut self, species: TreeSpecies) -> Self {
        self.forest.key_species.push(species);
        self
    }
    /// Add traditional use
    pub fn add_traditional_use(mut self, use_type: impl Into<String>) -> Self {
        self.forest.traditional_uses.push(use_type.into());
        self
    }
    /// Build the village forest
    pub fn build(self) -> VillageForest {
        self.forest
    }
}
/// Forestry violation type (ປະເພດການລະເມີດກົດໝາຍປ່າໄມ້)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationType {
    /// Illegal logging (ການຕັດໄມ້ຜິດກົດໝາຍ) - Article 107
    IllegalLogging,
    /// Wildlife trafficking (ການຄ້າສັດປ່າຜິດກົດໝາຍ) - Article 108
    WildlifeTrafficking,
    /// Forest fire (ໄຟໄໝ້ປ່າ) - Article 109
    ForestFire,
    /// Unauthorized land clearing (ການບຸກເບີກທີ່ດິນໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ)
    UnauthorizedLandClearing,
    /// Permit violation (ການລະເມີດໃບອະນຸຍາດ)
    PermitViolation,
    /// Encroachment on protected area (ການບຸກລຸກເຂດປ່າປ້ອງກັນ)
    ProtectedAreaEncroachment,
    /// Harvesting prohibited species (ການຕັດຊະນິດພັນທີ່ຫ້າມ)
    ProhibitedSpeciesHarvesting,
    /// Transport without permit (ການຂົນສົ່ງໂດຍບໍ່ມີໃບອະນຸຍາດ)
    TransportWithoutPermit,
    /// Failure to reforest (ການບໍ່ປູກປ່າຄືນ)
    FailureToReforest,
    /// CITES violation (ການລະເມີດ CITES)
    CitesViolation,
    /// Other (ອື່ນໆ)
    Other,
}
impl ViolationType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ViolationType::IllegalLogging => "ການຕັດໄມ້ຜິດກົດໝາຍ",
            ViolationType::WildlifeTrafficking => "ການຄ້າສັດປ່າຜິດກົດໝາຍ",
            ViolationType::ForestFire => "ໄຟໄໝ້ປ່າ",
            ViolationType::UnauthorizedLandClearing => "ການບຸກເບີກທີ່ດິນໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ",
            ViolationType::PermitViolation => "ການລະເມີດໃບອະນຸຍາດ",
            ViolationType::ProtectedAreaEncroachment => "ການບຸກລຸກເຂດປ່າປ້ອງກັນ",
            ViolationType::ProhibitedSpeciesHarvesting => "ການຕັດຊະນິດພັນທີ່ຫ້າມ",
            ViolationType::TransportWithoutPermit => "ການຂົນສົ່ງໂດຍບໍ່ມີໃບອະນຸຍາດ",
            ViolationType::FailureToReforest => "ການບໍ່ປູກປ່າຄືນ",
            ViolationType::CitesViolation => "ການລະເມີດ CITES",
            ViolationType::Other => "ອື່ນໆ",
        }
    }
    /// Get article number
    pub fn article_number(&self) -> Option<u32> {
        match self {
            ViolationType::IllegalLogging => Some(107),
            ViolationType::WildlifeTrafficking => Some(108),
            ViolationType::ForestFire => Some(109),
            ViolationType::FailureToReforest => Some(110),
            _ => None,
        }
    }
    /// Get fine multiplier range
    pub fn fine_multiplier_range(&self) -> (f64, f64) {
        match self {
            ViolationType::IllegalLogging => (
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MIN,
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX,
            ),
            ViolationType::WildlifeTrafficking => (
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MIN,
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX,
            ),
            ViolationType::ProhibitedSpeciesHarvesting => (
                ILLEGAL_LOGGING_FINE_MULTIPLIER_MAX,
                WILDLIFE_TRAFFICKING_FINE_MULTIPLIER_MAX,
            ),
            _ => (1.0, 5.0),
        }
    }
}
/// Builder for SawmillLicense
#[derive(Debug, Default)]
pub struct SawmillLicenseBuilder {
    license: SawmillLicense,
}
impl SawmillLicenseBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set license number
    pub fn license_number(mut self, number: impl Into<String>) -> Self {
        self.license.license_number = number.into();
        self
    }
    /// Set facility name
    pub fn facility_name(mut self, name: impl Into<String>) -> Self {
        self.license.facility_name = name.into();
        self
    }
    /// Set owner name
    pub fn owner_name(mut self, name: impl Into<String>) -> Self {
        self.license.owner_name = name.into();
        self
    }
    /// Set location
    pub fn location(mut self, province: impl Into<String>, district: impl Into<String>) -> Self {
        self.license.province = province.into();
        self.license.district = district.into();
        self
    }
    /// Set annual capacity
    pub fn annual_capacity_cubic_meters(mut self, capacity: f64) -> Self {
        self.license.annual_capacity_cubic_meters = capacity;
        self
    }
    /// Set issue date
    pub fn issue_date(mut self, date: impl Into<String>) -> Self {
        self.license.issue_date = date.into();
        self
    }
    /// Set expiry date
    pub fn expiry_date(mut self, date: impl Into<String>) -> Self {
        self.license.expiry_date = date.into();
        self
    }
    /// Set environmental compliance
    pub fn environmental_compliance(mut self, compliant: bool) -> Self {
        self.license.environmental_compliance = compliant;
        self
    }
    /// Set has log tracking
    pub fn has_log_tracking(mut self, has: bool) -> Self {
        self.license.has_log_tracking = has;
        self
    }
    /// Set status
    pub fn status(mut self, status: PermitStatus) -> Self {
        self.license.status = status;
        self
    }
    /// Add permitted species
    pub fn add_permitted_species(mut self, species: TreeSpecies) -> Self {
        self.license.permitted_species.push(species);
        self
    }
    /// Build the license
    pub fn build(self) -> SawmillLicense {
        self.license
    }
}
/// Species protection category (ປະເພດການປົກປ້ອງຊະນິດພັນ)
///
/// Articles 77-79: Species classification for protection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectionCategory {
    /// Category I: Strictly protected - no harvest (ປະເພດ I: ຫ້າມຕັດເດັດຂາດ)
    CategoryI,
    /// Category II: Managed species - quota required (ປະເພດ II: ຕ້ອງມີໂກຕ້າ)
    CategoryII,
    /// Category III: Common species - standard permit (ປະເພດ III: ໃບອະນຸຍາດທົ່ວໄປ)
    CategoryIII,
}
impl ProtectionCategory {
    /// Get the Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProtectionCategory::CategoryI => "ປະເພດ I - ຫ້າມຕັດເດັດຂາດ",
            ProtectionCategory::CategoryII => "ປະເພດ II - ຕ້ອງມີໂກຕ້າ",
            ProtectionCategory::CategoryIII => "ປະເພດ III - ໃບອະນຸຍາດທົ່ວໄປ",
        }
    }
    /// Check if harvesting is allowed
    pub fn allows_harvesting(&self) -> bool {
        !matches!(self, ProtectionCategory::CategoryI)
    }
    /// Check if quota is required
    pub fn requires_quota(&self) -> bool {
        matches!(self, ProtectionCategory::CategoryII)
    }
}
/// Builder for ForestConcession
#[derive(Debug, Default)]
pub struct ForestConcessionBuilder {
    concession: ForestConcession,
}
impl ForestConcessionBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Set concession number
    pub fn concession_number(mut self, number: impl Into<String>) -> Self {
        self.concession.concession_number = number.into();
        self
    }
    /// Set holder name
    pub fn holder_name(mut self, name: impl Into<String>) -> Self {
        self.concession.holder_name = name.into();
        self
    }
    /// Set holder name in Lao
    pub fn holder_name_lao(mut self, name: impl Into<String>) -> Self {
        self.concession.holder_name_lao = Some(name.into());
        self
    }
    /// Set concession type
    pub fn concession_type(mut self, concession_type: ConcessionType) -> Self {
        self.concession.concession_type = concession_type;
        self
    }
    /// Set area in hectares
    pub fn area_hectares(mut self, area: f64) -> Self {
        self.concession.area_hectares = area;
        self
    }
    /// Set term in years
    pub fn term_years(mut self, years: u32) -> Self {
        self.concession.term_years = years;
        self
    }
    /// Set province
    pub fn province(mut self, province: impl Into<String>) -> Self {
        self.concession.province = province.into();
        self
    }
    /// Add district
    pub fn add_district(mut self, district: impl Into<String>) -> Self {
        self.concession.districts.push(district.into());
        self
    }
    /// Set start date
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.concession.start_date = date.into();
        self
    }
    /// Set end date
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.concession.end_date = date.into();
        self
    }
    /// Set performance bond
    pub fn performance_bond_lak(mut self, amount: u64) -> Self {
        self.concession.performance_bond_lak = amount;
        self
    }
    /// Set project value
    pub fn project_value_lak(mut self, value: u64) -> Self {
        self.concession.project_value_lak = Some(value);
        self
    }
    /// Set has EIA
    pub fn has_eia(mut self, has: bool) -> Self {
        self.concession.has_eia = has;
        self
    }
    /// Set has management plan
    pub fn has_management_plan(mut self, has: bool) -> Self {
        self.concession.has_management_plan = has;
        self
    }
    /// Set status
    pub fn status(mut self, status: ConcessionStatus) -> Self {
        self.concession.status = status;
        self
    }
    /// Add primary species
    pub fn add_species(mut self, species: TreeSpecies) -> Self {
        self.concession.primary_species.push(species);
        self
    }
    /// Set annual production quota
    pub fn annual_production_quota_m3(mut self, quota: f64) -> Self {
        self.concession.annual_production_quota_m3 = Some(quota);
        self
    }
    /// Set reforestation commitment
    pub fn reforestation_commitment_hectares(mut self, hectares: f64) -> Self {
        self.concession.reforestation_commitment_hectares = Some(hectares);
        self
    }
    /// Build the concession
    pub fn build(self) -> ForestConcession {
        self.concession
    }
}
/// Concession type (ປະເພດສຳປະທານ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcessionType {
    /// Forest management concession (ສຳປະທານຄຸ້ມຄອງປ່າໄມ້) - Article 62
    Management,
    /// Forest plantation concession (ສຳປະທານປູກປ່າ) - Article 63
    Plantation,
}
impl ConcessionType {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConcessionType::Management => "ສຳປະທານຄຸ້ມຄອງປ່າໄມ້",
            ConcessionType::Plantation => "ສຳປະທານປູກປ່າ",
        }
    }
    /// Get maximum term in years
    pub fn max_term_years(&self) -> u32 {
        match self {
            ConcessionType::Management => MAX_MANAGEMENT_CONCESSION_YEARS,
            ConcessionType::Plantation => MAX_PLANTATION_CONCESSION_YEARS,
        }
    }
    /// Get maximum area in hectares
    pub fn max_area_hectares(&self) -> f64 {
        match self {
            ConcessionType::Management => MAX_MANAGEMENT_CONCESSION_HECTARES,
            ConcessionType::Plantation => MAX_PLANTATION_CONCESSION_HECTARES,
        }
    }
    /// Get required bond percentage
    pub fn bond_percentage(&self) -> f64 {
        match self {
            ConcessionType::Management => MANAGEMENT_CONCESSION_BOND_PERCENT,
            ConcessionType::Plantation => PLANTATION_CONCESSION_BOND_PERCENT,
        }
    }
}
/// Forestry violation record (ບັນທຶກການລະເມີດກົດໝາຍປ່າໄມ້)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForestryViolation {
    /// Case number (ເລກຄະດີ)
    pub case_number: String,
    /// Violation type (ປະເພດການລະເມີດ)
    pub violation_type: ViolationType,
    /// Violator name (ຊື່ຜູ້ລະເມີດ)
    pub violator_name: String,
    /// Violator name in Lao (ຊື່ຜູ້ລະເມີດເປັນພາສາລາວ)
    pub violator_name_lao: Option<String>,
    /// Location province (ແຂວງ)
    pub province: String,
    /// Location district (ເມືອງ)
    pub district: String,
    /// Detection date (ວັນທີກວດພົບ)
    pub detection_date: String,
    /// Species involved (ຊະນິດທີ່ກ່ຽວຂ້ອງ)
    pub species_involved: Option<TreeSpecies>,
    /// Volume involved in m3 (ປະລິມານທີ່ກ່ຽວຂ້ອງ)
    pub volume_cubic_meters: Option<f64>,
    /// Estimated value in LAK (ມູນຄ່າຄາດຄະເນ)
    pub estimated_value_lak: Option<u64>,
    /// Area affected in hectares (ເນື້ອທີ່ທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub area_affected_hectares: Option<f64>,
    /// Penalty assessment (ການປະເມີນໂທດ)
    pub penalty: Option<PenaltyAssessment>,
    /// Status (ສະຖານະ)
    pub status: ViolationStatus,
    /// Confiscated items (ຂອງທີ່ຖືກຍຶດ)
    pub confiscated_items: Vec<String>,
    /// Reforestation required (ຕ້ອງປູກປ່າຄືນ)
    pub reforestation_required: bool,
    /// Reforestation area (ເນື້ອທີ່ປູກປ່າຄືນ)
    pub reforestation_area_hectares: Option<f64>,
}
/// Processing facility license (ໃບອະນຸຍາດໂຮງງານແປຮູບ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessingFacilityLicense {
    /// License number (ເລກໃບອະນຸຍາດ)
    pub license_number: String,
    /// Facility name (ຊື່ໂຮງງານ)
    pub facility_name: String,
    /// Province (ແຂວງ)
    pub province: String,
    /// District (ເມືອງ)
    pub district: String,
    /// Processing type (ປະເພດການແປຮູບ)
    pub processing_type: String,
    /// Annual capacity (ກຳລັງການຜະລິດປະຈຳປີ)
    pub annual_capacity: f64,
    /// Capacity unit (ໜ່ວຍ)
    pub capacity_unit: String,
    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,
    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,
    /// Status (ສະຖານະ)
    pub status: PermitStatus,
    /// Raw material tracking (ການຕິດຕາມວັດຖຸດິບ)
    pub has_raw_material_tracking: bool,
}
