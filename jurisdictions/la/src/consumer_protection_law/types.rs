//! Consumer Protection Law Types (ປະເພດກົດໝາຍປົກປ້ອງຜູ້ບໍລິໂພກ)
//!
//! Type definitions for Lao consumer protection law based on the
//! **Law on Consumer Protection (Lao PDR), No. 02/NA, 30 June 2010**
//! (ກົດໝາຍວ່າດ້ວຍການປົກປ້ອງຜູ້ບໍລິໂພກ).
//!
//! # Legal References
//!
//! - Law on Consumer Protection 2010 (No. 02/NA) - the primary statute
//! - The recognised consumer rights track the internationally accepted set of
//!   consumer rights (the UN Guidelines for Consumer Protection), which the Lao
//!   law adopts as the foundation of consumer protection.
//!
//! # Numeric thresholds
//!
//! Where the underlying statute fixes a quantifiable requirement (such as the
//! mandatory use of the Lao language on labelling) it is encoded as a named,
//! documented constant. Quantities the statute does not fix precisely are modelled
//! as validated fields (checked for internal consistency) rather than as
//! fabricated statutory figures.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Consumer Protection 2010 (No. 02/NA)
// ============================================================================

/// Language that must appear on consumer product labelling sold in Lao PDR.
/// Lao labelling is mandatory under the consumer information / labelling rules.
/// ພາສາທີ່ຕ້ອງມີຢູ່ສະຫຼາກສິນຄ້າ
pub const REQUIRED_LABEL_LANGUAGE: &str = "Lao";

/// Number of internationally recognised fundamental consumer rights adopted as
/// the framework of the Lao consumer protection regime.
/// ຈຳນວນສິດຂັ້ນພື້ນຖານຂອງຜູ້ບໍລິໂພກ
pub const FUNDAMENTAL_CONSUMER_RIGHTS_COUNT: usize = 8;

// ============================================================================
// Consumer Rights - ສິດຂອງຜູ້ບໍລິໂພກ
// ============================================================================

/// Fundamental consumer right - ສິດຂັ້ນພື້ນຖານຂອງຜູ້ບໍລິໂພກ
///
/// The eight fundamental consumer rights forming the basis of the Lao consumer
/// protection regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConsumerRight {
    /// Right to safety (ສິດໄດ້ຮັບຄວາມປອດໄພ)
    Safety,
    /// Right to be informed (ສິດໄດ້ຮັບຂໍ້ມູນຂ່າວສານ)
    Information,
    /// Right to choose (ສິດເລືອກ)
    Choice,
    /// Right to be heard (ສິດສະແດງຄວາມຄິດເຫັນ)
    Representation,
    /// Right to redress (ສິດໄດ້ຮັບການແກ້ໄຂ/ຊົດເຊີຍ)
    Redress,
    /// Right to consumer education (ສິດໄດ້ຮັບການສຶກສາ)
    Education,
    /// Right to satisfaction of basic needs (ສິດໄດ້ຮັບການຕອບສະໜອງຄວາມຕ້ອງການພື້ນຖານ)
    BasicNeeds,
    /// Right to a healthy environment (ສິດໄດ້ຮັບສິ່ງແວດລ້ອມທີ່ດີ)
    HealthyEnvironment,
}

impl ConsumerRight {
    /// All eight fundamental consumer rights.
    pub fn all() -> [ConsumerRight; FUNDAMENTAL_CONSUMER_RIGHTS_COUNT] {
        [
            ConsumerRight::Safety,
            ConsumerRight::Information,
            ConsumerRight::Choice,
            ConsumerRight::Representation,
            ConsumerRight::Redress,
            ConsumerRight::Education,
            ConsumerRight::BasicNeeds,
            ConsumerRight::HealthyEnvironment,
        ]
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ConsumerRight::Safety => "ສິດໄດ້ຮັບຄວາມປອດໄພ",
            ConsumerRight::Information => "ສິດໄດ້ຮັບຂໍ້ມູນຂ່າວສານ",
            ConsumerRight::Choice => "ສິດເລືອກ",
            ConsumerRight::Representation => "ສິດສະແດງຄວາມຄິດເຫັນ",
            ConsumerRight::Redress => "ສິດໄດ້ຮັບການແກ້ໄຂ",
            ConsumerRight::Education => "ສິດໄດ້ຮັບການສຶກສາ",
            ConsumerRight::BasicNeeds => "ສິດໄດ້ຮັບການຕອບສະໜອງຄວາມຕ້ອງການພື້ນຖານ",
            ConsumerRight::HealthyEnvironment => "ສິດໄດ້ຮັບສິ່ງແວດລ້ອມທີ່ດີ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            ConsumerRight::Safety => "right to safety",
            ConsumerRight::Information => "right to be informed",
            ConsumerRight::Choice => "right to choose",
            ConsumerRight::Representation => "right to be heard",
            ConsumerRight::Redress => "right to redress",
            ConsumerRight::Education => "right to consumer education",
            ConsumerRight::BasicNeeds => "right to satisfaction of basic needs",
            ConsumerRight::HealthyEnvironment => "right to a healthy environment",
        }
    }
}

