//! Indonesian Construction Services Law - UU No. 2/2017
//!
//! ## Overview
//!
//! Law No. 2 of 2017 on Construction Services regulates:
//! - Construction service providers
//! - Construction work contracts
//! - Construction standards and certification
//! - Building permits (IMB - Izin Mendirikan Bangunan)
//!
//! ## Regulatory Authority
//!
//! - **Kementerian PUPR**: Ministry of Public Works and Housing
//! - **LPJK**: Construction Services Development Board (Lembaga Pengembangan Jasa Konstruksi)
//!
//! ## Key Principles
//!
//! - Safety, health, environment (K3 - Keselamatan dan Kesehatan Kerja)
//! - Quality assurance
//! - Professional certification

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of construction work
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionWorkType {
    /// Building construction (gedung)
    BuildingConstruction,
    /// Civil engineering (teknik sipil) - roads, bridges, dams
    CivilEngineering,
    /// Mechanical and electrical installation (mekanikal dan elektrikal)
    MechanicalElectricalInstallation,
    /// Integrated construction (konstruksi terintegrasi)
    IntegratedConstruction,
    /// Special construction (konstruksi khusus)
    SpecialConstruction,
}

impl ConstructionWorkType {
    /// Get work type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::BuildingConstruction => "Konstruksi Bangunan Gedung",
            Self::CivilEngineering => "Konstruksi Bangunan Sipil",
            Self::MechanicalElectricalInstallation => "Instalasi Mekanikal dan Elektrikal",
            Self::IntegratedConstruction => "Konstruksi Terintegrasi",
            Self::SpecialConstruction => "Konstruksi Khusus",
        }
    }

    /// Get work type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::BuildingConstruction => "Building Construction",
            Self::CivilEngineering => "Civil Engineering",
            Self::MechanicalElectricalInstallation => "Mechanical and Electrical Installation",
            Self::IntegratedConstruction => "Integrated Construction",
            Self::SpecialConstruction => "Special Construction",
        }
    }
}

/// Construction service provider type - Pasal 9
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceProviderType {
    /// Planner (perencana) - consultant for design and planning
    Planner,
    /// Implementer/Contractor (pelaksana) - construction contractor
    Contractor,
    /// Supervisor (pengawas) - construction supervision consultant
    Supervisor,
}

impl ServiceProviderType {
    /// Get provider type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Planner => "Perencana Konstruksi",
            Self::Contractor => "Pelaksana Konstruksi (Kontraktor)",
            Self::Supervisor => "Pengawas Konstruksi",
        }
    }

    /// Get provider type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Planner => "Construction Planner",
            Self::Contractor => "Construction Contractor",
            Self::Supervisor => "Construction Supervisor",
        }
    }
}

/// Business entity classification - Pasal 11
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessEntityClass {
    /// Small business (usaha kecil) - K1, K2, K3
    SmallBusiness { sub_class: SmallBusinessSubClass },
    /// Medium business (usaha menengah) - M1, M2
    MediumBusiness { sub_class: MediumBusinessSubClass },
    /// Large business (usaha besar) - B1, B2
    LargeBusiness { sub_class: LargeBusinessSubClass },
}

/// Small business sub-classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmallBusinessSubClass {
    /// K1 - up to Rp 1 billion
    K1,
    /// K2 - Rp 1-2.5 billion
    K2,
    /// K3 - Rp 2.5-5 billion
    K3,
}

/// Medium business sub-classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediumBusinessSubClass {
    /// M1 - Rp 5-10 billion
    M1,
    /// M2 - Rp 10-30 billion
    M2,
}

/// Large business sub-classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LargeBusinessSubClass {
    /// B1 - Rp 30-100 billion
    B1,
    /// B2 - over Rp 100 billion
    B2,
}

