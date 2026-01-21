//! Environmental Law Types (ປະເພດກົດໝາຍສິ່ງແວດລ້ອມ)
//!
//! Type definitions for Lao environmental law based on:
//! - **Environmental Protection Law 2012** (Law No. 29/NA, dated December 18, 2012)
//! - National Assembly of the Lao PDR
//!
//! # Legal References
//! - Environmental Protection Law 2012 (Law No. 29/NA) - ກົດໝາຍວ່າດ້ວຍການປົກປັກຮັກສາສິ່ງແວດລ້ອມ ປີ 2012
//! - EIA Decree (Prime Minister's Decree No. 112/PM) - ດຳລັດວ່າດ້ວຍການປະເມີນຜົນກະທົບຕໍ່ສິ່ງແວດລ້ອມ
//! - Pollution Control Regulations - ລະບຽບການຄວບຄຸມມົນລະພິດ
//! - Protected Areas Law - ກົດໝາຍວ່າດ້ວຍເຂດປ່າປ້ອງກັນ
//!
//! # Bilingual Support
//! All types include both Lao (ລາວ) and English field names where applicable.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Environmental Protection Law 2012 (ກົດໝາຍປົກປັກຮັກສາສິ່ງແວດລ້ອມ ປີ 2012)
// ============================================================================

/// Maximum PM2.5 annual average (Article 30) - μg/m³
/// ມາດຕະຖານ PM2.5 ສະເລ່ຍປີ
pub const MAX_PM25_ANNUAL: f64 = 25.0;

/// Maximum PM10 annual average (Article 30) - μg/m³
/// ມາດຕະຖານ PM10 ສະເລ່ຍປີ
pub const MAX_PM10_ANNUAL: f64 = 50.0;

/// Maximum Biochemical Oxygen Demand discharge (Article 31) - mg/L
/// ມາດຕະຖານ BOD ສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MAX_BOD_DISCHARGE: f64 = 20.0;

/// Maximum Chemical Oxygen Demand discharge (Article 31) - mg/L
/// ມາດຕະຖານ COD ສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MAX_COD_DISCHARGE: f64 = 120.0;

/// Maximum Total Suspended Solids discharge (Article 31) - mg/L
/// ມາດຕະຖານ TSS ສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MAX_TSS_DISCHARGE: f64 = 50.0;

/// Maximum noise level for residential areas during day (Article 32) - dB
/// ລະດັບສຽງສູງສຸດເຂດທີ່ຢູ່ອາໄສ (ກາງວັນ)
pub const MAX_NOISE_RESIDENTIAL_DAY: u8 = 55;

/// Maximum noise level for residential areas during night (Article 32) - dB
/// ລະດັບສຽງສູງສຸດເຂດທີ່ຢູ່ອາໄສ (ກາງຄືນ)
pub const MAX_NOISE_RESIDENTIAL_NIGHT: u8 = 45;

/// Maximum noise level for industrial areas (Article 32) - dB
/// ລະດັບສຽງສູງສຸດເຂດອຸດສາຫະກຳ
pub const MAX_NOISE_INDUSTRIAL: u8 = 75;

/// Maximum noise level for commercial areas during day (Article 32) - dB
/// ລະດັບສຽງສູງສຸດເຂດການຄ້າ (ກາງວັນ)
pub const MAX_NOISE_COMMERCIAL_DAY: u8 = 65;

/// Maximum noise level for commercial areas during night (Article 32) - dB
/// ລະດັບສຽງສູງສຸດເຂດການຄ້າ (ກາງຄືນ)
pub const MAX_NOISE_COMMERCIAL_NIGHT: u8 = 55;

/// Maximum pH level for water discharge (Article 31)
/// ຄ່າ pH ສູງສຸດສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MAX_PH_DISCHARGE: f64 = 9.0;

/// Minimum pH level for water discharge (Article 31)
/// ຄ່າ pH ຕ່ຳສຸດສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MIN_PH_DISCHARGE: f64 = 5.5;

/// Maximum temperature for water discharge (Article 31) - °C
/// ອຸນຫະພູມສູງສຸດສຳລັບການປ່ອຍນ້ຳເສຍ
pub const MAX_TEMPERATURE_DISCHARGE: f64 = 40.0;

/// EIA validity period for Category A projects (years) - Article 22
/// ໄລຍະເວລາຂອງ EIA ສຳລັບໂຄງການປະເພດ ກ (ປີ)
pub const EIA_VALIDITY_YEARS_CATEGORY_A: u32 = 2;

/// EIA validity period for Category B projects (years) - Article 22
/// ໄລຍະເວລາຂອງ EIA ສຳລັບໂຄງການປະເພດ ຂ (ປີ)
pub const EIA_VALIDITY_YEARS_CATEGORY_B: u32 = 3;

/// Environmental permit validity period (years) - Article 25
/// ໄລຍະເວລາຂອງໃບອະນຸຍາດສິ່ງແວດລ້ອມ (ປີ)
pub const ENVIRONMENTAL_PERMIT_VALIDITY_YEARS: u32 = 5;

/// Minimum buffer zone for protected areas (meters) - Article 40
/// ເຂດກັນຊົນຂັ້ນຕ່ຳສຳລັບເຂດປ່າປ້ອງກັນ (ແມັດ)
pub const MIN_BUFFER_ZONE_METERS: u32 = 500;

/// Maximum mining concession near protected area boundary (meters) - Article 41
/// ໄລຍະຫ່າງສູງສຸດຈາກເຂດປ່າປ້ອງກັນສຳລັບສຳປະທານບໍ່ແຮ່ (ແມັດ)
pub const MIN_MINING_DISTANCE_FROM_PROTECTED_AREA: u32 = 1000;

// ============================================================================
// Project Type (ປະເພດໂຄງການ)
// ============================================================================

/// Project type requiring environmental assessment (ປະເພດໂຄງການທີ່ຕ້ອງປະເມີນຜົນກະທົບ)
///
/// Article 18: Projects requiring EIA based on type, scale, and location
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProjectType {
    /// Mining project (ໂຄງການບໍ່ແຮ່)
    /// Articles 18-19: Mining projects require full EIA
    Mining {
        /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
        area_hectares: f64,
        /// Mineral type (ປະເພດແຮ່)
        mineral_type: Option<String>,
    },

    /// Hydropower project (ໂຄງການໄຟຟ້ານ້ຳຕົກ)
    /// Articles 18-19: Hydropower > 15MW requires Category A EIA
    Hydropower {
        /// Installed capacity in megawatts (ກຳລັງຕິດຕັ້ງເປັນເມກາວັດ)
        capacity_mw: f64,
        /// Reservoir area in hectares (ເນື້ອທີ່ອ່າງເກັບນ້ຳເປັນເຮັກຕາ)
        reservoir_hectares: Option<f64>,
    },

    /// Industrial project (ໂຄງການອຸດສາຫະກຳ)
    /// Article 18: Industrial facilities with significant emissions
    Industrial {
        /// Factory type (ປະເພດໂຮງງານ)
        factory_type: String,
        /// Production capacity (ກຳລັງການຜະລິດ)
        capacity_description: Option<String>,
    },

    /// Infrastructure project (ໂຄງການພື້ນຖານໂຄງລ່າງ)
    /// Article 18: Roads, bridges, railways, airports
    Infrastructure {
        /// Project type (ປະເພດໂຄງການ)
        project_type: String,
        /// Length in kilometers if applicable (ຄວາມຍາວເປັນກິໂລແມັດ)
        length_km: Option<f64>,
    },

    /// Agricultural project (ໂຄງການກະສິກຳ)
    /// Article 18: Large-scale plantations, irrigation projects
    Agricultural {
        /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
        area_hectares: f64,
        /// Crop type (ປະເພດພືດ)
        crop_type: Option<String>,
    },

    /// Tourism project (ໂຄງການທ່ອງທ່ຽວ)
    /// Article 18: Hotels, resorts, ecotourism facilities
    Tourism {
        /// Facility type (ປະເພດສິ່ງອຳນວຍຄວາມສະດວກ)
        facility_type: String,
        /// Number of rooms if applicable (ຈຳນວນຫ້ອງ)
        room_count: Option<u32>,
    },

    /// Forest concession project (ໂຄງການສຳປະທານປ່າໄມ້)
    /// Article 18: Logging and forest plantation projects
    ForestConcession {
        /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
        area_hectares: f64,
        /// Forest type (ປະເພດປ່າ)
        forest_type: Option<String>,
    },

    /// Waste management facility (ສິ່ງອຳນວຍຄວາມສະດວກຈັດການສິ່ງເສດເຫຼືອ)
    /// Article 18: Landfills, incinerators, recycling facilities
    WasteManagement {
        /// Facility type (ປະເພດສິ່ງອຳນວຍຄວາມສະດວກ)
        facility_type: String,
        /// Capacity in tons per day (ກຳລັງເປັນໂຕນຕໍ່ມື້)
        capacity_tons_per_day: f64,
    },

    /// Urban development project (ໂຄງການພັດທະນາເມືອງ)
    /// Article 18: Large residential/commercial developments
    UrbanDevelopment {
        /// Development type (ປະເພດການພັດທະນາ)
        development_type: String,
        /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
        area_hectares: f64,
    },

    /// Other project type (ໂຄງການປະເພດອື່ນໆ)
    Other {
        /// Description (ລາຍລະອຽດ)
        description: String,
    },
}