/// Supplier (business operator) obligation - ພັນທະຂອງຜູ້ສະໜອງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SupplierObligation {
    /// Provide accurate information about goods/services (ໃຫ້ຂໍ້ມູນທີ່ຖືກຕ້ອງ)
    AccurateInformation,
    /// Label products, including in the Lao language (ຕິດສະຫຼາກເປັນພາສາລາວ)
    LaoLanguageLabelling,
    /// Ensure product/service safety (ຮັບປະກັນຄວາມປອດໄພ)
    ProductSafety,
    /// Use fair and transparent contract terms (ໃຊ້ຂໍ້ກຳນົດສັນຍາທີ່ເປັນທຳ)
    FairContractTerms,
    /// Honour warranties and after-sales service (ຮັບປະກັນ ແລະ ບໍລິການຫຼັງການຂາຍ)
    WarrantyAndAfterSales,
    /// Provide redress for defective goods (ໃຫ້ການແກ້ໄຂສຳລັບສິນຄ້າບົກພ່ອງ)
    RedressForDefects,
    /// Refrain from deceptive or unfair practices (ບໍ່ກະທຳການຫຼອກລວງ)
    NoDeceptivePractices,
}

impl SupplierObligation {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            SupplierObligation::AccurateInformation => "ໃຫ້ຂໍ້ມູນທີ່ຖືກຕ້ອງ",
            SupplierObligation::LaoLanguageLabelling => "ຕິດສະຫຼາກເປັນພາສາລາວ",
            SupplierObligation::ProductSafety => "ຮັບປະກັນຄວາມປອດໄພ",
            SupplierObligation::FairContractTerms => "ໃຊ້ຂໍ້ກຳນົດສັນຍາທີ່ເປັນທຳ",
            SupplierObligation::WarrantyAndAfterSales => "ຮັບປະກັນ ແລະ ບໍລິການຫຼັງການຂາຍ",
            SupplierObligation::RedressForDefects => "ໃຫ້ການແກ້ໄຂສຳລັບສິນຄ້າບົກພ່ອງ",
            SupplierObligation::NoDeceptivePractices => "ບໍ່ກະທຳການຫຼອກລວງ",
        }
    }

    /// English label of the obligation.
    pub fn english_name(&self) -> &'static str {
        match self {
            SupplierObligation::AccurateInformation => "provide accurate information",
            SupplierObligation::LaoLanguageLabelling => "label products in the Lao language",
            SupplierObligation::ProductSafety => "ensure product safety",
            SupplierObligation::FairContractTerms => "use fair contract terms",
            SupplierObligation::WarrantyAndAfterSales => "honour warranty and after-sales service",
            SupplierObligation::RedressForDefects => "provide redress for defects",
            SupplierObligation::NoDeceptivePractices => "refrain from deceptive practices",
        }
    }
}

/// Prohibited business practice - ການກະທຳທີ່ຖືກຫ້າມ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProhibitedPractice {
    /// False or misleading advertising (ການໂຄສະນາທີ່ຕົວະ ຫຼື ເຮັດໃຫ້ເຂົ້າໃຈຜິດ)
    FalseAdvertising,
    /// Unfair contract terms (ຂໍ້ກຳນົດສັນຍາທີ່ບໍ່ເປັນທຳ)
    UnfairContractTerms,
    /// Sale of unsafe or defective goods (ການຂາຍສິນຄ້າທີ່ບໍ່ປອດໄພ/ບົກພ່ອງ)
    UnsafeGoods,
    /// Hoarding and price manipulation (ການກັກຕຸນ ແລະ ປັ່ນລາຄາ)
    HoardingAndPriceManipulation,
    /// Short measure / deceptive weighing (ການຊັ່ງຕວງບໍ່ຄົບ)
    ShortMeasure,
    /// Coerced or forced sales (ການບັງຄັບຊື້ຂາຍ)
    ForcedSales,
    /// Concealment of material defects (ການປິດບັງຂໍ້ບົກພ່ອງ)
    ConcealmentOfDefects,
}

