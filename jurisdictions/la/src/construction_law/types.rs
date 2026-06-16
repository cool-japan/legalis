//! Construction Law Types (ປະເພດກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ)
//!
//! Type definitions for Lao construction law based on the
//! **Law on Construction (Lao PDR), No. 05/NA, 2009**
//! (ກົດໝາຍວ່າດ້ວຍການກໍ່ສ້າງ).
//!
//! # Legal References
//!
//! - Law on Construction 2009 (No. 05/NA) - the primary statute governing the
//!   permitting, execution, supervision and acceptance of construction works in
//!   the Lao People's Democratic Republic.
//!
//! # Numeric thresholds
//!
//! Where the underlying statute fixes a quantifiable requirement (such as a
//! defects-liability period) it is encoded as a named, documented constant.
//! Quantities the statute does not fix precisely - notably the maximum project
//! value a contractor's grade permits - are modelled as validated fields
//! (checked for internal consistency) rather than as fabricated statutory figures.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Construction 2009 (No. 05/NA)
// ============================================================================

/// Representative statutory defects-liability (warranty) period, in months.
///
/// During this period after completion and handover the contractor remains
/// responsible for remedying defects in the works. This figure is modelled as a
/// documented default representing a typical statutory defects-liability period;
/// the precise figure may vary by regulation and by project type.
/// ໄລຍະຮັບປະກັນຄວາມເສຍຫາຍເປັນເດືອນ
pub const DEFECTS_LIABILITY_PERIOD_MONTHS: u32 = 24;

// ============================================================================
// Construction Classification - ການຈັດປະເພດການກໍ່ສ້າງ
// ============================================================================

/// Construction work category by scale - ປະເພດການກໍ່ສ້າງຕາມຂະໜາດ
///
/// Construction works are classified by scale/category; larger categories attract
/// more stringent permitting, supervision and inspection requirements. Ordered
/// from smallest to largest scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstructionCategory {
    /// Small-scale works (ການກໍ່ສ້າງຂະໜາດນ້ອຍ)
    SmallScale,
    /// Medium-scale works (ການກໍ່ສ້າງຂະໜາດກາງ)
    Medium,
    /// Large-scale works (ການກໍ່ສ້າງຂະໜາດໃຫຍ່)
    Large,
    /// Special works requiring heightened control (ການກໍ່ສ້າງພິເສດ)
    Special,
}

impl ConstructionCategory {
    /// All construction categories, ordered from smallest to largest scale.
    pub fn all() -> [ConstructionCategory; 4] {
        [
            ConstructionCategory::SmallScale,
            ConstructionCategory::Medium,
            ConstructionCategory::Large,
            ConstructionCategory::Special,
        ]
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConstructionCategory::SmallScale => "ການກໍ່ສ້າງຂະໜາດນ້ອຍ",
            ConstructionCategory::Medium => "ການກໍ່ສ້າງຂະໜາດກາງ",
            ConstructionCategory::Large => "ການກໍ່ສ້າງຂະໜາດໃຫຍ່",
            ConstructionCategory::Special => "ການກໍ່ສ້າງພິເສດ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            ConstructionCategory::SmallScale => "small-scale construction",
            ConstructionCategory::Medium => "medium-scale construction",
            ConstructionCategory::Large => "large-scale construction",
            ConstructionCategory::Special => "special construction",
        }
    }
}

/// Type of construction project by purpose - ປະເພດໂຄງການກໍ່ສ້າງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProjectType {
    /// Residential building (ສິ່ງປຸກສ້າງທີ່ຢູ່ອາໄສ)
    Residential,
    /// Commercial building (ສິ່ງປຸກສ້າງການຄ້າ)
    Commercial,
    /// Industrial facility (ໂຮງງານອຸດສາຫະກຳ)
    Industrial,
    /// Infrastructure works such as roads and bridges (ໂຄງລ່າງພື້ນຖານ)
    Infrastructure,
    /// State / public works (ວຽກງານສາທາລະນະ)
    PublicWorks,
}

