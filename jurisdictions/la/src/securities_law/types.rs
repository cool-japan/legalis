//! Securities Law Types (ປະເພດກົດໝາຍຫຼັກຊັບ)
//!
//! Type definitions for Lao securities and capital-markets law based on the
//! **Law on Securities (Lao PDR), 2012** (ກົດໝາຍວ່າດ້ວຍຫຼັກຊັບ).
//!
//! # Legal References
//!
//! - Law on Securities 2012 - the primary statute governing the Lao capital market.
//! - The market operator is the Lao Securities Exchange (LSX, ຕະຫຼາດຫຼັກຊັບລາວ),
//!   which opened in 2011; the regulator is the Lao Securities and Exchange
//!   Commission (Lao SEC, ຄະນະກຳມະການຄຸ້ມຄອງຫຼັກຊັບ).
//!
//! # Numeric thresholds
//!
//! Quantifiable requirements (such as the minimum public float for listing or the
//! cap on foreign ownership of a listed company) are encoded as named, documented
//! constants. Several of these are *representative* regulatory thresholds used as
//! modelling defaults — they are documented as such rather than asserted as exact
//! statutory figures, because the precise figures are set by implementing
//! regulations that this crate cannot independently verify.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Securities 2012
// ============================================================================

/// Representative minimum public float (free float) required for a listing,
/// expressed as a percentage of the company's shares.
///
/// A listed company must distribute a minimum proportion of its shares to the
/// public so that an active, liquid market can form. The exact figure is fixed by
/// the Lao SEC's listing rules; the value here is a representative modelling
/// default.
/// ສັດສ່ວນຮຸ້ນສ່ວນສາທາລະນະຂັ້ນຕ່ຳສຳລັບການຈົດທະບຽນ
pub const MIN_PUBLIC_FLOAT_PERCENT: u32 = 10;

/// Representative cap on aggregate foreign ownership in a listed company,
/// expressed as a percentage of the company's shares.
///
/// Foreign investor participation in listed companies may be capped. This is a
/// representative regulatory threshold used as a modelling default; the precise
/// limit is set by the applicable foreign-investment and securities regulations.
/// ຂີດຈຳກັດການຖືຄອງຮຸ້ນຂອງນັກລົງທຶນຕ່າງປະເທດ
pub const FOREIGN_OWNERSHIP_LIMIT_PERCENT: u32 = 10;

/// Representative deadline, in days, for disclosing a material event under the
/// continuous-disclosure obligation.
///
/// Listed issuers must disclose material information promptly. This is a
/// representative modelling default for the disclosure deadline; the precise
/// period is fixed by the Lao SEC's continuous-disclosure rules.
/// ກຳນົດເວລາ (ມື້) ສຳລັບການເປີດເຜີຍຂໍ້ມູນທີ່ສຳຄັນ
pub const MATERIAL_DISCLOSURE_DEADLINE_DAYS: u32 = 3;

/// Number of security-type categories modelled by this module.
/// ຈຳນວນປະເພດຫຼັກຊັບທີ່ສ້າງແບບຈຳລອງ
pub const SECURITY_TYPE_COUNT: usize = 7;

// ============================================================================
// Securities - ຫຼັກຊັບ
// ============================================================================

/// Type of security traded on the Lao capital market - ປະເພດຫຼັກຊັບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SecurityType {
    /// Ordinary (common) shares (ຮຸ້ນສາມັນ)
    OrdinaryShares,
    /// Preferred shares (ຮຸ້ນບຸລິມະສິດ)
    PreferredShares,
    /// Corporate bond (ພັນທະບັດບໍລິສັດ)
    CorporateBond,
    /// Government bond (ພັນທະບັດລັດຖະບານ)
    GovernmentBond,
    /// Debenture (ຮຸ້ນກູ້)
    Debenture,
    /// Warrant (ໃບສຳຄັນສະແດງສິດ)
    Warrant,
    /// Investment-fund unit (ໜ່ວຍລົງທຶນ)
    InvestmentFundUnit,
}