impl ProhibitedPractice {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProhibitedPractice::FalseAdvertising => "ການໂຄສະນາທີ່ຕົວະ",
            ProhibitedPractice::UnfairContractTerms => "ຂໍ້ກຳນົດສັນຍາທີ່ບໍ່ເປັນທຳ",
            ProhibitedPractice::UnsafeGoods => "ການຂາຍສິນຄ້າທີ່ບໍ່ປອດໄພ",
            ProhibitedPractice::HoardingAndPriceManipulation => "ການກັກຕຸນ ແລະ ປັ່ນລາຄາ",
            ProhibitedPractice::ShortMeasure => "ການຊັ່ງຕວງບໍ່ຄົບ",
            ProhibitedPractice::ForcedSales => "ການບັງຄັບຊື້ຂາຍ",
            ProhibitedPractice::ConcealmentOfDefects => "ການປິດບັງຂໍ້ບົກພ່ອງ",
        }
    }

    /// English label of the practice.
    pub fn english_name(&self) -> &'static str {
        match self {
            ProhibitedPractice::FalseAdvertising => "false advertising",
            ProhibitedPractice::UnfairContractTerms => "unfair contract terms",
            ProhibitedPractice::UnsafeGoods => "unsafe goods",
            ProhibitedPractice::HoardingAndPriceManipulation => "hoarding and price manipulation",
            ProhibitedPractice::ShortMeasure => "short measure",
            ProhibitedPractice::ForcedSales => "forced sales",
            ProhibitedPractice::ConcealmentOfDefects => "concealment of defects",
        }
    }
}

// ============================================================================
// Product Labelling - ການຕິດສະຫຼາກສິນຄ້າ
// ============================================================================

/// Product label - ສະຫຼາກສິນຄ້າ
///
/// Models the information that must be communicated to consumers. Labelling in
/// the Lao language is mandatory; imported goods may also carry other languages.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProductLabel {
    /// Product name (ຊື່ສິນຄ້າ)
    pub product_name: String,
    /// Languages present on the label (ພາສາທີ່ມີຢູ່ສະຫຼາກ)
    pub languages: Vec<String>,
    /// Whether the manufacturer / importer is identified (ລະບຸຜູ້ຜະລິດ/ນຳເຂົ້າ)
    pub has_manufacturer_info: bool,
    /// Manufacture date in YYYY-MM-DD form, if applicable (ວັນທີຜະລິດ)
    pub manufacture_date: Option<String>,
    /// Expiry date in YYYY-MM-DD form, if applicable (ວັນໝົດອາຍຸ)
    pub expiry_date: Option<String>,
    /// Whether net quantity / weight is stated (ລະບຸປະລິມານ/ນ້ຳໜັກ)
    pub has_net_quantity: bool,
    /// Whether usage instructions are present (ມີຄຳແນະນຳການນຳໃຊ້)
    pub has_usage_instructions: bool,
    /// Whether safety warnings are present where required (ມີຄຳເຕືອນຄວາມປອດໄພ)
    pub has_safety_warnings: bool,
    /// Whether this product requires safety warnings (ສິນຄ້າຕ້ອງມີຄຳເຕືອນ)
    pub requires_safety_warnings: bool,
}

impl ProductLabel {
    /// Whether the label includes the mandatory Lao language.
    pub fn has_lao_language(&self) -> bool {
        self.languages
            .iter()
            .any(|lang| lang.eq_ignore_ascii_case(REQUIRED_LABEL_LANGUAGE))
    }
}

// ============================================================================
// Consumer Contracts - ສັນຍາຜູ້ບໍລິໂພກ
// ============================================================================

/// Type of potentially unfair contract term - ປະເພດຂໍ້ກຳນົດສັນຍາ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContractTermType {
    /// Term excluding all supplier liability (ຍົກເວັ້ນຄວາມຮັບຜິດຊອບທັງໝົດ)
    TotalLiabilityExclusion,
    /// Term allowing unilateral price change (ປ່ຽນລາຄາຝ່າຍດຽວ)
    UnilateralPriceChange,
    /// Term waiving the consumer's right to redress (ສະຫຼະສິດການແກ້ໄຂ)
    WaiverOfRedress,
    /// Term imposing a disproportionate penalty (ໂທດປັບທີ່ບໍ່ສົມເຫດສົມຜົນ)
    DisproportionatePenalty,
    /// A standard, fair term (ຂໍ້ກຳນົດປົກກະຕິທີ່ເປັນທຳ)
    Standard,
}

