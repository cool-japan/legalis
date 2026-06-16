//! Intellectual Property Law Types (ປະເພດກົດໝາຍຊັບສິນທາງປັນຍາ)
//!
//! Type definitions for Lao intellectual property law based on the
//! **Law on Intellectual Property (Lao PDR), No. 38/NA, 2017**
//! (ກົດໝາຍວ່າດ້ວຍຊັບສິນທາງປັນຍາ), the consolidated/amended IP Law (originally
//! No. 01/NA 2011, amended 2017).
//!
//! # Legal References
//!
//! - Law on Intellectual Property 2017 (No. 38/NA) — the primary statute.
//! - As a WTO member, Lao PDR implements the minimum protection standards of the
//!   TRIPS Agreement; as a party to the Berne Convention it grants the Berne
//!   minimum copyright term; the Paris Convention and the PCT govern industrial
//!   property and international patent filing.
//!
//! # Numeric thresholds
//!
//! Where the underlying statute fixes a quantifiable protection term (such as the
//! 20-year patent term or the life-plus-50-years copyright term) it is encoded as
//! a named, documented constant whose value is the well-established TRIPS/Berne
//! minimum implemented by the IP Law 2017. Substantive qualifying criteria
//! (novelty, distinctiveness, secrecy, DUS, etc.) are modelled as validated
//! boolean fields rather than as fabricated statutory figures.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Intellectual Property 2017 (No. 38/NA), TRIPS/Berne terms
// ============================================================================

/// Patent term of protection in years, measured from the filing date.
/// TRIPS minimum (20 years) as implemented by the IP Law 2017.
/// ອາຍຸການປົກປ້ອງສິດທິບັດ (ປີ)
pub const PATENT_TERM_YEARS: u32 = 20;

/// Petty patent (utility/minor innovation) term of protection in years.
/// ອາຍຸການປົກປ້ອງອານຸສິດທິບັດ (ປີ)
pub const PETTY_PATENT_TERM_YEARS: u32 = 10;

/// Industrial design term of protection in years.
/// ອາຍຸການປົກປ້ອງແບບອຸດສາຫະກຳ (ປີ)
pub const INDUSTRIAL_DESIGN_TERM_YEARS: u32 = 15;

/// Trademark registration term in years, renewable for successive periods.
/// TRIPS minimum (10 years) as implemented by the IP Law 2017.
/// ອາຍຸການຈົດທະບຽນເຄື່ອງໝາຍການຄ້າ (ປີ, ສາມາດຕໍ່ອາຍຸໄດ້)
pub const TRADEMARK_TERM_YEARS: u32 = 10;

/// Copyright term measured in years after the death of the author.
/// Berne minimum (life of the author + 50 years).
/// ອາຍຸລິຂະສິດຫຼັງຈາກຜູ້ປະພັນເສຍຊີວິດ (ປີ)
pub const COPYRIGHT_TERM_AFTER_DEATH_YEARS: u32 = 50;

/// Layout-design (topography) of an integrated circuit term in years.
/// ອາຍຸການປົກປ້ອງແບບຜັງວົງຈອນລວມ (ປີ)
pub const LAYOUT_DESIGN_TERM_YEARS: u32 = 10;

/// New plant variety protection term in years.
/// ອາຍຸການປົກປ້ອງພັນພືດໃໝ່ (ປີ)
pub const PLANT_VARIETY_TERM_YEARS: u32 = 20;

/// Number of distinct categories of intellectual property rights modelled by
/// the consolidated IP Law.
/// ຈຳນວນປະເພດສິດຊັບສິນທາງປັນຍາ
pub const IP_RIGHT_TYPE_COUNT: usize = 12;

// ============================================================================
// IP Right Categories - ປະເພດສິດຊັບສິນທາງປັນຍາ
// ============================================================================

