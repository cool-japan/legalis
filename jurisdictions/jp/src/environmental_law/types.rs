//! Environmental Law Types
//!
//! Core data structures for Air Pollution Control Act (大気汚染防止法),
//! Water Pollution Prevention Act (水質汚濁防止法),
//! and Waste Management Act (廃棄物処理法).

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of pollution (公害の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PollutionType {
    /// Air pollution (大気汚染)
    Air,
    /// Water pollution (水質汚濁)
    Water,
    /// Soil contamination (土壌汚染)
    Soil,
    /// Noise (騒音)
    Noise,
    /// Vibration (振動)
    Vibration,
    /// Ground subsidence (地盤沈下)
    GroundSubsidence,
    /// Odor (悪臭)
    Odor,
}

impl PollutionType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Air => "大気汚染",
            Self::Water => "水質汚濁",
            Self::Soil => "土壌汚染",
            Self::Noise => "騒音",
            Self::Vibration => "振動",
            Self::GroundSubsidence => "地盤沈下",
            Self::Odor => "悪臭",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Air => "Air Pollution",
            Self::Water => "Water Pollution",
            Self::Soil => "Soil Contamination",
            Self::Noise => "Noise",
            Self::Vibration => "Vibration",
            Self::GroundSubsidence => "Ground Subsidence",
            Self::Odor => "Odor",
        }
    }
}

/// Type of pollutant (汚染物質)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pollutant {
    /// Sulfur oxides (硫黄酸化物 - SOx)
    SulfurOxides,
    /// Nitrogen oxides (窒素酸化物 - NOx)
    NitrogenOxides,
    /// Particulates (ばいじん)
    Particulates,
    /// Volatile organic compounds (揮発性有機化合物 - VOC)
    VolatileOrganic,
    /// Dioxins (ダイオキシン類)
    Dioxins,
    /// Heavy metals (重金属)
    HeavyMetals(HeavyMetal),
    /// Biochemical oxygen demand (生物化学的酸素要求量 - BOD)
    BiochemicalOxygen,
    /// Chemical oxygen demand (化学的酸素要求量 - COD)
    ChemicalOxygen,
}

/// Heavy metal type (重金属の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeavyMetal {
    /// Lead (鉛)
    Lead,
    /// Mercury (水銀)
    Mercury,
    /// Cadmium (カドミウム)
    Cadmium,
    /// Arsenic (ヒ素)
    Arsenic,
    /// Chromium (クロム)
    Chromium,
}

/// Emission limit (排出基準)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmissionLimit {
    /// Pollutant
    pub pollutant: Pollutant,
    /// Limit value
    pub limit_value: f64,
    /// Unit (e.g., "ppm", "mg/m³")
    pub unit: String,
    /// Legal basis (e.g., "大気汚染防止法第3条")
    pub legal_basis: String,
}

/// Facility type (施設の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FacilityType {
    /// Power plant (発電所)
    PowerPlant,
    /// Incinerator (焼却施設)
    Incinerator,
    /// Chemical plant (化学工場)
    ChemicalPlant,
    /// Steel mill (製鉄所)
    SteelMill,
    /// Waste processing facility (廃棄物処理施設)
    WasteProcessing,
    /// Wastewater treatment (排水処理施設)
    WastewaterTreatment,
}

impl FacilityType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::PowerPlant => "発電所",
            Self::Incinerator => "焼却施設",
            Self::ChemicalPlant => "化学工場",
            Self::SteelMill => "製鉄所",
            Self::WasteProcessing => "廃棄物処理施設",
            Self::WastewaterTreatment => "排水処理施設",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PowerPlant => "Power Plant",
            Self::Incinerator => "Incinerator",
            Self::ChemicalPlant => "Chemical Plant",
            Self::SteelMill => "Steel Mill",
            Self::WasteProcessing => "Waste Processing Facility",
            Self::WastewaterTreatment => "Wastewater Treatment",
        }
    }
}

/// Pollution control equipment (公害防止設備)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControlEquipment {
    /// Equipment type
    pub equipment_type: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Installation date
    pub installation_date: NaiveDate,
    /// Designed efficiency (%)
    pub designed_efficiency: f64,
}

/// Emission estimate (排出見込量)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmissionEstimate {
    /// Pollutant
    pub pollutant: Pollutant,
    /// Estimated value
    pub estimated_value: f64,
    /// Unit
    pub unit: String,
}

/// Monitoring requirement (監視要件)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonitoringRequirement {
    /// What to monitor
    pub parameter: String,
    /// Frequency (e.g., "daily", "monthly")
    pub frequency: String,
    /// Reporting obligation
    pub reporting_required: bool,
}