impl BusinessEntityClass {
    /// Get maximum contract value in Rupiah
    pub fn max_contract_value(&self) -> Option<i64> {
        match self {
            Self::SmallBusiness { sub_class } => match sub_class {
                SmallBusinessSubClass::K1 => Some(1_000_000_000),
                SmallBusinessSubClass::K2 => Some(2_500_000_000),
                SmallBusinessSubClass::K3 => Some(5_000_000_000),
            },
            Self::MediumBusiness { sub_class } => match sub_class {
                MediumBusinessSubClass::M1 => Some(10_000_000_000),
                MediumBusinessSubClass::M2 => Some(30_000_000_000),
            },
            Self::LargeBusiness { sub_class } => match sub_class {
                LargeBusinessSubClass::B1 => Some(100_000_000_000),
                LargeBusinessSubClass::B2 => None, // Unlimited
            },
        }
    }

    /// Get class name
    pub fn class_name(&self) -> String {
        match self {
            Self::SmallBusiness { sub_class } => match sub_class {
                SmallBusinessSubClass::K1 => "K1".to_string(),
                SmallBusinessSubClass::K2 => "K2".to_string(),
                SmallBusinessSubClass::K3 => "K3".to_string(),
            },
            Self::MediumBusiness { sub_class } => match sub_class {
                MediumBusinessSubClass::M1 => "M1".to_string(),
                MediumBusinessSubClass::M2 => "M2".to_string(),
            },
            Self::LargeBusiness { sub_class } => match sub_class {
                LargeBusinessSubClass::B1 => "B1".to_string(),
                LargeBusinessSubClass::B2 => "B2".to_string(),
            },
        }
    }
}

/// Construction work contract type - Pasal 20-22
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Lump sum - fixed price
    LumpSum,
    /// Unit price - price per unit of work
    UnitPrice,
    /// Cost plus fee - actual cost + fee
    CostPlusFee,
    /// Percentage - fee as percentage of work value
    Percentage,
    /// Turnkey - design-build
    Turnkey,
    /// Build-Operate-Transfer (BOT)
    BuildOperateTransfer,
}

impl ContractType {
    /// Get contract type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::LumpSum => "Lump Sum (Harga Pasti)",
            Self::UnitPrice => "Harga Satuan",
            Self::CostPlusFee => "Biaya Plus Fee",
            Self::Percentage => "Persentase",
            Self::Turnkey => "Turnkey (Penyerahan Kunci)",
            Self::BuildOperateTransfer => "Bangun Guna Serah (BOT)",
        }
    }

    /// Get contract type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::LumpSum => "Lump Sum",
            Self::UnitPrice => "Unit Price",
            Self::CostPlusFee => "Cost Plus Fee",
            Self::Percentage => "Percentage",
            Self::Turnkey => "Turnkey",
            Self::BuildOperateTransfer => "Build-Operate-Transfer (BOT)",
        }
    }
}

/// Construction contract record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionContract {
    /// Contract number
    pub contract_number: String,
    /// Project name
    pub project_name: String,
    /// Owner/client name
    pub owner_name: String,
    /// Contractor name
    pub contractor_name: String,
    /// Consultant name (if any)
    pub consultant_name: Option<String>,
    /// Work type
    pub work_type: ConstructionWorkType,
    /// Contract type
    pub contract_type: ContractType,
    /// Contract value (Rupiah)
    pub contract_value: i64,
    /// Contract signing date
    pub signing_date: NaiveDate,
    /// Work commencement date
    pub commencement_date: NaiveDate,
    /// Planned completion date
    pub completion_date: NaiveDate,
    /// Contract duration in days
    pub duration_days: u32,
    /// Performance guarantee percentage (usually 5%)
    pub performance_guarantee_percent: f64,
    /// Advance payment percentage (if any, max 20%)
    pub advance_payment_percent: Option<f64>,
    /// Retention money percentage (usually 5-10%)
    pub retention_percent: f64,
}

impl ConstructionContract {
    /// Calculate performance guarantee amount
    pub fn performance_guarantee_amount(&self) -> i64 {
        (self.contract_value as f64 * self.performance_guarantee_percent / 100.0).round() as i64
    }

