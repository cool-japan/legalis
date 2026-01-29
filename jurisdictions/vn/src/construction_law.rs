//! Vietnamese Construction Law 2014 (Luật Xây dựng 2014) - Law No. 50/2014/QH13
//!
//! Vietnam's law on construction activities, effective from January 1, 2015.
//! Amended by Law 35/2018 and Law 40/2019.
//!
//! ## Key Requirements
//!
//! - **Construction permits** (Giấy phép xây dựng): Required for most constructions
//! - **Fire safety** (Phòng cháy chữa cháy): Mandatory for all buildings
//! - **Quality management** (Quản lý chất lượng): Three-level quality control
//! - **Construction inspection** (Thanh tra xây dựng): State inspection required
//! - **Warranty period** (Thời hạn bảo hành): Minimum warranty requirements

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Construction classification (Phân loại công trình) - Article 4
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionClass {
    /// Class I - Particularly important (Công trình đặc biệt quan trọng)
    ClassI,
    /// Class II - Important (Công trình quan trọng)
    ClassII,
    /// Class III - Normal (Công trình thường)
    ClassIII,
    /// Class IV - Simple (Công trình đơn giản)
    ClassIV,
}

impl ConstructionClass {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::ClassI => "Cấp I - Công trình đặc biệt quan trọng",
            Self::ClassII => "Cấp II - Công trình quan trọng",
            Self::ClassIII => "Cấp III - Công trình thường",
            Self::ClassIV => "Cấp IV - Công trình đơn giản",
        }
    }

    /// Get English description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::ClassI => "Class I - Particularly important works",
            Self::ClassII => "Class II - Important works",
            Self::ClassIII => "Class III - Normal works",
            Self::ClassIV => "Class IV - Simple works",
        }
    }

    /// Check if requires construction permit
    pub fn requires_permit(&self) -> bool {
        matches!(self, Self::ClassI | Self::ClassII | Self::ClassIII)
    }

    /// Check if requires independent quality inspection
    pub fn requires_independent_inspection(&self) -> bool {
        matches!(self, Self::ClassI | Self::ClassII)
    }
}

/// Construction activities (Hoạt động xây dựng) - Article 7
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionActivity {
    /// Investment preparation (Chuẩn bị đầu tư xây dựng công trình)
    InvestmentPreparation,
    /// Survey (Khảo sát xây dựng)
    Survey,
    /// Design (Thiết kế xây dựng)
    Design,
    /// Construction (Thi công xây dựng)
    Construction,
    /// Installation (Lắp đặt)
    Installation,
    /// Supervision (Giám sát thi công xây dựng)
    Supervision,
    /// Inspection and acceptance (Nghiệm thu, bàn giao công trình)
    InspectionAcceptance,
    /// Maintenance (Bảo trì, bảo dưỡng công trình)
    Maintenance,
}

impl ConstructionActivity {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::InvestmentPreparation => "Chuẩn bị đầu tư xây dựng",
            Self::Survey => "Khảo sát xây dựng",
            Self::Design => "Thiết kế xây dựng",
            Self::Construction => "Thi công xây dựng",
            Self::Installation => "Lắp đặt thiết bị",
            Self::Supervision => "Giám sát thi công",
            Self::InspectionAcceptance => "Nghiệm thu công trình",
            Self::Maintenance => "Bảo trì, bảo dưỡng",
        }
    }

    /// Check if requires practicing certificate
    pub fn requires_practicing_certificate(&self) -> bool {
        matches!(self, Self::Survey | Self::Design | Self::Supervision)
    }
}

/// Construction permit types (Loại giấy phép xây dựng) - Article 91-96
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionPermitType {
    /// New construction (Xây dựng mới)
    NewConstruction,
    /// Renovation and repair (Sửa chữa, cải tạo)
    RenovationRepair,
    /// Expansion (Mở rộng)
    Expansion,
    /// Temporary construction (Xây dựng tạm)
    Temporary,
}