/// Pollution prevention agreement (公害防止協定)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PollutionPreventionAgreement {
    /// Facility name
    pub facility_name: String,
    /// Facility type
    pub facility_type: FacilityType,
    /// Operator name
    pub operator: String,
    /// Location
    pub location: String,
    /// Pollution types
    pub pollution_types: Vec<PollutionType>,
    /// Emission limits
    pub emission_limits: Vec<EmissionLimit>,
    /// Monitoring requirements
    pub monitoring_requirements: Vec<MonitoringRequirement>,
    /// Effective date
    pub effective_date: NaiveDate,
}

/// Factory setup notification (工場設置届出)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FactorySetupNotification {
    /// Facility name
    pub facility_name: String,
    /// Facility type
    pub facility_type: FacilityType,
    /// Location
    pub location: String,
    /// Installation date
    pub installation_date: NaiveDate,
    /// Expected emissions
    pub expected_emissions: Vec<EmissionEstimate>,
    /// Pollution control equipment
    pub pollution_control_equipment: Vec<ControlEquipment>,
    /// Submitted to agency
    pub submitted_to: crate::egov::GovernmentAgency,
    /// Notification date
    pub notification_date: Option<NaiveDate>,
}

impl FactorySetupNotification {
    /// Check if prior notification requirement is met (60 days before installation)
    pub fn meets_prior_notification(&self) -> bool {
        if let Some(notification_date) = self.notification_date {
            let days_before = (self.installation_date - notification_date).num_days();
            days_before >= 60
        } else {
            false
        }
    }

    /// Get days until installation
    pub fn days_until_installation(&self) -> i64 {
        let now = chrono::Utc::now().date_naive();
        (self.installation_date - now).num_days()
    }
}

/// Waste permit type (廃棄物許可の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WastePermitType {
    /// Collection and transport (収集運搬業 - Article 7)
    Collection,
    /// Disposal (処分業 - Article 14)
    Disposal,
    /// Intermediate treatment (中間処理)
    Intermediate,
    /// Final disposal (最終処分)
    Final,
    /// Industrial waste (産業廃棄物)
    IndustrialWaste,
}

impl WastePermitType {
    /// Get validity period in years
    pub fn validity_years(&self) -> u32 {
        match self {
            Self::Collection => 5,
            Self::Disposal => 7,
            Self::Intermediate => 5,
            Self::Final => 7,
            Self::IndustrialWaste => 5,
        }
    }

    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Collection => "収集運搬業",
            Self::Disposal => "処分業",
            Self::Intermediate => "中間処理",
            Self::Final => "最終処分",
            Self::IndustrialWaste => "産業廃棄物",
        }
    }
}

/// Waste type (廃棄物の種類)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WasteType {
    /// Municipal waste (一般廃棄物)
    Municipal,
    /// Industrial waste (産業廃棄物)
    Industrial,
    /// Special industrial waste (特別管理産業廃棄物)
    SpecialIndustrial,
    /// Infectious waste (感染性廃棄物)
    Infectious,
    /// Explosive waste (爆発性廃棄物)
    Explosive,
    /// Toxic waste (有害廃棄物)
    Toxic,
}

impl WasteType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Municipal => "一般廃棄物",
            Self::Industrial => "産業廃棄物",
            Self::SpecialIndustrial => "特別管理産業廃棄物",
            Self::Infectious => "感染性廃棄物",
            Self::Explosive => "爆発性廃棄物",
            Self::Toxic => "有害廃棄物",
        }
    }

    /// Check if special handling required
    pub fn requires_special_handling(&self) -> bool {
        matches!(
            self,
            Self::SpecialIndustrial | Self::Infectious | Self::Explosive | Self::Toxic
        )
    }
}

/// Party in waste transaction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Party {
    /// Name
    pub name: String,
    /// Address
    pub address: String,
    /// Contact information
    pub contact: Option<String>,
}

/// Waste management permit (廃棄物処理業許可)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WasteManagementPermit {
    /// Permit type
    pub permit_type: WastePermitType,
    /// Permit number
    pub permit_number: String,
    /// Operator name
    pub operator_name: String,
    /// Waste types authorized
    pub waste_types: Vec<WasteType>,
    /// Processing capacity (tons per day)
    pub processing_capacity_tons_per_day: f64,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Expiration date
    pub expiration_date: NaiveDate,
    /// Facility standards met
    pub facility_standards_met: bool,
}