impl SecurityType {
    /// All security-type categories modelled by this module.
    pub fn all() -> [SecurityType; SECURITY_TYPE_COUNT] {
        [
            SecurityType::OrdinaryShares,
            SecurityType::PreferredShares,
            SecurityType::CorporateBond,
            SecurityType::GovernmentBond,
            SecurityType::Debenture,
            SecurityType::Warrant,
            SecurityType::InvestmentFundUnit,
        ]
    }

    /// Whether this security is a debt instrument (bond or debenture).
    pub fn is_debt(&self) -> bool {
        matches!(
            self,
            SecurityType::CorporateBond | SecurityType::GovernmentBond | SecurityType::Debenture
        )
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            SecurityType::OrdinaryShares => "ຮຸ້ນສາມັນ",
            SecurityType::PreferredShares => "ຮຸ້ນບຸລິມະສິດ",
            SecurityType::CorporateBond => "ພັນທະບັດບໍລິສັດ",
            SecurityType::GovernmentBond => "ພັນທະບັດລັດຖະບານ",
            SecurityType::Debenture => "ຮຸ້ນກູ້",
            SecurityType::Warrant => "ໃບສຳຄັນສະແດງສິດ",
            SecurityType::InvestmentFundUnit => "ໜ່ວຍລົງທຶນ",
        }
    }

    /// English label of the security type.
    pub fn english_name(&self) -> &'static str {
        match self {
            SecurityType::OrdinaryShares => "ordinary shares",
            SecurityType::PreferredShares => "preferred shares",
            SecurityType::CorporateBond => "corporate bond",
            SecurityType::GovernmentBond => "government bond",
            SecurityType::Debenture => "debenture",
            SecurityType::Warrant => "warrant",
            SecurityType::InvestmentFundUnit => "investment-fund unit",
        }
    }
}

/// Type of securities offering - ປະເພດການສະເໜີຂາຍຫຼັກຊັບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OfferingType {
    /// Initial public offering (ການສະເໜີຂາຍຮຸ້ນຕໍ່ສາທາລະນະຄັ້ງທຳອິດ)
    Ipo,
    /// Secondary public offering (ການສະເໜີຂາຍຕໍ່ສາທາລະນະຄັ້ງຕໍ່ໄປ)
    SecondaryPublicOffering,
    /// Private placement to selected investors (ການສະເໜີຂາຍແບບສະເພາະເຈາະຈົງ)
    PrivatePlacement,
    /// Bond issue (ການອອກພັນທະບັດ)
    BondIssue,
}

impl OfferingType {
    /// Whether this offering type requires a prospectus and SEC approval.
    ///
    /// Public offerings (IPO, secondary public offering, public bond issue) require
    /// a prospectus with full disclosure; a private placement is exempt from the
    /// public-offering prospectus rules.
    pub fn requires_prospectus(&self) -> bool {
        !matches!(self, OfferingType::PrivatePlacement)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            OfferingType::Ipo => "ການສະເໜີຂາຍຮຸ້ນຕໍ່ສາທາລະນະຄັ້ງທຳອິດ",
            OfferingType::SecondaryPublicOffering => "ການສະເໜີຂາຍຕໍ່ສາທາລະນະຄັ້ງຕໍ່ໄປ",
            OfferingType::PrivatePlacement => "ການສະເໜີຂາຍແບບສະເພາະເຈາະຈົງ",
            OfferingType::BondIssue => "ການອອກພັນທະບັດ",
        }
    }

    /// English label of the offering type.
    pub fn english_name(&self) -> &'static str {
        match self {
            OfferingType::Ipo => "initial public offering",
            OfferingType::SecondaryPublicOffering => "secondary public offering",
            OfferingType::PrivatePlacement => "private placement",
            OfferingType::BondIssue => "bond issue",
        }
    }
}