impl ProjectType {
    /// Get the Lao name of the project type (ຮັບຊື່ປະເພດໂຄງການເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProjectType::Mining { .. } => "ໂຄງການບໍ່ແຮ່",
            ProjectType::Hydropower { .. } => "ໂຄງການໄຟຟ້ານ້ຳຕົກ",
            ProjectType::Industrial { .. } => "ໂຄງການອຸດສາຫະກຳ",
            ProjectType::Infrastructure { .. } => "ໂຄງການພື້ນຖານໂຄງລ່າງ",
            ProjectType::Agricultural { .. } => "ໂຄງການກະສິກຳ",
            ProjectType::Tourism { .. } => "ໂຄງການທ່ອງທ່ຽວ",
            ProjectType::ForestConcession { .. } => "ໂຄງການສຳປະທານປ່າໄມ້",
            ProjectType::WasteManagement { .. } => "ສິ່ງອຳນວຍຄວາມສະດວກຈັດການສິ່ງເສດເຫຼືອ",
            ProjectType::UrbanDevelopment { .. } => "ໂຄງການພັດທະນາເມືອງ",
            ProjectType::Other { .. } => "ໂຄງການປະເພດອື່ນໆ",
        }
    }

    /// Determine EIA category based on project type and scale (ກຳນົດປະເພດ EIA)
    ///
    /// Returns:
    /// - Some(EIACategory::A) for large-scale projects requiring full EIA
    /// - Some(EIACategory::B) for medium-scale projects requiring IEE
    /// - None for projects exempt from EIA
    pub fn eia_category(&self) -> Option<EIACategory> {
        match self {
            ProjectType::Mining { area_hectares, .. } => {
                if *area_hectares >= 100.0 {
                    Some(EIACategory::CategoryA)
                } else if *area_hectares >= 10.0 {
                    Some(EIACategory::CategoryB)
                } else {
                    None
                }
            }
            ProjectType::Hydropower { capacity_mw, .. } => {
                if *capacity_mw >= 15.0 {
                    Some(EIACategory::CategoryA)
                } else if *capacity_mw >= 1.0 {
                    Some(EIACategory::CategoryB)
                } else {
                    None
                }
            }
            ProjectType::Industrial { .. } => Some(EIACategory::CategoryB),
            ProjectType::Infrastructure { length_km, .. } => {
                if let Some(km) = length_km {
                    if *km >= 50.0 {
                        Some(EIACategory::CategoryA)
                    } else if *km >= 10.0 {
                        Some(EIACategory::CategoryB)
                    } else {
                        None
                    }
                } else {
                    Some(EIACategory::CategoryB)
                }
            }
            ProjectType::Agricultural { area_hectares, .. } => {
                if *area_hectares >= 500.0 {
                    Some(EIACategory::CategoryA)
                } else if *area_hectares >= 100.0 {
                    Some(EIACategory::CategoryB)
                } else {
                    None
                }
            }
            ProjectType::ForestConcession { area_hectares, .. } => {
                if *area_hectares >= 500.0 {
                    Some(EIACategory::CategoryA)
                } else if *area_hectares >= 100.0 {
                    Some(EIACategory::CategoryB)
                } else {
                    None
                }
            }
            ProjectType::WasteManagement {
                capacity_tons_per_day,
                ..
            } => {
                if *capacity_tons_per_day >= 100.0 {
                    Some(EIACategory::CategoryA)
                } else {
                    Some(EIACategory::CategoryB)
                }
            }
            ProjectType::UrbanDevelopment { area_hectares, .. } => {
                if *area_hectares >= 50.0 {
                    Some(EIACategory::CategoryA)
                } else if *area_hectares >= 10.0 {
                    Some(EIACategory::CategoryB)
                } else {
                    None
                }
            }
            _ => Some(EIACategory::CategoryB),
        }
    }
}

/// EIA category (ປະເພດການປະເມີນຜົນກະທົບ)
///
/// Article 18: Classification of projects for environmental assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EIACategory {
    /// Category A - Full Environmental Impact Assessment required
    /// ປະເພດ ກ - ຕ້ອງມີການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມເຕັມຮູບແບບ
    CategoryA,

    /// Category B - Initial Environmental Examination (IEE) required
    /// ປະເພດ ຂ - ຕ້ອງມີການກວດສອບສິ່ງແວດລ້ອມເບື້ອງຕົ້ນ (IEE)
    CategoryB,
}

impl EIACategory {
    /// Get the Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            EIACategory::CategoryA => "ປະເພດ ກ - ການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ",
            EIACategory::CategoryB => "ປະເພດ ຂ - ການກວດສອບສິ່ງແວດລ້ອມເບື້ອງຕົ້ນ",
        }
    }

    /// Get validity period in years (ຮັບໄລຍະເວລາໃຊ້ງານເປັນປີ)
    pub fn validity_years(&self) -> u32 {
        match self {
            EIACategory::CategoryA => EIA_VALIDITY_YEARS_CATEGORY_A,
            EIACategory::CategoryB => EIA_VALIDITY_YEARS_CATEGORY_B,
        }
    }
}

// ============================================================================
// Environmental Impact Assessment (ການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ)
// ============================================================================

