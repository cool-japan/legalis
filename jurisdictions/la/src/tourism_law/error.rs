//! Tourism Law Error Types (ປະເພດຄວາມຜິດພາດກົດໝາຍທ່ອງທ່ຽວ)
//!
//! Comprehensive error types for Lao tourism law validation and compliance.
//! All errors include bilingual messages (Lao/English).
//!
//! ## Legal Basis
//!
//! - **Tourism Law 2013** (Law No. 32/NA, effective September 2013)
//! - **Tourism Development Regulations**
//! - **ASEAN Tourism Agreement**

use thiserror::Error;

/// Result type for tourism law operations
pub type Result<T> = std::result::Result<T, TourismLawError>;

/// Tourism law errors (ຄວາມຜິດພາດກົດໝາຍທ່ອງທ່ຽວ)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum TourismLawError {
    // ========================================================================
    // Tourism Enterprise License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດວິສາຫະກິດທ່ອງທ່ຽວ)
    // ========================================================================
    /// Enterprise is unlicensed (Article 23)
    /// ວິສາຫະກິດບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 23)
    #[error(
        "Tourism enterprise is unlicensed: {enterprise_name} (Article 23)\nວິສາຫະກິດທ່ອງທ່ຽວບໍ່ມີໃບອະນຸຍາດ: {enterprise_name} (ມາດຕາ 23)"
    )]
    EnterpriseUnlicensed { enterprise_name: String },

    /// Enterprise license expired (Article 26)
    /// ໃບອະນຸຍາດວິສາຫະກິດໝົດອາຍຸ (ມາດຕາ 26)
    #[error(
        "Tourism enterprise license expired on {expiry_date}: {enterprise_name} (Article 26)\nໃບອະນຸຍາດວິສາຫະກິດທ່ອງທ່ຽວໝົດອາຍຸວັນທີ {expiry_date}: {enterprise_name} (ມາດຕາ 26)"
    )]
    EnterpriseLicenseExpired {
        enterprise_name: String,
        expiry_date: String,
    },

    /// Enterprise license suspended (Article 68)
    /// ໃບອະນຸຍາດວິສາຫະກິດຖືກລະງັບ (ມາດຕາ 68)
    #[error(
        "Tourism enterprise license suspended: {enterprise_name}, reason: {reason} (Article 68)\nໃບອະນຸຍາດວິສາຫະກິດທ່ອງທ່ຽວຖືກລະງັບ: {enterprise_name}, ເຫດຜົນ: {reason} (ມາດຕາ 68)"
    )]
    EnterpriseLicenseSuspended {
        enterprise_name: String,
        reason: String,
    },

    /// Enterprise license revoked (Article 69)
    /// ໃບອະນຸຍາດວິສາຫະກິດຖືກຍົກເລີກ (ມາດຕາ 69)
    #[error(
        "Tourism enterprise license revoked: {enterprise_name}, reason: {reason} (Article 69)\nໃບອະນຸຍາດວິສາຫະກິດທ່ອງທ່ຽວຖືກຍົກເລີກ: {enterprise_name}, ເຫດຜົນ: {reason} (ມາດຕາ 69)"
    )]
    EnterpriseLicenseRevoked {
        enterprise_name: String,
        reason: String,
    },

    /// Invalid enterprise type for activity (Article 22)
    /// ປະເພດວິສາຫະກິດບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ (ມາດຕາ 22)
    #[error(
        "Invalid enterprise type '{enterprise_type}' for activity '{activity}' (Article 22)\nປະເພດວິສາຫະກິດ '{enterprise_type}' ບໍ່ຖືກຕ້ອງສຳລັບກິດຈະກຳ '{activity}' (ມາດຕາ 22)"
    )]
    InvalidEnterpriseType {
        enterprise_type: String,
        activity: String,
    },

    // ========================================================================
    // Hotel Classification Errors (ຄວາມຜິດພາດການຈັດລະດັບໂຮງແຮມ)
    // ========================================================================
    /// Hotel not classified (Article 30)
    /// ໂຮງແຮມບໍ່ໄດ້ຈັດລະດັບ (ມາດຕາ 30)
    #[error(
        "Hotel is not classified: {hotel_name} (Article 30)\nໂຮງແຮມບໍ່ໄດ້ຈັດລະດັບ: {hotel_name} (ມາດຕາ 30)"
    )]
    HotelNotClassified { hotel_name: String },

    /// Insufficient rooms for star rating (Article 31)
    /// ຈຳນວນຫ້ອງບໍ່ພຽງພໍສຳລັບລະດັບດາວ (ມາດຕາ 31)
    #[error(
        "Insufficient rooms for {star_rating}-star rating: {actual} rooms, required minimum {required} (Article 31)\nຈຳນວນຫ້ອງບໍ່ພຽງພໍສຳລັບລະດັບ {star_rating} ດາວ: {actual} ຫ້ອງ, ຕ້ອງການຂັ້ນຕ່ຳ {required} (ມາດຕາ 31)"
    )]
    InsufficientRooms {
        star_rating: u8,
        actual: u32,
        required: u32,
    },

    /// Missing required facilities for star rating (Article 32)
    /// ຂາດສິ່ງອຳນວຍຄວາມສະດວກທີ່ຕ້ອງການສຳລັບລະດັບດາວ (ມາດຕາ 32)
    #[error(
        "Missing required facility '{facility}' for {star_rating}-star hotel (Article 32)\nຂາດສິ່ງອຳນວຍຄວາມສະດວກ '{facility}' ທີ່ຕ້ອງການສຳລັບໂຮງແຮມ {star_rating} ດາວ (ມາດຕາ 32)"
    )]
    MissingRequiredFacility { facility: String, star_rating: u8 },

    /// Star rating classification expired (Article 33)
    /// ການຈັດລະດັບດາວໝົດອາຍຸ (ມາດຕາ 33)
    #[error(
        "Star rating classification expired on {expiry_date}: {hotel_name} (Article 33)\nການຈັດລະດັບດາວໝົດອາຍຸວັນທີ {expiry_date}: {hotel_name} (ມາດຕາ 33)"
    )]
    StarRatingExpired {
        hotel_name: String,
        expiry_date: String,
    },

    // ========================================================================
    // Tour Guide License Errors (ຄວາມຜິດພາດໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວ)
    // ========================================================================
    /// Tour guide unlicensed (Article 35)
    /// ໄກດ໌ນຳທ່ຽວບໍ່ມີໃບອະນຸຍາດ (ມາດຕາ 35)
    #[error(
        "Tour guide is unlicensed: {guide_name} (Article 35)\nໄກດ໌ນຳທ່ຽວບໍ່ມີໃບອະນຸຍາດ: {guide_name} (ມາດຕາ 35)"
    )]
    GuideUnlicensed { guide_name: String },

    /// Tour guide license expired (Article 38)
    /// ໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວໝົດອາຍຸ (ມາດຕາ 38)
    #[error(
        "Tour guide license expired on {expiry_date}: {guide_name} (Article 38)\nໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວໝົດອາຍຸວັນທີ {expiry_date}: {guide_name} (ມາດຕາ 38)"
    )]
    GuideLicenseExpired {
        guide_name: String,
        expiry_date: String,
    },

    /// Tour guide license suspended (Article 40)
    /// ໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວຖືກລະງັບ (ມາດຕາ 40)
    #[error(
        "Tour guide license suspended: {guide_name}, reason: {reason} (Article 40)\nໃບອະນຸຍາດໄກດ໌ນຳທ່ຽວຖືກລະງັບ: {guide_name}, ເຫດຜົນ: {reason} (ມາດຕາ 40)"
    )]
    GuideLicenseSuspended { guide_name: String, reason: String },

    /// Insufficient language proficiency (Article 36)
    /// ຄວາມສາມາດທາງພາສາບໍ່ພຽງພໍ (ມາດຕາ 36)
    #[error(
        "Insufficient language proficiency for tour guide: {guide_name}, language: {language} (Article 36)\nຄວາມສາມາດທາງພາສາບໍ່ພຽງພໍສຳລັບໄກດ໌ນຳທ່ຽວ: {guide_name}, ພາສາ: {language} (ມາດຕາ 36)"
    )]
    InsufficientLanguageProficiency {
        guide_name: String,
        language: String,
    },

    /// Missing training certification (Article 37)
    /// ຂາດໃບຢັ້ງຢືນການຝຶກອົບຮົມ (ມາດຕາ 37)
    #[error(
        "Missing required training certification for tour guide: {guide_name} (Article 37)\nຂາດໃບຢັ້ງຢືນການຝຶກອົບຮົມທີ່ຕ້ອງການສຳລັບໄກດ໌ນຳທ່ຽວ: {guide_name} (ມາດຕາ 37)"
    )]
    MissingTrainingCertification { guide_name: String },

    /// Guide scope exceeded (Article 39)
    /// ເກີນຂອບເຂດໃບອະນຸຍາດໄກດ໌ (ມາດຕາ 39)
    #[error(
        "Guide scope exceeded: {guide_name} with {license_type} license operating in {zone} (Article 39)\nເກີນຂອບເຂດໃບອະນຸຍາດໄກດ໌: {guide_name} ມີໃບອະນຸຍາດ {license_type} ປະຕິບັດງານໃນ {zone} (ມາດຕາ 39)"
    )]
    GuideScopeExceeded {
        guide_name: String,
        license_type: String,
        zone: String,
    },

    // ========================================================================
    // Foreign Ownership Errors (ຄວາມຜິດພາດການເປັນເຈົ້າຂອງຕ່າງປະເທດ)
    // ========================================================================
    /// Foreign ownership limit exceeded (Article 25)
    /// ເກີນຂີດຈຳກັດການເປັນເຈົ້າຂອງຕ່າງປະເທດ (ມາດຕາ 25)
    #[error(
        "Foreign ownership limit exceeded: {actual_percent:.1}% exceeds maximum {max_percent:.1}% for {activity} (Article 25)\nເກີນຂີດຈຳກັດການເປັນເຈົ້າຂອງຕ່າງປະເທດ: {actual_percent:.1}% ເກີນສູງສຸດ {max_percent:.1}% ສຳລັບ {activity} (ມາດຕາ 25)"
    )]
    ForeignOwnershipLimitExceeded {
        actual_percent: f64,
        max_percent: f64,
        activity: String,
    },

    /// Foreign investment not permitted (Article 25)
    /// ການລົງທຶນຕ່າງປະເທດບໍ່ອະນຸຍາດ (ມາດຕາ 25)
    #[error(
        "Foreign investment not permitted for activity: {activity} (Article 25)\nການລົງທຶນຕ່າງປະເທດບໍ່ອະນຸຍາດສຳລັບກິດຈະກຳ: {activity} (ມາດຕາ 25)"
    )]
    ForeignInvestmentNotPermitted { activity: String },

    // ========================================================================
    // Tourism Zone Errors (ຄວາມຜິດພາດເຂດທ່ອງທ່ຽວ)
    // ========================================================================
    /// Operating in prohibited zone (Article 44)
    /// ປະຕິບັດງານໃນເຂດຫ້າມ (ມາດຕາ 44)
    #[error(
        "Operating in prohibited tourism zone: {zone_name}, reason: {reason} (Article 44)\nປະຕິບັດງານໃນເຂດທ່ອງທ່ຽວຫ້າມ: {zone_name}, ເຫດຜົນ: {reason} (ມາດຕາ 44)"
    )]
    ProhibitedZone { zone_name: String, reason: String },

    /// Zone permit required (Article 45)
    /// ຕ້ອງການໃບອະນຸຍາດເຂດ (ມາດຕາ 45)
    #[error(
        "Special permit required for tourism zone: {zone_name} (Article 45)\nຕ້ອງການໃບອະນຸຍາດພິເສດສຳລັບເຂດທ່ອງທ່ຽວ: {zone_name} (ມາດຕາ 45)"
    )]
    ZonePermitRequired { zone_name: String },

    /// Carrying capacity exceeded (Article 46)
    /// ເກີນຄວາມສາມາດຮອງຮັບ (ມາດຕາ 46)
    #[error(
        "Carrying capacity exceeded for {zone_name}: {actual} visitors, maximum {max_capacity} (Article 46)\nເກີນຄວາມສາມາດຮອງຮັບສຳລັບ {zone_name}: {actual} ນັກທ່ອງທ່ຽວ, ສູງສຸດ {max_capacity} (ມາດຕາ 46)"
    )]
    CarryingCapacityExceeded {
        zone_name: String,
        actual: u32,
        max_capacity: u32,
    },

    // ========================================================================
    // Tourist Rights Errors (ຄວາມຜິດພາດສິດນັກທ່ອງທ່ຽວ)
    // ========================================================================
    /// Tourist complaint not addressed (Article 50)
    /// ຄຳຮ້ອງທຸກຂອງນັກທ່ອງທ່ຽວບໍ່ໄດ້ຮັບການແກ້ໄຂ (ມາດຕາ 50)
    #[error(
        "Tourist complaint not addressed within {days} days: complaint #{complaint_id} (Article 50)\nຄຳຮ້ອງທຸກຂອງນັກທ່ອງທ່ຽວບໍ່ໄດ້ຮັບການແກ້ໄຂພາຍໃນ {days} ມື້: ຄຳຮ້ອງທຸກ #{complaint_id} (ມາດຕາ 50)"
    )]
    ComplaintNotAddressed { complaint_id: String, days: u32 },

    /// Missing travel insurance (Article 51)
    /// ຂາດປະກັນການເດີນທາງ (ມາດຕາ 51)
    #[error(
        "Missing required travel insurance for tour: {tour_name} (Article 51)\nຂາດປະກັນການເດີນທາງທີ່ຕ້ອງການສຳລັບທົວ: {tour_name} (ມາດຕາ 51)"
    )]
    MissingTravelInsurance { tour_name: String },

    /// Consumer protection violation (Article 52)
    /// ການລະເມີດການປົກປ້ອງຜູ້ບໍລິໂພກ (ມາດຕາ 52)
    #[error(
        "Consumer protection violation: {violation_type} affecting tourist {tourist_name} (Article 52)\nການລະເມີດການປົກປ້ອງຜູ້ບໍລິໂພກ: {violation_type} ກະທົບນັກທ່ອງທ່ຽວ {tourist_name} (ມາດຕາ 52)"
    )]
    ConsumerProtectionViolation {
        violation_type: String,
        tourist_name: String,
    },

    /// Emergency assistance not provided (Article 53)
    /// ບໍ່ໄດ້ສະໜອງການຊ່ວຍເຫຼືອສຸກເສີນ (ມາດຕາ 53)
    #[error(
        "Emergency assistance not provided to tourist: {tourist_name}, incident: {incident} (Article 53)\nບໍ່ໄດ້ສະໜອງການຊ່ວຍເຫຼືອສຸກເສີນແກ່ນັກທ່ອງທ່ຽວ: {tourist_name}, ເຫດການ: {incident} (ມາດຕາ 53)"
    )]
    EmergencyAssistanceNotProvided {
        tourist_name: String,
        incident: String,
    },

    // ========================================================================
    // Sustainable Tourism Errors (ຄວາມຜິດພາດການທ່ອງທ່ຽວແບບຍືນຍົງ)
    // ========================================================================
    /// Environmental impact violation (Article 54)
    /// ການລະເມີດຜົນກະທົບຕໍ່ສິ່ງແວດລ້ອມ (ມາດຕາ 54)
    #[error(
        "Environmental impact violation at {location}: {violation} (Article 54)\nການລະເມີດຜົນກະທົບຕໍ່ສິ່ງແວດລ້ອມທີ່ {location}: {violation} (ມາດຕາ 54)"
    )]
    EnvironmentalImpactViolation { location: String, violation: String },

    /// Cultural heritage damage (Article 55)
    /// ຄວາມເສຍຫາຍຕໍ່ມໍລະດົກວັດທະນະທຳ (ມາດຕາ 55)
    #[error(
        "Cultural heritage damage at {site_name}: {damage_description} (Article 55)\nຄວາມເສຍຫາຍຕໍ່ມໍລະດົກວັດທະນະທຳທີ່ {site_name}: {damage_description} (ມາດຕາ 55)"
    )]
    CulturalHeritageDamage {
        site_name: String,
        damage_description: String,
    },

    /// Community-based tourism violation (Article 56)
    /// ການລະເມີດການທ່ອງທ່ຽວໂດຍຊຸມຊົນ (ມາດຕາ 56)
    #[error(
        "Community-based tourism violation: {violation} at {village_name} (Article 56)\nການລະເມີດການທ່ອງທ່ຽວໂດຍຊຸມຊົນ: {violation} ທີ່ {village_name} (ມາດຕາ 56)"
    )]
    CommunityBasedTourismViolation {
        violation: String,
        village_name: String,
    },

    // ========================================================================
    // Tourism Fee Errors (ຄວາມຜິດພາດຄ່າທຳນຽມທ່ອງທ່ຽວ)
    // ========================================================================
    /// Tourism development fund contribution not paid (Article 60)
    /// ບໍ່ໄດ້ຈ່າຍເງິນປະກອບສ່ວນກອງທຶນພັດທະນາການທ່ອງທ່ຽວ (ມາດຕາ 60)
    #[error(
        "Tourism development fund contribution not paid: {enterprise_name}, amount: {amount_lak} LAK (Article 60)\nບໍ່ໄດ້ຈ່າຍເງິນປະກອບສ່ວນກອງທຶນພັດທະນາການທ່ອງທ່ຽວ: {enterprise_name}, ຈຳນວນ: {amount_lak} ກີບ (ມາດຕາ 60)"
    )]
    DevelopmentFundNotPaid {
        enterprise_name: String,
        amount_lak: u64,
    },

    /// License fee not paid (Article 27)
    /// ບໍ່ໄດ້ຈ່າຍຄ່າທຳນຽມໃບອະນຸຍາດ (ມາດຕາ 27)
    #[error(
        "License fee not paid: {enterprise_name}, fee: {fee_lak} LAK (Article 27)\nບໍ່ໄດ້ຈ່າຍຄ່າທຳນຽມໃບອະນຸຍາດ: {enterprise_name}, ຄ່າທຳນຽມ: {fee_lak} ກີບ (ມາດຕາ 27)"
    )]
    LicenseFeeNotPaid {
        enterprise_name: String,
        fee_lak: u64,
    },

    /// Entrance fee overcharge (Article 61)
    /// ເກັບຄ່າເຂົ້າຊົມເກີນລາຄາ (ມາດຕາ 61)
    #[error(
        "Entrance fee overcharge at {attraction_name}: charged {charged_lak} LAK, maximum {max_lak} LAK (Article 61)\nເກັບຄ່າເຂົ້າຊົມເກີນລາຄາທີ່ {attraction_name}: ເກັບ {charged_lak} ກີບ, ສູງສຸດ {max_lak} ກີບ (ມາດຕາ 61)"
    )]
    EntranceFeeOvercharge {
        attraction_name: String,
        charged_lak: u64,
        max_lak: u64,
    },

    // ========================================================================
    // Foreign Tourist Regulation Errors (ຄວາມຜິດພາດລະບຽບນັກທ່ອງທ່ຽວຕ່າງປະເທດ)
    // ========================================================================
    /// Invalid visa for tourism (Article 62)
    /// ວີຊາບໍ່ຖືກຕ້ອງສຳລັບການທ່ອງທ່ຽວ (ມາດຕາ 62)
    #[error(
        "Invalid visa for tourism: {tourist_name}, visa type: {visa_type} (Article 62)\nວີຊາບໍ່ຖືກຕ້ອງສຳລັບການທ່ອງທ່ຽວ: {tourist_name}, ປະເພດວີຊາ: {visa_type} (ມາດຕາ 62)"
    )]
    InvalidVisaForTourism {
        tourist_name: String,
        visa_type: String,
    },

    /// Entry into security zone (Article 63)
    /// ເຂົ້າໄປໃນເຂດຄວາມໝັ້ນຄົງ (ມາດຕາ 63)
    #[error(
        "Unauthorized entry into security zone: {zone_name} by tourist {tourist_name} (Article 63)\nການເຂົ້າໄປໃນເຂດຄວາມໝັ້ນຄົງໂດຍບໍ່ໄດ້ຮັບອະນຸຍາດ: {zone_name} ໂດຍນັກທ່ອງທ່ຽວ {tourist_name} (ມາດຕາ 63)"
    )]
    SecurityZoneViolation {
        zone_name: String,
        tourist_name: String,
    },

    /// Tourism statistics not reported (Article 64)
    /// ບໍ່ໄດ້ລາຍງານສະຖິຕິການທ່ອງທ່ຽວ (ມາດຕາ 64)
    #[error(
        "Tourism statistics not reported by {enterprise_name} for period {period} (Article 64)\nບໍ່ໄດ້ລາຍງານສະຖິຕິການທ່ອງທ່ຽວໂດຍ {enterprise_name} ສຳລັບໄລຍະ {period} (ມາດຕາ 64)"
    )]
    StatisticsNotReported {
        enterprise_name: String,
        period: String,
    },

    // ========================================================================
    // ASEAN Integration Errors (ຄວາມຜິດພາດການເຊື່ອມໂຍງອາຊຽນ)
    // ========================================================================
    /// ASEAN MRA non-compliance (Article 65)
    /// ບໍ່ປະຕິບັດຕາມ MRA ອາຊຽນ (ມາດຕາ 65)
    #[error(
        "ASEAN MRA non-compliance for {professional_type}: {issue} (Article 65)\nບໍ່ປະຕິບັດຕາມ MRA ອາຊຽນສຳລັບ {professional_type}: {issue} (ມາດຕາ 65)"
    )]
    AseanMraNonCompliance {
        professional_type: String,
        issue: String,
    },

    /// Cross-border package violation (Article 66)
    /// ການລະເມີດແພັກເກັດຂ້າມຊາຍແດນ (ມາດຕາ 66)
    #[error(
        "Cross-border tourism package violation: {violation} for package {package_name} (Article 66)\nການລະເມີດແພັກເກັດທ່ອງທ່ຽວຂ້າມຊາຍແດນ: {violation} ສຳລັບແພັກເກັດ {package_name} (ມາດຕາ 66)"
    )]
    CrossBorderPackageViolation {
        violation: String,
        package_name: String,
    },

    // ========================================================================
    // Tour Operator Errors (ຄວາມຜິດພາດຜູ້ປະກອບການທົວ)
    // ========================================================================
    /// Tour operator license scope exceeded (Article 28)
    /// ເກີນຂອບເຂດໃບອະນຸຍາດຜູ້ປະກອບການທົວ (ມາດຕາ 28)
    #[error(
        "Tour operator license scope exceeded: {operator_name} with {license_type} conducting {activity} (Article 28)\nເກີນຂອບເຂດໃບອະນຸຍາດຜູ້ປະກອບການທົວ: {operator_name} ມີໃບອະນຸຍາດ {license_type} ດຳເນີນ {activity} (ມາດຕາ 28)"
    )]
    TourOperatorScopeExceeded {
        operator_name: String,
        license_type: String,
        activity: String,
    },

    /// Itinerary not registered (Article 29)
    /// ເສັ້ນທາງການເດີນທາງບໍ່ໄດ້ຂຶ້ນທະບຽນ (ມາດຕາ 29)
    #[error(
        "Tour itinerary not registered: {itinerary_name} by {operator_name} (Article 29)\nເສັ້ນທາງການເດີນທາງບໍ່ໄດ້ຂຶ້ນທະບຽນ: {itinerary_name} ໂດຍ {operator_name} (ມາດຕາ 29)"
    )]
    ItineraryNotRegistered {
        itinerary_name: String,
        operator_name: String,
    },

    // ========================================================================
    // Tourism Transport Errors (ຄວາມຜິດພາດການຂົນສົ່ງທ່ອງທ່ຽວ)
    // ========================================================================
    /// Tourism transport license invalid (Article 34)
    /// ໃບອະນຸຍາດຂົນສົ່ງທ່ອງທ່ຽວບໍ່ຖືກຕ້ອງ (ມາດຕາ 34)
    #[error(
        "Tourism transport license invalid: {vehicle_id}, reason: {reason} (Article 34)\nໃບອະນຸຍາດຂົນສົ່ງທ່ອງທ່ຽວບໍ່ຖືກຕ້ອງ: {vehicle_id}, ເຫດຜົນ: {reason} (ມາດຕາ 34)"
    )]
    TransportLicenseInvalid { vehicle_id: String, reason: String },

    /// Transport safety violation (Article 34)
    /// ການລະເມີດຄວາມປອດໄພການຂົນສົ່ງ (ມາດຕາ 34)
    #[error(
        "Tourism transport safety violation: {vehicle_id}, violation: {violation} (Article 34)\nການລະເມີດຄວາມປອດໄພການຂົນສົ່ງທ່ອງທ່ຽວ: {vehicle_id}, ການລະເມີດ: {violation} (ມາດຕາ 34)"
    )]
    TransportSafetyViolation {
        vehicle_id: String,
        violation: String,
    },

    // ========================================================================
    // Code of Conduct Errors (ຄວາມຜິດພາດຈັນຍາບັນ)
    // ========================================================================
    /// Code of conduct violation (Article 41)
    /// ການລະເມີດຈັນຍາບັນ (ມາດຕາ 41)
    #[error(
        "Code of conduct violation by {person_name}: {violation} (Article 41)\nການລະເມີດຈັນຍາບັນໂດຍ {person_name}: {violation} (ມາດຕາ 41)"
    )]
    CodeOfConductViolation {
        person_name: String,
        violation: String,
    },

    // ========================================================================
    // General Errors (ຄວາມຜິດພາດທົ່ວໄປ)
    // ========================================================================
    /// Validation error
    /// ຄວາມຜິດພາດການກວດສອບ
    #[error("Validation error: {message}\nຄວາມຜິດພາດການກວດສອບ: {message}")]
    ValidationError { message: String },

    /// Missing required field
    /// ຂາດຊ່ອງຂໍ້ມູນທີ່ຕ້ອງການ
    #[error("Missing required field: {field_name}\nຂາດຊ່ອງຂໍ້ມູນທີ່ຕ້ອງການ: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid date
    /// ວັນທີບໍ່ຖືກຕ້ອງ
    #[error("Invalid date: {date_description}\nວັນທີບໍ່ຖືກຕ້ອງ: {date_description}")]
    InvalidDate { date_description: String },

    /// General tourism law violation
    /// ການລະເມີດກົດໝາຍທ່ອງທ່ຽວທົ່ວໄປ
    #[error(
        "Tourism law violation: {violation} (Article {article})\nການລະເມີດກົດໝາຍທ່ອງທ່ຽວ: {violation} (ມາດຕາ {article})"
    )]
    TourismLawViolation { violation: String, article: u32 },
}