impl ConstructionPermitType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::NewConstruction => "Giấy phép xây dựng mới",
            Self::RenovationRepair => "Giấy phép sửa chữa, cải tạo",
            Self::Expansion => "Giấy phép mở rộng",
            Self::Temporary => "Giấy phép xây dựng tạm",
        }
    }

    /// Get validity period in months
    pub fn validity_period_months(&self) -> Option<u16> {
        match self {
            Self::NewConstruction => Some(36),  // 3 years
            Self::RenovationRepair => Some(24), // 2 years
            Self::Expansion => Some(24),        // 2 years
            Self::Temporary => Some(12),        // 1 year
        }
    }
}

/// Quality control levels (Kiểm soát chất lượng) - Article 118-122
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualityControlLevel {
    /// Self-inspection by contractor (Tự kiểm tra của nhà thầu)
    ContractorSelfInspection,
    /// Supervision by consultant (Giám sát của tư vấn giám sát)
    ConsultantSupervision,
    /// Independent inspection (Kiểm tra độc lập)
    IndependentInspection,
}

impl QualityControlLevel {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::ContractorSelfInspection => "Kiểm tra chất lượng của nhà thầu thi công",
            Self::ConsultantSupervision => "Giám sát chất lượng của tư vấn giám sát",
            Self::IndependentInspection => "Kiểm tra chất lượng độc lập",
        }
    }
}

/// Construction warranty periods (Thời hạn bảo hành) - Article 139
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarrantyPeriod;

impl WarrantyPeriod {
    /// Minimum warranty for structural components (years)
    pub const STRUCTURAL: u8 = 24; // 24 months minimum

    /// Minimum warranty for mechanical/electrical systems (years)
    pub const MECHANICAL_ELECTRICAL: u8 = 12; // 12 months minimum

    /// Minimum warranty for other components (years)
    pub const OTHER_COMPONENTS: u8 = 6; // 6 months minimum

    /// Get minimum warranty period for component type
    pub fn minimum_months(component_type: ComponentType) -> u8 {
        match component_type {
            ComponentType::Structural => Self::STRUCTURAL,
            ComponentType::MechanicalElectrical => Self::MECHANICAL_ELECTRICAL,
            ComponentType::Other => Self::OTHER_COMPONENTS,
        }
    }
}

/// Construction component types for warranty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// Structural components (Kết cấu chính)
    Structural,
    /// Mechanical and electrical systems (Hệ thống M&E)
    MechanicalElectrical,
    /// Other components (Bộ phận khác)
    Other,
}

/// Fire safety classification (Phân loại cháy) - Article 107-109
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireSafetyRank {
    /// Very high fire risk (Nguy hiểm cháy rất cao)
    VeryHigh,
    /// High fire risk (Nguy hiểm cháy cao)
    High,
    /// Medium fire risk (Nguy hiểm cháy trung bình)
    Medium,
    /// Low fire risk (Nguy hiểm cháy thấp)
    Low,
}

impl FireSafetyRank {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::VeryHigh => "Nguy hiểm cháy rất cao",
            Self::High => "Nguy hiểm cháy cao",
            Self::Medium => "Nguy hiểm cháy trung bình",
            Self::Low => "Nguy hiểm cháy thấp",
        }
    }

    /// Check if requires automatic fire suppression system
    pub fn requires_automatic_suppression(&self) -> bool {
        matches!(self, Self::VeryHigh | Self::High)
    }
}

/// Construction compliance check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionCompliance {
    /// Construction classification
    pub construction_class: ConstructionClass,
    /// Has construction permit
    pub has_permit: bool,
    /// Has approved design documents
    pub has_approved_design: bool,
    /// Has qualified contractor
    pub has_qualified_contractor: bool,
    /// Has supervision consultant
    pub has_supervision: bool,
    /// Has fire safety approval
    pub has_fire_safety_approval: bool,
}