/// Type of market participant / securities intermediary - ປະເພດຜູ້ເຂົ້າຮ່ວມຕະຫຼາດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketParticipantType {
    /// Broker-dealer (ນາຍໜ້າຊື້ຂາຍຫຼັກຊັບ)
    BrokerDealer,
    /// Underwriter (ຜູ້ຮັບປະກັນການຈັດຈຳໜ່າຍ)
    Underwriter,
    /// Investment advisor (ທີ່ປຶກສາການລົງທຶນ)
    InvestmentAdvisor,
    /// Fund manager (ຜູ້ຈັດການກອງທຶນ)
    FundManager,
    /// Custodian (ຜູ້ຮັບຝາກຊັບສິນ)
    Custodian,
}

impl MarketParticipantType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            MarketParticipantType::BrokerDealer => "ນາຍໜ້າຊື້ຂາຍຫຼັກຊັບ",
            MarketParticipantType::Underwriter => "ຜູ້ຮັບປະກັນການຈັດຈຳໜ່າຍ",
            MarketParticipantType::InvestmentAdvisor => "ທີ່ປຶກສາການລົງທຶນ",
            MarketParticipantType::FundManager => "ຜູ້ຈັດການກອງທຶນ",
            MarketParticipantType::Custodian => "ຜູ້ຮັບຝາກຊັບສິນ",
        }
    }

    /// English label of the participant type.
    pub fn english_name(&self) -> &'static str {
        match self {
            MarketParticipantType::BrokerDealer => "broker-dealer",
            MarketParticipantType::Underwriter => "underwriter",
            MarketParticipantType::InvestmentAdvisor => "investment advisor",
            MarketParticipantType::FundManager => "fund manager",
            MarketParticipantType::Custodian => "custodian",
        }
    }
}

/// Listing status of a security on the exchange - ສະຖານະການຈົດທະບຽນ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ListingStatus {
    /// Listed and actively traded (ຈົດທະບຽນ ແລະ ຊື້ຂາຍຢູ່)
    Listed,
    /// Trading suspended (ໂຈະການຊື້ຂາຍ)
    Suspended,
    /// Delisted from the exchange (ຖອນອອກຈາກການຈົດທະບຽນ)
    Delisted,
    /// Listing application pending (ກຳລັງລໍຖ້າພິຈາລະນາການຈົດທະບຽນ)
    Pending,
}

impl ListingStatus {
    /// Whether the security is currently listed and actively tradable.
    pub fn is_active(&self) -> bool {
        matches!(self, ListingStatus::Listed)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ListingStatus::Listed => "ຈົດທະບຽນ",
            ListingStatus::Suspended => "ໂຈະການຊື້ຂາຍ",
            ListingStatus::Delisted => "ຖອນອອກຈາກການຈົດທະບຽນ",
            ListingStatus::Pending => "ກຳລັງລໍຖ້າພິຈາລະນາ",
        }
    }

    /// English label of the listing status.
    pub fn english_name(&self) -> &'static str {
        match self {
            ListingStatus::Listed => "listed",
            ListingStatus::Suspended => "suspended",
            ListingStatus::Delisted => "delisted",
            ListingStatus::Pending => "pending",
        }
    }
}

/// Prohibited market conduct - ການກະທຳທີ່ຖືກຫ້າມໃນຕະຫຼາດ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProhibitedConduct {
    /// Trading on material non-public information (ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ)
    InsiderTrading,
    /// Manipulating the market price or volume (ການປັ່ນປ່ວນຕະຫຼາດ)
    MarketManipulation,
    /// Securities fraud (ການສໍ້ໂກງ)
    Fraud,
    /// Front-running client orders (ການຊື້ຂາຍຕັດໜ້າ)
    FrontRunning,
}