/// Environmental Impact Assessment (ການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ)
///
/// Articles 18-24 of Environmental Protection Law 2012
/// Implements EIA requirements for development projects in Lao PDR
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnvironmentalImpactAssessment {
    // ========================================================================
    // Project Information (ຂໍ້ມູນໂຄງການ)
    // ========================================================================
    /// Project name in Lao (ຊື່ໂຄງການເປັນພາສາລາວ)
    pub project_name_lao: String,

    /// Project name in English (ຊື່ໂຄງການເປັນພາສາອັງກິດ)
    pub project_name_en: String,

    /// Project type (ປະເພດໂຄງການ)
    pub project_type: ProjectType,

    /// Project developer/proponent (ຜູ້ພັດທະນາໂຄງການ)
    pub project_developer: String,

    /// Province where project is located (ແຂວງ)
    pub location_province: String,

    /// District where project is located (ເມືອງ)
    pub location_district: Option<String>,

    /// Village where project is located (ບ້ານ)
    pub location_village: Option<String>,

    // ========================================================================
    // Assessment Details (ລາຍລະອຽດການປະເມີນ)
    // ========================================================================
    /// Assessment date (ວັນທີປະເມີນ)
    pub assessment_date: String,

    /// EIA category (ປະເພດ EIA)
    pub eia_category: EIACategory,

    /// EIA consultant (ທີ່ປຶກສາ EIA)
    pub eia_consultant: Option<String>,

    /// Assessed environmental impacts (ຜົນກະທົບທີ່ປະເມີນແລ້ວ)
    pub assessed_impacts: Vec<EnvironmentalImpact>,

    /// Proposed mitigation measures (ມາດຕະການບັນເທົາຜົນກະທົບ)
    pub mitigation_measures: Vec<MitigationMeasure>,

    /// Environmental management plan included (ລວມແຜນຄຸ້ມຄອງສິ່ງແວດລ້ອມ)
    pub has_management_plan: bool,

    /// Monitoring plan included (ລວມແຜນຕິດຕາມກວດກາ)
    pub has_monitoring_plan: bool,

    // ========================================================================
    // Public Participation (ການມີສ່ວນຮ່ວມຂອງປະຊາຊົນ)
    // ========================================================================
    /// Public consultation conducted (ດຳເນີນການປຶກສາຫາລືກັບປະຊາຊົນແລ້ວ)
    pub public_consultation_conducted: bool,

    /// Number of communities consulted (ຈຳນວນຊຸມຊົນທີ່ປຶກສາຫາລື)
    pub communities_consulted: u32,

    /// Affected households identified (ຄົວເຮືອນທີ່ໄດ້ຮັບຜົນກະທົບ)
    pub affected_households: Option<u32>,

    // ========================================================================
    // Approval Status (ສະຖານະການອະນຸມັດ)
    // ========================================================================
    /// Approval status (ສະຖານະການອະນຸມັດ)
    pub approval_status: EIAApprovalStatus,

    /// Approval date (ວັນທີອະນຸມັດ)
    pub approval_date: Option<String>,

    /// Approval authority (ອົງການອະນຸມັດ)
    pub approval_authority: Option<String>,

    /// Environmental certificate number (ເລກໃບຢັ້ງຢືນສິ່ງແວດລ້ອມ)
    pub certificate_number: Option<String>,

    /// Certificate expiry date (ວັນທີໝົດອາຍຸໃບຢັ້ງຢືນ)
    pub certificate_expiry: Option<String>,
}

impl Default for EnvironmentalImpactAssessment {
    fn default() -> Self {
        Self {
            project_name_lao: String::new(),
            project_name_en: String::new(),
            project_type: ProjectType::Other {
                description: String::new(),
            },
            project_developer: String::new(),
            location_province: String::new(),
            location_district: None,
            location_village: None,
            assessment_date: String::new(),
            eia_category: EIACategory::CategoryB,
            eia_consultant: None,
            assessed_impacts: Vec::new(),
            mitigation_measures: Vec::new(),
            has_management_plan: false,
            has_monitoring_plan: false,
            public_consultation_conducted: false,
            communities_consulted: 0,
            affected_households: None,
            approval_status: EIAApprovalStatus::Pending,
            approval_date: None,
            approval_authority: None,
            certificate_number: None,
            certificate_expiry: None,
        }
    }
}

/// Builder for EnvironmentalImpactAssessment
#[derive(Debug, Default)]
pub struct EnvironmentalImpactAssessmentBuilder {
    eia: EnvironmentalImpactAssessment,
}

impl EnvironmentalImpactAssessmentBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set project name in Lao
    pub fn project_name_lao(mut self, name: impl Into<String>) -> Self {
        self.eia.project_name_lao = name.into();
        self
    }

    /// Set project name in English
    pub fn project_name_en(mut self, name: impl Into<String>) -> Self {
        self.eia.project_name_en = name.into();
        self
    }

    /// Set project type
    pub fn project_type(mut self, project_type: ProjectType) -> Self {
        self.eia.project_type = project_type;
        self
    }

    /// Set project developer
    pub fn project_developer(mut self, developer: impl Into<String>) -> Self {
        self.eia.project_developer = developer.into();
        self
    }

    /// Set location province
    pub fn location_province(mut self, province: impl Into<String>) -> Self {
        self.eia.location_province = province.into();
        self
    }

    /// Set location district
    pub fn location_district(mut self, district: impl Into<String>) -> Self {
        self.eia.location_district = Some(district.into());
        self
    }

    /// Set assessment date
    pub fn assessment_date(mut self, date: impl Into<String>) -> Self {
        self.eia.assessment_date = date.into();
        self
    }

    /// Set EIA category
    pub fn eia_category(mut self, category: EIACategory) -> Self {
        self.eia.eia_category = category;
        self
    }

    /// Add an environmental impact
    pub fn add_impact(mut self, impact: EnvironmentalImpact) -> Self {
        self.eia.assessed_impacts.push(impact);
        self
    }

    /// Add a mitigation measure
    pub fn add_mitigation(mut self, measure: MitigationMeasure) -> Self {
        self.eia.mitigation_measures.push(measure);
        self
    }

    /// Set public consultation conducted
    pub fn public_consultation(mut self, conducted: bool, communities: u32) -> Self {
        self.eia.public_consultation_conducted = conducted;
        self.eia.communities_consulted = communities;
        self
    }

    /// Set approval status
    pub fn approval_status(mut self, status: EIAApprovalStatus) -> Self {
        self.eia.approval_status = status;
        self
    }

    /// Build the EIA
    pub fn build(self) -> EnvironmentalImpactAssessment {
        self.eia
    }
}

/// EIA approval status (ສະຖານະການອະນຸມັດ EIA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EIAApprovalStatus {
    /// Pending review (ລໍຖ້າການພິຈາລະນາ)
    Pending,

    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,

    /// Requires revision (ຕ້ອງແກ້ໄຂ)
    RequiresRevision,

    /// Approved (ອະນຸມັດແລ້ວ)
    Approved,

    /// Approved with conditions (ອະນຸມັດແບບມີເງື່ອນໄຂ)
    ApprovedWithConditions,

    /// Rejected (ປະຕິເສດ)
    Rejected,

    /// Expired (ໝົດອາຍຸ)
    Expired,

    /// Suspended (ໂຈະ)
    Suspended,
}

impl EIAApprovalStatus {
    /// Get Lao description (ຮັບລາຍລະອຽດເປັນພາສາລາວ)
    pub fn lao_description(&self) -> &'static str {
        match self {
            EIAApprovalStatus::Pending => "ລໍຖ້າການພິຈາລະນາ",
            EIAApprovalStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            EIAApprovalStatus::RequiresRevision => "ຕ້ອງແກ້ໄຂ",
            EIAApprovalStatus::Approved => "ອະນຸມັດແລ້ວ",
            EIAApprovalStatus::ApprovedWithConditions => "ອະນຸມັດແບບມີເງື່ອນໄຂ",
            EIAApprovalStatus::Rejected => "ປະຕິເສດ",
            EIAApprovalStatus::Expired => "ໝົດອາຍຸ",
            EIAApprovalStatus::Suspended => "ໂຈະ",
        }
    }
}

// ============================================================================
// Environmental Impact Types (ປະເພດຜົນກະທົບສິ່ງແວດລ້ອມ)
// ============================================================================

/// Impact severity level (ລະດັບຄວາມຮຸນແຮງຂອງຜົນກະທົບ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImpactSeverity {
    /// Negligible impact (ຜົນກະທົບເລັກນ້ອຍ)
    Negligible,

    /// Minor impact (ຜົນກະທົບນ້ອຍ)
    Minor,

    /// Moderate impact (ຜົນກະທົບປານກາງ)
    Moderate,

    /// Major impact (ຜົນກະທົບໃຫຍ່)
    Major,

    /// Critical impact (ຜົນກະທົບຮ້າຍແຮງ)
    Critical,
}

impl ImpactSeverity {
    /// Get Lao description (ຮັບລາຍລະອຽດເປັນພາສາລາວ)
    pub fn lao_description(&self) -> &'static str {
        match self {
            ImpactSeverity::Negligible => "ຜົນກະທົບເລັກນ້ອຍ",
            ImpactSeverity::Minor => "ຜົນກະທົບນ້ອຍ",
            ImpactSeverity::Moderate => "ຜົນກະທົບປານກາງ",
            ImpactSeverity::Major => "ຜົນກະທົບໃຫຍ່",
            ImpactSeverity::Critical => "ຜົນກະທົບຮ້າຍແຮງ",
        }
    }

    /// Check if requires mitigation (ກວດວ່າຕ້ອງມີມາດຕະການບັນເທົາ)
    pub fn requires_mitigation(&self) -> bool {
        matches!(
            self,
            ImpactSeverity::Moderate | ImpactSeverity::Major | ImpactSeverity::Critical
        )
    }
}