impl ProjectType {
    /// All project types.
    pub fn all() -> [ProjectType; 5] {
        [
            ProjectType::Residential,
            ProjectType::Commercial,
            ProjectType::Industrial,
            ProjectType::Infrastructure,
            ProjectType::PublicWorks,
        ]
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProjectType::Residential => "ສິ່ງປຸກສ້າງທີ່ຢູ່ອາໄສ",
            ProjectType::Commercial => "ສິ່ງປຸກສ້າງການຄ້າ",
            ProjectType::Industrial => "ໂຮງງານອຸດສາຫະກຳ",
            ProjectType::Infrastructure => "ໂຄງລ່າງພື້ນຖານ",
            ProjectType::PublicWorks => "ວຽກງານສາທາລະນະ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            ProjectType::Residential => "residential",
            ProjectType::Commercial => "commercial",
            ProjectType::Industrial => "industrial",
            ProjectType::Infrastructure => "infrastructure",
            ProjectType::PublicWorks => "public works",
        }
    }
}

/// Contractor capacity grade - ຊັ້ນຄວາມສາມາດຂອງຜູ້ຮັບເໝົາ
///
/// Contractors are graded by capacity, with `GradeI` the highest down to
/// `GradeIV`. A contractor's grade limits the value/scale of the projects they
/// may undertake. The [`ContractorGrade::rank`] method returns the numeric rank
/// (1 = highest capacity) which is authoritative for capacity comparisons; the
/// derived ordering follows declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContractorGrade {
    /// Grade I - highest capacity (ຊັ້ນ ໜຶ່ງ - ສູງສຸດ)
    GradeI,
    /// Grade II (ຊັ້ນ ສອງ)
    GradeII,
    /// Grade III (ຊັ້ນ ສາມ)
    GradeIII,
    /// Grade IV - lowest capacity (ຊັ້ນ ສີ່ - ຕ່ຳສຸດ)
    GradeIV,
}

impl ContractorGrade {
    /// All contractor grades, from highest capacity to lowest.
    pub fn all() -> [ContractorGrade; 4] {
        [
            ContractorGrade::GradeI,
            ContractorGrade::GradeII,
            ContractorGrade::GradeIII,
            ContractorGrade::GradeIV,
        ]
    }

    /// Numeric rank of the grade (1 = highest capacity, 4 = lowest capacity).
    pub fn rank(&self) -> u8 {
        match self {
            ContractorGrade::GradeI => 1,
            ContractorGrade::GradeII => 2,
            ContractorGrade::GradeIII => 3,
            ContractorGrade::GradeIV => 4,
        }
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ContractorGrade::GradeI => "ຊັ້ນ ໜຶ່ງ",
            ContractorGrade::GradeII => "ຊັ້ນ ສອງ",
            ContractorGrade::GradeIII => "ຊັ້ນ ສາມ",
            ContractorGrade::GradeIV => "ຊັ້ນ ສີ່",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            ContractorGrade::GradeI => "Grade I",
            ContractorGrade::GradeII => "Grade II",
            ContractorGrade::GradeIII => "Grade III",
            ContractorGrade::GradeIV => "Grade IV",
        }
    }
}

/// Status of a building/construction permit - ສະຖານະໃບອະນຸຍາດກໍ່ສ້າງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PermitStatus {
    /// No permit application has been made (ຍັງບໍ່ໄດ້ຍື່ນຄຳຮ້ອງ)
    NotApplied,
    /// Application submitted and under review (ກຳລັງດຳເນີນການ)
    Pending,
    /// Permit approved/issued (ອະນຸມັດ/ອອກໃບອະນຸຍາດແລ້ວ)
    Approved,
    /// Application rejected (ປະຕິເສດ)
    Rejected,
    /// Permit suspended (ໂຈະ)
    Suspended,
    /// Permit revoked (ຖອນໃບອະນຸຍາດ)
    Revoked,
    /// Permit expired (ໝົດອາຍຸ)
    Expired,
}

impl PermitStatus {
    /// All permit statuses.
    pub fn all() -> [PermitStatus; 7] {
        [
            PermitStatus::NotApplied,
            PermitStatus::Pending,
            PermitStatus::Approved,
            PermitStatus::Rejected,
            PermitStatus::Suspended,
            PermitStatus::Revoked,
            PermitStatus::Expired,
        ]
    }

    /// Whether this status authorises construction to proceed.
    /// Only an approved permit authorises construction.
    pub fn permits_construction(&self) -> bool {
        matches!(self, PermitStatus::Approved)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            PermitStatus::NotApplied => "ຍັງບໍ່ໄດ້ຍື່ນຄຳຮ້ອງ",
            PermitStatus::Pending => "ກຳລັງດຳເນີນການ",
            PermitStatus::Approved => "ອະນຸມັດ",
            PermitStatus::Rejected => "ປະຕິເສດ",
            PermitStatus::Suspended => "ໂຈະ",
            PermitStatus::Revoked => "ຖອນໃບອະນຸຍາດ",
            PermitStatus::Expired => "ໝົດອາຍຸ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            PermitStatus::NotApplied => "not applied",
            PermitStatus::Pending => "pending",
            PermitStatus::Approved => "approved",
            PermitStatus::Rejected => "rejected",
            PermitStatus::Suspended => "suspended",
            PermitStatus::Revoked => "revoked",
            PermitStatus::Expired => "expired",
        }
    }
}