/// Category of intellectual property right - ປະເພດສິດຊັບສິນທາງປັນຍາ
///
/// The categories of IP right protected by the consolidated Law on Intellectual
/// Property, spanning industrial property, copyright/related rights, and
/// sui generis rights such as new plant varieties and traditional knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IpRightType {
    /// Patent for an invention (ສິດທິບັດ)
    Patent,
    /// Petty patent / utility innovation (ອານຸສິດທິບັດ)
    PettyPatent,
    /// Industrial design (ແບບອຸດສາຫະກຳ)
    IndustrialDesign,
    /// Trademark / mark (ເຄື່ອງໝາຍການຄ້າ)
    Trademark,
    /// Trade name (ຊື່ທາງການຄ້າ)
    TradeName,
    /// Geographical indication (ສິ່ງບົ່ງຊີ້ທາງພູມສາດ)
    GeographicalIndication,
    /// Copyright in literary/artistic works (ລິຂະສິດ)
    Copyright,
    /// Related (neighbouring) rights (ສິດທິກ່ຽວຂ້ອງ)
    RelatedRights,
    /// Trade secret / undisclosed information (ຄວາມລັບທາງການຄ້າ)
    TradeSecret,
    /// Layout-design of an integrated circuit (ແບບຜັງວົງຈອນລວມ)
    LayoutDesign,
    /// New plant variety (ພັນພືດໃໝ່)
    PlantVariety,
    /// Traditional knowledge (ຄວາມຮູ້ດັ້ງເດີມ)
    TraditionalKnowledge,
}

impl IpRightType {
    /// All twelve categories of intellectual property right.
    pub fn all() -> [IpRightType; IP_RIGHT_TYPE_COUNT] {
        [
            IpRightType::Patent,
            IpRightType::PettyPatent,
            IpRightType::IndustrialDesign,
            IpRightType::Trademark,
            IpRightType::TradeName,
            IpRightType::GeographicalIndication,
            IpRightType::Copyright,
            IpRightType::RelatedRights,
            IpRightType::TradeSecret,
            IpRightType::LayoutDesign,
            IpRightType::PlantVariety,
            IpRightType::TraditionalKnowledge,
        ]
    }

    /// Fixed statutory term of protection in years, where the law fixes one.
    /// ອາຍຸການປົກປ້ອງທີ່ກຳນົດໄວ້ (ປີ)
    ///
    /// Returns `None` for rights whose duration is conditional or indefinite:
    /// - [`IpRightType::Copyright`] and [`IpRightType::RelatedRights`] are tied to
    ///   the life of the author / the date of fixation rather than a single fixed
    ///   span (see [`COPYRIGHT_TERM_AFTER_DEATH_YEARS`]).
    /// - [`IpRightType::TradeName`] subsists while the name remains in use.
    /// - [`IpRightType::GeographicalIndication`] subsists while the qualifying
    ///   origin link persists.
    /// - [`IpRightType::TradeSecret`] subsists while the information stays secret.
    /// - [`IpRightType::TraditionalKnowledge`] is protected indefinitely.
    pub fn protection_term_years(&self) -> Option<u32> {
        match self {
            IpRightType::Patent => Some(PATENT_TERM_YEARS),
            IpRightType::PettyPatent => Some(PETTY_PATENT_TERM_YEARS),
            IpRightType::IndustrialDesign => Some(INDUSTRIAL_DESIGN_TERM_YEARS),
            IpRightType::Trademark => Some(TRADEMARK_TERM_YEARS),
            IpRightType::LayoutDesign => Some(LAYOUT_DESIGN_TERM_YEARS),
            IpRightType::PlantVariety => Some(PLANT_VARIETY_TERM_YEARS),
            IpRightType::Copyright
            | IpRightType::RelatedRights
            | IpRightType::TradeName
            | IpRightType::GeographicalIndication
            | IpRightType::TradeSecret
            | IpRightType::TraditionalKnowledge => None,
        }
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            IpRightType::Patent => "ສິດທິບັດ",
            IpRightType::PettyPatent => "ອານຸສິດທິບັດ",
            IpRightType::IndustrialDesign => "ແບບອຸດສາຫະກຳ",
            IpRightType::Trademark => "ເຄື່ອງໝາຍການຄ້າ",
            IpRightType::TradeName => "ຊື່ທາງການຄ້າ",
            IpRightType::GeographicalIndication => "ສິ່ງບົ່ງຊີ້ທາງພູມສາດ",
            IpRightType::Copyright => "ລິຂະສິດ",
            IpRightType::RelatedRights => "ສິດທິກ່ຽວຂ້ອງ",
            IpRightType::TradeSecret => "ຄວາມລັບທາງການຄ້າ",
            IpRightType::LayoutDesign => "ແບບຜັງວົງຈອນລວມ",
            IpRightType::PlantVariety => "ພັນພືດໃໝ່",
            IpRightType::TraditionalKnowledge => "ຄວາມຮູ້ດັ້ງເດີມ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            IpRightType::Patent => "patent",
            IpRightType::PettyPatent => "petty patent",
            IpRightType::IndustrialDesign => "industrial design",
            IpRightType::Trademark => "trademark",
            IpRightType::TradeName => "trade name",
            IpRightType::GeographicalIndication => "geographical indication",
            IpRightType::Copyright => "copyright",
            IpRightType::RelatedRights => "related rights",
            IpRightType::TradeSecret => "trade secret",
            IpRightType::LayoutDesign => "layout-design of an integrated circuit",
            IpRightType::PlantVariety => "new plant variety",
            IpRightType::TraditionalKnowledge => "traditional knowledge",
        }
    }
}