/// Environmental impact type (ປະເພດຜົນກະທົບສິ່ງແວດລ້ອມ)
///
/// Article 28-35: Environmental quality standards and impacts
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EnvironmentalImpact {
    /// Air pollution impact (ຜົນກະທົບມົນລະພິດທາງອາກາດ)
    AirPollution {
        /// Severity level (ລະດັບຄວາມຮຸນແຮງ)
        severity: ImpactSeverity,
        /// Pollutants identified (ມົນລະພິດທີ່ກຳນົດ)
        pollutants: Vec<AirPollutant>,
    },

    /// Water pollution impact (ຜົນກະທົບມົນລະພິດທາງນ້ຳ)
    WaterPollution {
        /// Severity level (ລະດັບຄວາມຮຸນແຮງ)
        severity: ImpactSeverity,
        /// Pollutants identified (ມົນລະພິດທີ່ກຳນົດ)
        pollutants: Vec<WaterPollutant>,
    },

    /// Soil contamination impact (ຜົນກະທົບການປົນເປື້ອນດິນ)
    SoilContamination {
        /// Severity level (ລະດັບຄວາມຮຸນແຮງ)
        severity: ImpactSeverity,
        /// Contaminants identified (ສານປົນເປື້ອນທີ່ກຳນົດ)
        contaminants: Vec<String>,
    },

    /// Noise pollution impact (ຜົນກະທົບມົນລະພິດທາງສຽງ)
    NoiseLevel {
        /// Expected noise level in decibels (ລະດັບສຽງທີ່ຄາດໄວ້ເປັນ dB)
        decibels: u8,
        /// Duration of noise exposure (ໄລຍະເວລາທີ່ມີສຽງດັງ)
        duration_description: Option<String>,
    },

    /// Deforestation impact (ຜົນກະທົບການຕັດໄມ້ທຳລາຍປ່າ)
    Deforestation {
        /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
        area_hectares: f64,
        /// Forest type (ປະເພດປ່າ)
        forest_type: Option<String>,
    },

    /// Biodiversity loss impact (ຜົນກະທົບການສູນເສຍຊີວະນາໆພັນ)
    BiodiversityLoss {
        /// Number of species potentially affected (ຈຳນວນຊະນິດພັນທີ່ອາດຈະໄດ້ຮັບຜົນກະທົບ)
        species_affected: u32,
        /// Endangered species present (ມີຊະນິດພັນທີ່ໃກ້ສູນພັນ)
        endangered_species_present: bool,
    },

    /// Waste generation impact (ຜົນກະທົບການສ້າງຂີ້ເຫຍື້ອ)
    WasteGeneration {
        /// Estimated waste in tons per year (ຂີ້ເຫຍື້ອຄາດຄະເນເປັນໂຕນຕໍ່ປີ)
        tons_per_year: f64,
        /// Waste type (ປະເພດຂີ້ເຫຍື້ອ)
        waste_type: WasteType,
    },

    /// Greenhouse gas emissions (ການປ່ອຍອາຍແກັສເຮືອນແກ້ວ)
    GreenhouseGas {
        /// CO2 equivalent tons per year (ທຽບເທົ່າ CO2 ໂຕນຕໍ່ປີ)
        co2_equivalent_tons: f64,
    },

    /// Social/community impact (ຜົນກະທົບສັງຄົມ/ຊຸມຊົນ)
    SocialImpact {
        /// Severity level (ລະດັບຄວາມຮຸນແຮງ)
        severity: ImpactSeverity,
        /// Description (ລາຍລະອຽດ)
        description: String,
        /// Households affected (ຄົວເຮືອນທີ່ໄດ້ຮັບຜົນກະທົບ)
        households_affected: Option<u32>,
    },

    /// Resettlement required (ຕ້ອງຍົກຍ້າຍປະຊາຊົນ)
    Resettlement {
        /// Number of households (ຈຳນວນຄົວເຮືອນ)
        households: u32,
        /// Number of people (ຈຳນວນຄົນ)
        people: u32,
    },
}

impl EnvironmentalImpact {
    /// Get the severity of this impact (ຮັບລະດັບຄວາມຮຸນແຮງ)
    pub fn severity(&self) -> ImpactSeverity {
        match self {
            EnvironmentalImpact::AirPollution { severity, .. } => *severity,
            EnvironmentalImpact::WaterPollution { severity, .. } => *severity,
            EnvironmentalImpact::SoilContamination { severity, .. } => *severity,
            EnvironmentalImpact::NoiseLevel { decibels, .. } => {
                if *decibels >= 85 {
                    ImpactSeverity::Critical
                } else if *decibels >= 75 {
                    ImpactSeverity::Major
                } else if *decibels >= 65 {
                    ImpactSeverity::Moderate
                } else if *decibels >= 55 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
            EnvironmentalImpact::Deforestation { area_hectares, .. } => {
                if *area_hectares >= 100.0 {
                    ImpactSeverity::Critical
                } else if *area_hectares >= 50.0 {
                    ImpactSeverity::Major
                } else if *area_hectares >= 10.0 {
                    ImpactSeverity::Moderate
                } else if *area_hectares >= 1.0 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
            EnvironmentalImpact::BiodiversityLoss {
                species_affected,
                endangered_species_present,
                ..
            } => {
                if *endangered_species_present {
                    ImpactSeverity::Critical
                } else if *species_affected >= 20 {
                    ImpactSeverity::Major
                } else if *species_affected >= 5 {
                    ImpactSeverity::Moderate
                } else if *species_affected >= 1 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
            EnvironmentalImpact::WasteGeneration { tons_per_year, .. } => {
                if *tons_per_year >= 10000.0 {
                    ImpactSeverity::Critical
                } else if *tons_per_year >= 1000.0 {
                    ImpactSeverity::Major
                } else if *tons_per_year >= 100.0 {
                    ImpactSeverity::Moderate
                } else if *tons_per_year >= 10.0 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
            EnvironmentalImpact::GreenhouseGas {
                co2_equivalent_tons,
            } => {
                if *co2_equivalent_tons >= 100000.0 {
                    ImpactSeverity::Critical
                } else if *co2_equivalent_tons >= 10000.0 {
                    ImpactSeverity::Major
                } else if *co2_equivalent_tons >= 1000.0 {
                    ImpactSeverity::Moderate
                } else if *co2_equivalent_tons >= 100.0 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
            EnvironmentalImpact::SocialImpact { severity, .. } => *severity,
            EnvironmentalImpact::Resettlement { households, .. } => {
                if *households >= 100 {
                    ImpactSeverity::Critical
                } else if *households >= 50 {
                    ImpactSeverity::Major
                } else if *households >= 10 {
                    ImpactSeverity::Moderate
                } else if *households >= 1 {
                    ImpactSeverity::Minor
                } else {
                    ImpactSeverity::Negligible
                }
            }
        }
    }

    /// Get Lao name of impact type (ຮັບຊື່ປະເພດຜົນກະທົບເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            EnvironmentalImpact::AirPollution { .. } => "ມົນລະພິດທາງອາກາດ",
            EnvironmentalImpact::WaterPollution { .. } => "ມົນລະພິດທາງນ້ຳ",
            EnvironmentalImpact::SoilContamination { .. } => "ການປົນເປື້ອນດິນ",
            EnvironmentalImpact::NoiseLevel { .. } => "ມົນລະພິດທາງສຽງ",
            EnvironmentalImpact::Deforestation { .. } => "ການຕັດໄມ້ທຳລາຍປ່າ",
            EnvironmentalImpact::BiodiversityLoss { .. } => "ການສູນເສຍຊີວະນາໆພັນ",
            EnvironmentalImpact::WasteGeneration { .. } => "ການສ້າງຂີ້ເຫຍື້ອ",
            EnvironmentalImpact::GreenhouseGas { .. } => "ການປ່ອຍອາຍແກັສເຮືອນແກ້ວ",
            EnvironmentalImpact::SocialImpact { .. } => "ຜົນກະທົບທາງສັງຄົມ",
            EnvironmentalImpact::Resettlement { .. } => "ການຍົກຍ້າຍຈັດສັນ",
        }
    }
}

// ============================================================================
// Pollutant Types (ປະເພດມົນລະພິດ)
// ============================================================================

/// Air pollutant type (ປະເພດມົນລະພິດທາງອາກາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AirPollutant {
    /// Particulate Matter 2.5 (ຝຸ່ນລະອຽດ PM2.5)
    PM25,
    /// Particulate Matter 10 (ຝຸ່ນລະອຽດ PM10)
    PM10,
    /// Sulfur Dioxide (ຊັນເຟີ ໄດອອກໄຊ)
    SO2,
    /// Nitrogen Dioxide (ໄນໂຕຣເຈນ ໄດອອກໄຊ)
    NO2,
    /// Carbon Monoxide (ຄາບອນໂມໂນອອກໄຊ)
    CO,
    /// Ozone (ໂອໂຊນ)
    O3,
    /// Lead (ຕະກົ່ວ)
    Lead,
    /// Volatile Organic Compounds (ສານອິນຊີລະເຫີຍ)
    VOC,
    /// Hydrogen Sulfide (ໄຮໂດຣເຈນ ຊັນໄຟ)
    H2S,
    /// Ammonia (ແອັມໂມເນຍ)
    NH3,
}

impl AirPollutant {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            AirPollutant::PM25 => "ຝຸ່ນລະອຽດ PM2.5",
            AirPollutant::PM10 => "ຝຸ່ນລະອຽດ PM10",
            AirPollutant::SO2 => "ຊັນເຟີ ໄດອອກໄຊ",
            AirPollutant::NO2 => "ໄນໂຕຣເຈນ ໄດອອກໄຊ",
            AirPollutant::CO => "ຄາບອນໂມໂນອອກໄຊ",
            AirPollutant::O3 => "ໂອໂຊນ",
            AirPollutant::Lead => "ຕະກົ່ວ",
            AirPollutant::VOC => "ສານອິນຊີລະເຫີຍ",
            AirPollutant::H2S => "ໄຮໂດຣເຈນ ຊັນໄຟ",
            AirPollutant::NH3 => "ແອັມໂມເນຍ",
        }
    }
}

