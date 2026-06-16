//! Telecommunications Law Types (ປະເພດກົດໝາຍໂທລະຄົມມະນາຄົມ)
//!
//! Type definitions for Lao telecommunications law based on the
//! **Law on Telecommunications (Lao PDR), No. 09/NA, 2011**
//! (ກົດໝາຍວ່າດ້ວຍໂທລະຄົມມະນາຄົມ).
//!
//! # Legal References
//!
//! - Law on Telecommunications 2011 (No. 09/NA) - the primary statute
//! - The regime is administered through the ministry responsible for posts and
//!   telecommunications and its telecommunications regulatory authority.
//!
//! # Numeric thresholds
//!
//! Where a quantifiable requirement is modelled it is encoded as a named,
//! documented constant. Several thresholds are not fixed by a verifiable article
//! and are therefore documented as representative modelling defaults; physical
//! quantities (such as the extent of the radio-frequency spectrum) are encoded as
//! documented constants and validated for internal consistency rather than as
//! fabricated statutory figures.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Telecommunications 2011 (No. 09/NA)
// ============================================================================

/// Lower edge of the radio-frequency spectrum, in kilohertz (kHz).
///
/// The usable radio spectrum is conventionally taken to span from approximately
/// 9 kHz up to 300 GHz. This value is the lower bound used when validating that a
/// spectrum band lies within a representable range.
/// ຂອບເຂດຕ່ຳສຸດຂອງຄື້ນຄວາມຖີ່ວິທະຍຸ (kHz)
pub const SPECTRUM_MIN_KHZ: u64 = 9;

/// Upper edge of the radio-frequency spectrum, in gigahertz (GHz).
///
/// See [`SPECTRUM_MIN_KHZ`]; the usable radio spectrum extends up to ~300 GHz.
/// ຂອບເຂດສູງສຸດຂອງຄື້ນຄວາມຖີ່ວິທະຍຸ (GHz)
pub const SPECTRUM_MAX_GHZ: u32 = 300;

/// Upper edge of the radio-frequency spectrum expressed in megahertz (MHz),
/// derived from [`SPECTRUM_MAX_GHZ`] (300 GHz = 300,000 MHz).
///
/// Spectrum band fields ([`SpectrumAssignment`]) are expressed in MHz; a band
/// must fall at or below this upper bound to be representable.
/// ຂອບເຂດສູງສຸດຂອງຄື້ນຄວາມຖີ່ (MHz)
pub const SPECTRUM_MAX_MHZ: u32 = SPECTRUM_MAX_GHZ * 1_000;

/// Representative maximum term, in years, of a telecommunications licence.
///
/// Licence terms are set administratively; this value is documented as a
/// representative maximum (a modelling default) used to bound licence validity.
/// ໄລຍະເວລາສູງສຸດຂອງໃບອະນຸຍາດ (ປີ)
pub const LICENSE_VALIDITY_YEARS: u32 = 20;

/// Representative minimum service-availability target, as a percentage.
///
/// Quality-of-service targets are set by the regulator; this value is documented
/// as a representative target (a modelling default).
/// ເປົ້າໝາຍຄວາມພ້ອມໃຫ້ບໍລິການຕ່ຳສຸດ (%)
pub const MIN_SERVICE_AVAILABILITY_PERCENT: u32 = 99;

/// Representative maximum tolerated call-drop rate, in per-mille (parts per
/// thousand).
///
/// Documented as a representative quality-of-service target (a modelling
/// default).
/// ອັດຕາການຫຼຸດສາຍສູງສຸດທີ່ຍອມຮັບໄດ້ (ຕໍ່ພັນ)
pub const MAX_CALL_DROP_RATE_PERMILLE: u32 = 20;

// ============================================================================
// Service & Licence Classification - ການຈັດປະເພດການບໍລິການ ແລະ ໃບອະນຸຍາດ
// ============================================================================

/// Category of telecommunications service - ປະເພດການບໍລິການໂທລະຄົມມະນາຄົມ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ServiceType {
    /// Fixed-line telephony (ໂທລະສັບປະຈຳທີ່)
    FixedLine,
    /// Mobile / cellular service (ໂທລະສັບເຄື່ອນທີ່)
    Mobile,
    /// Internet access service (ບໍລິການອິນເຕີເນັດ)
    Internet,
    /// Satellite communications (ການສື່ສານຜ່ານດາວທຽມ)
    Satellite,
    /// Leased-line / dedicated circuit (ສາຍເຊົ່າ)
    LeasedLine,
    /// Data services (ບໍລິການຂໍ້ມູນ)
    DataServices,
    /// Broadcast transmission service (ການແຜ່ກະຈາຍສັນຍານ)
    BroadcastTransmission,
}