// ============================================================================
// Application & Registration Status - ສະຖານະຄຳຮ້ອງ ແລະ ການຈົດທະບຽນ
// ============================================================================

/// Status of an industrial-property application (e.g. patent, design).
/// ສະຖານະຄຳຮ້ອງຂໍສິດຊັບສິນທາງປັນຍາ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IpApplicationStatus {
    /// Application has been filed (ຍື່ນຄຳຮ້ອງແລ້ວ)
    Filed,
    /// Application is under substantive examination (ກຳລັງກວດສອບ)
    UnderExamination,
    /// Application has been published (ເຜີຍແຜ່ແລ້ວ)
    Published,
    /// Right has been granted (ໄດ້ຮັບການອະນຸມັດ)
    Granted,
    /// Application has been refused (ຖືກປະຕິເສດ)
    Refused,
    /// Application has been withdrawn by the applicant (ຖອນຄຳຮ້ອງ)
    Withdrawn,
    /// Application has lapsed (ໝົດອາຍຸ)
    Lapsed,
}

impl IpApplicationStatus {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            IpApplicationStatus::Filed => "ຍື່ນຄຳຮ້ອງແລ້ວ",
            IpApplicationStatus::UnderExamination => "ກຳລັງກວດສອບ",
            IpApplicationStatus::Published => "ເຜີຍແຜ່ແລ້ວ",
            IpApplicationStatus::Granted => "ໄດ້ຮັບການອະນຸມັດ",
            IpApplicationStatus::Refused => "ຖືກປະຕິເສດ",
            IpApplicationStatus::Withdrawn => "ຖອນຄຳຮ້ອງ",
            IpApplicationStatus::Lapsed => "ໝົດອາຍຸ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            IpApplicationStatus::Filed => "filed",
            IpApplicationStatus::UnderExamination => "under examination",
            IpApplicationStatus::Published => "published",
            IpApplicationStatus::Granted => "granted",
            IpApplicationStatus::Refused => "refused",
            IpApplicationStatus::Withdrawn => "withdrawn",
            IpApplicationStatus::Lapsed => "lapsed",
        }
    }
}

/// Status of a registered IP right (e.g. trademark, GI).
/// ສະຖານະຂອງສິດທີ່ໄດ້ຈົດທະບຽນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegistrationStatus {
    /// Registration is pending (ກຳລັງດຳເນີນການ)
    Pending,
    /// Right is registered and in force (ຈົດທະບຽນແລ້ວ)
    Registered,
    /// Registration has been renewed (ຕໍ່ອາຍຸແລ້ວ)
    Renewed,
    /// Registration has expired (ໝົດອາຍຸ)
    Expired,
    /// Registration has been cancelled (ຖືກຍົກເລີກ)
    Cancelled,
    /// Registration has been invalidated / declared void (ຖືກໂມຄະ)
    Invalidated,
}

impl RegistrationStatus {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            RegistrationStatus::Pending => "ກຳລັງດຳເນີນການ",
            RegistrationStatus::Registered => "ຈົດທະບຽນແລ້ວ",
            RegistrationStatus::Renewed => "ຕໍ່ອາຍຸແລ້ວ",
            RegistrationStatus::Expired => "ໝົດອາຍຸ",
            RegistrationStatus::Cancelled => "ຖືກຍົກເລີກ",
            RegistrationStatus::Invalidated => "ຖືກໂມຄະ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            RegistrationStatus::Pending => "pending",
            RegistrationStatus::Registered => "registered",
            RegistrationStatus::Renewed => "renewed",
            RegistrationStatus::Expired => "expired",
            RegistrationStatus::Cancelled => "cancelled",
            RegistrationStatus::Invalidated => "invalidated",
        }
    }
}

// ============================================================================
// Patents - ສິດທິບັດ
// ============================================================================