impl TourismLawError {
    /// Get the error message in Lao language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາລາວ
    pub fn lao_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the Lao part after the newline
        if let Some((_english, lao)) = full_msg.split_once('\n') {
            lao.to_string()
        } else {
            full_msg
        }
    }

    /// Get the error message in English language
    /// ຮັບຂໍ້ຄວາມຄວາມຜິດພາດເປັນພາສາອັງກິດ
    pub fn english_message(&self) -> String {
        let full_msg = format!("{}", self);
        // Extract the English part before the newline
        if let Some((english, _lao)) = full_msg.split_once('\n') {
            english.to_string()
        } else {
            full_msg
        }
    }

    /// Check if this is a critical violation requiring immediate action
    /// ກວດສອບວ່າເປັນການລະເມີດຮ້າຍແຮງທີ່ຕ້ອງແກ້ໄຂທັນທີ
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            TourismLawError::GuideUnlicensed { .. }
                | TourismLawError::EnterpriseUnlicensed { .. }
                | TourismLawError::SecurityZoneViolation { .. }
                | TourismLawError::TransportSafetyViolation { .. }
                | TourismLawError::EmergencyAssistanceNotProvided { .. }
                | TourismLawError::CulturalHeritageDamage { .. }
        )
    }

    /// Check if this is a tourist safety issue
    /// ກວດສອບວ່າເປັນບັນຫາຄວາມປອດໄພຂອງນັກທ່ອງທ່ຽວ
    pub fn is_tourist_safety_issue(&self) -> bool {
        matches!(
            self,
            TourismLawError::GuideUnlicensed { .. }
                | TourismLawError::GuideLicenseExpired { .. }
                | TourismLawError::GuideLicenseSuspended { .. }
                | TourismLawError::TransportLicenseInvalid { .. }
                | TourismLawError::TransportSafetyViolation { .. }
                | TourismLawError::MissingTravelInsurance { .. }
                | TourismLawError::EmergencyAssistanceNotProvided { .. }
        )
    }

    /// Get the article number referenced in this error, if any
    /// ຮັບເລກມາດຕາທີ່ອ້າງອິງໃນຄວາມຜິດພາດນີ້
    pub fn article_number(&self) -> Option<u32> {
        match self {
            TourismLawError::EnterpriseUnlicensed { .. } => Some(23),
            TourismLawError::EnterpriseLicenseExpired { .. } => Some(26),
            TourismLawError::EnterpriseLicenseSuspended { .. } => Some(68),
            TourismLawError::EnterpriseLicenseRevoked { .. } => Some(69),
            TourismLawError::InvalidEnterpriseType { .. } => Some(22),
            TourismLawError::HotelNotClassified { .. } => Some(30),
            TourismLawError::InsufficientRooms { .. } => Some(31),
            TourismLawError::MissingRequiredFacility { .. } => Some(32),
            TourismLawError::StarRatingExpired { .. } => Some(33),
            TourismLawError::GuideUnlicensed { .. } => Some(35),
            TourismLawError::GuideLicenseExpired { .. } => Some(38),
            TourismLawError::GuideLicenseSuspended { .. } => Some(40),
            TourismLawError::InsufficientLanguageProficiency { .. } => Some(36),
            TourismLawError::MissingTrainingCertification { .. } => Some(37),
            TourismLawError::GuideScopeExceeded { .. } => Some(39),
            TourismLawError::ForeignOwnershipLimitExceeded { .. } => Some(25),
            TourismLawError::ForeignInvestmentNotPermitted { .. } => Some(25),
            TourismLawError::ProhibitedZone { .. } => Some(44),
            TourismLawError::ZonePermitRequired { .. } => Some(45),
            TourismLawError::CarryingCapacityExceeded { .. } => Some(46),
            TourismLawError::ComplaintNotAddressed { .. } => Some(50),
            TourismLawError::MissingTravelInsurance { .. } => Some(51),
            TourismLawError::ConsumerProtectionViolation { .. } => Some(52),
            TourismLawError::EmergencyAssistanceNotProvided { .. } => Some(53),
            TourismLawError::EnvironmentalImpactViolation { .. } => Some(54),
            TourismLawError::CulturalHeritageDamage { .. } => Some(55),
            TourismLawError::CommunityBasedTourismViolation { .. } => Some(56),
            TourismLawError::DevelopmentFundNotPaid { .. } => Some(60),
            TourismLawError::LicenseFeeNotPaid { .. } => Some(27),
            TourismLawError::EntranceFeeOvercharge { .. } => Some(61),
            TourismLawError::InvalidVisaForTourism { .. } => Some(62),
            TourismLawError::SecurityZoneViolation { .. } => Some(63),
            TourismLawError::StatisticsNotReported { .. } => Some(64),
            TourismLawError::AseanMraNonCompliance { .. } => Some(65),
            TourismLawError::CrossBorderPackageViolation { .. } => Some(66),
            TourismLawError::TourOperatorScopeExceeded { .. } => Some(28),
            TourismLawError::ItineraryNotRegistered { .. } => Some(29),
            TourismLawError::TransportLicenseInvalid { .. } => Some(34),
            TourismLawError::TransportSafetyViolation { .. } => Some(34),
            TourismLawError::CodeOfConductViolation { .. } => Some(41),
            TourismLawError::TourismLawViolation { article, .. } => Some(*article),
            _ => None,
        }
    }

    /// Get the legal basis for this error
    /// ຮັບພື້ນຖານທາງກົດໝາຍສຳລັບຄວາມຜິດພາດນີ້
    pub fn legal_basis(&self) -> &'static str {
        match self {
            TourismLawError::AseanMraNonCompliance { .. }
            | TourismLawError::CrossBorderPackageViolation { .. } => {
                "ASEAN Tourism Agreement / Tourism Law 2013"
            }
            _ => "Tourism Law 2013 (Law No. 32/NA)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_error_messages() {
        let error = TourismLawError::EnterpriseUnlicensed {
            enterprise_name: "Test Hotel".to_string(),
        };

        let english = error.english_message();
        let lao = error.lao_message();

        assert!(english.contains("Tourism enterprise is unlicensed"));
        assert!(lao.contains("ວິສາຫະກິດທ່ອງທ່ຽວບໍ່ມີໃບອະນຸຍາດ"));
    }

    #[test]
    fn test_critical_violations() {
        let guide_unlicensed = TourismLawError::GuideUnlicensed {
            guide_name: "Test Guide".to_string(),
        };
        assert!(guide_unlicensed.is_critical());

        let license_expired = TourismLawError::EnterpriseLicenseExpired {
            enterprise_name: "Test Hotel".to_string(),
            expiry_date: "2023-01-01".to_string(),
        };
        assert!(!license_expired.is_critical());
    }

    #[test]
    fn test_tourist_safety_issues() {
        let transport_safety = TourismLawError::TransportSafetyViolation {
            vehicle_id: "V001".to_string(),
            violation: "No safety equipment".to_string(),
        };
        assert!(transport_safety.is_tourist_safety_issue());

        let fee_not_paid = TourismLawError::LicenseFeeNotPaid {
            enterprise_name: "Test".to_string(),
            fee_lak: 1_000_000,
        };
        assert!(!fee_not_paid.is_tourist_safety_issue());
    }

    #[test]
    fn test_article_numbers() {
        let error = TourismLawError::GuideUnlicensed {
            guide_name: "Test".to_string(),
        };
        assert_eq!(error.article_number(), Some(35));

        let error = TourismLawError::HotelNotClassified {
            hotel_name: "Test".to_string(),
        };
        assert_eq!(error.article_number(), Some(30));
    }

    #[test]
    fn test_legal_basis() {
        let asean_error = TourismLawError::AseanMraNonCompliance {
            professional_type: "Tour Guide".to_string(),
            issue: "Non-compliance".to_string(),
        };
        assert!(asean_error.legal_basis().contains("ASEAN"));

        let regular_error = TourismLawError::EnterpriseUnlicensed {
            enterprise_name: "Test".to_string(),
        };
        assert_eq!(
            regular_error.legal_basis(),
            "Tourism Law 2013 (Law No. 32/NA)"
        );
    }
}