impl ConstructionCompliance {
    /// Check if compliant with Construction Law
    pub fn is_compliant(&self) -> bool {
        // Check permit requirement
        if self.construction_class.requires_permit() && !self.has_permit {
            return false;
        }

        // Must have approved design
        if !self.has_approved_design {
            return false;
        }

        // Must have qualified contractor
        if !self.has_qualified_contractor {
            return false;
        }

        // Class I and II require supervision
        if self.construction_class.requires_independent_inspection() && !self.has_supervision {
            return false;
        }

        // Must have fire safety approval
        if !self.has_fire_safety_approval {
            return false;
        }

        true
    }

    /// Get list of compliance violations
    pub fn get_violations(&self) -> Vec<String> {
        let mut violations = Vec::new();

        if self.construction_class.requires_permit() && !self.has_permit {
            violations.push("Chưa có giấy phép xây dựng (Điều 91)".to_string());
        }

        if !self.has_approved_design {
            violations.push("Thiết kế chưa được phê duyệt (Điều 78)".to_string());
        }

        if !self.has_qualified_contractor {
            violations.push("Nhà thầu không đủ năng lực (Điều 148)".to_string());
        }

        if self.construction_class.requires_independent_inspection() && !self.has_supervision {
            violations.push("Chưa có giám sát thi công (Điều 131)".to_string());
        }

        if !self.has_fire_safety_approval {
            violations.push("Chưa có thẩm duyệt phòng cháy chữa cháy (Điều 107)".to_string());
        }

        violations
    }
}

/// Result type for construction law operations
pub type ConstructionResult<T> = Result<T, ConstructionError>;

/// Errors related to Construction Law
#[derive(Debug, Error)]
pub enum ConstructionError {
    /// Missing construction permit
    #[error("Chưa có giấy phép xây dựng (Điều 91): {reason}")]
    MissingPermit { reason: String },

    /// Unqualified contractor
    #[error("Nhà thầu không đủ năng lực (Điều 148): {reason}")]
    UnqualifiedContractor { reason: String },

    /// Construction quality violation
    #[error("Vi phạm chất lượng công trình (Điều 118): {reason}")]
    QualityViolation { reason: String },

    /// Fire safety violation
    #[error("Vi phạm phòng cháy chữa cháy (Điều 107): {reason}")]
    FireSafetyViolation { reason: String },

    /// Warranty violation
    #[error("Vi phạm quy định bảo hành (Điều 139): {reason}")]
    WarrantyViolation { reason: String },

    /// Other construction law violation
    #[error("Vi phạm Luật Xây dựng: {reason}")]
    ConstructionViolation { reason: String },
}

/// Validate construction compliance
pub fn validate_construction_compliance(
    compliance: &ConstructionCompliance,
) -> ConstructionResult<()> {
    if !compliance.is_compliant() {
        let violations = compliance.get_violations();
        return Err(ConstructionError::ConstructionViolation {
            reason: violations.join("; "),
        });
    }
    Ok(())
}

/// Validate warranty period
pub fn validate_warranty_period(
    component_type: ComponentType,
    warranty_months: u8,
) -> ConstructionResult<()> {
    let minimum = WarrantyPeriod::minimum_months(component_type);

    if warranty_months < minimum {
        Err(ConstructionError::WarrantyViolation {
            reason: format!(
                "Thời hạn bảo hành {} tháng không đủ tối thiểu {} tháng",
                warranty_months, minimum
            ),
        })
    } else {
        Ok(())
    }
}