/// Patent application for an invention - ຄຳຮ້ອງຂໍສິດທິບັດ
///
/// An invention is patentable only if it is new (novel), involves an inventive
/// step, and is capable of industrial application.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PatentApplication {
    /// Title of the invention (ຊື່ການປະດິດ)
    pub title: String,
    /// Whether the invention is novel / new (ມີຄວາມໃໝ່)
    pub is_novel: bool,
    /// Whether the invention involves an inventive step (ມີຂັ້ນຕອນການປະດິດສ້າງ)
    pub has_inventive_step: bool,
    /// Whether the invention is industrially applicable (ນຳໃຊ້ທາງອຸດສາຫະກຳໄດ້)
    pub is_industrially_applicable: bool,
    /// Filing year of the application (ປີທີ່ຍື່ນຄຳຮ້ອງ)
    pub filing_year: u32,
}

impl PatentApplication {
    /// Whether all three substantive patentability criteria are satisfied.
    pub fn is_patentable(&self) -> bool {
        self.is_novel && self.has_inventive_step && self.is_industrially_applicable
    }

    /// Year in which the 20-year patent term expires, measured from filing.
    pub fn expiry_year(&self) -> u32 {
        self.filing_year.saturating_add(PATENT_TERM_YEARS)
    }
}

// ============================================================================
// Trademarks - ເຄື່ອງໝາຍການຄ້າ
// ============================================================================

/// Trademark registration - ການຈົດທະບຽນເຄື່ອງໝາຍການຄ້າ
///
/// A mark is registrable if it is distinctive, not deceptive/misleading, and
/// does not conflict with an earlier registered mark. Registration runs for a
/// fixed term and is renewable for successive periods.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrademarkRegistration {
    /// The mark itself (text or description) (ເຄື່ອງໝາຍ)
    pub mark: String,
    /// Whether the mark is distinctive (ມີຄຸນລັກສະນະທີ່ໂດດເດັ່ນ)
    pub is_distinctive: bool,
    /// Whether the mark is deceptive or misleading (ຫຼອກລວງ/ເຮັດໃຫ້ເຂົ້າໃຈຜິດ)
    pub is_deceptive: bool,
    /// Whether the mark conflicts with a prior registered mark (ຂັດກັບເຄື່ອງໝາຍກ່ອນ)
    pub conflicts_with_prior_mark: bool,
    /// Year the mark was first registered (ປີທີ່ຈົດທະບຽນ)
    pub registration_year: u32,
    /// Number of times the registration has been renewed (ຈຳນວນຄັ້ງທີ່ຕໍ່ອາຍຸ)
    pub renewal_count: u32,
}

impl TrademarkRegistration {
    /// Whether the mark satisfies the substantive registrability criteria.
    pub fn is_registrable(&self) -> bool {
        self.is_distinctive && !self.is_deceptive && !self.conflicts_with_prior_mark
    }

    /// Year in which the current registration term expires.
    ///
    /// Each renewal adds a further [`TRADEMARK_TERM_YEARS`] period on top of the
    /// initial term.
    pub fn expiry_year(&self) -> u32 {
        let periods = self.renewal_count.saturating_add(1);
        self.registration_year
            .saturating_add(periods.saturating_mul(TRADEMARK_TERM_YEARS))
    }
}

// ============================================================================
// Copyright - ລິຂະສິດ
// ============================================================================

/// Copyright work - ຜົນງານທີ່ມີລິຂະສິດ
///
/// Copyright subsists automatically in original works without any registration
/// requirement; the economic term runs for the life of the author plus the
/// statutory post-mortem period.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyrightWork {
    /// Title of the work (ຊື່ຜົນງານ)
    pub title: String,
    /// Author of the work (ຜູ້ປະພັນ)
    pub author: String,
    /// Whether the work is original (ມີຄວາມເປັນຕົ້ນສະບັບ)
    pub is_original: bool,
    /// Year of the author's death, if deceased (ປີທີ່ຜູ້ປະພັນເສຍຊີວິດ)
    pub author_death_year: Option<u32>,
    /// Reference (current) year used to assess the term (ປີປັດຈຸບັນ)
    pub current_year: u32,
}

impl CopyrightWork {
    /// Year in which copyright expires into the public domain, if the author is
    /// deceased. Returns `None` while the author is living (term still running).
    pub fn public_domain_year(&self) -> Option<u32> {
        self.author_death_year
            .map(|death| death.saturating_add(COPYRIGHT_TERM_AFTER_DEATH_YEARS))
    }
}