impl ContractTermType {
    /// Whether this term type is presumptively unfair to the consumer.
    pub fn is_unfair(&self) -> bool {
        !matches!(self, ContractTermType::Standard)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ContractTermType::TotalLiabilityExclusion => "ຍົກເວັ້ນຄວາມຮັບຜິດຊອບທັງໝົດ",
            ContractTermType::UnilateralPriceChange => "ປ່ຽນລາຄາຝ່າຍດຽວ",
            ContractTermType::WaiverOfRedress => "ສະຫຼະສິດການແກ້ໄຂ",
            ContractTermType::DisproportionatePenalty => "ໂທດປັບທີ່ບໍ່ສົມເຫດສົມຜົນ",
            ContractTermType::Standard => "ຂໍ້ກຳນົດປົກກະຕິ",
        }
    }
}

/// Consumer contract - ສັນຍາຜູ້ບໍລິໂພກ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsumerContract {
    /// Description of the goods/services (ລາຍລະອຽດສິນຄ້າ/ບໍລິການ)
    pub subject: String,
    /// Price in LAK (ລາຄາເປັນກີບ)
    pub price_lak: u64,
    /// Contract terms classified by type (ຂໍ້ກຳນົດສັນຍາ)
    pub terms: Vec<ContractTermType>,
    /// Whether the contract is written in or available in Lao (ມີສະບັບພາສາລາວ)
    pub available_in_lao: bool,
}

// ============================================================================
// Product Safety & Recalls - ຄວາມປອດໄພສິນຄ້າ ແລະ ການເກັບຄືນ
// ============================================================================

/// Product hazard severity - ລະດັບຄວາມຮ້າຍແຮງຂອງໄພອັນຕະລາຍ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HazardSeverity {
    /// No identified hazard (ບໍ່ມີໄພອັນຕະລາຍ)
    None,
    /// Low hazard (ໄພອັນຕະລາຍຕ່ຳ)
    Low,
    /// Moderate hazard (ໄພອັນຕະລາຍປານກາງ)
    Moderate,
    /// Serious hazard - risk of injury (ໄພອັນຕະລາຍຮ້າຍແຮງ)
    Serious,
    /// Critical hazard - risk to life (ໄພອັນຕະລາຍວິກິດ)
    Critical,
}

impl HazardSeverity {
    /// Whether a product with this hazard level must be recalled from the market.
    pub fn requires_recall(&self) -> bool {
        matches!(self, HazardSeverity::Serious | HazardSeverity::Critical)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            HazardSeverity::None => "ບໍ່ມີໄພອັນຕະລາຍ",
            HazardSeverity::Low => "ໄພອັນຕະລາຍຕ່ຳ",
            HazardSeverity::Moderate => "ໄພອັນຕະລາຍປານກາງ",
            HazardSeverity::Serious => "ໄພອັນຕະລາຍຮ້າຍແຮງ",
            HazardSeverity::Critical => "ໄພອັນຕະລາຍວິກິດ",
        }
    }
}

/// Product safety assessment - ການປະເມີນຄວາມປອດໄພສິນຄ້າ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProductSafetyAssessment {
    /// Product name (ຊື່ສິນຄ້າ)
    pub product_name: String,
    /// Identified hazard severity (ລະດັບໄພອັນຕະລາຍ)
    pub hazard_severity: HazardSeverity,
    /// Whether the product complies with applicable safety standards (ໄດ້ມາດຕະຖານ)
    pub meets_safety_standard: bool,
    /// Whether the product has been recalled (ໄດ້ເກັບຄືນແລ້ວ)
    pub recalled: bool,
}

/// Product recall - ການເກັບຄືນສິນຄ້າ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProductRecall {
    /// Product name (ຊື່ສິນຄ້າ)
    pub product_name: String,
    /// Hazard severity prompting the recall (ລະດັບໄພອັນຕະລາຍ)
    pub hazard_severity: HazardSeverity,
    /// Whether consumers have been publicly notified (ໄດ້ແຈ້ງຜູ້ບໍລິໂພກ)
    pub consumers_notified: bool,
    /// Remedy offered to affected consumers (ການແກ້ໄຂທີ່ສະເໜີ)
    pub remedy: RedressType,
}