/// Water pollutant type (ປະເພດມົນລະພິດທາງນ້ຳ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WaterPollutant {
    /// Biochemical Oxygen Demand (ຄວາມຕ້ອງການອອກຊີເຈນທາງຊີວະເຄມີ)
    BOD,
    /// Chemical Oxygen Demand (ຄວາມຕ້ອງການອອກຊີເຈນທາງເຄມີ)
    COD,
    /// Total Suspended Solids (ຂອງແຂງແຂວນລອຍທັງໝົດ)
    TSS,
    /// pH Level (ລະດັບ pH)
    PH,
    /// Temperature (ອຸນຫະພູມ)
    Temperature,
    /// Heavy Metals (ໂລຫະໜັກ)
    HeavyMetals,
    /// Oil and Grease (ນ້ຳມັນ ແລະ ໄຂ)
    OilGrease,
    /// Total Nitrogen (ໄນໂຕຣເຈນທັງໝົດ)
    TotalNitrogen,
    /// Total Phosphorus (ຟອສຟໍຣັສທັງໝົດ)
    TotalPhosphorus,
    /// Fecal Coliform (ແບັກທີເຣຍ)
    FecalColiform,
}

impl WaterPollutant {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            WaterPollutant::BOD => "ຄວາມຕ້ອງການອອກຊີເຈນທາງຊີວະເຄມີ",
            WaterPollutant::COD => "ຄວາມຕ້ອງການອອກຊີເຈນທາງເຄມີ",
            WaterPollutant::TSS => "ຂອງແຂງແຂວນລອຍທັງໝົດ",
            WaterPollutant::PH => "ລະດັບ pH",
            WaterPollutant::Temperature => "ອຸນຫະພູມ",
            WaterPollutant::HeavyMetals => "ໂລຫະໜັກ",
            WaterPollutant::OilGrease => "ນ້ຳມັນ ແລະ ໄຂ",
            WaterPollutant::TotalNitrogen => "ໄນໂຕຣເຈນທັງໝົດ",
            WaterPollutant::TotalPhosphorus => "ຟອສຟໍຣັສທັງໝົດ",
            WaterPollutant::FecalColiform => "ແບັກທີເຣຍໃນອາຈົມ",
        }
    }
}

// ============================================================================
// Mitigation Measures (ມາດຕະການບັນເທົາຜົນກະທົບ)
// ============================================================================

/// Mitigation measure (ມາດຕະການບັນເທົາຜົນກະທົບ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MitigationMeasure {
    /// Measure description in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,

    /// Measure description in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,

    /// Target impact type (ປະເພດຜົນກະທົບທີ່ເປົ້າໝາຍ)
    pub target_impact: String,

    /// Implementation phase (ໄລຍະການດຳເນີນ)
    pub implementation_phase: ImplementationPhase,

    /// Estimated cost in LAK (ຄ່າໃຊ້ຈ່າຍຄາດຄະເນ)
    pub estimated_cost_lak: Option<u64>,

    /// Responsible party (ຝ່າຍຮັບຜິດຊອບ)
    pub responsible_party: Option<String>,

    /// Monitoring indicator (ຕົວຊີ້ວັດການຕິດຕາມ)
    pub monitoring_indicator: Option<String>,
}

/// Implementation phase (ໄລຍະການດຳເນີນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImplementationPhase {
    /// Pre-construction phase (ກ່ອນກໍ່ສ້າງ)
    PreConstruction,
    /// Construction phase (ໄລຍະກໍ່ສ້າງ)
    Construction,
    /// Operation phase (ໄລຍະດຳເນີນງານ)
    Operation,
    /// Decommissioning phase (ໄລຍະປິດໂຄງການ)
    Decommissioning,
    /// All phases (ທຸກໄລຍະ)
    AllPhases,
}

impl ImplementationPhase {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ImplementationPhase::PreConstruction => "ກ່ອນກໍ່ສ້າງ",
            ImplementationPhase::Construction => "ໄລຍະກໍ່ສ້າງ",
            ImplementationPhase::Operation => "ໄລຍະດຳເນີນງານ",
            ImplementationPhase::Decommissioning => "ໄລຍະປິດໂຄງການ",
            ImplementationPhase::AllPhases => "ທຸກໄລຍະ",
        }
    }
}

// ============================================================================
// Pollution Source (ແຫຼ່ງມົນລະພິດ)
// ============================================================================

/// Pollution source (ແຫຼ່ງມົນລະພິດ)
///
/// Article 28-35: Pollution control and monitoring
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PollutionSource {
    /// Source name (ຊື່ແຫຼ່ງມົນລະພິດ)
    pub source_name: String,

    /// Source type (ປະເພດແຫຼ່ງ)
    pub source_type: PollutionSourceType,

    /// Emission type (ປະເພດການປ່ອຍມົນລະພິດ)
    pub emission_type: EmissionType,

    /// Emission quantity (ປະລິມານການປ່ອຍ)
    pub emission_quantity: f64,

    /// Measurement unit (ໜ່ວຍວັດແທກ)
    pub measurement_unit: String,

    /// Location (ສະຖານທີ່)
    pub location: Option<String>,

    /// Province (ແຂວງ)
    pub province: Option<String>,

    /// Compliance status (ສະຖານະການປະຕິບັດຕາມ)
    pub compliant: bool,

    /// Last inspection date (ວັນທີກວດກາລ່າສຸດ)
    pub last_inspection_date: Option<String>,
}