// ============================================================================
// Industrial Designs - ແບບອຸດສາຫະກຳ
// ============================================================================

/// Industrial design - ແບບອຸດສາຫະກຳ
///
/// A design is registrable only if it is new / original.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndustrialDesign {
    /// Title of the design (ຊື່ແບບ)
    pub title: String,
    /// Whether the design is new / original (ໃໝ່/ຕົ້ນສະບັບ)
    pub is_new: bool,
    /// Filing year of the application (ປີທີ່ຍື່ນຄຳຮ້ອງ)
    pub filing_year: u32,
}

// ============================================================================
// Geographical Indications - ສິ່ງບົ່ງຊີ້ທາງພູມສາດ
// ============================================================================

/// Geographical indication - ສິ່ງບົ່ງຊີ້ທາງພູມສາດ
///
/// A GI is registrable where a given quality, reputation or other characteristic
/// of the product is essentially attributable to its geographical origin.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeographicalIndication {
    /// Name of the geographical indication (ຊື່ສິ່ງບົ່ງຊີ້)
    pub name: String,
    /// Region / place of origin (ເຂດ/ແຫຼ່ງກຳເນີດ)
    pub region: String,
    /// Whether the product's quality/reputation is linked to its origin
    /// (ຄຸນນະພາບ/ຊື່ສຽງເຊື່ອມໂຍງກັບແຫຼ່ງກຳເນີດ)
    pub quality_linked_to_origin: bool,
}

// ============================================================================
// Trade Secrets - ຄວາມລັບທາງການຄ້າ
// ============================================================================

/// Trade secret (undisclosed information) - ຄວາມລັບທາງການຄ້າ
///
/// Information is protected as a trade secret if it is secret (not generally
/// known), has commercial value because it is secret, and the holder has taken
/// reasonable steps to keep it secret.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TradeSecret {
    /// Description of the undisclosed information (ລາຍລະອຽດຂໍ້ມູນລັບ)
    pub description: String,
    /// Whether the information is secret / not generally known (ເປັນຄວາມລັບ)
    pub is_secret: bool,
    /// Whether the information has commercial value (ມີມູນຄ່າທາງການຄ້າ)
    pub has_commercial_value: bool,
    /// Whether reasonable steps were taken to keep it secret (ມີມາດຕະການປົກປ້ອງ)
    pub reasonable_protection_steps: bool,
}

impl TradeSecret {
    /// Whether all three trade-secret protection criteria are satisfied.
    pub fn is_protectable(&self) -> bool {
        self.is_secret && self.has_commercial_value && self.reasonable_protection_steps
    }
}

// ============================================================================
// Plant Varieties - ພັນພືດໃໝ່
// ============================================================================

/// New plant variety - ພັນພືດໃໝ່
///
/// A variety qualifies for protection if it is New and meets the DUS criteria —
/// Distinct, Uniform and Stable.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlantVariety {
    /// Denomination (name) of the variety (ຊື່ພັນພືດ)
    pub denomination: String,
    /// Whether the variety is new (ໃໝ່)
    pub is_new: bool,
    /// Whether the variety is distinct (ມີຄວາມແຕກຕ່າງ)
    pub is_distinct: bool,
    /// Whether the variety is uniform (ມີຄວາມສະໝ່ຳສະເໝີ)
    pub is_uniform: bool,
    /// Whether the variety is stable (ມີຄວາມໝັ້ນຄົງ)
    pub is_stable: bool,
}

impl PlantVariety {
    /// Whether the variety satisfies novelty plus the DUS criteria.
    pub fn meets_dus_criteria(&self) -> bool {
        self.is_new && self.is_distinct && self.is_uniform && self.is_stable
    }
}

// ============================================================================
// Infringement - ການລະເມີດ
// ============================================================================