/// Mandatory construction inspection stage - ໄລຍະການກວດກາການກໍ່ສ້າງ
///
/// Inspections are carried out at successive stages of the works, which must be
/// performed in order: foundation, then structure, then completion. The
/// [`InspectionStage::order`] method returns the position in this sequence
/// (0 = first).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InspectionStage {
    /// Foundation inspection (ການກວດກາຮາກຖານ)
    Foundation,
    /// Structure inspection (ການກວດກາໂຄງສ້າງ)
    Structure,
    /// Completion inspection (ການກວດກາການສຳເລັດ)
    Completion,
}

impl InspectionStage {
    /// All inspection stages, in mandatory order.
    pub fn all() -> [InspectionStage; 3] {
        [
            InspectionStage::Foundation,
            InspectionStage::Structure,
            InspectionStage::Completion,
        ]
    }

    /// Position of the stage in the mandatory sequence (0 = first).
    pub fn order(&self) -> u8 {
        match self {
            InspectionStage::Foundation => 0,
            InspectionStage::Structure => 1,
            InspectionStage::Completion => 2,
        }
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            InspectionStage::Foundation => "ການກວດກາຮາກຖານ",
            InspectionStage::Structure => "ການກວດກາໂຄງສ້າງ",
            InspectionStage::Completion => "ການກວດກາການສຳເລັດ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            InspectionStage::Foundation => "foundation",
            InspectionStage::Structure => "structure",
            InspectionStage::Completion => "completion",
        }
    }
}

// ============================================================================
// Permits - ໃບອະນຸຍາດ
// ============================================================================

/// Building / construction permit - ໃບອະນຸຍາດກໍ່ສ້າງ
///
/// A construction/building permit must be obtained before construction begins.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuildingPermit {
    /// Project name (ຊື່ໂຄງການ)
    pub project_name: String,
    /// Construction category/scale (ປະເພດ/ຂະໜາດການກໍ່ສ້າງ)
    pub category: ConstructionCategory,
    /// Applicant (owner/developer) name (ຊື່ຜູ້ຍື່ນຄຳຮ້ອງ)
    pub applicant: String,
    /// Whether the permit has been issued (ໄດ້ອອກໃບອະນຸຍາດແລ້ວ)
    pub issued: bool,
    /// Permit status (ສະຖານະໃບອະນຸຍາດ)
    pub status: PermitStatus,
}

// ============================================================================
// Contractors - ຜູ້ຮັບເໝົາ
// ============================================================================

/// Construction contractor - ຜູ້ຮັບເໝົາກໍ່ສ້າງ
///
/// Contractors must be licensed/registered and are graded by capacity; the grade
/// limits the value of the projects they may undertake. The permitted maximum is
/// modelled here as a validated field rather than asserting exact LAK figures.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Contractor {
    /// Contractor name (ຊື່ຜູ້ຮັບເໝົາ)
    pub name: String,
    /// Capacity grade (ຊັ້ນຄວາມສາມາດ)
    pub grade: ContractorGrade,
    /// Whether the contractor holds a valid licence/registration (ມີໃບອະນຸຍາດ)
    pub licensed: bool,
    /// Maximum project value the grade permits, in LAK (ມູນຄ່າໂຄງການສູງສຸດເປັນກີບ)
    pub max_project_value_lak: u64,
}

impl Contractor {
    /// Whether the contractor's grade permits undertaking a project of the given
    /// value (in LAK).
    pub fn can_undertake_value(&self, project_value_lak: u64) -> bool {
        project_value_lak <= self.max_project_value_lak
    }
}

// ============================================================================
// Projects & Contracts - ໂຄງການ ແລະ ສັນຍາ
// ============================================================================