impl ServiceType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ServiceType::FixedLine => "ໂທລະສັບປະຈຳທີ່",
            ServiceType::Mobile => "ໂທລະສັບເຄື່ອນທີ່",
            ServiceType::Internet => "ບໍລິການອິນເຕີເນັດ",
            ServiceType::Satellite => "ການສື່ສານຜ່ານດາວທຽມ",
            ServiceType::LeasedLine => "ສາຍເຊົ່າ",
            ServiceType::DataServices => "ບໍລິການຂໍ້ມູນ",
            ServiceType::BroadcastTransmission => "ການແຜ່ກະຈາຍສັນຍານ",
        }
    }

    /// English label of the service.
    pub fn english_name(&self) -> &'static str {
        match self {
            ServiceType::FixedLine => "fixed-line telephony",
            ServiceType::Mobile => "mobile service",
            ServiceType::Internet => "internet access",
            ServiceType::Satellite => "satellite communications",
            ServiceType::LeasedLine => "leased line",
            ServiceType::DataServices => "data services",
            ServiceType::BroadcastTransmission => "broadcast transmission",
        }
    }
}

/// Telecommunications licence category - ປະເພດໃບອະນຸຍາດໂທລະຄົມມະນາຄົມ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LicenseCategory {
    /// Network facilities — physical infrastructure (ສິ່ງອຳນວຍຄວາມສະດວກໂຄງຂ່າຍ)
    NetworkFacilities,
    /// Network services — carriage over facilities (ບໍລິການໂຄງຂ່າຍ)
    NetworkServices,
    /// Application services — services delivered to end users (ບໍລິການແອັບພລິເຄຊັນ)
    ApplicationServices,
    /// Spectrum / radio-frequency licence (ໃບອະນຸຍາດຄື້ນຄວາມຖີ່)
    Spectrum,
}

impl LicenseCategory {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            LicenseCategory::NetworkFacilities => "ສິ່ງອຳນວຍຄວາມສະດວກໂຄງຂ່າຍ",
            LicenseCategory::NetworkServices => "ບໍລິການໂຄງຂ່າຍ",
            LicenseCategory::ApplicationServices => "ບໍລິການແອັບພລິເຄຊັນ",
            LicenseCategory::Spectrum => "ໃບອະນຸຍາດຄື້ນຄວາມຖີ່",
        }
    }

    /// English label of the licence category.
    pub fn english_name(&self) -> &'static str {
        match self {
            LicenseCategory::NetworkFacilities => "network facilities",
            LicenseCategory::NetworkServices => "network services",
            LicenseCategory::ApplicationServices => "application services",
            LicenseCategory::Spectrum => "spectrum / radio frequency",
        }
    }
}

/// Status of a telecommunications licence - ສະຖານະຂອງໃບອະນຸຍາດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LicenseStatus {
    /// Application pending decision (ກຳລັງລໍຖ້າພິຈາລະນາ)
    Pending,
    /// Active and in force (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Temporarily suspended (ຖືກໂຈະຊົ່ວຄາວ)
    Suspended,
    /// Revoked by the regulator (ຖືກຖອນ)
    Revoked,
    /// Expired at the end of its term (ໝົດອາຍຸ)
    Expired,
}

impl LicenseStatus {
    /// Whether a licence in this status currently permits the operator to
    /// provide telecommunications services.
    pub fn permits_operation(&self) -> bool {
        matches!(self, LicenseStatus::Active)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            LicenseStatus::Pending => "ກຳລັງລໍຖ້າພິຈາລະນາ",
            LicenseStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            LicenseStatus::Suspended => "ຖືກໂຈະຊົ່ວຄາວ",
            LicenseStatus::Revoked => "ຖືກຖອນ",
            LicenseStatus::Expired => "ໝົດອາຍຸ",
        }
    }

    /// English label of the status.
    pub fn english_name(&self) -> &'static str {
        match self {
            LicenseStatus::Pending => "pending",
            LicenseStatus::Active => "active",
            LicenseStatus::Suspended => "suspended",
            LicenseStatus::Revoked => "revoked",
            LicenseStatus::Expired => "expired",
        }
    }
}