    /// Calculate advance payment amount
    pub fn advance_payment_amount(&self) -> Option<i64> {
        self.advance_payment_percent
            .map(|percent| (self.contract_value as f64 * percent / 100.0).round() as i64)
    }

    /// Calculate retention amount
    pub fn retention_amount(&self) -> i64 {
        (self.contract_value as f64 * self.retention_percent / 100.0).round() as i64
    }

    /// Check if advance payment is within limit (max 20%)
    pub fn is_valid_advance_payment(&self) -> bool {
        match self.advance_payment_percent {
            Some(percent) => percent <= 20.0,
            None => true,
        }
    }
}

/// Construction certification type - Pasal 69-75
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificationType {
    /// Business entity certificate (Sertifikat Badan Usaha - SBU)
    BusinessEntityCertificate,
    /// Competency certificate for workers (Sertifikat Kompetensi Kerja - SKK)
    WorkerCompetencyCertificate,
    /// Professional certificate (Sertifikat Keahlian/Keterampilan)
    ProfessionalCertificate,
}

impl CertificationType {
    /// Get certificate type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::BusinessEntityCertificate => "Sertifikat Badan Usaha (SBU)",
            Self::WorkerCompetencyCertificate => "Sertifikat Kompetensi Kerja (SKK)",
            Self::ProfessionalCertificate => "Sertifikat Keahlian/Keterampilan",
        }
    }

    /// Get certificate type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::BusinessEntityCertificate => "Business Entity Certificate",
            Self::WorkerCompetencyCertificate => "Worker Competency Certificate",
            Self::ProfessionalCertificate => "Professional Certificate",
        }
    }
}

/// Occupational safety and health (K3) requirements - Pasal 51-55
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K3Requirements {
    /// Whether K3 plan is in place
    pub has_k3_plan: bool,
    /// Whether safety officer (Ahli K3) is appointed
    pub has_safety_officer: bool,
    /// Whether workers have K3 training
    pub workers_have_training: bool,
    /// Whether personal protective equipment (APD) is provided
    pub has_ppe: bool,
    /// Whether emergency response procedures are established
    pub has_emergency_procedures: bool,
}

impl K3Requirements {
    /// Check if K3 requirements are met
    pub fn is_compliant(&self) -> bool {
        self.has_k3_plan
            && self.has_safety_officer
            && self.workers_have_training
            && self.has_ppe
            && self.has_emergency_procedures
    }

    /// Get non-compliant items
    pub fn non_compliant_items(&self) -> Vec<&str> {
        let mut items = Vec::new();
        if !self.has_k3_plan {
            items.push("K3 Plan not in place");
        }
        if !self.has_safety_officer {
            items.push("Safety Officer not appointed");
        }
        if !self.workers_have_training {
            items.push("Workers lack K3 training");
        }
        if !self.has_ppe {
            items.push("Personal Protective Equipment not provided");
        }
        if !self.has_emergency_procedures {
            items.push("Emergency Response Procedures not established");
        }
        items
    }
}

/// Building function classification - for IMB (Izin Mendirikan Bangunan)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingFunction {
    /// Residential (hunian)
    Residential,
    /// Commercial (usaha)
    Commercial,
    /// Industrial (industri)
    Industrial,
    /// Office (perkantoran)
    Office,
    /// Educational (pendidikan)
    Educational,
    /// Healthcare (kesehatan)
    Healthcare,
    /// Religious (peribadatan)
    Religious,
    /// Mixed use (campuran)
    MixedUse,
    /// Special function (khusus)
    SpecialFunction,
}