/// Get Construction Law checklist
pub fn get_construction_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Giấy phép xây dựng", "Construction permit", "Điều 91-96"),
        ("Thẩm định thiết kế", "Design appraisal", "Điều 78-86"),
        (
            "Năng lực nhà thầu",
            "Contractor qualification",
            "Điều 148-150",
        ),
        (
            "Giám sát thi công",
            "Construction supervision",
            "Điều 131-135",
        ),
        ("Phòng cháy chữa cháy", "Fire safety", "Điều 107-109"),
        (
            "Nghiệm thu công trình",
            "Construction acceptance",
            "Điều 136-138",
        ),
        (
            "Bảo hành công trình",
            "Construction warranty",
            "Điều 139-141",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_classification() {
        assert!(ConstructionClass::ClassI.requires_permit());
        assert!(ConstructionClass::ClassI.requires_independent_inspection());

        assert!(ConstructionClass::ClassIII.requires_permit());
        assert!(!ConstructionClass::ClassIII.requires_independent_inspection());

        assert!(!ConstructionClass::ClassIV.requires_permit());
    }

    #[test]
    fn test_construction_activities() {
        assert!(ConstructionActivity::Survey.requires_practicing_certificate());
        assert!(ConstructionActivity::Design.requires_practicing_certificate());
        assert!(!ConstructionActivity::Construction.requires_practicing_certificate());
    }

    #[test]
    fn test_permit_validity() {
        assert_eq!(
            ConstructionPermitType::NewConstruction.validity_period_months(),
            Some(36)
        );
        assert_eq!(
            ConstructionPermitType::Temporary.validity_period_months(),
            Some(12)
        );
    }

    #[test]
    fn test_warranty_periods() {
        assert_eq!(WarrantyPeriod::STRUCTURAL, 24);
        assert_eq!(WarrantyPeriod::MECHANICAL_ELECTRICAL, 12);
        assert_eq!(WarrantyPeriod::OTHER_COMPONENTS, 6);

        assert_eq!(
            WarrantyPeriod::minimum_months(ComponentType::Structural),
            24
        );
    }

    #[test]
    fn test_warranty_validation() {
        // Valid warranty
        assert!(validate_warranty_period(ComponentType::Structural, 24).is_ok());
        assert!(validate_warranty_period(ComponentType::Structural, 36).is_ok());

        // Invalid warranty
        assert!(validate_warranty_period(ComponentType::Structural, 12).is_err());
        assert!(validate_warranty_period(ComponentType::MechanicalElectrical, 6).is_err());
    }

    #[test]
    fn test_fire_safety_rank() {
        assert!(FireSafetyRank::VeryHigh.requires_automatic_suppression());
        assert!(FireSafetyRank::High.requires_automatic_suppression());
        assert!(!FireSafetyRank::Medium.requires_automatic_suppression());
        assert!(!FireSafetyRank::Low.requires_automatic_suppression());
    }

    #[test]
    fn test_construction_compliance() {
        let compliant = ConstructionCompliance {
            construction_class: ConstructionClass::ClassII,
            has_permit: true,
            has_approved_design: true,
            has_qualified_contractor: true,
            has_supervision: true,
            has_fire_safety_approval: true,
        };

        assert!(compliant.is_compliant());
        assert!(compliant.get_violations().is_empty());

        let non_compliant = ConstructionCompliance {
            construction_class: ConstructionClass::ClassI,
            has_permit: false,
            has_approved_design: false,
            has_qualified_contractor: true,
            has_supervision: false,
            has_fire_safety_approval: true,
        };

        assert!(!non_compliant.is_compliant());
        assert_eq!(non_compliant.get_violations().len(), 3);
    }

    #[test]
    fn test_validation() {
        let compliant = ConstructionCompliance {
            construction_class: ConstructionClass::ClassIII,
            has_permit: true,
            has_approved_design: true,
            has_qualified_contractor: true,
            has_supervision: false, // Not required for Class III
            has_fire_safety_approval: true,
        };

        assert!(validate_construction_compliance(&compliant).is_ok());

        let non_compliant = ConstructionCompliance {
            construction_class: ConstructionClass::ClassI,
            has_permit: true,
            has_approved_design: true,
            has_qualified_contractor: true,
            has_supervision: false, // Required for Class I!
            has_fire_safety_approval: true,
        };

        assert!(validate_construction_compliance(&non_compliant).is_err());
    }

    #[test]
    fn test_construction_checklist() {
        let checklist = get_construction_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 7);
    }
}