/// Classification of a telecommunications operator - ປະເພດຜູ້ປະກອບການໂທລະຄົມມະນາຄົມ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OperatorType {
    /// Facilities-based operator owning network infrastructure (ຜູ້ປະກອບການທີ່ມີໂຄງຂ່າຍຂອງຕົນ)
    FacilitiesBased,
    /// Service-based operator using leased facilities (ຜູ້ໃຫ້ບໍລິການຜ່ານໂຄງຂ່າຍເຊົ່າ)
    ServiceBased,
    /// Reseller of another operator's services (ຜູ້ຂາຍຕໍ່ການບໍລິການ)
    Reseller,
    /// Virtual operator, e.g. a mobile virtual network operator (ຜູ້ປະກອບການເສມືອນ)
    VirtualOperator,
}

impl OperatorType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            OperatorType::FacilitiesBased => "ຜູ້ປະກອບການທີ່ມີໂຄງຂ່າຍຂອງຕົນ",
            OperatorType::ServiceBased => "ຜູ້ໃຫ້ບໍລິການຜ່ານໂຄງຂ່າຍເຊົ່າ",
            OperatorType::Reseller => "ຜູ້ຂາຍຕໍ່ການບໍລິການ",
            OperatorType::VirtualOperator => "ຜູ້ປະກອບການເສມືອນ",
        }
    }

    /// English label of the operator type.
    pub fn english_name(&self) -> &'static str {
        match self {
            OperatorType::FacilitiesBased => "facilities-based operator",
            OperatorType::ServiceBased => "service-based operator",
            OperatorType::Reseller => "reseller",
            OperatorType::VirtualOperator => "virtual operator",
        }
    }
}

// ============================================================================
// Licensing - ການອອກໃບອະນຸຍາດ
// ============================================================================

/// Telecommunications licence held by an operator - ໃບອະນຸຍາດໂທລະຄົມມະນາຄົມ
///
/// An operator must hold a licence to provide telecommunications services. The
/// licence has a category, a validity term and a current status.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TelecomLicense {
    /// Name of the licensed operator (ຊື່ຜູ້ປະກອບການ)
    pub operator: String,
    /// Licence category (ປະເພດໃບອະນຸຍາດ)
    pub category: LicenseCategory,
    /// Whether the licence has been granted by the regulator (ໄດ້ຮັບການອະນຸຍາດ)
    pub granted: bool,
    /// Validity term of the licence, in years (ໄລຍະເວລາໃບອະນຸຍາດ, ປີ)
    pub validity_years: u32,
    /// Year in which the licence term begins (ປີເລີ່ມຕົ້ນ)
    pub start_year: u32,
    /// Current licence status (ສະຖານະປັດຈຸບັນ)
    pub status: LicenseStatus,
}

impl TelecomLicense {
    /// Final year of the licence term (`start_year + validity_years`).
    /// ປີສິ້ນສຸດຂອງໃບອະນຸຍາດ
    pub fn expiry_year(&self) -> u32 {
        self.start_year.saturating_add(self.validity_years)
    }

    /// Whether the licence currently permits the operator to provide service.
    /// ໃບອະນຸຍາດອະນຸຍາດໃຫ້ບໍລິການຢູ່ບໍ່
    pub fn permits_operation(&self) -> bool {
        self.granted && self.status.permits_operation()
    }
}

// ============================================================================
// Radio-frequency Spectrum - ຄື້ນຄວາມຖີ່ວິທະຍຸ
// ============================================================================

/// Assignment of a radio-frequency band to an operator - ການມອບຄື້ນຄວາມຖີ່
///
/// Radio-frequency spectrum is a scarce national resource assigned in
/// non-overlapping bands. The band edges are expressed in megahertz (MHz).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpectrumAssignment {
    /// Operator to which the band is assigned (ຜູ້ປະກອບການທີ່ໄດ້ຮັບການມອບ)
    pub operator: String,
    /// Lower edge of the assigned band, in MHz (ຂອບເຂດຕ່ຳ, MHz)
    pub band_start_mhz: u32,
    /// Upper edge of the assigned band, in MHz (ຂອບເຂດສູງ, MHz)
    pub band_end_mhz: u32,
    /// Whether the assignment is exclusive to this operator (ມອບແບບຜູກຂາດ)
    pub exclusive: bool,
}