impl ProhibitedConduct {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ProhibitedConduct::InsiderTrading => "ການຊື້ຂາຍໂດຍໃຊ້ຂໍ້ມູນພາຍໃນ",
            ProhibitedConduct::MarketManipulation => "ການປັ່ນປ່ວນຕະຫຼາດ",
            ProhibitedConduct::Fraud => "ການສໍ້ໂກງ",
            ProhibitedConduct::FrontRunning => "ການຊື້ຂາຍຕັດໜ້າ",
        }
    }

    /// English label of the prohibited conduct.
    pub fn english_name(&self) -> &'static str {
        match self {
            ProhibitedConduct::InsiderTrading => "insider trading",
            ProhibitedConduct::MarketManipulation => "market manipulation",
            ProhibitedConduct::Fraud => "fraud",
            ProhibitedConduct::FrontRunning => "front-running",
        }
    }
}

// ============================================================================
// Public Offerings - ການສະເໜີຂາຍຕໍ່ສາທາລະນະ
// ============================================================================

/// Public offering of securities - ການສະເໜີຂາຍຫຼັກຊັບ
///
/// Models an offering of securities. A public offering (IPO, secondary public
/// offering or public bond issue) requires a prospectus with full and accurate
/// disclosure and approval by the Lao SEC; a private placement is exempt from the
/// public-offering prospectus rules.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PublicOffering {
    /// Name of the issuer (ຊື່ຜູ້ອອກຫຼັກຊັບ)
    pub issuer: String,
    /// Type of offering (ປະເພດການສະເໜີຂາຍ)
    pub offering_type: OfferingType,
    /// Whether a prospectus has been filed (ມີໜັງສືຊີ້ຊວນ)
    pub has_prospectus: bool,
    /// Whether the prospectus discloses full and accurate information (ໜັງສືຊີ້ຊວນສົມບູນ)
    pub prospectus_complete: bool,
    /// Whether the Lao SEC has approved the offering (ໄດ້ຮັບອະນຸມັດຈາກ ຄຄຫ)
    pub sec_approved: bool,
    /// Total value of the offering in LAK (ມູນຄ່າລວມເປັນກີບ)
    pub total_value_lak: u64,
}

// ============================================================================
// Listed Companies - ບໍລິສັດຈົດທະບຽນ
// ============================================================================

/// Listed company on the exchange - ບໍລິສັດຈົດທະບຽນໃນຕະຫຼາດຫຼັກຊັບ
///
/// Models a company whose securities are listed on the Lao Securities Exchange.
/// Listing requires a minimum public float, ongoing/continuous disclosure of
/// material information, and current periodic financial reporting.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListedCompany {
    /// Company name (ຊື່ບໍລິສັດ)
    pub name: String,
    /// Public float (free float) as a percentage of shares (ສັດສ່ວນຮຸ້ນສ່ວນສາທາລະນະ)
    pub public_float_percent: u32,
    /// Aggregate foreign ownership as a percentage of shares (ສັດສ່ວນການຖືຄອງຂອງຕ່າງປະເທດ)
    pub foreign_ownership_percent: u32,
    /// Whether periodic financial reporting is current (ລາຍງານການເງິນເປັນປະຈຸບັນ)
    pub financial_reports_current: bool,
    /// Current listing status (ສະຖານະການຈົດທະບຽນ)
    pub status: ListingStatus,
}

// ============================================================================
// Securities Companies / Intermediaries - ບໍລິສັດຫຼັກຊັບ / ຕົວກາງ
// ============================================================================

/// Securities company or intermediary - ບໍລິສັດຫຼັກຊັບ ຫຼື ຕົວກາງ
///
/// Securities companies and intermediaries (broker-dealers, underwriters,
/// investment advisors, fund managers, custodians) must be licensed by the Lao
/// SEC and adequately capitalised.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecuritiesCompany {
    /// Company name (ຊື່ບໍລິສັດ)
    pub name: String,
    /// Type of market participant (ປະເພດຜູ້ເຂົ້າຮ່ວມຕະຫຼາດ)
    pub participant_type: MarketParticipantType,
    /// Whether the company holds a Lao SEC licence (ມີໃບອະນຸຍາດ)
    pub licensed: bool,
    /// Registered capital in LAK (ທຶນຈົດທະບຽນເປັນກີບ)
    pub registered_capital_lak: u64,
}