impl BuildingFunction {
    /// Get function name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Residential => "Hunian",
            Self::Commercial => "Usaha",
            Self::Industrial => "Industri",
            Self::Office => "Perkantoran",
            Self::Educational => "Pendidikan",
            Self::Healthcare => "Kesehatan",
            Self::Religious => "Peribadatan",
            Self::MixedUse => "Campuran",
            Self::SpecialFunction => "Fungsi Khusus",
        }
    }

    /// Get function name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Residential => "Residential",
            Self::Commercial => "Commercial",
            Self::Industrial => "Industrial",
            Self::Office => "Office",
            Self::Educational => "Educational",
            Self::Healthcare => "Healthcare",
            Self::Religious => "Religious",
            Self::MixedUse => "Mixed Use",
            Self::SpecialFunction => "Special Function",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_work_type() {
        let building = ConstructionWorkType::BuildingConstruction;
        assert_eq!(building.name_id(), "Konstruksi Bangunan Gedung");
        assert_eq!(building.name_en(), "Building Construction");
    }

    #[test]
    fn test_business_entity_class() {
        let k1 = BusinessEntityClass::SmallBusiness {
            sub_class: SmallBusinessSubClass::K1,
        };
        assert_eq!(k1.max_contract_value(), Some(1_000_000_000));
        assert_eq!(k1.class_name(), "K1");

        let b2 = BusinessEntityClass::LargeBusiness {
            sub_class: LargeBusinessSubClass::B2,
        };
        assert_eq!(b2.max_contract_value(), None); // Unlimited
        assert_eq!(b2.class_name(), "B2");
    }

    #[test]
    fn test_contract_calculations() {
        let contract = ConstructionContract {
            contract_number: "CON001".to_string(),
            project_name: "Test Project".to_string(),
            owner_name: "Test Owner".to_string(),
            contractor_name: "Test Contractor".to_string(),
            consultant_name: None,
            work_type: ConstructionWorkType::BuildingConstruction,
            contract_type: ContractType::LumpSum,
            contract_value: 1_000_000_000,
            signing_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            commencement_date: NaiveDate::from_ymd_opt(2024, 2, 1).expect("Valid date"),
            completion_date: NaiveDate::from_ymd_opt(2024, 12, 31).expect("Valid date"),
            duration_days: 365,
            performance_guarantee_percent: 5.0,
            advance_payment_percent: Some(20.0),
            retention_percent: 5.0,
        };

        assert_eq!(contract.performance_guarantee_amount(), 50_000_000);
        assert_eq!(contract.advance_payment_amount(), Some(200_000_000));
        assert_eq!(contract.retention_amount(), 50_000_000);
        assert!(contract.is_valid_advance_payment());
    }

    #[test]
    fn test_invalid_advance_payment() {
        let contract = ConstructionContract {
            contract_number: "CON002".to_string(),
            project_name: "Test Project 2".to_string(),
            owner_name: "Test Owner".to_string(),
            contractor_name: "Test Contractor".to_string(),
            consultant_name: None,
            work_type: ConstructionWorkType::BuildingConstruction,
            contract_type: ContractType::LumpSum,
            contract_value: 1_000_000_000,
            signing_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid date"),
            commencement_date: NaiveDate::from_ymd_opt(2024, 2, 1).expect("Valid date"),
            completion_date: NaiveDate::from_ymd_opt(2024, 12, 31).expect("Valid date"),
            duration_days: 365,
            performance_guarantee_percent: 5.0,
            advance_payment_percent: Some(30.0), // Over 20% limit
            retention_percent: 5.0,
        };

        assert!(!contract.is_valid_advance_payment());
    }

    #[test]
    fn test_k3_requirements() {
        let compliant = K3Requirements {
            has_k3_plan: true,
            has_safety_officer: true,
            workers_have_training: true,
            has_ppe: true,
            has_emergency_procedures: true,
        };
        assert!(compliant.is_compliant());
        assert!(compliant.non_compliant_items().is_empty());

        let non_compliant = K3Requirements {
            has_k3_plan: true,
            has_safety_officer: false,
            workers_have_training: false,
            has_ppe: true,
            has_emergency_procedures: true,
        };
        assert!(!non_compliant.is_compliant());
        assert_eq!(non_compliant.non_compliant_items().len(), 2);
    }

    #[test]
    fn test_service_provider_type() {
        let contractor = ServiceProviderType::Contractor;
        assert_eq!(contractor.name_id(), "Pelaksana Konstruksi (Kontraktor)");
        assert_eq!(contractor.name_en(), "Construction Contractor");
    }
}