impl SpectrumAssignment {
    /// Bandwidth of the assigned band, in MHz.
    /// ຄວາມກວ້າງຂອງແຖບຄື້ນຄວາມຖີ່ (MHz)
    pub fn bandwidth_mhz(&self) -> u32 {
        self.band_end_mhz.saturating_sub(self.band_start_mhz)
    }

    /// Whether this assignment's frequency band overlaps that of `other`.
    ///
    /// Two half-open bands `[start, end)` overlap when each starts strictly
    /// before the other ends; adjacent bands that merely touch do not overlap.
    /// ກວດເບິ່ງວ່າແຖບຄື້ນຄວາມຖີ່ຊ້ອນກັນຫຼືບໍ່
    pub fn overlaps(&self, other: &SpectrumAssignment) -> bool {
        self.band_start_mhz < other.band_end_mhz && other.band_start_mhz < self.band_end_mhz
    }
}

// ============================================================================
// Interconnection - ການເຊື່ອມຕໍ່ໂຄງຂ່າຍ
// ============================================================================

/// Request by one operator to interconnect with another - ຄຳຮ້ອງຂໍເຊື່ອມຕໍ່ໂຄງຂ່າຍ
///
/// Interconnection must be provided on fair, reasonable and non-discriminatory
/// terms.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InterconnectionRequest {
    /// Operator requesting interconnection (ຜູ້ປະກອບການທີ່ຮ້ອງຂໍ)
    pub requesting_operator: String,
    /// Host operator asked to interconnect (ຜູ້ປະກອບການເຈົ້າຂອງໂຄງຂ່າຍ)
    pub host_operator: String,
    /// Whether interconnection has been granted (ໄດ້ຮັບການອະນຸຍາດ)
    pub granted: bool,
    /// Whether the offered terms are non-discriminatory (ເງື່ອນໄຂບໍ່ເລືອກປະຕິບັດ)
    pub non_discriminatory: bool,
    /// Whether the offered terms are fair and reasonable (ເງື່ອນໄຂທີ່ເປັນທຳ ແລະ ສົມເຫດສົມຜົນ)
    pub fair_terms: bool,
}

// ============================================================================
// Quality of Service & Tariffs - ຄຸນນະພາບການບໍລິການ ແລະ ອັດຕາຄ່າບໍລິການ
// ============================================================================

/// Quality-of-service measurement for a service - ການວັດແທກຄຸນນະພາບການບໍລິການ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServiceQuality {
    /// Service being measured (ການບໍລິການທີ່ຖືກວັດແທກ)
    pub service_type: ServiceType,
    /// Measured service availability, as a percentage (ຄວາມພ້ອມໃຫ້ບໍລິການ, %)
    pub availability_percent: u32,
    /// Measured call-drop rate, in per-mille (ອັດຕາການຫຼຸດສາຍ, ຕໍ່ພັນ)
    pub call_drop_rate_permille: u32,
}

impl ServiceQuality {
    /// Whether the measurement meets the representative quality-of-service
    /// targets ([`MIN_SERVICE_AVAILABILITY_PERCENT`] and
    /// [`MAX_CALL_DROP_RATE_PERMILLE`]).
    /// ໄດ້ມາດຕະຖານຄຸນນະພາບການບໍລິການຫຼືບໍ່
    pub fn meets_targets(&self) -> bool {
        self.availability_percent >= MIN_SERVICE_AVAILABILITY_PERCENT
            && self.call_drop_rate_permille <= MAX_CALL_DROP_RATE_PERMILLE
    }
}

/// Tariff (price) for a telecommunications service - ອັດຕາຄ່າບໍລິການໂທລະຄົມມະນາຄົມ
///
/// Tariffs may require approval by the regulatory authority before they apply.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tariff {
    /// Service the tariff applies to (ການບໍລິການທີ່ກ່ຽວຂ້ອງ)
    pub service_type: ServiceType,
    /// Price in Lao kip, LAK (ລາຄາເປັນກີບ)
    pub price_lak: u64,
    /// Whether the tariff carries regulatory approval (ໄດ້ຮັບການອະນຸມັດຈາກອົງການຄຸ້ມຄອງ)
    pub regulator_approved: bool,
}