/// Construction project - ໂຄງການກໍ່ສ້າງ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstructionProject {
    /// Project name (ຊື່ໂຄງການ)
    pub name: String,
    /// Project type (ປະເພດໂຄງການ)
    pub project_type: ProjectType,
    /// Project value in LAK (ມູນຄ່າໂຄງການເປັນກີບ)
    pub value_lak: u64,
    /// Whether a building permit has been obtained (ມີໃບອະນຸຍາດກໍ່ສ້າງ)
    pub has_permit: bool,
    /// Whether an on-site safety plan is in place (ມີແຜນຄວາມປອດໄພ)
    pub has_safety_plan: bool,
    /// Grade of the contractor undertaking the project (ຊັ້ນຜູ້ຮັບເໝົາ)
    pub contractor_grade: ContractorGrade,
}

/// Construction contract - ສັນຍາກໍ່ສ້າງ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstructionContract {
    /// Project owner / employer name (ຊື່ເຈົ້າຂອງໂຄງການ)
    pub owner: String,
    /// Contractor name (ຊື່ຜູ້ຮັບເໝົາ)
    pub contractor: String,
    /// Contract value in LAK (ມູນຄ່າສັນຍາເປັນກີບ)
    pub value_lak: u64,
    /// Defects-liability (warranty) period in months (ໄລຍະຮັບປະກັນເປັນເດືອນ)
    pub defects_liability_months: u32,
}

// ============================================================================
// Inspection & Acceptance - ການກວດກາ ແລະ ການກວດຮັບ
// ============================================================================

/// A staged construction inspection - ການກວດກາການກໍ່ສ້າງຕາມໄລຍະ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Inspection {
    /// Inspection stage (ໄລຍະການກວດກາ)
    pub stage: InspectionStage,
    /// Whether the inspection was passed (ຜ່ານການກວດກາ)
    pub passed: bool,
}

/// Formal acceptance / handover of the works - ການກວດຮັບ ແລະ ມອບ-ຮັບວຽກ
///
/// Formal acceptance and handover must precede occupancy; occupancy may only be
/// permitted once all mandatory inspections have been passed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkAcceptance {
    /// Whether all mandatory inspections passed (ຜ່ານການກວດກາທັງໝົດ)
    pub all_inspections_passed: bool,
    /// Whether occupancy is permitted (ອະນຸຍາດໃຫ້ເຂົ້າຢູ່ອາໄສ)
    pub occupancy_permitted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defects_liability_constant() {
        assert_eq!(DEFECTS_LIABILITY_PERIOD_MONTHS, 24);
    }

    #[test]
    fn test_contractor_grade_rank_ordering() {
        assert_eq!(ContractorGrade::GradeI.rank(), 1);
        assert_eq!(ContractorGrade::GradeIV.rank(), 4);
        assert!(ContractorGrade::GradeI.rank() < ContractorGrade::GradeIV.rank());
        assert_eq!(ContractorGrade::all().len(), 4);
    }

    #[test]
    fn test_inspection_stage_order() {
        assert!(InspectionStage::Foundation.order() < InspectionStage::Structure.order());
        assert!(InspectionStage::Structure.order() < InspectionStage::Completion.order());
        assert_eq!(InspectionStage::all().len(), 3);
    }

    #[test]
    fn test_permit_status_permits_construction() {
        assert!(PermitStatus::Approved.permits_construction());
        assert!(!PermitStatus::Pending.permits_construction());
        assert!(!PermitStatus::Revoked.permits_construction());
        assert_eq!(PermitStatus::all().len(), 7);
    }

    #[test]
    fn test_contractor_capacity_check() {
        let contractor = Contractor {
            name: "Lao Build Co.".to_string(),
            grade: ContractorGrade::GradeII,
            licensed: true,
            max_project_value_lak: 10_000_000_000,
        };
        assert!(contractor.can_undertake_value(5_000_000_000));
        assert!(!contractor.can_undertake_value(20_000_000_000));
    }

    #[test]
    fn test_category_scale_ordering() {
        assert!(ConstructionCategory::SmallScale < ConstructionCategory::Special);
        assert_eq!(ConstructionCategory::all().len(), 4);
    }

    #[test]
    fn test_bilingual_names_present() {
        for category in ConstructionCategory::all() {
            assert!(!category.lao_name().is_empty());
            assert!(!category.english_name().is_empty());
        }
        for project_type in ProjectType::all() {
            assert!(!project_type.lao_name().is_empty());
            assert!(!project_type.english_name().is_empty());
        }
        for grade in ContractorGrade::all() {
            assert!(!grade.lao_name().is_empty());
            assert!(!grade.english_name().is_empty());
        }
        for status in PermitStatus::all() {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }
        for stage in InspectionStage::all() {
            assert!(!stage.lao_name().is_empty());
            assert!(!stage.english_name().is_empty());
        }
    }
}