// ============================================================================
// Complaints, Redress & Dispute Resolution - ຄຳຮ້ອງທຸກ, ການແກ້ໄຂ ແລະ ການແກ້ໄຂຂໍ້ຂັດແຍ່ງ
// ============================================================================

/// Type of redress / remedy - ປະເພດການແກ້ໄຂ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RedressType {
    /// Repair of the defective product (ສ້ອມແປງ)
    Repair,
    /// Replacement with conforming goods (ປ່ຽນສິນຄ້າໃໝ່)
    Replacement,
    /// Refund of the purchase price (ສົ່ງຄືນເງິນ)
    Refund,
    /// Monetary compensation for damages (ຊົດເຊີຍຄ່າເສຍຫາຍ)
    Compensation,
    /// No remedy offered (ບໍ່ມີການແກ້ໄຂ)
    None,
}

impl RedressType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            RedressType::Repair => "ສ້ອມແປງ",
            RedressType::Replacement => "ປ່ຽນສິນຄ້າໃໝ່",
            RedressType::Refund => "ສົ່ງຄືນເງິນ",
            RedressType::Compensation => "ຊົດເຊີຍຄ່າເສຍຫາຍ",
            RedressType::None => "ບໍ່ມີການແກ້ໄຂ",
        }
    }
}

/// Dispute resolution method - ວິທີການແກ້ໄຂຂໍ້ຂັດແຍ່ງ
///
/// Ordered from least to most formal; the Lao consumer protection regime
/// encourages amicable settlement before litigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisputeResolutionMethod {
    /// Direct negotiation between consumer and supplier (ການເຈລະຈາ)
    Negotiation,
    /// Mediation by the consumer protection authority (ການໄກ່ເກ່ຍ)
    Mediation,
    /// Administrative complaint to the competent authority (ຄຳຮ້ອງທາງບໍລິຫານ)
    AdministrativeComplaint,
    /// Litigation before the People's Court (ການຟ້ອງຮ້ອງຕໍ່ສານ)
    Litigation,
}

impl DisputeResolutionMethod {
    /// Escalation order (0 = first / least formal).
    pub fn escalation_order(&self) -> u8 {
        match self {
            DisputeResolutionMethod::Negotiation => 0,
            DisputeResolutionMethod::Mediation => 1,
            DisputeResolutionMethod::AdministrativeComplaint => 2,
            DisputeResolutionMethod::Litigation => 3,
        }
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            DisputeResolutionMethod::Negotiation => "ການເຈລະຈາ",
            DisputeResolutionMethod::Mediation => "ການໄກ່ເກ່ຍ",
            DisputeResolutionMethod::AdministrativeComplaint => "ຄຳຮ້ອງທາງບໍລິຫານ",
            DisputeResolutionMethod::Litigation => "ການຟ້ອງຮ້ອງຕໍ່ສານ",
        }
    }
}

/// Consumer complaint status - ສະຖານະຄຳຮ້ອງທຸກ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComplaintStatus {
    /// Received (ໄດ້ຮັບແລ້ວ)
    Received,
    /// Under review (ກຳລັງພິຈາລະນາ)
    UnderReview,
    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,
    /// Rejected (ປະຕິເສດ)
    Rejected,
    /// Escalated to a higher method (ຍົກລະດັບ)
    Escalated,
}

impl ComplaintStatus {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ComplaintStatus::Received => "ໄດ້ຮັບແລ້ວ",
            ComplaintStatus::UnderReview => "ກຳລັງພິຈາລະນາ",
            ComplaintStatus::Resolved => "ແກ້ໄຂແລ້ວ",
            ComplaintStatus::Rejected => "ປະຕິເສດ",
            ComplaintStatus::Escalated => "ຍົກລະດັບ",
        }
    }
}

/// Consumer complaint - ຄຳຮ້ອງທຸກຂອງຜູ້ບໍລິໂພກ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsumerComplaint {
    /// Consumer name (ຊື່ຜູ້ບໍລິໂພກ)
    pub consumer_name: String,
    /// Supplier name (ຊື່ຜູ້ສະໜອງ)
    pub supplier_name: String,
    /// Description of the grievance in Lao (ລາຍລະອຽດເປັນພາສາລາວ)
    pub description_lao: String,
    /// Description of the grievance in English (ລາຍລະອຽດເປັນພາສາອັງກິດ)
    pub description_en: String,
    /// Consumer right alleged to be infringed (ສິດທີ່ຖືກລະເມີດ)
    pub right_invoked: ConsumerRight,
    /// Amount of claimed loss in LAK (ມູນຄ່າຄວາມເສຍຫາຍ)
    pub claimed_loss_lak: u64,
    /// Resolution method being pursued (ວິທີການແກ້ໄຂ)
    pub resolution_method: DisputeResolutionMethod,
    /// Requested remedy (ການແກ້ໄຂທີ່ຮ້ອງຂໍ)
    pub requested_remedy: RedressType,
    /// Status (ສະຖານະ)
    pub status: ComplaintStatus,
}