/// Pollution source type (ປະເພດແຫຼ່ງມົນລະພິດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PollutionSourceType {
    /// Industrial source (ແຫຼ່ງອຸດສາຫະກຳ)
    Industrial,
    /// Vehicle source (ແຫຼ່ງຍານພາຫະນະ)
    Vehicle,
    /// Agricultural source (ແຫຼ່ງກະສິກຳ)
    Agricultural,
    /// Domestic source (ແຫຼ່ງຄົວເຮືອນ)
    Domestic,
    /// Mining source (ແຫຼ່ງບໍ່ແຮ່)
    Mining,
    /// Construction source (ແຫຼ່ງກໍ່ສ້າງ)
    Construction,
}

impl PollutionSourceType {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PollutionSourceType::Industrial => "ແຫຼ່ງອຸດສາຫະກຳ",
            PollutionSourceType::Vehicle => "ແຫຼ່ງຍານພາຫະນະ",
            PollutionSourceType::Agricultural => "ແຫຼ່ງກະສິກຳ",
            PollutionSourceType::Domestic => "ແຫຼ່ງຄົວເຮືອນ",
            PollutionSourceType::Mining => "ແຫຼ່ງບໍ່ແຮ່",
            PollutionSourceType::Construction => "ແຫຼ່ງກໍ່ສ້າງ",
        }
    }
}

/// Emission type (ປະເພດການປ່ອຍມົນລະພິດ)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmissionType {
    /// Air emission (ການປ່ອຍທາງອາກາດ)
    AirEmission {
        /// Pollutant type (ປະເພດມົນລະພິດ)
        pollutant: AirPollutant,
    },

    /// Water discharge (ການປ່ອຍນ້ຳເສຍ)
    WaterDischarge {
        /// Pollutant type (ປະເພດມົນລະພິດ)
        pollutant: WaterPollutant,
    },

    /// Solid waste (ສິ່ງເສດເຫຼືອແຂງ)
    SolidWaste {
        /// Waste type (ປະເພດຂີ້ເຫຍື້ອ)
        waste_type: WasteType,
    },

    /// Noise emission (ການປ່ອຍສຽງ)
    Noise {
        /// Source description (ລາຍລະອຽດແຫຼ່ງ)
        source: String,
    },
}

/// Waste type (ປະເພດຂີ້ເຫຍື້ອ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WasteType {
    /// General waste (ຂີ້ເຫຍື້ອທົ່ວໄປ)
    General,
    /// Hazardous waste (ຂີ້ເຫຍື້ອອັນຕະລາຍ)
    Hazardous,
    /// Medical waste (ຂີ້ເຫຍື້ອທາງການແພດ)
    Medical,
    /// Industrial waste (ຂີ້ເຫຍື້ອອຸດສາຫະກຳ)
    Industrial,
    /// Construction waste (ຂີ້ເຫຍື້ອກໍ່ສ້າງ)
    Construction,
    /// Electronic waste (ຂີ້ເຫຍື້ອອີເລັກໂຕຣນິກ)
    Electronic,
    /// Organic waste (ຂີ້ເຫຍື້ອອິນຊີ)
    Organic,
    /// Recyclable waste (ຂີ້ເຫຍື້ອທີ່ຣີໄຊເຄິນໄດ້)
    Recyclable,
}

impl WasteType {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            WasteType::General => "ຂີ້ເຫຍື້ອທົ່ວໄປ",
            WasteType::Hazardous => "ຂີ້ເຫຍື້ອອັນຕະລາຍ",
            WasteType::Medical => "ຂີ້ເຫຍື້ອທາງການແພດ",
            WasteType::Industrial => "ຂີ້ເຫຍື້ອອຸດສາຫະກຳ",
            WasteType::Construction => "ຂີ້ເຫຍື້ອກໍ່ສ້າງ",
            WasteType::Electronic => "ຂີ້ເຫຍື້ອອີເລັກໂຕຣນິກ",
            WasteType::Organic => "ຂີ້ເຫຍື້ອອິນຊີ",
            WasteType::Recyclable => "ຂີ້ເຫຍື້ອທີ່ຣີໄຊເຄິນໄດ້",
        }
    }

    /// Check if this is hazardous waste requiring special handling
    pub fn is_hazardous(&self) -> bool {
        matches!(
            self,
            WasteType::Hazardous | WasteType::Medical | WasteType::Electronic
        )
    }
}

// ============================================================================
// Protected Areas (ເຂດປ່າປ້ອງກັນ)
// ============================================================================

/// Protected area (ເຂດປ່າປ້ອງກັນ)
///
/// Article 38-45: Protected area management
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProtectedArea {
    /// Name in Lao (ຊື່ເປັນພາສາລາວ)
    pub name_lao: String,

    /// Name in English (ຊື່ເປັນພາສາອັງກິດ)
    pub name_en: String,

    /// Area type (ປະເພດເຂດ)
    pub area_type: ProtectedAreaType,

    /// Area in hectares (ເນື້ອທີ່ເປັນເຮັກຕາ)
    pub area_hectares: f64,

    /// Province (ແຂວງ)
    pub province: String,

    /// Districts covered (ເມືອງທີ່ກວມເອົາ)
    pub districts: Vec<String>,

    /// Establishment date (ວັນທີສ້າງຕັ້ງ)
    pub establishment_date: String,

    /// IUCN category (ປະເພດ IUCN)
    pub iucn_category: Option<IUCNCategory>,

    /// Management authority (ອົງການຄຸ້ມຄອງ)
    pub management_authority: Option<String>,

    /// Key species present (ຊະນິດພັນສຳຄັນ)
    pub key_species: Vec<String>,

    /// Buffer zone area in hectares (ເຂດກັນຊົນເປັນເຮັກຕາ)
    pub buffer_zone_hectares: Option<f64>,
}

/// Protected area type (ປະເພດເຂດປ່າປ້ອງກັນ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectedAreaType {
    /// National Protected Area (ເຂດປ່າປ້ອງກັນແຫ່ງຊາດ)
    NationalProtectedArea,
    /// National Park (ສວນສັດແຫ່ງຊາດ)
    NationalPark,
    /// Wildlife Sanctuary (ເຂດປ່າສະຫງວນສັດປ່າ)
    WildlifeSanctuary,
    /// Protection Forest (ປ່າປ້ອງກັນ)
    ProtectionForest,
    /// Conservation Forest (ປ່າສະຫງວນ)
    ConservationForest,
    /// Wetland Reserve (ເຂດສະຫງວນທີ່ດິນບຶງ)
    WetlandReserve,
    /// Provincial Protected Area (ເຂດປ່າປ້ອງກັນຂອງແຂວງ)
    ProvincialProtectedArea,
    /// District Protected Area (ເຂດປ່າປ້ອງກັນຂອງເມືອງ)
    DistrictProtectedArea,
}

impl ProtectedAreaType {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProtectedAreaType::NationalProtectedArea => "ເຂດປ່າປ້ອງກັນແຫ່ງຊາດ",
            ProtectedAreaType::NationalPark => "ສວນສັດແຫ່ງຊາດ",
            ProtectedAreaType::WildlifeSanctuary => "ເຂດປ່າສະຫງວນສັດປ່າ",
            ProtectedAreaType::ProtectionForest => "ປ່າປ້ອງກັນ",
            ProtectedAreaType::ConservationForest => "ປ່າສະຫງວນ",
            ProtectedAreaType::WetlandReserve => "ເຂດສະຫງວນທີ່ດິນບຶງ",
            ProtectedAreaType::ProvincialProtectedArea => "ເຂດປ່າປ້ອງກັນຂອງແຂວງ",
            ProtectedAreaType::DistrictProtectedArea => "ເຂດປ່າປ້ອງກັນຂອງເມືອງ",
        }
    }

    /// Get restriction level (ລະດັບການຈຳກັດ)
    pub fn restriction_level(&self) -> RestrictionLevel {
        match self {
            ProtectedAreaType::NationalProtectedArea | ProtectedAreaType::WildlifeSanctuary => {
                RestrictionLevel::Strict
            }
            ProtectedAreaType::NationalPark | ProtectedAreaType::ConservationForest => {
                RestrictionLevel::High
            }
            ProtectedAreaType::ProtectionForest | ProtectedAreaType::WetlandReserve => {
                RestrictionLevel::Moderate
            }
            ProtectedAreaType::ProvincialProtectedArea
            | ProtectedAreaType::DistrictProtectedArea => RestrictionLevel::Low,
        }
    }
}