// ============================================================================
// Trading & Disclosure - ການຊື້ຂາຍ ແລະ ການເປີດເຜີຍຂໍ້ມູນ
// ============================================================================

/// A securities trade evaluated for prohibited conduct - ການຊື້ຂາຍຫຼັກຊັບ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecuritiesTrade {
    /// Security being traded (ຫຼັກຊັບທີ່ຊື້ຂາຍ)
    pub security: SecurityType,
    /// Whether the trade used material non-public (inside) information (ໃຊ້ຂໍ້ມູນພາຍໃນ)
    pub used_inside_information: bool,
    /// Whether the trade is manipulative (ມີລັກສະນະປັ່ນປ່ວນຕະຫຼາດ)
    pub manipulative: bool,
}

/// A disclosure event under the continuous-disclosure obligation - ການເປີດເຜີຍຂໍ້ມູນ
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisclosureEvent {
    /// Description of the event (ລາຍລະອຽດຂອງເຫດການ)
    pub description: String,
    /// Whether the event is material (ເປັນຂໍ້ມູນທີ່ສຳຄັນ)
    pub material: bool,
    /// Whether it was disclosed within the deadline (ເປີດເຜີຍພາຍໃນກຳນົດເວລາ)
    pub disclosed_within_deadline: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offering_requires_prospectus() {
        assert!(OfferingType::Ipo.requires_prospectus());
        assert!(OfferingType::SecondaryPublicOffering.requires_prospectus());
        assert!(OfferingType::BondIssue.requires_prospectus());
        assert!(!OfferingType::PrivatePlacement.requires_prospectus());
    }

    #[test]
    fn test_security_type_debt_classification() {
        assert!(SecurityType::CorporateBond.is_debt());
        assert!(SecurityType::GovernmentBond.is_debt());
        assert!(SecurityType::Debenture.is_debt());
        assert!(!SecurityType::OrdinaryShares.is_debt());
        assert!(!SecurityType::Warrant.is_debt());
    }

    #[test]
    fn test_all_security_types_count_and_names() {
        assert_eq!(SecurityType::all().len(), SECURITY_TYPE_COUNT);
        assert_eq!(SECURITY_TYPE_COUNT, 7);
        for security in SecurityType::all() {
            assert!(!security.lao_name().is_empty());
            assert!(!security.english_name().is_empty());
        }
    }

    #[test]
    fn test_listing_status_active() {
        assert!(ListingStatus::Listed.is_active());
        assert!(!ListingStatus::Suspended.is_active());
        assert!(!ListingStatus::Delisted.is_active());
        assert!(!ListingStatus::Pending.is_active());
    }

    #[test]
    fn test_threshold_constants() {
        assert_eq!(MIN_PUBLIC_FLOAT_PERCENT, 10);
        assert_eq!(FOREIGN_OWNERSHIP_LIMIT_PERCENT, 10);
        assert_eq!(MATERIAL_DISCLOSURE_DEADLINE_DAYS, 3);
    }

    #[test]
    fn test_bilingual_names_present() {
        let participants = [
            MarketParticipantType::BrokerDealer,
            MarketParticipantType::Underwriter,
            MarketParticipantType::InvestmentAdvisor,
            MarketParticipantType::FundManager,
            MarketParticipantType::Custodian,
        ];
        for participant in participants {
            assert!(!participant.lao_name().is_empty());
            assert!(!participant.english_name().is_empty());
        }

        let conducts = [
            ProhibitedConduct::InsiderTrading,
            ProhibitedConduct::MarketManipulation,
            ProhibitedConduct::Fraud,
            ProhibitedConduct::FrontRunning,
        ];
        for conduct in conducts {
            assert!(!conduct.lao_name().is_empty());
            assert!(!conduct.english_name().is_empty());
        }
    }
}