impl WasteManagementPermit {
    /// Check if permit is currently valid
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().date_naive();
        now >= self.issue_date && now <= self.expiration_date
    }

    /// Get days until expiration
    pub fn days_until_expiration(&self) -> i64 {
        let now = chrono::Utc::now().date_naive();
        (self.expiration_date - now).num_days()
    }

    /// Check if permit validity period is correct
    pub fn has_correct_validity_period(&self) -> bool {
        let expected_years = self.permit_type.validity_years();
        let actual_days = (self.expiration_date - self.issue_date).num_days();
        let expected_days = (expected_years * 365) as i64;

        // Allow for some tolerance (±30 days)
        (actual_days - expected_days).abs() <= 30
    }
}

/// Waste manifest (マニフェスト)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WasteManifest {
    /// Manifest number
    pub manifest_number: String,
    /// Waste type
    pub waste_type: WasteType,
    /// Quantity in kilograms
    pub quantity_kg: f64,
    /// Generator (排出事業者)
    pub generator: Party,
    /// Transporter (収集運搬業者)
    pub transporter: Party,
    /// Processor (処分業者)
    pub processor: Party,
    /// Issue date
    pub issue_date: NaiveDate,
    /// Completion date
    pub completion_date: Option<NaiveDate>,
}

impl WasteManifest {
    /// Check if manifest is complete
    pub fn is_complete(&self) -> bool {
        self.completion_date.is_some()
    }

    /// Get days since issue
    pub fn days_since_issue(&self) -> i64 {
        let now = chrono::Utc::now().date_naive();
        (now - self.issue_date).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_pollution_type_names() {
        assert_eq!(PollutionType::Air.name_ja(), "大気汚染");
        assert_eq!(PollutionType::Air.name_en(), "Air Pollution");
    }

    #[test]
    fn test_facility_type_names() {
        assert_eq!(FacilityType::PowerPlant.name_ja(), "発電所");
        assert_eq!(FacilityType::PowerPlant.name_en(), "Power Plant");
    }

    #[test]
    fn test_waste_permit_validity_years() {
        assert_eq!(WastePermitType::Collection.validity_years(), 5);
        assert_eq!(WastePermitType::Disposal.validity_years(), 7);
    }

    #[test]
    fn test_waste_type_special_handling() {
        assert!(WasteType::Infectious.requires_special_handling());
        assert!(!WasteType::Municipal.requires_special_handling());
    }

    #[test]
    fn test_factory_notification_prior_requirement() {
        let notification = FactorySetupNotification {
            facility_name: "Test Factory".to_string(),
            facility_type: FacilityType::ChemicalPlant,
            location: "Tokyo".to_string(),
            installation_date: Utc::now().date_naive() + chrono::Duration::days(90),
            expected_emissions: vec![],
            pollution_control_equipment: vec![],
            submitted_to: crate::egov::GovernmentAgency::MinistryOfEnvironment,
            notification_date: Some(Utc::now().date_naive()),
        };

        assert!(notification.meets_prior_notification());
    }

    #[test]
    fn test_waste_permit_validity() {
        let permit = WasteManagementPermit {
            permit_type: WastePermitType::Collection,
            permit_number: "WASTE-001".to_string(),
            operator_name: "Test Waste Management".to_string(),
            waste_types: vec![WasteType::Industrial],
            processing_capacity_tons_per_day: 50.0,
            issue_date: Utc::now().date_naive() - chrono::Duration::days(30),
            expiration_date: Utc::now().date_naive() + chrono::Duration::days(365 * 5),
            facility_standards_met: true,
        };

        assert!(permit.is_valid());
        assert!(permit.has_correct_validity_period());
    }

    #[test]
    fn test_expired_permit() {
        let permit = WasteManagementPermit {
            permit_type: WastePermitType::Collection,
            permit_number: "WASTE-002".to_string(),
            operator_name: "Test Waste Management".to_string(),
            waste_types: vec![WasteType::Municipal],
            processing_capacity_tons_per_day: 30.0,
            issue_date: Utc::now().date_naive() - chrono::Duration::days(365 * 6),
            expiration_date: Utc::now().date_naive() - chrono::Duration::days(30),
            facility_standards_met: true,
        };

        assert!(!permit.is_valid());
    }

    #[test]
    fn test_waste_manifest_completion() {
        let mut manifest = WasteManifest {
            manifest_number: "MF-001".to_string(),
            waste_type: WasteType::Industrial,
            quantity_kg: 1000.0,
            generator: Party {
                name: "Generator Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: Some("03-1234-5678".to_string()),
            },
            transporter: Party {
                name: "Transporter Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            processor: Party {
                name: "Processor Co.".to_string(),
                address: "Tokyo".to_string(),
                contact: None,
            },
            issue_date: Utc::now().date_naive(),
            completion_date: None,
        };

        assert!(!manifest.is_complete());

        manifest.completion_date = Some(Utc::now().date_naive());
        assert!(manifest.is_complete());
    }
}