/// Restriction level for protected areas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RestrictionLevel {
    /// Strict - No development allowed
    Strict,
    /// High - Limited ecotourism only
    High,
    /// Moderate - Sustainable use permitted
    Moderate,
    /// Low - Some activities permitted with approval
    Low,
}

/// IUCN protected area category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IUCNCategory {
    /// Ia - Strict Nature Reserve
    Ia,
    /// Ib - Wilderness Area
    Ib,
    /// II - National Park
    II,
    /// III - Natural Monument
    III,
    /// IV - Habitat/Species Management Area
    IV,
    /// V - Protected Landscape/Seascape
    V,
    /// VI - Protected Area with Sustainable Use
    VI,
}

impl IUCNCategory {
    /// Get description (ຮັບລາຍລະອຽດ)
    pub fn description(&self) -> &'static str {
        match self {
            IUCNCategory::Ia => "Strict Nature Reserve",
            IUCNCategory::Ib => "Wilderness Area",
            IUCNCategory::II => "National Park",
            IUCNCategory::III => "Natural Monument",
            IUCNCategory::IV => "Habitat/Species Management Area",
            IUCNCategory::V => "Protected Landscape/Seascape",
            IUCNCategory::VI => "Protected Area with Sustainable Use",
        }
    }
}

// ============================================================================
// Environmental Permits (ໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
// ============================================================================

/// Environmental permit (ໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
///
/// Article 25-27: Environmental permits and compliance
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnvironmentalPermit {
    /// Permit number (ເລກໃບອະນຸຍາດ)
    pub permit_number: String,

    /// Holder name (ຊື່ຜູ້ຖືໃບອະນຸຍາດ)
    pub holder_name: String,

    /// Holder name in Lao (ຊື່ຜູ້ຖືໃບອະນຸຍາດເປັນພາສາລາວ)
    pub holder_name_lao: Option<String>,

    /// Permit type (ປະເພດໃບອະນຸຍາດ)
    pub permit_type: EnvironmentalPermitType,

    /// Issue date (ວັນທີອອກ)
    pub issue_date: String,

    /// Expiry date (ວັນທີໝົດອາຍຸ)
    pub expiry_date: String,

    /// Issuing authority (ອົງການອອກໃບອະນຸຍາດ)
    pub issuing_authority: String,

    /// Conditions attached (ເງື່ອນໄຂທີ່ແນບມາ)
    pub conditions: Vec<PermitCondition>,

    /// Permit status (ສະຖານະໃບອະນຸຍາດ)
    pub status: PermitStatus,

    /// Project/facility name (ຊື່ໂຄງການ/ສະຖານທີ່)
    pub project_name: Option<String>,

    /// Location province (ແຂວງ)
    pub location_province: Option<String>,
}

impl EnvironmentalPermit {
    /// Check if permit is valid (ກວດວ່າໃບອະນຸຍາດຍັງໃຊ້ໄດ້)
    pub fn is_valid(&self) -> bool {
        self.status == PermitStatus::Active
    }
}

/// Environmental permit type (ປະເພດໃບອະນຸຍາດສິ່ງແວດລ້ອມ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EnvironmentalPermitType {
    /// EIA Certificate (ໃບຢັ້ງຢືນ EIA)
    EIACertificate,
    /// Emission Permit (ໃບອະນຸຍາດປ່ອຍມົນລະພິດ)
    EmissionPermit,
    /// Waste Disposal Permit (ໃບອະນຸຍາດຖິ້ມຂີ້ເຫຍື້ອ)
    WasteDisposalPermit,
    /// Water Extraction Permit (ໃບອະນຸຍາດສູບນ້ຳ)
    WaterExtractionPermit,
    /// Forestry Permit (ໃບອະນຸຍາດປ່າໄມ້)
    ForestryPermit,
    /// Mining Environmental Permit (ໃບອະນຸຍາດສິ່ງແວດລ້ອມບໍ່ແຮ່)
    MiningEnvironmentalPermit,
    /// Environmental Compliance Certificate (ໃບຢັ້ງຢືນການປະຕິບັດຕາມສິ່ງແວດລ້ອມ)
    EnvironmentalComplianceCertificate,
    /// Hazardous Waste Transport Permit (ໃບອະນຸຍາດຂົນສົ່ງຂີ້ເຫຍື້ອອັນຕະລາຍ)
    HazardousWasteTransportPermit,
}

impl EnvironmentalPermitType {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            EnvironmentalPermitType::EIACertificate => "ໃບຢັ້ງຢືນ EIA",
            EnvironmentalPermitType::EmissionPermit => "ໃບອະນຸຍາດປ່ອຍມົນລະພິດ",
            EnvironmentalPermitType::WasteDisposalPermit => "ໃບອະນຸຍາດຖິ້ມຂີ້ເຫຍື້ອ",
            EnvironmentalPermitType::WaterExtractionPermit => "ໃບອະນຸຍາດສູບນ້ຳ",
            EnvironmentalPermitType::ForestryPermit => "ໃບອະນຸຍາດປ່າໄມ້",
            EnvironmentalPermitType::MiningEnvironmentalPermit => "ໃບອະນຸຍາດສິ່ງແວດລ້ອມບໍ່ແຮ່",
            EnvironmentalPermitType::EnvironmentalComplianceCertificate => {
                "ໃບຢັ້ງຢືນການປະຕິບັດຕາມສິ່ງແວດລ້ອມ"
            }
            EnvironmentalPermitType::HazardousWasteTransportPermit => "ໃບອະນຸຍາດຂົນສົ່ງຂີ້ເຫຍື້ອອັນຕະລາຍ",
        }
    }
}

/// Permit condition (ເງື່ອນໄຂໃບອະນຸຍາດ)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PermitCondition {
    /// Condition description (ລາຍລະອຽດເງື່ອນໄຂ)
    pub description: String,

    /// Condition description in Lao (ລາຍລະອຽດເງື່ອນໄຂເປັນພາສາລາວ)
    pub description_lao: Option<String>,

    /// Compliance deadline (ກຳນົດການປະຕິບັດຕາມ)
    pub compliance_deadline: Option<String>,

    /// Compliance status (ສະຖານະການປະຕິບັດຕາມ)
    pub compliant: bool,
}

/// Permit status (ສະຖານະໃບອະນຸຍາດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PermitStatus {
    /// Active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Pending (ລໍຖ້າ)
    Pending,
    /// Suspended (ໂຈະ)
    Suspended,
    /// Revoked (ຖືກຖອນ)
    Revoked,
    /// Expired (ໝົດອາຍຸ)
    Expired,
    /// Renewed (ຕໍ່ອາຍຸແລ້ວ)
    Renewed,
}

impl PermitStatus {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            PermitStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            PermitStatus::Pending => "ລໍຖ້າ",
            PermitStatus::Suspended => "ໂຈະ",
            PermitStatus::Revoked => "ຖືກຖອນ",
            PermitStatus::Expired => "ໝົດອາຍຸ",
            PermitStatus::Renewed => "ຕໍ່ອາຍຸແລ້ວ",
        }
    }
}

// ============================================================================
// Zone Type (ປະເພດເຂດ)
// ============================================================================

/// Zone type for noise standards (ປະເພດເຂດສຳລັບມາດຕະຖານສຽງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ZoneType {
    /// Residential area (ເຂດທີ່ຢູ່ອາໄສ)
    Residential,
    /// Commercial area (ເຂດການຄ້າ)
    Commercial,
    /// Industrial area (ເຂດອຸດສາຫະກຳ)
    Industrial,
    /// Mixed use area (ເຂດນຳໃຊ້ຫຼາຍຈຸດປະສົງ)
    MixedUse,
    /// Rural area (ເຂດຊົນນະບົດ)
    Rural,
    /// Protected area (ເຂດປ່າປ້ອງກັນ)
    Protected,
}