/// Redress offered/awarded to a consumer - ການແກ້ໄຂທີ່ໃຫ້ຜູ້ບໍລິໂພກ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Redress {
    /// Remedy type (ປະເພດການແກ້ໄຂ)
    pub redress_type: RedressType,
    /// Original purchase price in LAK (ລາຄາຊື້ເດີມ)
    pub purchase_price_lak: u64,
    /// Monetary amount of the remedy in LAK (ມູນຄ່າການແກ້ໄຂ)
    pub amount_lak: u64,
}

// ============================================================================
// Administrative Sanctions - ການລົງໂທດທາງບໍລິຫານ
// ============================================================================

/// Administrative sanction against a supplier - ການລົງໂທດຜູ້ສະໜອງ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SanctionType {
    /// Formal warning / education (ການເຕືອນ/ສຶກສາອົບຮົມ)
    Warning,
    /// Monetary fine (ປັບໃໝ)
    Fine,
    /// Order to suspend the practice or product (ສັ່ງໂຈະ)
    Suspension,
    /// Revocation of business licence (ຖອນໃບອະນຸຍາດ)
    LicenceRevocation,
    /// Referral for criminal prosecution (ສົ່ງດຳເນີນຄະດີອາຍາ)
    CriminalReferral,
}

impl SanctionType {
    /// Sanction severity (0 = least severe).
    pub fn severity(&self) -> u8 {
        match self {
            SanctionType::Warning => 0,
            SanctionType::Fine => 1,
            SanctionType::Suspension => 2,
            SanctionType::LicenceRevocation => 3,
            SanctionType::CriminalReferral => 4,
        }
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            SanctionType::Warning => "ການເຕືອນ",
            SanctionType::Fine => "ປັບໃໝ",
            SanctionType::Suspension => "ສັ່ງໂຈະ",
            SanctionType::LicenceRevocation => "ຖອນໃບອະນຸຍາດ",
            SanctionType::CriminalReferral => "ສົ່ງດຳເນີນຄະດີອາຍາ",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_rights_count() {
        assert_eq!(
            ConsumerRight::all().len(),
            FUNDAMENTAL_CONSUMER_RIGHTS_COUNT
        );
        assert_eq!(FUNDAMENTAL_CONSUMER_RIGHTS_COUNT, 8);
    }

    #[test]
    fn test_lao_language_detection() {
        let label = ProductLabel {
            product_name: "Fish sauce".to_string(),
            languages: vec!["Lao".to_string(), "English".to_string()],
            has_manufacturer_info: true,
            manufacture_date: Some("2025-01-01".to_string()),
            expiry_date: Some("2026-01-01".to_string()),
            has_net_quantity: true,
            has_usage_instructions: true,
            has_safety_warnings: false,
            requires_safety_warnings: false,
        };
        assert!(label.has_lao_language());
    }

    #[test]
    fn test_unfair_term_classification() {
        assert!(ContractTermType::WaiverOfRedress.is_unfair());
        assert!(!ContractTermType::Standard.is_unfair());
    }

    #[test]
    fn test_hazard_recall_trigger() {
        assert!(HazardSeverity::Critical.requires_recall());
        assert!(HazardSeverity::Serious.requires_recall());
        assert!(!HazardSeverity::Low.requires_recall());
    }

    #[test]
    fn test_dispute_escalation_ordering() {
        assert!(
            DisputeResolutionMethod::Negotiation.escalation_order()
                < DisputeResolutionMethod::Litigation.escalation_order()
        );
    }

    #[test]
    fn test_sanction_severity_ordering() {
        assert!(SanctionType::Warning.severity() < SanctionType::LicenceRevocation.severity());
    }

    #[test]
    fn test_bilingual_names_present() {
        for right in ConsumerRight::all() {
            assert!(!right.lao_name().is_empty());
            assert!(!right.english_name().is_empty());
        }
    }
}