// ============================================================================
// Equipment Type-approval - ການຮັບຮອງປະເພດອຸປະກອນ
// ============================================================================

/// Type-approval record for telecommunications equipment - ການຮັບຮອງປະເພດອຸປະກອນ
///
/// Telecommunications equipment requires type-approval before it may be
/// connected to a public network or placed on the market.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EquipmentTypeApproval {
    /// Name / model of the equipment (ຊື່/ລຸ້ນຂອງອຸປະກອນ)
    pub equipment_name: String,
    /// Whether the equipment is type-approved (ໄດ້ຮັບການຮັບຮອງປະເພດ)
    pub approved: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_overlap_helper() {
        let a = SpectrumAssignment {
            operator: "A".to_string(),
            band_start_mhz: 800,
            band_end_mhz: 820,
            exclusive: true,
        };
        let b = SpectrumAssignment {
            operator: "B".to_string(),
            band_start_mhz: 810,
            band_end_mhz: 830,
            exclusive: true,
        };
        let c = SpectrumAssignment {
            operator: "C".to_string(),
            band_start_mhz: 820,
            band_end_mhz: 840,
            exclusive: true,
        };
        assert!(a.overlaps(&b));
        // Adjacent half-open bands that merely touch do not overlap.
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn test_bandwidth_mhz() {
        let a = SpectrumAssignment {
            operator: "A".to_string(),
            band_start_mhz: 900,
            band_end_mhz: 960,
            exclusive: true,
        };
        assert_eq!(a.bandwidth_mhz(), 60);
    }

    #[test]
    fn test_license_expiry_and_permission() {
        let license = TelecomLicense {
            operator: "Lao Telecom".to_string(),
            category: LicenseCategory::NetworkServices,
            granted: true,
            validity_years: 15,
            start_year: 2020,
            status: LicenseStatus::Active,
        };
        assert_eq!(license.expiry_year(), 2035);
        assert!(license.permits_operation());
    }

    #[test]
    fn test_suspended_license_does_not_permit_operation() {
        let license = TelecomLicense {
            operator: "Lao Telecom".to_string(),
            category: LicenseCategory::NetworkServices,
            granted: true,
            validity_years: 15,
            start_year: 2020,
            status: LicenseStatus::Suspended,
        };
        assert!(!license.permits_operation());
        assert!(!LicenseStatus::Suspended.permits_operation());
        assert!(LicenseStatus::Active.permits_operation());
    }

    #[test]
    fn test_service_quality_meets_targets() {
        let good = ServiceQuality {
            service_type: ServiceType::Mobile,
            availability_percent: 99,
            call_drop_rate_permille: 10,
        };
        assert!(good.meets_targets());

        let bad = ServiceQuality {
            service_type: ServiceType::Mobile,
            availability_percent: 95,
            call_drop_rate_permille: 10,
        };
        assert!(!bad.meets_targets());
    }

    #[test]
    fn test_bilingual_names_present() {
        let services = [
            ServiceType::FixedLine,
            ServiceType::Mobile,
            ServiceType::Internet,
            ServiceType::Satellite,
            ServiceType::LeasedLine,
            ServiceType::DataServices,
            ServiceType::BroadcastTransmission,
        ];
        for service in services {
            assert!(!service.lao_name().is_empty());
            assert!(!service.english_name().is_empty());
        }

        let categories = [
            LicenseCategory::NetworkFacilities,
            LicenseCategory::NetworkServices,
            LicenseCategory::ApplicationServices,
            LicenseCategory::Spectrum,
        ];
        for category in categories {
            assert!(!category.lao_name().is_empty());
            assert!(!category.english_name().is_empty());
        }

        let statuses = [
            LicenseStatus::Pending,
            LicenseStatus::Active,
            LicenseStatus::Suspended,
            LicenseStatus::Revoked,
            LicenseStatus::Expired,
        ];
        for status in statuses {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }

        let operators = [
            OperatorType::FacilitiesBased,
            OperatorType::ServiceBased,
            OperatorType::Reseller,
            OperatorType::VirtualOperator,
        ];
        for operator in operators {
            assert!(!operator.lao_name().is_empty());
            assert!(!operator.english_name().is_empty());
        }
    }
}