/// Alleged infringement of an IP right - ການລະເມີດສິດຊັບສິນທາງປັນຍາ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IpInfringement {
    /// Category of right alleged to be infringed (ປະເພດສິດທີ່ຖືກລະເມີດ)
    pub right_type: IpRightType,
    /// Whether the use was authorised by the right holder (ໄດ້ຮັບອະນຸຍາດ)
    pub authorized: bool,
    /// Description of the use complained of (ລາຍລະອຽດການນຳໃຊ້)
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection_term_years() {
        assert_eq!(IpRightType::Patent.protection_term_years(), Some(20));
        assert_eq!(IpRightType::PettyPatent.protection_term_years(), Some(10));
        assert_eq!(
            IpRightType::IndustrialDesign.protection_term_years(),
            Some(15)
        );
        assert_eq!(IpRightType::Trademark.protection_term_years(), Some(10));
        assert_eq!(IpRightType::LayoutDesign.protection_term_years(), Some(10));
        assert_eq!(IpRightType::PlantVariety.protection_term_years(), Some(20));
        // Conditional / indefinite duration => None.
        assert_eq!(IpRightType::Copyright.protection_term_years(), None);
        assert_eq!(IpRightType::RelatedRights.protection_term_years(), None);
        assert_eq!(IpRightType::TradeName.protection_term_years(), None);
        assert_eq!(
            IpRightType::GeographicalIndication.protection_term_years(),
            None
        );
        assert_eq!(IpRightType::TradeSecret.protection_term_years(), None);
        assert_eq!(
            IpRightType::TraditionalKnowledge.protection_term_years(),
            None
        );
    }

    #[test]
    fn test_all_right_types_count() {
        assert_eq!(IpRightType::all().len(), IP_RIGHT_TYPE_COUNT);
        assert_eq!(IP_RIGHT_TYPE_COUNT, 12);
    }

    #[test]
    fn test_bilingual_names_present() {
        for right in IpRightType::all() {
            assert!(!right.lao_name().is_empty());
            assert!(!right.english_name().is_empty());
        }
    }

    #[test]
    fn test_status_names_present() {
        let app_statuses = [
            IpApplicationStatus::Filed,
            IpApplicationStatus::UnderExamination,
            IpApplicationStatus::Published,
            IpApplicationStatus::Granted,
            IpApplicationStatus::Refused,
            IpApplicationStatus::Withdrawn,
            IpApplicationStatus::Lapsed,
        ];
        for status in app_statuses {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }

        let reg_statuses = [
            RegistrationStatus::Pending,
            RegistrationStatus::Registered,
            RegistrationStatus::Renewed,
            RegistrationStatus::Expired,
            RegistrationStatus::Cancelled,
            RegistrationStatus::Invalidated,
        ];
        for status in reg_statuses {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }
    }

    #[test]
    fn test_patent_application_helpers() {
        let app = PatentApplication {
            title: "Improved rice husker".to_string(),
            is_novel: true,
            has_inventive_step: true,
            is_industrially_applicable: true,
            filing_year: 2020,
        };
        assert!(app.is_patentable());
        assert_eq!(app.expiry_year(), 2040);

        let weak = PatentApplication {
            title: "Known wheel".to_string(),
            is_novel: false,
            has_inventive_step: true,
            is_industrially_applicable: true,
            filing_year: 2020,
        };
        assert!(!weak.is_patentable());
    }

    #[test]
    fn test_trademark_registrability_and_expiry() {
        let mark = TrademarkRegistration {
            mark: "LaoSilk".to_string(),
            is_distinctive: true,
            is_deceptive: false,
            conflicts_with_prior_mark: false,
            registration_year: 2020,
            renewal_count: 0,
        };
        assert!(mark.is_registrable());
        assert_eq!(mark.expiry_year(), 2030);

        let renewed = TrademarkRegistration {
            renewal_count: 2,
            ..mark.clone()
        };
        assert_eq!(renewed.expiry_year(), 2050);
    }

    #[test]
    fn test_copyright_public_domain_year() {
        let work = CopyrightWork {
            title: "Lao folk anthology".to_string(),
            author: "Author".to_string(),
            is_original: true,
            author_death_year: Some(1990),
            current_year: 2025,
        };
        assert_eq!(work.public_domain_year(), Some(2040));

        let living = CopyrightWork {
            author_death_year: None,
            ..work.clone()
        };
        assert_eq!(living.public_domain_year(), None);
    }

    #[test]
    fn test_trade_secret_and_plant_variety_helpers() {
        let secret = TradeSecret {
            description: "Secret recipe".to_string(),
            is_secret: true,
            has_commercial_value: true,
            reasonable_protection_steps: true,
        };
        assert!(secret.is_protectable());

        let variety = PlantVariety {
            denomination: "LaoJasmine-1".to_string(),
            is_new: true,
            is_distinct: true,
            is_uniform: true,
            is_stable: true,
        };
        assert!(variety.meets_dus_criteria());
    }
}