impl ZoneType {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ZoneType::Residential => "ເຂດທີ່ຢູ່ອາໄສ",
            ZoneType::Commercial => "ເຂດການຄ້າ",
            ZoneType::Industrial => "ເຂດອຸດສາຫະກຳ",
            ZoneType::MixedUse => "ເຂດນຳໃຊ້ຫຼາຍຈຸດປະສົງ",
            ZoneType::Rural => "ເຂດຊົນນະບົດ",
            ZoneType::Protected => "ເຂດປ່າປ້ອງກັນ",
        }
    }

    /// Get maximum noise level for day (ຮັບລະດັບສຽງສູງສຸດສຳລັບກາງວັນ)
    pub fn max_noise_day(&self) -> u8 {
        match self {
            ZoneType::Residential => MAX_NOISE_RESIDENTIAL_DAY,
            ZoneType::Commercial => MAX_NOISE_COMMERCIAL_DAY,
            ZoneType::Industrial => MAX_NOISE_INDUSTRIAL,
            ZoneType::MixedUse => 60,
            ZoneType::Rural => 50,
            ZoneType::Protected => 45,
        }
    }

    /// Get maximum noise level for night (ຮັບລະດັບສຽງສູງສຸດສຳລັບກາງຄືນ)
    pub fn max_noise_night(&self) -> u8 {
        match self {
            ZoneType::Residential => MAX_NOISE_RESIDENTIAL_NIGHT,
            ZoneType::Commercial => MAX_NOISE_COMMERCIAL_NIGHT,
            ZoneType::Industrial => 70,
            ZoneType::MixedUse => 50,
            ZoneType::Rural => 40,
            ZoneType::Protected => 35,
        }
    }
}

// ============================================================================
// Activity Type for Protected Areas (ປະເພດກິດຈະກຳສຳລັບເຂດປ່າປ້ອງກັນ)
// ============================================================================

/// Activity type for protected areas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProtectedAreaActivity {
    /// Research (ການຄົ້ນຄວ້າ)
    Research,
    /// Ecotourism (ການທ່ອງທ່ຽວນິເວດ)
    Ecotourism,
    /// Education (ການສຶກສາ)
    Education,
    /// Traditional use (ການນຳໃຊ້ແບບດັ້ງເດີມ)
    TraditionalUse,
    /// Sustainable harvesting (ການເກັບກ່ຽວແບບຍືນຍົງ)
    SustainableHarvesting,
    /// Infrastructure development (ການພັດທະນາພື້ນຖານໂຄງລ່າງ)
    InfrastructureDevelopment,
    /// Mining (ການຂຸດຄົ້ນບໍ່ແຮ່)
    Mining,
    /// Logging (ການຕັດໄມ້)
    Logging,
    /// Agriculture (ການກະສິກຳ)
    Agriculture,
    /// Settlement (ການຕັ້ງຖິ່ນຖານ)
    Settlement,
}

impl ProtectedAreaActivity {
    /// Get Lao name (ຮັບຊື່ເປັນພາສາລາວ)
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProtectedAreaActivity::Research => "ການຄົ້ນຄວ້າ",
            ProtectedAreaActivity::Ecotourism => "ການທ່ອງທ່ຽວນິເວດ",
            ProtectedAreaActivity::Education => "ການສຶກສາ",
            ProtectedAreaActivity::TraditionalUse => "ການນຳໃຊ້ແບບດັ້ງເດີມ",
            ProtectedAreaActivity::SustainableHarvesting => "ການເກັບກ່ຽວແບບຍືນຍົງ",
            ProtectedAreaActivity::InfrastructureDevelopment => "ການພັດທະນາພື້ນຖານໂຄງລ່າງ",
            ProtectedAreaActivity::Mining => "ການຂຸດຄົ້ນບໍ່ແຮ່",
            ProtectedAreaActivity::Logging => "ການຕັດໄມ້",
            ProtectedAreaActivity::Agriculture => "ການກະສິກຳ",
            ProtectedAreaActivity::Settlement => "ການຕັ້ງຖິ່ນຖານ",
        }
    }

    /// Check if activity is allowed in a given protected area type
    pub fn is_allowed_in(&self, area_type: &ProtectedAreaType) -> bool {
        match area_type.restriction_level() {
            RestrictionLevel::Strict => matches!(self, ProtectedAreaActivity::Research),
            RestrictionLevel::High => matches!(
                self,
                ProtectedAreaActivity::Research
                    | ProtectedAreaActivity::Education
                    | ProtectedAreaActivity::Ecotourism
            ),
            RestrictionLevel::Moderate => matches!(
                self,
                ProtectedAreaActivity::Research
                    | ProtectedAreaActivity::Education
                    | ProtectedAreaActivity::Ecotourism
                    | ProtectedAreaActivity::TraditionalUse
                    | ProtectedAreaActivity::SustainableHarvesting
            ),
            RestrictionLevel::Low => !matches!(
                self,
                ProtectedAreaActivity::Mining | ProtectedAreaActivity::Logging
            ),
        }
    }
}

// ============================================================================
// Waste Disposal Method (ວິທີກຳຈັດຂີ້ເຫຍື້ອ)
// ============================================================================

/// Waste disposal method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WasteDisposalMethod {
    /// Landfill (ຖິ້ມໃສ່ບ່ອນຖິ້ມຂີ້ເຫຍື້ອ)
    Landfill,
    /// Incineration (ເຜົາ)
    Incineration,
    /// Recycling (ຣີໄຊເຄິນ)
    Recycling,
    /// Composting (ເຮັດປຸ໋ຍ)
    Composting,
    /// Treatment (ບຳບັດ)
    Treatment,
    /// Deep well injection (ສີດໃສ່ບໍ່ເລິກ)
    DeepWellInjection,
    /// Ocean disposal (ຖິ້ມໃສ່ທະເລ)
    OceanDisposal,
    /// Specialized facility (ສະຖານທີ່ພິເສດ)
    SpecializedFacility,
}

impl WasteDisposalMethod {
    /// Get Lao name
    pub fn lao_name(&self) -> &'static str {
        match self {
            WasteDisposalMethod::Landfill => "ຖິ້ມໃສ່ບ່ອນຖິ້ມຂີ້ເຫຍື້ອ",
            WasteDisposalMethod::Incineration => "ເຜົາ",
            WasteDisposalMethod::Recycling => "ຣີໄຊເຄິນ",
            WasteDisposalMethod::Composting => "ເຮັດປຸ໋ຍ",
            WasteDisposalMethod::Treatment => "ບຳບັດ",
            WasteDisposalMethod::DeepWellInjection => "ສີດໃສ່ບໍ່ເລິກ",
            WasteDisposalMethod::OceanDisposal => "ຖິ້ມໃສ່ທະເລ",
            WasteDisposalMethod::SpecializedFacility => "ສະຖານທີ່ພິເສດ",
        }
    }

    /// Check if method is appropriate for waste type
    pub fn is_appropriate_for(&self, waste_type: &WasteType) -> bool {
        match waste_type {
            WasteType::General => matches!(
                self,
                WasteDisposalMethod::Landfill
                    | WasteDisposalMethod::Incineration
                    | WasteDisposalMethod::Recycling
            ),
            WasteType::Hazardous => matches!(
                self,
                WasteDisposalMethod::Treatment | WasteDisposalMethod::SpecializedFacility
            ),
            WasteType::Medical => matches!(
                self,
                WasteDisposalMethod::Incineration | WasteDisposalMethod::SpecializedFacility
            ),
            WasteType::Organic => matches!(
                self,
                WasteDisposalMethod::Composting | WasteDisposalMethod::Landfill
            ),
            WasteType::Recyclable => matches!(self, WasteDisposalMethod::Recycling),
            _ => true,
        }
    }
}
